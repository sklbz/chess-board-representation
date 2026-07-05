use super::bishop::BishopMove;
use super::knight::KnightMove;
use super::pawn::{PawnCapture, PawnCapturePattern, PawnMove};
use super::piece::{PieceType, Promotable};
use super::rook::RookMove;
use super::square::ChessFile;

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
    PawnMove(PawnMove),
    PawnCapture(PawnCapture),
    Move(PieceMove),
    Capture(PieceCapture),
    Castle {
        side: CastleSide,
        color: ChessColor,
    },
    EnPassant {},
    Promotion {
        file: ChessFile,
        color: ChessColor,
        promoted: Promotable,
    },
    PromotionCapture {
        file: ChessFile,
        pattern: PawnCapturePattern,
        captured: PieceType,
        color: ChessColor,
        promoted: Promotable,
    },
}
