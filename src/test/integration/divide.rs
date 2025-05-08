use crate::{board::Board, debug::divide::divide, legal_moves::misc::Color};

fn depth_3() {
    let board = Board::init();

    let stockfish_ref = [
        ("a2a3", 380),
        ("b2b3", 420),
        ("c2c3", 420),
        ("d2d3", 539),
        ("e2e3", 599),
        ("f2f3", 380),
        ("g2g3", 420),
        ("h2h3", 380),
        ("a2a4", 420),
        ("b2b4", 421),
        ("c2c4", 441),
        ("d2d4", 560),
        ("e2e4", 600),
        ("f2f4", 401),
        ("g2g4", 421),
        ("h2h4", 420),
        ("b1a3", 400),
        ("b1c3", 440),
        ("g1f3", 440),
        ("g1h3", 400),
    ];

    let depth = 3;
    let result = divide(&board, Color::White, depth - 1);

    for (move_, count) in result {
        println!("{}: {}", move_, count);
        let reference = stockfish_ref.iter().find(|(m, _)| *m == move_).unwrap();
        assert_eq!(
            count, reference.1,
            "\nDepth: {}\nEngine: {} => {}\n Stockfish: {} => {}",
            depth, move_, count, reference.0, reference.1
        );
    }
}

fn depth_4() {
    let board = Board::init();

    let stockfish_ref = [
        ("a2a3", 8457),
        ("b2b3", 9345),
        ("c2c3", 9272),
        ("d2d3", 11959),
        ("e2e3", 13134),
        ("f2f3", 8457),
        ("g2g3", 9345),
        ("h2h3", 8457),
        ("a2a4", 9329),
        ("b2b4", 9332),
        ("c2c4", 9744),
        ("d2d4", 12435),
        ("e2e4", 13160),
        ("f2f4", 8929),
        ("g2g4", 9328),
        ("h2h4", 9329),
        ("b1a3", 8885),
        ("b1c3", 9755),
        ("g1f3", 9748),
        ("g1h3", 8881),
    ];

    let depth = 4;
    let result = divide(&board, Color::White, depth - 1);

    for (move_, count) in result {
        println!("{}: {}", move_, count);
        let reference = stockfish_ref.iter().find(|(m, _)| *m == move_).unwrap();
        assert_eq!(
            count, reference.1,
            "\nDepth: {}\nEngine: {} => {}\n Stockfish: {} => {}",
            depth, move_, count, reference.0, reference.1
        );
    }
}

fn depth_5() {
    let board = Board::init();

    let stockfish_ref = [
        ("a2a3", 181046),
        ("b2b3", 215255),
        ("c2c3", 222861),
        ("d2d3", 328511),
        ("e2e3", 402988),
        ("f2f3", 178889),
        ("g2g3", 217210),
        ("h2h3", 181044),
        ("a2a4", 217832),
        ("b2b4", 216145),
        ("c2c4", 240082),
        ("d2d4", 361790),
        ("e2e4", 405385),
        ("f2f4", 198473),
        ("g2g4", 214048),
        ("h2h4", 218829),
        ("b1a3", 198572),
        ("b1c3", 234656),
        ("g1f3", 233491),
        ("g1h3", 198502),
    ];

    let depth = 5;
    let result = divide(&board, Color::White, depth - 1);

    for (move_, count) in result {
        println!("{}: {}", move_, count);
        let reference = stockfish_ref.iter().find(|(m, _)| *m == move_).unwrap();
        assert_eq!(
            count, reference.1,
            "\nDepth: {}\nEngine: {} => {}\n Stockfish: {} => {}",
            depth, move_, count, reference.0, reference.1
        );
    }
}

#[test]
fn perft() {
    depth_3();
    depth_4();
    depth_5();
}

#[test]
fn alternate_position() {
    let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");

    let stockfish_ref = [
        ("a2a3", 2186),
        ("b2b3", 1964),
        ("g2g3", 1882),
        ("d5d6", 1991),
        ("a2a4", 2149),
        ("g2g4", 1843),
        ("g2h3", 1970),
        ("d5e6", 2241),
        ("c3b1", 2038),
        ("c3d1", 2040),
        ("c3a4", 2203),
        ("c3b5", 2138),
        ("e5d3", 1803),
        ("e5c4", 1880),
        ("e5g4", 1878),
        ("e5c6", 2027),
        ("e5g6", 1997),
        ("e5d7", 2124),
        ("e5f7", 2080),
        ("d2c1", 1963),
        ("d2e3", 2136),
        ("d2f4", 2000),
        ("d2g5", 2134),
        ("d2h6", 2019),
        ("e2d1", 1733),
        ("e2f1", 2060),
        ("e2d3", 2050),
        ("e2c4", 2082),
        ("e2b5", 2057),
        ("e2a6", 1907),
        ("a1b1", 1969),
        ("a1c1", 1968),
        ("a1d1", 1885),
        ("h1f1", 1929),
        ("h1g1", 2013),
        ("f3d3", 2005),
        ("f3e3", 2174),
        ("f3g3", 2214),
        ("f3h3", 2360),
        ("f3f4", 2132),
        ("f3g4", 2169),
        ("f3f5", 2396),
        ("f3h5", 2267),
        ("f3f6", 2111),
        ("e1d1", 1894),
        ("e1f1", 1855),
        ("e1g1", 2059),
        ("e1c1", 1887),
    ];

    let depth = 3;
    let result = divide(&board, Color::White, depth - 1);

    for (move_, count) in result {
        println!("{}: {}", move_, count);
        let reference = stockfish_ref.iter().find(|(m, _)| *m == move_).unwrap();
        assert_eq!(
            count, reference.1,
            "\nEngine: {} => {}\n Stockfish: {} => {}",
            move_, count, reference.0, reference.1
        );
    }
}
