#[cfg(test)]
mod board_reverse {
    #[test]
    fn get_size_n_reverses_8() {
        use reversi::board::board_reverse::get_size_n_reverses_8;

        let actual = get_size_n_reverses_8();
        // o x - x x o x -
        let index: usize =
            2 * 3usize.pow(6) + 3usize.pow(5) + 2 * 3usize.pow(4) + 2 * 3usize.pow(3) + 2 * 3 + 1;
        let expected = [0, 0, 3, 0, 0, 0, 0, 1];
        assert_eq!(actual[(8 * index)..(8 * index + 8)], expected);
    }

    #[test]
    fn static_n_reverses_8() {
        use reversi::board::board_reverse::N_REVERSES_8;

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
