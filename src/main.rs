use dotenv::dotenv;
use std::env;
use std::net::Ipv4Addr;
use warp::Filter;

use serde::{Deserialize, Serialize};
use serde_sheets::{get_sheets, service_account_from_env};

use std::convert::Infallible;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ExampleObject {
    name: String,
    number_of_foos: u64,
    number_of_bars: f64,
}

fn generate_sample_objects(n: u64) -> Vec<ExampleObject> {
    (0..n)
        .map(|i| ExampleObject {
            name: format!("Object {}", i),
            number_of_foos: i * 10,
            number_of_bars: i as f64 + 0.5,
        })
        .collect()
}

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

pub async fn get_response() -> Result<impl warp::Reply, Infallible> {
    let document_id: String = match env::var("DOCUMENT_ID") {
        Ok(val) => val.parse().expect("Document id not found"),
        Err(_) => return Ok("Error parsing document id".to_string()),
    };

    let tab_id: String = match env::var("TAB_NAME") {
        Ok(val) => val.parse().expect("Tab name not found"),
        Err(_) => return Ok("Error parsing tab name".to_string()),
    };


    return Ok("works".to_string());
}