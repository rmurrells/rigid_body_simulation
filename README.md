# Rigid Body Simulation

A simple 3D rigid body simulation for various polyhedrons written in Rust. Implemented in SDL and WebAssembly.

# Instructions

Both implementations require the [standard Rust toolchain](https://www.rust-lang.org/tools/install).

## SDL

Opens the simulation in a window.

#### Requirements
* an up to date C compiler

Enter the rigid_body_sdl directory and run with:
```
cargo run --release
```

If using Windows MSVC this command should be called from the appropriate native tools command prompt:
* x86 Native Tools Command Prompt for 32-bit architecture
* x64 Native Tools Command prompt for 64-bit architecture

## WebAssembly

Creates a web server from which the simulation can be viewed through a browser.

#### Requirements
* [npm](https://www.npmjs.com/get-npm)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Enter the rigid_body_wasm directory and run:
```
wasm-pack build
```

Enter the web directory and run:
```
npm install
```

Start the web server from the web directory with:
```
npm start
```

Access the simulation locally by entering
```
localhost:30000
```
into a browser address bar.

# Controls

* Left-click and drag to move the camera
* Mouse scroll to zoom in and out
* Space - pause
* R - reset
* Escape - close the window (SDL only)
* Z - debug display mode 
