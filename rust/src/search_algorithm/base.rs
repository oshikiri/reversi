use crate::player::Player;

#[derive(Clone, Debug, PartialEq)]
pub struct GameTreeLeaf {
    player: Player,
    score: f64,
    moves: Vec<u64>,
}

impl GameTreeLeaf {
    pub fn create(player: Player, score: f64, moves: Vec<u64>) -> GameTreeLeaf {
        GameTreeLeaf {
            player,
            score,
            moves,
        }
    }
}

pub trait SearchAlgorithm {
    fn best_leaves(&self) -> Vec<GameTreeLeaf>;
}
