use std::time::Instant;

mod bitboard;

mod game;

fn main() {
    let game = game::game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let runs = 10;
    let mut total_nps = 0.0;
    for _ in 0..runs {
        let now = Instant::now();
        let mut nodes = 0;
        for depth in 1..7 {
            let moves = game.count_legal_moves(depth);
            nodes += moves;
            println!("Depth {}: {} moves", depth, moves);
        }
        let time = now.elapsed().as_nanos();
        let seconds = time as f64 / 1_000_000_000_f64;
        let nps = nodes as f64 / seconds;
        total_nps += nps;
        println!("");
        println!("Seconds: {}", seconds);
        println!("Nodes: {}", nodes);
        println!("NPS: {}", nps as u64);
        println!("\n");
    }

    println!("Average NPS: {}", (total_nps / runs as f64) as u64);
    // To beat: 68_485_356
}
