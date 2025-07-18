# How to use
Run in windows powershell
Clone the repository 
```
git clone https://github.com/git-guuud/scrible-pad.git
```
Run the server (make sure localhost:8080 is not in use already)
```
scrible-pad/backend/backend.exe
```
In another terminal, navigate to the project directory and run:
```
npx serve scrible-pad/site
```

Or to build from source:
```
git clone https://github.com/git-guuud/scrible-pad.git
cd scrible-pad/backend
cargo run
```

Then in another terminal, navigate to the project directory and run:
```
wasm-pack build --target web
cd site
npm install ../pkg
npx serve .
```

Open your browser and go to `http://localhost:3000` or the port specified by `npx serve`.

# Scrible-Pad
Scrible-Pad is a simple drawing application built with Rust and WebAssembly. It allows users to draw on a canvas, save their drawings, and load them back later. The backend (a simple broadcasting websocket server) is written in Rust compiled to wasm, while the frontend uses HTML, CSS, and JavaScript. 
It supports basic features like drawing with different colors and widths, saving the current drawing as a PNG image, and loading previously saved drawings.
