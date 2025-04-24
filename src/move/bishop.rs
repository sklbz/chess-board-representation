use crate::{
    bitboard::{BitBoard, BitBoardGetter},
    bitmask::{
        bottom_left_mask, bottom_right_mask, diagonal_cross_mask, top_left_mask, top_right_mask,
    },
    legal_moves::misc::*,
};

pub fn bishop_move_bitmask(
    square: &Square,
    allies_board: &BitBoard,
    ennemy_board: &BitBoard,
) -> BitBoard {
    let blockers_mask =
        (allies_board | ennemy_board) & !square.to_bitboard() & diagonal_cross_mask(*square);

    let block_mask = blockers_mask
        .get_occupied_squares()
        .iter()
        .map(|blocker_square: &Square| {
            if blocker_square == square {
                return 0;
            }

            if blocker_square < square {
                if blocker_square % 8 < square % 8 {
                    return bottom_left_mask(*blocker_square);
                }
                return bottom_right_mask(*blocker_square);
            }

            if blocker_square % 8 < square % 8 {
                return top_left_mask(*blocker_square);
            }
            top_right_mask(*blocker_square)
        })
        .fold(*allies_board, |acc: BitBoard, mask: BitBoard| acc | mask);
    diagonal_cross_mask(*square) & !block_mask
}
