use chrono::Duration;
use reqwest::StatusCode;
use warp::Filter;

use crate::models::database::*;
use crate::models::interfaces::*;

use crate::supabase::supabase::SupabaseDb;

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

    let today = chrono::offset::Utc::now();
    let to = today + Duration::weeks(request.weeks);

    let database_response = database.get_entries(today, to).await;

    match database_response {
        Ok(response) => return Ok(warp::reply::with_status(response, StatusCode::OK)),
        Err(err) => return Ok(warp::reply::with_status(err, StatusCode::BAD_REQUEST)),
    }
}