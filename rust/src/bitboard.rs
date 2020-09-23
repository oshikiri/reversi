use crate::board::Board;

// Buro, M. (2003) The Evolution of Strong Othello Programs. Entertainment Computing. IFIP, vol 112. Springer, Boston, MA.
//
// for n in range(64):
//      if n % 8 == 0:
//          print("\n|", end="")
//      print(f"{n}|", end="")
//
// | 0| 1| 2| 3| 4| 5| 6| 7|
// | 8| 9|10|11|12|13|14|15|
// |16|17|18|19|20|21|22|23|
// |24|25|26|27|28|29|30|31|
// |32|33|34|35|36|37|38|39|
// |40|41|42|43|44|45|46|47|
// |48|49|50|51|52|53|54|55|
// |56|57|58|59|60|61|62|63|
pub mod pattern {
    pub type Pattern = [i64; 10];

    pub const DIAG_4: Pattern = [3, 10, 17, 24, -1, -1, -1, -1, -1, -1];
    pub const DIAG_5: Pattern = [4, 11, 18, 25, 32, -1, -1, -1, -1, -1];
    pub const DIAG_6: Pattern = [5, 12, 19, 26, 33, 40, -1, -1, -1, -1];
    pub const DIAG_7: Pattern = [6, 13, 20, 27, 34, 41, 48, -1, -1, -1];
    pub const DIAG_8: Pattern = [7, 14, 21, 28, 35, 42, 49, 56, -1, -1];

    pub const HOR_VERT_2: Pattern = [8, 9, 10, 11, 12, 13, 14, 15, -1, -1];
    pub const HOR_VERT_3: Pattern = [16, 17, 18, 19, 20, 21, 22, 23, -1, -1];
    pub const HOR_VERT_4: Pattern = [24, 25, 26, 27, 28, 29, 30, 31, -1, -1];

    pub const EDGE_2X: Pattern = [0, 1, 2, 3, 4, 5, 6, 7, 9, 14];
    pub const CORNER_2_5: Pattern = [0, 1, 2, 3, 4, 8, 9, 10, 11, 12];
    pub const CORNER_3_3: Pattern = [0, 1, 2, 8, 9, 10, 16, 17, 18, -1];
}

pub fn extract_pattern_instance_indices(board: &Board) -> Vec<u64> {
    let first = u64_to_bitvec(board.first());
    let second = u64_to_bitvec(board.second());

    vec![
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::DIAG_4),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::DIAG_5),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::DIAG_6),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::DIAG_7),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::DIAG_8),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::HOR_VERT_2),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::HOR_VERT_3),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::HOR_VERT_4),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::EDGE_2X),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::CORNER_2_5),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::CORNER_3_3),
    ]
}

pub fn cell_state_vec_to_pattern_instance_index(
    first: &Vec<u64>,
    second: &Vec<u64>,
    pattern: pattern::Pattern,
) -> u64 {
    let length = pattern.len();
    let mut power = 1;
    let mut index = 0;
    for i in 0..length {
        if pattern[i] < 0 {
            break;
        }
        let i_pattern = pattern[i] as usize;
        index += (first[i_pattern] + 2 * second[i_pattern]) * power;
        power *= 3;
    }
    index
}

pub fn u64_to_bitvec(n_original: u64) -> Vec<u64> {
    let mut n = n_original;
    let mut bitvec = vec![0; 64];
    for i in 0..63 {
        bitvec[i] = n & 1;
        n = n >> 1;
    }
    bitvec
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

        Board::create(first, second)
    }

    mod bitboard_test {
        use super::create_board_fixture;
        use crate::bitboard;

        #[test]
        fn u64_to_bitvec() {
            let n = 0b_01000000_00000000_00000000_00000000_00000000_00000000_00000000_00010001;
            let actual = bitboard::u64_to_bitvec(n);

            #[rustfmt::skip]
          let expected = vec![
              1, 0, 0, 0, 1, 0, 0, 0,
              0, 0, 0, 0, 0, 0, 0, 0,
              0, 0, 0, 0, 0, 0, 0, 0,
              0, 0, 0, 0, 0, 0, 0, 0,
              0, 0, 0, 0, 0, 0, 0, 0,
              0, 0, 0, 0, 0, 0, 0, 0,
              0, 0, 0, 0, 0, 0, 0, 0,
              0, 0, 0, 0, 0, 0, 1, 0,
          ];

            assert_eq!(actual, expected);
        }

        // import random
        // for n in range(64):
        //   if n % 8 == 0:
        //     print("\n", end="")
        //   x = random.choice(['-', 'o', 'x'])
        //   print(f"{x} ", end="")
        #[test]
        fn pattern_instance_hor_vert_2() {
            let board = create_board_fixture(
                "
              x o x - x o o x
              - o - o x - o -
              x x o x o - o x
              x - o o x o x -
              - x - - o - - o
              - o - x x x o o
              x - x x o - x -
              x - x x - x o o
          ",
            );
            let first = bitboard::u64_to_bitvec(board.first());
            let second = bitboard::u64_to_bitvec(board.second());

            let actual = bitboard::cell_state_vec_to_pattern_instance_index(
                &first,
                &second,
                bitboard::pattern::HOR_VERT_2,
            );
            let expected = 0
                + 1 * 3
                + 0 * 3 * 3
                + 1 * 3 * 3 * 3
                + 2 * 3 * 3 * 3 * 3
                + 0 * 3 * 3 * 3 * 3 * 3
                + 1 * 3 * 3 * 3 * 3 * 3 * 3
                + 0 * 3 * 3 * 3 * 3 * 3 * 3 * 3;
            assert_eq!(actual, expected);
        }
    }
}
