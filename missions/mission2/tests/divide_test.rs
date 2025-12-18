use mission2_movegen::board::Board;
use mission2_movegen::movegen::MoveGenerator;

fn perft(movegen: &MoveGenerator, board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = movegen.generate_legal_moves(board);
    
    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0;
    for m in moves.moves() {
        let mut new_board = board.clone();
        use mission2_movegen::board_ext::BoardExt;
        new_board.make_move_complete(*m);
        nodes += perft(movegen, &new_board, depth - 1);
    }

    nodes
}

fn perft_divide(movegen: &MoveGenerator, board: &Board, depth: u32) -> u64 {
    let moves = movegen.generate_legal_moves(board);
    let mut total = 0;

    println!("Moves from position (depth {}):", depth);
    for m in moves.moves() {
        let mut new_board = board.clone();
        use mission2_movegen::board_ext::BoardExt;
        new_board.make_move_complete(*m);
        let count = if depth > 1 { perft(movegen, &new_board, depth - 1) } else { 1 };
        println!("{}: {}", m.to_uci(), count);
        total += count;
    }
    println!("Total: {}", total);

    total
}

#[test]
fn divide_kiwipete_depth_2() {
    let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    let result = perft_divide(&movegen, &board, 2);
    assert_eq!(result, 2_039, "Expected 2039 nodes, got {}", result);
}

#[test]
fn divide_position3_depth_2() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    let result = perft_divide(&movegen, &board, 2);
    assert_eq!(result, 191, "Expected 191 nodes, got {}", result);
}
