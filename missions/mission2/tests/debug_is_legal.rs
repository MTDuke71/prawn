use mission2_movegen::board::{Board, Color, PieceType, Square};
use mission2_movegen::board_ext::BoardExt;
use mission2_movegen::movegen::MoveGenerator;
use mission2_movegen::moves::Move;

#[test]
fn debug_is_legal_move() {
    // Pawn is pinned to king by rook
    let board = Board::from_fen("4r3/8/8/8/4P3/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    println!("Initial position:");
    println!("FEN: {}", board.to_fen());
    println!(
        "White in check: {}",
        movegen.is_in_check(&board, Color::White)
    );
    println!();

    // Test the pawn move E4->E5
    let m = Move::new_quiet(Square::E4, Square::E5);
    println!("Testing move E4->E5");

    // Clone board and make the move
    let mut test_board = board.clone();
    println!("Before make_move:");
    println!("  Pawn on E4: {:?}", test_board.piece_at(Square::E4));
    println!("  Pawn on E5: {:?}", test_board.piece_at(Square::E5));
    println!("  King on E1: {:?}", test_board.piece_at(Square::E1));
    println!("  Rook on E8: {:?}", test_board.piece_at(Square::E8));
    println!("  Occupancy: {:064b}", test_board.all_occupancy());

    test_board.make_move_complete(m);

    println!("\nAfter make_move:");
    println!("  FEN: {}", test_board.to_fen());
    println!("  Side to move: {:?}", test_board.side_to_move());
    println!("  Pawn on E4: {:?}", test_board.piece_at(Square::E4));
    println!("  Pawn on E5: {:?}", test_board.piece_at(Square::E5));
    println!("  King on E1: {:?}", test_board.piece_at(Square::E1));
    println!("  Rook on E8: {:?}", test_board.piece_at(Square::E8));
    println!("  Occupancy: {:064b}", test_board.all_occupancy());

    // Check if white king is in check
    let white_in_check = movegen.is_in_check(&test_board, Color::White);
    println!("\nWhite king in check after move: {}", white_in_check);

    // Check if E1 is attacked
    let e1_attacked = movegen.is_square_attacked(&test_board, Square::E1, Color::Black);
    println!("E1 attacked by black: {}", e1_attacked);

    // Get rook attacks
    use mission2_movegen::magic::MagicTable;
    let magic = MagicTable::new();
    let rook_attacks = magic.rook_attacks(Square::E8, test_board.all_occupancy());
    println!("\nRook attacks from E8:");
    println!("  Bitboard: {:064b}", rook_attacks);
    println!("  E1 bit:   {:064b}", 1u64 << Square::E1.index());
    println!("  E5 bit:   {:064b}", 1u64 << Square::E5.index());
    println!(
        "  Attacks E1: {}",
        (rook_attacks & (1u64 << Square::E1.index())) != 0
    );
    println!(
        "  Attacks E5: {}",
        (rook_attacks & (1u64 << Square::E5.index())) != 0
    );
}
