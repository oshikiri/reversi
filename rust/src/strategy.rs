use wasm_bindgen::prelude::*;

use crate::board::bitboard;
use crate::board::player::Player;
use crate::board::Board;
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
        match alphabeta.search(&root_board, depth) {
            Some((Some(best_move), score)) => Ok((Some(best_move), score)),
            _ => Err(String::from("Result of alpha-beta pruning search is empty")),
        }
    }
}

pub struct PatternLookahead1Strategy {}

impl Strategy for PatternLookahead1Strategy {
    fn get_next_move(
        &mut self,
        board: &Board,
        player: &Player,
        _i_step: usize,
    ) -> Result<(Option<u64>, f32), String> {
        let mut alphabeta =
            AlphaBeta::create(1000000000, |board: &Board, player: &Player| -> f32 {
                let pattern_instance_indices =
                    bitboard::extract_pattern_instance_indices(board, player);
                Board::calculate_pattern_score(pattern_instance_indices)
            });
        let root_board = match player {
            Player::First => board.clone(),
            Player::Second => Board::reverse(&board),
        };
        match alphabeta.search(&root_board, 0) {
            Some((Some(best_move), score)) => Ok((Some(best_move), score)),
            _ => Err(String::from("Result of alpha-beta pruning search is empty")),
        }
    }
}
