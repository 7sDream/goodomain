# Goodomain WASM Binding

## Build

You needs Rust's `wasm32-wasm-wasm` target.

Then, compile with:

```rust
cargo build -p goodomain-wasm --target wasm32-unknown-unknown --release
```

You can find the wasm file in `/target/release/goodomain.wasm`.

For use it in browser, install `wasm-bindgen-cli` or `wasm-pack` and read their document.

Or you can refer to `/web/app/README.md`, it uses this wasm module.
