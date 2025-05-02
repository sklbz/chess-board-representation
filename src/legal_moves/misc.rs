use std::ops::Not;

use crate::bitboard::BitBoard;

pub type Square = u8;
pub type Move = (Square, Square);
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color {
    White,
    Black,
    Null,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece {
    pub r#type: Type,
    pub color: Color,
}
impl Piece {
    pub fn new(r#type: Type, color: Color) -> Self {
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

pub trait ToBitBoard {
    fn to_bitboard(&self) -> BitBoard;
}

impl ToBitBoard for Square {
    fn to_bitboard(&self) -> BitBoard {
        1 << self
    }
}

#[allow(unused)]
pub trait Coord {
    fn rank(&self) -> u8;
    fn file(&self) -> u8;
}

impl Coord for Square {
    fn rank(&self) -> u8 {
        self / 8
    }
    fn file(&self) -> u8 {
        self % 8
    }
}
