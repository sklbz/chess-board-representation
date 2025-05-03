use crate::{
    board::Board,
    debug::divide::divide,
    legal_moves::{is_move_possible::is_possible, misc::Color},
    utils::{string_to_move, user_input},
};

pub fn uci() {
    let mut board = Board::init();
    let mut play_turn = Color::White;
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

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
            "board" => board.display(),
            "move" => {
                for move_ in input_args {
                    if !is_possible(&board, &string_to_move(move_)) {
                        play_turn = !play_turn;
                        println!("Illegal move: {}", move_);
                        break;
                    }

                    board.play_move(&string_to_move(move_));
                }
            }
            "divide" => {
                let depth = input_args[0].parse::<usize>().unwrap();

                let result = divide(&board, play_turn, depth - 1);

                for (move_, score) in result {
                    println!("{} {}", move_, score);
                }
            }
            "turn" => match play_turn {
                Color::White => println!("White to move"),
                Color::Black => println!("Black to move"),
                _ => println!("Unknown turn"),
            },
            _ => println!("Unknown command"),
        };
    }
}
