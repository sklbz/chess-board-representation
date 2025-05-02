use crate::{
    bitboard::{BitBoard, BitBoardGetter, Display},
    board::Board,
    legal_moves::misc::ToBitBoard,
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

    let ennemy_color = !color;

    for i in 0..direction_moves.len() {
        let mask = generate_attack_mask(
            board,
            &ennemy_color,
            &king.to_bitboard(),
            &(direction_mask & !direction_moves[i]),
        );

        if (1 << king) & mask != 0 {
            // DEBUG
            mask.display();
            board.display();

            return direction_offsets[i].unsigned_abs();
        }
    }

    let ennemy_horsey = board.get_bitboard(&ennemy_color, &Type::Knight);
    let mask_no_knights: u64 = generate_attack_mask(board, &ennemy_color, &ennemy_horsey, &0);

    if (1 << king) & mask_no_knights == 0 {
        return 0;
    }

    println!("King must move!");
    println!("Attack :");
    generate_attack_mask(board, &!color, &king.to_bitboard(), &0).display();
    board.display();

    0
}
