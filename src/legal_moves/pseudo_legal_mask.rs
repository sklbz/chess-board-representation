use crate::{
    bitboard::BitBoard,
    board::Board,
    r#move::{
        bishop::bishop_move_bitmask,
        king::king_move_mask,
        knight::knight_move_bitmask,
        pawn::{pawn_move_black, pawn_move_white},
        queen::queen_move_bitmask,
        rook::rook_move_bitmask,
    },
};

use super::{
    generate_possible_moves::generate_attack_mask,
    misc::{Color, Square, Type},
};

pub fn generate_pseudo_move_mask(board: &Board, start: &Square) -> BitBoard {
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
        Type::King => king_move_mask(start, &generate_attack_mask(board, &ennemy, start), allies),
        _ => 0,
    }
}
