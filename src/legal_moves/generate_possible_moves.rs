use crate::{
    bitboard::{BitBoard, BitBoardGetter},
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

use super::misc::{Color, Move, Square, Type};

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
    let piece = board.get_piece(start);

    if piece.r#type == Type::None || piece.color == Color::Null {
        return 0;
    }

    let pawn_move: &dyn Fn(&Square, &BitBoard, &BitBoard) -> BitBoard = match &piece.color {
        Color::White => &pawn_move_white,
        Color::Black => &pawn_move_black,
        _ => &|_, _, _| 0,
    };

    let allies: &BitBoard = &board.get_bitboard(&piece.color, &Type::None);
    let ennemy: Color = !piece.color;
    let enemies: &BitBoard = &board.get_bitboard(&ennemy, &Type::None);

    match piece.r#type {
        Type::Pawn => pawn_move(start, allies, enemies),
        Type::Rook => rook_move_bitmask(start, allies, enemies),
        Type::Bishop => bishop_move_bitmask(start, allies, enemies),
        Type::Queen => queen_move_bitmask(start, allies, enemies),
        Type::Knight => knight_move_bitmask(start, allies),
        Type::King => king_move_mask(start, &generate_attack_mask(board, &ennemy), allies),
        _ => 0,
    }
}

pub fn generate_attack_mask(board: &Board, color: &Color) -> BitBoard {
    let pieces: BitBoard = board.get_bitboard(color, &Type::None);

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
