use crate::bitboard::put_position_to_coord;
use crate::board::Board;
use crate::console_log;
use crate::player::Player;

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

    pub fn set_score(&mut self, new_score: f32) {
        self.score = new_score;
    }

    pub fn moves(&self) -> Vec<Option<u64>> {
        self.moves.clone()
    }
}

pub trait SearchAlgorithm {
    fn best_leaves(&self) -> Vec<GameTreeLeaf>;

    fn increment_n_evaluated_leaves(&mut self) -> ();

    fn evaluate_board(&self, board: &Board, player: &Player) -> f32;

    fn evaluate_leaf(
        &mut self,
        player: &Player,
        board: &Board,
        put_positions: Vec<Option<u64>>,
    ) -> GameTreeLeaf {
        self.increment_n_evaluated_leaves();
        let leaf_score = self.evaluate_board(board, player);
        let new_leaf = GameTreeLeaf::create(player.clone(), leaf_score, put_positions);
        new_leaf
    }

    fn print_best_leaves(&self) {
        for leaf in self.best_leaves() {
            let move_strs = leaf
                .moves()
                .iter()
                .map(|m| put_position_to_coord(*m).unwrap())
                .collect::<Vec<String>>()
                .join(" ");
            console_log!("{}: {}", leaf.score(), move_strs);
        }
    }
}
