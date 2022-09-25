use async_trait::async_trait;
use reqwest::StatusCode;
use warp::{Filter, Rejection};

use crate::models::constants::{database_secret, database_table_name, database_url, endpoint};
use crate::models::database::*;
use crate::models::entry::{DateFormat, Entry};
use postgrest::Postgrest;

pub fn post_entry() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let endpoint = endpoint();

    warp::path(endpoint)
        .and(warp::post())
        .and(entry_json_body())
        .and_then(handle_post_entry)
}

pub fn delete_entry() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let endpoint = endpoint();

    warp::path(endpoint)
        .and(warp::delete())
        .and(dateformat_json_body())
        .and_then(handle_delete_entry)
}

pub fn get_next_entry() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let endpoint = endpoint();

    warp::path(endpoint)
        .and(warp::get())
        .and_then(handle_get_next_entry)
}

pub async fn handle_post_entry(entry: Entry) -> Result<impl warp::Reply, Rejection> {
    let database: SupabaseDb = Database::new();

    let database_response = database.add(entry).await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(response, StatusCode::CREATED)),
        Err(_) => return Err(warp::reject()),
    }
}

pub async fn handle_delete_entry(entry_date: DateFormat) -> Result<impl warp::Reply, Rejection> {
    let database: SupabaseDb = Database::new();

    let database_response = database.delete(entry_date).await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(response, StatusCode::CREATED)),
        Err(_) => return Err(warp::reject()),
    }
}

pub async fn handle_get_next_entry() -> Result<impl warp::Reply, Rejection> {
    let database: SupabaseDb = Database::new();

    let database_response = database.get_latest().await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(response, StatusCode::CREATED)),
        Err(_) => return Err(warp::reject()),
    }
}

fn entry_json_body() -> impl Filter<Extract = (Entry,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn dateformat_json_body() -> impl Filter<Extract = (DateFormat,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[async_trait]
impl Database for SupabaseDb {
    async fn delete(&self, date: DateFormat) -> Result<String, String> {
        let client = Self::get_client();

        let resp = client
            .from(database_table_name())
            .delete()
            .eq("date", date)
            .execute()
            .await
            .expect("Error getting response");

        match resp.json::<Entry>().await {
            Ok(res) => return Ok(serde_json::to_string(&res).unwrap()),
            Err(err) => return Err(format!("Error deleting booking: {}", err)),
        }
    }

    async fn add(&self, entry: Entry) -> Result<String, String> {
        let client = Self::get_client();

        let resp = client
            .from(database_table_name())
            .insert(format!("[{}]", serde_json::to_string(&entry).unwrap()))
            .execute()
            .await
            .expect("Error getting response");

        match resp.json::<Entry>().await {
            Ok(res) => return Ok(serde_json::to_string(&res).unwrap()),
            Err(err) => return Err(format!("Error adding booking: {}", err)),
        }
    }

    async fn get_latest(&self) -> Result<String, String> {
        let client = Self::get_client();

        let resp = client
            .from(database_table_name())
            .select("date, person")
            .order("date.desc")
            .limit(1)
            .single()
            .execute()
            .await
            .expect("Error getting response");

        match resp.json::<Entry>().await {
            Ok(res) => return Ok(serde_json::to_string(&res).unwrap()),
            Err(err) => return Err(format!("Error retrieving latest booking: {}", err)),
        }
    }

    fn new() -> SupabaseDb {
        return SupabaseDb {};
    }

    fn get_client() -> Postgrest {
        return Postgrest::new(database_url()).insert_header("apikey", database_secret());
    }
}
