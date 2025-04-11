fn main() {
    let board = Board::start();
    is_possible(board, &(0, 0));
    println!("Hello, world!");
}

type BitBoard = u64;

trait BitBoardOperations {
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
        reverse = (reverse >> 32) | (reverse << 32);
        reverse
    }
}

struct Board {
    // Pawns
    white_pawns: BitBoard,
    black_pawns: BitBoard,
    // Knights
    white_knights: BitBoard,
    black_knights: BitBoard,
    // Bishops
    white_bishops: BitBoard,
    black_bishops: BitBoard,
    // Rooks
    white_rooks: BitBoard,
    black_rooks: BitBoard,
    // Queens
    white_queens: BitBoard,
    black_queens: BitBoard,
    // Kings
    white_king: BitBoard,
    black_king: BitBoard,
}

trait BoardTrait {
    fn setup() -> Board;
}

impl BoardTrait for Board {
    fn setup() -> Board {
        let pawn_pattern: u64 = 0xff00;
        let knight_pattern: u64 = 0x42;
        let bishop_pattern: u64 = 0x24;
        let rook_pattern: u64 = 0x81;
        let queen_pattern: u64 = 0x10;
        let king_pattern: u64 = 0x8;

        Board {
            white_pawns: pawn_pattern,
            black_pawns: pawn_pattern.bitwise_reverse(),
            white_knights: knight_pattern,
            black_knights: knight_pattern.bitwise_reverse(),
            white_bishops: bishop_pattern,
            black_bishops: bishop_pattern.bitwise_reverse(),
            white_rooks: rook_pattern,
            black_rooks: rook_pattern.bitwise_reverse(),
            white_queens: queen_pattern,
            black_queens: queen_pattern.bitwise_reverse(),
            white_king: king_pattern,
            black_king: king_pattern.bitwise_reverse(),
        }
    }
}

type Move = (u64, u64);

fn is_possible(board: &Board, r#move: &Move) -> bool {
    true
}
