use crate::legal_moves::attack_mask::generate_attack_mask;
use crate::{
    bitboard::{BitBoard, BitBoardGetter},
    bitmask::{down_mask, left_diagonal_mask, left_mask, right_diagonal_mask, right_mask, up_mask},
    board::Board,
    r#move::{king::king_move_mask, queen::queen_move_bitmask},
    utils::mask_to_moves,
};

use super::castle::castle_mask::castle_mask;
use super::king_check_direction::{get_check_direction, get_checking_knight};
use super::misc::ToBitBoard;
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

    let protector_mask: BitBoard = queen_move_bitmask(
        &king_square,
        &0,
        &board.get_bitboard(&Color::Null, &Type::None),
    );

    if protector_mask & (1 << start) == 0 {
        return generate_pseudo_move_mask(board, start);
    }

    let normal_attack = generate_attack_mask(
        board,
        &!board.get_piece(start).color,
        &king_square.to_bitboard(),
        &0,
    );

    let is_checked = normal_attack & (1 << king_square) != 0;

    let attack_mask = generate_attack_mask(
        board,
        &!board.get_piece(start).color,
        &(start.to_bitboard() | board.get_bitboard(&Color::Null, &Type::Knight)),
        &king_move_mask(&king_square, &0, &0),
    );

    let is_pinned = attack_mask & (1 << king_square) != 0;

    if !is_checked && !is_pinned {
        return generate_pseudo_move_mask(board, start);
    }

    let offset = king_square as i8 - *start as i8;

    let direction = get_direction(offset);

    let protection_mask: BitBoard = match is_pinned {
        true => match direction {
            1 => !(up_mask(*start) | down_mask(*start)),
            7 => left_diagonal_mask(*start),
            8 => !(left_mask(*start) | right_mask(*start)),
            9 => right_diagonal_mask(*start),
            _ => unreachable!(),
        },
        false => u64::MAX,
    };

    let color = board.get_piece(start).color;

    let deflection_mask: BitBoard = match is_checked {
        true => match get_check_direction(board, start, board.get_piece(start).color) {
            0 => 0,
            1 => !(up_mask(*start) | down_mask(*start)),
            7 => left_diagonal_mask(*start),
            8 => !(left_mask(*start) | right_mask(*start)),
            9 => right_diagonal_mask(*start),
            10 => get_checking_knight(board, color, &board.get_bitboard(&color, &Type::King)),
            u8::MAX => u64::MAX,
            _ => unreachable!(),
        },
        false => u64::MAX,
    };

    generate_pseudo_move_mask(board, start) & protection_mask & deflection_mask
}

fn get_direction(offset: i8) -> u8 {
    if offset < 7 {
        return 1;
    }

    if offset % 7 == 0 {
        return 7;
    }

    if offset % 8 == 0 {
        return 8;
    }

    if offset % 9 == 0 {
        return 9;
    }

    unreachable!()
}
