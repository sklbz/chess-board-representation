use crate::{board::Board, debug::divide::divide, legal_moves::misc::Color, utils::string_to_move};

#[test]
fn depth_3() {
    let board = Board::init();

    let stockfish_ref = [
        ("a2a3", 380),
        ("b2b3", 420),
        ("c2c3", 420),
        ("d2d3", 539),
        ("e2e3", 599),
        ("f2f3", 380),
        ("g2g3", 420),
        ("h2h3", 380),
        ("a2a4", 420),
        ("b2b4", 421),
        ("c2c4", 441),
        ("d2d4", 560),
        ("e2e4", 600),
        ("f2f4", 401),
        ("g2g4", 421),
        ("h2h4", 420),
        ("b1a3", 400),
        ("b1c3", 440),
        ("g1f3", 440),
        ("g1h3", 400),
    ];

    let depth = 3;
    let result = divide(&board, Color::White, depth - 1);

    for (move_, count) in result {
        println!("{}: {}", move_, count);
        let reference = stockfish_ref.iter().find(|(m, _)| *m == move_).unwrap();
        assert_eq!(
            count, reference.1,
            "\nEngine: {} => {}\n Stockfish: {} => {}",
            move_, count, reference.0, reference.1
        );
    }
}

#[test]
fn b1a3_depth_3() {
    let mut board = Board::init();

    let mut depth = 3;

    board.play_move(&string_to_move("b1a3"));

    depth -= 1;

    let stockfish_ref = [
        ("a7a6", 20),
        ("b7b6", 20),
        ("c7c6", 20),
        ("d7d6", 20),
        ("e7e6", 20),
        ("f7f6", 20),
        ("g7g6", 20),
        ("h7h6", 20),
        ("a7a5", 20),
        ("b7b5", 20),
        ("c7c5", 20),
        ("d7d5", 20),
        ("e7e5", 20),
        ("f7f5", 20),
        ("g7g5", 20),
        ("h7h5", 20),
        ("b8a6", 20),
        ("b8c6", 20),
        ("g8f6", 20),
        ("g8h6", 20),
    ];

    let result = divide(&board, Color::Black, depth - 1);

    for (move_, count) in result {
        println!("{}: {}", move_, count);
        let reference = stockfish_ref.iter().find(|(m, _)| *m == move_).unwrap();
        assert_eq!(
            count, reference.1,
            "\nEngine: {} => {}\n Stockfish: {} => {}",
            move_, count, reference.0, reference.1
        );
    }
}
