use reversi::ggf;

use std::fs::File;
use std::io::{BufRead, BufReader};

// ex. cargo run ~/Downloads/Othello.02e4.ggf
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("path is required");

    let mut i = 0;
    for result in BufReader::new(File::open(path)?).lines() {
        let line: String = result?;
        let game: ggf::Game = ggf::parse(line);
        let pattern_instance_histories: Vec<ggf::PatternInstanceHistory> =
            ggf::extract_pattarn_instance_histories(&game);
        println!("{:?}", game);

        for history in pattern_instance_histories {
            println!("{:?}", history);
        }

        i += 1;
        if i >= 1 {
            break;
        }
    }
    Ok(())
}
