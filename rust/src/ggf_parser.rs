type CharTriple = (char, char, char);

#[derive(Debug, PartialEq)]
pub struct Game {
    name: String,
    place: String,
    datetime: String,
    first_name: String,
    second_name: String,
    first_rating: f64,
    second_rating: f64,
    game_type: String,
    result_score: f64,
    board_type: String,
    moves: Vec<CharTriple>,
}

#[allow(dead_code)] // TODO: remove
pub fn new_game(
    name_str: &str,
    place_str: &str,
    datetime_str: &str,
    first_name_str: &str,
    second_name_str: &str,
    first_rating: f64,
    second_rating: f64,
    game_type_str: &str,
    result_score: f64,
    board_type_str: &str,
    moves: Vec<CharTriple>,
) -> Game {
    Game {
        name: name_str.to_string(),
        place: place_str.to_string(),
        datetime: datetime_str.to_string(),
        first_name: first_name_str.to_string(),
        second_name: second_name_str.to_string(),
        first_rating,
        second_rating,
        game_type: game_type_str.to_string(),
        result_score,
        board_type: board_type_str.to_string(),
        moves,
    }
}

fn comsume_until_close_bracket(chars: &Vec<char>, i: usize) -> (usize, String) {
    let mut j = i;
    loop {
        if chars[j] == ']' {
            break;
        }

        j += 1;
    }

    let content: String = chars.get(i..j).unwrap().into_iter().collect();
    (j, content)
}

fn parse_move_content(turn: char, i: usize, chars: &Vec<char>, content: String) -> CharTriple {
    let first_slash = content.find('/').unwrap_or(content.len());
    let first_element = content.get(0..first_slash).unwrap();
    if first_element == "pass" {
        (turn, '*', '*')
    } else {
        let x = chars.get(i + 1).unwrap();
        let y = chars.get(i + 2).unwrap();
        (turn, *x, *y)
    }
}

#[allow(dead_code)] // TODO: remove
pub fn parse(game_string: String) -> Game {
    let mut game = new_game("", "", "", "", "", 0.0, 0.0, "", 0.0, "", Vec::new());

    let mut buffer = String::from("");
    let chars = game_string.chars().collect::<Vec<char>>();
    let mut i = 0;
    loop {
        let c = chars[i];
        buffer.push(c);

        let buffer_str: &str = &buffer;
        match buffer_str {
            "(;" | ";)" => {
                buffer.clear();
            }
            "GM[" => {
                let (i_next, game_name) = comsume_until_close_bracket(&chars, i + 1);
                i = i_next;
                game.name = game_name;
                buffer.clear();
            }
            "PC[" => {
                let (i_next, place) = comsume_until_close_bracket(&chars, i + 1);
                i = i_next;
                game.place = place;
                buffer.clear();
            }
            "DT[" => {
                let (i_next, datetime_str) = comsume_until_close_bracket(&chars, i + 1);
                i = i_next;
                game.datetime = datetime_str;
                buffer.clear();
            }
            "PB[" => {
                let (i_next, player_black) = comsume_until_close_bracket(&chars, i + 1);
                i = i_next;
                game.first_name = player_black;
                buffer.clear();
            }
            "PW[" => {
                let (i_next, player_white) = comsume_until_close_bracket(&chars, i + 1);
                i = i_next;
                game.second_name = player_white;
                buffer.clear();
            }
            "RB[" => {
                let (i_next, rating_str) = comsume_until_close_bracket(&chars, i + 1);
                i = i_next;
                game.first_rating = rating_str.parse::<f64>().unwrap();
                buffer.clear();
            }
            "RW[" => {
                let (i_next, rating_str) = comsume_until_close_bracket(&chars, i + 1);
                i = i_next;
                game.second_rating = rating_str.parse::<f64>().unwrap();
                buffer.clear();
            }
            "TY[" => {
                let (i_next, game_type) = comsume_until_close_bracket(&chars, i + 1);
                i = i_next;
                game.game_type = game_type;
                buffer.clear();
            }
            "RE[" => {
                let (i_next, result_score_str) = comsume_until_close_bracket(&chars, i + 1);
                i = i_next;
                let result_score_str: Vec<&str> = result_score_str.split(':').collect();
                game.result_score = result_score_str[0].parse::<f64>().unwrap();
                buffer.clear();
            }
            "BO[" => {
                let (i_next, content) = comsume_until_close_bracket(&chars, i + 1);
                game.board_type = content;
                i = i_next;
                buffer.clear();
            }
            "B[" => {
                let (i_next, content) = comsume_until_close_bracket(&chars, i + 1);
                let next_move = parse_move_content('B', i, &chars, content);
                game.moves.push(next_move);
                i = i_next;
                buffer.clear();
            }
            "W[" => {
                let (i_next, content) = comsume_until_close_bracket(&chars, i + 1);
                let next_move = parse_move_content('W', i, &chars, content);
                game.moves.push(next_move);
                i = i_next;
                buffer.clear();
            }
            _ => {
                // Skip when it is unhandled case
                if c == ']' {
                    println!("{}", buffer);
                    buffer.clear();
                }
            }
        }

        i += 1;
        if i >= chars.len() {
            break;
        }
    }

    game
}

#[cfg(test)]
mod tests {
    use ggf_parser::*;

    #[test]
    fn parse_game_01e4_1_modified() {
        // https://www.skatgame.net/mburo/ggs/game-archive/Othello/
        // bzgrep . Othello.01e4.ggf.bz2 | head -1
        let game_string = String::from("(;GM[Othello]PC[GGS/os]DT[2000-4-16 11:13 EST]PB[fangr]PW[patzer]RB[1457.12]RW[1631.74]TI[15:00//02:00]TY[8]RE[-40.00:r]BO[8 -------- -------- -------- ---O*--- ---*O--- -------- -------- -------- *]B[E6//4.09]W[H8/40.00/0.01]B[pass//1.67]W[G7/40.00/0.01]B[pass//2.10]W[G8];)");

        let actual = parse(game_string);
        let expected = new_game(
            "Othello",
            "GGS/os",
            "2000-4-16 11:13 EST",
            "fangr",
            "patzer",
            1457.12,
            1631.74,
            "8",
            -40.0,
            "8 -------- -------- -------- ---O*--- ---*O--- -------- -------- -------- *",
            vec![
                ('B', 'E', '6'),
                ('W', 'H', '8'),
                ('B', '*', '*'),
                ('W', 'G', '7'),
                ('B', '*', '*'),
                ('W', 'G', '8'),
            ],
        );
        assert_eq!(actual, expected);
    }
}
