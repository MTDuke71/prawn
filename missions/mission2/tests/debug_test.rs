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
    
    // Pawn on E4 should not be able to move (pinned)
    let pawn_moves: Vec<_> = all_moves.moves().iter()
        .filter(|m| m.from() == Square::E4)
        .collect();
    println!("Pawn moves from E4: {}", pawn_moves.len());
    
    assert_eq!(pawn_moves.len(), 0, "Pawn should be pinned and have no moves");
}
