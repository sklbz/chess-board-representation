use crate::{
    Board, Color, Type,
    bitboard::Display,
    game::action_state::{Action, get_action, get_input, get_mask},
    utils::{extract_move, extract_square, user_input},
};

pub fn run(board: &mut Board) {
    loop {
        print!("{esc}[2J", esc = 27 as char);
        board.display();

        let input = user_input();

        let action: Action = get_action(&input);

        let game_input = get_input(&input, &action);

        let square = extract_square(&game_input);

        let mask = get_mask(&action);

        match action {
            Action::Move => board.play_move(&extract_move(&game_input)),

            Action::Mask(_) => {
                let test = Board::from_mask(mask(square));

                test.display();

                break;
            }

            Action::Quit => break,
        }
    }
}
