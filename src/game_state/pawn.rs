use super::action::ChessColor;
use super::coord::{Coordinate2, Coordinate5, Coordinate6};
use super::square::ChessFile;
pub enum PawnMove {
    Simple {
        color: ChessColor,
        file: ChessFile,
        from: Coordinate5,
    },
    Double {
        color: ChessColor,
        file: ChessFile,
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
