use crate::{bitboard::BitBoard, board::Board, r#move::rook::rook_move_bitmask};
use std::ops::Not;

pub(crate) type Square = u8;
pub(crate) type Move = (Square, Square);
#[derive(PartialEq)]
pub(crate) enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

#[derive(PartialEq)]
pub(crate) enum Color {
    White,
    Black,
    Null,
}

#[derive(PartialEq)]
pub(crate) struct Piece {
    pub(crate) r#type: Type,
    pub(crate) color: Color,
}

pub(crate) fn is_possible(board: &Board, r#move: &Move) -> bool {
    let (start, end): (Square, Square) = *r#move;

    let piece = board.get_piece(&start);

    match piece.r#type {
        Type::King => println!("King"),
        Type::Queen => println!("Queen"),
        Type::Rook => println!("Rook"),
        Type::Bishop => println!("Bishop"),
        Type::Knight => println!("Knight"),
        Type::Pawn => println!("Pawn"),
        Type::None => println!("None"),
    };

    match (piece.r#type, piece.color) {
        (Type::None, _) | (_, Color::Null) => return false,
        (Type::Rook, color) => {
            let move_mask = rook_move_bitmask(
                &start,
                &board.get_bitboard(&color, &Type::None),
                &board.get_bitboard(&!color, &Type::None),
            );

            if end.to_bitboard() & move_mask == 0 {
                return false;
            }

            return true;
        }
        _ => (),
    }

    true
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::Null => Color::Null,
        }
    }
}

pub(crate) trait ToBitBoard {
    fn to_bitboard(&self) -> BitBoard;
}

impl ToBitBoard for Square {
    fn to_bitboard(&self) -> BitBoard {
        1 << self
    }
}
