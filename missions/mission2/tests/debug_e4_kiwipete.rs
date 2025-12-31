use mission2_movegen::board::{Board, Color, PieceType, Square};
use mission2_movegen::movegen::MoveGenerator;

#[test]
fn debug_e4_pawn_kiwipete() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    let movegen = MoveGenerator::new();

    println!("FEN: {}", board.to_fen());
    println!("Piece on E4: {:?}", board.piece_at(Square::E4));
    println!("Piece on E5: {:?}", board.piece_at(Square::E5));

    // Get all legal moves
    let all_moves = movegen.generate_legal_moves(&board);
    println!("\nTotal legal moves: {}", all_moves.len());

    // Filter for E4 moves
    let e4_moves: Vec<_> = all_moves
        .moves()
        .iter()
        .filter(|m| m.from() == Square::E4)
        .collect();

    println!("Moves from E4: {}", e4_moves.len());
    for m in e4_moves {
        println!("  {}", m.to_uci());
    }

    // Get pseudo-legal moves
    let pseudo_legal = movegen.generate_pseudo_legal_moves(&board);
    let e4_pseudo: Vec<_> = pseudo_legal
        .moves()
        .iter()
        .filter(|m| m.from() == Square::E4)
        .collect();

    println!("\nPseudo-legal moves from E4: {}", e4_pseudo.len());
    for m in &e4_pseudo {
        println!("  {}", m.to_uci());
    }

    // Check if E4-E5 is legal
    if !e4_pseudo.is_empty() {
        use mission2_movegen::board_ext::BoardExt;
        use mission2_movegen::moves::Move;
        let test_move = Move::new_quiet(Square::E4, Square::E5);

        let mut test_board = board.clone();
        test_board.make_move_complete(test_move);

        println!("\nAfter E4-E5:");
        println!("FEN: {}", test_board.to_fen());
        println!(
            "White in check: {}",
            movegen.is_in_check(&test_board, Color::White)
        );
    }
}
