mod bitboard;
mod bitmask;
mod board;
mod game;
mod legal_moves;
mod r#move;
mod test;
mod utils;

use game::game_loop::run;
use utils::squarewise_display;

use crate::board::*;

fn main() {
    let mut board = Board::init();

    let square_by_square_check: bool = false;

    if square_by_square_check {
        squarewise_display(&board);
    }

    run(&mut board);
}
