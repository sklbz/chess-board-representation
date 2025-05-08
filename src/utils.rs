use crate::bitboard::{BitBoard, BitBoardGetter};
use crate::board::Board;
use crate::legal_moves::misc::{Color, Move, Piece, Square, Type};

pub fn string_to_file(file: char) -> u8 {
    match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Invalid file : {}", file),
    }
}

pub fn piece_to_icon(color: &Color, piece_type: &Type) -> char {
    match (color, piece_type) {
        (Color::White, Type::Pawn) => '󰡙',
        (Color::Black, Type::Pawn) => '♙',
        (Color::White, Type::Knight) => '󰡘',
        (Color::Black, Type::Knight) => '',
        (Color::White, Type::Bishop) => '󰡜',
        (Color::Black, Type::Bishop) => '',
        (Color::White, Type::Rook) => '󰡛',
        (Color::Black, Type::Rook) => '',
        (Color::White, Type::Queen) => '󰡚',
        (Color::Black, Type::Queen) => '',
        (Color::White, Type::King) => '󰡗',
        (Color::Black, Type::King) => '',
        _ => ' ',
    }
}

pub fn piece_from_char(char: char) -> Piece {
    match char {
        'P' => Piece {
            r#type: Type::Pawn,
            color: Color::White,
        },
        'p' => Piece {
            r#type: Type::Pawn,
            color: Color::Black,
        },
        'N' => Piece {
            r#type: Type::Knight,
            color: Color::White,
        },
        'n' => Piece {
            r#type: Type::Knight,
            color: Color::Black,
        },
        'B' => Piece {
            r#type: Type::Bishop,
            color: Color::White,
        },
        'b' => Piece {
            r#type: Type::Bishop,
            color: Color::Black,
        },
        'R' => Piece {
            r#type: Type::Rook,
            color: Color::White,
        },
        'r' => Piece {
            r#type: Type::Rook,
            color: Color::Black,
        },
        'Q' => Piece {
            r#type: Type::Queen,
            color: Color::White,
        },
        'q' => Piece {
            r#type: Type::Queen,
            color: Color::Black,
        },
        'K' => Piece {
            r#type: Type::King,
            color: Color::White,
        },
        'k' => Piece {
            r#type: Type::King,
            color: Color::Black,
        },
        _ => panic!("Invalid FEN"),
    }
}

pub fn move_to_string((start, end): &Move) -> String {
    let start = square_to_string(*start);
    let end = square_to_string(*end);

    format!("{}{}", start, end)
}

pub fn string_to_move(string: &str) -> Move {
    let start = string_to_square(&string[0..2]);
    let end = string_to_square(&string[2..4]);

    (start, end)
}

pub fn square_to_string(square: Square) -> String {
    let letter = match square % 8 {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!("Invalid square : {}", square),
    };

    let number = match square / 8 {
        0 => '1',
        1 => '2',
        2 => '3',
        3 => '4',
        4 => '5',
        5 => '6',
        6 => '7',
        7 => '8',
        _ => panic!("Invalid square : {}", square),
    };

    format!("{}{}", letter, number)
}

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
pub fn squarewise_display(_board: &Board) {
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
        sleep(Duration::from_millis(50));
    }
}

pub fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

pub fn extract_square(input: &str) -> Square {
    input
        .split_whitespace()
        .map(string_to_square)
        .collect::<Vec<u8>>()[0]
}

pub fn extract_move(input: &str) -> (Square, Square) {
    let squares: Vec<u8> = input
        .split_whitespace()
        .map(string_to_square)
        .collect::<Vec<u8>>();

    (squares[0], squares[1])
}

use std::io::stdin;
pub fn user_input() -> String {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    input.pop();

    input.trim().to_string()
}

pub fn mask_to_moves(mask: BitBoard, start: &Square) -> Vec<Move> {
    mask.get_occupied_squares()
        .iter()
        .map(|square| (*start, *square))
        .collect::<Vec<Move>>()
}
