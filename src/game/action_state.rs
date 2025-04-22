use crate::utils::min;

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

    if input[0..min(input.len(), 7)] == "mask up" {
        return Action::MaskUp;
    }

    Action::Quit
}
