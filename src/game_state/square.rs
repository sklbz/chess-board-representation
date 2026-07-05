use super::Coordinate8;

pub struct ChessSquare {
    pub file: Coordinate8,
    pub rank: Coordinate8,
}
impl ChessSquare {}

// pub enum ChessFile {
//     A,
//     B,
//     C,
//     D,
//     E,
//     F,
//     G,
//     H,
// }
// pub enum ChessRank {
//     R1,
//     R2,
//     R3,
//     R4,
//     R5,
//     R6,
//     R7,
//     R8,
// }
//
// impl ChessFile {
//     pub fn as_coord(&self) -> Coordinate8 {
//         use ChessFile::*;
//         use Coordinate8::*;
//         match self {
//             A => Alpha,
//             B => Beta,
//             C => Gamma,
//             D => Delta,
//             E => Epsilon,
//             F => Zeta,
//             G => Eta,
//             H => Theta,
//         }
//     }
// }
// impl ChessRank {
//     pub fn as_coord(&self) -> Coordinate8 {
//         use ChessRank::*;
//         use Coordinate8::*;
//         match self {
//             R1 => Alpha,
//             R2 => Beta,
//             R3 => Gamma,
//             R4 => Delta,
//             R5 => Epsilon,
//             R6 => Zeta,
//             R7 => Eta,
//             R8 => Theta,
//         }
//     }
// }
