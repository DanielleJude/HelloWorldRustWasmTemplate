# HelloWorldRustWasmTemplate
Hello World Rust WASM Project with Bindgen Functionality, will probably serve as a template for future projects. 

Following example in Programming WebAssembly with Rust Book.

Build
```
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/HelloWorldRustWasmTemplate.wasm --out-dir .
```
