use crate::utils::min;

use super::algebraic_notation::is_square_notation;

pub enum Action {
    Move,
    MaskUp,
    MaskDown,
    MaskLeft,
    MaskRight,
    MaskTopLeft,
    MaskTopRight,
    MaskBottomRight,
    MaskBottomLeft,
    Quit,
}

pub fn get_action(input: &String) -> Action {
    if input.is_empty() {
        return Action::Quit;
    }

    if input == "quit" {
        return Action::Quit;
    }

    if input == "exit" {
        return Action::Quit;
    }

    if &input[0..min(input.len(), 7)] == "mask up" {
        return Action::MaskUp;
    }

    if &input[0..min(input.len(), 9)] == "mask down" {
        return Action::MaskDown;
    }

    if &input[0..min(input.len(), 9)] == "mask left" {
        return Action::MaskLeft;
    }

    if &input[0..min(input.len(), 10)] == "mask right" {
        return Action::MaskRight;
    }

    if &input[0..min(input.len(), 13)] == "mask top left" {
        return Action::MaskTopLeft;
    }

    if &input[0..min(input.len(), 14)] == "mask top right" {
        return Action::MaskTopRight;
    }

    if &input[0..min(input.len(), 17)] == "mask bottom right" {
        return Action::MaskBottomRight;
    }

    if &input[0..min(input.len(), 16)] == "mask bottom left" {
        return Action::MaskBottomLeft;
    }

    if is_square_notation(input) {
        return Action::Move;
    }

    Action::Quit
}

pub fn get_input(input: &String, action: &Action) -> String {
    match action {
        Action::Move => input.to_string(),
        Action::MaskUp => input[7..].to_string(),
        Action::MaskDown => input[9..].to_string(),
        Action::MaskLeft => input[9..].to_string(),
        Action::MaskRight => input[10..].to_string(),
        Action::MaskTopLeft => input[13..].to_string(),
        Action::MaskTopRight => input[14..].to_string(),
        Action::MaskBottomRight => input[17..].to_string(),
        Action::MaskBottomLeft => input[16..].to_string(),
        Action::Quit => input.to_string(),
    }
}
