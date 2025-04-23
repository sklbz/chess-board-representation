use crate::{Square, bitboard::BitBoard, utils::min};

use super::{algebraic_notation::is_square_notation, mask_action::MaskAction};

pub enum Action {
    Move,
    Mask(MaskAction),
    Quit,
}
impl Action {
    pub fn get_mask_action(&self) -> Option<&MaskAction> {
        match self {
            Action::Mask(maskAction) => Some(maskAction),
            _ => None,
        }
    }
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
        return Action::Mask(MaskAction::Up);
    }

    if &input[0..min(input.len(), 9)] == "mask down" {
        return Action::Mask(MaskAction::Down);
    }

    if &input[0..min(input.len(), 9)] == "mask left" {
        return Action::Mask(MaskAction::Left);
    }

    if &input[0..min(input.len(), 10)] == "mask right" {
        return Action::Mask(MaskAction::Right);
    }

    if &input[0..min(input.len(), 13)] == "mask top left" {
        return Action::Mask(MaskAction::TopLeft);
    }

    if &input[0..min(input.len(), 14)] == "mask top right" {
        return Action::Mask(MaskAction::TopRight);
    }

    if &input[0..min(input.len(), 17)] == "mask bottom right" {
        return Action::Mask(MaskAction::BottomRight);
    }

    if &input[0..min(input.len(), 16)] == "mask bottom left" {
        return Action::Mask(MaskAction::BottomLeft);
    }

    if &input[0..min(input.len(), 10)] == "diag right" {
        return Action::Mask(MaskAction::RightDiagonal);
    }

    if is_square_notation(input) {
        return Action::Move;
    }

    Action::Quit
}

pub fn get_input(input: &String, action: &Action) -> String {
    match action {
        Action::Move => return input.to_string(),
        Action::Quit => return input.to_string(),
        _ => (),
    };

    let mask: &MaskAction = match action.get_mask_action() {
        Some(mask) => mask,
        _ => return input.to_string(),
    };

    match mask {
        MaskAction::Up => input[7..].to_string(),
        MaskAction::Down => input[9..].to_string(),
        MaskAction::Left => input[9..].to_string(),
        MaskAction::Right => input[10..].to_string(),
        MaskAction::TopLeft => input[13..].to_string(),
        MaskAction::TopRight => input[14..].to_string(),
        MaskAction::BottomRight => input[17..].to_string(),
        MaskAction::BottomLeft => input[16..].to_string(),
        MaskAction::RightDiagonal => input[10..].to_string(),
    }
}

pub fn get_mask(action: &Action) -> Box<dyn Fn(Square) -> BitBoard> {
    match action {
        Action::Mask(mask_action) => mask_action.get_mask(),
        _ => Box::new(|_| 0),
    }
}
