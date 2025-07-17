// mod client;



use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use tokio_tungstenite_wasm::{connect, Message};
use futures_util::{SinkExt, StreamExt};

#[derive(Serialize, Deserialize)]
struct Stroke {
    pub color: String,
    pub width: f32,
    pub points: Vec<Pos>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
struct Pos {
    x: f32,
    y: f32,
}

#[wasm_bindgen(module = "/site/rust_call.js")]
extern "C" {
    pub fn draw(jsonStroke: String);
    pub fn log(message: String);
}

#[wasm_bindgen(start)]
async fn main() {
    let stroke = Stroke {
        color: "red".to_string(),
        width: 5.0,
        points: vec![
            Pos { x: 10.0, y: 20.0 },
            Pos { x: 30.0, y: 40.0 },
            Pos { x: 50.0, y: 60.0 },
        ],
    };
    // draw(stroke.width, stroke.color, stroke.points);
    let ws_stream = connect("ws://localhost:8080").await.unwrap();
    log("WebSocket connected".to_string());
    let (mut write, mut read) = ws_stream.split();
    let msg = Message::Text(serde_json::to_string(&stroke).unwrap().into());

    write.send(msg).await.unwrap();
    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            log(format!("Received: {}", text));
            draw(text.to_string());
        }
    }
}

#[wasm_bindgen]
pub async fn send(json: String) {
    let ws_stream = connect("ws://localhost:8080").await.unwrap();
    let (mut write, _) = ws_stream.split();
    let msg = Message::Text(json.into());
    write.send(msg).await.unwrap();
}