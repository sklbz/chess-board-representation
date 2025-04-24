use std::ops::Not;

use crate::bitboard::BitBoard;

pub(crate) type Square = u8;
pub(crate) type Move = (Square, Square);
#[derive(PartialEq, Debug)]
pub(crate) enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

#[derive(PartialEq, Debug)]
pub(crate) enum Color {
    White,
    Black,
    Null,
}

#[derive(PartialEq, Debug)]
pub struct Piece {
    pub r#type: Type,
    pub color: Color,
}
impl Piece {
    pub(crate) fn new(r#type: Type, color: Color) -> Self {
        Self { r#type, color }
    }
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
