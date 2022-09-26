use handler_lib::{
    filters::filters::{
        delete_json, handle_delete_entry, handle_get_next_entry, handle_post_entry, post_json,
    },
    models::constants::endpoint,
};
use std::{env, net::Ipv4Addr};
use warp::Filter;

#[tokio::main]
async fn main() {
    let endpoint = endpoint().unwrap();

    let add_items = warp::post()
        .and(warp::path("api"))
        .and(warp::path(endpoint.clone()))
        .and(warp::path::end())
        .and(post_json())
        .and_then(handle_post_entry);

    let get_items = warp::get()
        .and(warp::path("api"))
        .and(warp::path(endpoint.clone()))
        .and(warp::path::end())
        .and_then(handle_get_next_entry);

    let delete_item = warp::delete()
        .and(warp::path("api"))
        .and(warp::path(endpoint.clone()))
        .and(warp::path::end())
        .and(delete_json())
        .and_then(handle_delete_entry);

    let routes = add_items.or(get_items).or(delete_item);

    let port: u16 = match env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3001,
    };

    warp::serve(routes).run((Ipv4Addr::LOCALHOST, port)).await
}
