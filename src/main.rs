use std::env;
use std::net::Ipv4Addr;
use warp::Filter;


#[tokio::main]
async fn main() {
    let api = warp::path!("api" / "v1").and_then(todo());

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(api).run((Ipv4Addr::LOCALHOST, port)).await
}