pub enum ChessFile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
pub enum ChessRank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}
pub struct ChessSquare {
    file: ChessFile,
    rank: ChessRank,
}
