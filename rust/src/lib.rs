#![feature(test)]
extern crate wasm_bindgen;

#[macro_use]
extern crate lazy_static;

pub mod bitboard;
pub mod board;
pub mod board_reverse;
mod game;
pub mod ggf;
pub mod parameters;
pub mod player;
pub mod search_algorithm;
pub mod strategy;
mod utils;
