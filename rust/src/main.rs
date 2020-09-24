use reversi::ggf;

use std::fs::File;
use std::io::{BufRead, BufReader};

// ex. cargo run ~/Downloads/Othello.02e4.ggf > ../python/data/game-histories/02.csv
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("path is required");

    for (i_game, game_str) in BufReader::new(File::open(path)?).lines().enumerate() {
        let game: ggf::Game = ggf::parse(game_str?);
        let pattern_instance_histories: Vec<ggf::PatternInstanceHistory> =
            ggf::extract_pattarn_instance_histories(&game);

        for history in pattern_instance_histories {
            println!("{},{}", i_game, history);
        }
    }
    Ok(())
}
