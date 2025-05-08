use crate::bitboard::BitBoard;
use crate::board::Board;

use super::{
    generate_possible_moves::generate_move_mask,
    misc::{Color, Move, Square, ToBitBoard, Type},
};

pub fn is_possible(board: &Board, move_: &Move, color: Color) -> bool {
    let (start, end): (Square, Square) = *move_;
    let piece = board.get_piece(&start);

    if piece.r#type == Type::None {
        return false;
    }

    if piece.color == Color::Null {
        return false;
    }

    if piece.color != color {
        return false;
    }

    let move_mask: BitBoard = generate_move_mask(board, &start);

    move_mask & end.to_bitboard() != 0
}
