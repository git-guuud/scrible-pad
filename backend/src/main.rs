use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};

use tokio::{net::{TcpListener, TcpStream}};
use futures_util::{SinkExt, StreamExt, future, pin_mut, TryStreamExt};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use futures_channel::mpsc::{unbounded, UnboundedSender};

type PeerMap = Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>;

#[tokio::main]
pub async fn main() {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    let peer_map: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(
            // let ws_stream = accept_async(stream).await.unwrap();
            // let (mut write, mut read) = ws_stream.split();
            // let msg = Message::Text("Hello WebSocket".into());
            // // println!("New WebSocket connection established");
            // write.send(msg).await.unwrap();

            // while let Some(Ok(msg)) = read.next().await {
                
            // }
            handle_connection(peer_map.clone(), stream, addr)
        );
    }

}

async fn handle_connection(
    peer_map: PeerMap,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    let ws_stream = accept_async(raw_stream).await.unwrap();
    println!("New WebSocket connection established from {}", addr);
    let (mut write, mut read) = ws_stream.split();
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);
    let broadcast_incoming = read.try_for_each(|msg| {
        println!("Received message from {}: {:?}", addr, msg.to_text().unwrap());
        // let msg = msg.unwrap();
        let peers = peer_map.lock().unwrap();
        for (peer_addr, peer_tx) in peers.iter() {
            if *peer_addr != addr {
                let _ = peer_tx.unbounded_send(msg.clone());
            }
        }
        future::ok(())
    });

    let recieve_messages = rx.map(Ok).forward(write);
    pin_mut!(broadcast_incoming, recieve_messages);
    future::select(broadcast_incoming, recieve_messages).await;
    peer_map.lock().unwrap().remove(&addr);
    println!("WebSocket connection closed from {}", addr);
}