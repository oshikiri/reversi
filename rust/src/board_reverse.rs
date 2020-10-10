use crate::bitboard;
use crate::board;
use crate::player::Player;

pub const SIZE_N_REVERSES_3: usize = 27; // 3**3
pub const SIZE_N_REVERSES_4: usize = 6561; // 3**4
pub const SIZE_N_REVERSES_5: usize = 6561; // 3**5
pub const SIZE_N_REVERSES_6: usize = 6561; // 3**6
pub const SIZE_N_REVERSES_7: usize = 6561; // 3**7
pub const SIZE_N_REVERSES_8: usize = 6561; // 3**8

lazy_static! {
    pub static ref N_REVERSES_3: [u64; SIZE_N_REVERSES_3] = [0; SIZE_N_REVERSES_3];
    pub static ref N_REVERSES_4: [u64; SIZE_N_REVERSES_4] = [0; SIZE_N_REVERSES_4];
    pub static ref N_REVERSES_5: [u64; SIZE_N_REVERSES_5] = [0; SIZE_N_REVERSES_5];
    pub static ref N_REVERSES_6: [u64; SIZE_N_REVERSES_6] = [0; SIZE_N_REVERSES_6];
    pub static ref N_REVERSES_7: [u64; SIZE_N_REVERSES_7] = [0; SIZE_N_REVERSES_7];
    pub static ref N_REVERSES_8: Box<[u64; 8 * SIZE_N_REVERSES_8]> =
        Box::new(get_size_n_reverses_8());
}

fn get_size_n_reverses_8() -> [u64; 8 * SIZE_N_REVERSES_8] {
    let mut n_reverses = [0; 8 * SIZE_N_REVERSES_8];
    for i in 0..SIZE_N_REVERSES_8 {
        let cells = board::parse_reverse_index(i as u64);

        let mut first = 0;
        let mut second = 0;
        for (i_cell, &cell) in cells.iter().enumerate() {
            if cell == 1 {
                first |= 1 << i_cell;
            } else if cell == 2 {
                second |= 1 << i_cell;
            }
        }
        let board = board::Board::create(first, second);
        let reverse_patterns = board.entire_reverse_patterns(&Player::First);
        for j in 0..8 {
            n_reverses[8 * i + j] = board::count_bits(reverse_patterns[j]);
        }
    }
    n_reverses
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn get_size_n_reverses_8() {
        let actual = crate::board_reverse::get_size_n_reverses_8();
        // o x - x x o x -
        let index: usize =
            2 * 3usize.pow(6) + 3usize.pow(5) + 2 * 3usize.pow(4) + 2 * 3usize.pow(3) + 2 * 3 + 1;
        let expected = [0, 0, 3, 0, 0, 0, 0, 1];
        assert_eq!(actual[(8 * index)..(8 * index + 8)], expected);
    }

    #[test]
    fn static_n_reverses_8() {
        use crate::board_reverse::N_REVERSES_8;
        // o x - x x o x -
        let index: usize =
            2 * 3usize.pow(6) + 3usize.pow(5) + 2 * 3usize.pow(4) + 2 * 3usize.pow(3) + 2 * 3 + 1;
        for i in (8 * index)..(8 * index + 8) {
            println!("N_REVERSES_8[{}] = {}", i, N_REVERSES_8[i]);
        }

        let expected = [0, 0, 3, 0, 0, 0, 0, 1];
        assert_eq!(N_REVERSES_8[(8 * index)..(8 * index + 8)], expected);
    }
}
