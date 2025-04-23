use crate::Square;
use crate::ToBitBoard;
use crate::bitboard::*;

pub fn pawn_move(pawn: &Square, allies_board: &BitBoard, ennemy_board: &BitBoard) -> BitBoard {
    (pawn_motion(pawn, ennemy_board) & !allies_board) | pawn_attack(pawn, ennemy_board)
}

pub fn pawn_motion(pawn: &Square, ennemy_board: &BitBoard) -> BitBoard {
    (pawn.to_bitboard() << 8) & !ennemy_board
}

pub fn pawn_attack(pawn: &Square, ennemy_board: &BitBoard) -> BitBoard {
    (pawn.to_bitboard() << 9 | pawn.to_bitboard() << 7) & ennemy_board
}
