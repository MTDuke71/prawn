use mission2_movegen::board::{Board, Square};
use mission2_movegen::movegen::MoveGenerator;

#[test]
fn debug_pinned_piece() {
    // Pawn is pinned to king by rook
    let board = Board::from_fen("4r3/8/8/8/4P3/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    println!("Board FEN: {}", board.to_fen());
    println!("Side to move: {:?}", board.side_to_move());

    let all_moves = movegen.generate_legal_moves(&board);
    println!("Total legal moves: {}", all_moves.len());

    for m in all_moves.moves() {
        println!("  Move: {:?} -> {:?} ({})", m.from(), m.to(), m.to_uci());
    }

    // Pawn on E4 CAN move to E5 (along the pin ray)
    let pawn_moves: Vec<_> = all_moves
        .moves()
        .iter()
        .filter(|m| m.from() == Square::E4)
        .collect();
    println!("Pawn moves from E4: {}", pawn_moves.len());

    // A pinned piece can move along the pin ray
    assert_eq!(pawn_moves.len(), 1, "Pawn can move along pin ray (E4->E5)");
    assert_eq!(pawn_moves[0].to(), Square::E5, "Pawn should move to E5");
}
