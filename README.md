# WebAssembly-first-try

## Setup

Currently, only the nightly toolchain of Rust is supporting WebAssembly:

```shell
rustup default nightly
rustup target add wasm32-unknown-unknown
```

In the first step we'll need [`wasm-gc`](https://github.com/alexcrichton/wasm-gc) to clean up the wasm file generated.

```shell
cargo install wasm-gc
```
