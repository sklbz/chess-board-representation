use crate::bitboard::*;
use crate::legal_moves::misc::*;

pub fn pawn_move_white(
    pawn: &Square,
    allies_board: &BitBoard,
    ennemy_board: &BitBoard,
) -> BitBoard {
    (pawn_motion_white(pawn, ennemy_board) & !allies_board) | pawn_attack_white(pawn, ennemy_board)
}

pub fn pawn_motion_white(pawn: &Square, ennemy_board: &BitBoard) -> BitBoard {
    if pawn <= &15
    /* h2 */
    {
        return (pawn.to_bitboard() << 8 | pawn.to_bitboard() << 16) & !ennemy_board;
    }
    (pawn.to_bitboard() << 8) & !ennemy_board
}

pub fn pawn_attack_white(pawn: &Square, ennemy_board: &BitBoard) -> BitBoard {
    (pawn.to_bitboard() << 9 | pawn.to_bitboard() << 7) & ennemy_board
}

pub fn pawn_move_black(
    pawn: &Square,
    allies_board: &BitBoard,
    ennemy_board: &BitBoard,
) -> BitBoard {
    (pawn_motion_black(pawn, ennemy_board) & !allies_board) | pawn_attack_black(pawn, ennemy_board)
}

pub fn pawn_motion_black(pawn: &Square, ennemy_board: &BitBoard) -> BitBoard {
    if pawn >= &48
    /* a7 */
    {
        return (pawn.to_bitboard() >> 8 | pawn.to_bitboard() >> 16) & !ennemy_board;
    }
    (pawn.to_bitboard() >> 8) & !ennemy_board
}

pub fn pawn_attack_black(pawn: &Square, ennemy_board: &BitBoard) -> BitBoard {
    (pawn.to_bitboard() >> 9 | pawn.to_bitboard() >> 7) & ennemy_board
}
