use crate::board::Board;
use crate::player::Player;

pub struct GameTree {
    root_board: Board,
    player: Player,
    children: Vec<GameTreeNode>,
}

impl GameTree {
    pub fn create(player: Player, root_board: Board) -> GameTree {
        GameTree {
            root_board,
            player,
            children: vec![],
        }
    }

    fn fill_children(&mut self, player: Player) {
        for legal_move in self.root_board.get_all_legal_moves(&player) {
            let mut current_board: Board = self.root_board.clone();
            current_board.put_and_reverse(&player, legal_move);
            let child = GameTreeNode::create(player.opponent().clone(), legal_move, current_board);
            self.children.push(child);
        }
    }

    pub fn alpha_beta_pruning_search(&mut self, depth: u64) -> Option<(GameTreeNode, f32)> {
        self.fill_children(self.player.clone());

        if self.children.len() == 0 {
            let empty_child = GameTreeNode {
                player: self.player.opponent().clone(),
                put_position: None,
                current_board: self.root_board.clone(),
            };
            self.children = vec![empty_child];
        }

        let mut node_max_score: Option<&mut GameTreeNode> = None;
        let mut max_score_opt: Option<f32> = None;

        for child in &mut self.children {
            let child_score = -child.alpha_beta_pruning_search(depth - 1, -f32::MAX, f32::MAX);
            match (child_score, max_score_opt) {
                (child_score, None) => {
                    max_score_opt = Some(child_score);
                    node_max_score = Some(child);
                }
                (child_score, Some(max_score)) => {
                    if child_score > max_score {
                        max_score_opt = Some(child_score);
                        node_max_score = Some(child);
                    }
                }
            };
        }

        match (node_max_score, max_score_opt) {
            (Some(node), Some(score)) => Some((node.clone(), score)),
            __ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameTreeNode {
    player: Player,
    pub put_position: Option<u64>,
    current_board: Board,
}

impl GameTreeNode {
    pub fn create(player: Player, put_position: u64, current_board: Board) -> GameTreeNode {
        GameTreeNode {
            player,
            put_position: Some(put_position),
            current_board,
        }
    }

    fn get_children(&mut self) -> Vec<GameTreeNode> {
        let mut children = vec![];

        for legal_move in self.current_board.get_all_legal_moves(&self.player) {
            let mut current_board: Board = self.current_board.clone();
            current_board.put_and_reverse(&self.player, legal_move);
            let child =
                GameTreeNode::create(self.player.opponent().clone(), legal_move, current_board);
            children.push(child);
        }

        children
    }

    fn alpha_beta_pruning_search(&mut self, depth: u64, alpha: f32, beta: f32) -> f32 {
        if self.current_board.is_full() {
            return self.current_board.score_numdisk(self.player.clone());
        }

        let mut children = self.get_children();

        if children.len() == 0 && self.put_position.is_some() {
            let empty_child = GameTreeNode {
                player: self.player.opponent().clone(),
                put_position: None,
                current_board: self.current_board.clone(),
            };
            children = vec![empty_child];
        }

        if children.len() > 0 && depth > 0 {
            let mut alpha = alpha;
            for child in children.iter_mut() {
                let depth_new = match self.put_position {
                    Some(_) => depth - 1,
                    None => depth,
                };
                alpha = alpha.max(-child.alpha_beta_pruning_search(depth_new, -beta, -alpha));
                if alpha >= beta {
                    break;
                }
            }
            alpha
        } else {
            self.current_board.score_numdisk(self.player.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    mod game_tree_test {
        use crate::bitboard;
        use crate::board::Board;
        use crate::game_tree::GameTree;
        use crate::player::Player;

        #[test]
        fn alpha_beta_pruning_search() {
            // Diagram 13-10 in Brian Rose, "Othello: A Minute to Learn...A Lifetime to Master"
            let current_board = Board::create_from_str(
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
            let mut game_tree = GameTree::create(Player::First, current_board.clone());
            let (best_move, score) = game_tree.alpha_beta_pruning_search(5).unwrap();
            assert_eq!(best_move.put_position, None);
            assert_eq!(score, -2.0);

            // when next turn is white
            let mut game_tree = GameTree::create(Player::Second, current_board.clone());
            let (best_move, score) = game_tree.alpha_beta_pruning_search(3).unwrap();
            assert_eq!(
                bitboard::put_position_to_coord(best_move.put_position),
                Ok("a8".to_string())
            );
            assert_eq!(score, 2.0);
        }

        #[test]
        fn alpha_beta_pruning_search_pass() {
            // Puzzle 99 in Brian Rose, "Othello: A Minute to Learn...A Lifetime to Master"
            let current_board = Board::create_from_str(
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
            let mut game_tree = GameTree::create(Player::First, current_board);
            let (best_move, score) = game_tree.alpha_beta_pruning_search(9).unwrap();

            // NOTE: b7 is also one of the best moves
            assert_eq!(
                bitboard::put_position_to_coord(best_move.put_position),
                Ok("g1".to_string())
            );
            assert_eq!(score, 38.0);
        }
    }

    mod benches {
        extern crate test;
        use test::Bencher;

        use crate::board::Board;
        use crate::game_tree::{GameTree, GameTreeNode};
        use crate::player::Player;

        #[bench]
        fn get_children(bench: &mut Bencher) {
            let current_board = Board::create_from_str(
                "
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - o o o - - -
                - - - o x - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            let mut node = GameTreeNode::create(Player::Second, 1 << 26, current_board);

            bench.iter(|| {
                node.get_children();
            })
        }

        #[bench]
        fn alpha_beta_pruning_search(bench: &mut Bencher) {
            let current_board = Board::create_from_str(
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
            bench.iter(|| {
                let mut game_tree = GameTree::create(Player::First, current_board.clone());
                let _best_move = game_tree.alpha_beta_pruning_search(8).unwrap();
            })
        }
    }
}
