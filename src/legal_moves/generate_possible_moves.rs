use crate::{
    bitboard::{BitBoard, BitBoardGetter},
    bitmask::{down_mask, left_diagonal_mask, left_mask, right_diagonal_mask, right_mask, up_mask},
    board::Board,
    r#move::{
        bishop::bishop_move_bitmask,
        king::king_move_mask,
        knight::knight_move_bitmask,
        pawn::{pawn_attack_black, pawn_attack_white, pawn_move_black, pawn_move_white},
        queen::queen_move_bitmask,
        rook::rook_move_bitmask,
    },
    utils::mask_to_moves,
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
    if board.get_piece(start).r#type == Type::King {
        return generate_pseudo_move_mask(board, start);
    }

    let king_bitboard = board.get_bitboard(&board.get_piece(start).color, &Type::King);

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

    let normal_attack = generate_attack_mask(board, &!board.get_piece(start).color, &king_square);

    if normal_attack & (1 << king_square) != 0 {
        panic!("King is under attack");
    }

    let attack_mask = generate_attack_mask(board, &!board.get_piece(start).color, start);

    if attack_mask & (1 << king_square) == 0 {
        return generate_pseudo_move_mask(board, start);
    }

    let offset = king_square as i8 - *start as i8;

    let direction = get_direction(offset);

    let protection_mask: BitBoard = match direction {
        1 => !(up_mask(*start) | down_mask(*start)),
        7 => left_diagonal_mask(*start),
        8 => !(left_mask(*start) | right_mask(*start)),
        9 => right_diagonal_mask(*start),
        _ => unreachable!(),
    };

    generate_pseudo_move_mask(board, start) & protection_mask
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

pub fn generate_attack_mask(board: &Board, color: &Color, cleared_piece: &Square) -> BitBoard {
    let pieces: BitBoard = board.get_bitboard(color, &Type::None) & !(1 << cleared_piece);

    pieces
        .get_occupied_squares()
        .iter()
        .map(|square| generate_attack_mask_single_square(board, square))
        .fold(0, |acc, x| acc | x)
}

pub fn generate_attack_mask_single_square(board: &Board, start: &Square) -> BitBoard {
    let piece = board.get_piece(start);

    if piece.r#type == Type::None || piece.color == Color::Null {
        return 0;
    }

    let pawn_attack: &dyn Fn(&Square, &BitBoard) -> BitBoard = match &piece.color {
        Color::White => &pawn_attack_white,
        Color::Black => &pawn_attack_black,
        _ => &|_, _| 0,
    };

    let blockers = board.get_bitboard(&Color::Null, &Type::None);

    match piece.r#type {
        Type::Pawn => pawn_attack(start, &u64::MAX),
        Type::Rook => rook_move_bitmask(start, &0, &blockers),
        Type::Bishop => bishop_move_bitmask(start, &0, &blockers),
        Type::Queen => queen_move_bitmask(start, &0, &blockers),
        Type::Knight => knight_move_bitmask(start, &0),
        Type::King => king_move_mask(start, &0, &0),
        _ => 0,
    }
}
