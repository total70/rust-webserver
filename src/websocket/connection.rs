use crate::{Clients, structs::client::Client};
use futures::{FutureExt, StreamExt};

use warp::ws::{WebSocket};

pub async fn client_connection(ws: WebSocket, uuid: String, clients: Clients, mut client: Client) {
  let (client_ws_sender, client_ws_rcv) = ws.split();
  let (client_sender, client_rcv) = futures::channel::mpsc::unbounded();

  tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
    if let Err(e) = result {
      eprint!("error sending websocket msg: {}", e);
    }
  }));

  client.sender = Some(client_sender);
  clients.write().await.insert(uuid.clone(), client);

  println!("{} connected", uuid);
}