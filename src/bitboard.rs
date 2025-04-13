pub(crate) type BitBoard = u64;

pub(crate) trait BitBoardOperations {
    fn bitwise_reverse(&self) -> BitBoard;
}

impl BitBoardOperations for BitBoard {
    pub(crate) fn bitwise_reverse(&self) -> BitBoard {
        let mut reverse =
            ((self >> 1) & 0x5555_5555_5555_5555) | ((self & 0x5555_5555_5555_5555) << 1);
        reverse =
            ((reverse >> 2) & 0x3333_3333_3333_3333) | ((reverse & 0x3333_3333_3333_3333) << 2);
        reverse =
            ((reverse >> 4) & 0x0f0f_0f0f_0f0f_0f0f) | ((reverse & 0x0f0f_0f0f_0f0f_0f0f) << 4);
        reverse =
            ((reverse >> 8) & 0x00ff_00ff_00ff_00ff) | ((reverse & 0x00ff_00ff_00ff_00ff) << 8);
        reverse =
            ((reverse >> 16) & 0x0000_ffff_0000_ffff) | ((reverse & 0x0000_ffff_0000_ffff) << 16);
        reverse = (reverse >> 32) | (reverse << 32);
        reverse
    }
}
