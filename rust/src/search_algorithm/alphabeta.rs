use crate::board::Board;
use crate::player::Player;
use crate::search_algorithm::base::*;

// 次は先手番だとしてalphabeta探索をする。
// 後手番で読みたい場合は、bitboardを入れ替える前処理を噛ませてから実行する。
pub struct AlphaBeta {
    initial_board: Board,
    max_n_leaves: usize,
    max_n_best_leaves: usize,
    n_evaluated_leaves: usize,
    best_leaves: Vec<GameTreeLeaf>,
}

impl AlphaBeta {
    pub fn create(
        initial_board: Board,
        max_n_leaves: usize,
        max_n_best_leaves: usize,
    ) -> AlphaBeta {
        AlphaBeta {
            initial_board,
            max_n_leaves,
            max_n_best_leaves,
            n_evaluated_leaves: 0,
            best_leaves: vec![],
        }
    }

    pub fn search(&mut self, remaining_depth: u64) {
        for legal_move in self.initial_board.get_all_legal_moves(&Player::First) {
            let mut board = self.initial_board.clone();
            board.put_and_reverse(&Player::First, legal_move);

            self.search_inner(
                vec![Some(legal_move)],
                &Player::Second,
                board,
                remaining_depth,
                -f32::MAX,
                f32::MAX,
            );
        }
    }

    fn search_inner(
        &mut self,
        put_positions: Vec<Option<u64>>,
        player: &Player,
        board: Board,
        remaining_depth: u64,
        alpha: f32,
        beta: f32,
    ) -> f32 {
        let put_position = put_positions.last().unwrap();

        if board.is_full() || remaining_depth == 0 {
            return self.evaluate_leaf(&Player::First, board, put_positions);
        }

        let legal_moves = board.get_all_legal_moves(&player);

        if legal_moves.len() == 0 {
            if put_position.is_some() {
                // when there is no legal next moves and current move is non-empty, then create empty node.
                let mut put_positions = put_positions.clone();
                put_positions.push(None);

                let child_score = self.search_inner(
                    put_positions,
                    &player.opponent(),
                    board,
                    remaining_depth, // NOTE: do not consume depth when passing
                    -beta,
                    -alpha,
                );
                alpha.max(-child_score)
            } else {
                // when there is no legal next moves and next move is empty, then it is a leaf node
                self.evaluate_leaf(&Player::First, board, put_positions)
            }
        } else {
            // when there is at least one legal move, search children of the moves
            let mut alpha = alpha;
            for legal_move in legal_moves.iter() {
                let mut put_positions = put_positions.clone();
                put_positions.push(Some(*legal_move));

                let mut next_board = board.clone();
                next_board.put_and_reverse(&player, *legal_move);

                let child_score = self.search_inner(
                    put_positions,
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

    fn evaluate_leaf(
        &mut self,
        player: &Player,
        board: Board,
        put_positions: Vec<Option<u64>>,
    ) -> f32 {
        self.n_evaluated_leaves += 1;
        let leaf_score = board.score_numdisk(player.clone());
        let new_leaf = GameTreeLeaf::create(player.clone(), leaf_score, put_positions);

        // FIXME: cache best_move_min_opt
        let best_move_min_opt: Option<&GameTreeLeaf> = self.best_leaves.iter().min_by(|l, r| {
            l.score()
                .partial_cmp(&r.score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut best_leaves = self.best_leaves.clone();
        best_leaves.push(new_leaf);

        match best_move_min_opt {
            Some(best_move_min) => {
                if self.best_leaves.len() > self.max_n_best_leaves
                    && best_move_min.score() < leaf_score
                {
                    best_leaves.sort_by(|l, r| {
                        r.score()
                            .partial_cmp(&l.score())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    let (head, _tail) = best_leaves.split_at(self.max_n_best_leaves);
                    best_leaves = head.to_vec();
                }
            }
            None => {}
        }

        self.best_leaves = best_leaves;
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
        AlphaBeta::create(board, 10000, 5)
    }

    #[test]
    fn create() {
        let search = fixture_alphabeta();
        assert_eq!(search.max_n_leaves, 10000);
    }

    #[test]
    fn best_leaves() {
        let search = fixture_alphabeta();
        let best_leaves = search.best_leaves();
        assert_eq!(best_leaves, vec![])
    }

    #[test]
    fn search() {
        use crate::bitboard::put_position_to_coord;

        let mut algorithm = fixture_alphabeta();
        algorithm.search(9);

        let best_leaf = &algorithm.best_leaves()[0];
        let expected_best_score = 38.0;
        let actual_moves = best_leaf
            .moves()
            .iter()
            .map(|p| put_position_to_coord(*p).unwrap())
            .collect::<Vec<String>>();
        // NOTE: there are other best moves that have the same score
        let expected_moves = vec![
            "g1", "passed", "a1", "passed", "b7", "passed", "a2", "b2", "a8", "passed", "g7",
            "passed", "h8",
        ];

        assert_eq!(best_leaf.score(), expected_best_score);
        assert_eq!(actual_moves, expected_moves);
        assert_eq!(algorithm.n_evaluated_leaves, 120);
    }
}
