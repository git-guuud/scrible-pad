// mod client;




use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use std::sync::{OnceLock, Mutex};
use ws_stream_wasm::{WsMessage, WsMeta, WsStream};
use futures_util::{stream::StreamExt, SinkExt};

static GLOBAL_WS: OnceLock<Mutex<WsStream>> = OnceLock::new();

#[wasm_bindgen(start)]
async fn connect_to_websocket() {
    let (_, ws_stream) = WsMeta::connect("ws://127.0.0.1:8080", None).await.unwrap();
    GLOBAL_WS.get_or_init(|| Mutex::new(ws_stream));
}

async fn get_global_ws_stream() -> Option<std::sync::MutexGuard<'static, WsStream>> {
    GLOBAL_WS.get()
        .and_then(|mutex| mutex.lock().ok())
}


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

#[wasm_bindgen]
pub async fn send(message: String) {
    // receive_messages().await; // Ensure we are ready to receive messages
    if let Some(mut stream) = get_global_ws_stream().await {
        let _ = stream.send(WsMessage::Text(message)).await;
    }
}

#[wasm_bindgen]
pub async fn receive_messages() {
    if let Some(mut stream) = get_global_ws_stream().await {
        while let Some(msg) = stream.next().await {
            match msg {
                WsMessage::Text(text) => {
                    log(format!("Received: {}", text));
                    draw(text);
                }
                _ => {
                    // Handle other message types if needed
                }
            }
        }
        log("Stopped receiving messages".to_string());
    }
}