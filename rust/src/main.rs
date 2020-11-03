use reversi::ggf;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1).as_ref().map(String::as_str) {
        Some("parse-ggf") => {
            // ex. cargo run parse-ggf ~/Downloads/Othello.02e4.ggf > ../python/data/game-histories/02.csv
            let path = std::env::args().nth(2).expect("path is required");

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
        _subcommand => {
            // cargo build --release && cargo profiler callgrind --bin ./target/release/reversi -n 10 bench
            println!("Benchmark");
            use reversi::board::Board;
            use reversi::player::Player;
            use reversi::search_algorithm::alphabeta::AlphaBeta;

            let current_board = Board::create_from_str(
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
            let mut alphabeta = AlphaBeta::create(10000, |board: Board, player: Player| -> f32 {
                board.score_numdisk(player.clone())
            });
            let search_results = alphabeta.search(current_board, 7);
            println!("{:?}", search_results);

            Ok(())
        }
    }
}
