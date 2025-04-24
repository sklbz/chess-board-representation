use crate::{
    Board,
    game::action_state::{Action, get_action, get_input, get_mask},
    legal_moves::{
        is_move_possible::*,
        misc::{Color, Piece, Type},
    },
    utils::{extract_move, extract_square, user_input},
};

pub fn run(board: &mut Board) {
    loop {
        print!("{esc}[2J", esc = 27 as char);
        board.display();

        let input = user_input();

        let action: Action = get_action(&input);

        let game_input = get_input(&input, &action);

        match action {
            Action::Move => {
                let move_to_play = extract_move(&game_input);

                if is_possible(board, &move_to_play) {
                    board.play_move(&move_to_play)
                }
            }

            Action::Mask(_) => {
                let square = extract_square(&game_input);
                let mask = get_mask(&action);

                let test = Board::from_mask(mask(square), Piece::new(Type::Pawn, Color::White));

                test.display();

                break;
            }

            Action::Quit => break,
        }
    }
}
