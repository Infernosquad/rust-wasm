# WASM example built on top of the Rust

Tutorial from: https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm

## Installation guide

Install wasm pack:

```bash
cargo install wasm-pack
```

Compile the Rust code to the WebAssembly:

```bash
wasm-pack build --target bundler
```

Install npm dependencies:

```bash
npm install
```

Start the server:

```bash
npm run serve
```
