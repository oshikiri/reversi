use crate::board::Board;
use crate::player::Player;
use crate::search_algorithm::base::*;

pub struct AlphaBeta {
    player: Player,
    initial_board: Board,
    max_n_leaves: usize,
    max_n_best_leaves: usize,
    n_evaluated_leaves: usize,
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
            n_evaluated_leaves: 0,
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
            return self.evaluate_leaf(player, board);
        }

        let legal_moves = self.initial_board.get_all_legal_moves(&player);

        if legal_moves.len() == 0 {
            if put_position.is_some() {
                // when there is no legal next moves and current move is non-empty, then create empty node.
                let put_position = None;
                let child_score = self.search_inner(
                    put_position,
                    &player.opponent(),
                    board,
                    remaining_depth, // NOTE: do not consume depth when passing
                    -beta,
                    -alpha,
                );
                alpha.max(-child_score)
            } else {
                // when there is no legal next moves and next move is empty, then it is a leaf node
                self.evaluate_leaf(player, board)
            }
        } else {
            // when there is at least one legal move, search children of the moves
            let mut alpha = alpha;
            for legal_move in legal_moves.iter() {
                let mut next_board = board.clone();
                next_board.put_and_reverse(&player, *legal_move);
                let child_score = self.search_inner(
                    Some(*legal_move),
                    &player.opponent(),
                    next_board,
                    remaining_depth - 1,
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

    fn evaluate_leaf(&mut self, player: &Player, board: Board) -> f32 {
        self.n_evaluated_leaves += 1;
        let leaf_score = board.score_numdisk(player.clone());
        // TODO: append to best_leaves if this leaf is better than one of best_leaves
        leaf_score
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
            - x x x x x - o
            - - x x x x x o
            o x x x o x o o
            o o o x x x o o
            o x x x x x o o
            o x x x x x o o
            o - x x x x - o
            - x x x x x x -
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
        assert_eq!(algorithm.n_evaluated_leaves, 256);
    }
}
