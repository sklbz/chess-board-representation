use crate::{
    Square,
    bitboard::BitBoard,
    bitmask::{
        bottom_left_mask, bottom_right_mask, down_mask, left_mask, right_mask, top_left_mask,
        top_right_mask, up_mask,
    },
};

pub enum MaskAction {
    Up,
    Down,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl MaskAction {
    pub fn get_mask(&self) -> Box<dyn Fn(Square) -> BitBoard> {
        match self {
            MaskAction::Up => Box::new(up_mask),
            MaskAction::Down => Box::new(down_mask),
            MaskAction::Left => Box::new(left_mask),
            MaskAction::Right => Box::new(right_mask),
            MaskAction::TopLeft => Box::new(top_left_mask),
            MaskAction::TopRight => Box::new(top_right_mask),
            MaskAction::BottomRight => Box::new(bottom_right_mask),
            MaskAction::BottomLeft => Box::new(bottom_left_mask),
        }
    }
}
