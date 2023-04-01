mod bitboard;
mod chess_move;
mod direction;
mod game;
// mod mcts;
mod piece;
// mod policy_network;
mod position;
// mod train;
// mod value_network;

use clap::{App, Arg};
use std::time::Instant;

use crate::game::Game;

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
                )
        )
        .subcommand(
            App::new("train")
                .about("Train value and policy networks using training data generated during self-play")
                .arg(
                    Arg::new("IDX")
                        .short('i')
                        .long("index")
                        .help("The indices under which the training data is stores")
                        .takes_value(true)
                        .multiple_values(true)
                        .required(true),
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("perft", _)) => {
            let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

            let runs = 3;
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
                println!("NPS: {}", nps);
                println!("\n");
            }

            println!("Average NPS: {}", (total_nps / runs as f64) as u64);

            println!("========\n========\n========\n");

            let runs = 3;
            let mut total_nps = 0.0;
            for _ in 0..runs {
                let now = Instant::now();
                let mut nodes = 0;
                for depth in 1..7 {
                    let moves = game.count_legal_moves2(depth);
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
                println!("NPS: {}", nps);
                println!("\n");
            }

            println!("Average NPS: {}", (total_nps / runs as f64) as u64);
            // To beat: 148_463_968
        }
        // Some(("mcts", sub_matches)) => {
        //     let run_index = sub_matches.value_of("IDX").unwrap().to_owned();
        //     let parallel_games = sub_matches
        //         .value_of("GAMES")
        //         .unwrap()
        //         .parse::<usize>()
        //         .unwrap();

        //     match mcts::run(run_index, parallel_games) {
        //         Err(err) => panic!("Running MCTS failed: {:?}", err),
        //         Ok(_) => {}
        //     }
        // }
        // Some(("train", sub_matches)) => {
        //     let run_indices: Vec<&str> = sub_matches.values_of("IDX").unwrap().collect();
        //     match train::run(run_indices) {
        //         Err(err) => panic!("Training failed: {:?}", err),
        //         Ok(_) => {}
        //     }
        // }
        _ => unreachable!(),
    };
}
