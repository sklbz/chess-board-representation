use super::action::ChessColor;
use super::coord::Coordinate7;
use super::direction::RookDirection;
use super::square::ChessSquare;

pub struct RookMove {
    pub direction: RookDirection,
    pub from: ChessSquare,
    pub to: Coordinate7,
    pub color: ChessColor,
}
