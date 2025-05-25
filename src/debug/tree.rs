use crate::{
    board::Board,
    legal_moves::{generate_possible_moves::generate_move_vec, misc::Color},
};

pub fn search_tree(board: &Board, color: Color, depth: usize) -> usize {
    if depth == 0 {
        return generate_move_vec(board, color).len();
    }

    let moves = generate_move_vec(board, color);

    let mut result = 0;

    for move_ in moves {
        let mut next_board = board.clone();
        next_board.play_move(&move_);

        result += search_tree(&next_board, !color, depth - 1);
    }

    result
}
