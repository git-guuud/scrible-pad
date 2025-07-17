mod client;
mod server;



// use core::str;

// use tokio;

// use yew::prelude::*;

// #[function_component]
// fn App() -> Html {
//     html! { "hello world" }
// }

// #[tokio::main]
// async fn main() {
    // yew::Renderer::<App>::new().render();
    // let url: &'static str = "wss://echo.websocket.events";
    // let (ws_stream, _) = connect_async(url).await.unwrap();
    // let (mut write, mut read) = ws_stream.split();
    // let msg = Message::Text("Hello WebSocket".into());
    // write.send(msg).await.unwrap();

    // while let Some(Ok(msg)) = read.next().await {
    //     if let Message::Text(text) = msg {
    //         println!("Received: {}", text);
    //     }
    // }

    
    

    // let addr = "127.0.0.1:8080";
    // tokio::spawn(client::print_next(0, "ws://127.0.0.1:8080"));
    // tokio::spawn(client::print_next(1, "ws://127.0.0.1:8080"));
    // tokio::spawn(client::print_next(2, "ws://127.0.0.1:8080"));
    // server::run(addr).await;

// }


use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub fn draw(stroke_width: f32, stroke_color: String, stroke_points: Vec<Pos>);
}

#[wasm_bindgen(start)]
fn main() {
    let stroke = Stroke {
        color: "red".to_string(),
        width: 5.0,
        points: vec![
            Pos { x: 10.0, y: 20.0 },
            Pos { x: 30.0, y: 40.0 },
            Pos { x: 50.0, y: 60.0 },
        ],
    };
    draw(stroke.width, stroke.color, stroke.points);
}