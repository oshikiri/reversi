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
