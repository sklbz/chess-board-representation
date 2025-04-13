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
    None,
}

pub(crate) enum Color {
    White,
    Black,
    None,
}

pub(crate) struct Piece {
    pub(crate) r#type: Type,
    pub(crate) color: Color,
}

pub(crate) fn is_possible(board: &Board, r#move: &Move) -> bool {
    let start: Square = r#move.0;

    let piece = board.get_piece(&start);

    match piece.r#type {
        Type::King => print!("King\t"),
        Type::Queen => print!("Queen\t"),
        Type::Rook => print!("Rook\t"),
        Type::Bishop => print!("Bishop\t"),
        Type::Knight => print!("Knight\t"),
        Type::Pawn => print!("Pawn\t"),
        Type::None => print!("None\t"),
    }

    match (piece.r#type, piece.color) {
        (Type::None, _) | (_, Color::None) => return false,
        _ => (),
    }

    true
}
