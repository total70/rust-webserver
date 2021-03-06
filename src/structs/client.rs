use futures::channel::mpsc;
use std::result::Result;
use warp::{ws::Message, Error};

#[derive(Debug, Clone)]
pub struct Client {
  pub uuid: String,
  pub sender: Option<mpsc::UnboundedSender<Result<Message, Error>>>
}