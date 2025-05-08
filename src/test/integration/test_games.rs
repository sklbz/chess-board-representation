use crate::board::Board;
use crate::legal_moves::is_move_possible::*;
use crate::legal_moves::misc::{Color, Move};
use crate::utils::move_to_string;
use crate::utils::{string_to_move, string_to_square};

#[test]
fn opening() {
    let mut board = Board::init();

    let moves = [
        ("e2", "e4"),
        ("c7", "c6"),
        ("d2", "d4"),
        ("d7", "d5"),
        ("e4", "e5"),
        ("c6", "c5"),
        ("g1", "f3"),
        ("c8", "g4"),
        ("c1", "e3"),
        ("b8", "c6"),
        ("d4", "c5"),
        ("e7", "e6"),
        ("h2", "h3"),
        ("g4", "f3"),
        ("d1", "f3"),
        ("c6", "e5"),
        ("f3", "d1"),
        ("d8", "a5"),
        ("c2", "c3"),
        ("f8", "c5"),
        ("e3", "c5"),
        ("a5", "c5"),
        ("f1", "e2"),
        ("g8", "f6"),
    ]
    .iter()
    .map(|(a, b)| (string_to_square(a), string_to_square(b)))
    .collect::<Vec<Move>>();

    for move_ in moves.iter() {
        assert!(is_possible(&board, move_, Color::White));
        board.play_move(move_);
    }
}

#[test]
fn full_game_no_castling_no_en_passant_no_promotion() {
    let mut board = Board::init();

    let game: Vec<Move> = [
        "e2e4", "c7c6", "d2d4", "d7d5", "b1c3", "d5e4", "c3e4", "g8f6", "e4f6", "e7f6", "c2c3",
        "c8f5", "g1e2", "g7g6", "e2g3", "f5e6", "f1d3", "b8d7", "f2f3", "d8c7", "e1f2", "c7d8",
        "h1e1", "d8c7", "f2g1", "f8d6", "d3c4", "d6g3", "h2g3", "c7g3", "c4e6", "f7e6", "e1e6",
        "e8d8", "e6e8", "h8e8", "d1e1", "g3f3", "e1e8", "d8c7", "e8a8", "f3g2", "g1g2", "h7h5",
        "g2g3", "h5h4", "g3g4", "h4h3", "a8a7", "h3h2", "a7b7", "c7d8", "b7c6", "d8e7", "c6e6",
        "e7e6", "c1g5", "f6g5", "g4g5", "d7b6", "a1h1", "b6c4", "h1h2", "c4b2", "h2h6", "b2a4",
        "h6g6", "e6d5", "g6c6", "a4c3", "c6c3", "d5d4", "a2a3", "d4c3", "g5f4", "c3b2", "f4e3",
        "b2a3",
    ]
    .iter()
    .map(|s| string_to_move(s))
    .collect();

    let mut turn = Color::White;

    for (move_, i) in game.iter().zip(1..) {
        assert!(
            is_possible(&board, move_, turn),
            "Failed at move {i}: {}",
            move_to_string(move_)
        );
        board.play_move(move_);

        turn = !turn;
    }
}

#[test]
fn magnus_carlsen() {
    let mut board = Board::init();

    let game = "e2e4 g8f6 e4e5 f6d5 d2d4 d7d6 g1f3 d6e5 f3e5 c7c6 
        f1d3 b8d7 e5f3 d7f6 h2h3 d5b4 d3e2 c8f5 b1a3 e7e6 c2c3 b4d5 a3c4 f8e7 
        O-O O-O c4e5 c6c5 a2a3 h7h6 c3c4 d5b6 d4c5 e7c5 b2b4 c5e7 c1b2 d8c7 
        d1b3 a7a5 a1d1 a5b4 a3b4 f8c8 b2d4 b6d7 c4c5 f6d5 e2c4 d7f6 f1e1 b7b6 
        c5b6 d5b6 d4b6 c7b6 e5f7 g8f7 c4e6 f5e6 e1e6 c8c3 e6f6 f7f6 b3c3 f6f7 
        f3e5 f7g8 c3c4 g8h8 c4e4 a8e8 e5g6 h8g8 g6e7 g8f7 d1e1 b6d6";

    let mut turn = Color::White;

    for (move_, i) in game.split_whitespace().zip(1..) {
        if move_.starts_with('O') {
            board.castle(
                move_,
                if let 1 = i % 2 {
                    &Color::White
                } else {
                    &Color::Black
                },
            );

            continue;
        }

        let algebric_move = string_to_move(move_);

        assert!(
            is_possible(&board, &algebric_move, turn),
            "Failed at move {i}: {}",
            move_
        );
        board.play_move(&algebric_move);

        turn = !turn;
    }
}
