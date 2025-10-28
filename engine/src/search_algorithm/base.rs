use crate::board::bitboard::put_position_to_coord;
use crate::board::Board;
use crate::board::Player;
use crate::console_log;

#[derive(Clone, Debug, PartialEq)]
pub struct GameTreeLeaf {
    player: Player,
    score: f32,
    moves: Vec<Option<u64>>,
}

impl GameTreeLeaf {
    pub fn create(player: Player, score: f32, moves: Vec<Option<u64>>) -> GameTreeLeaf {
        GameTreeLeaf {
            player,
            score,
            moves,
        }
    }

    pub fn score(&self) -> f32 {
        self.score
    }

    pub fn moves(&self) -> Vec<Option<u64>> {
        self.moves.clone()
    }
}

pub trait SearchAlgorithm {
    fn n_evaluated_leaves(&self) -> usize;

    fn best_leaves(&self) -> Vec<GameTreeLeaf>;

    fn increment_n_evaluated_leaves(&mut self);

    fn evaluate_board(&self, board: &Board, player: &Player) -> f32;

    fn print_search_results(&self) {
        console_log!("  evaluated leaves = {}", self.n_evaluated_leaves());
        for leaf in self.best_leaves() {
            let move_strs = leaf
                .moves()
                .iter()
                .map(|m| put_position_to_coord(*m).unwrap())
                .collect::<Vec<String>>()
                .join(" ");
            console_log!("  {}: {}", leaf.score(), move_strs);
        }
    }
}
