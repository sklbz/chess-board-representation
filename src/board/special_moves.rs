use crate::{legal_moves::misc::Color, utils::string_to_move};

use super::{board::Board, unmove::Unmove};

pub trait Castle {
    fn castle(&mut self, code: &str, side: &Color) -> Unmove;
}

impl Castle for Board {
    fn castle(&mut self, code: &str, side: &Color) -> Unmove {
        match (code, side) {
            ("O-O", Color::White) => {
                let _ = self.make_move(&string_to_move("e1g1"));
                let _ = self.make_move(&string_to_move("h1f1"));

                Unmove {
                    moves: vec![string_to_move("g1e1"), string_to_move("f1h1")],
                    castling_rights: self.castling_rights,
                    en_passant_square: None,
                    en_passant_pawn: None,
                    en_passant: self.en_passant,
                }
            }
            ("O-O", Color::Black) => {
                let _ = self.make_move(&string_to_move("e8g8"));
                let _ = self.make_move(&string_to_move("h8f8"));

                Unmove {
                    moves: vec![string_to_move("g8e8"), string_to_move("f8h8")],
                    castling_rights: self.castling_rights,
                    en_passant_square: None,
                    en_passant_pawn: None,
                    en_passant: self.en_passant,
                }
            }
            ("O-O-O", Color::White) => {
                let _ = self.make_move(&string_to_move("e1c1"));
                let _ = self.make_move(&string_to_move("a1d1"));

                Unmove {
                    moves: vec![string_to_move("c1e1"), string_to_move("d1a1")],
                    castling_rights: self.castling_rights,
                    en_passant_square: None,
                    en_passant_pawn: None,
                    en_passant: self.en_passant,
                }
            }
            ("O-O-O", Color::Black) => {
                let _ = self.make_move(&string_to_move("e8c8"));
                let _ = self.make_move(&string_to_move("a8d8"));

                Unmove {
                    moves: vec![string_to_move("c8e8"), string_to_move("d8a8")],
                    castling_rights: self.castling_rights,
                    en_passant_square: None,
                    en_passant_pawn: None,
                    en_passant: self.en_passant,
                }
            }
            _ => panic!("Invalid castle code!"),
        }
    }
}
