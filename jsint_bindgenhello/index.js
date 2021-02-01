const wasm = import('./HelloWorldRustWasmTemplate');

wasm
    .then(h => h.hello("world!"))
    .catch(console.error);