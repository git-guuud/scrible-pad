export function draw(jsonStroke) {
    const stroke = JSON.parse(jsonStroke);
    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");
    ctx.lineWidth = stroke.width;
    ctx.lineCap = "round";
    ctx.strokeStyle = stroke.color;
    ctx.beginPath();
    if (stroke.points.length > 0) {
        ctx.moveTo(stroke.points[0].x, stroke.points[0].y);
        for (let i = 1; i < stroke.points.length; i++) {
            ctx.lineTo(stroke.points[i].x, stroke.points[i].y);
        }
    }
    ctx.stroke();
    console.log("Drawn stroke:", jsonStroke);
}

export function log(message) {
    console.log("Log from Rust:", message);
}