use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::Filter;

mod handlers;
mod structs;

type Clients = Arc<RwLock<HashMap<String, structs::client::Client>>>;

#[tokio::main]
async fn main() {
    println!("Starting up webserver...");

    let health_check = warp::get()
    .and(warp::path("health_check"))
    .and_then(handlers::health_check::get);

    let generate_uuid = warp::get()
    .and(warp::path("generate"))
    .and_then(handlers::generate_uuid::get);

    let routes = health_check
        .or(generate_uuid);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
   warp::any().map(move || clients.clone())
}