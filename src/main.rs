mod bitboard;
mod bitmask;
mod board;
mod game;
mod r#move;
mod moves;
mod utils;

use game::run;
use utils::squarewise_display;

use crate::board::*;
use crate::moves::*;

fn main() {
    let mut board = Board::init();

    let square_by_square_check: bool = false;

    if square_by_square_check {
        squarewise_display(&board);
    }

    run(&mut board);
}
