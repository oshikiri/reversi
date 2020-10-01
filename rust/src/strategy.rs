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

pub struct NumdiskLookaheadMoreStrategy {
    game_tree: GameTree,
}

impl Strategy for NumdiskLookaheadMoreStrategy {
    fn get_next_move(&mut self, board: &Board, player: &Player) -> u64 {
        let depth = 3;
        let root_board: Board = board.clone();
        self.game_tree = GameTree::create(root_board, player.clone());
        let best_move = self.game_tree.alpha_beta_pruning_search(player, depth);
        best_move.unwrap().put_position
    }
}

impl NumdiskLookaheadMoreStrategy {
    #[allow(dead_code)]
    fn fill_score(&mut self) {
        for child in self.game_tree.children.iter_mut() {
            child.score = child.current_board.score_numdisk(Player::Second);
        }
    }
}

fn alpha_beta_pruning_search(node: &mut GameTreeNode, depth: u64, alpha: f32, beta: f32) -> f32 {
    node.fill_children();

    if node.has_children() && depth > 0 {
        match node.player {
            Player::First => {
                let mut alpha = alpha;
                for child in node.children.iter_mut() {
                    alpha = alpha_beta_pruning_search(child, depth - 1, alpha, beta).max(alpha);
                    if alpha >= beta {
                        break;
                    }
                }
                alpha
            }
            Player::Second => {
                let mut beta = beta;
                for child in &mut node.children.iter_mut() {
                    beta = alpha_beta_pruning_search(child, depth - 1, alpha, beta).min(beta);
                    if alpha >= beta {
                        break;
                    }
                }
                beta
            }
        }
    } else {
        node.current_board.score_numdisk(node.player.clone())
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

    fn fill_children(&mut self, player: &Player) {
        for legal_move in self.root_board.get_all_legal_moves(&self.player) {
            let mut current_board: Board = self.root_board.clone();
            current_board.put_and_reverse(&self.player, legal_move);
            let child = GameTreeNode::create(player.clone(), legal_move, current_board);
            self.children.push(child);
        }
    }

    fn alpha_beta_pruning_search(
        &mut self,
        player: &Player,
        depth: u64,
    ) -> Option<&mut GameTreeNode> {
        self.fill_children(player);

        let mut node_max_score: Option<&mut GameTreeNode> = None;
        let mut max_score = -f32::MAX;
        for child in &mut self.children {
            child.score = alpha_beta_pruning_search(child, depth - 1, -f32::MAX, f32::MAX);
            if child.score >= max_score {
                max_score = child.score;
                node_max_score = Some(child);
            }
        }
        node_max_score
    }
}

struct GameTreeNode {
    player: Player,
    put_position: u64,
    current_board: Board,
    score: f32,
    children: Vec<GameTreeNode>,
}

impl GameTreeNode {
    fn create(player: Player, put_position: u64, current_board: Board) -> GameTreeNode {
        GameTreeNode {
            player,
            put_position,
            current_board,
            score: 0.0,
            children: vec![],
        }
    }

    fn has_children(&self) -> bool {
        self.children.len() > 0
    }

    fn fill_children(&mut self) {
        for legal_move in self.current_board.get_all_legal_moves(&self.player) {
            let mut current_board: Board = self.current_board.clone();
            current_board.put_and_reverse(&self.player, legal_move);
            let child = GameTreeNode::create(self.player.clone(), legal_move, current_board);
            self.children.push(child);
        }
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
