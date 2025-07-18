let strokes = [];
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

export function addStroke(jsonStroke) {
    const stroke = JSON.parse(jsonStroke);
    ctx.clearRect(0, 0, canvas.width, canvas.height);


    strokes.push(stroke);
    for (const s of strokes) {
        ctx.lineCap = "round";
        ctx.strokeStyle = s.color;
        ctx.lineWidth = s.width;
        ctx.beginPath();
        if (s.points.length > 0) {
            ctx.moveTo(s.points[0].x, s.points[0].y);
            for (let i = 1; i < s.points.length; i++) {
                ctx.lineTo(s.points[i].x, s.points[i].y);
                ctx.stroke();
                ctx.beginPath();
                ctx.moveTo(s.points[i].x, s.points[i].y);
            }
        }
    }
}

export function addPoint(jsonPoint) {
    const { x, y } = JSON.parse(jsonPoint);
    ctx.lineTo(x, y);
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(x, y);
    strokes[strokes.length - 1].points.push({ x, y });
}

export function clear() {
    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    strokes = [];
    lastStrokeId = 0;
    console.log("Canvas cleared");
}

export function log(message) {
    console.log("Log from Rust:", message);
}

export function getLastStrokeId() {
    return lastStrokeId;
}   