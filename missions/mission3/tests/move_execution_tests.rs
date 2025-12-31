// Mission 3: Move Execution - Integration Tests
// Following TDD methodology: Write tests BEFORE implementation

use mission2_movegen::Move;
use mission3_move_execution::{GameState, ZobristHasher};
use prawn::board::{Board, Color, Piece, PieceType, Square};

// ============================================================================
// REQ-1: Make Move (Update Board State)
// ============================================================================

#[test]
fn req1_make_move_updates_board() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board.clone(), zobrist);

    // White plays e2-e4
    let e2_e4 = Move::new_quiet(Square::E2, Square::E4);
    game.make_move(e2_e4);

    // Verify piece moved
    assert_eq!(game.board().piece_at(Square::E2), None);
    assert_eq!(
        game.board().piece_at(Square::E4),
        Some(Piece::from_type_and_color(PieceType::Pawn, Color::White))
    );

    // Verify side to move switched
    assert_eq!(game.board().side_to_move(), Color::Black);
}

#[test]
fn req1_make_move_capture() {
    // Start with a position where White can capture on e5
    let zobrist = ZobristHasher::new();

    let d4 = Square::D4;
    let e5 = Square::E5;

    // Position where it's White's turn and d4 pawn can capture e5
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 3")
        .expect("Valid FEN");
    let mut game = GameState::new(board, zobrist);

    let capture_move = Move::new_capture(
        d4,
        e5,
        Piece::from_type_and_color(PieceType::Pawn, Color::Black),
    );

    game.make_move(capture_move);

    // Verify captured piece is gone
    assert_eq!(game.board().piece_at(d4), None);
    assert_eq!(
        game.board().piece_at(e5),
        Some(Piece::from_type_and_color(PieceType::Pawn, Color::White))
    );
}

#[test]
fn req1_make_move_castling() {
    // Position where White can castle kingside
    let board =
        Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").expect("Valid FEN");
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    // Castle kingside
    let castle_move = Move::new_kingside_castle(Square::E1, Square::G1);
    game.make_move(castle_move);

    // Verify king and rook moved
    assert_eq!(game.board().piece_at(Square::E1), None);
    assert_eq!(game.board().piece_at(Square::H1), None);
    assert_eq!(
        game.board().piece_at(Square::G1),
        Some(Piece::from_type_and_color(PieceType::King, Color::White))
    );
    assert_eq!(
        game.board().piece_at(Square::F1),
        Some(Piece::from_type_and_color(PieceType::Rook, Color::White))
    );

    // Verify castling rights removed for White
    assert!(!game.board().can_castle_kingside(Color::White));
    assert!(!game.board().can_castle_queenside(Color::White));
}

#[test]
fn req1_make_move_en_passant() {
    // Position where en passant is possible
    // White pawn on e5, Black pawn moves d7-d5 (double push)
    let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2")
        .expect("Valid FEN");
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    // Capture en passant
    let ep_move = Move::new_en_passant(Square::E5, Square::D6);
    game.make_move(ep_move);

    // Verify pawn moved and captured pawn removed
    assert_eq!(game.board().piece_at(Square::E5), None);
    assert_eq!(game.board().piece_at(Square::D5), None); // Captured pawn
    assert_eq!(
        game.board().piece_at(Square::D6),
        Some(Piece::from_type_and_color(PieceType::Pawn, Color::White))
    );
}

#[test]
fn req1_make_move_promotion() {
    // White pawn on 7th rank ready to promote
    let board = Board::from_fen("8/4P3/8/8/8/8/8/4K2k w - - 0 1").expect("Valid FEN");
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    // Promote to queen
    let promotion = Move::new_promotion(Square::E7, Square::E8, PieceType::Queen);
    game.make_move(promotion);

    // Verify pawn is now a queen
    assert_eq!(game.board().piece_at(Square::E7), None);
    assert_eq!(
        game.board().piece_at(Square::E8),
        Some(Piece::from_type_and_color(PieceType::Queen, Color::White))
    );
}

// ============================================================================
// REQ-2: Unmake Move (Restore Previous State)
// ============================================================================

#[test]
fn req2_unmake_move_restores_state() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board.clone(), zobrist);

    // Make a move
    let e2_e4 = Move::new_quiet(Square::E2, Square::E4);
    game.make_move(e2_e4);

    // Unmake the move
    game.unmake_move();

    // Verify board is back to starting position
    assert_eq!(
        game.board().piece_at(Square::E2),
        Some(Piece::from_type_and_color(PieceType::Pawn, Color::White))
    );
    assert_eq!(game.board().piece_at(Square::E4), None);
    assert_eq!(game.board().side_to_move(), Color::White);
}

#[test]
fn req2_unmake_move_capture() {
    // Position with a capture possible
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 3")
        .expect("Valid FEN");
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    let original_hash = game.zobrist_hash();

    // Capture
    let capture = Move::new_capture(
        Square::D4,
        Square::E5,
        Piece::from_type_and_color(PieceType::Pawn, Color::Black),
    );
    game.make_move(capture);

    // Unmake
    game.unmake_move();

    // Verify captured piece is restored
    assert_eq!(
        game.board().piece_at(Square::D4),
        Some(Piece::from_type_and_color(PieceType::Pawn, Color::White))
    );
    assert_eq!(
        game.board().piece_at(Square::E5),
        Some(Piece::from_type_and_color(PieceType::Pawn, Color::Black))
    );
    assert_eq!(game.zobrist_hash(), original_hash);
}

#[test]
fn req2_unmake_move_castling() {
    let board =
        Board::from_fen("r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1").expect("Valid FEN");
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    let original_hash = game.zobrist_hash();

    // Castle kingside
    let castle = Move::new_kingside_castle(Square::E1, Square::G1);
    game.make_move(castle);

    // Unmake
    game.unmake_move();

    // Verify king and rook are back
    assert_eq!(
        game.board().piece_at(Square::E1),
        Some(Piece::from_type_and_color(PieceType::King, Color::White))
    );
    assert_eq!(
        game.board().piece_at(Square::H1),
        Some(Piece::from_type_and_color(PieceType::Rook, Color::White))
    );
    assert_eq!(game.board().piece_at(Square::G1), None);
    assert_eq!(game.board().piece_at(Square::F1), None);

    // Verify castling rights restored
    assert!(game.board().can_castle_kingside(Color::White));
    assert_eq!(game.zobrist_hash(), original_hash);
}

#[test]
fn req2_unmake_move_en_passant() {
    let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2")
        .expect("Valid FEN");
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    let original_hash = game.zobrist_hash();

    // En passant capture
    let ep = Move::new_en_passant(Square::E5, Square::D6);
    game.make_move(ep);

    // Unmake
    game.unmake_move();

    // Verify both pawns are back
    assert_eq!(
        game.board().piece_at(Square::E5),
        Some(Piece::from_type_and_color(PieceType::Pawn, Color::White))
    );
    assert_eq!(
        game.board().piece_at(Square::D5),
        Some(Piece::from_type_and_color(PieceType::Pawn, Color::Black))
    );
    assert_eq!(game.board().piece_at(Square::D6), None);
    assert_eq!(game.board().en_passant_square(), Some(Square::D6));
    assert_eq!(game.zobrist_hash(), original_hash);
}

#[test]
fn req2_make_unmake_sequence() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    let original_hash = game.zobrist_hash();

    // Make several moves
    let moves = vec![
        Move::new_quiet(Square::E2, Square::E4),
        Move::new_quiet(Square::E7, Square::E5),
        Move::new_quiet(Square::G1, Square::F3),
    ];

    for m in &moves {
        game.make_move(*m);
    }

    // Unmake all moves
    for _ in 0..moves.len() {
        game.unmake_move();
    }

    // Verify back to starting position
    assert_eq!(game.zobrist_hash(), original_hash);
    assert_eq!(game.board().side_to_move(), Color::White);
}

// ============================================================================
// REQ-3: Move History Tracking
// ============================================================================

#[test]
fn req3_move_history_tracking() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    assert_eq!(game.move_history().len(), 0);

    // Make some moves
    game.make_move(Move::new_quiet(Square::E2, Square::E4));
    assert_eq!(game.move_history().len(), 1);

    game.make_move(Move::new_quiet(Square::E7, Square::E5));
    assert_eq!(game.move_history().len(), 2);

    // Unmake a move
    game.unmake_move();
    assert_eq!(game.move_history().len(), 1);
}

#[test]
fn req3_move_history_depth() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    // Make many moves
    for _ in 0..100 {
        game.make_move(Move::new_quiet(Square::E2, Square::E4));
        game.unmake_move();
    }

    // History should handle arbitrary depth
    assert_eq!(game.move_history().len(), 0);
}

#[test]
fn req3_restore_from_history() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    let moves = vec![
        Move::new_quiet(Square::E2, Square::E4),
        Move::new_quiet(Square::E7, Square::E5),
        Move::new_quiet(Square::G1, Square::F3),
    ];

    for m in &moves {
        game.make_move(*m);
    }

    // Get history
    let history = game.move_history();
    assert_eq!(history.len(), 3);
    assert_eq!(history[0], moves[0]);
    assert_eq!(history[1], moves[1]);
    assert_eq!(history[2], moves[2]);
}

// ============================================================================
// REQ-4: Halfmove Clock (50-Move Rule)
// ============================================================================

#[test]
fn req4_halfmove_clock_increment() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    assert_eq!(game.board().halfmove_clock(), 0);

    // Non-pawn, non-capture move
    game.make_move(Move::new_quiet(Square::G1, Square::F3));
    assert_eq!(game.board().halfmove_clock(), 1);

    game.make_move(Move::new_quiet(Square::G8, Square::F6));
    assert_eq!(game.board().halfmove_clock(), 2);
}

#[test]
fn req4_halfmove_clock_reset_pawn() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    // Pawn move
    game.make_move(Move::new_quiet(Square::E2, Square::E4));
    assert_eq!(
        game.board().halfmove_clock(),
        0,
        "Halfmove clock should reset on pawn move"
    );
}

#[test]
fn req4_halfmove_clock_reset_capture() {
    // Position with capture available
    let board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 5 3")
        .expect("Valid FEN");
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    assert_eq!(game.board().halfmove_clock(), 5);

    // Capture
    let capture = Move::new_capture(
        Square::D4,
        Square::E5,
        Piece::from_type_and_color(PieceType::Pawn, Color::Black),
    );
    game.make_move(capture);

    assert_eq!(
        game.board().halfmove_clock(),
        0,
        "Halfmove clock should reset on capture"
    );
}

#[test]
fn req4_fifty_move_rule() {
    let board = Board::from_fen("4k3/8/8/8/8/8/8/4K3 w - - 99 100").expect("Valid FEN");
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    assert_eq!(game.board().halfmove_clock(), 99);

    // One more move reaches 50-move rule
    game.make_move(Move::new_quiet(Square::E1, Square::E2));
    assert_eq!(game.board().halfmove_clock(), 100);

    // Can be claimed as draw
    assert!(game.is_fifty_move_rule());
}

// ============================================================================
// REQ-5: Fullmove Counter
// ============================================================================

#[test]
fn req5_fullmove_counter_starts_one() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let game = GameState::new(board, zobrist);

    assert_eq!(game.board().fullmove_number(), 1);
}

#[test]
fn req5_fullmove_counter_increment() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    // White's move - fullmove should not increment
    game.make_move(Move::new_quiet(Square::E2, Square::E4));
    assert_eq!(game.board().fullmove_number(), 1);

    // Black's move - fullmove should increment
    game.make_move(Move::new_quiet(Square::E7, Square::E5));
    assert_eq!(game.board().fullmove_number(), 2);

    // White's move - fullmove should not increment
    game.make_move(Move::new_quiet(Square::G1, Square::F3));
    assert_eq!(game.board().fullmove_number(), 2);

    // Black's move - fullmove should increment
    game.make_move(Move::new_quiet(Square::G8, Square::F6));
    assert_eq!(game.board().fullmove_number(), 3);
}

#[test]
fn req5_fullmove_counter_unmake() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    game.make_move(Move::new_quiet(Square::E2, Square::E4));
    game.make_move(Move::new_quiet(Square::E7, Square::E5));
    assert_eq!(game.board().fullmove_number(), 2);

    // Unmake Black's move
    game.unmake_move();
    assert_eq!(game.board().fullmove_number(), 1);

    // Unmake White's move
    game.unmake_move();
    assert_eq!(game.board().fullmove_number(), 1);
}

// ============================================================================
// REQ-6: Zobrist Hashing for Position Identification
// ============================================================================

#[test]
fn req6_zobrist_hash_uniqueness() {
    let zobrist = ZobristHasher::new();

    let board1 = Board::starting_position();
    let board2 = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
        .expect("Valid FEN");

    let hash1 = zobrist.hash_board(&board1);
    let hash2 = zobrist.hash_board(&board2);

    assert_ne!(
        hash1, hash2,
        "Different positions should have different hashes"
    );
}

#[test]
fn req6_zobrist_incremental_update() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board.clone(), zobrist.clone());

    // Hash before move
    let hash_before = game.zobrist_hash();

    // Make move
    game.make_move(Move::new_quiet(Square::E2, Square::E4));

    // Incremental hash
    let incremental_hash = game.zobrist_hash();

    // Compute hash from scratch
    let scratch_hash = zobrist.hash_board(game.board());

    assert_eq!(
        incremental_hash, scratch_hash,
        "Incremental hash should match hash computed from scratch"
    );
    assert_ne!(
        hash_before, incremental_hash,
        "Hash should change after move"
    );
}

#[test]
fn req6_zobrist_same_position_same_hash() {
    let zobrist = ZobristHasher::new();

    let board1 = Board::starting_position();
    let board2 = Board::starting_position();

    let hash1 = zobrist.hash_board(&board1);
    let hash2 = zobrist.hash_board(&board2);

    assert_eq!(hash1, hash2, "Same position should have same hash");
}

#[test]
fn req6_zobrist_different_position_different_hash() {
    let zobrist = ZobristHasher::new();

    // Different side to move
    let board1 = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("Valid FEN");
    let board2 = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1")
        .expect("Valid FEN");

    let hash1 = zobrist.hash_board(&board1);
    let hash2 = zobrist.hash_board(&board2);

    assert_ne!(
        hash1, hash2,
        "Different side to move should have different hash"
    );
}

// ============================================================================
// REQ-7: Threefold Repetition Detection
// ============================================================================

#[test]
fn req7_threefold_repetition() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    // Repeat a position 3 times (Nf3-Ng1 back and forth)
    let moves = vec![
        Move::new_quiet(Square::G1, Square::F3),
        Move::new_quiet(Square::G8, Square::F6),
        Move::new_quiet(Square::F3, Square::G1),
        Move::new_quiet(Square::F6, Square::G8),
        Move::new_quiet(Square::G1, Square::F3),
        Move::new_quiet(Square::G8, Square::F6),
        Move::new_quiet(Square::F3, Square::G1),
        Move::new_quiet(Square::F6, Square::G8),
    ];

    for m in moves {
        game.make_move(m);
    }

    // After these moves, we should be back at starting position for the 3rd time
    assert!(
        game.is_threefold_repetition(),
        "Should detect threefold repetition"
    );
}

#[test]
fn req7_threefold_non_consecutive() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    // First occurrence (starting position) - counted at initialization
    let hash1 = game.zobrist_hash();

    // Make moves to come back to starting position by playing Nf3 Nf6 Ng1 Ng8
    // This creates a cycle: start -> after Nf3 -> after Nf6 -> after Ng1 -> after Ng8 (=start, 2nd occurrence)
    game.make_move(Move::new_quiet(Square::G1, Square::F3));
    game.make_move(Move::new_quiet(Square::G8, Square::F6));
    game.make_move(Move::new_quiet(Square::F3, Square::G1));
    game.make_move(Move::new_quiet(Square::F6, Square::G8));

    let hash2 = game.zobrist_hash();
    assert_eq!(hash1, hash2, "Should be back to starting position");

    // Make different moves and come back again
    game.make_move(Move::new_quiet(Square::B1, Square::C3));
    game.make_move(Move::new_quiet(Square::B8, Square::C6));
    game.make_move(Move::new_quiet(Square::C3, Square::B1));
    game.make_move(Move::new_quiet(Square::C6, Square::B8));

    let hash3 = game.zobrist_hash();
    assert_eq!(hash1, hash3, "Should be back to starting position again");

    // Now we have 3 occurrences of the starting position
    assert!(
        game.is_threefold_repetition(),
        "Should detect non-consecutive repetition"
    );
}

#[test]
fn req7_no_false_positives() {
    let board = Board::starting_position();
    let zobrist = ZobristHasher::new();
    let mut game = GameState::new(board, zobrist);

    // Make some moves without repeating
    game.make_move(Move::new_quiet(Square::E2, Square::E4));
    game.make_move(Move::new_quiet(Square::E7, Square::E5));
    game.make_move(Move::new_quiet(Square::G1, Square::F3));

    assert!(
        !game.is_threefold_repetition(),
        "Should not detect repetition when there is none"
    );
}
