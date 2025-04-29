use crate::{
    bitboard::{BitBoard, BitBoardGetter, Display},
    board::Board,
    r#move::king::king_move_mask,
};

use super::{
    attack_mask::generate_attack_mask,
    misc::{Color, Square, Type},
};

pub fn get_check_direction(board: &Board, king: &Square, color: Color) -> u8 {
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

    for i in 0..direction_moves.len() {
        let mask = generate_attack_mask(
            board,
            &!color,
            &king,
            &(direction_mask & !direction_moves[i]),
        );

        if (1 << king) & mask != 0 {
            // DEBUG
            mask.display();
            board.display();

            return direction_offsets[i].unsigned_abs();
        }
    }

    println!("King must move!");

    0
}
