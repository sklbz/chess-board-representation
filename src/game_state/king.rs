use super::ChessColor;
use super::Coordinate3;
use super::Coordinate4;
use super::Coordinate5;
use super::Coordinate6;
use super::Coordinate8;
use super::Coordinate9;

pub enum KingMove {
    Vertex {
        color: ChessColor,
        quadrant: Coordinate4,
        to: Coordinate3,
    },
    Edge {
        color: ChessColor,
        quadrant: Coordinate4,
        from: Coordinate6,
        to: Coordinate5,
    },
    Hull {
        color: ChessColor,
        quadrant: Coordinate4,
        from: Coordinate9,
        to: Coordinate8,
    },
}
