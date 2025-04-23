#[cfg(test)]
mod tests {
    use crate::bitmask::*;

    #[test]
    fn test_up_mask() {
        assert_eq!(up_mask(0), 0xFFFF_FFFF_FFFF_FF00);
        assert_eq!(up_mask(56), 0);
    }

    #[test]
    fn test_down_mask() {
        assert_eq!(down_mask(0), 0);
        assert_eq!(down_mask(8), 0xFF);
    }

    #[test]
    fn test_left_right_masks() {
        // Test left/right symmetry
        for square in 0..64 {
            let left = left_mask(square);
            let right = right_mask(square);
            assert_eq!(left.reverse_bits(), right);
        }
    }

    #[test]
    fn test_cross_mask() {
        // Center square should attack all squares in rank/file
        assert_eq!(cross_mask(28).count_ones(), 14); // 7 horizontal + 7 vertical
    }
}
