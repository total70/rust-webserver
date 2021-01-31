use warp::{http::StatusCode, Reply, Rejection};

pub async fn get() -> Result<impl Reply, Rejection> {
  Ok(warp::reply::with_status(
    "hello",
    StatusCode::OK
  ))
}