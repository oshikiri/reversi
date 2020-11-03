use wasm_bindgen::prelude::*;

use crate::bitboard;
use crate::board::{count_bits, Board};
use crate::console_log;
use crate::player::Player;
use crate::search_algorithm::alphabeta::AlphaBeta;
use crate::search_algorithm::base::GameTreeLeaf;

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
    // TODO: 高速化
    fn get_next_move(
        &mut self,
        board: &Board,
        player: &Player,
        _i_step: usize,
    ) -> Result<(Option<u64>, f32), String> {
        let root_board = match player {
            Player::First => board.clone(),
            Player::Second => Board::reverse(&board),
        };

        let mut leaves: Vec<GameTreeLeaf> = Vec::new();
        for i_cell in 0..64 {
            let put_position = 1 << i_cell;
            let reverse_pattern =
                board.get_reverse_pattern(root_board.first(), root_board.second(), put_position);
            if count_bits(reverse_pattern) <= 0 {
                continue;
            }

            let mut next_board = root_board.clone();
            next_board.put_and_reverse(&player, put_position);

            let pattern_instance_indices =
                bitboard::extract_pattern_instance_indices(&next_board, &player);
            let pattern_score = Board::calculate_pattern_score(pattern_instance_indices);
            let leaf =
                GameTreeLeaf::create(player.clone(), pattern_score, vec![Some(put_position)]);
            leaves.push(leaf);
        }

        leaves.sort_by(|l, r| {
            r.score()
                .partial_cmp(&l.score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        console_log!("  leaves: {:?}", leaves);

        if leaves.is_empty() {
            Err(String::from("leaves is empty"))
        } else {
            let best_leaf = leaves[0].clone();
            Ok((best_leaf.moves()[0], best_leaf.score()))
        }
    }
}
