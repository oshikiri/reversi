#![feature(test)]

#[cfg(test)]
mod board {
    use reversi::board::Board;

    #[test]
    fn test_create_board_fixture() {
        let actual = Board::create_from_str(
            "
            o - - - - - - -
            - - - - - - - x
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
            - - - - - - - -
        ",
        );
        let expected = Board::create(1, 1 << 15);
        assert_eq!(actual, expected);
    }

    mod board_test {
        use reversi::board::Board;
        use reversi::player::Player;

        #[test]
        fn equivalence() {
            let board1 = Board::create(0, 1);
            let board2 = Board::create(0, 1);
            let board3 = Board::create(0, 4);
            assert_eq!(board1, board2);
            assert_ne!(board1, board3);
        }

        #[test]
        fn is_full_should_return_false_when_board_is_empty() {
            let board = Board::create_from_str(
                "
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            assert_eq!(board.is_full(), false);
        }

        #[test]
        fn is_full_should_return_true_when_board_is_full() {
            let board = Board::create_from_str(
                "
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
                x x x x x x x x
            ",
            );
            assert_eq!(board.is_full(), true);
        }

        #[test]
        fn entire_reverse_patterns() {
            let board = Board::create_from_str(
                "
                o x - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            let reverse_patterns = board.entire_reverse_patterns(&Player::First);

            let mut expected = vec![0; 64];
            expected[2] = 2;

            assert_eq!(reverse_patterns, expected)
        }

        #[test]
        fn put_and_reverse_should_reverse_pieces() {
            let mut board = Board::create_from_str(
                "
                o x x - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            board.put_and_reverse(&Player::First, 8);
            let expected = Board::create(15, 0);
            assert_eq!(board, expected);
        }

        #[test]
        fn is_empty() {
            let board = Board::create_from_str(
                "
                o x - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            assert_eq!(board.is_empty(1), false);
            assert_eq!(board.is_empty(1 << 63), true);
        }

        #[test]
        fn get_n_reverses() {
            let board = Board::create_from_str(
                "
                - - - - - - - -
                - - - - - - - -
                - - o - - - - -
                - - o o o - - -
                - - - o x - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            let actual = board.get_n_reverses(&Player::Second);
            let expected = [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ];
            assert_eq!(actual, expected);
        }
    }

    mod utils_test {
        use reversi::board;
        // use reversi::board::convert_indices_to_bitboard;

        #[test]
        fn count_bits_should_return_count_bits() {
            assert_eq!(board::count_bits(0), 0);
            assert_eq!(board::count_bits(u64::MAX), 64);
        }

        #[test]
        fn coordinate_to_bitboard_should_convert_notations() {
            assert_eq!(board::coordinate_to_bitboard(0, 0), Ok(1));
            assert_eq!(board::coordinate_to_bitboard(7, 7), Ok(1 << 63));
            assert_eq!(
                board::coordinate_to_bitboard(8, 7),
                Err("out of index: (8, 7)".to_string())
            );
            assert_eq!(
                board::coordinate_to_bitboard(8, 8),
                Err("out of index: (8, 8)".to_string())
            );
        }

        #[test]
        fn extract_pattern_instances() {
            let board = board::Board::create_from_str(
                "
                o - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - x
            ",
            );
            println!("{:?}", board);

            let bitboard_a1 = board::convert_indices_to_bitboard('A', '1');
            let bitboard_h8 = board::convert_indices_to_bitboard('H', '8');

            assert_eq!(bitboard_a1, Ok(board.first()));
            assert_eq!(bitboard_h8, Ok(board.second()));
        }

        #[test]
        fn parse_reverse_index() {
            use reversi::board::parse_reverse_index;
            let n = 3u64.pow(6) + 2 * 3u64.pow(5) + 3u64.pow(4) + 1 * 3 + 2;
            let actual = parse_reverse_index(n);
            let expected = [2, 1, 0, 0, 1, 2, 1, 0];
            assert_eq!(actual, expected);
        }
    }

    mod benches {
        extern crate test;
        use test::Bencher;

        use reversi::board::Board;
        use reversi::player::Player;

        #[bench]
        fn get_all_legal_moves(bench: &mut Bencher) {
            let current_board = Board::create_from_str(
                "
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - o o o - - -
                - - - o x - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );

            bench.iter(|| {
                current_board.get_all_legal_moves(&Player::Second);
            })
        }
    }
}
