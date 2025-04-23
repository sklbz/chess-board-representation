use crate::{Square, ToBitBoard, bitboard::*, bitmask::cross_mask};

pub fn rook_move_bitmask(
    rook_square: &Square,
    _allies_board: &BitBoard,
    _ennemy_board: &BitBoard,
) -> BitBoard {
    let move_mask: BitBoard = cross_mask(*rook_square) & !(rook_square.to_bitboard());

    move_mask
}
