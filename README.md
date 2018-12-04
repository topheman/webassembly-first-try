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

## Resources

- [wasm-pack](https://github.com/rustwasm/wasm-pack) / [Doc](https://rustwasm.github.io/wasm-pack/book/) - wasm workflow tool
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) / [Doc](https://rustwasm.github.io/wasm-bindgen/) - Facilitating high-level interactions between wasm modules and JavaScript
  - [Our Vision for wasm-bindgen](https://rustwasm.github.io/2018/07/02/vision-for-wasm-bindgen.html)
- [wasm-gc](https://github.com/alexcrichton/wasm-gc) - gc-sections for wasm
- [rustwasm/wasm-pack-template](https://github.com/rustwasm/wasm-pack-template) - A template for starting a rust-wasm project to be used with wasm-pack
- [@wasm-tool/wasm-pack-plugin](https://github.com/wasm-tool/wasm-pack-plugin) - Webpack plugin for Rust
- [nikgraf/webassembly-rust-course](https://github.com/nikgraf/webassembly-rust-course)
- [📺 WebAssembly by Tensor Programming](https://www.youtube.com/watch?v=yEiGVCF99tA&list=PLJbE2Yu2zumDDxkhXFwZrjxC1xCoWEVE2)
- [koute/stdweb](https://github.com/koute/stdweb) - A standard library for the client-side Web
  - [stdweb vs wasm-bindgen](https://github.com/rustwasm/team/issues/226)
