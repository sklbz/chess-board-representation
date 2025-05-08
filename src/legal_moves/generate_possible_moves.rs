use crate::{
    bitboard::{BitBoard, BitBoardGetter, Display},
    board::Board,
    utils::mask_to_moves,
};

use super::{
    castle::castle_mask::castle_mask,
    helper_functions::is_pinned,
    helper_functions::is_pre_pinned,
    helper_functions::{deflection_mask, protection_mask},
    misc::{Color, Move, Square, Type},
    pseudo_legal_mask::generate_pseudo_move_mask,
};

pub fn generate_move_vec(board: &Board, color: Color) -> Vec<Move> {
    let occupied_squares: BitBoard = board.get_bitboard(&color, &Type::None);

    let moves: Vec<Move> = occupied_squares
        .get_occupied_squares()
        .iter()
        .flat_map(|square| generate_move_vec_single_square(board, square))
        .collect();

    moves
}

pub fn generate_move_vec_single_square(board: &Board, start: &Square) -> Vec<Move> {
    mask_to_moves(generate_move_mask(board, start), start)
}

pub fn generate_move_mask(board: &Board, start: &Square) -> BitBoard {
    let color = board.get_piece(start).color;
    let piece_type = board.get_piece(start).r#type;

    if piece_type == Type::King {
        return generate_pseudo_move_mask(board, start) | castle_mask(board, color);
    }

    let king_square = match board.get_bitboard(&color, &Type::King) {
        0 => return generate_pseudo_move_mask(board, start),
        king_bit => king_bit.get_occupied_squares()[0],
    };

    let is_checked = board.is_check(color);

    if !is_checked && !is_pre_pinned(board, start, &king_square) {
        return generate_pseudo_move_mask(board, start);
    }

    let is_pinned = is_pinned(board, start, &king_square);

    if !is_checked && !is_pinned {
        return generate_pseudo_move_mask(board, start);
    }

    if is_checked && is_pinned {
        return 0;
    }

    let protection_mask: BitBoard = protection_mask(king_square, start, is_pinned);

    let deflection_mask: BitBoard = deflection_mask(is_checked, board, color);

    let mask: BitBoard =
        generate_pseudo_move_mask(board, start) & protection_mask & deflection_mask;

    assert!(
        mask & !protection_mask == 0,
        "Move mask should be a subset of the protection mask"
    );

    assert!(
        mask & !deflection_mask == 0,
        "Move mask should be a subset of the deflection mask"
    );

    mask
}
