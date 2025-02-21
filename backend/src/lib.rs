use wasm_bindgen::prelude::*;

pub mod miniscript;
pub mod network;
pub mod sign;
pub mod util;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
