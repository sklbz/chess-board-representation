#[cfg(test)]
mod tests {
    use crate::bitmask::*;

    #[test]
    fn test_up_mask() {
        assert_eq!(up_mask(0), 0xFFFF_FFFF_FFFF_FF00);
        assert_eq!(up_mask(63), 0);
    }

    #[test]
    fn test_down_mask() {
        assert_eq!(down_mask(0), 0);
        assert_eq!(down_mask(8), 0xFF);
    }

    #[test]
    fn test_cross_mask() {
        // Center square should attack all squares in rank/file
        assert_eq!(cross_mask(28).count_ones(), 14 + 1); // 7 horizontal + 7 vertical
    }

    #[test]
    fn test_right_diagonal_mask() {
        // Test center squares
        assert_eq!(right_diagonal_mask(28), 0x8040_2010_0804_0201); // e4
        assert_eq!(right_diagonal_mask(35), 0x0080_4020_1008_0402); // d5

        // Test edge cases
        assert_eq!(right_diagonal_mask(0), 0x8040_2010_0804_0201); // a1
        assert_eq!(right_diagonal_mask(7), 0x0000_0000_0000_0080); // h1
        assert_eq!(right_diagonal_mask(56), 0x100_0000_0000_0000); // a8
        assert_eq!(right_diagonal_mask(7), 0x80); // h8 (only one square)

        // Test specific patterns
        let e4_mask = right_diagonal_mask(28);
        assert!(e4_mask & (1 << 0) != 0); // a1 should be in the mask
        assert!(e4_mask & (1 << 9) != 0); // b2
        assert!(e4_mask & (1 << 18) != 0); // c3
        assert!(e4_mask & (1 << 27) != 0); // d4
        assert!(e4_mask & (1 << 36) != 0); // f5
        assert!(e4_mask & (1 << 45) != 0); // g6
        assert!(e4_mask & (1 << 54) != 0); // h7
    }

    #[test]
    fn test_left_diagonal_mask() {
        // Test center squares
        assert_eq!(left_diagonal_mask(28), 0x0102_0408_1020_4080); // e4
        assert_eq!(left_diagonal_mask(35), 0x0204_0810_2040_8000); // d5

        // Test edge cases
        assert_eq!(left_diagonal_mask(7), 0x0102_0408_1020_4080); // h1
        assert_eq!(left_diagonal_mask(0), 0x0000_0000_0000_0001); // a1 (only one square)
        assert_eq!(left_diagonal_mask(56), 0x0000_0000_0000_0080); // a8
        assert_eq!(left_diagonal_mask(63), 0x0102_0408_1020_4080); // h8

        // Test specific patterns
        let d5_mask = left_diagonal_mask(35);
        assert!(d5_mask & (1 << 7) != 0); // h1 should be in the mask
        assert!(d5_mask & (1 << 14) != 0); // g2
        assert!(d5_mask & (1 << 21) != 0); // f3
        assert!(d5_mask & (1 << 28) != 0); // e4
        assert!(d5_mask & (1 << 42) != 0); // c6
        assert!(d5_mask & (1 << 49) != 0); // b7
        assert!(d5_mask & (1 << 56) != 0); // a8
    }

    #[test]
    fn test_diagonal_cross_mask() {
        // Test center square
        let e4_mask = diagonal_cross_mask(28);
        assert_eq!(e4_mask.count_ones(), 8 + 7); // 8 in one diagonal + 7 in the other (center counted once)

        // Test edge square
        let a1_mask = diagonal_cross_mask(0);
        assert_eq!(a1_mask.count_ones(), 8); // Only one diagonal for corner

        // Test composition with individual masks
        for square in 0..64 {
            let cross = diagonal_cross_mask(square);
            let left = left_diagonal_mask(square);
            let right = right_diagonal_mask(square);
            assert_eq!(cross, left | right);
        }
    }

    #[test]
    fn test_diagonal_masks_no_overlap() {
        // For squares not on the same diagonals, masks shouldn't overlap
        assert_eq!(right_diagonal_mask(0) & right_diagonal_mask(1), 0);
        assert_eq!(left_diagonal_mask(0) & left_diagonal_mask(8), 0);

        // Except for squares on the same diagonal
        assert_ne!(right_diagonal_mask(0) & right_diagonal_mask(9), 0);
        assert_ne!(left_diagonal_mask(7) & left_diagonal_mask(14), 0);
    }

    #[test]
    fn test_diagonal_mask_symmetry() {
        // Test symmetry properties
        for square in 0..64 {
            let right_mask = right_diagonal_mask(square);
            let left_mask = left_diagonal_mask(63 - square);

            // The right diagonal of square should match the left diagonal of its mirrored position
            assert_eq!(right_mask, left_mask.reverse_bits());
        }
    }

    #[test]
    fn test_diagonal_mask_edge_cases() {
        // Test all edge squares
        let edges = [0, 7, 56, 63]; // a1, h1, a8, h8
        for &square in &edges {
            let right = right_diagonal_mask(square);
            let left = left_diagonal_mask(square);

            // Edge squares should only have one diagonal
            if square == 0 || square == 63 {
                assert_eq!(right.count_ones(), 8);
                assert_eq!(left.count_ones(), 1);
            } else {
                assert_eq!(right.count_ones(), 1);
                assert_eq!(left.count_ones(), 8);
            }
        }
    }

    #[test]
    fn test_bishop_movement() {
        // Place white bishop at c3 (18)
        let bishop_square = 18;

        // Get all possible moves
        let moves = diagonal_cross_mask(bishop_square);

        // Should include these squares
        assert!(moves & (1 << 0) != 0); // a1
        assert!(moves & (1 << 9) != 0); // b2
        assert!(moves & (1 << 27) != 0); // d4
        assert!(moves & (1 << 36) != 0); // e5
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_diagonal_masks_contain_origin(square in 0..64u8) {
            let right = right_diagonal_mask(square);
            let left = left_diagonal_mask(square);
            assert!(right & (1 << square) != 0);
            assert!(left & (1 << square) != 0);
        }

        #[test]
        fn test_diagonal_masks_symmetric(square in 0..64u8) {
            let right = right_diagonal_mask(square);
            let left = left_diagonal_mask(square);
            // For non-edge squares, both diagonals should exist
            if square % 8 != square / 8 && square % 8 + square / 8 != 7 {
                assert!(right.count_ones() > 1);
                assert!(left.count_ones() > 1);
            }
        }

        #[test]
        fn test_diagonal_cross_contains_both(square in 0..64u8) {
            let cross = diagonal_cross_mask(square);
            let right = right_diagonal_mask(square);
            let left = left_diagonal_mask(square);
            assert_eq!(cross, right | left);
        }
    }
}
