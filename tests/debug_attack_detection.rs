use prawn::board::{Board, Color, Square, PieceType};
use prawn::movegen::MoveGenerator;

#[test]
fn debug_rook_attack_detection() {
    // King on E1, Rook on E8, nothing in between
    let board = Board::from_fen("4r3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    
    println!("FEN: {}", board.to_fen());
    println!("All occupancy: {:064b}", board.all_occupancy());
    println!("Rook bitboard: {:064b}", board.piece_bitboard(Color::Black, PieceType::Rook));
    
    // Check if E1 is attacked by black
    let e1_attacked = movegen.is_square_attacked(&board, Square::E1, Color::Black);
    println!("\nE1 attacked by black: {}", e1_attacked);
    
    // Check what squares the rook on E8 can attack
    println!("\nManually checking rook on E8:");
    let rook_square = Square::E8;
    println!("Rook square index: {}", rook_square.index());
    
    // Get the magic table directly
    use prawn::magic::MagicTable;
    let magic = MagicTable::new();
    let rook_attacks = magic.rook_attacks(rook_square, board.all_occupancy());
    println!("Rook attacks from E8: {:064b}", rook_attacks);
    println!("E1 index: {}, bit: {}", Square::E1.index(), 1u64 << Square::E1.index());
    println!("Rook attacks E1: {}", (rook_attacks & (1u64 << Square::E1.index())) != 0);
    
    assert!(e1_attacked, "E1 should be attacked by rook on E8");
}

#[test]
fn debug_rook_attack_with_pawn() {
    // King on E1, Rook on E8, Pawn on E5 (blocking)
    let board = Board::from_fen("4r3/8/8/4P3/8/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    
    println!("FEN: {}", board.to_fen());
    println!("All occupancy: {:064b}", board.all_occupancy());
    
    // Check if E1 is attacked by black
    let e1_attacked = movegen.is_square_attacked(&board, Square::E1, Color::Black);
    println!("\nE1 attacked by black (should be false): {}", e1_attacked);
    
    // Get the magic table directly
    use prawn::magic::MagicTable;
    let magic = MagicTable::new();
    let rook_attacks = magic.rook_attacks(Square::E8, board.all_occupancy());
    println!("Rook attacks from E8: {:064b}", rook_attacks);
    println!("E1 bit: {:064b}", 1u64 << Square::E1.index());
    println!("E5 bit: {:064b}", 1u64 << Square::E5.index());
    println!("Rook attacks E1: {}", (rook_attacks & (1u64 << Square::E1.index())) != 0);
    
    assert!(!e1_attacked, "E1 should NOT be attacked (pawn blocks)");
}

#[test]
fn debug_after_pawn_moves() {
    // Start with pawn blocking
    let mut board = Board::from_fen("4r3/8/8/8/4P3/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    
    println!("Initial FEN: {}", board.to_fen());
    println!("Initial occupancy: {:064b}", board.all_occupancy());
    
    // Move pawn E4 to E5
    use prawn::board_ext::BoardExt;
    use prawn::moves::Move;
    let m = Move::new_quiet(Square::E4, Square::E5);
    board.make_move_complete(m);
    
    println!("\nAfter E4-E5:");
    println!("FEN: {}", board.to_fen());
    println!("New occupancy: {:064b}", board.all_occupancy());
    println!("Side to move: {:?}", board.side_to_move());
    
    // Now check if white king is in check
    let white_in_check = movegen.is_in_check(&board, Color::White);
    println!("\nWhite king in check: {}", white_in_check);
    
    // Check attack detection directly
    let e1_attacked = movegen.is_square_attacked(&board, Square::E1, Color::Black);
    println!("E1 attacked by black: {}", e1_attacked);
    
    // Get rook attacks manually
    use prawn::magic::MagicTable;
    let magic = MagicTable::new();
    let rook_attacks = magic.rook_attacks(Square::E8, board.all_occupancy());
    println!("\nRook attacks from E8: {:064b}", rook_attacks);
    println!("E1 bit:                {:064b}", 1u64 << Square::E1.index());
    println!("E5 bit:                {:064b}", 1u64 << Square::E5.index());
    println!("Rook attacks E1: {}", (rook_attacks & (1u64 << Square::E1.index())) != 0);
    
    // The pawn moved from E4 to E5, which means it's STILL on the E-file
    // and STILL blocking the rook from attacking E1. So white king is NOT in check.
    assert!(!white_in_check, "White king should NOT be in check (pawn still blocks on E5)");
}
