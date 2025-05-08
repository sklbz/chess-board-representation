use crate::{
    Board,
    game::action_state::{Action, get_action, get_input, get_mask},
    legal_moves::{
        is_move_possible::*,
        misc::{Color, Piece, Type},
    },
    utils::{extract_move, extract_square, user_input},
};

use std::{thread, time};

pub fn run(board: &mut Board) {
    let mut play_turn = Color::White;
    loop {
        print!("{esc}[2J", esc = 27 as char);
        board.display();

        let input = user_input();

        let action: Action = get_action(&input);

        let game_input = get_input(&input, &action);

        match action {
            Action::Move => {
                let move_to_play = extract_move(&game_input);

                if is_possible(board, &move_to_play, play_turn) {
                    board.play_move(&move_to_play);
                    play_turn = !play_turn;
                } else {
                    println!("Illegal move: {}", game_input);
                    thread::sleep(time::Duration::from_millis(1000));
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
