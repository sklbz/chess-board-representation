use crate::{
    board::{board::Board, fen_handling::FenHandling},
    legal_moves::{
        generate_possible_moves::generate_move_vec,
        misc::{Color, Move},
    },
};

pub struct SearchTree {
    pub root: Vec<SearchTreeNode>,
}

#[derive(Clone)]
pub struct SearchTreeNode {
    pub move_: Move,
    pub fen: String,
    pub score: usize,
    pub children: Vec<SearchTreeNode>,
}

pub fn search_tree_nodes(
    board: &Board,
    searched_move: Move,
    color: Color,
    depth: usize,
) -> SearchTreeNode {
    if depth == 0 {
        return SearchTreeNode {
            move_: searched_move,
            fen: board.to_fen(color),
            score: 1,
            children: Vec::new(),
        }; //generate_move_vec(board, color).len();
    }

    let moves: Vec<Move> = generate_move_vec(board, color);

    let mut children: Vec<SearchTreeNode> = Vec::new();

    for move_ in moves {
        let child_board = board.as_played(&move_);

        children.push(search_tree_nodes(&child_board, move_, !color, depth - 1));
    }

    let score: usize = children.iter().fold(0, |acc, x| acc + x.score);

    SearchTreeNode {
        move_: searched_move,
        fen: board.to_fen(color),
        score,
        children,
    }
}

pub fn search_tree(board: &Board, color: Color, depth: usize) -> SearchTree {
    if depth == 0 {
        panic!("Depth is 0");
    }

    let moves: Vec<Move> = generate_move_vec(board, color);

    let mut children: Vec<SearchTreeNode> = Vec::new();

    for move_ in moves {
        let child_board = board.as_played(&move_);

        children.push(search_tree_nodes(&child_board, move_, !color, depth - 1));
    }

    SearchTree { root: children }
}
