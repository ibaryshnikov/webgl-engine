# Demo WebGl engine

The main goal is to show that using `WebGl` and `DOM` APIs is seamless in Rust with `wasm-bindgen`<br>
It also can give some insights about the performance

## Building

```bash
wasm-pack target --web
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
