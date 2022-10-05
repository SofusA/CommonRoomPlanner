use async_trait::async_trait;
use chrono::Duration;
use reqwest::StatusCode;
use warp::Filter;

use crate::models::constants::*;
use crate::models::database::*;
use crate::models::interfaces::*;
use postgrest::Postgrest;

pub fn delete_json() -> impl Filter<Extract = (EntryId,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn post_json() -> impl Filter<Extract = (Entry,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn handle_post_entry(entry: Entry) -> Result<impl warp::Reply, warp::Rejection> {
    let database: SupabaseDb = Database::new();

    let database_response = database.add(entry).await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(format!("Response from database: {}", response), response)),
        Err(err) => return Ok(warp::reply::with_status(err, StatusCode::BAD_REQUEST)),
    }
}

pub async fn handle_delete_entry(entry_id: EntryId) -> Result<impl warp::Reply, warp::Rejection> {
    let database: SupabaseDb = Database::new();

    let database_response = database.delete(entry_id.date).await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(format!("Response from database: {}", response), response)),
        Err(err) => return Ok(warp::reply::with_status(err, StatusCode::BAD_REQUEST)),
    }
}

pub async fn handle_get_next_entry(request: WeekRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let database: SupabaseDb = Database::new();

    let database_response = database.get_next_weeks(request).await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(response, StatusCode::OK)),
        Err(err) => return Ok(warp::reply::with_status(err, StatusCode::BAD_REQUEST)),
    }
}

#[async_trait]
impl Database for SupabaseDb {
    async fn delete(&self, entry_id: DateFormat) -> Result<StatusCode, String> {
        let client = match Self::get_client() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let database_table_name = match database_table_name() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let resp = match client
            .from(database_table_name)
            .delete()
            .eq("date", entry_id)
            .execute()
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(format!("Error from Supabase: {}", err)),
        };

        return Ok(resp.status());
    }

    async fn add(&self, entry: Entry) -> Result<StatusCode, String> {
        let client = match Self::get_client() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let serialised_entry = match serde_json::to_string(&entry) {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let database_table_name = match database_table_name() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let resp = match client
            .from(database_table_name)
            .insert(format!("[{}]", serialised_entry))
            .execute()
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(format!("Error from Supabase: {}", err)),
        };

        return Ok(resp.status());

    }

    async fn get_next_weeks(&self, request: WeekRequest) -> Result<String, String> {
        let client = match Self::get_client() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let database_table_name = match database_table_name() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        let today = chrono::offset::Utc::now();
        let next_week = today + Duration::weeks(request.weeks);

        let resp = match client
            .from(database_table_name)
            .select("date, person")
            .gte("date", today.format("%Y-%m-%d").to_string())
            .lt("date", next_week.format("%Y-%m-%d").to_string())
            .order("date.asc")
            .execute()
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(format!("Error from Supabase: {}", err)),
        };

        match resp.json::<Vec<Entry>>().await {
            Ok(res) => {
                return Ok(match serde_json::to_string(&res) {
                    Ok(res) => res,
                    Err(_) => "error serialising json".to_string(),
                })
            }
            Err(err) => return Err(format!("Error getting latest booking: {}", err)),
        }
    }

    fn new() -> SupabaseDb {
        return SupabaseDb {};
    }

    fn get_client() -> Result<Postgrest, String> {
        let url = match database_url() {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let database_secret = match database_secret() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        return Ok(Postgrest::new(url).insert_header("apikey", database_secret));
    }
}
