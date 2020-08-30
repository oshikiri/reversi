extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Board {
    first: u64,  // black, 先手
    second: u64, // white, 後手
}

#[wasm_bindgen]
pub fn newBoard() -> Board {
    Board {
        first: 0b_00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000,
        second: 0b_00000000_00000000_00000000_00001000_00010000_00000000_00000000_00000000,
    }
}

#[wasm_bindgen]
impl Board {
    #![allow(non_snake_case)]

    pub fn getBitboard(&self, is_second: bool) -> js_sys::Array {
        let bitboard = match is_second {
            false => self.first,
            true => self.second,
        };
        let bitarray = convert_bitboard_to_array(bitboard);
        convert_vec_to_jsarray(bitarray)
    }

    pub fn putAndReverse(&mut self, is_second: bool, i: u8, j: u8) {
        let put_position = convert_indices_to_bitboard(i as u64, j as u64);
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

    pub fn putNextMove(&mut self, is_second: bool, strategy: u8) {
        self.put_next_move(is_second, strategy);
    }
}

impl Board {
    pub fn print_board(&self) {
        println!("print");
        let mut first = self.first;
        let mut second = self.second;

        while first != 0 || second != 0 {
            if first & 1 == 1 {
                print!("o");
            } else if second & 1 == 1 {
                print!("x")
            } else {
                print!("-")
            }

            first >>= 1;
            second >>= 1;
        }
    }

    pub fn is_full(&self) -> bool {
        (self.first | self.second) == u64::MAX
    }

    fn is_empty(&self, position: u64) -> bool {
        ((self.first | self.second) & position) == 0
    }

    pub fn is_valid(&self) -> bool {
        true
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
        reverse_patterns.reverse();

        reverse_patterns
    }

    pub fn get_all_legal_position(&self, is_second: bool) -> Vec<PositionEvaluation> {
        let mut legal_positions = Vec::new();
        let entire_reverse_patterns = self.entire_reverse_patterns(is_second);

        for k in 0..64 {
            let i = k % 8;
            let j = k / 8;
            let reverse_pattern = entire_reverse_patterns[k];
            if reverse_pattern > 0 {
                let position_evaluation = PositionEvaluation {
                    i,
                    j,
                    evaluation: count_bits(reverse_pattern),
                };
                legal_positions.push(position_evaluation);
            }
        }

        legal_positions
    }

    pub fn put_next_move(&mut self, is_second: bool, strategy: u8) {
        match strategy {
            0 => self.put_next_move_greedy(is_second),
            _ => panic!("Invalid strategy: {}", strategy),
        };
    }

    fn put_next_move_greedy(&mut self, is_second: bool) {
        let reverse_counts: Vec<u64> = self
            .entire_reverse_patterns(is_second)
            .into_iter()
            .map(|cell| count_bits(cell))
            .collect();

        let mut non_zero_counts = Vec::new();
        for k in 0..64 {
            if reverse_counts[k] > 0 {
                non_zero_counts.push(reverse_counts[k])
            }
        }
        let i_max = argmax(reverse_counts);
        let put_position = 1 << (63 - i_max);
        self.put_and_reverse(is_second, put_position);
    }
}

pub fn argmax(v: Vec<u64>) -> usize {
    let n = v.len();
    let mut v_max = 0;
    let mut i_max = 0;
    for i in 0..n {
        if v[i] > v_max {
            v_max = v[i];
            i_max = i;
        }
    }
    i_max
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Board(first={}, second={})", self.first, self.second)
    }
}

pub struct PositionEvaluation {
    i: usize,
    j: usize,
    evaluation: u64,
}

impl PartialEq for PositionEvaluation {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j && self.evaluation == other.evaluation
    }
}

impl std::fmt::Display for PositionEvaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "PositionEvaluation(i={}, j={}, evaluation={})",
            self.i, self.j, self.evaluation
        )
    }
}

impl std::fmt::Debug for PositionEvaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PositionEvaluation")
            .field("i", &self.i)
            .field("j", &self.j)
            .field("evaluation", &self.evaluation)
            .finish()
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

pub fn convert_bitboard_to_array(bitboard: u64) -> Vec<u64> {
    let mut reverse_patterns = Vec::<u64>::new();

    for i in 0..64 {
        let occupied = (bitboard >> i) & 1;
        reverse_patterns.push(occupied);
    }
    reverse_patterns.reverse();

    reverse_patterns
}

pub fn convert_indices_to_bitboard(i: u64, j: u64) -> u64 {
    let idx = i + 8 * j;
    1 << (63 - idx)
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
    1 << i
}

#[cfg(test)]
mod tests {
    use reversi;

    mod board {
        use reversi::Board;
        use reversi::PositionEvaluation;

        #[test]
        fn is_full_should_return_false_when_board_is_empty() {
            let board = Board {
                first: 0,
                second: 0,
            };
            assert_eq!(board.is_full(), false);
        }

        #[test]
        fn is_full_should_return_true_when_board_is_full() {
            let board = Board {
                first: 0,
                second: u64::MAX,
            };
            assert_eq!(board.is_full(), true);
        }

        #[test]
        fn entire_reverse_patterns() {
            let board = Board {
                first: 1,
                second: 2,
            };
            let reverse_patterns = board.entire_reverse_patterns(false);

            let mut expected = vec![0; 64];
            expected[61] = 2;

            assert_eq!(reverse_patterns, expected)
        }

        #[test]
        fn put_and_reverse_should_reverse_pieces() {
            let mut board = Board {
                first: 1,
                second: 6,
            };
            board.put_and_reverse(false, 8);
            assert_eq!(board.first, 15);
            assert_eq!(board.second, 0);
        }

        #[test]
        fn is_empty() {
            let board = Board {
                first: 1,
                second: 2,
            };
            assert_eq!(board.is_empty(1), false);
            assert_eq!(board.is_empty(1 << 63), true);
        }

        #[test]
        fn get_all_legal_position() {
            let board = Board {
                first: 0b_00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000,
                second: 0b_00000000_00000000_00000000_00001000_00010000_00000000_00000000_00000000,
            };
            let legal_positions = board.get_all_legal_position(false);

            let expected = vec![
                PositionEvaluation {
                    i: 4,
                    j: 2,
                    evaluation: 1,
                },
                PositionEvaluation {
                    i: 5,
                    j: 3,
                    evaluation: 1,
                },
                PositionEvaluation {
                    i: 2,
                    j: 4,
                    evaluation: 1,
                },
                PositionEvaluation {
                    i: 3,
                    j: 5,
                    evaluation: 1,
                },
            ];

            for i in 0..4 {
                assert_eq!(legal_positions[i], expected[i]);
            }
        }
    }

    #[test]
    fn count_bits_should_return_count_bits() {
        assert_eq!(reversi::count_bits(0), 0);
        assert_eq!(reversi::count_bits(u64::MAX), 64);
    }

    #[test]
    fn coordinate_to_bitboard_should_convert_notations() {
        assert_eq!(reversi::coordinate_to_bitboard(0, 0), 1);
        assert_eq!(reversi::coordinate_to_bitboard(7, 7), 1 << 63);
    }
}
