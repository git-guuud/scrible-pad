// mod client;




use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use std::sync::{OnceLock, Mutex};
use ws_stream_wasm::{WsMessage, WsMeta, WsStream};
use futures_util::{stream::{SplitSink, SplitStream, StreamExt}, SinkExt};

static GLOBAL_WRITE: OnceLock<Mutex<SplitSink<WsStream, WsMessage>>> = OnceLock::new();
static GLOBAL_READ: OnceLock<Mutex<SplitStream<WsStream>>> = OnceLock::new();


// #[derive(Serialize, Deserialize)]
// struct Stroke {
//     pub color: String,
//     pub width: f32,
//     pub points: Vec<Pos>,
// }

// #[wasm_bindgen]
// #[derive(Serialize, Deserialize)]
// struct Pos {
//     x: f32,
//     y: f32,
// }

#[wasm_bindgen(module = "/site/rust_call.js")]
extern "C" {
    pub fn addStroke(jsonStroke: String);
    pub fn addPoint(jsonPoint: String);
    pub fn log(message: String);
    pub fn clear();
}




#[wasm_bindgen]
pub async fn connect_to_websocket() {
    let (_, ws_stream) = WsMeta::connect("ws://127.0.0.1:8080", None).await.unwrap();
    let (write, read) = ws_stream.split();
    GLOBAL_WRITE.get_or_init(|| Mutex::new(write));
    GLOBAL_READ.get_or_init(|| Mutex::new(read));
}

async fn get_global_read() -> Option<std::sync::MutexGuard<'static, SplitStream<WsStream>>> {
    GLOBAL_READ.get()
        .and_then(|mutex| mutex.lock().ok())
}

async fn get_global_write() -> Option<std::sync::MutexGuard<'static, SplitSink<WsStream, WsMessage>>> {
    GLOBAL_WRITE.get()
        .and_then(|mutex| mutex.lock().ok())
}

#[wasm_bindgen]
pub async fn send(message: String) {
    // receive_messages().await; // Ensure we are ready to receive messages
    if let Some(mut stream) = get_global_write().await {
        let _ = stream.send(WsMessage::Text(message)).await;
    }
}

#[wasm_bindgen]
pub async fn receive_messages() {
    if let Some(mut stream) = get_global_read().await {
        while let Some(msg) = stream.next().await {
            match msg {
                WsMessage::Text(text) => {
                    log(format!("Received: {}", text));
                    if text == "Clear" {
                        clear();
                        continue;
                    }
                    else if text.starts_with("Stroke:") {
                        addStroke((&text[7..]).to_string()); // Remove "Stroke:" prefix
                    }
                    else {
                        addPoint(text);
                    }
                }
                _ => {
                    // Handle other message types if needed
                }
            }
        }
        log("Stopped receiving messages".to_string());
    }
}