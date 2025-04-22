use crate::{
    Board,
    bitmask::{down_mask, left_mask, right_mask, up_mask},
    utils::{min, string_to_square},
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

        if input == "quit\n" {
            break;
        }

        if input == "\n" {
            break;
        }

        if &input[0..min(input.len(), 7)] == "mask up" {
            let square: u64 = input
                .get(7..)
                .expect("Failed to extract square")
                .split_whitespace()
                .map(string_to_square)
                .collect::<Vec<u64>>()[0];

            let test = Board::from_mask(up_mask(square));

            test.display();

            break;
        }

        if &input[0..min(input.len(), 9)] == "mask down" {
            let square: u64 = input
                .get(9..)
                .expect("Failed to extract square")
                .split_whitespace()
                .map(string_to_square)
                .collect::<Vec<u64>>()[0];

            let test = Board::from_mask(down_mask(square));
            test.display();

            break;
        }

        if &input[0..min(input.len(), 9)] == "mask left" {
            let square: u64 = input
                .get(9..)
                .expect("Failed to extract square")
                .split_whitespace()
                .map(string_to_square)
                .collect::<Vec<u64>>()[0];

            let test = Board::from_mask(left_mask(square));
            test.display();

            break;
        }

        if &input[0..min(input.len(), 10)] == "mask right" {
            let square: u64 = input
                .get(10..)
                .expect("Failed to extract square")
                .split_whitespace()
                .map(string_to_square)
                .collect::<Vec<u64>>()[0];

            let test = Board::from_mask(right_mask(square));
            test.display();

            break;
        }

        let squares: Vec<u64> = input
            .split_whitespace()
            .map(string_to_square)
            .collect::<Vec<u64>>();

        board.play_move(&(squares[0], squares[1]));
    }
}
