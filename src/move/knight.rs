use crate::{
    bitboard::BitBoard,
    legal_moves::misc::{Coord, Square, ToBitBoard},
};

pub fn knight_move_bitmask(knight_square: &Square, allies: &BitBoard) -> BitBoard {
    let mut mask: BitBoard = 0;

    if knight_square.file() > 1 {
        mask |= knight_square.to_bitboard() << (7 + 8);
        mask |= knight_square.to_bitboard() >> (9 + 8);
    }
    if knight_square.file() > 2 {
        mask |= knight_square.to_bitboard() << (8 - 2);
        mask |= knight_square.to_bitboard() >> (8 + 2);
    }

    if knight_square.file() < 6 {
        mask |= knight_square.to_bitboard() << (9 + 8);
        mask |= knight_square.to_bitboard() >> (7 + 8);
    }
    if knight_square.file() < 5 {
        mask |= knight_square.to_bitboard() << (8 + 2);
        mask |= knight_square.to_bitboard() >> (8 - 2);
    }

    if knight_square.rank() < 1 {
        mask &= !(knight_square.to_bitboard() >> (8 + 2));
        mask &= !(knight_square.to_bitboard() >> (8 - 2));
    }
    if knight_square.rank() < 2 {
        mask &= !(knight_square.to_bitboard() >> (7 + 8));
        mask &= !(knight_square.to_bitboard() >> (9 + 8));
    }

    if knight_square.rank() > 5 {
        mask &= !(knight_square.to_bitboard() << (7 + 8));
        mask &= !(knight_square.to_bitboard() << (9 + 8));
    }
    if knight_square.rank() < 7 {
        mask &= !(knight_square.to_bitboard() << (8 + 2));
        mask &= !(knight_square.to_bitboard() << (8 - 2));
    }

    mask & !allies
}
