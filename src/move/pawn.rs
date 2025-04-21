use crate::bitboard::*;

pub fn pawn_motion(pawn_board: &BitBoard, ennemys_board: &BitBoard) -> BitBoard {
    (pawn_board << 8) & !ennemys_board
}
