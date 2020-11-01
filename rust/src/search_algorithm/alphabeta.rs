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

    pub fn search(&mut self, remaining_depth: u64) -> Option<(Option<u64>, f32)> {
        let legal_moves = self.initial_board.get_all_legal_moves(&Player::First);
        if legal_moves.len() == 0 {
            let child_score = self.search_inner(
                vec![None],
                &Player::Second,
                self.initial_board.clone(),
                remaining_depth,
                -f32::MAX,
                f32::MAX,
            );

            Some((None, child_score))
        } else {
            let mut node_max_score: Option<u64> = None;
            let mut max_score_opt: Option<f32> = None;
            for legal_move in legal_moves {
                let mut board = self.initial_board.clone();
                board.put_and_reverse(&Player::First, legal_move);

                let child_score = -self.search_inner(
                    vec![Some(legal_move)],
                    &Player::Second,
                    board,
                    remaining_depth,
                    -f32::MAX,
                    f32::MAX,
                );

                println!("child_score={}, max_score_opt={:?}", child_score, max_score_opt);

                match (child_score, max_score_opt) {
                    (child_score, None) => {
                        max_score_opt = Some(child_score);
                        node_max_score = Some(legal_move);
                    }
                    (child_score, Some(max_score)) => {
                        if child_score > max_score {
                            max_score_opt = Some(child_score);
                            node_max_score = Some(legal_move);
                        }
                    }
                };
            }

            match (node_max_score, max_score_opt) {
                (Some(node), Some(score)) => Some((Some(node), score)),
                __ => None,
            }
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
        if board.is_full() || remaining_depth == 0 {
            return self.evaluate_leaf(player, board, put_positions.clone());
        }

        let legal_moves = board.get_all_legal_moves(&player);

        if legal_moves.len() == 0 {
            let last_move = put_positions.last().unwrap();
            if last_move.is_some() {
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
                self.evaluate_leaf(player, board, put_positions)
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

        let mut best_leaves = self.best_leaves.clone();
        best_leaves.push(new_leaf);

        // FIXME: efficiency
        let best_move_min_opt: Option<&GameTreeLeaf> = self.best_leaves.iter().min_by(|l, r| {
            l.score()
                .partial_cmp(&r.score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if best_move_min_opt.is_some() {
            if best_leaves.len() > self.max_n_best_leaves {
                best_leaves.sort_by(|l, r| {
                    r.score()
                        .partial_cmp(&l.score())
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                let (head, _tail) = best_leaves.split_at(self.max_n_best_leaves);
                best_leaves = head.to_vec();
            }
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
    use crate::bitboard;
    use crate::search_algorithm::alphabeta::*;

    fn fixture_alphabeta() -> AlphaBeta {
        // Puzzle 99 in Brian Rose, "Othello: A Minute to Learn...A Lifetime to Master"
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
    fn search_case_first_doesnt_have_legal_moves() {
        // Diagram 13-10 in Brian Rose, "Othello: A Minute to Learn...A Lifetime to Master"
        let board = Board::create_from_str(
            "
            o o o o o o o o
            o o o o o x x o
            o x x o x x x o
            o x o x o x x o
            o o o o x x x o
            o o o x x x x o
            - o o x o o o o
            - - o x x x x x
            ",
        );

        // when next turn is black
        let mut alphabeta = AlphaBeta::create(board.clone(), 10000, 5);
        let search_result = alphabeta.search(5);
        assert_eq!(search_result.is_some(), true);
        let search_result = search_result.unwrap();
        assert_eq!(search_result.0.is_none(), true);
        // let actual_best_score = search_result.1;
        // assert_eq!(actual_best_score, -2.0); // 33-31 FIXME

        // when next turn is white
        let reversed_board = Board::create(board.second(), board.first());
        let mut alphabeta = AlphaBeta::create(reversed_board, 10000, 5);
        let search_result = alphabeta.search(5);
        assert_eq!(search_result.is_some(), true);
        let search_result = search_result.unwrap();
        assert_eq!(search_result.0.is_some(), true);
        let actual_best_move = search_result.0;
        let actual_best_score = search_result.1;
        assert_eq!(
            bitboard::put_position_to_coord(actual_best_move),
            Ok("a8".to_string())
        );
        assert_eq!(actual_best_score, 2.0); // 33-31
    }

    #[test]
    fn search_case_puzzle99() {
        let mut algorithm = fixture_alphabeta();
        let search_result = algorithm.search(9);
        assert_eq!(search_result.is_some(), true);
        let search_result = search_result.unwrap();
        let actual_best_move = search_result.0;
        let actual_best_score = search_result.1;

        // NOTE: there are other best moves that have the same score
        let _expected_moves = vec![
            "g1", "passed", "a1", "passed", "b7", "passed", "a2", "b2", "a8", "passed", "g7",
            "passed", "h8",
        ];

        assert_eq!(actual_best_score, 38.0);
        assert_eq!(
            bitboard::put_position_to_coord(actual_best_move),
            Ok("g1".to_string())
        );
    }
}
