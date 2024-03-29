use crate::board::Board;
use crate::board::Player;
use crate::search_algorithm::base::*;

pub struct AlphaBeta {
    max_n_leaves: usize,
    n_evaluated_leaves: usize,
    best_leaves: Vec<GameTreeLeaf>,
    evaluate_board_func: fn(&Board, &Player) -> f32,
}

impl SearchAlgorithm for AlphaBeta {
    fn n_evaluated_leaves(&self) -> usize {
        self.n_evaluated_leaves
    }

    fn best_leaves(&self) -> Vec<GameTreeLeaf> {
        self.best_leaves.clone()
    }

    fn increment_n_evaluated_leaves(&mut self) -> () {
        self.n_evaluated_leaves += 1;
    }

    fn evaluate_board(&self, board: &Board, player: &Player) -> f32 {
        let evaluate = self.evaluate_board_func;
        evaluate(board, player)
    }
}

impl AlphaBeta {
    pub fn create(
        max_n_leaves: usize,
        evaluate_board_func: fn(&Board, &Player) -> f32,
    ) -> AlphaBeta {
        AlphaBeta {
            max_n_leaves,
            n_evaluated_leaves: 0,
            best_leaves: vec![],
            evaluate_board_func,
        }
    }

    /// This function assume the next turn is the first player (black).
    pub fn search(&mut self, initial_board: &Board, depth: u64) -> Option<(Option<u64>, f32)> {
        let legal_moves = initial_board.get_all_legal_moves(&Player::First);
        let search_results = if legal_moves.len() == 0 {
            let (child_score, mut leaf_moves) = self.search_inner(
                None,
                &Player::Second,
                initial_board,
                depth,
                -f32::MAX,
                f32::MAX,
            );
            leaf_moves.push(None);
            leaf_moves.reverse();

            let leaf = GameTreeLeaf::create(Player::First, -child_score, leaf_moves);
            self.best_leaves.push(leaf);

            Some((None, -child_score))
        } else {
            let mut node_max_score: Option<u64> = None;
            let mut max_score_opt: Option<f32> = None;
            for legal_move in legal_moves {
                let mut board = initial_board.clone();
                board.put_and_reverse(&Player::First, legal_move);

                let (child_score, mut leaf_moves) = self.search_inner(
                    Some(legal_move),
                    &Player::Second,
                    &board,
                    depth,
                    -f32::MAX,
                    f32::MAX,
                );
                leaf_moves.push(Some(legal_move));
                leaf_moves.reverse();

                let leaf = GameTreeLeaf::create(Player::First, -child_score, leaf_moves);
                self.best_leaves.push(leaf);

                match max_score_opt {
                    Some(max_score) if -child_score <= max_score => (),
                    _ => {
                        max_score_opt = Some(-child_score);
                        node_max_score = Some(legal_move);
                    }
                };
            }

            match (node_max_score, max_score_opt) {
                (Some(node), Some(score)) => Some((Some(node), score)),
                __ => None,
            }
        };

        self.best_leaves.sort_by(|l, r| {
            r.score()
                .partial_cmp(&l.score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.print_search_results();

        search_results
    }

    fn search_inner(
        &mut self,
        last_move: Option<u64>,
        player: &Player,
        board: &Board,
        remaining_depth: u64,
        alpha: f32,
        beta: f32,
    ) -> (f32, Vec<Option<u64>>) {
        if board.is_full() || remaining_depth == 0 || self.n_evaluated_leaves > self.max_n_leaves {
            let score = self.evaluate_board(board, player);
            return (score, vec![]);
        }

        let legal_moves = board.get_all_legal_moves(&player);
        let mut best_current_move: Option<u64> = None;
        let mut best_leaf_moves = vec![];

        let mut alpha = alpha;

        if legal_moves.len() == 0 {
            if last_move.is_some() {
                // when there is no legal next moves and current move is non-empty, then create empty node.
                let (child_score, current_best_move) = self.search_inner(
                    None,
                    &player.opponent(),
                    board,
                    remaining_depth, // NOTE: do not consume depth when passing
                    -beta,
                    -alpha,
                );

                if alpha < -child_score {
                    alpha = -child_score;
                    best_current_move = None;
                    best_leaf_moves = current_best_move;
                }
            } else {
                // when there is no legal next moves and next move is empty, then it is a leaf node
                alpha = self.evaluate_board(&board, player);
            }
        } else {
            // when there is at least one legal move, search children of the moves
            for legal_move in legal_moves.iter() {
                let mut next_board = board.clone();
                next_board.put_and_reverse(&player, *legal_move);

                let (child_score, current_moves) = self.search_inner(
                    Some(*legal_move),
                    &player.opponent(),
                    &next_board,
                    remaining_depth - 1,
                    -beta,
                    -alpha,
                );
                if alpha < -child_score {
                    alpha = -child_score;
                    best_current_move = Some(*legal_move);
                    best_leaf_moves = current_moves;
                }
                if alpha >= beta {
                    break;
                }
            }
        };
        best_leaf_moves.push(best_current_move);
        (alpha, best_leaf_moves)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard;
    use crate::board::bitboard::put_position_to_coord;
    use crate::search_algorithm::alphabeta::*;

    fn fixture_board() -> Board {
        // Puzzle 99 in Brian Rose, "Othello: A Minute to Learn...A Lifetime to Master"
        Board::create_from_str(
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
        )
    }

    #[test]
    fn create() {
        let search = AlphaBeta::create(10000, |board: &Board, player: &Player| -> f32 {
            board.score_numdisk(player)
        });
        assert_eq!(search.max_n_leaves, 10000);
    }

    #[test]
    fn best_leaves() {
        let search = AlphaBeta::create(10000, |board: &Board, player: &Player| -> f32 {
            board.score_numdisk(player)
        });
        let best_leaves = search.best_leaves();
        assert_eq!(best_leaves, vec![])
    }

    #[test]
    fn search_case_first_doesnt_have_legal_moves_black() {
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
        let mut alphabeta = AlphaBeta::create(10000, |board: &Board, player: &Player| -> f32 {
            board.score_numdisk(player)
        });
        let search_result = alphabeta.search(&board, 5);
        assert_eq!(search_result.is_some(), true);
        let search_result = search_result.unwrap();
        assert_eq!(search_result.0.is_none(), true);
        let actual_best_score = search_result.1;
        assert_eq!(actual_best_score, -2.0); // 33-31
        let actual_best_moves = alphabeta.best_leaves[0]
            .moves()
            .iter()
            .map(|m| put_position_to_coord(*m).unwrap())
            .collect::<Vec<String>>();
        assert_eq!(actual_best_moves, vec!["**", "a8", "b8", "a7"]);
    }

    #[test]
    fn search_case_first_doesnt_have_legal_moves_white() {
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

        // when next turn is white
        let reversed_board = Board::create(board.second(), board.first());
        let mut alphabeta = AlphaBeta::create(10000, |board: &Board, player: &Player| -> f32 {
            board.score_numdisk(player)
        });
        let search_result = alphabeta.search(&reversed_board, 5);
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
        let actual_best_moves = alphabeta.best_leaves[0]
            .moves()
            .iter()
            .map(|m| put_position_to_coord(*m).unwrap())
            .collect::<Vec<String>>();
        assert_eq!(actual_best_moves, vec!["a8", "b8", "a7"]);
    }

    #[test]
    fn search_case_puzzle99() {
        let mut alphabeta = AlphaBeta::create(10000, |board: &Board, player: &Player| -> f32 {
            board.score_numdisk(player)
        });
        let search_result = alphabeta.search(&fixture_board(), 9);
        assert_eq!(search_result.is_some(), true);
        let search_result = search_result.unwrap();
        let actual_best_move = search_result.0;
        let actual_best_score = search_result.1;

        // NOTE: there are other best moves that have the same score
        let expected_moves = vec![
            "g1", "**", "a1", "**", "b7", "**", "a2", "b2", "a8", "**", "g7", "**", "h8",
        ];

        assert_eq!(actual_best_score, 38.0);
        assert_eq!(
            bitboard::put_position_to_coord(actual_best_move),
            Ok("g1".to_string())
        );
        let actual_best_moves = alphabeta.best_leaves[0]
            .moves()
            .iter()
            .map(|m| put_position_to_coord(*m).unwrap())
            .collect::<Vec<String>>();
        assert_eq!(actual_best_moves, expected_moves);
    }
}
