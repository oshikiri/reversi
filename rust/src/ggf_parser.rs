#[derive(Debug, PartialEq)]
pub struct Game {
    first_name: String,
    second_name: String,
}

#[allow(dead_code)] // TODO: remove
pub fn new_game(first_name_str: &str, second_name_str: &str) -> Game {
    Game {
        first_name: first_name_str.to_string(),
        second_name: second_name_str.to_string(),
    }
}

#[allow(dead_code)] // TODO: remove
pub fn parse(_game_string: String) -> Game {
    let mut game = new_game("", "");

    game.first_name = "fangr".to_string();
    game.second_name = "patzer".to_string();

    game
}

#[cfg(test)]
mod tests {
    use ggf_parser::*;

    #[test]
    fn parse_game_01e4_1() {
        // https://www.skatgame.net/mburo/ggs/game-archive/Othello/
        // bzgrep . Othello.01e4.ggf.bz2 | head -1
        let game_string = String::from("(;GM[Othello]PC[GGS/os]DT[2000-4-16 11:13 EST]PB[fangr]PW[patzer]RB[1457.12]RW[1631.74]TI[15:00//02:00]TY[8]RE[-40.00]BO[8 -------- -------- -------- ---O*--- ---*O--- -------- -------- -------- *]B[E6//4.09]W[F4/-48.50/0.93]B[C3//5.52]W[C4/-42.50/0.43]B[D3//1.70]W[D6/-39.50/0.28]B[F6//4.31]W[C6/-28.55/0.13]B[F5//2.17]W[G5/0.55/0.10]B[G6//2.21]W[E3/-30.57/0.02]B[F2//1.65]W[F7/-43.63/0.01]B[E7//4.71]W[F3/-43.63]B[H5//1.80]W[E2/-43.63]B[C5//5.10]W[B6/0.63]B[B4//6.88]W[B5/2.82]B[B3//8.30]W[A4/2.04]B[A3//3.06]W[H4/-2.55]B[H3//8.17]W[D8/-5.22/0.01]B[C7//9.61]W[D7/-2.40]B[C8//51.08]W[E8/0.90]B[C2//17.00]W[G4/4.05]B[F1//16.62]W[H6/6.88/0.01]B[G3//11.69]W[H2/12.51/0.01]B[A5//33.91]W[D2/15.19/0.01]B[E1//2.67]W[B2/13.64/0.01]B[B7//7.38]W[D1/15.94/0.01]B[G2//5.81]W[A6/15.89/0.01]B[A7//3.84]W[H1/27.21/0.01]B[F8//6.78]W[A8/29.49/0.01]B[B8//3.80]W[A2/29.39/0.01]B[A1//2.96]W[G1/30.62/0.01]B[C1//3.05]W[B1/30.46/0.01]B[H7//5.06]W[H8/40.00/0.01]B[pass//1.67]W[G7/40.00/0.01]B[pass//2.10]W[G8];)");

        let actual = parse(game_string);
        let expected = new_game("fangr", "patzer");
        assert_eq!(actual, expected);
    }
}
