use super::{bishop::BishopMove, knight::KnightMove, piece::PieceType, rook::RookMove};

pub enum ChessColor {
    White,
    Black,
}
pub enum CastleSide {
    KingSide,
    QueenSide,
}
pub enum PieceMove {
    Knight(KnightMove),
    Bishop(BishopMove),
    Rook(RookMove),
}
pub struct PieceCapture {
    base: PieceMove,
    captured: PieceType,
}
pub enum ChessAction {
    PawnMove {},
    PawnCapture {},
    Move(PieceMove),
    Capture(PieceCapture),
    Castle { side: CastleSide, color: ChessColor },
    EnPassant {},
    Promotion {},
}
