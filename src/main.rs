mod bitboard;
mod board;
mod r#move;
mod moves;

use crate::bitboard::*;
use crate::board::*;
use crate::moves::*;

use std::thread;
use std::time::Duration;

fn main() {
    let board = Board::init();

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

    board.display();
}
