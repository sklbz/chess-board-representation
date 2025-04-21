pub(crate) type BitBoard = u64;

pub(crate) trait BitBoardOperations {
    fn bitwise_reverse(&self) -> BitBoard;
}

impl BitBoardOperations for BitBoard {
    fn bitwise_reverse(&self) -> BitBoard {
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
        reverse = reverse.rotate_left(32);

        reverse
    }
}

pub trait Display {
    fn display(&self);
}
impl Display for BitBoard {
    fn display(&self) {
        let board = self.bitwise_reverse();

        let mut grid: String = String::new();
        for i in 0..64 {
            if i % 8 == 0 {
                grid.push('\n');
            }

            if board >> i & 1 == 1 {
                grid.push('■')
            } else {
                grid.push('□')
            }
        }

        println!("{}", grid);
    }
}
