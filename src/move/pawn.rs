use crate::bitboard::*;
use crate::legal_moves::misc::*;

pub fn pawn_move_white(
    pawn: &Square,
    allies_board: &BitBoard,
    ennemy_board: &BitBoard,
) -> BitBoard {
    pawn_motion_white(pawn, &(ennemy_board | allies_board)) | pawn_attack_white(pawn, ennemy_board)
}

pub fn pawn_motion_white(pawn: &Square, blockers: &BitBoard) -> BitBoard {
    let pawn_move = (pawn.to_bitboard() << 8) & !blockers;
    let pawn_double_move = (pawn_move << 8) & !blockers;

    if pawn <= &15
    /* h2 */
    {
        return pawn_move | pawn_double_move;
    }

    pawn_move
}

pub fn pawn_attack_white(pawn: &Square, ennemy_board: &BitBoard) -> BitBoard {
    let attack_pattern = match pawn.file() {
        0 => pawn.to_bitboard() << 9,
        7 => pawn.to_bitboard() << 7,
        _ => pawn.to_bitboard() << 9 | pawn.to_bitboard() << 7,
    };

    attack_pattern & ennemy_board
}

pub fn pawn_move_black(
    pawn: &Square,
    allies_board: &BitBoard,
    ennemy_board: &BitBoard,
) -> BitBoard {
    pawn_motion_black(pawn, &(ennemy_board | allies_board)) | pawn_attack_black(pawn, ennemy_board)
}

pub fn pawn_motion_black(pawn: &Square, blockers: &BitBoard) -> BitBoard {
    let pawn_move = (pawn.to_bitboard() >> 8) & !blockers;
    let pawn_double_move = (pawn_move >> 8) & !blockers;

    if pawn >= &48
    /* a7 */
    {
        return pawn_move | pawn_double_move;
    }

    pawn_move
}

pub fn pawn_attack_black(pawn: &Square, ennemy_board: &BitBoard) -> BitBoard {
    (pawn.to_bitboard() >> 9 | pawn.to_bitboard() >> 7) & ennemy_board
}
