# WebAssembly-first-try

For this project, I'm using **Rust** to generate WebAssembly.

## Prerequisites

- Rust
- Node / npm (optional for 01 and 02)

## Setup

Currently, only the nightly toolchain of Rust is supporting WebAssembly:

```shell
rustup default nightly
rustup target add wasm32-unknown-unknown
```

## Notes

### Rust Nightly

Running `rustup default nightly` will let you use rust nighly by default:

- That way, for each command, you won't have to specify the toolchain like:
  - `cargo +nightly [cmd]`
- To get back to stable channel (you may have other project relying on stable channel), run:
  - `rustup default stable`
- To check which default toolchain you're on, run the following:
  - `rustup toolchain list`
