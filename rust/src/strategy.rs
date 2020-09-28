use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Strategy {
    NumdiskLookahead1,
    PatternLookahead1,
}
