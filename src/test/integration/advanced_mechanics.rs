use crate::legal_moves::misc::Color;
use crate::legal_moves::misc::Move;
use crate::{
    board::Board, legal_moves::is_move_possible::is_possible, utils::move_to_string,
    utils::string_to_move,
};

#[test]
fn pinning() {
    let mut board = Board::init();

    let moves_to_play: Vec<Move> = [
        "e2e4", "d7d5", "e4d5", "h7h5", "g1e2", "h8h6", "d5d6", "h6e6",
    ]
    .iter()
    .map(|s| string_to_move(s))
    .collect();

    let mut turn = Color::White;

    for move_ in moves_to_play {
        println!("move: {}", move_to_string(&move_));
        assert!(
            is_possible(&board, &move_, turn),
            "Incorrect move : {}",
            move_to_string(&move_)
        );
        board.play_move(&move_);

        turn = !turn;
    }

    assert!(
        !is_possible(&board, &string_to_move("e2d4"), Color::White),
        "Knight at e2 should be pinned to the king"
    );
}
