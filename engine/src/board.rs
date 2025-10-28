extern crate wasm_bindgen;

pub mod bitboard;
pub mod board_reverse;
mod player;

pub use player::Player;

use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::parameters::PATTERN_INSTANCES;

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

    pub fn getBitboard(&self, player: Player) -> js_sys::Array {
        let bitboard = match player {
            Player::First => self.first,
            Player::Second => self.second,
        };
        let bitarray = bitboard::u64_to_bitvec(bitboard);
        convert_vec_to_jsarray(bitarray)
    }

    pub fn getAllLegalPosition(&self, player: Player) -> js_sys::Array {
        let legal_positions: Vec<u64> = self
            .entire_reverse_patterns(&player)
            .into_iter()
            .map(count_bits)
            .collect();
        convert_vec_to_jsarray(legal_positions)
    }
}

impl Board {
    pub fn create(first: u64, second: u64) -> Board {
        Board { first, second }
    }

    pub fn first(&self) -> u64 {
        self.first
    }

    pub fn second(&self) -> u64 {
        self.second
    }

    pub fn is_full(&self) -> bool {
        (self.first | self.second) == u64::MAX
    }

    pub fn is_empty(&self, position: u64) -> bool {
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

    pub fn score_numdisk(&self, player: &Player) -> f32 {
        let score = count_bits(self.first) as f32 - count_bits(self.second) as f32;
        match player {
            Player::First => score,
            Player::Second => -score,
        }
    }

    pub fn get_all_legal_moves(&self, player: &Player) -> Vec<u64> {
        let n_reverses = self.get_n_reverses(player);
        let mut legal_moves = Vec::new();
        for (i, &reverse_count) in n_reverses.iter().enumerate() {
            if reverse_count > 0 {
                legal_moves.push(1 << i);
            }
        }

        legal_moves
    }

    pub fn get_n_reverses(&self, player: &Player) -> [u8; 64] {
        use crate::board::board_reverse::*;

        let (current, opponent) = match player {
            Player::First => (self.first, self.second),
            Player::Second => (self.second, self.first),
        };

        let mut coded_board: [u8; 64] = [0; 64];
        for (i, coded_cell) in coded_board.iter_mut().enumerate() {
            let put_position = 1 << i;
            *coded_cell = if current & put_position > 0 {
                1
            } else if opponent & put_position > 0 {
                2
            } else {
                0
            };
        }

        let mut n_reverses: [u8; 64] = [0; 64];

        for pattern in REVERSE_LINE_PATTERN_3.iter() {
            let mut index: usize = 0;
            for i in 0..3 {
                let cell = coded_board[pattern[i]] as usize;
                index += cell * 3usize.pow(i as u32);
            }

            for i in 0..3 {
                n_reverses[pattern[i]] += N_REVERSES_3[3 * index + i] as u8;
            }
        }
        for pattern in REVERSE_LINE_PATTERN_4.iter() {
            let mut index: usize = 0;
            for i in 0..4 {
                let cell = coded_board[pattern[i]] as usize;
                index += cell * 3usize.pow(i as u32);
            }

            for i in 0..4 {
                n_reverses[pattern[i]] += N_REVERSES_4[4 * index + i] as u8;
            }
        }
        for pattern in REVERSE_LINE_PATTERN_5.iter() {
            let mut index: usize = 0;
            for i in 0..5 {
                let cell = coded_board[pattern[i]] as usize;
                index += cell * 3usize.pow(i as u32);
            }

            for i in 0..5 {
                n_reverses[pattern[i]] += N_REVERSES_5[5 * index + i] as u8;
            }
        }
        for pattern in REVERSE_LINE_PATTERN_6.iter() {
            let mut index: usize = 0;
            for i in 0..6 {
                let cell = coded_board[pattern[i]] as usize;
                index += cell * 3usize.pow(i as u32);
            }

            for i in 0..6 {
                n_reverses[pattern[i]] += N_REVERSES_6[6 * index + i] as u8;
            }
        }
        for pattern in REVERSE_LINE_PATTERN_7.iter() {
            let mut index: usize = 0;
            for i in 0..7 {
                let cell = coded_board[pattern[i]] as usize;
                index += cell * 3usize.pow(i as u32);
            }

            for i in 0..7 {
                n_reverses[pattern[i]] += N_REVERSES_7[7 * index + i] as u8;
            }
        }
        for pattern in REVERSE_LINE_PATTERN_8.iter() {
            let mut index: usize = 0;
            for i in 0..8 {
                let cell = coded_board[pattern[i]] as usize;
                index += cell * 3usize.pow(i as u32);
            }

            for i in 0..8 {
                n_reverses[pattern[i]] += N_REVERSES_8[8 * index + i] as u8;
            }
        }

        n_reverses
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
                n_cells += 1;
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
        let shift = 1 << (i - 1);
        bits = (bits & mask) + ((bits >> shift) & mask);
    }
    bits
}

pub fn coordinate_to_bitboard(x: u64, y: u64) -> Result<u64, String> {
    if x >= 8 || y >= 8 {
        Err(format!("out of index: ({}, {})", x, y))
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

pub fn parse_reverse_index(n: u64) -> [u8; 8] {
    let mut n_remainder = n;
    let mut reverse_line = [0; 8];
    for cell in reverse_line.iter_mut() {
        *cell = (n_remainder % 3) as u8;
        n_remainder /= 3;
    }
    reverse_line
}
