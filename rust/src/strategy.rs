use wasm_bindgen::prelude::*;

use crate::board;
use crate::board::Board;

#[wasm_bindgen]
pub enum StrategyType {
    NumdiskLookahead1,
    PatternLookahead1,
}

pub fn new_strategy() -> NumdiskLookahead1Strategy {
  NumdiskLookahead1Strategy {}
}

pub trait Strategy {
    fn get_next_move(&self, board: &Board, is_second: bool) -> u64;
}

pub struct NumdiskLookahead1Strategy {}

impl Strategy for NumdiskLookahead1Strategy {
    fn get_next_move(&self, board: &Board, is_second: bool) -> u64 {
        let reverse_counts: Vec<u64> = board
            .entire_reverse_patterns(is_second)
            .into_iter()
            .map(|cell| board::count_bits(cell))
            .collect();

        let i_max = positive_argmax(reverse_counts).unwrap();
        1 << i_max
    }
}

fn positive_argmax(v: Vec<u64>) -> Option<usize> {
    let mut v_max = 0;
    let mut i_max = 0;

    for i in 0..v.len() {
        if v[i] > 0 && v[i] > v_max {
            v_max = v[i];
            i_max = i;
        }
    }
    if v_max == 0 {
        None
    } else {
        Some(i_max)
    }
}
