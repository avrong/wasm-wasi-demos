# WASM & WASI demos

## wasm-js
JS and Rust interop using `wasm-bindgen` and `wasm-pack`.

```
wasm-pack build
cd www/
./start.bat
```

## yew-app
Yew (React-like) framework app.

```
trunk serve
```

## wasi-hello
Hello World with WASI.

```
cargo build --target wasm32-wasi
wasmtime target/wasm32-wasi/debug/wasi-hello.wasm Stranger
```

## wasi-copy
Copy file using wasi with need to provide file access capability.

```
cargo build --target wasm32-wasi
wasmtime --dir . target/wasm32-wasi/debug/wasi-copy.wasm test1.txt test2.txt
```

## wasi-lib
A library to call from another language (e.g. Python) using `wasmtime` for python.

```
cargo wasi build
```

Copy compiled wasm into dir with python script and import it this way:
```
import wasmtime.loader
import wasi_lib

sum_12 = wasi_lib.add(1, 2)
```

## spin-http
Microservice using Spin framework.

```
spin up
```
