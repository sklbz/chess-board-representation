use crate::{bitboard::BitBoard, legal_moves::misc::Square};

pub fn knight_move_bitmask(knight_square: &Square, allies: &BitBoard) -> BitBoard {
    let horse_pattern: BitBoard =
        0b0000_0000_0000_0010_1000_0100_0100_0000_0000_0000_0100_0100_0010_1000_0000_0000; // movement
    // pattern for an e4 knight

    let e4: Square = 28;
    let offset: i8 = *knight_square as i8 - e4 as i8;

    if offset < 0 {
        return (horse_pattern << offset.unsigned_abs()) & !allies;
    }

    (horse_pattern >> offset.unsigned_abs()) & !allies
}
