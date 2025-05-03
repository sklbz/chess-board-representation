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
            (
                // White pieces
                any::<BitBoard>(), // pawns
                any::<BitBoard>(), // knights
                any::<BitBoard>(), // bishops
                any::<BitBoard>(), // rooks
                any::<BitBoard>(), // queens
                0..63u8,           // king
                // Black pieces
                any::<BitBoard>(), // pawns
                any::<BitBoard>(), // knights
                any::<BitBoard>(), // bishops
                any::<BitBoard>(), // rooks
                any::<BitBoard>(), // queens
                0..63u8,           // king
            )
                .prop_map(
                    |(wp, wn, wb, wr, wq, w_king_square, bp, bn, bb, br, bq, b_king_square)| {
                        let arb_wk: BitBoard = 1 << w_king_square;
                        let arb_bk: BitBoard = 1 << b_king_square;

                        // Ensure kings exist and are unique
                        let wk = if arb_wk.count_ones() == 1 {
                            arb_wk
                        } else {
                            1 << 0
                        };
                        let bk = if arb_bk.count_ones() == 1 {
                            arb_bk
                        } else {
                            1 << 63
                        };

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
                            wk,
                            bp & !overlap,
                            bn & !overlap,
                            bb & !overlap,
                            br & !overlap,
                            bq & !overlap,
                            bk,
                        )
                    },
                )
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
            let wk = board.get_bitboard(&Color::White, &Type::King);
            let bk = board.get_bitboard(&Color::Black, &Type::King);

            if wk.count_ones() != 1 || bk.count_ones() != 1 {
                board.display();
            }

            assert_eq!(wk.count_ones(), 1, "White must have exactly one king");
            assert_eq!(bk.count_ones(), 1, "Black must have exactly one king");
            assert_ne!(
                wk.trailing_zeros() as u8,
                bk.trailing_zeros() as u8,
                "Kings cannot be on the same square"
            );
        }

    }
}
