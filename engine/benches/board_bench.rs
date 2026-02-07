use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use reversi::board::{Board, Player};

fn bench_get_all_legal_moves(c: &mut Criterion) {
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

    c.bench_function("board::get_all_legal_moves", |b| {
        b.iter(|| {
            let board = black_box(&current_board);
            board.get_all_legal_moves(black_box(&Player::Second));
        });
    });
}

criterion_group!(board_benches, bench_get_all_legal_moves);
criterion_main!(board_benches);
