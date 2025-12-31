use mission2_movegen::board::{Board, Square};
use mission2_movegen::board_ext::BoardExt;
use mission2_movegen::movegen::MoveGenerator;
use mission2_movegen::moves::Move;

#[test]
fn debug_en_passant_square() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    println!("Initial FEN: {}", board.to_fen());
    println!("En passant square: {:?}", board.en_passant_square());

    // Make a double pawn push e2e4
    let mut new_board = board.clone();
    let m = Move::new_double_pawn_push(Square::E2, Square::E4);
    new_board.make_move_complete(m);

    println!("\nAfter e2e4:");
    println!("FEN: {}", new_board.to_fen());
    println!("En passant square: {:?}", new_board.en_passant_square());
    println!("Expected: Some(E3)");

    // Now generate legal moves for black
    let black_moves = movegen.generate_legal_moves(&new_board);
    println!("\nBlack has {} legal moves", black_moves.len());

    // Check for en passant captures
    use mission2_movegen::moves::MoveType;
    let ep_moves: Vec<_> = black_moves
        .moves()
        .iter()
        .filter(|m| matches!(m.move_type(), MoveType::EnPassant))
        .collect();

    println!("En passant captures available: {}", ep_moves.len());
    for m in ep_moves {
        println!("  {}", m.to_uci());
    }
}
