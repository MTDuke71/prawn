use prawn::board::{Board, Square};
use prawn::movegen::MoveGenerator;

#[test]
fn debug_knight_surrounded() {
    let board = Board::from_fen("8/8/2PPP3/2PNP3/2PPP3/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    
    println!("FEN: {}", board.to_fen());
    
    let moves = movegen.generate_legal_moves(&board);
    println!("Total legal moves: {}", moves.len());
    
    for m in moves.moves() {
        println!("  {} (from {:?})", m.to_uci(), m.from());
    }
    
    // Knight surrounded by own pawns cannot move
    let knight_moves: Vec<_> = moves.moves().iter()
        .filter(|m| m.from() == Square::D5)
        .collect();
    println!("\nKnight moves from D5: {}", knight_moves.len());
    for m in knight_moves {
        println!("  {}", m.to_uci());
    }
}
