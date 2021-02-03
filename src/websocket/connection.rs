use crate::{Clients, structs::client::Client};
use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use serde_json::from_str;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub async fn client_connection(ws: WebSocket, uuid: String, clients: Clients, mut client: Client) {
  let (client_ws_sender, mut client_ws_rcv) = ws.split();
  let (client_sender, client_rcv) = mpsc::unbounded_channel();

  tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
    if let Err(e) = result {
      eprint!("error sending websocket msg: {}", e);
    }
  }));

  client.sender = Some(client_sender);
  clients.write().await.insert(uuid.clone(), client);

  println!("{} connected", uuid);
}