use crate::{
    bitboard::*,
    bitmask::{cross_mask, down_mask, left_mask, right_mask, up_mask},
    legal_moves::misc::*,
};

pub fn rook_move_bitmask(
    rook_square: &Square,
    allies_board: &BitBoard,
    ennemy_board: &BitBoard,
) -> BitBoard {
    let rook_mask = cross_mask(*rook_square) & !(rook_square.to_bitboard());
    let blockers: BitBoard = (allies_board | ennemy_board) & rook_mask;

    let block_mask = blockers
        .get_occupied_squares()
        .iter()
        .map(|square| -> BitBoard {
            if square / 8 > rook_square / 8 {
                return up_mask(*square);
            }

            if square / 8 < rook_square / 8 {
                return down_mask(*square);
            }

            if square % 8 > rook_square % 8 {
                return right_mask(*square);
            }

            if square % 8 < rook_square % 8 {
                return left_mask(*square);
            }

            0
        })
        .fold(*allies_board, |acc, mask| acc | mask);

    let move_mask: BitBoard = rook_mask & !block_mask;

    move_mask
}
