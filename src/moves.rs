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
    Null,
}

pub(crate) struct Piece {
    pub(crate) r#type: Type,
    pub(crate) color: Color,
}

pub(crate) fn is_possible(board: &Board, r#move: &Move) -> bool {
    let start: Square = r#move.0;

    let piece = board.get_piece(&start);

    match piece.r#type {
        Type::King => println!("King"),
        Type::Queen => println!("Queen"),
        Type::Rook => println!("Rook"),
        Type::Bishop => println!("Bishop"),
        Type::Knight => println!("Knight"),
        Type::Pawn => println!("Pawn"),
        Type::None => println!("None"),
    }

    match (piece.r#type, piece.color) {
        (Type::None, _) | (_, Color::Null) => return false,
        _ => (),
    }

    true
}
