use super::ChessColor;
use super::{Coordinate2, Coordinate3, Coordinate4, Coordinate5, Coordinate6, Coordinate8};

pub enum KnightMove {
    Move2(KnightMove2),
    Move3(KnightMove3),
    Move4(KnightMove4),
    Move6(KnightMove6),
    Move8(KnightMove8),
}
pub struct KnightMove2 {
    color: ChessColor,
    quadrant: Coordinate4,
    to: Coordinate2,
}
pub struct KnightMove3 {
    color: ChessColor,
    quadrant: Coordinate4,
    from: Coordinate2,
    to: Coordinate3,
}
pub struct KnightMove4 {
    color: ChessColor,
    quadrant: Coordinate4,
    from: Coordinate5,
    to: Coordinate4,
}
pub struct KnightMove6 {
    color: ChessColor,
    quadrant: Coordinate4,
    from: Coordinate4,
    to: Coordinate6,
}
pub struct KnightMove8 {
    color: ChessColor,
    quadrant: Coordinate4,
    from: Coordinate4,
    to: Coordinate8,
}
