use crate::player::Player;

#[derive(Clone, Debug, PartialEq)]
pub struct GameTreeLeaf {
    player: Player,
    score: f32,
    moves: Vec<u64>,
}

impl GameTreeLeaf {
    pub fn create(player: Player, score: f32, moves: Vec<u64>) -> GameTreeLeaf {
        GameTreeLeaf {
            player,
            score,
            moves,
        }
    }

    pub fn score(&self) -> f32 {
        self.score
    }
}

pub trait SearchAlgorithm {
    fn best_leaves(&self) -> Vec<GameTreeLeaf>;
}
