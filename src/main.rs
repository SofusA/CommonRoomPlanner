use dotenv::dotenv;
use handler_lib::{
    filters::filters::{
        delete_json, handle_delete_entry, handle_get_next_entry, handle_post_entry, post_json,
    },
    models::interfaces::WeekRequest,
};
use std::net::SocketAddr;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let add_items = warp::post()
        .and(warp::path("api"))
        .and(warp::path("booking"))
        .and(warp::path::end())
        .and(post_json())
        .and_then(handle_post_entry);

    let get_items = warp::get()
        .and(warp::path("api"))
        .and(warp::path("booking"))
        .and(warp::path::end())
        .and(warp::query::<WeekRequest>())
        .and_then(handle_get_next_entry);

    let delete_item = warp::delete()
        .and(warp::path("api"))
        .and(warp::path("booking"))
        .and(warp::path::end())
        .and(delete_json())
        .and_then(handle_delete_entry);

    let routes = add_items.or(get_items).or(delete_item);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    warp::serve(routes).run(addr).await
}
