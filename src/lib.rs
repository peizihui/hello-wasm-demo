extern crate wasm_bindgen;

//The first line says "hey Rust, we're using a library called wasm_bindgen."
// Libraries are called "crates" in Rust,
// and we're using an external one, so we use the extern keyword.

use wasm_bindgen::prelude::*;
// This line contains a use command, which imports code from a library into your code.
// In this case, we're importing everything in the wasm_bindgen::prelude module.
// We use these features in the next section.

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}