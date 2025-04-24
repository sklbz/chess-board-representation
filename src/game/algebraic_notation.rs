pub fn is_square_notation(input: &str) -> bool {
    let parsed = input.split_whitespace().collect::<Vec<&str>>();

    input.split_whitespace().all(is_square) && parsed.len() == 2
}

pub fn is_standard_notation(_input: &str) -> bool {
    todo!("Standard notation")
}

fn is_square(input: &str) -> bool {
    let letter = input.chars().next().expect("Unreadable letter");

    let columns = matches!(letter, 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h');

    let number = input.chars().last().expect("Unreadable number");

    let rows = matches!(number, '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8');

    columns && rows
}
