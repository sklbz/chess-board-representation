use crate::{
    bitboard::BitBoard,
    legal_moves::misc::{Move, Piece, Square},
};

pub struct Unmove {
    pub moves: Vec<Move>,
    pub castling_rights: [bool; 4],
    pub en_passant_square: Option<Square>,
    pub en_passant_pawn: Option<Piece>,
    pub en_passant: BitBoard,
}
