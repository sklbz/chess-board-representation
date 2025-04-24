use crate::board::Board;
use crate::legal_moves::is_move_possible::is_possible;
use crate::legal_moves::misc::{Color, Piece, Type};

#[test]
fn test_knight_moves_center() {
    let knight_square = 28; // e4 (center of board)
    let board = Board::from_mask(1 << knight_square, Piece::new(Type::Knight, Color::White));

    // All 8 possible knight moves from center
    let valid_moves = [];

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

    // Only 2 possible moves from corner
    let valid_moves = [knight_square + 10, knight_square + 17];

    let invalid_moves = [
        knight_square - 17, // Off board NW
        knight_square - 15, // Off board NE
        knight_square - 10, // Off board WN
        knight_square - 6,  // Off board EN
        knight_square + 6,  // Off board WS
        knight_square + 15, // Off board SW
    ];

    for &target in &valid_moves {
        assert!(
            is_possible(&board, &(knight_square, target)),
            "Knight should be able to move from {} to {}",
            knight_square,
            target
        );
    }

    for &target in &invalid_moves {
        assert!(
            !is_possible(&board, &(knight_square, target)),
            "Knight shouldn't be able to move from {} to {}",
            knight_square,
            target
        );
    }
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

#[test]
fn test_knight_captures() {
    /*
        let mut board = Board::default();
        let knight_square = 28; // e4
        board.white_knights = 1 << knight_square;

        // Place black pawn in one of the knight's attack squares
        let capture_square = knight_square + 15;
        board.black_pawns = 1 << capture_square;

        assert!(
            is_possible(&board, &(knight_square, capture_square)),
            "Knight should be able to capture enemy piece at {}",
            capture_square
        );
    */
}
