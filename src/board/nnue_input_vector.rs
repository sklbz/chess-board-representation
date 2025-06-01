use super::board::Board;
use crate::bitboard::BitBoardGetter;

pub trait VectorOutput {
    fn to_vector(&self) -> Vec<f32>;
}

impl VectorOutput for Board {
    fn to_vector(&self) -> Vec<f32> {
        let mut input_vector = vec![0.0f32; 768];

        for &square in self.white_pawns.get_occupied_squares().iter() {
            input_vector[square as usize] = 1.0;
        }
        for &square in self.black_pawns.get_occupied_squares().iter() {
            input_vector[6 * 64 + square as usize] = 1.0;
        }

        for &square in self.white_knights.get_occupied_squares().iter() {
            input_vector[64 + square as usize] = 1.0;
        }
        for &square in self.black_knights.get_occupied_squares().iter() {
            input_vector[6 * 64 + 64 + square as usize] = 1.0;
        }

        for &square in self.white_bishops.get_occupied_squares().iter() {
            input_vector[2 * 64 + square as usize] = 1.0;
        }
        for &square in self.black_bishops.get_occupied_squares().iter() {
            input_vector[6 * 64 + 2 * 64 + square as usize] = 1.0;
        }

        for &square in self.white_rooks.get_occupied_squares().iter() {
            input_vector[3 * 64 + square as usize] = 1.0;
        }
        for &square in self.black_rooks.get_occupied_squares().iter() {
            input_vector[6 * 64 + 3 * 64 + square as usize] = 1.0;
        }

        for &square in self.white_queens.get_occupied_squares().iter() {
            input_vector[4 * 64 + square as usize] = 1.0;
        }
        for &square in self.black_queens.get_occupied_squares().iter() {
            input_vector[6 * 64 + 4 * 64 + square as usize] = 1.0;
        }

        for &square in self.white_king.get_occupied_squares().iter() {
            input_vector[5 * 64 + square as usize] = 1.0;
        }
        for &square in self.black_king.get_occupied_squares().iter() {
            input_vector[6 * 64 + 5 * 64 + square as usize] = 1.0;
        }

        input_vector
    }
}
