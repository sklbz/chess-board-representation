use crate::{
    bitboard::{BitBoard, BitBoardGetter},
    bitmask::{down_mask, left_diagonal_mask, left_mask, right_diagonal_mask, right_mask, up_mask},
    board::board::Board,
    r#move::{king::king_move_mask, knight::knight_move_bitmask, queen::queen_move_bitmask},
};

use super::{
    attack_mask::generate_attack_mask,
    king_check_direction::{get_check_direction, get_checking_knight},
    misc::{Color, Square, ToBitBoard, Type},
};

pub(super) fn get_direction(offset: i8) -> u8 {
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

pub(super) fn is_pre_pinned(board: &Board, start: &Square, king_square: &Square) -> bool {
    let protector_mask: BitBoard = queen_move_bitmask(
        king_square,
        &0,
        &board.get_bitboard(&Color::Null, &Type::None),
    );

    protector_mask & (1 << start) != 0
}

pub(super) fn is_pinned(board: &Board, start: &Square, king_square: &Square) -> bool {
    let ennemy_color = !board.get_piece(start).color;

    let defensive_mask = king_move_mask(king_square, &0, &0) & !start.to_bitboard();

    let attack_mask = generate_attack_mask(
        board,
        &ennemy_color,
        &(start.to_bitboard() | board.get_bitboard(&ennemy_color, &Type::Knight)),
        &defensive_mask,
    );

    attack_mask & (1 << king_square) != 0
}

pub(super) fn protection_mask(king_square: Square, start: &Square, is_pinned: bool) -> BitBoard {
    if !is_pinned {
        return u64::MAX;
    }

    let offset = king_square as i8 - *start as i8;
    let direction = get_direction(offset);

    match direction {
        1 => !(up_mask(*start) | down_mask(*start)),
        7 => left_diagonal_mask(*start),
        8 => !(left_mask(*start) | right_mask(*start)),
        9 => right_diagonal_mask(*start),
        _ => unreachable!("Invalid direction"),
    }
}

pub(super) fn deflection_mask(is_checked: bool, board: &Board, color: Color) -> BitBoard {
    if !is_checked {
        return u64::MAX;
    }

    let king: Square = board
        .get_bitboard(&color, &Type::King)
        .get_occupied_squares()[0];

    let king_vision: BitBoard =
        queen_move_bitmask(&king, &0, &board.get_bitboard(&Color::Null, &Type::None))
            | knight_move_bitmask(&king, &0);

    king_vision
        & match get_check_direction(board, &king, color) {
            0 => 0,
            1 => !(up_mask(king) | down_mask(king)),
            7 => left_diagonal_mask(king),
            8 => !(left_mask(king) | right_mask(king)),
            9 => right_diagonal_mask(king),
            10 => get_checking_knight(board, color, &board.get_bitboard(&color, &Type::King)),
            u8::MAX => u64::MAX,
            _ => unreachable!("Invalid check direction"),
        }
}
