use crate::{
    bitboard::BitBoard,
    legal_moves::misc::{Coord, Square, ToBitBoard},
};

pub fn king_move_mask(king: &Square, attacked: &BitBoard, allies: &BitBoard) -> BitBoard {
    let king_bit = king.to_bitboard();

    let _ = " --------
             │1 │0 │7 │
             │--------│
             │2 │K │6 │
             │--------│
             │3 │4 │5 │
              -------- ";
    let individual_masks = [
        king_bit << 8,
        king_bit << 7,
        king_bit >> 1,
        king_bit >> 9,
        king_bit >> 8,
        king_bit >> 7,
        king_bit << 1,
        king_bit << 9,
    ];

    let mut mask: BitBoard = individual_masks.iter().fold(0, |acc, x| acc | *x);

    if king.file() == 0 {
        mask &= !individual_masks[1];
        mask &= !individual_masks[2];
        mask &= !individual_masks[3];
    }

    if king.file() == 7 {
        mask &= !individual_masks[5];
        mask &= !individual_masks[6];
        mask &= !individual_masks[7];
    }

    mask & !allies & !attacked
}
