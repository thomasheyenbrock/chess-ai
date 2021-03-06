mod bitboard;
mod chess_move;
mod game;
mod mcts;
mod piece;
mod policy_network;
mod position;
mod value_network;

use crate::game::Game;
use clap::{App, Arg};
use std::time::Instant;

fn main() {
    let matches = App::new("cheers")
        .about("A chess engine built in Rust that uses AI")
        .subcommand(App::new("perft").about("Run performance tests for move generation"))
        .subcommand(
            App::new("mcts")
                .about("Generate training data using self-play with Monte-Carlo Tree Search")
                .arg(
                    Arg::new("IDX")
                        .short('i')
                        .long("index")
                        .help("The index under which to store the training data")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("GAMES")
                        .short('g')
                        .long("games")
                        .help("The number of parallel games to simulate")
                        .takes_value(true)
                        .validator(|value| match value.parse::<u8>() {
                            Err(_) => Err("Must be an integer"),
                            Ok(_) => Ok(()),
                        })
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("perft", _)) => {
            let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

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
        Some(("mcts", sub_matches)) => {
            let run_index = sub_matches.value_of("IDX").unwrap().to_owned();
            let parallel_games = sub_matches
                .value_of("GAMES")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            match mcts::run(run_index, parallel_games) {
                Err(err) => panic!("Running MCTS failed: {:?}", err),
                Ok(_) => {}
            }
        }
        _ => unreachable!(),
    };
}
