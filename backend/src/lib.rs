use wasm_bindgen::prelude::*;

pub mod network;
pub mod scripts;
pub mod sign;
pub mod tx;
pub mod util;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
