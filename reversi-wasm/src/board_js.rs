#![allow(non_snake_case)]

use wasm_bindgen::prelude::*;
use reversi::Board;

#[allow(dead_code)]
#[wasm_bindgen]
pub fn newBoard() -> Board {
    Board {
        first: 0b_00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000,
        second: 0b_00000000_00000000_00000000_00001000_00010000_00000000_00000000_00000000,
    }
}
