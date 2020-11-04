use crate::board;
use crate::player::Player;

pub const SIZE_N_REVERSES_3: usize = 27; // 3**3
pub const SIZE_N_REVERSES_4: usize = 81; // 3**4
pub const SIZE_N_REVERSES_5: usize = 243; // 3**5
pub const SIZE_N_REVERSES_6: usize = 729; // 3**6
pub const SIZE_N_REVERSES_7: usize = 2187; // 3**7
pub const SIZE_N_REVERSES_8: usize = 6561; // 3**8

lazy_static! {
    pub static ref N_REVERSES_3: Box<[u64; 3 * SIZE_N_REVERSES_3]> =
        Box::new(get_size_n_reverses_3());
    pub static ref N_REVERSES_4: Box<[u64; 4 * SIZE_N_REVERSES_4]> =
        Box::new(get_size_n_reverses_4());
    pub static ref N_REVERSES_5: Box<[u64; 5 * SIZE_N_REVERSES_5]> =
        Box::new(get_size_n_reverses_5());
    pub static ref N_REVERSES_6: Box<[u64; 6 * SIZE_N_REVERSES_6]> =
        Box::new(get_size_n_reverses_6());
    pub static ref N_REVERSES_7: Box<[u64; 7 * SIZE_N_REVERSES_7]> =
        Box::new(get_size_n_reverses_7());
    pub static ref N_REVERSES_8: Box<[u64; 8 * SIZE_N_REVERSES_8]> =
        Box::new(get_size_n_reverses_8());
}

// TODO: refactor?
pub fn get_size_n_reverses_3() -> [u64; 3 * SIZE_N_REVERSES_3] {
    let mut n_reverses = [0; 3 * SIZE_N_REVERSES_3];
    for i in 0..SIZE_N_REVERSES_3 {
        let cells = board::parse_reverse_index(i as u64);
        let board = embed_coded_line_to_top_horizontal_line(&cells);
        let reverse_patterns = board.entire_reverse_patterns(&Player::First);
        for j in 0..3 {
            n_reverses[3 * i + j] = board::count_bits(reverse_patterns[j]);
        }
    }
    n_reverses
}
pub fn get_size_n_reverses_4() -> [u64; 4 * SIZE_N_REVERSES_4] {
    let mut n_reverses = [0; 4 * SIZE_N_REVERSES_4];
    for i in 0..SIZE_N_REVERSES_4 {
        let cells = board::parse_reverse_index(i as u64);
        let board = embed_coded_line_to_top_horizontal_line(&cells);
        let reverse_patterns = board.entire_reverse_patterns(&Player::First);
        for j in 0..4 {
            n_reverses[4 * i + j] = board::count_bits(reverse_patterns[j]);
        }
    }
    n_reverses
}
pub fn get_size_n_reverses_5() -> [u64; 5 * SIZE_N_REVERSES_5] {
    let mut n_reverses = [0; 5 * SIZE_N_REVERSES_5];
    for i in 0..SIZE_N_REVERSES_5 {
        let cells = board::parse_reverse_index(i as u64);
        let board = embed_coded_line_to_top_horizontal_line(&cells);
        let reverse_patterns = board.entire_reverse_patterns(&Player::First);
        for j in 0..5 {
            n_reverses[5 * i + j] = board::count_bits(reverse_patterns[j]);
        }
    }
    n_reverses
}
pub fn get_size_n_reverses_6() -> [u64; 6 * SIZE_N_REVERSES_6] {
    let mut n_reverses = [0; 6 * SIZE_N_REVERSES_6];
    for i in 0..SIZE_N_REVERSES_6 {
        let cells = board::parse_reverse_index(i as u64);
        let board = embed_coded_line_to_top_horizontal_line(&cells);
        let reverse_patterns = board.entire_reverse_patterns(&Player::First);
        for j in 0..6 {
            n_reverses[6 * i + j] = board::count_bits(reverse_patterns[j]);
        }
    }
    n_reverses
}
pub fn get_size_n_reverses_7() -> [u64; 7 * SIZE_N_REVERSES_7] {
    let mut n_reverses = [0; 7 * SIZE_N_REVERSES_7];
    for i in 0..SIZE_N_REVERSES_7 {
        let cells = board::parse_reverse_index(i as u64);
        let board = embed_coded_line_to_top_horizontal_line(&cells);
        let reverse_patterns = board.entire_reverse_patterns(&Player::First);
        for j in 0..7 {
            n_reverses[7 * i + j] = board::count_bits(reverse_patterns[j]);
        }
    }
    n_reverses
}
pub fn get_size_n_reverses_8() -> [u64; 8 * SIZE_N_REVERSES_8] {
    let mut n_reverses = [0; 8 * SIZE_N_REVERSES_8];
    for i in 0..SIZE_N_REVERSES_8 {
        let cells = board::parse_reverse_index(i as u64);
        let board = embed_coded_line_to_top_horizontal_line(&cells);
        let reverse_patterns = board.entire_reverse_patterns(&Player::First);
        for j in 0..8 {
            n_reverses[8 * i + j] = board::count_bits(reverse_patterns[j]);
        }
    }
    n_reverses
}

fn embed_coded_line_to_top_horizontal_line(cells: &[u8; 8]) -> board::Board {
    let mut first = 0;
    let mut second = 0;
    for (i_cell, &cell) in cells.iter().enumerate() {
        if cell == 1 {
            first |= 1 << i_cell;
        } else if cell == 2 {
            second |= 1 << i_cell;
        }
    }
    board::Board::create(first, second)
}

pub static REVERSE_LINE_PATTERN_3: [[usize; 3]; 4] =
    [[5, 14, 23], [40, 49, 58], [2, 9, 16], [47, 54, 61]];

pub static REVERSE_LINE_PATTERN_4: [[usize; 4]; 4] = [
    [4, 14, 22, 31],
    [32, 41, 50, 59],
    [3, 10, 17, 24],
    [39, 46, 53, 60],
];

pub static REVERSE_LINE_PATTERN_5: [[usize; 5]; 4] = [
    [3, 12, 21, 30, 39],
    [24, 33, 42, 51, 60],
    [4, 11, 18, 25, 32],
    [31, 38, 45, 52, 59],
];

pub static REVERSE_LINE_PATTERN_6: [[usize; 6]; 4] = [
    [2, 11, 20, 29, 38, 47],
    [16, 25, 34, 43, 52, 61],
    [5, 12, 19, 26, 33, 40],
    [23, 30, 37, 44, 51, 58],
];

pub static REVERSE_LINE_PATTERN_7: [[usize; 7]; 4] = [
    [1, 10, 19, 28, 37, 46, 55],
    [8, 17, 26, 35, 44, 53, 62],
    [6, 13, 20, 27, 34, 41, 48],
    [15, 22, 29, 36, 43, 50, 57],
];

pub static REVERSE_LINE_PATTERN_8: [[usize; 8]; 18] = [
    // horizontal
    [0, 8, 16, 24, 32, 40, 48, 56],
    [1, 9, 17, 25, 33, 41, 49, 57],
    [2, 10, 18, 26, 34, 42, 50, 58],
    [3, 11, 19, 27, 35, 43, 51, 59],
    [4, 12, 20, 28, 36, 44, 52, 60],
    [5, 13, 21, 29, 37, 45, 53, 61],
    [6, 14, 22, 30, 38, 46, 54, 62],
    [7, 15, 23, 31, 39, 47, 55, 63],
    // vertical
    [0, 1, 2, 3, 4, 5, 6, 7],
    [8, 9, 10, 11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20, 21, 22, 23],
    [24, 25, 26, 27, 28, 29, 30, 31],
    [32, 33, 34, 35, 36, 37, 38, 39],
    [40, 41, 42, 43, 44, 45, 46, 47],
    [48, 49, 50, 51, 52, 53, 54, 55],
    [56, 57, 58, 59, 60, 61, 62, 63],
    // diagonal
    [0, 9, 18, 27, 36, 45, 54, 63],
    [7, 14, 21, 28, 35, 42, 49, 56],
];
