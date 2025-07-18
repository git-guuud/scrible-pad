import initSync, {send, receive_messages, connect_to_websocket} from "./node_modules/scrible-pad/scrible_pad.js";
import {addPoint, addStroke, clear, setPNG} from "./node_modules/scrible-pad/snippets/scrible-pad-5c7370a289053622/site/rust_call.js"

let painting = false;
let stroke = {
    width: 5,
    color: "black",
    points: []
}
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

await initSync();
connect_to_websocket().then(() => {
    console.log("Connected to WebSocket");
    receive_messages();
}).catch(err => {
    console.error("Error connecting to WebSocket:", err);
});

async function startPainting(e) {
    // ctx.beginPath();
    await send("Stroke:" + JSON.stringify(stroke));
    addStroke(JSON.stringify(stroke));
    painting = true;
    // draw(e);
}

async function stopPainting() {
    if (!painting) return;
    painting = false;
    if (stroke.points.length === 0) return; 
    let pngData = canvas.toDataURL("image/png").split(",")[1];
    // ctx.clearRect(0, 0, canvas.width, canvas.height);
    setPNG(pngData); 
    send("Load:" + pngData);
    stroke.points = []; // Clear points after sending
}

function draw(e) { // Draw locally and broadcast 
    if (!painting) return;

    // ctx.lineWidth = stroke.width;
    // ctx.lineCap = "round";
    // ctx.strokeStyle = stroke.color;
    
    // ctx.lineTo(e.offsetX, e.offsetY);
    // ctx.stroke();
    stroke.points.push({ x: e.offsetX, y: e.offsetY });
    // ctx.beginPath();
    // ctx.moveTo(e.offsetX, e.offsetY);
    send(JSON.stringify({
        x: e.offsetX,
        y: e.offsetY
    }));
    addPoint(JSON.stringify({
        x: e.offsetX,
        y: e.offsetY
    }));
}

canvas.addEventListener("mousedown", startPainting);
document.addEventListener("mouseup", stopPainting);
canvas.addEventListener("mousemove", draw);
document.getElementById("clearButton").addEventListener("click", () => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    stroke.points = []; // Clear points on clear
    clear();
    send("Clear");
});

document.getElementById("colorPicker").addEventListener("input", (e) => {
    stroke.color = e.target.value;
});

document.getElementById("widthPicker").addEventListener("input", (e) => {
    stroke.width = parseInt(e.target.value, 10);
});

document.getElementById("saveButton").addEventListener("click", () => {
    let imageData = canvas.toDataURL("image/png");

    let link = document.createElement("a");
    link.href = imageData;
    link.download = "drawing.png";
    link.click();
});

document.getElementById("loadButton").addEventListener("click", () => {
    let fileInput = document.createElement("input");
    fileInput.type = "file";
    fileInput.accept = "image/png";
    fileInput.onchange = async (e) => {
        let file = e.target.files[0];
        if (file) {
            let reader = new FileReader();
            reader.onload = (event) => {
                let img = new Image();
                img.onload = () => {
                    ctx.clearRect(0, 0, canvas.width, canvas.height);
                    ctx.drawImage(img, 0, 0);
                };
                img.src = event.target.result;
                setPNG(event.target.result.split(",")[1]); 
                send("Load:" + event.target.result.split(",")[1]); // Send base64 data to Rust
            };
            reader.readAsDataURL(file);
        }
    };
    fileInput.click();
});
receive_messages();