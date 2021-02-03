use crate::{structs::client::Client, Clients, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, ws::Message, Reply, Rejection};

#[derive(Deserialize, Debug)]
pub struct Event {
    uuid: Option<String>,
    category: String,
}

pub async fn post(body: Event, clients: Clients) -> Result<impl Reply> {
  clients
  .read()
  .await
  .iter()
  .filter(|(_, client)| match &body.uuid {
    Some(v) => client.uuid.eq(v),
    None => true
  })
  .for_each(|(_, client)| {
    if let Some(sender) = &client.sender {
      let _ = sender.unbounded_send(Ok(Message::text(body.category.clone())));
    }
  });
  //Ok()
  Ok(warp::reply::with_status(
    "hello",
    StatusCode::OK
  ))
}