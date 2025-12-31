use mission2_movegen::board::{Board, Color};
use mission2_movegen::movegen::MoveGenerator;

#[test]
fn find_actual_checkmate() {
    let movegen = MoveGenerator::new();

    // Try: King on g8, rook giving check from g1, pawns blocking escape
    let board = Board::from_fen("6k1/5ppp/8/8/8/8/8/6RK b - - 0 1").unwrap();
    println!("Position 1: {}", board.to_fen());
    println!(
        "  Black in check: {}",
        movegen.is_in_check(&board, Color::Black)
    );
    let moves = movegen.generate_legal_moves(&board);
    println!("  Black moves: {}", moves.len());
    for m in moves.moves() {
        println!("    {}", m.to_uci());
    }
    println!("  Is checkmate: {}", movegen.is_checkmate(&board));
    println!();

    // Try a real back rank mate
    let board2 = Board::from_fen("6k1/8/6K1/8/8/8/8/6R1 b - - 0 1").unwrap();
    println!("Position 2: {}", board2.to_fen());
    println!(
        "  Black in check: {}",
        movegen.is_in_check(&board2, Color::Black)
    );
    let moves2 = movegen.generate_legal_moves(&board2);
    println!("  Black moves: {}", moves2.len());
    for m in moves2.moves() {
        println!("    {}", m.to_uci());
    }
    println!("  Is checkmate: {}", movegen.is_checkmate(&board2));
}
