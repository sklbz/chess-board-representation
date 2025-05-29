use crate::{
    bitboard::{BitBoard, BitBoardGetter},
    board::board::Board,
    legal_moves::misc::ToBitBoard,
    r#move::king::king_move_mask,
};

use super::{
    attack_mask::generate_attack_mask,
    misc::{Color, Square, Type},
};

pub fn get_check_direction(board: &Board, king: &Square, color: Color) -> i8 {
    let direction_mask: BitBoard =
        king_move_mask(king, &0, &board.get_bitboard(&color, &Type::None));
    let direction_moves: Vec<BitBoard> = direction_mask
        .get_occupied_squares()
        .iter()
        .map(|square| 1 << square)
        .collect();
    let direction_offsets: Vec<i8> = direction_mask
        .get_occupied_squares()
        .iter()
        .map(|square| *square as i8 - *king as i8)
        .collect();

    let ennemy_color = !color;

    if !board.is_check(color) {
        return i8::MAX;
    }

    let ennemy_horsey = board.get_bitboard(&ennemy_color, &Type::Knight);
    let mask_no_knights: u64 = generate_attack_mask(board, &ennemy_color, &ennemy_horsey, &0);

    if (1 << king) & mask_no_knights == 0 {
        // 10 for knights
        return 10;
    }
    // Now I know that there is one check not from a knight

    let mask_knights_only: u64 =
        generate_attack_mask(board, &ennemy_color, &king.to_bitboard(), &direction_mask);

    if (1 << king) & mask_knights_only != 0 {
        // In case of double check, the king has to move
        return 0;
    }

    for i in 0..direction_moves.len() {
        let mask = generate_attack_mask(board, &ennemy_color, &0, &direction_moves[i]);

        if (1 << king) & mask == 0 {
            return direction_offsets[i];
        }
    }

    panic!("No check direction found");
}

pub fn get_checking_knight(board: &Board, color: Color, king: &BitBoard) -> BitBoard {
    let ennemy_color = !color;
    let ennemy_knights = board.get_bitboard(&ennemy_color, &Type::Knight);
    let knight_squares = ennemy_knights.get_occupied_squares();

    if ennemy_knights == 0 {
        return u64::MAX;
    }

    if !board.is_check(color) {
        return u64::MAX;
    }

    for square in knight_squares {
        let mask = generate_attack_mask(board, &ennemy_color, &(1 << square), &0);

        if king & mask == 0 {
            return 1 << square;
        }
    }

    // We probably are going to assume that there are multiple knights checking
    // (This is impossible during a game but can happen in the testing phase)

    let mask = generate_attack_mask(board, &ennemy_color, &ennemy_knights, &0);

    if king & mask == 0 {
        panic!("No checking knight found");
    }

    0
}
