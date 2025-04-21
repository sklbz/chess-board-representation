use crate::bitboard::*;

pub fn pawn_move(
    pawn_board: &BitBoard,
    allies_board: &BitBoard,
    ennemy_board: &BitBoard,
) -> BitBoard {
    (pawn_motion(pawn_board, ennemy_board) & !allies_board) | pawn_attack(pawn_board, ennemy_board)
}

pub fn pawn_motion(pawn_board: &BitBoard, ennemy_board: &BitBoard) -> BitBoard {
    (pawn_board << 8) & !ennemy_board
}

pub fn pawn_attack(pawn_board: &BitBoard, ennemy_board: &BitBoard) -> BitBoard {
    (pawn_board << 9 | pawn_board << 7) & ennemy_board
}
