use crate::{Clients, Result};
use serde::{Deserialize};
use std::collections::HashMap;
use warp::{ws::Message, Reply};

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
  let mut response = HashMap::new();
  response.insert("status", "event_send");
  Ok(warp::reply::json(&response))
}