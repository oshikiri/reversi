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
        use reversi::board::MoveError;
        use reversi::board::Player;

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
            assert!(!board.is_full());
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
            assert!(board.is_full());
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
            board.put_and_reverse(&Player::First, 8).unwrap();
            let expected = Board::create(15, 0);
            assert_eq!(board, expected);
        }

        #[test]
        fn put_and_reverse_should_reject_zero_position() {
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
            let original = board.clone();

            let actual = board.put_and_reverse(&Player::First, 0);

            assert_eq!(actual, Err(MoveError::NotSingleBit(0)));
            assert_eq!(board, original);
        }

        #[test]
        fn put_and_reverse_should_reject_multi_bit_position() {
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
            let original = board.clone();

            let actual = board.put_and_reverse(&Player::First, 8 | 16);

            assert_eq!(actual, Err(MoveError::NotSingleBit(8 | 16)));
            assert_eq!(board, original);
        }

        #[test]
        fn put_and_reverse_should_reject_occupied_position() {
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
            let original = board.clone();

            let actual = board.put_and_reverse(&Player::First, 1);

            assert_eq!(actual, Err(MoveError::Occupied(1)));
            assert_eq!(board, original);
        }

        #[test]
        fn put_and_reverse_should_reject_move_that_reverses_no_disks() {
            let mut board = Board::create_from_str(
                "
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
                - - - o x - - -
                - - - x o - - -
                - - - - - - - -
                - - - - - - - -
                - - - - - - - -
            ",
            );
            let original = board.clone();

            let actual = board.put_and_reverse(&Player::First, 1);

            assert_eq!(actual, Err(MoveError::NoReversedDisk(1)));
            assert_eq!(board, original);
        }

        #[test]
        fn try_create_should_reject_overlapping_bitboards() {
            assert_eq!(
                Board::try_create(1, 1),
                Err(reversi::board::BoardError::OverlappingDisks {
                    first: 1,
                    second: 1,
                })
            );
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
            assert!(!board.is_empty(1));
            assert!(board.is_empty(1 << 63));
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
            let n = 3u64.pow(6) + 2 * 3u64.pow(5) + 3u64.pow(4) + 3 + 2;
            let actual = parse_reverse_index(n);
            let expected = [2, 1, 0, 0, 1, 2, 1, 0];
            assert_eq!(actual, expected);
        }
    }
}
