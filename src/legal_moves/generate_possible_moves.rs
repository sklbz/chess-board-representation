use crate::{
    Board,
    bitboard::{BitBoard, BitBoardGetter, Display},
    r#move::{
        bishop::bishop_move_bitmask,
        knight::knight_move_bitmask,
        pawn::{pawn_move_black, pawn_move_white},
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
    let enemies: &BitBoard = &board.get_bitboard(&!piece.color, &Type::None);

    match piece.r#type {
        Type::Pawn => pawn_move(start, allies, enemies),
        Type::Rook => rook_move_bitmask(start, allies, enemies),
        Type::Bishop => bishop_move_bitmask(start, allies, enemies),
        Type::Queen => queen_move_bitmask(start, allies, enemies),
        Type::Knight => knight_move_bitmask(start, allies),
        _ => 0,
    }
}
