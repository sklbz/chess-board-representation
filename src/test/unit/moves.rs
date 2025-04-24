#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::moves::*;

    #[test]
    fn test_pawn_moves() {
        let mut board = Board::init();

        // Test initial white pawn moves
        let e2 = 12;
        assert!(is_possible(&board, &(e2, e2 + 8))); // Single push
        assert!(is_possible(&board, &(e2, e2 + 16))); // Double push

        // Test invalid pawn moves
        let h2 = 15;
        assert!(!is_possible(&board, &(e2, e2 + 7))); // No capture
        assert!(!is_possible(&board, &(e2, e2 + 9))); // No capture

        let a7 = 8 * 7;
        let a5 = 8 * 5;
        let a4 = 8 * 4;
        let a3 = 8 * 3;
        board.play_move(&(a7, a5));
        board.play_move(&(a5, a4));
        board.play_move(&(a4, a3));
        assert!(!is_possible(&board, &(h2, a3))); // Take around border
    }

    #[test]
    fn test_rook_moves() {
        let board = Board::from_mask(1 << 0, Piece::new(Type::Rook, Color::White)); // White rook at a1

        // Valid rook moves
        assert!(is_possible(&board, &(0, 7))); // Horizontal
        assert!(is_possible(&board, &(0, 56))); // Vertical

        // Invalid rook moves
        assert!(!is_possible(&board, &(0, 9))); // Diagonal
    }

    #[test]
    fn test_color_negation() {
        assert_eq!(!Color::White, Color::Black);
        assert_eq!(!Color::Black, Color::White);
        assert_eq!(!Color::Null, Color::Null);
    }
}
