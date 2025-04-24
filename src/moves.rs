use crate::{
    bitboard::BitBoard,
    board::Board,
    r#move::{
        bishop::bishop_move_bitmask,
        pawn::{pawn_move_black, pawn_move_white},
        queen::queen_move_bitmask,
        rook::rook_move_bitmask,
    },
};
use std::ops::Not;

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

    if piece.r#type == Type::None || piece.color == Color::Null {
        return false;
    }

    let pawn_move: &dyn Fn(&Square, &BitBoard, &BitBoard) -> BitBoard = match &piece.color {
        Color::White => &pawn_move_white,
        Color::Black => &pawn_move_black,
        _ => &|_, _, _| 0,
    };

    let allies: &BitBoard = &board.get_bitboard(&piece.color, &Type::None);
    let enemies: &BitBoard = &board.get_bitboard(&!piece.color, &Type::None);

    match piece.r#type {
        Type::Pawn => {
            let move_mask = pawn_move(&start, allies, enemies);

            let move_in_mask = end.to_bitboard() & move_mask != 0;
            return move_in_mask;
        }
        Type::Rook => {
            let move_mask = rook_move_bitmask(&start, allies, enemies);

            let move_in_mask = end.to_bitboard() & move_mask != 0;
            return move_in_mask;
        }
        Type::Bishop => {
            let move_mask = bishop_move_bitmask(&start, allies, enemies);

            let move_in_mask = end.to_bitboard() & move_mask != 0;
            return move_in_mask;
        }
        Type::Queen => {
            let move_mask = queen_move_bitmask(&start, allies, enemies);

            let move_in_mask = end.to_bitboard() & move_mask != 0;
            return move_in_mask;
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
