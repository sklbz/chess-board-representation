use crate::bitboard::BitBoardOperations;
use crate::{Square, bitboard::BitBoard, utils::min};

pub fn up_mask(square: Square) -> BitBoard {
    if square >= 64 - 8 {
        return 0;
    }

    let offset = min((square - (square % 8) + 8).into(), 63);
    u64::MAX << offset
}

pub fn down_mask(square: Square) -> BitBoard {
    if square < 8 {
        return 0;
    }

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

pub fn top_left_mask(square: Square) -> BitBoard {
    up_mask(square) & left_mask(square)
}

pub fn top_right_mask(square: Square) -> BitBoard {
    up_mask(square) & right_mask(square)
}

pub fn bottom_right_mask(square: Square) -> BitBoard {
    down_mask(square) & right_mask(square)
}

pub fn bottom_left_mask(square: Square) -> BitBoard {
    down_mask(square) & left_mask(square)
}

pub fn cross_mask(square: Square) -> BitBoard {
    !corners_mask(square)
}

fn corners_mask(square: Square) -> BitBoard {
    top_left_mask(square)
        | top_right_mask(square)
        | bottom_right_mask(square)
        | bottom_left_mask(square)
}

pub fn diagonal_cross_mask(square: Square) -> BitBoard {
    right_diagonal_mask(square) | left_diagonal_mask(square)
}

// ╱
pub fn right_diagonal_mask(square: Square) -> BitBoard {
    let main_diag = 0x8040_2010_0804_0201;
    let diag_shift = (square / 8).wrapping_sub(square % 8);

    if diag_shift < 8 {
        main_diag >> (8 * diag_shift)
    } else {
        main_diag << (8 * diag_shift.wrapping_neg())
    }
    .bitwise_reverse()
}

// ╲
pub fn left_diagonal_mask(square: Square) -> BitBoard {
    let main_diag = 0x0102_0408_1020_4080;
    let diag_shift = (square / 8).wrapping_sub(1 + square % 8);

    if diag_shift < 8 {
        main_diag >> (8 * diag_shift)
    } else {
        main_diag << (8 * diag_shift.wrapping_neg())
    }
    .bitwise_reverse()
}
