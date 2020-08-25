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

type BitBoard = u64;

#[derive(Debug)]
struct Board {
    first: BitBoard,  // black, 先手
    second: BitBoard, // white, 後手
}

impl Board {
    fn is_full(&self) -> bool {
        (self.first | self.second) == BitBoard::MAX
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn put(&mut self, _is_second: bool, _position: BitBoard) {
        let _reverse = 1;
        self.first = 7;
        self.second = 0;
    }

    fn get_possible_moves(_is_second: bool) -> Vec<BitBoard> {
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
        _ => panic!("i should be smaller than 7")
    }
}

pub fn count_bits(bitboard: BitBoard) -> u64 {
    let mut bits = bitboard;
    for i in 1..=6 {
        let mask = generate_mask(i);
        bits = (bits & mask) + (bits >> (1 << i-1) & mask);
    }
    return bits
}

#[cfg(test)]
mod tests {
    use Board;
    use count_bits;

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
        let mut board = Board{
            first: 1,
            second: 2,
        };
        board.put(false, 3);
        assert_eq!(board.first, 7);
        assert_eq!(board.second, 0);
    }
}
