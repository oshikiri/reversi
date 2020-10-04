extern crate wasm_bindgen;

use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::bitboard;
use crate::console_log;
use crate::parameters::parameters::PATTERN_INSTANCES;
use crate::player::Player;
use crate::strategy::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    first: u64,  // black, 先手
    second: u64, // white, 後手
}

#[wasm_bindgen]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn newBoard() -> Board {
    Board {
        first: 0b_00000000_00000000_00000000_00001000_00010000_00000000_00000000_00000000,
        second: 0b_00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000,
    }
}

#[wasm_bindgen]
impl Board {
    #![allow(non_snake_case)]

    pub fn create(first: u64, second: u64) -> Board {
        Board { first, second }
    }

    pub fn getBitboard(&self, player: Player) -> js_sys::Array {
        let bitboard = match player {
            Player::First => self.first,
            Player::Second => self.second,
        };
        let bitarray = bitboard::u64_to_bitvec(bitboard);
        convert_vec_to_jsarray(bitarray)
    }

    pub fn putAndReverse(&mut self, player: Player, i: u8, j: u8) {
        let put_position = coordinate_to_bitboard(i as u64, j as u64).unwrap();
        console_log!(
            "move {:?} {}",
            player,
            bitboard::put_position_to_coord(put_position).unwrap_or("*".to_string())
        );
        self.put_and_reverse(&player, put_position);
    }

    pub fn entireReversePatterns(&self, player: Player) -> js_sys::Array {
        let reverse_patterns = self.entire_reverse_patterns(&player);
        convert_vec_to_jsarray(reverse_patterns)
    }

    pub fn getAllLegalPosition(&self, player: Player) -> js_sys::Array {
        let legal_positions: Vec<u64> = self
            .entire_reverse_patterns(&player)
            .into_iter()
            .map(count_bits)
            .collect();
        convert_vec_to_jsarray(legal_positions)
    }

    pub fn putNextMove(&mut self, player: Player, strategy: StrategyType) -> js_sys::Array {
        let result = self.put_next_move(&player, strategy);
        match result {
            Ok(put_position) => {
                console_log!(
                    "move {:?} {}",
                    player,
                    bitboard::put_position_to_coord(put_position).unwrap_or("*".to_string())
                );
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
}

impl Board {
    pub fn first(&self) -> u64 {
        self.first
    }

    pub fn second(&self) -> u64 {
        self.second
    }

    pub fn is_full(&self) -> bool {
        (self.first | self.second) == u64::MAX
    }

    fn is_empty(&self, position: u64) -> bool {
        ((self.first | self.second) & position) == 0
    }

    pub fn put_and_reverse(&mut self, player: &Player, put_position: u64) -> (Player, u64) {
        match player {
            Player::First => {
                let reverse_pattern =
                    self.get_reverse_pattern(self.first, self.second, put_position);
                self.first ^= put_position | reverse_pattern;
                self.second ^= reverse_pattern;
            }
            Player::Second => {
                let reverse_pattern =
                    self.get_reverse_pattern(self.second, self.first, put_position);
                self.first ^= reverse_pattern;
                self.second ^= put_position | reverse_pattern;
            }
        };
        (player.clone(), put_position)
    }

    pub fn get_reverse_pattern(&self, current: u64, opponent: u64, put_position: u64) -> u64 {
        if !self.is_empty(put_position) {
            return 0;
        }

        let mut reverse_pattern = 0;
        for direction in 0..8 {
            reverse_pattern |=
                Board::get_reverse_pattern_direction(current, opponent, put_position, direction);
        }
        reverse_pattern
    }

    fn get_reverse_pattern_direction(
        current: u64,
        opponent: u64,
        put_position: u64,
        direction: u8,
    ) -> u64 {
        let mut reverse_pattern = 0;
        let mut mask = Board::transfer_board(put_position, direction);

        while mask != 0 && (mask & opponent) != 0 {
            reverse_pattern |= mask;
            mask = Board::transfer_board(mask, direction);
        }

        if mask & current == 0 {
            0
        } else {
            reverse_pattern
        }
    }

    fn transfer_board(board: u64, direction: u8) -> u64 {
        match direction {
            0 => {
                // right
                (board >> 1)
                    & 0b_01111111_01111111_01111111_01111111_01111111_01111111_01111111_01111111
            }
            1 => {
                // right-down
                (board >> 9)
                    & 0b_00000000_01111111_01111111_01111111_01111111_01111111_01111111_01111111
            }
            2 => {
                // down
                (board >> 8)
                    & 0b_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111
            }
            3 => {
                // left-down
                (board >> 7)
                    & 0b_00000000_11111110_11111110_11111110_11111110_11111110_11111110_11111110
            }
            4 => {
                // left
                (board << 1)
                    & 0b_11111110_11111110_11111110_11111110_11111110_11111110_11111110_11111110
            }
            5 => {
                // left-up
                (board << 9)
                    & 0b_11111110_11111110_11111110_11111110_11111110_11111110_11111110_00000000
            }
            6 => {
                // up
                (board << 8)
                    & 0b_11111111_11111111_11111111_11111111_11111111_11111111_11111111_00000000
            }
            7 => {
                // right-up
                (board << 7)
                    & 0b_01111111_01111111_01111111_01111111_01111111_01111111_01111111_00000000
            }
            _ => panic!("{}", direction),
        }
    }

    pub fn entire_reverse_patterns(&self, player: &Player) -> Vec<u64> {
        let (current, opponent) = match player {
            Player::First => (self.first, self.second),
            Player::Second => (self.second, self.first),
        };
        let mut reverse_patterns = Vec::new();

        for i in 0..64 {
            let put_position = 1 << i;
            let reverse_pattern = self.get_reverse_pattern(current, opponent, put_position);
            reverse_patterns.push(reverse_pattern);
        }

        reverse_patterns
    }

    pub fn put_next_move(
        &mut self,
        player: &Player,
        strategy_type: StrategyType,
    ) -> Result<u64, String> {
        use StrategyType::*;
        let mut strategy: Box<dyn Strategy> = match strategy_type {
            NumdiskLookahead => Box::new(NumdiskLookaheadStrategy {}),
            PatternLookahead1 => Box::new(PatternLookahead1Strategy {}),
        };

        match strategy.get_next_move(&*self, &player) {
            Ok(next_position) => {
                let (_player, put_position) = self.put_and_reverse(&player, next_position);
                Ok(put_position)
            }
            Err(msg) => Err(format!("Skipped because: {}", msg)),
        }
    }

    pub fn calculate_pattern_score(pattern_instance_indices: Vec<u64>) -> f32 {
        // offsets = np.hstack([[0], (3 ** np.array(n_cells_each_pattern[:10])).cumsum()])
        let offsets: [usize; 11] = [
            0, 81, 324, 1053, 3240, 9801, 16362, 22923, 29484, 88533, 147582,
        ];

        let mut total_score = 0.0;
        for (i, pattern_instance_index) in pattern_instance_indices.iter().enumerate() {
            let pattern_instance_index: usize = TryFrom::try_from(*pattern_instance_index).unwrap();
            total_score += PATTERN_INSTANCES[pattern_instance_index + offsets[i % offsets.len()]];
        }
        total_score
    }

    pub fn score_numdisk(&self, player: Player) -> f32 {
        let score = count_bits(self.first) as f32 - count_bits(self.second) as f32;
        match player {
            Player::First => score,
            Player::Second => -score,
        }
    }

    // FIXME
    pub fn get_all_legal_moves(&self, player: &Player) -> Vec<u64> {
        let (current, opponent) = match player {
            Player::First => (self.first, self.second),
            Player::Second => (self.second, self.first),
        };
        let mut legal_moves = Vec::new();

        for i in 0..64 {
            let put_position = 1 << i;
            let reverse_pattern = self.get_reverse_pattern(current, opponent, put_position);
            if reverse_pattern > 0 {
                legal_moves.push(put_position);
            }
        }

        legal_moves
    }

    pub fn create_from_str(board_str: &str) -> Board {
        let mut n_cells = 0;
        let mut first = 0;
        let mut second = 0;

        for c in String::from(board_str).chars() {
            if c == '-' || c == 'o' || c == 'x' {
                if c == 'o' {
                    first |= 1 << n_cells;
                } else if c == 'x' {
                    second |= 1 << n_cells;
                }
                n_cells = n_cells + 1;
            }
        }

        Board { first, second }
    }

    pub fn reverse(board: &Board) -> Board {
        Board {
            first: board.second,
            second: board.first,
        }
    }
}

fn generate_mask(i: u64) -> u64 {
    match i {
        1 => 0b_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101,
        2 => 0b_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011,
        3 => 0b_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111,
        4 => 0b_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111,
        5 => 0b_00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111,
        6 => 0b_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111,
        _ => panic!("i should be smaller than 7"),
    }
}

pub fn convert_vec_to_jsarray(vector: Vec<u64>) -> js_sys::Array {
    let length = vector.len() as u32;
    let jsarray = js_sys::Array::new_with_length(length);

    for i in 0..length {
        jsarray.set(i, JsValue::from_f64(vector[i as usize] as f64));
    }

    jsarray
}

pub fn count_bits(bitboard: u64) -> u64 {
    let mut bits = bitboard;
    for i in 1..=6 {
        let mask = generate_mask(i);
        bits = (bits & mask) + (bits >> (1 << i - 1) & mask);
    }
    return bits;
}

pub fn coordinate_to_bitboard(x: u64, y: u64) -> Result<u64, String> {
    if x >= 8 && y >= 8 {
        // FIXME: ||?
        Err("out of index".to_string())
    } else {
        let i = x + 8 * y;
        Ok(1 << i)
    }
}

pub fn convert_indices_to_bitboard(x: char, y: char) -> Result<u64, String> {
    let ix = match x {
        'A' => Ok(0),
        'B' => Ok(1),
        'C' => Ok(2),
        'D' => Ok(3),
        'E' => Ok(4),
        'F' => Ok(5),
        'G' => Ok(6),
        'H' => Ok(7),
        _ => Err(format!("invalid x={}", x)),
    };
    let iy = match y {
        '1' => Ok(0),
        '2' => Ok(1),
        '3' => Ok(2),
        '4' => Ok(3),
        '5' => Ok(4),
        '6' => Ok(5),
        '7' => Ok(6),
        '8' => Ok(7),
        _ => Err(format!("invalid y={}", y)),
    };
    let i = ix? + 8 * iy?;
    Ok(1 << i)
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn test_create_board_fixture() {
        let actual = Board::create_from_str(
            "
            o - - - - - - -
            - - - - - - - x
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
        ",
        );
        let expected = Board {
            first: 1,
            second: 1 << 15,
        };
        assert_eq!(actual, expected);
    }

    mod board_test {
        use crate::board::{Board, Player};

        #[test]
        fn equivalence() {
            let board1 = Board {
                first: 0,
                second: 1,
            };
            let board2 = Board {
                first: 0,
                second: 1,
            };
            let board3 = Board {
                first: 0,
                second: 4,
            };
            assert_eq!(board1, board2);
            assert_ne!(board1, board3);
        }

        #[test]
        fn is_full_should_return_false_when_board_is_empty() {
            let board = Board::create_from_str(
                "
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            assert_eq!(board.is_full(), false);
        }

        #[test]
        fn is_full_should_return_true_when_board_is_full() {
            let board = Board::create_from_str(
                "
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
            ",
            );
            assert_eq!(board.is_full(), true);
        }

        #[test]
        fn entire_reverse_patterns() {
            let board = Board::create_from_str(
                "
                o x - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            let reverse_patterns = board.entire_reverse_patterns(&Player::First);

            let mut expected = vec![0; 64];
            expected[2] = 2;

            assert_eq!(reverse_patterns, expected)
        }

        #[test]
        fn put_next_move_numdisk_lookahead_1_initial_move() {
            // https://github.com/oshikiri/reversi/pull/8
            let mut board = Board::create_from_str(
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
            let result = board.put_next_move(
                &Player::First,
                crate::strategy::StrategyType::NumdiskLookahead,
            );

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

            assert_eq!(result, Ok(1 << 20));
            assert_eq!(board, expected);
        }

        #[test]
        fn put_next_move_no_legal_move() {
            let mut board = Board::create_from_str(
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
            let result = board.put_next_move(
                &Player::First,
                crate::strategy::StrategyType::NumdiskLookahead,
            );

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

            assert_eq!(board, expected);
            assert_eq!(
                result,
                Err("Skipped because: Result of alpha_beta_pruning_search is empty".to_string())
            )
        }

        #[test]
        fn put_and_reverse_should_reverse_pieces() {
            let mut board = Board::create_from_str(
                "
                o x x - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            board.put_and_reverse(&Player::First, 8);
            let expected = Board {
                first: 15,
                second: 0,
            };
            assert_eq!(board, expected);
        }

        #[test]
        fn is_empty() {
            let board = Board::create_from_str(
                "
                o x - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            assert_eq!(board.is_empty(1), false);
            assert_eq!(board.is_empty(1 << 63), true);
        }
    }

    mod utils_test {
        use crate::board;
        #[test]
        fn count_bits_should_return_count_bits() {
            assert_eq!(board::count_bits(0), 0);
            assert_eq!(board::count_bits(u64::MAX), 64);
        }

        #[test]
        fn coordinate_to_bitboard_should_convert_notations() {
            assert_eq!(board::coordinate_to_bitboard(0, 0), Ok(1));
            assert_eq!(board::coordinate_to_bitboard(7, 7), Ok(1 << 63));
        }

        #[test]
        fn extract_pattern_instances() {
            let board = board::Board::create_from_str(
                "
                o - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - x
            ",
            );
            println!("{:?}", board);

            let bitboard_a1 = board::convert_indices_to_bitboard('A', '1');
            let bitboard_h8 = board::convert_indices_to_bitboard('H', '8');

            assert_eq!(bitboard_a1, Ok(board.first));
            assert_eq!(bitboard_h8, Ok(board.second));
        }
    }
}
