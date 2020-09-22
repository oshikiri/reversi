fn cell_state_vec_to_pattern_instance_index(_v: Vec<u64>) -> u64 {
  0
}

// [hor./vert.2]
// - - - - - - - -
// * * * * * * * *
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub fn pattern_instance_hor_vert_2(first: Vec<u64>, second: Vec<u64>) -> u64 {
    (first[8] + 2 * second[8])
        + 3 * ((first[9] + 2 * second[9])
            + 3 * ((first[10] + 2 * second[10])
                + 3 * ((first[11] + 2 * second[11])
                    + 3 * ((first[12] + 2 * second[12])
                        + 3 * ((first[13] + 2 * second[13])
                            + 3 * ((first[14] + 2 * second[14])
                                + 3 * (first[15] + 2 * second[15])))))))
}

// [hor./vert.3]
// - - - - - - - -
// - - - - - - - -
// * * * * * * * *
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
// - - - - - - - -
pub fn pattern_instance_hor_vert_3(first: Vec<u64>, second: Vec<u64>) -> u64 {
  0
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

            let actual = bitboard::pattern_instance_hor_vert_2(first, second);
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
