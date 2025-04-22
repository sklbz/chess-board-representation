use crate::{Square, bitboard::BitBoard};

pub fn up_mask(square: Square) -> BitBoard {
    let offset = square - (square % 8) + 8;
    u64::MAX << offset
}

pub fn down_mask(square: Square) -> BitBoard {
    let offset = square - 8;
    !up_mask(offset)
}

pub fn left_mask(square: Square) -> BitBoard {
    let offset = square % 8;

    if offset == 0 {
        return 0;
    }

    let mut mask = 0;

    for i in 0..offset {
        mask |= 1 << i;
    }

    [8, 16, 32].iter().for_each(|i| {
        mask |= mask << i;
    });

    mask
}

pub fn right_mask(square: Square) -> BitBoard {
    let offset = square + 1;
    !left_mask(offset)
}
