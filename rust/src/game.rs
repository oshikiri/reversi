extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use crate::bitboard;
use crate::board::*;
use crate::console_log;
use crate::player::Player;
use crate::strategy::*;

#[wasm_bindgen]
pub struct Game {
    player_human: Player,
    current_board: Board,
    history: Vec<u64>,
    opponent_strategy: Box<dyn Strategy>,
}

#[wasm_bindgen]
impl Game {
    #![allow(non_snake_case)]

    pub fn create(player_human: Player, opponent_strategy_type: StrategyType) -> Game {
        let current_board = newBoard();
        let opponent_strategy: Box<dyn Strategy> = match opponent_strategy_type {
            StrategyType::NumdiskLookahead => Box::new(NumdiskLookaheadStrategy {}),
            StrategyType::PatternLookahead1 => Box::new(PatternLookahead1Strategy {}),
        };
        Game {
            player_human,
            current_board,
            history: vec![],
            opponent_strategy,
        }
    }

    pub fn currentBoard(&self) -> Board {
        self.current_board.clone()
    }

    pub fn putAndReverse(&mut self, i: u8, j: u8) {
        let put_position = coordinate_to_bitboard(i as u64, j as u64).unwrap();
        self.current_board
            .put_and_reverse(&self.player_human, put_position);
        self.history.push(put_position);
        self.print_move(&self.player_human, put_position);
    }

    pub fn putAndReverseOpponent(&mut self) -> js_sys::Array {
        let player = self.player_human.opponent();

        let next_position_result = self
            .opponent_strategy
            .get_next_move(&self.current_board, &player);
        let result = match next_position_result {
            Ok(next_position) => {
                let (_player, put_position) =
                    self.current_board.put_and_reverse(&player, next_position);
                self.history.push(put_position);
                Ok(put_position)
            }
            Err(msg) => Err(format!("Skipped because: {}", msg)),
        };

        match result {
            Ok(put_position) => {
                self.print_move(&player, put_position);
                match bitboard::put_position_to_xy(put_position) {
                    Some((i, j)) => convert_vec_to_jsarray(vec![i, j]),
                    None => convert_vec_to_jsarray(vec![]),
                }
            }
            Err(msg) => {
                console_log!("passed (reason: {})", msg);
                convert_vec_to_jsarray(vec![])
            }
        }
    }

    pub fn getCurrentAllLegalPosition(&self, player: Player) -> js_sys::Array {
        self.current_board.getAllLegalPosition(player)
    }

    fn print_move(&self, player: &Player, put_position: u64) {
        console_log!(
            "move[{}] {} {:?}",
            self.history.len(),
            bitboard::put_position_to_coord(put_position).unwrap_or("*".to_string()),
            player,
        );
    }
}
