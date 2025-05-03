use crate::bitboard::BitBoard;
use crate::board::Board;

use super::{
    generate_possible_moves::generate_move_mask,
    misc::{Color, Move, Square, ToBitBoard, Type},
};

pub fn is_possible(board: &Board, r#move: &Move) -> bool {
    let (start, end): (Square, Square) = *r#move;

    if board.get_piece(&start).r#type == Type::None {
        return false;
    }

    if board.get_piece(&start).color == Color::Null {
        return false;
    }

    let move_mask: BitBoard = generate_move_mask(board, &start);

    move_mask & end.to_bitboard() != 0
}
