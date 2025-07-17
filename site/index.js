import initSync, {send, receive_messages} from "./node_modules/scrible-pad/scrible_pad.js";

let painting = false;
let stroke = {
    width: 5,
    color: "black",
    points: []
}
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");


async function startPainting(e) {
    await initSync();
    ctx.beginPath();
    painting = true;
    draw(e);
}

async function stopPainting() {
    await send(JSON.stringify(stroke));
    painting = false;
    stroke.points = []; // Clear points after sending
}

function draw(e) {
    if (!painting) return;

    ctx.lineWidth = stroke.width;
    ctx.lineCap = "round";
    ctx.strokeStyle = stroke.color;
    
    ctx.lineTo(e.offsetX, e.offsetY);
    ctx.stroke();
    stroke.points.push({ x: e.offsetX, y: e.offsetY });
    ctx.beginPath();
    ctx.moveTo(e.offsetX, e.offsetY);
}

canvas.addEventListener("mousedown", startPainting);
canvas.addEventListener("mouseup", stopPainting);
canvas.addEventListener("mousemove", draw);

document.getElementById("clearButton").onclick = receive_messages;