use crate::bitmask::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_vertical_mask_square_amount(square in 0..64u8) {
        let vertical = !right_mask(square) & !left_mask(square);
        assert_eq!(vertical.count_ones(), 8);
    }

    #[test]
    fn test_cross_mask_square_amount(square in 0..64u8) {
        assert_eq!(cross_mask(square).count_ones(), 15);
    }
}
