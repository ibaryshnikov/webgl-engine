cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir pkg --target web ./target/wasm32-unknown-unknown/debug/webgl_engine.wasm
