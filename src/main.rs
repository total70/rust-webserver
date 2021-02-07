use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{RwLock};
use warp::{Filter, Rejection};

mod handlers;
mod structs;
mod websocket;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, structs::client::Client>>>;

#[tokio::main]
async fn main() {
    println!("Starting up webserver...");

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let health_check = warp::get()
    .and(warp::path("health_check"))
    .and_then(handlers::health_check::get);

    let generate_uuid = warp::get()
    .and(warp::path("generate"))
    .and(with_clients(clients.clone()))
    .and_then(handlers::generate_uuid::get);

    let publish = warp::post()
    .and(warp::path("publish"))
    .and(warp::body::json())
    .and(with_clients(clients.clone()))
    .and_then(handlers::publish::post);

    let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(warp::path::param())
    .and(with_clients(clients.clone()))
    .and_then(handlers::ws::set_websocket_connection);

    let routes = health_check
        .or(generate_uuid)
        .or(publish)
        .or(ws_route);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
   warp::any().map(move || clients.clone())
}