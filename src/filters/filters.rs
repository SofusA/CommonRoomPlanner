use async_trait::async_trait;
use reqwest::StatusCode;
use warp::Filter;

use crate::models::constants::{database_secret, database_table_name, database_url};
use crate::models::database::*;
use crate::models::entry::{DateFormat, Entry};
use postgrest::Postgrest;

pub fn delete_json() -> impl Filter<Extract = (DateFormat, ), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


pub fn post_json() -> impl Filter<Extract = (Entry, ), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn handle_post_entry(entry: Entry) -> Result<impl warp::Reply, warp::Rejection> {
    let database: SupabaseDb = Database::new();

    let database_response = database.add(entry).await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(response, StatusCode::CREATED)),
        Err(_) => return Err(warp::reject()),
    }
}

pub async fn handle_delete_entry(entry_date: DateFormat) -> Result<impl warp::Reply, warp::Rejection>  {
    let database: SupabaseDb = Database::new();

    let database_response = database.delete(entry_date).await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(response, StatusCode::OK)),
        Err(_) => return Err(warp::reject()),
    }
}

pub async fn handle_get_next_entry() -> Result<impl warp::Reply, warp::Rejection> {
    let database: SupabaseDb = Database::new();

    let database_response = database.get_latest().await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(response, StatusCode::OK)),
        Err(_) => return Err(warp::reject()),
    }
}

#[async_trait]
impl Database for SupabaseDb {
    async fn delete(&self, date: DateFormat) -> Result<String, String> {
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
            .eq("date", date)
            .execute()
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(format!("Error deleting entry: {}", err)),
        };

        match resp.json::<Entry>().await {
            Ok(res) => {
                return Ok(match serde_json::to_string(&res) {
                    Ok(res) => res,
                    Err(_) => "error serialising json".to_string(),
                })
            }
            Err(err) => return Err(format!("Error deleting booking: {}", err)),
        }
    }

    async fn add(&self, entry: Entry) -> Result<String, String> {
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
            Err(err) => return Err(format!("Error deleting entry: {}", err)),
        };

        match resp.json::<Entry>().await {
            Ok(res) => {
                return Ok(match serde_json::to_string(&res) {
                    Ok(res) => res,
                    Err(_) => "error serialising json".to_string(),
                })
            }
            Err(err) => return Err(format!("Error deleting booking: {}", err)),
        }
    }

    async fn get_latest(&self) -> Result<String, String> {
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
            .select("date, person")
            .order("date.asc")
            .limit(1)
            .single()
            .execute()
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(format!("Error deleting entry: {}", err)),
        };

        match resp.json::<Entry>().await {
            Ok(res) => {
                return Ok(match serde_json::to_string(&res) {
                    Ok(res) => res,
                    Err(_) => "error serialising json".to_string(),
                })
            }
            Err(err) => return Err(format!("Error deleting booking: {}", err)),
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
