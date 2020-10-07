use crate::board::Board;
use crate::player::Player;

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

    pub const N_PATTERN_CELLS: [usize; 11] = [4, 5, 6, 7, 8, 8, 8, 8, 10, 10, 9];
    pub const N_PATTERNS: usize = 4 * N_PATTERN_CELLS.len();
    pub type PatternIndices = [u64; N_PATTERNS];

    pub const ALL_PATTERNS: [Pattern; N_PATTERNS] = [
        // rotate: +0
        [3, 10, 17, 24, -1, -1, -1, -1, -1, -1],  // diag4
        [4, 11, 18, 25, 32, -1, -1, -1, -1, -1],  // diag5
        [5, 12, 19, 26, 33, 40, -1, -1, -1, -1],  // diag6
        [6, 13, 20, 27, 34, 41, 48, -1, -1, -1],  // diag7
        [7, 14, 21, 28, 35, 42, 49, 56, -1, -1],  // diag8
        [8, 9, 10, 11, 12, 13, 14, 15, -1, -1],   // hor./vert. 2
        [16, 17, 18, 19, 20, 21, 22, 23, -1, -1], // hor./vert. 3
        [24, 25, 26, 27, 28, 29, 30, 31, -1, -1], // hor./vert. 4
        [0, 1, 2, 3, 4, 5, 6, 7, 9, 14],          // edge-2x
        [0, 1, 2, 3, 4, 8, 9, 10, 11, 12],        // corner-2x5
        [0, 1, 2, 8, 9, 10, 16, 17, 18, -1],      // corner-3x3
        // rotate: +90
        [31, 22, 13, 4, -1, -1, -1, -1, -1, -1], // diag4
        [39, 30, 21, 12, 3, -1, -1, -1, -1, -1], // diag5
        [47, 38, 29, 20, 11, 2, -1, -1, -1, -1], // diag6
        [55, 46, 37, 28, 19, 10, 1, -1, -1, -1], // diag7
        [63, 54, 45, 36, 27, 18, 9, 0, -1, -1],  // diag8
        [6, 14, 22, 30, 38, 46, 54, 62, -1, -1], // hor./vert. 2
        [5, 13, 21, 29, 37, 45, 53, 61, -1, -1], // hor./vert. 3
        [4, 12, 20, 28, 36, 44, 52, 60, -1, -1], // hor./vert. 4
        [7, 15, 23, 31, 39, 47, 55, 63, 14, 54], // edge-2x
        [7, 15, 23, 31, 39, 6, 14, 22, 30, 38],  // corner-2x5
        [7, 15, 23, 8, 9, 10, 16, 17, 18, -1],   // corner-3x3
        // rotate: +180
        [60, 53, 46, 39, -1, -1, -1, -1, -1, -1], // diag4
        [59, 52, 45, 38, 31, -1, -1, -1, -1, -1], // diag5
        [58, 51, 44, 37, 30, 23, -1, -1, -1, -1], // diag6
        [57, 50, 43, 36, 29, 22, 15, -1, -1, -1], // diag7
        [56, 49, 42, 35, 28, 21, 14, 7, -1, -1],  // diag8
        [55, 54, 53, 52, 51, 50, 49, 48, -1, -1], // hor./vert. 2
        [47, 46, 45, 44, 43, 42, 41, 40, -1, -1], // hor./vert. 3
        [39, 38, 37, 36, 35, 34, 33, 32, -1, -1], // hor./vert. 4
        [63, 62, 61, 60, 59, 58, 57, 56, 54, 49], // edge-2x
        [63, 62, 61, 60, 59, 55, 54, 53, 52, 51], // corner-2x5
        [63, 62, 61, 55, 54, 53, 47, 46, 45, -1], // corner-3x3
        // rotate: +270
        [32, 41, 50, 59, -1, -1, -1, -1, -1, -1], // diag4
        [24, 33, 42, 51, 60, -1, -1, -1, -1, -1], // diag5
        [16, 25, 34, 43, 52, 61, -1, -1, -1, -1], // diag6
        [8, 17, 26, 35, 44, 53, 62, -1, -1, -1],  // diag7
        [0, 9, 18, 27, 36, 45, 54, 63, -1, -1],   // diag8
        [57, 49, 41, 33, 25, 17, 9, 1, -1, -1],   // hor./vert. 2
        [58, 50, 42, 34, 26, 18, 10, 2, -1, -1],  // hor./vert. 3
        [59, 51, 43, 35, 27, 19, 11, 3, -1, -1],  // hor./vert. 4
        [56, 48, 40, 32, 24, 16, 8, 0, 49, 9],    // edge-2x
        [56, 48, 40, 32, 24, 57, 49, 41, 33, 25], // corner-2x5
        [56, 48, 40, 57, 49, 41, 58, 50, 42, -1], // corner-3x3
    ];
}

pub fn extract_pattern_instance_indices(board: &Board, player: &Player) -> Vec<u64> {
    let (current, opponent) = match player {
        Player::First => (board.first(), board.second()),
        Player::Second => (board.second(), board.first()),
    };
    let current = u64_to_bitvec(current);
    let opponent = u64_to_bitvec(opponent);

    let patterns: Vec<pattern::Pattern> = pattern::ALL_PATTERNS.to_vec();

    patterns
        .iter()
        .map(|&p| cell_state_vec_to_pattern_instance_index(&current, &opponent, p))
        .collect()
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
    for i in 0..64 {
        bitvec[i] = n & 1;
        n = n >> 1;
    }
    bitvec
}

pub fn put_position_to_coord(position: Option<u64>) -> Result<String, String> {
    if position.is_none() {
        return Ok("passed".to_string());
    }
    let mut position = position.unwrap();
    let mut i_position = None;
    for i in 0..64 {
        if position & 1 == 1 {
            i_position = Some(i);
        }
        position = position >> 1;
    }
    match i_position {
        Some(k) => {
            let i_str = match k % 8 {
                0 => Ok("a"),
                1 => Ok("b"),
                2 => Ok("c"),
                3 => Ok("d"),
                4 => Ok("e"),
                5 => Ok("f"),
                6 => Ok("g"),
                7 => Ok("h"),
                _ => Err("invalid i"),
            };
            let j = k / 8 + 1;
            Ok(format!("{}{}", i_str?, j))
        }
        None => Err(format!("invalid position = {:?}", position)),
    }
}

pub fn put_position_to_xy(position: u64) -> Option<(u64, u64)> {
    let mut position = position;
    let mut i_position = None;
    for i in 0..64 {
        if position & 1 == 1 {
            i_position = Some(i);
        }
        position = position >> 1;
    }
    i_position.map(|i| (i % 8, i / 8))
}

#[cfg(test)]
mod tests {
    mod bitboard_test {
        use crate::bitboard;
        use crate::board::Board;

        #[test]
        fn u64_to_bitvec() {
            let n = 0b_11000000_00000000_00000000_00000000_00000000_00000000_00000000_00010001;
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
              0, 0, 0, 0, 0, 0, 1, 1,
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
            let board = Board::create_from_str(
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
                bitboard::pattern::ALL_PATTERNS[5], // rotate:+0, hor./vert. 2
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

        #[test]
        fn put_position_to_coord() {
            assert_eq!(
                bitboard::put_position_to_coord(Some(1)),
                Ok("a1".to_string())
            );
            assert_eq!(
                bitboard::put_position_to_coord(Some(1 << 7)),
                Ok("h1".to_string())
            );
            assert_eq!(
                bitboard::put_position_to_coord(Some(1 << 63)),
                Ok("h8".to_string())
            );
        }
    }
}
