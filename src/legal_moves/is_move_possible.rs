use crate::bitboard::BitBoard;
use crate::board::Board;

use super::{
    generate_possible_moves::generate_move_mask,
    misc::{Move, Square, ToBitBoard},
};

pub fn is_possible(board: &Board, r#move: &Move) -> bool {
    let (start, end): (Square, Square) = *r#move;

    let move_mask: BitBoard = generate_move_mask(board, &start);

    move_mask & end.to_bitboard() != 0
}
