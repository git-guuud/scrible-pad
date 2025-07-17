use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{future, pin_mut, StreamExt};

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:8080"; 

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    // tokio::spawn(read_stdin(stdin_tx));
    
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    
    let (write, read) = ws_stream.split();
    
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            println!("Received: {}", message.unwrap().into_text().unwrap());
        })
    };
    stdin_tx.unbounded_send(Message::text("Hello WebSocket!")).unwrap();

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}