use super::ChessColor;
use super::{Coordinate2, Coordinate5, Coordinate6, Coordinate8};
pub enum PawnMove {
    Simple {
        color: ChessColor,
        file: Coordinate8,
        from: Coordinate5,
    },
    Double {
        color: ChessColor,
        file: Coordinate8,
    },
}
pub enum PawnCapturePattern {
    Center {
        color: ChessColor,
        file: Coordinate6,
        to: Coordinate2,
    },
    Flank {
        color: ChessColor,
        flank: Coordinate2,
    },
}
pub struct PawnCapture {
    pattern: PawnCapturePattern,
    from: Coordinate5,
}
