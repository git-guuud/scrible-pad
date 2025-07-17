use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};

use tokio::{net::TcpListener, sync::mpsc::UnboundedSender};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, tungstenite::Message};


type PeerMap = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;
pub async fn run(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split();
            let msg = Message::Text("Hello WebSocket".into());
            // println!("New WebSocket connection established");
            write.send(msg).await.unwrap();

            while let Some(Ok(msg)) = read.next().await {
                
            }
        });
    }
}