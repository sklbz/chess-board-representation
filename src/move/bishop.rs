use crate::{Square, bitboard::BitBoard, bitmask::diagonal_cross_mask};

pub fn bishop_move_bitmask(
    square: &Square,
    allies_board: &BitBoard,
    ennemy_board: &BitBoard,
) -> BitBoard {
    diagonal_cross_mask(*square)
}
