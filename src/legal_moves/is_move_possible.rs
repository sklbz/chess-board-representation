use crate::board::board::Board;
use crate::utils::string_to_square;
use crate::{bitboard::BitBoard, board::mask_handling::MaskHandler};

use super::{
    generate_possible_moves::generate_move_mask,
    misc::{Color, Move, Piece, Square, ToBitBoard, Type},
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

    /* if start == string_to_square("f5") {
        Board::from_mask(move_mask, Piece::new(Type::Pawn, Color::White)).display();
    } */

    let result: bool = move_mask & end.to_bitboard() != 0;

    /* if !result {
        board.display();
        Board::from_mask(move_mask, Piece::new(Type::Pawn, Color::White))
            .set(&start, piece)
            .set(&end, Piece::new(Type::Pawn, Color::Black))
            .display();
    } */

    result
}
