use prawn::board::{Board, Color, Square};
use prawn::board_ext::BoardExt;
use prawn::movegen::MoveGenerator;
use prawn::moves::Move;

#[test]
fn debug_move_execution() {
    let mut board = Board::from_fen("4r3/8/8/8/4P3/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    
    println!("Before move:");
    println!("  Side to move: {:?}", board.side_to_move());
    println!("  King in check: {}", movegen.is_in_check(&board, Color::White));
    
    // Make the pawn move E4-E5
    let m = Move::new_quiet(Square::E4, Square::E5);
    board.make_move_complete(m);
    
    println!("\nAfter E4-E5:");
    println!("  FEN: {}", board.to_fen());
    println!("  Side to move: {:?}", board.side_to_move());
    println!("  White king in check: {}", movegen.is_in_check(&board, Color::White));
    println!("  Black king in check: {}", movegen.is_in_check(&board, Color::Black));
    
    // The pawn moved along the pin ray (E4->E5), so it still blocks the rook
    // White king should NOT be in check
    assert!(!movegen.is_in_check(&board, Color::White), "White king should NOT be in check (pawn still blocks)");
}
