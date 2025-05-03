use crate::board::Board;
use crate::legal_moves::generate_possible_moves::generate_move_vec;
use crate::legal_moves::misc::Color;

//Perft, (performance test, move path enumeration)
// a debugging function to walk the move generation tree of strictly legal moves to count all the leaf nodes of a certain depth, which can be compared to predetermined values and used to isolate bugs. In perft, nodes are only counted at the end after the last makemove. Thus "higher" terminal nodes (e.g. mate or stalemate) are not counted, instead the number of move paths of a certain depth. Perft ignores draws by repetition, by the fifty-move rule and by insufficient material. By recording the amount of time taken for each iteration, it's possible to compare the performance of different move generators or the same generator on different machines, though this must be done with caution since there are variations to perft.

#[test]
fn depth_3() {
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

    let mut move_count: Vec<usize> = vec![moves.len(), 0, 0];

    for played_move in moves {
        board.play_move(&played_move);

        let current_moves = generate_move_vec(&board, Color::Black);

        move_count[1] += current_moves.len();

        for move_ in current_moves {
            let mut next_board = board.clone();
            next_board.play_move(&move_);

            let next_moves = generate_move_vec(&next_board, Color::White);
            move_count[2] += next_moves.len();
        }

        board = Board::init();
    }

    for i in 0..move_count.len() {
        assert_eq!(move_count[i], perft[i], "Minimal failing depth: {}", i + 1);
    }
}
#[test]
fn alternate_position() {
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");

    let moves = generate_move_vec(&board, Color::White);

    let perft: Vec<usize> = vec![48, 2039, 97862, 4085603, 193690690, 8031647685];

    let mut move_count: Vec<usize> = vec![moves.len(), 0, 0];

    for played_move in moves {
        board.play_move(&played_move);

        let current_moves = generate_move_vec(&board, Color::Black);

        move_count[1] += current_moves.len();

        for move_ in current_moves {
            let mut next_board = board.clone();
            next_board.play_move(&move_);

            let next_moves = generate_move_vec(&next_board, Color::White);
            move_count[2] += next_moves.len();
        }

        board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");
    }

    for i in 0..move_count.len() {
        assert_eq!(move_count[i], perft[i], "Minimal failing depth: {}", i + 1);
    }
}

#[test]
fn alternate_position_2() {
    let mut board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8");

    let moves = generate_move_vec(&board, Color::White);

    let perft: Vec<usize> = vec![14, 191, 2812, 43238];

    let mut move_count: Vec<usize> = vec![moves.len(), 0, 0];

    for played_move in moves {
        board.play_move(&played_move);

        let current_moves = generate_move_vec(&board, Color::Black);

        move_count[1] += current_moves.len();

        for move_ in current_moves {
            let mut next_board = board.clone();
            next_board.play_move(&move_);

            let next_moves = generate_move_vec(&next_board, Color::White);
            move_count[2] += next_moves.len();
        }

        board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8");
    }

    for i in 0..move_count.len() {
        assert_eq!(move_count[i], perft[i], "Minimal failing depth: {}", i + 1);
    }
}
