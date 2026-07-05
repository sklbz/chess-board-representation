use super::Coordinate7;
use super::Coordinate8;
use std::ops::Add;

impl Add<Coordinate7> for Coordinate8 {
    type Output = Self;
    fn add(self, rhs: Coordinate7) -> Self {
        unsafe { std::mem::transmute((self as u8 + rhs as u8 + 1) % 8) }
    }
}
