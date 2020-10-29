use crate::board::Board;
use crate::player::Player;
use crate::search_algorithm::base::*;

pub struct AlphaBeta {
    player: Player,
    initial_board: Board,
    max_n_leaves: usize,
    max_n_best_leaves: usize,
    n_leaves_evaluated: usize,
    best_leaves: Vec<GameTreeLeaf>,
}

impl AlphaBeta {
    pub fn create(
        player: Player,
        initial_board: Board,
        max_n_leaves: usize,
        max_n_best_leaves: usize,
    ) -> AlphaBeta {
        AlphaBeta {
            player,
            initial_board,
            max_n_leaves,
            max_n_best_leaves,
            n_leaves_evaluated: 0,
            best_leaves: vec![],
        }
    }

    pub fn search(&mut self, remaining_depth: u64, alpha: f32, beta: f32) {
        for legal_move in self.initial_board.get_all_legal_moves(&Player::First) {
            self.search_inner(
                Some(legal_move),
                &self.player.clone(),
                self.initial_board.clone(),
                remaining_depth,
                alpha,
                beta,
            );
        }
    }

    fn search_inner(
        &mut self,
        put_position: Option<u64>,
        player: &Player,
        board: Board,
        remaining_depth: u64,
        alpha: f32,
        beta: f32,
    ) -> f32 {
        if board.is_full() || remaining_depth == 0 {
            self.n_leaves_evaluated += 1;
            return board.score_numdisk(player.clone());
        }

        let legal_moves = self.initial_board.get_all_legal_moves(&player);

        if legal_moves.len() == 0 {
            if put_position.is_some() {
                let put_position = None;
                let child_score = self.search_inner(
                    put_position,
                    &player.opponent(),
                    board,
                    remaining_depth - 1,
                    -beta,
                    -alpha,
                );
                alpha.max(-child_score)
            } else {
                self.n_leaves_evaluated += 1;
                board.score_numdisk(player.clone())
            }
        } else {
            let mut alpha = alpha;
            for legal_move in legal_moves.iter() {
                let remaining_depth_new = match put_position {
                    Some(_) => remaining_depth - 1,
                    None => remaining_depth,
                };
                let mut next_board = board.clone();
                next_board.put_and_reverse(&player, *legal_move);
                let child_score = self.search_inner(
                    Some(*legal_move),
                    &player.opponent(),
                    next_board,
                    remaining_depth_new,
                    -beta,
                    -alpha,
                );
                alpha = alpha.max(-child_score);
                if alpha >= beta {
                    break;
                }
            }
            alpha
        }
    }
}

impl SearchAlgorithm for AlphaBeta {
    fn best_leaves(&self) -> Vec<GameTreeLeaf> {
        self.best_leaves.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::search_algorithm::alphabeta::*;

    fn fixture_alphabeta() -> AlphaBeta {
        let board = Board::create_from_str(
            "
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - x o - - -
            - - - o x - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            ",
        );
        AlphaBeta::create(Player::First, board, 0, 0)
    }

    #[test]
    fn create() {
        let search = fixture_alphabeta();
        assert_eq!(search.max_n_leaves, 0);
    }

    #[test]
    fn best_leaves() {
        let search = fixture_alphabeta();
        let best_leaves = search.best_leaves();
        assert_eq!(best_leaves, vec![])
    }

    #[test]
    fn search() {
        let mut algorithm = fixture_alphabeta();
        algorithm.search(5, -f32::MAX, f32::MAX);
        assert_eq!(algorithm.best_leaves(), vec![]);
    }
}
