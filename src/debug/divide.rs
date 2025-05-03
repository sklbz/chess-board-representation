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

pub fn divide(board: &Board, color: Color, depth: usize) -> Vec<(String, usize)> {
    let moves = generate_move_vec(board, color);

    moves
        .iter()
        .map(|move_| {
            let mut next_board = board.clone();
            next_board.play_move(move_);

            let nodes_count = if depth == 0 {
                1
            } else {
                search(&next_board, depth - 1, !color)
            };

            (move_to_string(move_), nodes_count)
        })
        .collect()
}
