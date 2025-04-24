use crate::{Square, bitboard::BitBoard};

use super::{bishop::bishop_move_bitmask, rook::rook_move_bitmask};

pub fn queen_move_bitmask(
    square: &Square,
    allies_board: &BitBoard,
    ennemy_board: &BitBoard,
) -> BitBoard {
    bishop_move_bitmask(square, allies_board, ennemy_board)
        | rook_move_bitmask(square, allies_board, ennemy_board)
}
