use crate::{websocket::connection::client_connection, Clients, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

pub async fn set_websocket_connection(ws: warp::ws::Ws, uuid: String, clients: Clients) -> Result<impl Reply> {
  let client = clients.read().await.get(&uuid).cloned();
  match client {
    Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket, uuid, clients, c))),
    None => Err(warp::reject::not_found())
  }
}