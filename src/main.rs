mod bitboard;
mod board;
mod moves;

use crate::board::Board;
use crate::moves::is_possible;

fn main() {
    let board = Board::init();
    is_possible(board, &(0, 0));
    println!("Hello, world!");
}
