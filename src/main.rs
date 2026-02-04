mod bitboard;
mod bitmask;
mod board;
mod debug;
mod game;
mod legal_moves;
mod r#move;
mod test;
mod utils;

use crate::board::fen_handling::FenHandling;
use board::board::Board;
use debug::divide::divide;
use legal_moves::is_move_possible::is_possible;
use legal_moves::misc::Color;
use utils::{string_to_move, user_input};

fn main() {
    // let mut _board = Board::init();
    let mut _board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq");
    let mut turn = Color::White;

    loop {
        _board.display();

        let input = user_input();

        if !is_possible(&_board, &string_to_move(&input), turn) {
            println!("Invalid move");
            continue;
        }

        let _ = _board.make_move_str(&input);

        turn = !turn;
    }

    //DEBUG---------------------------------------------------
    /* let mut board = Board::init();

        let game = "e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 g1f3 d6e5 f3e5 c7c6
            f1d3 b8d7 e5f3 d7f6 h2h3 d5b4 d3e2 c8f5 b1a3 e7e6 c2c3 b4d5 a3c4 f8e7
            O-O O-O c4e5 c6c5 a2a3 h7h6 c3c4 d5b6 d4c5 e7c5 b2b4 c5e7 c1b2 d8c7
            d1b3 a7a5 a1d1 a5b4 a3b4 f8c8 b2d4 b6d7 c4c5 f6d5 e2c4 d7f6 f1e1 b7b6
            c5b6 d5b6 d4b6 c7b6 e5f7 g8f7 c4e6 f5e6 e1e6 c8c3 e6f6 f7f6 b3c3 f6f7
            f3e5 f7g8 c3c4 g8h8 c4e4 a8e8 e5g6 h8g8 g6e7 g8f7 d1e1 b6d6";

        let mut turn = Color::White;

        for (move_, i) in game.split_whitespace().zip(1..) {
            if move_.starts_with('O') {
                let _ = board.castle(
                    move_,
                    if let 1 = i % 2 {
                        &Color::White
                    } else {
                        &Color::Black
                    },
                );

                turn = !turn;
                continue;
            }

            let algebric_move = string_to_move(move_);

            assert!(
                is_possible(&board, &algebric_move, turn),
                "Failed at move {i}: {}",
                move_
            );
            let _ = board.play_move(&algebric_move);

            turn = !turn;
        }
        //--------------------------------------------------------

        let mut board = Board::init();

        let square_by_square_check: bool = false;

        if square_by_square_check {
            squarewise_display(&board);
        }

        let input = user_input();

        match input.as_str() {
            "uci" => uci(),
            "tree" => tree("".to_string(), 4),
            "run" => run(&mut board),
            "board" => squarewise_display(&board),
            _ => println!("Invalid input"),
        }
    */
}

use std::fs::File;

#[allow(unused)]
fn tree(moves: String, depth: usize) {
    let mut board = Board::init();

    for m in moves.split_whitespace() {
        let _ = board.make_move_str(m);
    }

    let turn = match moves.split_whitespace().count() % 2 {
        1 => Color::White,
        0 => Color::Black,
        _ => panic!("Invalid moves length"),
    };

    // let (stockfish_ref, result) = get_stockfish_output(board.to_fen(turn), depth);
    let stockfish_ref = get_stockfish_output(board.to_fen(turn), depth + 1);

    let result = divide(&board, turn, depth);

    /* for (move_, count) in result.iter() {
            println!(
                "{}: Engine => {}, Stockfish => {}",
                move_,
                count,
                stockfish_ref.iter().find(|(m, _)| m == move_).unwrap().1
            );
        }
    */

    if result == stockfish_ref {
        println!("OK");
        return;
    }

    for (move_, count) in result
        .iter()
        .filter(|(m, _)| stockfish_ref.iter().any(|(s, _)| s == m))
    {
        let reference = stockfish_ref.iter().find(|(m, _)| m == move_).unwrap();

        if *count != reference.1 {
            println!(
                "\nEngine: {} => {}\n Stockfish: {} => {}",
                move_, count, reference.0, reference.1
            );
            tree(format!("{} {}", moves, move_), depth - 1);
        }
    }

    for (move_, _) in result
        .iter()
        .filter(|(m, _)| !stockfish_ref.iter().any(|(s, _)| s == m))
    {
        println!("Extra path: {} {}", moves, move_);
        writeln!(
            File::open("./logs/perft_extra_paths.log").unwrap(),
            "{} {}",
            moves,
            move_
        );
    }

    for (move_, _) in stockfish_ref
        .iter()
        .filter(|(m, _)| !result.iter().any(|(s, _)| s == m))
    {
        println!("Missing path: {} {}", moves, move_);
        writeln!(
            File::open("./logs/perft_missing_paths.log").unwrap(),
            "{} {}",
            moves,
            move_
        );
    }
}

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
fn get_stockfish_output(fen: String, depth: usize) -> Vec<(String, usize)> /*, Vec<(String, usize)>)*/
{
    // Launch Stockfish process
    let mut stockfish = Command::new("stockfish")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // Get handles to stdin and stdout
    let stdin = stockfish.stdin.as_mut().unwrap();
    let stdout = stockfish.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    let mut send_command = |command: &str| {
        writeln!(stdin, "{}", command).unwrap();
    };

    let read_perft_output = || {
        let mut output = String::new();
        for line in reader.lines() {
            let line = line.unwrap();

            if line.starts_with("Nodes searched") {
                break;
            }

            if line.split_whitespace().count() > 4 {
                continue;
            }

            output.push_str(&line);
            output.push('\n');
        }

        output
    };

    send_command(&format!("position fen {}", fen));
    send_command(&format!("go perft {}", depth));

    /*
        // Stockfish is faster than our own perft so we will uses this property to speed up our tests
        let turn = match fen.split_whitespace().nth(1).unwrap() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid color"),
        };
        let fen_board = Board::from_fen(&fen);

        let custom_perft = divide(&fen_board, turn, depth);
    */

    sleep(Duration::from_millis(100));

    let perft_results = read_perft_output();
    // println!("{}", perft_results);

    send_command("quit");
    let _ = stockfish.wait();

    let formatted_perft = perft_results
        .trim()
        .split('\n')
        .map(|s| -> (String, usize) {
            let s = s.replace(":", "");
            let move_ = s
                .split_whitespace()
                .next()
                .unwrap_or_else(|| panic!("{}", s))
                .to_string();
            let count = s
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            (move_, count)
        })
        .collect::<Vec<(String, usize)>>();

    formatted_perft
}
