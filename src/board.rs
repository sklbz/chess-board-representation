use crate::{
    bitboard::*,
    legal_moves::misc::{Color, Move, Piece, Square, Type},
    utils::string_to_move,
};

#[derive(Debug, Clone)]
pub struct Board {
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
        let queen_pattern: u64 = 0x8;
        let king_pattern: u64 = 0x10;

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
            black_queens: king_pattern.bitwise_reverse(),
            white_king: king_pattern,
            black_king: queen_pattern.bitwise_reverse(),
        }
    }

    pub fn empty() -> Board {
        Board {
            white_pawns: 0,
            black_pawns: 0,
            white_knights: 0,
            black_knights: 0,
            white_bishops: 0,
            black_bishops: 0,
            white_rooks: 0,
            black_rooks: 0,
            white_queens: 0,
            black_queens: 0,
            white_king: 0,
            black_king: 0,
        }
    }

    pub fn new(
        white_pawns: BitBoard,
        black_pawns: BitBoard,
        white_knights: BitBoard,
        black_knights: BitBoard,
        white_bishops: BitBoard,
        black_bishops: BitBoard,
        white_rooks: BitBoard,
        black_rooks: BitBoard,
        white_queens: BitBoard,
        black_queens: BitBoard,
        white_king: BitBoard,
        black_king: BitBoard,
    ) -> Board {
        Board {
            white_pawns,
            black_pawns,
            white_knights,
            black_knights,
            white_bishops,
            black_bishops,
            white_rooks,
            black_rooks,
            white_queens,
            black_queens,
            white_king,
            black_king,
        }
    }
    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::empty();
        let mut pieces: Vec<(Piece, Square)> = Vec::new();

        let mut idx = 0;
        fen.split('/').enumerate().for_each(|(file, row)| {
            row.chars().for_each(|char| {
                if char.is_numeric() {
                    idx += char.to_string().parse::<u8>().unwrap();
                    return;
                }

                let piece = match char {
                    'P' => Piece {
                        r#type: Type::Pawn,
                        color: Color::White,
                    },
                    'p' => Piece {
                        r#type: Type::Pawn,
                        color: Color::Black,
                    },
                    'N' => Piece {
                        r#type: Type::Knight,
                        color: Color::White,
                    },
                    'n' => Piece {
                        r#type: Type::Knight,
                        color: Color::Black,
                    },
                    'B' => Piece {
                        r#type: Type::Bishop,
                        color: Color::White,
                    },
                    'b' => Piece {
                        r#type: Type::Bishop,
                        color: Color::Black,
                    },
                    'R' => Piece {
                        r#type: Type::Rook,
                        color: Color::White,
                    },
                    'r' => Piece {
                        r#type: Type::Rook,
                        color: Color::Black,
                    },
                    'Q' => Piece {
                        r#type: Type::Queen,
                        color: Color::White,
                    },
                    'q' => Piece {
                        r#type: Type::Queen,
                        color: Color::Black,
                    },
                    'K' => Piece {
                        r#type: Type::King,
                        color: Color::White,
                    },
                    'k' => Piece {
                        r#type: Type::King,
                        color: Color::Black,
                    },
                    _ => panic!("Invalid FEN"),
                };

                pieces.push((piece, 8u8 * (7u8 - file as u8) + idx));
                idx += 1;
            });

            idx = 0;
        });

        pieces.iter().for_each(|(piece, square)| {
            board.set(square, *piece);
        });

        board
    }

    pub fn from_mask(mask: BitBoard, piece: Piece) -> Board {
        let Piece {
            r#type: piece_type,
            color,
        } = piece;

        match (piece_type, color) {
            (Type::Pawn, Color::White) => {
                let mut board = Board::empty();
                board.white_pawns = mask;
                board
            }
            (Type::Pawn, Color::Black) => {
                let mut board = Board::empty();
                board.black_pawns = mask;
                board
            }
            (Type::Knight, Color::White) => {
                let mut board = Board::empty();
                board.white_knights = mask;
                board
            }
            (Type::Knight, Color::Black) => {
                let mut board = Board::empty();
                board.black_knights = mask;
                board
            }
            (Type::Bishop, Color::White) => {
                let mut board = Board::empty();
                board.white_bishops = mask;
                board
            }
            (Type::Bishop, Color::Black) => {
                let mut board = Board::empty();
                board.black_bishops = mask;
                board
            }
            (Type::Rook, Color::White) => {
                let mut board = Board::empty();
                board.white_rooks = mask;
                board
            }
            (Type::Rook, Color::Black) => {
                let mut board = Board::empty();
                board.black_rooks = mask;
                board
            }
            (Type::Queen, Color::White) => {
                let mut board = Board::empty();
                board.white_queens = mask;
                board
            }
            (Type::Queen, Color::Black) => {
                let mut board = Board::empty();
                board.black_queens = mask;
                board
            }
            (Type::King, Color::White) => {
                let mut board = Board::empty();
                board.white_king = mask;
                board
            }
            (Type::King, Color::Black) => {
                let mut board = Board::empty();
                board.black_king = mask;
                board
            }
            _ => panic!("Piece not found!"),
        }
    }

    pub fn castle(&mut self, code: &str, side: &Color) {
        match (code, side) {
            ("O-O", Color::White) => {
                self.play_move(&string_to_move("e1g1"));
                self.play_move(&string_to_move("h1f1"));
            }
            ("O-O", Color::Black) => {
                self.play_move(&string_to_move("e8g8"));
                self.play_move(&string_to_move("h8f8"));
            }
            ("O-O-O", Color::White) => {
                self.play_move(&string_to_move("e1c1"));
                self.play_move(&string_to_move("a1d1"));
            }
            ("O-O-O", Color::Black) => {
                self.play_move(&string_to_move("e8c8"));
                self.play_move(&string_to_move("a8d8"));
            }
            _ => panic!("Invalid castle code!"),
        }
    }

    pub fn play_move(&mut self, r#move: &Move) {
        let start: Square = r#move.0;

        let piece = self.get_piece(&start);
        self.remove_piece(&start);

        let end: Square = r#move.1;

        self.remove_piece(&end);

        self.add_piece(&end, piece);
    }

    fn set(&mut self, square: &Square, piece: Piece) {
        self.remove_piece(square);
        self.add_piece(square, piece);
    }

    fn remove_piece(&mut self, square: &Square) {
        let mask = 1 << square;

        self.white_pawns &= !mask;
        self.black_pawns &= !mask;
        self.white_knights &= !mask;
        self.black_knights &= !mask;
        self.white_bishops &= !mask;
        self.black_bishops &= !mask;
        self.white_rooks &= !mask;
        self.black_rooks &= !mask;
        self.white_queens &= !mask;
        self.black_queens &= !mask;
        self.white_king &= !mask;
        self.black_king &= !mask;
    }

    fn add_piece(&mut self, square: &Square, piece: Piece) {
        let mask = 1 << square;

        match (piece.color, piece.r#type) {
            (Color::White, Type::Pawn) => self.white_pawns |= mask,
            (Color::Black, Type::Pawn) => self.black_pawns |= mask,
            (Color::White, Type::Knight) => self.white_knights |= mask,
            (Color::Black, Type::Knight) => self.black_knights |= mask,
            (Color::White, Type::Bishop) => self.white_bishops |= mask,
            (Color::Black, Type::Bishop) => self.black_bishops |= mask,
            (Color::White, Type::Rook) => self.white_rooks |= mask,
            (Color::Black, Type::Rook) => self.black_rooks |= mask,
            (Color::White, Type::Queen) => self.white_queens |= mask,
            (Color::Black, Type::Queen) => self.black_queens |= mask,
            (Color::White, Type::King) => self.white_king |= mask,
            (Color::Black, Type::King) => self.black_king |= mask,
            _ => (),
        }
    }

    pub fn get_bitboard(&self, color: &Color, piece: &Type) -> BitBoard {
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

            (Color::Null, r#type) => {
                self.get_bitboard(&Color::White, r#type) | self.get_bitboard(&Color::Black, r#type)
            }
            (color, Type::None) => {
                self.get_bitboard(color, &Type::Pawn)
                    | self.get_bitboard(color, &Type::Knight)
                    | self.get_bitboard(color, &Type::Bishop)
                    | self.get_bitboard(color, &Type::Rook)
                    | self.get_bitboard(color, &Type::Queen)
                    | self.get_bitboard(color, &Type::King)
            }
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

    pub fn display(&self) {
        let hline = "-------------------------------";
        let mut board = String::new();

        let mut row = "".to_string();

        for i in 0..64 {
            if i % 8 == 0 {
                let line = if i == 0 {
                    format!("\t {hline}\n")
                } else {
                    format!("\t│{hline}│\n")
                };
                board = format!("{line}\t│ {}\n{}", row, board);
                row = "".to_string();
            }

            let Piece { r#type, color } = self.get_piece(&i);
            match (color, r#type) {
                (Color::White, Type::Pawn) => row.push('󰡙'),
                (Color::Black, Type::Pawn) => row.push('♙'),
                (Color::White, Type::Knight) => row.push('󰡘'),
                (Color::Black, Type::Knight) => row.push(''),
                (Color::White, Type::Bishop) => row.push('󰡜'),
                (Color::Black, Type::Bishop) => row.push(''),
                (Color::White, Type::Rook) => row.push('󰡛'),
                (Color::Black, Type::Rook) => row.push(''),
                (Color::White, Type::Queen) => row.push('󰡚'),
                (Color::Black, Type::Queen) => row.push(''),
                (Color::White, Type::King) => row.push('󰡗'),
                (Color::Black, Type::King) => row.push(''),
                _ => row.push(' '),
            }

            row.push(' ');
            row.push('│');
            row.push(' ');
        }

        board = format!("\t {hline}\n\t│ {}\n{}", row, board);

        board.pop();
        board.pop();
        board.pop();
        board.pop();

        println!("{}", board);
    }
}
