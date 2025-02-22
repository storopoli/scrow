use wasm_bindgen::prelude::*;

pub mod network;
pub mod scripts;
pub mod sign;
pub mod tx;
pub mod util;
pub mod wasm_sign;
pub mod scripts_wasm;
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
