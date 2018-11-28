# 01-basic-invoke-wasm-from-js

Invoke WebAssembly from JavaScript.

Calling Rust `sum(x, y)` function exposed via WebAssembly, from JavaScript.

## Build

```shell
cargo build --target wasm32-unknown-unknown --release
wasm-gc ./target/wasm32-unknown-unknown/release/utils.wasm -o ./utils.gc.wasm
```

## Run

Serve with an http server that will add `Content-Type: application/wasm` headers to the `*.wasm` files.
