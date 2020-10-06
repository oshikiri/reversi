use crate::bitboard;
use crate::board::Board;
use crate::console_log;
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

    pub fn alpha_beta_pruning_search(&mut self, depth: u64) -> Option<GameTreeNode> {
        self.fill_children(self.player.clone());

        println!(
            "{:?}",
            self.children
                .iter_mut()
                .map(|c| bitboard::put_position_to_coord(c.put_position).unwrap())
                .collect::<Vec<String>>()
        );

        let mut node_max_score: Option<&mut GameTreeNode> = None;
        let mut max_score_opt: Option<f32> = None;

        for child in &mut self.children {
            println!("{:?}", bitboard::put_position_to_coord(child.put_position));
            let child_score = -child.alpha_beta_pruning_search(depth - 1, -f32::MAX, f32::MAX);
            child.score = Some(child_score);
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

        node_max_score.map(|x| x.clone())
    }

    #[allow(dead_code)]
    pub fn print_tree(&self) -> Result<(), String> {
        for child in &self.children {
            let next_move = bitboard::put_position_to_coord(child.put_position)?;
            console_log!("{:?} {:?}", next_move, child.score);
            child.print_tree(1, vec![next_move])?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct GameTreeNode {
    player: Player,
    pub put_position: Option<u64>,
    current_board: Board,
    pub score: Option<f32>,
    children: Vec<GameTreeNode>,
}

impl GameTreeNode {
    pub fn create(player: Player, put_position: u64, current_board: Board) -> GameTreeNode {
        GameTreeNode {
            player,
            put_position: Some(put_position),
            current_board,
            score: None,
            children: vec![],
        }
    }

    fn has_children(&self) -> bool {
        self.children.len() > 0
    }

    fn fill_children(&mut self) {
        for legal_move in self.current_board.get_all_legal_moves(&self.player) {
            let mut current_board: Board = self.current_board.clone();
            current_board.put_and_reverse(&self.player, legal_move);
            let child =
                GameTreeNode::create(self.player.opponent().clone(), legal_move, current_board);
            self.children.push(child);
        }
    }

    fn alpha_beta_pruning_search(&mut self, depth: u64, alpha: f32, beta: f32) -> f32 {
        self.fill_children();

        if self.children.len() == 0 {
            let empty_child = GameTreeNode {
                player: self.player.clone(),
                put_position: None,
                current_board: self.current_board.clone(),
                score: None,
                children: vec![],
            };
            self.children = vec![empty_child];
        }

        println!(
            "GameTreeNode::alpha_beta_pruning_search {:?} {:?}",
            bitboard::put_position_to_coord(self.put_position),
            self.children
                .iter_mut()
                .map(|c| bitboard::put_position_to_coord(c.put_position).unwrap_or("empty".to_string()))
                .collect::<Vec<String>>()
        );

        let score = if self.has_children() && depth > 0 {
            let mut alpha = alpha;
            for child in self.children.iter_mut() {
                alpha = alpha.max(-child.alpha_beta_pruning_search(depth - 1, -beta, -alpha));
                if alpha >= beta {
                    break;
                }
            }
            alpha
        } else {
            self.current_board.score_numdisk(self.player.clone())
        };
        self.score = Some(score);
        score
    }

    pub fn print_tree(&self, depth: usize, move_histories: Vec<String>) -> Result<(), String> {
        for child in &self.children {
            let mut move_histories = move_histories.clone();
            move_histories.push(bitboard::put_position_to_coord(child.put_position)?);
            println!("{:?} {:?}", move_histories, child.score);
            if child.has_children() {
                child.print_tree(depth + 1, move_histories)?;
            }
        }
        Ok(())
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
            let mut game_tree = GameTree::create(Player::Second, current_board);
            let best_move = game_tree.alpha_beta_pruning_search(3).unwrap();

            assert_eq!(
                bitboard::put_position_to_coord(best_move.put_position),
                Ok("a8".to_string())
            );
            assert_eq!(best_move.score, Some(2.0));
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
            let best_move = game_tree.alpha_beta_pruning_search(1).unwrap();
            game_tree.print_tree();

            // FIXME
            assert_ne!(
                bitboard::put_position_to_coord(best_move.put_position),
                Ok("a8".to_string())
            );
            assert_ne!(best_move.score, Some(2.0));
        }
    }
}
