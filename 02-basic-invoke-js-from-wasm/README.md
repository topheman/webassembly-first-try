# 02-basic-invoke-js-from-wasm

Invoke JavaScript from WebAssembly.

## Install

We will need [`wasm-gc`](https://github.com/alexcrichton/wasm-gc) to clean up the wasm file generated.

```shell
cargo install wasm-gc
```

## Build

```shell
cargo build --target wasm32-unknown-unknown --release
wasm-gc ./target/wasm32-unknown-unknown/release/utils.wasm -o ./utils.gc.wasm
```

## Run

Serve with an http server that will add `Content-Type: application/wasm` headers to the `*.wasm` files.

Example with [`serve`](https://www.npmjs.com/package/serve):

```shell
npx serve .
```
