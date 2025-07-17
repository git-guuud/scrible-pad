function draw(stroke_width, stroke_color, stroke_points) {
    const drawing = document.getElementById("drawing");
    const ctx = drawing.getContext("2d");
    console.log("Drawing stroke:", {
        width: stroke_width,
        color: stroke_color,
        points: stroke_points
    });
    
}