use crate::{
    bitboard::{BitBoard, BitBoardGetter},
    board::Board,
    r#move::{
        bishop::bishop_move_bitmask,
        king::king_move_mask,
        knight::knight_move_bitmask,
        pawn::{pawn_attack_black, pawn_attack_white},
        queen::queen_move_bitmask,
        rook::rook_move_bitmask,
    },
};

use super::misc::{Color, Square, Type};

pub fn generate_attack_mask(
    board: &Board,
    color: &Color,
    cleared_piece: &Square,
    added_blockers: &BitBoard,
) -> BitBoard {
    let pieces: BitBoard = board.get_bitboard(color, &Type::None) & !(1 << cleared_piece);

    pieces
        .get_occupied_squares()
        .iter()
        .map(|square| {
            generate_attack_mask_single_square(board, square, cleared_piece, added_blockers)
        })
        .fold(0, |acc, x| acc | x)
}

pub fn generate_attack_mask_single_square(
    board: &Board,
    start: &Square,
    cleared_piece: &Square,
    added_blockers: &BitBoard,
) -> BitBoard {
    let piece = board.get_piece(start);

    if piece.r#type == Type::None || piece.color == Color::Null {
        return 0;
    }

    let pawn_attack: &dyn Fn(&Square, &BitBoard) -> BitBoard = match &piece.color {
        Color::White => &pawn_attack_white,
        Color::Black => &pawn_attack_black,
        _ => &|_, _| 0,
    };

    let blockers =
        (board.get_bitboard(&Color::Null, &Type::None) | added_blockers) & !(1 << cleared_piece);

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
