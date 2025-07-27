# Demo WebGl engine

The main goal is to show that using `WebGl` and `DOM` APIs is seamless in Rust with `wasm-bindgen`<br>
It also can give some insights about the performance


## Installation

In order to build the project `wasm-bindgen-cli` version should match the version from `Cargo.toml`

```bash
cargo install wasm-bindgen-cli --version 0.2.100
```


## Building

```bash
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir pkg --target web ./target/wasm32-unknown-unknown/debug/webgl_engine.wasm
```


## Running

```bash
# using https://crates.io/crates/https
http

# using python
python -m SimpleHTTPServer
```


## More details

There are around 3600 rotating cubes on the scene<br>
The main idea for the scene and the shaders are borrowed from
[MDN tutorial](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Creating_3D_objects_using_WebGL)
