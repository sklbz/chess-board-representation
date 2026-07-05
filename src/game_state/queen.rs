use super::bishop::BishopMove;
use super::rook::RookMove;
pub enum QueenMove {
    Orthogonal(RookMove),
    Diagonal(BishopMove),
}
