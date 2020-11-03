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
}
