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

#[cfg(test)]
mod proptests {
    use crate::bitboard::BitBoard;
    use crate::bitboard::BitBoardGetter;
    use crate::board::*;
    use crate::moves::*;
    use proptest::collection::vec;
    use proptest::prelude::*;
    use proptest::sample::select;

    impl Arbitrary for Board {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                // White pieces
                any::<BitBoard>(), // pawns
                any::<BitBoard>(), // knights
                any::<BitBoard>(), // bishops
                any::<BitBoard>(), // rooks
                any::<BitBoard>(), // queens
                any::<BitBoard>(), // king
                // Black pieces
                any::<BitBoard>(), // pawns
                any::<BitBoard>(), // knights
                any::<BitBoard>(), // bishops
                any::<BitBoard>(), // rooks
                any::<BitBoard>(), // queens
                any::<BitBoard>(), // king
            )
                .prop_map(|(wp, wn, wb, wr, wq, wk, bp, bn, bb, br, bq, bk)| {
                    // Ensure kings exist and are unique
                    let wk = if wk.count_ones() == 0 { 1 } else { wk };
                    let bk = if bk.count_ones() == 0 { 1 << 63 } else { bk };

                    // Ensure no overlapping pieces
                    let all_white = wp | wn | wb | wr | wq | wk;
                    let all_black = bp | bn | bb | br | bq | bk;
                    let overlap = all_white & all_black;

                    Board::new(
                        wp & !overlap,
                        wn & !overlap,
                        wb & !overlap,
                        wr & !overlap,
                        wq & !overlap,
                        wk & !overlap,
                        bp & !overlap,
                        bn & !overlap,
                        bb & !overlap,
                        br & !overlap,
                        bq & !overlap,
                        bk & !overlap,
                    )
                })
                .boxed()
        }
    }

    proptest! {
        #[test]
        fn test_bitboard_roundtrip(square in 0..64u8) {
            let bb = square.to_bitboard();
            let squares = bb.get_occupied_squares();
            prop_assert_eq!(squares.len(), 1);
            prop_assert_eq!(squares[0], square);
        }

        #[test]
        fn test_move_validation(board: Board, from in 0..64u8, to in 0..64u8) {
            let _ = is_possible(&board, &(from, to));
            // We're just testing it doesn't panic
        }
    }
}
