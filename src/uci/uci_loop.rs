use crate::board::fen_handling::FenHandling;
use crate::{
    board::board::Board,
    debug::divide::divide,
    legal_moves::{is_move_possible::is_possible, misc::Color},
    utils::{string_to_move, user_input},
};

pub fn uci() {
    let mut board = Board::init();
    let mut play_turn = Color::White;
    loop {
        let input = user_input();

        println!();

        let input_action = input.split_whitespace().next().unwrap();
        let input_args = input.split_whitespace().skip(1).collect::<Vec<_>>();

        if input == "quit" {
            break;
        }

        match input_action {
            "startpos" => {
                board = Board::init();
                play_turn = Color::White;
            }
            "fen" => board = Board::from_fen(input_args[0]),
            "board" => board.display(),
            "ischeck" => println!("{}", board.is_check(play_turn)),
            "move" => {
                for move_ in input_args {
                    if !is_possible(&board, &string_to_move(move_), play_turn) {
                        println!("Illegal move: {}", move_);
                        break;
                    }

                    play_turn = !play_turn;
                    board.play_move(&string_to_move(move_));
                }
            }
            "perft" => {
                let depth = input_args[0].parse::<usize>().unwrap();

                let result = divide(&board, play_turn, depth - 1);
                let nodes = result.iter().fold(0, |acc, (_, leaves)| acc + leaves);

                for (move_, leaves) in result {
                    println!("{}: {}", move_, leaves);
                }

                println!("Nodes searched: {}", nodes);
            }
            "turn" => match play_turn {
                Color::White => println!("White to move"),
                Color::Black => println!("Black to move"),
                _ => println!("Unknown turn"),
            },
            "clear" => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
            _ => println!("Unknown command"),
        };
    }
}
