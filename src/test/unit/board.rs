#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::moves::*;

    #[test]
    fn test_board_init() {
        let board = Board::init();

        // Test initial piece counts
        assert_eq!(
            board.get_bitboard(&Color::White, &Type::Pawn).count_ones(),
            8
        );
        assert_eq!(
            board
                .get_bitboard(&Color::White, &Type::Knight)
                .count_ones(),
            2
        );
        assert_eq!(
            board.get_bitboard(&Color::White, &Type::King).count_ones(),
            1
        );
        // ... similar for other pieces
    }

    #[test]
    fn test_move_execution() {
        let mut board = Board::init();
        let e2 = 8 + 4; // e2 square (12)
        let e4 = 24 + 4; // e4 square (28)

        // Verify initial state
        assert!(board.get_piece(&e2).r#type == Type::Pawn);
        assert!(board.get_piece(&e4).r#type == Type::None);

        // Execute move
        board.play_move(&(e2, e4));

        // Verify new state
        assert!(board.get_piece(&e2).r#type == Type::None);
        assert!(board.get_piece(&e4).r#type == Type::Pawn);
    }

    #[test]
    fn test_capture_logic() {
        let mut board = Board::init();
        let e2 = 12; // e2
        let e7 = 52; // e7

        // Move white pawn to e4
        board.play_move(&(e2, e2 + 16));

        // Move black pawn to e5
        board.play_move(&(e7, e7 - 16));

        // White captures black
        board.play_move(&(e2 + 16, e7 - 16));

        assert!(board.get_piece(&(e7 - 16)).color == Color::White);
    }
}
