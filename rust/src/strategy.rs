use wasm_bindgen::prelude::*;

use crate::bitboard;
use crate::board::{count_bits, Board, Player};
use crate::console_log;

#[wasm_bindgen]
pub enum StrategyType {
    NumdiskLookahead1,
    PatternLookahead1,
}

pub fn new_strategy() -> NumdiskLookahead1Strategy {
    NumdiskLookahead1Strategy {}
}

pub trait Strategy {
    fn get_next_move(&self, board: &Board, palyer: &Player) -> u64;
}

pub struct NumdiskLookahead1Strategy {}

impl Strategy for NumdiskLookahead1Strategy {
    fn get_next_move(&self, board: &Board, player: &Player) -> u64 {
        let reverse_counts: Vec<u64> = board
            .entire_reverse_patterns(&player)
            .into_iter()
            .map(|cell| count_bits(cell))
            .collect();

        let i_max = positive_argmax(reverse_counts).unwrap();
        1 << i_max
    }
}

pub struct NumdiskLookaheadMoreStrategy {
    game_tree: GameTree,
}

impl Strategy for NumdiskLookaheadMoreStrategy {
    fn get_next_move(&self, board: &Board, player: &Player) -> u64 {
        let root_board: Board = board;
        self.game_tree = GameTree::create(board, player);
        let best_move = self.game_tree.get_best_move();
        best_move.unwrap().put_position
    }
}

impl NumdiskLookaheadMoreStrategy {
    fn fill_children(&self, board: &Board, player: &Player, current_node: &mut GameTreeNode) {
        for (i, reverse_pattern) in board
            .entire_reverse_patterns(&player)
            .into_iter()
            .enumerate()
        {
            if count_bits(reverse_pattern) == 0 {
                continue;
            }

            let put_position = 1 << i;
            let mut current_board = Board::create(board.first(), board.second());
            current_board.put_and_reverse(player, put_position);
            let node = GameTreeNode::create(put_position, current_board);

            current_node.children.push(node);
        }
    }

    fn fill_score(&self) {
        for child in self.game_tree.children {
            child.score = child.current_board.score_numdisk(player);
        }
    }
}

struct GameTree {
    root_board: Board,
    player: Player,
    children: Vec<GameTreeNode>,
}

impl GameTree {
    fn create(root_board: Board, player: Player) -> GameTree {
        GameTree {
            root_board,
            player,
            children: vec![],
        }
    }

    fn fill_children(&mut self, depth: u16) {
        for legal_move in self.root_board.get_all_legal_moves(self.player) {
            let current_board: Board = self.root_board.put_and_reverse(self.player, legal_move);
            let child = GameTreeNode::create(legal_move, current_board);
            self.children.push(child);
        }
    }

    fn get_best_move(&self) -> Option<GameTreeNode> {
        self.children.iter().fold(None, |m, &x| {
            m.map_or(Some(x), |mv| Some(if x.score > mv.score { x } else { mv }))
        })
    }
}

struct GameTreeNode {
    put_position: u64,
    current_board: Board,
    score: f64,
    children: Vec<GameTreeNode>,
}

impl GameTreeNode {
    fn create(put_position: u64, current_board: Board) -> GameTreeNode {
        GameTreeNode {
            put_position,
            current_board,
            score: 0.0,
            children: vec![],
        }
    }
}

pub struct PatternLookahead1Strategy {}

impl Strategy for PatternLookahead1Strategy {
    // TODO: 高速化
    fn get_next_move(&self, current_board: &Board, player: &Player) -> u64 {
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
