use super::direction::BishopDirection;
use super::ChessColor;
use super::Coordinate2;
use super::Coordinate3;
use super::Coordinate4;
use super::Coordinate5;
use super::Coordinate6;
use super::Coordinate7;
use super::Coordinate8;

pub enum BishopMove {
    Diag2(BishopMove2),
    Diag3(BishopMove3),
    Diag4(BishopMove4),
    Diag5(BishopMove5),
    Diag6(BishopMove6),
    Diag7(BishopMove7),
    Diag8(BishopMove8),
}

pub struct BishopMove2 {
    color: ChessColor,
    diag: Coordinate2,
    direction: BishopDirection,
    from: Coordinate2,
}

pub struct BishopMove3 {
    color: ChessColor,
    diag: Coordinate2,
    direction: BishopDirection,
    from: Coordinate3,
    to: Coordinate2,
}

pub struct BishopMove4 {
    color: ChessColor,
    diag: Coordinate2,
    direction: BishopDirection,
    from: Coordinate4,
    to: Coordinate3,
}

pub struct BishopMove5 {
    color: ChessColor,
    diag: Coordinate2,
    direction: BishopDirection,
    from: Coordinate5,
    to: Coordinate4,
}

pub struct BishopMove6 {
    color: ChessColor,
    diag: Coordinate2,
    direction: BishopDirection,
    from: Coordinate6,
    to: Coordinate5,
}

pub struct BishopMove7 {
    color: ChessColor,
    diag: Coordinate2,
    direction: BishopDirection,
    from: Coordinate7,
    to: Coordinate6,
}

pub struct BishopMove8 {
    color: ChessColor,
    direction: BishopDirection,
    from: Coordinate8,
    to: Coordinate7,
}
