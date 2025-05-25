use crate::{
    bitboard::BitBoard,
    legal_moves::misc::{Color, Piece, Type},
};

use super::board::Board;

pub trait MaskHandler {
    fn from_mask(mask: BitBoard, piece: Piece) -> Self;
}

impl MaskHandler for Board {
    fn from_mask(mask: BitBoard, piece: Piece) -> Board {
        let Piece {
            r#type: piece_type,
            color,
        } = piece;

        let mut board = Board::empty();

        match (piece_type, color) {
            (Type::Pawn, Color::White) => {
                board.white_pawns = mask;
                board
            }
            (Type::Pawn, Color::Black) => {
                board.black_pawns = mask;
                board
            }
            (Type::Knight, Color::White) => {
                board.white_knights = mask;
                board
            }
            (Type::Knight, Color::Black) => {
                board.black_knights = mask;
                board
            }
            (Type::Bishop, Color::White) => {
                board.white_bishops = mask;
                board
            }
            (Type::Bishop, Color::Black) => {
                board.black_bishops = mask;
                board
            }
            (Type::Rook, Color::White) => {
                board.white_rooks = mask;
                board
            }
            (Type::Rook, Color::Black) => {
                board.black_rooks = mask;
                board
            }
            (Type::Queen, Color::White) => {
                board.white_queens = mask;
                board
            }
            (Type::Queen, Color::Black) => {
                board.black_queens = mask;
                board
            }
            (Type::King, Color::White) => {
                board.white_king = mask;
                board
            }
            (Type::King, Color::Black) => {
                board.black_king = mask;
                board
            }
            _ => panic!("Piece not found!"),
        }
    }
}
