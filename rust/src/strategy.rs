use wasm_bindgen::prelude::*;

use crate::bitboard;
use crate::board::{count_bits, Board};
use crate::console_log;
use crate::player::Player;
use crate::search_algorithm::alphabeta::AlphaBeta;

#[wasm_bindgen]
#[derive(Debug)]
pub enum StrategyType {
    NumdiskLookahead,
    PatternLookahead1,
}

pub fn new_strategy() -> NumdiskLookaheadStrategy {
    NumdiskLookaheadStrategy {}
}

pub trait Strategy {
    fn get_next_move(
        &mut self,
        board: &Board,
        palyer: &Player,
        i_step: usize,
    ) -> Result<(Option<u64>, f32), String>;
}

pub struct NumdiskLookaheadStrategy {}

impl Strategy for NumdiskLookaheadStrategy {
    fn get_next_move(
        &mut self,
        board: &Board,
        player: &Player,
        i_step: usize,
    ) -> Result<(Option<u64>, f32), String> {
        let mut alphabeta =
            AlphaBeta::create(1000000000, |board: &Board, player: &Player| -> f32 {
                board.score_numdisk(player)
            });
        let root_board = match player {
            Player::First => board.clone(),
            Player::Second => Board::reverse(&board),
        };
        let depth = match i_step {
            45..=61 => 13,
            41..=44 => 9,
            _ => 7,
        };
        match alphabeta.search(root_board, depth) {
            Some((Some(best_move), score)) => Ok((Some(best_move), score)),
            _ => Err(String::from("Result of alpha-beta pruning search is empty")),
        }
    }
}

pub struct PatternLookahead1Strategy {}

impl Strategy for PatternLookahead1Strategy {
    // TODO: 高速化
    fn get_next_move(
        &mut self,
        current_board: &Board,
        player: &Player,
        _i_step: usize,
    ) -> Result<(Option<u64>, f32), String> {
        let mut scores = [-f32::MAX].repeat(64);

        for i_cell in 0..64 {
            let (current, opponent) = match player {
                Player::First => (current_board.first(), current_board.second()),
                Player::Second => (current_board.second(), current_board.first()),
            };
            let put_position = 1 << i_cell;
            let reverse_pattern =
                current_board.get_reverse_pattern(current, opponent, put_position);
            if count_bits(reverse_pattern) <= 0 {
                continue;
            }

            let mut next_board = Board::create(current_board.first(), current_board.second());
            next_board.put_and_reverse(&player, put_position);

            let pattern_instance_indices =
                bitboard::extract_pattern_instance_indices(&next_board, &player);
            scores[i_cell] = Board::calculate_pattern_score(pattern_instance_indices);
        }

        console_log!("{:?}", scores);

        match argmax_f32(&scores) {
            Some(i_max) => Ok((Some(1 << i_max), scores[i_max])),
            None => Err(String::from("reverse_counts is all zero")),
        }
    }
}

fn argmax_f32(v: &Vec<f32>) -> Option<usize> {
    let mut v_max = -f32::MAX;
    let mut i_max = 0;

    for i in 1..v.len() {
        if v[i] > v_max {
            v_max = v[i];
            i_max = i;
        }
    }
    if v_max == -f32::MAX {
        None
    } else {
        Some(i_max)
    }
}
