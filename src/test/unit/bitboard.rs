use crate::bitboard::*;
use crate::legal_moves::misc::*;

#[test]
fn test_to_bitboard() {
    assert_eq!(0.to_bitboard(), 0b1);
    assert_eq!(1.to_bitboard(), 0b10);
    assert_eq!(63.to_bitboard(), 1 << 63);
}

#[test]
fn test_get_occupied_squares() {
    let bb: BitBoard = 0b1010;
    assert_eq!(bb.get_occupied_squares(), vec![1, 3]);

    let bb: BitBoard = 1 << 32 | 1 << 16;
    assert_eq!(bb.get_occupied_squares(), vec![16, 32]);
}

#[test]
fn test_bitwise_reverse() {
    assert_eq!(0x1.bitwise_reverse(), 1 << 63);
    assert_eq!(0xF.bitwise_reverse(), 0xF << 60);
    assert_eq!(0x123456789ABCDEF.bitwise_reverse(), 0xF7B3D591E6A2C480);
}
