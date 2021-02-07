use crate::{Clients, structs::client::Client};
use uuid::Uuid;
use std::collections::HashMap;
use warp::{Reply, Rejection};


async fn register_client(uuid: &String, clients: Clients) {
  clients.write().await.insert(
    uuid.to_owned(),
    Client {
      uuid: uuid.clone(),
      sender: None
    }
  );
}

pub async fn get(clients: Clients) -> Result<impl Reply, Rejection> {
  let mut result = HashMap::new();
  let uuid = Uuid::new_v4();
  result.insert("id", uuid);
  register_client(&uuid.to_string(), clients).await;
  Ok(warp::reply::json(&result))
}