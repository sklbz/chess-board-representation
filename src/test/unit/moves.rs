#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::moves::*;

    #[test]
    fn test_pawn_moves() {
        let board = Board::init();

        // Test initial white pawn moves
        let e2 = 12;
        assert!(is_possible(&board, &(e2, e2 + 8))); // Single push
        assert!(is_possible(&board, &(e2, e2 + 16))); // Double push

        // Test invalid pawn moves
        assert!(!is_possible(&board, &(e2, e2 + 7))); // No capture
        assert!(!is_possible(&board, &(e2, e2 + 9))); // No capture
    }

    #[test]
    fn test_rook_moves() {
        let board = Board::from_mask(1 << 0, Piece(Type::Rook, Color::White)); // White rook at a1

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
