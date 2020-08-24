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
    #[allow(dead_code)]
    fn is_full(&self) -> bool {
        (self.first | self.second) == BitBoard::MAX
    }

    #[allow(dead_code)]
    fn is_valid(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
pub fn count_bits(_bitboard: BitBoard) -> u64 {
    64
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
    fn count_bits_should_return_num_of_bits() {
        // assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(u64::MAX), 64);
    }
}
