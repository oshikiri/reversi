use wasm_bindgen::prelude::*;

use crate::board::bitboard;
use crate::board::Player;
use crate::board::*;
use crate::console_log;
use crate::strategy::*;
use crate::utils;

#[wasm_bindgen]
pub struct Game {
    player_human: Player,
    current_board: Board,
    history: Vec<u64>,
    opponent_strategy: Box<dyn Strategy>,
}

impl Game {
    fn put_and_reverse_opponent_inner(&mut self) -> Result<Option<u64>, String> {
        let player = self.player_human.opponent();
        let next_position_result =
            self.opponent_strategy
                .get_next_move(&self.current_board, &player, self.history.len());

        match next_position_result {
            Ok((best_move, _score)) => {
                let (_player, put_position) = self
                    .current_board
                    .put_and_reverse(&player, best_move.unwrap());
                self.history.push(put_position);
                Ok(best_move)
            }
            Err(msg) => Err(format!("Skipped because: {}", msg)),
        }
    }
}

#[wasm_bindgen]
impl Game {
    pub fn create(player_human: Player, opponent_strategy_type: StrategyType) -> Game {
        utils::set_panic_hook();
        let current_board = new_board();
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

    #[wasm_bindgen(js_name = currentBoard)]
    pub fn current_board(&self) -> Board {
        self.current_board.clone()
    }

    #[wasm_bindgen(js_name = putAndReverse)]
    pub fn put_and_reverse(&mut self, i: u8, j: u8) {
        let put_position = coordinate_to_bitboard(i as u64, j as u64).unwrap();
        self.current_board
            .put_and_reverse(&self.player_human, put_position);
        self.history.push(put_position);
        self.print_move(&self.player_human, put_position);
    }

    #[wasm_bindgen(js_name = putAndReverseOpponent)]
    pub fn put_and_reverse_opponent(&mut self) -> js_sys::Array {
        let player = self.player_human.opponent();
        match self.put_and_reverse_opponent_inner() {
            Ok(Some(best_move)) => {
                self.print_move(&player, best_move);
                match bitboard::put_position_to_xy(best_move) {
                    Some((i, j)) => convert_vec_to_jsarray(vec![i, j]),
                    None => convert_vec_to_jsarray(vec![]),
                }
            }
            Ok(_) => {
                console_log!("passed (reason: best_move.put_position = None)");
                convert_vec_to_jsarray(vec![])
            }
            Err(msg) => {
                console_log!("passed (reason: {})", msg);
                convert_vec_to_jsarray(vec![])
            }
        }
    }

    #[wasm_bindgen(js_name = getCurrentAllLegalPosition)]
    pub fn get_current_all_legal_position(&self, player: Player) -> js_sys::Array {
        self.current_board.get_all_legal_position(player)
    }

    fn print_move(&self, player: &Player, put_position: u64) {
        println!("{:?}", bitboard::put_position_to_coord(Some(put_position)));
        console_log!(
            "move[{}] {} {:?}",
            self.history.len(),
            bitboard::put_position_to_coord(Some(put_position)).unwrap_or("*".to_string()),
            player,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::Player;
    use crate::game::Game;
    use crate::strategy::StrategyType;

    #[test]
    fn put_next_move_numdisk_lookahead_1_initial_move() {
        // https://github.com/oshikiri/reversi/pull/8
        let mut game = Game::create(Player::Second, StrategyType::NumdiskLookahead);
        game.current_board = Board::create_from_str(
            "
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - o x - - -
            - - - x o - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
        ",
        );
        let result = game.put_and_reverse_opponent_inner();

        let expected = Board::create_from_str(
            "
            - - - - - - - -
            - - - - - - - -
            - - - - o - - -
            - - - o o - - -
            - - - x o - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
        ",
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(1 << 20));
        assert_eq!(game.current_board, expected);
    }

    #[test]
    fn put_next_move_no_legal_move() {
        let mut game = Game::create(Player::Second, StrategyType::NumdiskLookahead);
        game.current_board = Board::create_from_str(
            "
            x o - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
        ",
        );
        let result = game.put_and_reverse_opponent_inner();

        let expected = Board::create_from_str(
            "
            x o - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
        ",
        );

        assert_eq!(game.current_board, expected);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Skipped because: Result of alpha-beta pruning search is empty".to_string()
        );
    }
}
