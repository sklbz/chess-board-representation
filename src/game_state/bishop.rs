use super::coord::{
    Coordinate2, Coordinate3, Coordinate4, Coordinate5, Coordinate6, Coordinate7, Coordinate8,
};

use super::action::ChessColor;

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
    from: Coordinate2,
}

pub struct BishopMove3 {
    color: ChessColor,
    diag: Coordinate2,
    from: Coordinate3,
    to: Coordinate2,
}

pub struct BishopMove4 {
    color: ChessColor,
    diag: Coordinate2,
    from: Coordinate4,
    to: Coordinate3,
}

pub struct BishopMove5 {
    color: ChessColor,
    diag: Coordinate2,
    from: Coordinate5,
    to: Coordinate4,
}

pub struct BishopMove6 {
    color: ChessColor,
    diag: Coordinate2,
    from: Coordinate6,
    to: Coordinate5,
}

pub struct BishopMove7 {
    color: ChessColor,
    diag: Coordinate2,
    from: Coordinate7,
    to: Coordinate6,
}

pub struct BishopMove8 {
    color: ChessColor,
    diag: Coordinate2,
    from: Coordinate8,
    to: Coordinate7,
}
