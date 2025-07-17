use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};



pub async fn print_next(id: u32, url: &str) {
    // yew::Renderer::<App>::new().render();
    // let url = "wss://echo.websocket.events";
    let (ws_stream, _) = connect_async(url).await.unwrap();
    println!("WebSocket connection established at {}", url);
    let (mut write, mut read) = ws_stream.split();
    let msg = Message::Text(format!("I am client_{}", id).into());
    write.send(msg).await.unwrap();

    if let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            println!("Received on client_{}: {}", id, text);
        }
    }
}