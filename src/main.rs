mod bitboard;
mod board;
mod moves;

use crate::board::Board;
use crate::moves::*;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let board = Board::init();

    for i in 0..64 {
        print!("{} ", i);
        is_possible(&board, &(i, 0));
        thread::sleep(Duration::from_millis(25));
    }

    println!("{:b}", &board.get_bitboard(Color::White, Type::Rook));
}
