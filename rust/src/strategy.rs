use wasm_bindgen::prelude::*;

use crate::bitboard;
use crate::board::{count_bits, Board, Player};
use crate::console_log;
use crate::game_tree::GameTree;

#[wasm_bindgen]
#[derive(Debug)]
pub enum StrategyType {
    NumdiskLookahead1,
    NumdiskLookahead,
    PatternLookahead1,
}

pub fn new_strategy() -> NumdiskLookahead1Strategy {
    NumdiskLookahead1Strategy {}
}

pub trait Strategy {
    fn get_next_move(&mut self, board: &Board, palyer: &Player) -> u64;
}

pub struct NumdiskLookahead1Strategy {}

impl Strategy for NumdiskLookahead1Strategy {
    fn get_next_move(&mut self, board: &Board, player: &Player) -> u64 {
        let reverse_counts: Vec<u64> = board
            .entire_reverse_patterns(&player)
            .into_iter()
            .map(|cell| count_bits(cell))
            .collect();

        let i_max = positive_argmax(reverse_counts).unwrap();
        1 << i_max
    }
}

pub struct NumdiskLookaheadMoreStrategy {}

impl Strategy for NumdiskLookaheadMoreStrategy {
    fn get_next_move(&mut self, board: &Board, player: &Player) -> u64 {
        let depth = 3;
        let (player, root_board) = match player {
            Player::First => (player.clone(), board.clone()),
            Player::Second => {
                (player.opponent().clone(), Board::reverse(&board))
            }
        };
        let mut game_tree = GameTree::create(player.clone(), root_board);
        let best_move = game_tree.alpha_beta_pruning_search(depth);
        game_tree.print_tree();
        best_move.unwrap().put_position
    }
}

pub struct PatternLookahead1Strategy {}

impl Strategy for PatternLookahead1Strategy {
    // TODO: 高速化
    fn get_next_move(&mut self, current_board: &Board, player: &Player) -> u64 {
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

        let i_max = argmax_f32(scores).unwrap();
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

fn argmax_f32(v: Vec<f32>) -> Option<usize> {
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
