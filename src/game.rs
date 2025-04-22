use crate::{
    Board,
    bitmask::{down_mask, left_mask, right_mask, top_left_mask, up_mask},
    utils::{extract_move, extract_square, min},
};

use std::io::stdin;

pub fn run(board: &mut Board) {
    loop {
        print!("{esc}[2J", esc = 27 as char);
        board.display();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        input.pop();
        input = input.trim().to_string();

        if input == "exit" {
            break;
        }

        if input == "quit" {
            break;
        }

        if input.is_empty() {
            break;
        }

        if &input[0..min(input.len(), 7)] == "mask up" {
            let square: u64 = extract_square(input, 7);

            let test = Board::from_mask(up_mask(square));

            test.display();

            break;
        }

        if &input[0..min(input.len(), 9)] == "mask down" {
            let square: u64 = extract_square(input, 9);

            let test = Board::from_mask(down_mask(square));
            test.display();

            break;
        }

        if &input[0..min(input.len(), 9)] == "mask left" {
            let square: u64 = extract_square(input, 9);

            let test = Board::from_mask(left_mask(square));
            test.display();

            break;
        }

        if &input[0..min(input.len(), 10)] == "mask right" {
            let square: u64 = extract_square(input, 10);

            let test = Board::from_mask(right_mask(square));
            test.display();

            break;
        }

        if &input[0..min(input.len(), 13)] == "mask top left" {
            let square: u64 = extract_square(input, 13);

            let test = Board::from_mask(top_left_mask(square));
            test.display();

            break;
        }

        board.play_move(&extract_move(input, 0));
    }
}
