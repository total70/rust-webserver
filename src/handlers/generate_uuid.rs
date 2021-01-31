use uuid::Uuid;
use std::collections::HashMap;
use warp::{Reply, Rejection};

pub async fn get() -> Result<impl Reply, Rejection> {
  let mut result = HashMap::new();
  let uuid = Uuid::new_v4();
  result.insert("id", uuid);

  Ok(warp::reply::json(&result))
}