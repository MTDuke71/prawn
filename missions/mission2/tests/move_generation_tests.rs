// Comprehensive move generation tests
// Tests for REQ-1 through REQ-11

use mission2_movegen::board::{Board, Color, Piece, PieceType, Square};
use mission2_movegen::movegen::MoveGenerator;
use mission2_movegen::moves::Move;

// ============================================================================
// REQ-1: Pawn Move Generation Tests
// ============================================================================

#[test]
fn req1_pawn_single_move() {
    let board = Board::from_fen("8/8/8/8/8/8/4P3/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // White pawn on E2 can move to E3
    assert!(moves.moves().iter().any(|m| m.from() == Square::E2 && m.to() == Square::E3));
}

#[test]
fn req1_pawn_double_move() {
    let board = Board::from_fen("8/8/8/8/8/8/4P3/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // White pawn on E2 can move to E4 (double push)
    assert!(moves.moves().iter().any(|m| m.from() == Square::E2 && m.to() == Square::E4));
    assert_eq!(moves.len(), 2); // E3 and E4
}

#[test]
fn req1_pawn_capture() {
    let board = Board::from_fen("8/8/8/8/3p1p2/4P3/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // White pawn on E3 can capture on D4 and F4
    let captures: Vec<&Move> = moves.moves().iter().filter(|m| m.is_capture()).collect();
    assert_eq!(captures.len(), 2);
}

#[test]
fn req1_pawn_blocked() {
    let board = Board::from_fen("8/8/8/8/8/4p3/4P3/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // White pawn on E2 is blocked by pawn on E3
    assert_eq!(moves.len(), 0);
}

#[test]
fn req1_pawn_promotion() {
    let board = Board::from_fen("8/4P3/8/8/8/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Pawn on E7 can promote to 4 pieces
    assert_eq!(moves.len(), 4); // Q, R, B, N
    assert!(moves.moves().iter().all(|m| m.is_promotion()));
}

#[test]
fn req1_en_passant() {
    // Position where en passant is possible
    let board = Board::from_fen("8/8/8/3Pp3/8/8/8/8 w - e6 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Should have en passant capture
    let ep_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| matches!(m.move_type(), mission2_movegen::moves::MoveType::EnPassant))
        .collect();
    assert!(ep_moves.len() > 0);
}

// ============================================================================
// REQ-2: Knight Move Generation Tests
// ============================================================================

#[test]
fn req2_knight_moves() {
    let board = Board::from_fen("8/8/8/8/4N3/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Knight on E4 has 8 possible moves
    assert_eq!(moves.len(), 8);
}

#[test]
fn req2_knight_edge_cases() {
    let board = Board::from_fen("N7/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Knight in corner has only 2 moves
    assert_eq!(moves.len(), 2);
}

#[test]
fn req2_knight_blocked_by_own_pieces() {
    let board = Board::from_fen("8/8/2PPP3/2PNP3/2PPP3/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Knight surrounded by own pawns cannot move
    let knight_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.from() == Square::D5)
        .collect();
    assert_eq!(knight_moves.len(), 0);
}

// ============================================================================
// REQ-3: Bishop Move Generation Tests
// ============================================================================

#[test]
fn req3_bishop_moves() {
    let board = Board::from_fen("8/8/8/8/4B3/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Bishop on E4 on empty board has 13 moves
    assert_eq!(moves.len(), 13);
}

#[test]
fn req3_bishop_blocked() {
    let board = Board::from_fen("8/8/6P1/8/4B3/8/2P5/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Bishop blocked by own pawns
    let bishop_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.from() == Square::E4)
        .collect();
    // Should have fewer than 13 moves
    assert!(bishop_moves.len() < 13);
}

#[test]
fn req3_bishop_captures() {
    let board = Board::from_fen("8/8/6p1/8/4B3/8/2p5/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Bishop can capture enemy pawns
    let captures: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.is_capture() && m.from() == Square::E4)
        .collect();
    assert_eq!(captures.len(), 2); // C2 and G6
}

// ============================================================================
// REQ-4: Rook Move Generation Tests
// ============================================================================

#[test]
fn req4_rook_moves() {
    let board = Board::from_fen("8/8/8/8/4R3/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Rook on E4 on empty board has 14 moves
    assert_eq!(moves.len(), 14);
}

#[test]
fn req4_rook_blocked() {
    let board = Board::from_fen("8/8/4P3/8/P3R2P/8/4P3/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Rook blocked by own pawns
    let rook_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.from() == Square::E4)
        .collect();
    // Can only move to F4, G4 (east) and B4, C4, D4 (west)
    assert!(rook_moves.len() < 14);
}

#[test]
fn req4_rook_captures() {
    let board = Board::from_fen("8/8/4p3/8/p3R2p/8/4p3/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Rook can capture in all 4 directions
    let captures: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.is_capture() && m.from() == Square::E4)
        .collect();
    assert_eq!(captures.len(), 4);
}

// ============================================================================
// REQ-5: Queen Move Generation Tests
// ============================================================================

#[test]
fn req5_queen_moves() {
    let board = Board::from_fen("8/8/8/8/4Q3/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Queen on E4 on empty board has 27 moves (14 rook + 13 bishop)
    assert_eq!(moves.len(), 27);
}

#[test]
fn req5_queen_all_directions() {
    let board = Board::from_fen("8/8/8/8/4Q3/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Verify queen can move in all 8 directions
    let queen_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.from() == Square::E4)
        .collect();
    assert_eq!(queen_moves.len(), 27);
}

// ============================================================================
// REQ-6: King Move Generation Tests
// ============================================================================

#[test]
fn req6_king_moves() {
    let board = Board::from_fen("8/8/8/8/4K3/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // King on E4 has 8 adjacent squares
    assert_eq!(moves.len(), 8);
}

#[test]
fn req6_king_edge_cases() {
    let board = Board::from_fen("K7/8/8/8/8/8/8/8 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // King in corner has only 3 moves
    assert_eq!(moves.len(), 3);
}

// ============================================================================
// REQ-7: Castling Tests
// ============================================================================

#[test]
fn req7_kingside_castle() {
    let board = Board::from_fen("8/8/8/8/8/8/8/4K2R w K - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Should be able to castle kingside
    let castle_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.is_castle())
        .collect();
    assert!(castle_moves.len() > 0);
}

#[test]
fn req7_queenside_castle() {
    let board = Board::from_fen("8/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Should be able to castle queenside
    let castle_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.is_castle())
        .collect();
    assert!(castle_moves.len() > 0);
}

#[test]
fn req7_castle_blocked() {
    let board = Board::from_fen("8/8/8/8/8/8/8/4KN1R w K - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Cannot castle - knight in the way
    let castle_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.is_castle())
        .collect();
    assert_eq!(castle_moves.len(), 0);
}

#[test]
fn req7_castle_through_check() {
    // King would move through check
    let board = Board::from_fen("8/8/8/8/8/5r2/8/4K2R w K - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Cannot castle - would move through check on F1
    let castle_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.is_castle())
        .collect();
    assert_eq!(castle_moves.len(), 0);
}

// ============================================================================
// REQ-8: Move Validation (No Self-Check) Tests
// ============================================================================

#[test]
fn req8_illegal_move_into_check() {
    // King on E1, enemy rook on E8
    let board = Board::from_fen("4r3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // King cannot move to D1, E2, F1 (still in check from rook)
    // Should have limited moves
    assert!(moves.len() < 8);
}

#[test]
fn req8_pinned_piece() {
    // Pawn is pinned to king by rook
    let board = Board::from_fen("4r3/8/8/8/4P3/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();
    let moves = movegen.generate_legal_moves(&board);

    // Pawn on E4 cannot move (pinned)
    let pawn_moves: Vec<&Move> = moves.moves().iter()
        .filter(|m| m.from() == Square::E4)
        .collect();
    assert_eq!(pawn_moves.len(), 0);
}

// ============================================================================
// REQ-9: Check Detection Tests
// ============================================================================

#[test]
fn req9_check_detection() {
    let board = Board::from_fen("4r3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    // White king is in check from black rook
    assert!(movegen.is_in_check(&board, Color::White));
}

#[test]
fn req9_no_check() {
    let board = Board::from_fen("8/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    // King is not in check
    assert!(!movegen.is_in_check(&board, Color::White));
}

// ============================================================================
// REQ-10: Checkmate Detection Tests
// ============================================================================

#[test]
fn req10_checkmate_detected() {
    // Back rank mate
    let board = Board::from_fen("6k1/5ppp/8/8/8/8/8/4R2K b - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    // Black is in checkmate
    assert!(movegen.is_checkmate(&board));
}

#[test]
fn req10_scholars_mate() {
    // Scholar's mate position
    let board = Board::from_fen("r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    // Black is in checkmate
    assert!(movegen.is_checkmate(&board));
}

#[test]
fn req10_not_checkmate_can_block() {
    // King in check but can block
    let board = Board::from_fen("4r3/8/8/8/8/8/4B3/4K3 w - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    // Not checkmate - can block with bishop
    assert!(!movegen.is_checkmate(&board));
}

// ============================================================================
// REQ-11: Stalemate Detection Tests
// ============================================================================

#[test]
fn req11_stalemate_detected() {
    // Classic stalemate position
    let board = Board::from_fen("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    // Black is in stalemate
    assert!(movegen.is_stalemate(&board));
}

#[test]
fn req11_not_stalemate_has_moves() {
    let board = Board::from_fen("7k/8/6K1/8/8/8/8/8 b - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    // Not stalemate - king has moves
    assert!(!movegen.is_stalemate(&board));
}

#[test]
fn req11_not_stalemate_in_check() {
    let board = Board::from_fen("6k1/5ppp/8/8/8/8/8/4R2K b - - 0 1").unwrap();
    let movegen = MoveGenerator::new();

    // Not stalemate - is checkmate (in check)
    assert!(!movegen.is_stalemate(&board));
}
