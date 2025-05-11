use crate::{
    bitboard::*,
    legal_moves::{
        attack_mask::{generate_attack_mask, generate_attack_mask_single_square},
        generate_possible_moves::generate_move_vec,
        misc::{Color, Coord, Move, Piece, Square, Type},
    },
    utils::{piece_from_char, piece_to_char, piece_to_icon, string_to_move},
};

#[derive(Debug, Clone)]
pub struct Board {
    // Pawns
    white_pawns: BitBoard,
    black_pawns: BitBoard,
    en_passant: BitBoard,
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
    // White: Kingside/Queenside, Black: Kingside/Queenside
    castling_rights: [bool; 4],
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
            en_passant: 0,
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
            castling_rights: [true; 4],
        }
    }

    pub fn empty() -> Board {
        Board {
            white_pawns: 0,
            black_pawns: 0,
            en_passant: 0,
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
            castling_rights: [true; 4],
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
            en_passant: 0,
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
            castling_rights: [false; 4],
        }
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut board = Board::empty();
        let mut pieces: Vec<(Piece, Square)> = Vec::new();

        let piece_str = fen.split_whitespace().next().unwrap();
        let castling = fen
            .split_whitespace()
            .last()
            .unwrap()
            .chars()
            .collect::<Vec<char>>();

        let castling_rights: [bool; 4] = [
            castling.contains(&'K'),
            castling.contains(&'Q'),
            castling.contains(&'k'),
            castling.contains(&'q'),
        ];

        board.castling_rights = castling_rights;

        let mut idx = 0;
        piece_str.split('/').enumerate().for_each(|(file, row)| {
            row.chars().for_each(|char| {
                if char.is_numeric() {
                    idx += char.to_string().parse::<u8>().unwrap();
                    return;
                }

                let piece = piece_from_char(char);

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

    pub fn to_fen(&self) -> String {
        let mut fen: String = String::new();
        let mut empty_spaces: u8 = 0;

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let piece = self.get_piece(&square);

                if piece.is_none() {
                    empty_spaces += 1;
                } else {
                    if empty_spaces > 0 {
                        fen.push_str(&empty_spaces.to_string());
                        empty_spaces = 0;
                    }

                    fen.push(piece_to_char(&piece));
                }
            }

            if empty_spaces > 0 {
                fen.push_str(&empty_spaces.to_string());
                empty_spaces = 0;
            }

            if rank > 0 {
                fen.push('/');
            }
        }

        let castling: String = self
            .castling_rights
            .iter()
            .enumerate()
            .filter_map(|(idx, right)| {
                if *right {
                    match idx {
                        0 => Some('K'),
                        1 => Some('Q'),
                        2 => Some('k'),
                        3 => Some('q'),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect();

        format!("{} {} 0 1", fen, castling)
    }

    pub fn from_mask(mask: BitBoard, piece: Piece) -> Board {
        let Piece {
            r#type: piece_type,
            color,
        } = piece;

        let mut board = Board::empty();

        match (piece_type, color) {
            (Type::Pawn, Color::White) => {
                board.white_pawns = mask;
                board
            }
            (Type::Pawn, Color::Black) => {
                board.black_pawns = mask;
                board
            }
            (Type::Knight, Color::White) => {
                board.white_knights = mask;
                board
            }
            (Type::Knight, Color::Black) => {
                board.black_knights = mask;
                board
            }
            (Type::Bishop, Color::White) => {
                board.white_bishops = mask;
                board
            }
            (Type::Bishop, Color::Black) => {
                board.black_bishops = mask;
                board
            }
            (Type::Rook, Color::White) => {
                board.white_rooks = mask;
                board
            }
            (Type::Rook, Color::Black) => {
                board.black_rooks = mask;
                board
            }
            (Type::Queen, Color::White) => {
                board.white_queens = mask;
                board
            }
            (Type::Queen, Color::Black) => {
                board.black_queens = mask;
                board
            }
            (Type::King, Color::White) => {
                board.white_king = mask;
                board
            }
            (Type::King, Color::Black) => {
                board.black_king = mask;
                board
            }
            _ => panic!("Piece not found!"),
        }
    }

    pub fn can_castle_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => self.castling_rights[0],
            Color::Black => self.castling_rights[2],
            Color::Null => panic!("Color is null"),
        }
    }

    pub fn can_castle_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => self.castling_rights[1],
            Color::Black => self.castling_rights[3],
            Color::Null => panic!("Color is null"),
        }
    }

    pub fn is_check(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_king & self.black_attack_mask() != 0,
            Color::Black => self.black_king & self.white_attack_mask() != 0,
            Color::Null => panic!("Color is null"),
        }
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        match color {
            Color::White => {
                self.is_check(Color::White) && self.get_legal_moves(&Color::White).is_empty()
            }
            Color::Black => {
                self.is_check(Color::Black) && self.get_legal_moves(&Color::Black).is_empty()
            }
            Color::Null => panic!("Color is null"),
        }
    }

    pub fn get_legal_moves(&self, color: &Color) -> Vec<Move> {
        generate_move_vec(self, *color)
    }

    pub fn black_attack_mask(&self) -> BitBoard {
        generate_attack_mask(self, &Color::Black, &0, &0)
    }

    pub fn white_attack_mask(&self) -> BitBoard {
        generate_attack_mask(self, &Color::White, &0, &0)
    }

    pub fn castle(&mut self, code: &str, side: &Color) {
        match (code, side) {
            ("O-O", Color::White) => {
                self.make_move(&string_to_move("e1g1"));
                self.make_move(&string_to_move("h1f1"));
            }
            ("O-O", Color::Black) => {
                self.make_move(&string_to_move("e8g8"));
                self.make_move(&string_to_move("h8f8"));
            }
            ("O-O-O", Color::White) => {
                self.make_move(&string_to_move("e1c1"));
                self.make_move(&string_to_move("a1d1"));
            }
            ("O-O-O", Color::Black) => {
                self.make_move(&string_to_move("e8c8"));
                self.make_move(&string_to_move("a8d8"));
            }
            _ => panic!("Invalid castle code!"),
        }
    }

    fn update_castling_rights(&mut self, start: &Square, piece_type: Type, color: Color) {
        match piece_type {
            Type::King => match color {
                Color::White => {
                    self.castling_rights[0] = false;
                    self.castling_rights[1] = false;
                }
                Color::Black => {
                    self.castling_rights[2] = false;
                    self.castling_rights[3] = false;
                }
                Color::Null => panic!("Color is null"),
            },
            Type::Rook => match start.file() {
                0 => match color {
                    Color::White => self.castling_rights[1] = false,
                    Color::Black => self.castling_rights[3] = false,
                    Color::Null => panic!("Color is null"),
                },
                7 => match color {
                    Color::White => self.castling_rights[0] = false,
                    Color::Black => self.castling_rights[2] = false,
                    Color::Null => panic!("Color is null"),
                },
                _ => (),
            },
            _ => (),
        };
    }

    pub fn make_move_str(&mut self, move_: &str) {
        self.play_move(&string_to_move(move_));
    }

    pub fn play_move(&mut self, move_: &Move) {
        let (start, end): &(Square, Square) = move_;

        let Piece {
            r#type: piece_type,
            color,
        } = self.get_piece(start);

        let offset = *end as i8 - *start as i8;

        self.update_castling_rights(start, piece_type, color);

        if piece_type == Type::King && offset.abs() == 2 {
            let castle_code = if start > end { "O-O-O" } else { "O-O" };
            self.castle(castle_code, &color);

            return;
        }

        self.handle_en_passant(piece_type, start, end, offset);

        self.make_move(move_);
    }

    fn make_move(&mut self, move_: &Move) {
        let (start, end): &(Square, Square) = move_;

        let piece = self.get_piece(start);

        self.remove_piece(start);

        self.remove_piece(end);

        self.add_piece(end, piece);
    }

    fn handle_en_passant(&mut self, piece_type: Type, start: &Square, end: &Square, offset: i8) {
        if piece_type != Type::Pawn {
            self.en_passant = 0;
            return;
        }

        if self.en_passant & (1 << end) != 0 {
            let sign = if offset > 0 { 1 } else { -1 };

            let en_passant_square = *start as i8 + (offset.abs() - 8) * sign;
            self.remove_piece(&(en_passant_square as Square));

            self.en_passant = 0;
            return;
        }

        if offset.abs() == 16 {
            self.en_passant = 1 << (*start as i8 + offset / 2);
            return;
        }

        self.en_passant = 0;
    }

    // WARNING: make this function private after testing
    pub fn set(&mut self, square: &Square, piece: Piece) -> Board {
        self.remove_piece(square);
        self.add_piece(square, piece);

        // WARNING: This may cause performance issues
        self.clone()
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

    pub fn en_passant_board(&self) -> BitBoard {
        self.en_passant
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

    pub fn get_attackers_to(&self, square: Square) -> BitBoard {
        let color = !self.get_piece_color(&square);

        let attackers: BitBoard = self
            .get_bitboard(&color, &Type::None)
            .get_occupied_squares()
            .iter()
            .filter(|s| (1 << square) & generate_attack_mask_single_square(self, s, &0, &0) != 0)
            .fold(0, |acc, attacker| acc | (1 << attacker));

        attackers
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

            row.push(piece_to_icon(&color, &r#type));

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
