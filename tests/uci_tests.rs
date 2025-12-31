//! UCI Protocol Tests - Mission 4
//!
//! Tests for Basic UCI / Debug Shell implementation

use prawn::UciHandler;

// =============================================================================
// REQ-1: UCI Handshake Tests
// =============================================================================

#[test]
fn req1_uci_handshake() {
    let mut handler = UciHandler::new();
    let output = handler.process_command("uci");

    assert!(output.contains("id name"), "Should contain engine name");
    assert!(output.contains("id author"), "Should contain author");
    assert!(output.contains("uciok"), "Should end with uciok");
}

#[test]
fn req1_isready_readyok() {
    let mut handler = UciHandler::new();
    let output = handler.process_command("isready");

    assert_eq!(output.trim(), "readyok");
}

#[test]
fn req1_uci_contains_engine_name() {
    let mut handler = UciHandler::new();
    let output = handler.process_command("uci");

    assert!(output.contains("prawn"), "Engine name should be 'prawn'");
}

// =============================================================================
// REQ-2: Position Setup Tests
// =============================================================================

#[test]
fn req2_position_startpos() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");

    let fen = handler.current_fen();
    assert_eq!(
        fen,
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
}

#[test]
fn req2_position_startpos_with_moves() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos moves e2e4");

    let fen = handler.current_fen();
    // After 1. e4
    assert!(fen.contains("PPPP1PPP"), "e-pawn should have moved");
    assert!(fen.contains("4P3"), "e4 should have a pawn");
    assert!(fen.contains(" b "), "Side to move should be black");
}

#[test]
fn req2_position_startpos_with_multiple_moves() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos moves e2e4 e7e5");

    let fen = handler.current_fen();
    assert!(
        fen.contains(" w "),
        "Side to move should be white after 2 moves"
    );
}

#[test]
fn req2_position_fen() {
    let mut handler = UciHandler::new();
    let kiwipete = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    handler.process_command(&format!("position fen {}", kiwipete));

    let fen = handler.current_fen();
    assert!(
        fen.starts_with("r3k2r/p1ppqpb1"),
        "FEN should match kiwipete"
    );
}

#[test]
fn req2_position_fen_with_moves() {
    let mut handler = UciHandler::new();
    handler.process_command(
        "position fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 moves e7e5",
    );

    let fen = handler.current_fen();
    assert!(
        fen.contains(" w "),
        "Should be white's turn after black moves"
    );
}

// =============================================================================
// REQ-3: Position Display Tests
// =============================================================================

#[test]
fn req3_display_command() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("d");

    // Should contain board representation
    assert!(
        output.contains("r") && output.contains("R"),
        "Should show pieces"
    );
    assert!(output.contains("FEN:"), "Should show FEN");
}

#[test]
fn req3_display_shows_fen() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("d");

    assert!(
        output.contains("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
        "Should contain starting FEN"
    );
}

#[test]
fn req3_display_shows_side_to_move() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("d");

    // Should indicate white to move
    assert!(
        output.to_lowercase().contains("white") || output.contains("w"),
        "Should show side to move"
    );
}

// =============================================================================
// REQ-4: Perft Command Tests
// =============================================================================

#[test]
fn req4_perft_command_depth_1() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("go perft 1");

    // Depth 1 from startpos = 20 moves
    assert!(output.contains("20"), "Perft 1 should be 20 nodes");
}

#[test]
fn req4_perft_startpos_depth_2() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("go perft 2");

    // Depth 2 from startpos = 400 nodes
    assert!(output.contains("400"), "Perft 2 should be 400 nodes");
}

#[test]
fn req4_perft_shows_divide() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("go perft 1");

    // Should show per-move breakdown
    assert!(
        output.contains("a2a3") || output.contains("e2e4"),
        "Should show divide output with moves"
    );
}

#[test]
fn req4_perft_shows_total() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("go perft 2");

    // Should clearly indicate total
    assert!(
        output.to_lowercase().contains("nodes") || output.to_lowercase().contains("total"),
        "Should show total node count"
    );
}

// =============================================================================
// REQ-5: Basic Go Command Tests
// =============================================================================

#[test]
fn req5_go_returns_bestmove() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("go");

    assert!(output.starts_with("bestmove"), "Should return bestmove");
}

#[test]
fn req5_bestmove_format() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("go");

    // Format: "bestmove e2e4" or similar
    let parts: Vec<&str> = output.split_whitespace().collect();
    assert_eq!(parts[0], "bestmove");
    assert!(parts.len() >= 2, "Should have a move after bestmove");

    let mv = parts[1];
    assert!(
        mv.len() >= 4,
        "Move should be at least 4 chars (e.g., e2e4)"
    );
}

#[test]
fn req5_go_returns_legal_move() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    let output = handler.process_command("go");

    let parts: Vec<&str> = output.split_whitespace().collect();
    let mv = parts[1];

    // The move should be one of the legal starting moves
    let legal_starting_moves = [
        "a2a3", "a2a4", "b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f2f3",
        "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "b1a3", "b1c3", "g1f3", "g1h3",
    ];

    assert!(
        legal_starting_moves.contains(&mv),
        "Move {} should be legal from startpos",
        mv
    );
}

#[test]
fn req5_go_from_different_position() {
    let mut handler = UciHandler::new();
    // Position after 1. e4 e5
    handler.process_command("position startpos moves e2e4 e7e5");
    let output = handler.process_command("go");

    assert!(output.starts_with("bestmove"), "Should return bestmove");

    let parts: Vec<&str> = output.split_whitespace().collect();
    let mv = parts[1];

    // Should not be black's moves
    assert!(!mv.starts_with("e5"), "Should be white's move, not black's");
}

// =============================================================================
// REQ-6: Quit Command Tests
// =============================================================================

#[test]
fn req6_quit_command() {
    let mut handler = UciHandler::new();
    let should_quit = handler.process_command("quit");

    // The quit command should signal termination
    // Implementation can return empty string or special marker
    assert!(handler.should_quit(), "Handler should signal quit");
}

#[test]
fn req6_quit_is_clean() {
    let mut handler = UciHandler::new();
    handler.process_command("position startpos");
    handler.process_command("go perft 1");
    handler.process_command("quit");

    // Should not panic or error - test passes if we get here
    assert!(handler.should_quit());
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn integration_typical_session() {
    let mut handler = UciHandler::new();

    // Typical GUI startup sequence
    let output = handler.process_command("uci");
    assert!(output.contains("uciok"));

    let output = handler.process_command("isready");
    assert_eq!(output.trim(), "readyok");

    handler.process_command("position startpos");
    handler.process_command("position startpos moves e2e4 e7e5 g1f3");

    let output = handler.process_command("go");
    assert!(output.starts_with("bestmove"));
}

#[test]
fn integration_unknown_command_ignored() {
    let mut handler = UciHandler::new();

    // Unknown commands should be silently ignored (UCI spec)
    let output = handler.process_command("foobar");
    assert!(output.is_empty() || output.trim().is_empty());
}

#[test]
fn integration_ucinewgame() {
    let mut handler = UciHandler::new();

    // ucinewgame should reset state
    handler.process_command("position startpos moves e2e4");
    handler.process_command("ucinewgame");
    handler.process_command("position startpos");

    let fen = handler.current_fen();
    assert_eq!(
        fen,
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );
}
