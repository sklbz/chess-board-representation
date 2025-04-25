use crate::{
    bitboard::BitBoard,
    legal_moves::misc::{Coord, Square, ToBitBoard},
};

pub fn knight_move_bitmask(knight_square: &Square, allies: &BitBoard) -> BitBoard {
    let knight_bb = knight_square.to_bitboard();

    let _ = " --------------
             │  │2 │  │0 │  │
             │--------------│
             │6 │  │  │  │4 │
             │--------------│
             │  │  │󰡘 │  │  │
             │--------------│
             │5 │  │  │  │7 │
             │--------------│
             │  │1 │  │3 │  │
              -------------- ";
    let individual_masks = [
        knight_bb << 17,
        knight_bb >> 17,
        knight_bb << 15,
        knight_bb >> 15,
        knight_bb << 10,
        knight_bb >> 10,
        knight_bb << 6,
        knight_bb >> 6,
    ];

    let mut mask: BitBoard = individual_masks.iter().fold(0, |acc, x| acc | *x);

    if knight_square.file() == 0 {
        mask &= !individual_masks[1];
        mask &= !individual_masks[2];
    }
    if knight_square.file() < 2 {
        mask &= !individual_masks[5];
        mask &= !individual_masks[6];
    }

    if knight_square.file() > 5 {
        mask &= !individual_masks[4];
        mask &= !individual_masks[7];
    }
    if knight_square.file() == 7 {
        mask &= !individual_masks[0];
        mask &= !individual_masks[3];
    }

    mask & !allies
}
