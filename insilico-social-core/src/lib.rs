mod utils;
pub mod models;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub use models::academia::Academia;