extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

/// Import 'window.alert'
/// binds this funciton to the alert JS function
/// Wasm-Bindgen will attach the right window context and make 
/// all JavaScript pieces work properly
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Export a 'hello' function
/// Generated wrapper code will product the necessary
/// plumbing (in wast instrcutions and JS biolerplate)
/// to allow complex data (such as string, which isn't possible in pure WASM)
/// to flow seamlessesly bw boundaries
#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(format!("Hello, {}!", name).as_str());
}
