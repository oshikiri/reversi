extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::bitboard;

#[wasm_bindgen]
pub enum Strategy {
    Greedy,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Board {
    first: u64,  // black, 先手
    second: u64, // white, 後手
}

#[wasm_bindgen]
#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn newBoard() -> Board {
    Board {
        first: 0b_00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000,
        second: 0b_00000000_00000000_00000000_00001000_00010000_00000000_00000000_00000000,
    }
}

#[wasm_bindgen]
impl Board {
    #![allow(non_snake_case)]

    pub fn create(first: u64, second: u64) -> Board {
        Board { first, second }
    }

    pub fn getBitboard(&self, is_second: bool) -> js_sys::Array {
        let bitboard = match is_second {
            false => self.first,
            true => self.second,
        };
        let bitarray = bitboard::u64_to_bitvec(bitboard);
        convert_vec_to_jsarray(bitarray)
    }

    pub fn putAndReverse(&mut self, is_second: bool, i: u8, j: u8) {
        let put_position = coordinate_to_bitboard(i as u64, j as u64);
        self.put_and_reverse(is_second, put_position);
    }

    pub fn entireReversePatterns(&self, is_second: bool) -> js_sys::Array {
        let reverse_patterns = self.entire_reverse_patterns(is_second);
        convert_vec_to_jsarray(reverse_patterns)
    }

    pub fn getAllLegalPosition(&self, is_second: bool) -> js_sys::Array {
        let legal_positions: Vec<u64> = self
            .entire_reverse_patterns(is_second)
            .into_iter()
            .map(count_bits)
            .collect();
        convert_vec_to_jsarray(legal_positions)
    }

    pub fn putNextMove(&mut self, is_second: bool, strategy: Strategy) {
        self.put_next_move(is_second, strategy);
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

    pub fn put_and_reverse(&mut self, is_second: bool, put_position: u64) {
        if !is_second {
            let reverse_pattern = self.get_reverse_pattern(self.first, self.second, put_position);
            self.first ^= put_position | reverse_pattern;
            self.second ^= reverse_pattern;
        } else {
            let reverse_pattern = self.get_reverse_pattern(self.second, self.first, put_position);
            self.first ^= reverse_pattern;
            self.second ^= put_position | reverse_pattern;
        }
    }

    fn get_reverse_pattern(&self, current: u64, opponent: u64, put_position: u64) -> u64 {
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

    fn entire_reverse_patterns(&self, is_second: bool) -> Vec<u64> {
        let (current, opponent) = match is_second {
            false => (self.first, self.second),
            true => (self.second, self.first),
        };
        let mut reverse_patterns = Vec::new();

        for i in 0..64 {
            let put_position = 1 << i;
            let reverse_pattern = self.get_reverse_pattern(current, opponent, put_position);
            reverse_patterns.push(reverse_pattern);
        }

        reverse_patterns
    }

    pub fn put_next_move(&mut self, is_second: bool, strategy: Strategy) {
        use self::Strategy::*;
        match strategy {
            Greedy => self.put_next_move_greedy(is_second),
        };
    }

    fn put_next_move_greedy(&mut self, is_second: bool) {
        let reverse_counts: Vec<u64> = self
            .entire_reverse_patterns(is_second)
            .into_iter()
            .map(|cell| count_bits(cell))
            .collect();

        match positive_argmax(reverse_counts) {
            Some(i_max) => {
                let put_position = 1 << i_max;
                self.put_and_reverse(is_second, put_position);
            }
            None => {}
        }
    }
}

pub fn positive_argmax(v: Vec<u64>) -> Option<usize> {
    let mut v_max = 0;
    let mut i_max = 0;

    for i in 0..v.len() {
        if v[i] > 0 && v[i] > v_max {
            v_max = v[i];
            i_max = i;
        }
    }
    if v_max == 0 {
        None
    } else {
        Some(i_max)
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
    let jsarray = js_sys::Array::new_with_length(64);

    for i in 0..64 {
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

pub fn coordinate_to_bitboard(x: u64, y: u64) -> u64 {
    if x >= 8 && y >= 8 {
        panic!("out of index");
    }
    let i = x + 8 * y;
    1 << (63 - i)
}

pub fn convert_indices_to_bitboard(x: char, y: char) -> u64 {
    let ix = match x {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        _ => panic!("invalid x={}", x), // TODO
    };
    let iy = match y {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => panic!("invalid y={}", y), // TODO
    };
    let i = ix + 8 * iy;
    1 << i
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    fn create_board_fixture(board_str: &str) -> Board {
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

    #[test]
    fn test_create_board_fixture() {
        let actual = create_board_fixture(
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
        use super::create_board_fixture;
        use crate::board::Board;

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
            let board = create_board_fixture(
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
            let board = create_board_fixture(
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
            let board = create_board_fixture(
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
            let reverse_patterns = board.entire_reverse_patterns(false);

            let mut expected = vec![0; 64];
            expected[2] = 2;

            assert_eq!(reverse_patterns, expected)
        }

        #[test]
        fn put_next_move_greedy_bug_0_0() {
            // https://github.com/oshikiri/reversi/pull/7
            let mut board = create_board_fixture(
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
            board.put_next_move_greedy(true);

            let expected = create_board_fixture(
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

            assert_eq!(board, expected);
        }

        #[test]
        fn put_next_move_greedy_initial_move() {
            // https://github.com/oshikiri/reversi/pull/8
            let mut board = create_board_fixture(
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
            board.put_next_move_greedy(false);

            let expected = create_board_fixture(
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

            assert_eq!(board, expected);
        }

        #[test]
        fn put_and_reverse_should_reverse_pieces() {
            let mut board = create_board_fixture(
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
            board.put_and_reverse(false, 8);
            let expected = Board {
                first: 15,
                second: 0,
            };
            assert_eq!(board, expected);
        }

        #[test]
        fn is_empty() {
            let board = create_board_fixture(
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
        use super::create_board_fixture;
        use crate::board;
        #[test]
        fn count_bits_should_return_count_bits() {
            assert_eq!(board::count_bits(0), 0);
            assert_eq!(board::count_bits(u64::MAX), 64);
        }

        #[test]
        fn coordinate_to_bitboard_should_convert_notations() {
            assert_eq!(board::coordinate_to_bitboard(0, 0), 1 << 63);
            assert_eq!(board::coordinate_to_bitboard(7, 7), 1);
        }

        #[test]
        fn extract_pattern_instances() {
            let board = create_board_fixture(
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

            assert_eq!(bitboard_a1, board.first);
            assert_eq!(bitboard_h8, board.second);
        }
    }
}
