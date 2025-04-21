use crate::bitboard::*;
use crate::moves::*;

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
    pub fn init() -> Board {
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

    pub fn get_bitboard(&self, color: Color, piece: Type) -> BitBoard {
        match (color, piece) {
            (Color::White, Type::Pawn) => self.white_pawns,
            (Color::White, Type::Knight) => self.white_knights,
            (Color::White, Type::Bishop) => self.white_bishops,
            (Color::White, Type::Rook) => self.white_rooks,
            (Color::White, Type::Queen) => self.white_queens,
            (Color::White, Type::King) => self.white_king,
            (Color::Black, Type::Pawn) => self.black_pawns,
            (Color::Black, Type::Knight) => self.black_knights,
            (Color::Black, Type::Bishop) => self.black_bishops,
            (Color::Black, Type::Rook) => self.black_rooks,
            (Color::Black, Type::Queen) => self.black_queens,
            (Color::Black, Type::King) => self.black_king,
            _ => 0,
        }
    }

    pub fn get_piece(&self, square: &Square) -> Piece {
        Piece {
            r#type: self.get_piece_type(square),
            color: self.get_piece_color(square),
        }
    }

    fn get_piece_color(&self, square: &Square) -> Color {
        let white = self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king;

        if (white >> square) & 1 == 1 {
            return Color::White;
        }

        let black = self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king;

        if (black >> square) & 1 == 1 {
            return Color::Black;
        }

        Color::Null
    }

    fn get_piece_type(&self, square: &Square) -> Type {
        let pawns = self.white_pawns | self.black_pawns;

        if (pawns >> square) & 1 == 1 {
            return Type::Pawn;
        }

        let knights = self.white_knights | self.black_knights;

        if (knights >> square) & 1 == 1 {
            return Type::Knight;
        }

        let bishops = self.white_bishops | self.black_bishops;

        if (bishops >> square) & 1 == 1 {
            return Type::Bishop;
        }

        let rooks = self.white_rooks | self.black_rooks;

        if (rooks >> square) & 1 == 1 {
            return Type::Rook;
        }

        let queens = self.white_queens | self.black_queens;

        if (queens >> square) & 1 == 1 {
            return Type::Queen;
        }

        let kings = self.white_king | self.black_king;

        if (kings >> square) & 1 == 1 {
            return Type::King;
        }

        Type::None
    }
}
