use crate::{
    Board,
    bitmask::{
        bottom_right_mask, down_mask, left_mask, right_mask, top_left_mask, top_right_mask, up_mask,
    },
    game::action_state::{Action, get_action, get_input},
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
            Action::Move => board.play_move(&extract_move(game_input)),

            Action::MaskUp => {
                let square: u64 = extract_square(game_input);

                let test = Board::from_mask(up_mask(square));

                test.display();

                break;
            }

            Action::MaskDown => {
                let square: u64 = extract_square(game_input);

                let test = Board::from_mask(down_mask(square));
                test.display();

                break;
            }

            Action::MaskLeft => {
                let square: u64 = extract_square(game_input);

                let test = Board::from_mask(left_mask(square));
                test.display();

                break;
            }

            Action::MaskRight => {
                let square: u64 = extract_square(game_input);

                let test = Board::from_mask(right_mask(square));
                test.display();

                break;
            }

            Action::MaskTopLeft => {
                let square: u64 = extract_square(game_input);

                let test = Board::from_mask(top_left_mask(square));
                test.display();

                break;
            }

            Action::MaskTopRight => {
                let square: u64 = extract_square(game_input);

                let test = Board::from_mask(top_right_mask(square));
                test.display();

                break;
            }

            Action::MaskBottomRight => {
                let square: u64 = extract_square(game_input);

                let test = Board::from_mask(bottom_right_mask(square));
                test.display();

                break;
            }

            Action::MaskBottomLeft => {
                let square: u64 = extract_square(game_input);

                let test = Board::from_mask(bottom_right_mask(square));
                test.display();

                break;
            }

            Action::Quit => break,
        }
    }
}
