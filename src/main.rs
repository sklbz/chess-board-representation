mod bitboard;
mod board;
mod moves;

use crate::board::Board;
use crate::moves::is_possible;

fn main() {
    println!("Hello, world!");

    let board = Board::init();

    for i in 0..64 {
        print!("{} ", i);
        is_possible(&board, &(i, 0));
    }
}
