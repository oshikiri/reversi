use crate::board::{Board, Player};

pub struct GameTree {
    root_board: Board,
    player: Player,
    children: Vec<GameTreeNode>,
}

impl GameTree {
    pub fn create(root_board: Board, player: Player) -> GameTree {
        GameTree {
            root_board,
            player,
            children: vec![],
        }
    }

    fn fill_children(&mut self, player: &Player) {
        for legal_move in self.root_board.get_all_legal_moves(&self.player) {
            let mut current_board: Board = self.root_board.clone();
            current_board.put_and_reverse(&self.player, legal_move);
            let child = GameTreeNode::create(player.clone(), legal_move, current_board);
            self.children.push(child);
        }
    }

    pub fn alpha_beta_pruning_search(
        &mut self,
        player: &Player,
        depth: u64,
    ) -> Option<&mut GameTreeNode> {
        self.fill_children(player);

        let mut node_max_score: Option<&mut GameTreeNode> = None;
        let mut max_score_opt: Option<f32> = None;

        for child in &mut self.children {
            let child_score = child.alpha_beta_pruning_search(depth - 1, -f32::MAX, f32::MAX);
            child.score = Some(child_score);
            match (child_score, max_score_opt) {
                (child_score, None) => max_score_opt = Some(child_score),
                (child_score, Some(max_score)) => {
                    if child_score >= max_score {
                        max_score_opt = Some(child_score);
                        node_max_score = Some(child);
                    }
                }
            };
        }
        node_max_score
    }
}

pub struct GameTreeNode {
    player: Player,
    pub put_position: u64,
    current_board: Board,
    score: Option<f32>,
    children: Vec<GameTreeNode>,
}

impl GameTreeNode {
    fn create(player: Player, put_position: u64, current_board: Board) -> GameTreeNode {
        GameTreeNode {
            player,
            put_position,
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
            let child = GameTreeNode::create(self.player.clone(), legal_move, current_board);
            self.children.push(child);
        }
    }

    fn alpha_beta_pruning_search(&mut self, depth: u64, alpha: f32, beta: f32) -> f32 {
        self.fill_children();

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
}
