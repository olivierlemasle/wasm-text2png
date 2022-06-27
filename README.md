# wasm-text2png

Small demo project for WebAssembly + WASI.

## Build

First, [install `cargo wasi`](https://bytecodealliance.github.io/cargo-wasi/install.html):

    $ cargo install cargo-wasi

Then, build with:

    $ cargo wasi build --release

## Run

### Wasmtime

    $ wasmtime target/wasm32-wasi/release/wasm-text2png.wasm "<message>" | display

or:

    $ wasmtime target/wasm32-wasi/release/wasm-text2png.wasm "<message>" > picture.png
