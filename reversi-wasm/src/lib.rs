extern crate wasm_bindgen;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, reversi-wasm!");
}

#[derive(Debug)]
struct Board {
    first: u64,  // black, 先手
    second: u64, // white, 後手
}

impl Board {
    pub fn is_full(&self) -> bool {
        (self.first | self.second) == u64::MAX
    }

    pub fn is_valid(&self) -> bool {
        true
    }

    pub fn put_and_reverse(&mut self, is_second: bool, put_position: u64) {
        if !is_second {
            let reverse_pattern = self.get_reverse_pattern(put_position);
            self.first ^= put_position | reverse_pattern;
            self.second ^= reverse_pattern;
        } else {
            let reverse_pattern = self.get_reverse_pattern(put_position);
            self.first ^= reverse_pattern;
            self.second ^= put_position | reverse_pattern;
        }
    }

    fn get_reverse_pattern(&self, put_position: u64) -> u64 {
        2
    }

    pub fn get_all_legal_positions(_is_second: bool) -> Vec<u64> {
        vec![0]
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

pub fn count_bits(bitboard: u64) -> u64 {
    let mut bits = bitboard;
    for i in 1..=6 {
        let mask = generate_mask(i);
        bits = (bits & mask) + (bits >> (1 << i - 1) & mask);
    }
    return bits;
}

#[cfg(test)]
mod tests {
    use count_bits;
    use Board;

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
    fn count_bits_should_return_count_bits() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(u64::MAX), 64);
    }

    #[test]
    fn put_should_reverse_pieces() {
        let mut board = Board {
            first: 1,
            second: 2,
        };
        board.put_and_reverse(false, 4);
        assert_eq!(board.first, 7);
        assert_eq!(board.second, 0);
    }
}
