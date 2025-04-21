use crate::{Square, bitboard::BitBoard};

pub fn mask_up(square: Square) -> BitBoard {
    let offset = square - (square % 8) + 8;
    u64::MAX << offset
}

pub fn mask_down(square: Square) -> BitBoard {
    let offset = square - 8;
    !mask_up(offset)
}
