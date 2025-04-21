mod bitboard;
mod bitmask;
mod board;
mod r#move;
mod moves;
mod utils;

use std::io::stdin;

use bitmask::mask_down;
use bitmask::mask_up;
use utils::min;
use utils::squarewise_display;
use utils::string_to_square;

use crate::board::*;
use crate::moves::*;

fn main() {
    let mut board = Board::init();

    let square_by_square_check: bool = false;

    if square_by_square_check {
        squarewise_display(&board);
    }

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

            let test = Board::from_mask(mask_up(square));

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

            let test = Board::from_mask(mask_down(square));

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
