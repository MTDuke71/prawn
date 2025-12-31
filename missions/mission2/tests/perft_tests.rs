// Perft (Performance Test) - validates move generation correctness
// Counts all leaf nodes at each depth to verify move generation is bug-free

use mission2_movegen::board::{Board, Color};
use mission2_movegen::board_ext::BoardExt;
use mission2_movegen::movegen::MoveGenerator;

/// Perft function - recursively counts nodes at depth
fn perft(movegen: &MoveGenerator, board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = movegen.generate_legal_moves(board);
    let mut nodes = 0u64;

    for m in moves.moves() {
        let mut new_board = board.clone();
        new_board.make_move_complete(*m);
        nodes += perft(movegen, &new_board, depth - 1);
    }

    nodes
}

/// Perft with move breakdown (for debugging)
fn perft_divide(movegen: &MoveGenerator, board: &Board, depth: u32) {
    let moves = movegen.generate_legal_moves(board);
    let mut total = 0u64;

    for m in moves.moves() {
        let mut new_board = board.clone();
        new_board.make_move_complete(*m);
        let count = perft(movegen, &new_board, depth - 1);
        println!("{}: {}", m.to_uci(), count);
        total += count;
    }

    println!("\nTotal: {}", total);
}

// ============================================================================
// Starting Position Perft Tests
// ============================================================================

#[test]
fn perft_startpos_depth_1() {
    let board = Board::default(); // Starting position
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 1), 20);
}

#[test]
fn perft_startpos_depth_2() {
    let board = Board::default();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 2), 400);
}

#[test]
fn perft_startpos_depth_3() {
    let board = Board::default();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 3), 8_902);
}

#[test]
fn perft_startpos_depth_4() {
    let board = Board::default();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 4), 197_281);
}

#[test]
#[ignore] // Takes a few seconds
fn perft_startpos_depth_5() {
    let board = Board::default();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 5), 4_865_609);
}

#[test]
#[ignore] // Takes ~1-2 minutes
fn perft_startpos_depth_6() {
    let board = Board::default();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 6), 119_060_324);
}

// ============================================================================
// Kiwipete Position - Complex middlegame position
// ============================================================================

#[test]
fn perft_kiwipete_depth_1() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 1), 48);
}

#[test]
fn perft_kiwipete_depth_2() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 2), 2_039);
}

#[test]
fn perft_kiwipete_depth_3() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 3), 97_862);
}

#[test]
#[ignore] // Takes a few seconds
fn perft_kiwipete_depth_4() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 4), 4_085_603);
}

#[test]
#[ignore] // Takes ~1-2 minutes
fn perft_kiwipete_depth_5() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 5), 193_690_690);
}

// ============================================================================
// Position 3 - Castling rights
// ============================================================================

#[test]
fn perft_position3_depth_1() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 1), 14);
}

#[test]
fn perft_position3_depth_2() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 2), 191);
}

#[test]
fn perft_position3_depth_3() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 3), 2_812);
}

#[test]
fn perft_position3_depth_4() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 4), 43_238);
}

#[test]
#[ignore] // Takes a few seconds
fn perft_position3_depth_5() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 5), 674_624);
}

// ============================================================================
// Position 4 - En passant and promotions
// ============================================================================

#[test]
fn perft_position4_depth_1() {
    let board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
        .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 1), 6);
}

#[test]
fn perft_position4_depth_2() {
    let board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
        .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 2), 264);
}

#[test]
fn perft_position4_depth_3() {
    let board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
        .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 3), 9_467);
}

#[test]
#[ignore] // Takes a few seconds
fn perft_position4_depth_4() {
    let board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
        .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 4), 422_333);
}

// ============================================================================
// Position 5 - More complex
// ============================================================================

#[test]
fn perft_position5_depth_1() {
    let board =
        Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 1), 44);
}

#[test]
fn perft_position5_depth_2() {
    let board =
        Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 2), 1_486);
}

#[test]
fn perft_position5_depth_3() {
    let board =
        Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 3), 62_379);
}

#[test]
#[ignore] // Takes a few seconds
fn perft_position5_depth_4() {
    let board =
        Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 4), 2_103_487);
}

// ============================================================================
// Position 6 - Endgame
// ============================================================================

#[test]
fn perft_position6_depth_1() {
    let board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10")
            .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 1), 46);
}

#[test]
fn perft_position6_depth_2() {
    let board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10")
            .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 2), 2_079);
}

#[test]
fn perft_position6_depth_3() {
    let board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10")
            .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 3), 89_890);
}

#[test]
#[ignore] // Takes a few seconds
fn perft_position6_depth_4() {
    let board =
        Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10")
            .unwrap();
    let movegen = MoveGenerator::new();

    assert_eq!(perft(&movegen, &board, 4), 3_894_594);
}

// ============================================================================
// Utility function for manual testing
// ============================================================================

#[test]
#[ignore]
fn perft_divide_startpos() {
    let board = Board::default();
    let movegen = MoveGenerator::new();

    println!("\nPerft divide for starting position (depth 4):");
    perft_divide(&movegen, &board, 4);
}
