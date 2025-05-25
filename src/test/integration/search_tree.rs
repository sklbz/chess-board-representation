use crate::board::board::Board;
use crate::board::fen_handling::FenHandling;
use crate::debug::tree::*;
use crate::legal_moves::misc::Color;
use crate::utils::move_to_string;

#[test]
fn depth_4() {
    let init_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let depth: usize = 4;

    perft(&init_fen, "".to_string(), depth);
}

fn perft(fen: &String, moves: String, depth: usize) {
    let board = Board::init();

    let stockfish_ref = get_stockfish_output(board.to_fen(Color::White), depth);

    let SearchTree { root } = search_tree(&board, Color::White, depth);

    let result = root
        .iter()
        .map(|x| -> (String, usize) { (move_to_string(&x.move_), x.score) })
        .collect::<Vec<(String, usize)>>();

    /* for (move_, count) in result {
        let reference = stockfish_ref.iter().find(|(m, _)| *m == move_).unwrap();
        assert_eq!(
            count, reference.1,
            "\nDepth: {}\nEngine: {} => {}\n Stockfish: {} => {}",
            depth, move_, count, reference.0, reference.1
        );
    }
    */

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
            perft(fen, format!("{} {}", moves, move_), depth - 1);
        }
    }

    for (move_, _) in result
        .iter()
        .filter(|(m, _)| !stockfish_ref.iter().any(|(s, _)| s == m))
    {
        println!("Extra path: {} {}", moves, move_);
    }

    for (move_, _) in stockfish_ref
        .iter()
        .filter(|(m, _)| !result.iter().any(|(s, _)| s == m))
    {
        println!("Missing path: {} {}", moves, move_);
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
