use wasm_bindgen::prelude::*;

pub mod miniscript;
pub mod util;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
