use crate::{
    legal_moves::misc::{Color, Piece, Square},
    utils::{piece_from_char, piece_to_char},
};

use super::board::Board;

pub trait FenHandling {
    fn from_fen(fen: &str) -> Self;
    fn to_fen(&self, color: Color) -> String;
}

impl FenHandling for Board {
    fn from_fen(fen: &str) -> Board {
        let mut board = Board::empty();
        let mut pieces: Vec<(Piece, Square)> = Vec::new();

        let piece_str = fen.split_whitespace().next().unwrap();
        let castling = fen
            .split_whitespace()
            .nth(2)
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

    fn to_fen(&self, color: Color) -> String {
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

        let turn = match color {
            Color::White => 'w',
            Color::Black => 'b',
            _ => panic!("Invalid color"),
        };

        format!("{} {} {} 0 1", fen, turn, castling)
    }
}
