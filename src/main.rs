use dotenv::dotenv;
use std::{env, net::Ipv4Addr};
use warp::{Filter, Rejection};

use postgrest::Postgrest;
use handler_lib::models::entry::Entry;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let api = warp::path!("api" / "booking").and_then(get_response);

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(api).run((Ipv4Addr::LOCALHOST, port)).await
}

pub async fn get_response() -> Result<impl warp::Reply, Rejection> {
    let database_secret: String = match env::var("DB_SECRET") {
        Ok(val) => val.parse().expect("Error parsing client secret"),
        Err(err) => panic!("Error reading client secret: {}", err),
    };

    let database_url: String = match env::var("DB_URL") {
        Ok(val) => val.parse().expect("Error parsing client secret"),
        Err(err) => panic!("Error reading client secret: {}", err),
    };

    let database_table: String = match env::var("DB_TABLE") {
        Ok(val) => val.parse().expect("Error parsing client secret"),
        Err(err) => panic!("Error reading client secret: {}", err),
    };

    let client = Postgrest::new(database_url)
        .insert_header("apikey", database_secret);
    let resp = client.from(database_table).select("*").execute().await.expect("Error getting response");
    let body = resp.json::<Vec::<Entry>>().await.unwrap();

    match body.first() {
        Some(res) => return Ok(res.person.clone()),
        None => return Ok("somereturn".to_string()),
    };
}
