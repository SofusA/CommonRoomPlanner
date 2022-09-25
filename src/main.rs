use handler_lib::filters::filters;
use std::{env, net::Ipv4Addr};
use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::path("api").and(
        filters::post_entry()
            .or(filters::delete_entry())
            .or(filters::get_next_entry()),
    );

    let port: u16 = match env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(routes).run((Ipv4Addr::LOCALHOST, port)).await
}
