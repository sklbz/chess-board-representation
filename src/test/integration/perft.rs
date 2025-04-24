use crate::board::Board;
use crate::legal_moves::generate_possible_moves::generate_move_vec;
use crate::legal_moves::misc::Color;
use crate::utils::move_to_string;

//Perft, (performance test, move path enumeration)
// a debugging function to walk the move generation tree of strictly legal moves to count all the leaf nodes of a certain depth, which can be compared to predetermined values and used to isolate bugs. In perft, nodes are only counted at the end after the last makemove. Thus "higher" terminal nodes (e.g. mate or stalemate) are not counted, instead the number of move paths of a certain depth. Perft ignores draws by repetition, by the fifty-move rule and by insufficient material. By recording the amount of time taken for each iteration, it's possible to compare the performance of different move generators or the same generator on different machines, though this must be done with caution since there are variations to perft.

#[test]
fn depth_1() {
    let mut board = Board::init();

    let moves = generate_move_vec(&board, Color::White);

    let perft: Vec<usize> = vec![
        20,
        400,
        8_902,
        197_281,
        4_865_609,
        119_060_324,
        3_195_901_860,
        84_998_978_956,
        2_439_530_234_167,
    ];

    let mut move_count: Vec<usize> = vec![moves.len()];

    let mut current_depth_move_count = 0;

    for played_move in moves {
        board.play_move(&played_move);

        let current = generate_move_vec(&board, Color::Black).len();
        current_depth_move_count += current;

        println!("{}: {}", move_to_string(&played_move), current);

        board = Board::init();
    }

    move_count.push(current_depth_move_count);

    for i in 0..move_count.len() {
        assert_eq!(move_count[i], perft[i]);
    }
}

#[test]
fn depth_2() {
    let mut board = Board::init();

    let moves = generate_move_vec(&board, Color::White);

    let perft: Vec<usize> = vec![
        20,
        400,
        8_902,
        197_281,
        4_865_609,
        119_060_324,
        3_195_901_860,
        84_998_978_956,
        2_439_530_234_167,
    ];

    let mut move_count: Vec<usize> = vec![moves.len()];

    let mut current_depth_move_count = 0;

    for played_move in moves {
        board.play_move(&played_move);

        let current = generate_move_vec(&board, Color::Black).len();
        current_depth_move_count += current;

        println!("{}: {}", move_to_string(&played_move), current);

        board = Board::init();
    }

    move_count.push(current_depth_move_count);

    for i in 0..move_count.len() {
        assert_eq!(move_count[i], perft[i]);
    }
}
