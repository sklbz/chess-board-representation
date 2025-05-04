use crate::{
    bitboard::{BitBoard, BitBoardGetter},
    bitmask::{down_mask, left_diagonal_mask, left_mask, right_diagonal_mask, right_mask, up_mask},
    board::Board,
    legal_moves::attack_mask::generate_attack_mask,
    r#move::{king::king_move_mask, queen::queen_move_bitmask},
    utils::mask_to_moves,
};

use super::{
    castle::castle_mask::castle_mask,
    helper_functions::{deflection_mask, get_direction, protection_mask},
};
use super::{helper_functions::is_pinned, misc::ToBitBoard};
use super::{
    helper_functions::is_pre_pinned,
    king_check_direction::{get_check_direction, get_checking_knight},
};
use super::{
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

    let king_bitboard = board.get_bitboard(&color, &Type::King);

    if king_bitboard == 0 {
        return generate_pseudo_move_mask(board, start);
    }

    let king_square = king_bitboard.get_occupied_squares()[0];

    if is_pre_pinned(board, start, &king_square) {
        return generate_pseudo_move_mask(board, start);
    }

    let is_checked = board.is_check(color);

    let is_pinned = is_pinned(board, start, &king_square);

    if !is_checked && !is_pinned {
        return generate_pseudo_move_mask(board, start);
    }

    let protection_mask: BitBoard = protection_mask(king_square, start, is_pinned);

    let deflection_mask: BitBoard = deflection_mask(is_checked, board, start, color);

    generate_pseudo_move_mask(board, start) & protection_mask & deflection_mask
}
