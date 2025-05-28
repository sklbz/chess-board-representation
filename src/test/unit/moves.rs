use crate::board::board::Board;
use crate::board::mask_handling::MaskHandler;
use crate::legal_moves::is_move_possible::*;

#[test]
fn test_pawn_moves() {
    let board = Board::init();

    // Test initial white pawn moves
    let e2 = 12;
    assert!(is_possible(&board, &(e2, e2 + 8), Color::White)); // Single push
    assert!(is_possible(&board, &(e2, e2 + 16), Color::White)); // Double push

    // Test invalid pawn moves
    assert!(!is_possible(&board, &(e2, e2 + 7), Color::White)); // No capture
    assert!(!is_possible(&board, &(e2, e2 + 9), Color::White)); // No capture
}

#[test]
fn test_pawn_capture_wrap_around_border() {
    let mut board = Board::init();

    let a7 = 8 * 7;
    let a5 = 8 * 5;
    let a4 = 8 * 4;
    let a3 = 8 * 3;
    let h2 = 15;
    let _ = board.play_move(&(a7, a5));
    let _ = board.play_move(&(a5, a4));
    let _ = board.play_move(&(a4, a3));
    assert!(!is_possible(&board, &(h2, a3), Color::White)); // Take around border
}

#[test]
fn test_rook_moves() {
    let board = Board::from_mask(1 << 0, Piece::new(Type::Rook, Color::White)); // White rook at a1

    // Valid rook moves
    assert!(is_possible(&board, &(0, 7), Color::White)); // Horizontal
    assert!(is_possible(&board, &(0, 56), Color::White)); // Vertical

    // Invalid rook moves
    assert!(!is_possible(&board, &(0, 9), Color::White)); // Diagonal
}

#[test]
fn test_bishop_moves() {
    let a1 = 0;
    let b2 = 9;
    let c3 = 18;
    let d4 = 27;
    let e5 = 36;
    let f6 = 45;
    let g7 = 54;
    let h8 = 63;
    let a8 = 56;
    let b7 = 49;
    let c6 = 42;
    let d5 = 35;
    let e4 = 28;
    let f3 = 21;
    let g2 = 14;
    let h1 = 7;

    use crate::legal_moves::misc::*;
    let board = Board::from_mask(e5.to_bitboard(), Piece::new(Type::Bishop, Color::White)); // White bishop at e5
                                                                                            // Light squared bishop

    assert!(is_possible(&board, &(e5, a1), Color::White));
    assert!(is_possible(&board, &(e5, b2), Color::White));
    assert!(is_possible(&board, &(e5, c3), Color::White));
    assert!(is_possible(&board, &(e5, d4), Color::White));
    assert!(is_possible(&board, &(e5, f6), Color::White));
    assert!(is_possible(&board, &(e5, g7), Color::White));
    assert!(is_possible(&board, &(e5, h8), Color::White));

    let board_alt = Board::from_mask(d5.to_bitboard(), Piece::new(Type::Bishop, Color::White)); // White bishop at d5
                                                                                                // Dark squared bishop

    assert!(is_possible(&board_alt, &(d5, a8), Color::White));
    assert!(is_possible(&board_alt, &(d5, b7), Color::White));
    assert!(is_possible(&board_alt, &(d5, c6), Color::White));
    assert!(is_possible(&board_alt, &(d5, e4), Color::White));
    assert!(is_possible(&board_alt, &(d5, f3), Color::White));
    assert!(is_possible(&board_alt, &(d5, g2), Color::White));
    assert!(is_possible(&board_alt, &(d5, h1), Color::White));

    // testign invalid moves

    assert!(!is_possible(&board_alt, &(d5, a1), Color::White));
    assert!(!is_possible(&board_alt, &(d5, b2), Color::White));
}

#[test]
pub fn test_blockers_rook() {
    let board = Board::init();
    assert!(!is_possible(&board, &(0, 1), Color::White));
}

#[test]
pub fn test_blockers_bishop() {
    let board = Board::init();
    assert!(!is_possible(&board, &(3, 10), Color::White));
}

#[test]
fn test_color_negation() {
    assert_eq!(!Color::White, Color::Black);
    assert_eq!(!Color::Black, Color::White);
    assert_eq!(!Color::Null, Color::Null);
}

use crate::legal_moves::misc::{Color, Piece, Type};
use crate::r#move::bishop::bishop_move_bitmask;
use crate::r#move::king::king_move_mask;
use crate::r#move::pawn::{pawn_move_black, pawn_move_white};
use crate::r#move::queen::queen_move_bitmask;
use crate::r#move::rook::rook_move_bitmask;
use proptest::prelude::*;
proptest! {

    #[test]
    fn knight_start_not_included_in_mask(square in 0..64u8) {
        let knight = king_move_mask(&square, &0, &0);
        assert_eq!(knight & (1 << square), 0, "Knight starting square included in mask");
    }


    #[test]
    fn pawn_start_not_included_in_mask(square in 0..64u8) {
        let pawn = pawn_move_white(&square, &0, &0) | pawn_move_black(&square, &0, &0);
        assert_eq!(pawn & (1 << square), 0, "Pawn starting square included in mask");
    }


    #[test]
    fn king_start_not_included_in_mask(square in 0..64u8) {
        let king = king_move_mask(&square, &0, &0);
        assert_eq!(king & (1 << square), 0, "King starting square included in mask");
    }


    #[test]
    fn queen_start_not_included_in_mask(square in 0..64u8) {
        let queen = queen_move_bitmask(&square, &0, &0);
        assert_eq!(queen & (1 << square), 0, "Queen starting square included in mask");
    }


    #[test]
    fn rook_start_not_included_in_mask(square in 0..64u8) {
        let rook = rook_move_bitmask(&square, &0, &0);
        assert_eq!(rook & (1 << square), 0, "Rook starting square included in mask");
    }


    #[test]
    fn bishop_start_not_included_in_mask(square in 0..64u8) {
        let bishop = bishop_move_bitmask(&square, &0, &0);
        assert_eq!(bishop & (1 << square), 0, "Bishop starting square included in mask");
    }


    #[test]
    fn test_starting_pawn_move(pawn in 8..16u8) {
        let board = Board::init();

        // Single push
        assert!(is_possible(&board, &(pawn, pawn + 8), Color::White));
        // Double push
        assert!(is_possible(&board, &(pawn, pawn + 16), Color::White));
    }

    #[test]
    fn test_queen_moves(square in 0..64u8) {
        let queen = queen_move_bitmask(&square, &0, &0);
        let rook = rook_move_bitmask(&square, &0, &0);
        let bishop = bishop_move_bitmask(&square, &0, &0);

        assert!(queen.count_ones() > 20);
        assert_eq!(bishop & rook, 0);
        assert_eq!(queen, (bishop | rook) & !(1 << square));
        assert_eq!(queen.count_ones(), bishop.count_ones() + rook.count_ones());
    }
}
