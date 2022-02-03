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
    let mut nodes = 0;
    for depth in 1..7 {
        let moves = game.count_legal_moves(depth, &c);
        nodes += moves;
        println!("Depth {}: {} moves", depth, moves);
    }
    let time = now.elapsed().as_nanos();
    let seconds = time as f64 / 1_000_000_000_f64;
    println!("");
    println!("Seconds: {}", seconds);
    println!("Nodes: {}", nodes);
    println!("NPS: {}", nodes / seconds as u64);
    // To beat: 12_413_253
}
