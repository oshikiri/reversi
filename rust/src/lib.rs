#![feature(test)]
extern crate wasm_bindgen;

#[macro_use]
extern crate lazy_static;

pub mod bitboard;
pub mod board;
mod board_reverse;
mod game;
pub mod game_tree;
pub mod ggf;
pub mod parameters;
pub mod player;
mod search_algorithm;
pub mod strategy;
mod utils;
