use crate::{
    board::Board,
    legal_moves::{generate_possible_moves::generate_move_vec, misc::Color},
    utils::move_to_string,
};

fn search(board: &Board, depth: usize, color: Color) -> usize {
    if depth == 0 {
        return generate_move_vec(board, color).len();
    }

    let moves = generate_move_vec(board, color);

    let mut result = 0;

    for move_ in moves {
        let mut next_board = board.clone();
        next_board.play_move(&move_);

        result += search(&next_board, depth - 1, !color);
    }

    result
}

fn divide(board: &Board, color: Color, depth: usize) -> Vec<(String, usize)> {
    let moves = generate_move_vec(board, color);

    moves
        .iter()
        .map(|move_| {
            let mut next_board = board.clone();
            next_board.play_move(move_);
            (move_to_string(move_), search(&next_board, depth - 1, color))
        })
        .collect()
}

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
