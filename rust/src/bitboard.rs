use crate::board::Board;

// Buro, M. (2003) The Evolution of Strong Othello Programs. Entertainment Computing. IFIP, vol 112. Springer, Boston, MA.
mod pattern {
    pub type Pattern = [i64; 10];
    pub const HOR_VERT_2: Pattern = [8, 9, 10, 11, 12, 13, 14, 15, -1, -1];
    pub const HOR_VERT_3: Pattern = [16, 17, 18, 19, 20, 21, 22, 23, -1, -1];
}

pub fn extract_pattern_instance_indices(board: Board) -> Vec<u64> {
    let first = u64_to_bitvec(board.first);
    let second = u64_to_bitvec(board.second);

    vec![
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::HOR_VERT_2),
        cell_state_vec_to_pattern_instance_index(&first, &second, pattern::HOR_VERT_3),
    ]
}

fn cell_state_vec_to_pattern_instance_index(
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

        Board { first, second }
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

        #[test]
        fn pattern_instance_hor_vert_2() {
            let board = create_board_fixture(
                "
              x o x o x o x o
              x o x o x o x o
              x o x o x o x o
              x o x o x o x o
              x o x o x o x o
              x o x o x o x o
              x o x o x o x o
              x o x o x o x o
          ",
            );
            let first = bitboard::u64_to_bitvec(board.first);
            let second = bitboard::u64_to_bitvec(board.second);

            let actual = bitboard::cell_state_vec_to_pattern_instance_index(
                &first,
                &second,
                bitboard::pattern::HOR_VERT_2,
            );
            let expected = 2
                + 3 * 1
                + 3 * 3 * 2
                + 3 * 3 * 3 * 1
                + 3 * 3 * 3 * 3 * 2
                + 3 * 3 * 3 * 3 * 3 * 1
                + 3 * 3 * 3 * 3 * 3 * 3 * 2
                + 3 * 3 * 3 * 3 * 3 * 3 * 3 * 1;
            assert_eq!(actual, expected);
        }
    }
}
