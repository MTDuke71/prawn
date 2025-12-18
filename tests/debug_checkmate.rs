use prawn::board::{Board, Color};
use prawn::movegen::MoveGenerator;

#[test]
fn debug_checkmate() {
    // Back rank mate
    let board = Board::from_fen("6k1/5ppp/8/8/8/8/8/4R2K b - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    
    println!("FEN: {}", board.to_fen());
    println!("Black in check: {}", movegen.is_in_check(&board, Color::Black));
    
    let moves = movegen.generate_legal_moves(&board);
    println!("Black legal moves: {}", moves.len());
    for m in moves.moves() {
        println!("  {}", m.to_uci());
    }
    
    println!("Is checkmate: {}", movegen.is_checkmate(&board));
}
