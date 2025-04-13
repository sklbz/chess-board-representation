use crate::board::Board;

pub(crate) type Square = u64;
pub(crate) type Move = (Square, Square);
pub(crate) enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub(crate) enum Color {
    White,
    Black,
}

pub(crate) struct Piece {
    r#type: Type,
    color: Color,
}

pub(crate) fn is_possible(board: &Board, r#move: &Move) -> bool {
    let start: Square = r#move.0;

    let piece = board.get_piece(&start);

    true
}
