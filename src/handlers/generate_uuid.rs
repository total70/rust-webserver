use crate::{Clients, structs::client::Client};
use uuid::Uuid;
use std::collections::HashMap;
use warp::{Reply, Rejection};


async fn register_client(uuid: String, clients: Clients) {
  let copy = uuid.clone();
  clients.write().await.insert(
    copy,
    Client {
      uuid: uuid,
      sender: None
    }
  );
}

pub async fn get(clients: Clients) -> Result<impl Reply, Rejection> {
  let mut result = HashMap::new();
  let uuid = Uuid::new_v4();
  result.insert("id", uuid);
  register_client(uuid.to_string().clone(), clients).await;
  Ok(warp::reply::json(&result))
}