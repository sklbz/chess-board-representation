mod bitboard;
mod board;
mod r#move;
mod moves;
mod utils;

use std::io::stdin;

use utils::string_to_square;

use crate::board::*;
use crate::moves::*;

use std::thread;
use std::time::Duration;

fn main() {
    let mut board = Board::init();

    let square_by_square_check: bool = false;

    if square_by_square_check {
        for i in 0..64 {
            let letter = match i % 8 {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => unreachable!(),
            };
            print!("{}{}\t", letter, i / 8 + 1);
            is_possible(&board, &(i, 0));
            thread::sleep(Duration::from_millis(50));
        }
    }

    loop {
        if false {
            break;
        }
        board.display();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        if input == "quit\n" {
            break;
        }

        let squares: Vec<u64> = input
            .split_whitespace()
            .map(string_to_square)
            .collect::<Vec<u64>>();

        println!("{} {}", squares[0], squares[1]);

        board.play_move(&(squares[0], squares[1]));

        print!("{esc}[2J", esc = 27 as char);
    }
}
