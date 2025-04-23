use crate::Square;

pub(crate) type BitBoard = u64;

pub(crate) trait BitBoardGetter {
    fn get_occupied_squares(&self) -> Vec<Square>;
}

impl BitBoardGetter for BitBoard {
    fn get_occupied_squares(&self) -> Vec<Square> {
        /*
        // Recursive way
        fn reccursive_get_squares(board: BitBoard, squares: &mut Vec<Square>) -> [Square] {
            if board == 0 {
                return squares.into_boxed_slice();
            }

            let square_offset = board.trailing_zeros() as u8;

            let square = match squares.len() {
                0 => square_offset,
                _ => square_offset + squares[0],
            };

            let mut updated_squares: Vec<Square> = squares.splice(0..0, [square]).collect();

            reccursive_get_squares(board >> square_offset, &mut updated_squares)
        }

        let mut init: Vec<Square> = Vec::new();

        let squares: [Square] = reccursive_get_squares(*self, &mut init);

        */
        // Iterative way

        let mut occupied_squares: Vec<Square> = Vec::new();

        for i in 0..64 {
            if self & (1 << i) != 0 {
                occupied_squares.push(i as Square);
            }
        }

        occupied_squares
    }
}

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

pub(crate) trait Display {
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
