use crate::{
    Board,
    bitboard::BitBoardGetter,
    legal_moves::{
        generate_possible_moves::generate_move_mask,
        is_move_possible::*,
        misc::{Color, Piece, Square, ToBitBoard, Type},
    },
    utils::{string_to_square, user_input},
};

#[allow(unused)]
pub fn run_debug() {
    let mut board = knight_board(
        string_to_square("a1"),
        &Board::from_mask(1, Piece::new(Type::Knight, Color::White)),
    );

    loop {
        print!("{esc}[2J", esc = 27 as char);
        board.display();

        let input = user_input();

        if input.is_empty() {
            break;
        }

        let move_to_play = (
            board
                .get_bitboard(&Color::White, &Type::Knight)
                .get_occupied_squares()[0],
            string_to_square(&input),
        );

        if is_possible(&board, &move_to_play) {
            board = knight_board(move_to_play.1, &board);
        }
    }
}

fn knight_board(square: Square, board: &Board) -> Board {
    Board::new(
        0,
        generate_move_mask(board, &square),
        square.to_bitboard(),
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    )
}
