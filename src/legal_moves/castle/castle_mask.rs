use crate::{
    bitboard::{BitBoard, BitBoardGetter},
    board::Board,
    legal_moves::{
        attack_mask::{self, generate_attack_mask, generate_attack_mask_single_square},
        misc::{Color, Square, Type},
    },
};

pub fn castle_mask(board: &Board, color: Color) -> BitBoard {
    let home_square: Square = match color {
        Color::White => 4,
        Color::Black => 60,
        _ => panic!("Null king does not castle"),
    };

    let rook_squares: Vec<Square> = match color {
        Color::White => vec![0, 7],
        Color::Black => vec![56, 63],
        _ => panic!("Null king does not castle"),
    };

    let king: Square = board
        .get_bitboard(&color, &Type::King)
        .get_occupied_squares()[0];
    let rooks: Vec<Square> = board
        .get_bitboard(&color, &Type::Rook)
        .get_occupied_squares();

    let valid_rooks: Vec<&Square> = rooks
        .iter()
        .filter(|&rook| rook_squares.contains(rook))
        .collect();

    if king != home_square {
        return BitBoard::empty();
    }

    let mut castle_mask = 0;

    let king_bit = 1 << king;

    for rook in valid_rooks.iter() {
        let mask: BitBoard = generate_attack_mask_single_square(board, rook, &(1 << king), &0);

        if mask & king_bit == 0 {
            continue;
        }

        if *rook > &king {
            if !board.can_castle_kingside(color) {
                continue;
            }

            let travel_mask: BitBoard = king_bit | king_bit << 1 | king_bit << 2;
            let attack_mask: BitBoard = generate_attack_mask(board, &!color, &0, &0);

            if attack_mask & travel_mask == 0 {
                castle_mask |= 1 << (king - 2);
            }

            continue;
        }

        if !board.can_castle_queenside(color) {
            continue;
        }

        let attack_mask: BitBoard = generate_attack_mask(board, &!color, &0, &0);
        let travel_mask: BitBoard = king_bit | king_bit >> 1 | king_bit >> 2;

        if attack_mask & travel_mask == 0 {
            castle_mask |= 1 << (king + 2);
        }
    }

    castle_mask
}
