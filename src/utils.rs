use crate::{Board, Square, board, is_possible};

pub fn string_to_square(string: &str) -> Square {
    let letter = string.chars().next().expect("Unable to read letter");
    let number = string.chars().last().expect("Unable to read number");

    let column = match letter {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Invalid letter : {}", letter),
    };

    let row = match number {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => panic!("Invalid number : {}", number),
    };

    row * 8 + column
}

use std::{thread::sleep, time::Duration};
pub fn squarewise_display(board: &Board) {
    for i in 0..64 {
        let letter = match i % 8 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => unreachable!(),
        };
        print!("{}{}\t", letter, i / 8 + 1);
        is_possible(board, &(i, 0));
        sleep(Duration::from_millis(50));
    }
}

pub fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}
