use super::bishop::BishopMove;
use super::king::KingMove;
use super::knight::KnightMove;
use super::pawn::{PawnCapture, PawnCapturePattern, PawnMove};
use super::piece::{PieceType, Promotable};
use super::queen::QueenMove;
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
    Queen(QueenMove),
    King(KingMove),
}
pub enum ChessAction {
    PawnMove(PawnMove),
    PawnCapture(PawnCapture),
    Move(PieceMove),
    Capture {
        base: PieceMove,
        captured: PieceType,
    },
    Castle {
        side: CastleSide,
        color: ChessColor,
    },
    EnPassant(PawnCapturePattern),
    Promotion {
        file: ChessFile,
        color: ChessColor,
        promoted: Promotable,
    },
    PromotionCapture {
        file: ChessFile,
        pattern: PawnCapturePattern,
        captured: PieceType,
        promoted: Promotable,
    },
}
