mod bitboard;
mod bitmask;
mod board;
mod debug;
mod game;
mod legal_moves;
mod r#move;
mod test;
mod utils;

// use debug::knight_movement::run_debug;
use chess::uci::uci_loop::uci;
use game::game_loop::run;
use utils::squarewise_display;

use crate::board::*;

fn main() {
    let mut board = Board::init();

    let square_by_square_check: bool = false;

    if square_by_square_check {
        squarewise_display(&board);
    }

    // run(&mut board);
    // run_debug();
    uci();
}
