# 02-basic-invoke-js-from-wasm

Invoke JavaScript from WebAssembly.

## Build

```shell
cargo build --target wasm32-unknown-unknown --release
wasm-gc ./target/wasm32-unknown-unknown/release/utils.wasm -o ./utils.gc.wasm
```

## Run

Serve with an http server that will add `Content-Type: application/wasm` headers to the `*.wasm` files.
