use std::time::Instant;

mod bitboard;

mod constants;

mod game;

fn main() {
    let c = constants::get();
    let game = game::game_from_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        &c,
    );
    let now = Instant::now();
    println!("{}", game.count_legal_moves(5, &c));
    println!("{}", now.elapsed().as_millis());
}
