use crate::board::Board;
use crate::legal_moves::generate_possible_moves::generate_move_mask;
use crate::legal_moves::is_move_possible::is_possible;
use crate::legal_moves::misc::{Color, Piece, Type};
use crate::utils::string_to_square;

#[test]
fn test_knight_moves_center() {
    let knight_square = 28; // e4 (center of board)
    let board = Board::from_mask(1 << knight_square, Piece::new(Type::Knight, Color::White));

    assert_eq!(generate_move_mask(&board, &knight_square).count_ones(), 8);

    let valid_moves = [
        string_to_square("d6"),
        string_to_square("f6"),
        string_to_square("g5"),
        string_to_square("g3"),
        string_to_square("f2"),
        string_to_square("d2"),
        string_to_square("c3"),
        string_to_square("c5"),
    ];

    for &target in &valid_moves {
        assert!(
            is_possible(&board, &(knight_square, target)),
            "Knight should be able to move from {} to {}",
            knight_square,
            target
        );
    }
}

#[test]
fn test_knight_moves_edge() {
    let knight_square = 0; // a1 (corner)
    let board = Board::new(0, 0, 1 << knight_square, 0, 0, 0, 0, 0, 0, 0, 0, 0);

    assert_eq!(generate_move_mask(&board, &knight_square).count_ones(), 2);
}

#[test]
fn test_knight_blocked_by_own_pieces() {
    let knight_square = 28; // e4
    // Block one of the knight's moves with a white pawn
    let blocked_square = knight_square + 15;
    let board = Board::new(
        1 << blocked_square,
        0,
        1 << knight_square,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    );

    assert!(
        !is_possible(&board, &(knight_square, blocked_square)),
        "Knight shouldn't be able to capture own piece at {}",
        blocked_square
    );
}
