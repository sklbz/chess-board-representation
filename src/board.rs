use crate::bitboard::BitBoard;

pub(crate) struct Board {
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

impl Board {
    fn init() -> Board {
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
