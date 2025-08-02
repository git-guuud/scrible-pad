use std::{collections::HashMap, net::SocketAddr, sync::{Arc}};

use tokio::{net::{TcpListener, TcpStream}};
use tokio::sync::Mutex;
use futures_util::{SinkExt, StreamExt, future, pin_mut, TryStreamExt};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use futures_channel::mpsc::{unbounded, UnboundedSender};

type PeerMap = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;

#[tokio::main]
pub async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(addr).await.unwrap();
    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    let saved_data = Arc::new(Mutex::new("".to_string()));
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(
            // let ws_stream = accept_async(stream).await.unwrap();
            // let (mut write, mut read) = ws_stream.split();
            // let msg = Message::Text("Hello WebSocket".into());
            // // println!("New WebSocket connection established");
            // write.send(msg).await.unwrap();

            // while let Some(Ok(msg)) = read.next().await {
                
            // }
            handle_connection(peer_map.clone(), stream, addr, saved_data.clone())
        );
    }

}

async fn handle_connection(
    peer_map: PeerMap,
    raw_stream: TcpStream,
    addr: SocketAddr,
    saved_data: Arc<Mutex<String>>,
) {
    let ws_stream = accept_async(raw_stream).await.expect("Error during WebSocket handshake");
    println!("New WebSocket connection established from {}", addr);
    let (mut write, read) = ws_stream.split();
    let (tx, rx) = unbounded();
    peer_map.lock().await.insert(addr, tx);
    let saved_text = saved_data.lock().await.clone();
    if !saved_text.is_empty() {
        write.send(Message::Text(format!("Load:{}", saved_text).into())).await.unwrap();
    }
    let broadcast_incoming = read.try_for_each(async |msg| {
        if msg.is_close() {
            
        } else if msg.is_text() {
            let text = msg.to_text().unwrap_or("Invalid message");
            println!("Received message from {}: {}", addr, msg.to_text().unwrap());
            

            if text.starts_with("Load:") {
                saved_data.lock().await.clear();
                saved_data.lock().await.push_str(&text[5..]);
            } else if text == "Clear" {
                saved_data.lock().await.clear();
            }
            // let msg = msg.unwrap();
            let peers = peer_map.lock().await;
            for (peer_addr, peer_tx) in peers.iter() {
                if *peer_addr != addr {
                    let _ = peer_tx.unbounded_send(msg.clone());
                }
            }
        }
        Ok(())
    });

    let recieve_messages = rx.map(Ok).forward(write);
    pin_mut!(broadcast_incoming, recieve_messages);
    future::select(broadcast_incoming, recieve_messages).await;
    peer_map.lock().await.remove(&addr);
    println!("WebSocket connection closed from {}", addr);
}