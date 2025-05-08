use crate::{
    bitboard::{BitBoard, BitBoardGetter},
    board::Board,
    legal_moves::{
        attack_mask::{generate_attack_mask, generate_attack_mask_single_square},
        misc::{Color, Square, Type},
    },
    utils::{square_to_string, string_to_square},
};

pub fn castle_mask(board: &Board, color: Color) -> BitBoard {
    let home_square: Square = match color {
        Color::White => string_to_square("e1"),
        Color::Black => string_to_square("e8"),
        _ => panic!("Null king does not castle"),
    };

    let rook_squares: Vec<Square> = match color {
        Color::White => vec![string_to_square("a1"), string_to_square("h1")],
        Color::Black => vec![string_to_square("a8"), string_to_square("h8")],
        _ => panic!("Null king does not castle"),
    };

    let king_bit: BitBoard = board.get_bitboard(&color, &Type::King);

    let king: Square = king_bit.get_occupied_squares()[0];
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
                castle_mask |= 1 << (king + 2);
            }

            continue;
        }

        if !board.can_castle_queenside(color) {
            continue;
        }

        println!("5");
        let attack_mask: BitBoard = generate_attack_mask(board, &!color, &0, &0);
        let travel_mask: BitBoard = king_bit | king_bit >> 1 | king_bit >> 2;

        if attack_mask & travel_mask == 0 {
            castle_mask |= 1 << (king - 2);
        }
    }

    castle_mask
}
