#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::legal_moves::misc::{Color, Type};

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
    use crate::{bitboard::BitBoard, bitboard::BitBoardGetter, board::*};
    use proptest::arbitrary::Arbitrary;
    use proptest::prelude::*;
    use proptest::strategy::BoxedStrategy;

    impl Arbitrary for Board {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            // Generate random squares for kings first (ensuring they're different)
            (0..64u8, 0..64u8)
                .prop_filter("Kings must be on different squares", |(wk, bk)| wk != bk)
                .prop_flat_map(|(white_king_sq, black_king_sq)| {
                    // Generate other pieces (0-15 of each type)
                    (
                        prop::collection::vec(0..64u8, 0..16), // white pawns
                        prop::collection::vec(0..64u8, 0..16), // white knights
                        prop::collection::vec(0..64u8, 0..16), // white bishops
                        prop::collection::vec(0..64u8, 0..16), // white rooks
                        prop::collection::vec(0..64u8, 0..16), // white queens
                        prop::collection::vec(0..64u8, 0..16), // black pawns
                        prop::collection::vec(0..64u8, 0..16), // black knights
                        prop::collection::vec(0..64u8, 0..16), // black bishops
                        prop::collection::vec(0..64u8, 0..16), // black rooks
                        prop::collection::vec(0..64u8, 0..16), // black queens
                    )
                        .prop_map(
                            move |(wp, wn, wb, wr, wq, bp, bn, bb, br, bq)| {
                                // Create bitboards from square lists
                                let to_bitboard = |squares: Vec<u8>| {
                                    squares.iter().fold(0u64, |acc, &sq| acc | (1 << sq))
                                };

                                // Create king bitboards
                                let wk = 1u64 << white_king_sq;
                                let bk = 1u64 << black_king_sq;

                                // Convert to bitboard
                                let mut wp_bb = to_bitboard(wp);
                                let mut wn_bb = to_bitboard(wn);
                                let mut wb_bb = to_bitboard(wb);
                                let mut wr_bb = to_bitboard(wr);
                                let mut wq_bb = to_bitboard(wq);
                                let mut bp_bb = to_bitboard(bp);
                                let mut bn_bb = to_bitboard(bn);
                                let mut bb_bb = to_bitboard(bb);
                                let mut br_bb = to_bitboard(br);
                                let mut bq_bb = to_bitboard(bq);

                                // Remove any pieces that overlap with kings
                                wp_bb &= !wk;
                                wn_bb &= !wk;
                                wb_bb &= !wk;
                                wr_bb &= !wk;
                                wq_bb &= !wk;

                                bp_bb &= !bk;
                                bn_bb &= !bk;
                                bb_bb &= !bk;
                                br_bb &= !bk;
                                bq_bb &= !bk;

                                // Remove any overlapping pieces between colors
                                let all_white = wp_bb | wn_bb | wb_bb | wr_bb | wq_bb | wk;
                                let all_black = bp_bb | bn_bb | bb_bb | br_bb | bq_bb | bk;

                                let overlap = all_white & all_black;

                                if overlap != 0 {
                                    wp_bb &= !overlap;
                                    wn_bb &= !overlap;
                                    wb_bb &= !overlap;
                                    wr_bb &= !overlap;
                                    wq_bb &= !overlap;

                                    bp_bb &= !overlap;
                                    bn_bb &= !overlap;
                                    bb_bb &= !overlap;
                                    br_bb &= !overlap;
                                    bq_bb &= !overlap;
                                }

                                // Combine all pieces
                                Board::new(
                                    wp_bb, wn_bb, wb_bb, wr_bb, wq_bb, wk, bp_bb, bn_bb, bb_bb,
                                    br_bb, bq_bb, bk,
                                )
                            },
                        )
                })
                .boxed()
        }
    }

    use crate::legal_moves::is_move_possible::is_possible;
    use crate::legal_moves::misc::*;
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

        #[test]
        fn always_one_king_per_color(board: Board) {
            assert_eq!(board.get_bitboard(&Color::White, &Type::King).count_ones(), 1, "White must have exactly one king");
            assert_eq!(board.get_bitboard(&Color::Black, &Type::King).count_ones(), 1, "Black must have exactly one king");
            assert_ne!(
                board.get_bitboard(&Color::White, &Type::King).trailing_zeros() as u8,
                board.get_bitboard(&Color::Black, &Type::King).trailing_zeros() as u8,
                "Kings cannot be on the same square"
            );
        }

    }
}
