pub fn is_square_notation(input: &String) -> bool {
    let parsed = input.split_whitespace().collect::<Vec<&str>>();

    input.split_whitespace().all(is_square) && parsed.len() == 2
}

pub fn is_standard_notation(_input: &String) -> bool {
    todo!("Standard notation")
}

fn is_square(input: &str) -> bool {
    let letter = input.chars().next().expect("Unreadable letter");

    let columns = match letter {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => true,
        _ => false,
    };

    let number = input.chars().last().expect("Unreadable number");

    let rows = match number {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => true,
        _ => false,
    };

    columns && rows
}
