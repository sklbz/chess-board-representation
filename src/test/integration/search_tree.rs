use crate::board::board::Board;
use crate::board::fen_handling::FenHandling;
use crate::debug::tree::*;
use crate::legal_moves::misc::Color;
use crate::utils::move_to_string;

#[test]
fn stockfish_comparison() {
    let init_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    let depth: usize = 3;
    println!();
    println!("\nDepth: {}", depth);
    println!();
    let search_tree_root = search_info(&init_fen, depth);
    perft(&search_tree_root, &init_fen, "".to_string(), depth);
}

#[test]
fn stockfish_comparison_alt() {
    let init_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq".to_string();
    let depth: usize = 3;
    println!();
    println!("\nDepth: {}", depth);
    println!();
    let search_tree_root = search_info(&init_fen, depth);
    perft(&search_tree_root, &init_fen, "".to_string(), depth);
    panic!();
}

#[test]
fn stockfish_comparison_alt_2() {
    let init_fen = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w -".to_string();
    // fails at 5, succeed at 3, unknown for 4
    let depth: usize = 3;
    println!();
    println!("\nDepth: {}", depth);
    println!();
    let search_tree_root = search_info(&init_fen, depth);
    perft(&search_tree_root, &init_fen, "".to_string(), depth);
    panic!();
}

fn search_info(fen: &str, depth: usize) -> Vec<SearchTreeNode> {
    let board = Board::from_fen(fen);

    let SearchTree { root } = search_tree(&board, Color::White, depth);

    root
}

fn perft(nodes: &[SearchTreeNode], fen: &String, moves: String, depth: usize) {
    if depth == 0 {
        return;
    }

    let stockfish_ref = get_stockfish_output(fen, depth);

    let result = nodes
        .iter()
        .map(|x| -> (String, usize, SearchTreeNode) {
            (move_to_string(&x.move_), x.score, x.clone())
        })
        .collect::<Vec<(String, usize, SearchTreeNode)>>();

    for (move_, count, node) in result
        .iter()
        .filter(|(m, _, _)| stockfish_ref.iter().any(|(s, _)| s == m))
    {
        let reference = stockfish_ref.iter().find(|(m, _)| m == move_).unwrap();

        if *count != reference.1 {
            /* println!(
                "\nEngine: {} => {}\n Stockfish: {} => {}",
                move_, count, reference.0, reference.1
            ); */
            perft(
                &node.children,
                &node.fen,
                format!("{} {}", moves, move_),
                depth - 1,
            );
        }

        /* else {
            println!("{}: {}", move_, count);
        } */
    }

    for (move_, _, _) in result
        .iter()
        .filter(|(m, _, _)| !stockfish_ref.iter().any(|(s, _)| s == m))
    {
        println!("Extra path: {} {}", moves, move_);
    }

    for (move_, _) in stockfish_ref
        .iter()
        .filter(|(m, _)| !result.iter().any(|(s, _, _)| s == m))
    {
        println!("Missing path: {} {}", moves, move_);
    }

    // let total = result.iter().fold(0, |acc, (_, count, _)| acc + count);

    // println!();
    // println!();
    // println!("Nodes searched: {}", total);
}

use core::panic;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
fn get_stockfish_output(fen: &String, depth: usize) -> Vec<(String, usize)> /*, Vec<(String, usize)>)*/
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

    let extra_time: u64 = if depth >= 6 { 50 * depth as u64 } else { 0 };

    sleep(Duration::from_millis(100 + depth as u64 * 10 + extra_time));

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
