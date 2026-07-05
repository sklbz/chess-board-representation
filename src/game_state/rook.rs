use super::direction::RookDirection;
use super::square::ChessSquare;
use super::ChessColor;
use super::Coordinate7;

pub struct RookMove {
    direction: RookDirection,
    from: ChessSquare,
    to: Coordinate7,
    color: ChessColor,
}
impl RookMove {
    fn to(self) -> ChessSquare {
        match self.direction {
            RookDirection::File => ChessSquare {
                file: self.from.file + self.to,
                rank: self.from.rank,
            },
            RookDirection::Rank => ChessSquare {
                file: self.from.file,
                rank: self.from.rank + self.to,
            },
        }
    }
}
