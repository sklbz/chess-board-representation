use crate::{Square, bitboard::BitBoard};

pub fn mask_up(square: Square) -> BitBoard {
    let offset = square - (square % 8) + 8;
    u64::MAX << offset
}
