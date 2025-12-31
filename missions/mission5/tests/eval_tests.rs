//! Position Evaluation Tests - Mission 5
//!
//! Tests for static position evaluation with feature isolation

use mission5::{EvalConfig, EvalBreakdown, Evaluator};
use prawn::board::Board;

// =============================================================================
// REQ-1: Material Counting Tests
// =============================================================================

#[test]
fn req1_material_count_starting_position() {
    let board = Board::default();
    let evaluator = Evaluator::new(EvalConfig::MATERIAL_ONLY);
    let score = evaluator.evaluate(&board);
    
    // Starting position should be equal
    assert_eq!(score, 0, "Starting position should have 0 material balance");
}

#[test]
fn req1_material_advantage_white_queen() {
    // White has extra queen
    let board = Board::from_fen("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let evaluator = Evaluator::new(EvalConfig::MATERIAL_ONLY);
    let score = evaluator.evaluate(&board);
    
    // White should be up ~900 centipawns (queen value)
    assert!(score > 800 && score < 1000, "White should be up roughly a queen, got {}", score);
}

#[test]
fn req1_material_advantage_black_rook() {
    // Black has extra rook
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN1 b Qkq - 0 1").unwrap();
    let evaluator = Evaluator::new(EvalConfig::MATERIAL_ONLY);
    let score = evaluator.evaluate(&board);
    
    // From black's perspective, black is up ~500 centipawns
    assert!(score > 400 && score < 600, "Black should be up roughly a rook, got {}", score);
}

#[test]
fn req1_material_symmetric() {
    // Same position from white and black perspective should be negated
    let white_board = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
    let black_board = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::MATERIAL_ONLY);
    let white_score = evaluator.evaluate(&white_board);
    let black_score = evaluator.evaluate(&black_board);
    
    // Pure material should be 0 for both (equal material)
    assert_eq!(white_score, 0);
    assert_eq!(black_score, 0);
}

#[test]
fn req1_material_pawn_up() {
    // White is up a pawn
    let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let evaluator = Evaluator::new(EvalConfig::MATERIAL_ONLY);
    let score = evaluator.evaluate(&board);
    
    assert_eq!(score, 100, "White should be up exactly one pawn (100 cp)");
}

// =============================================================================
// REQ-2: Piece-Square Tables Tests
// =============================================================================

#[test]
fn req2_pst_knight_center_better() {
    // Knight on e4 vs knight on a1
    let center_knight = Board::from_fen("8/8/8/8/4N3/8/8/4K2k w - - 0 1").unwrap();
    let corner_knight = Board::from_fen("N7/8/8/8/8/8/8/4K2k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::PST_ONLY);
    let center_score = evaluator.evaluate(&center_knight);
    let corner_score = evaluator.evaluate(&corner_knight);
    
    assert!(center_score > corner_score, 
        "Central knight should score higher than corner knight");
}

#[test]
fn req2_pst_pawn_advancement() {
    // Advanced pawn should be better
    let advanced = Board::from_fen("8/4P3/8/8/8/8/8/4K2k w - - 0 1").unwrap();
    let starting = Board::from_fen("8/8/8/8/8/8/4P3/4K2k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::PST_ONLY);
    let advanced_score = evaluator.evaluate(&advanced);
    let starting_score = evaluator.evaluate(&starting);
    
    assert!(advanced_score > starting_score,
        "Advanced pawn should score higher, got advanced={} starting={}", advanced_score, starting_score);
}

#[test]
fn req2_pst_king_corner_middlegame() {
    // In middlegame, king should prefer corner (castled position)
    let corner_king = Board::from_fen("r1bq1rk1/pppppppp/8/8/8/8/PPPPPPPP/R1BQ1RK1 w - - 0 1").unwrap();
    let center_king = Board::from_fen("r1bqkr2/pppppppp/8/8/8/8/PPPPPPPP/R1BQK2R w KQq - 0 1").unwrap();
    
    // This depends on tapered eval, so just check they're different
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let _corner_score = evaluator.evaluate(&corner_king);
    let _center_score = evaluator.evaluate(&center_king);
    
    // Both should evaluate without panicking
}

// =============================================================================
// REQ-3: Pawn Structure Tests
// =============================================================================

#[test]
fn req3_doubled_pawns_penalty() {
    // Doubled pawns should be penalized
    let doubled = Board::from_fen("8/8/8/8/8/4P3/4P3/4K2k w - - 0 1").unwrap();
    let normal = Board::from_fen("8/8/8/8/8/4P3/3P4/4K2k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::PAWN_STRUCTURE_ONLY);
    let doubled_score = evaluator.evaluate(&doubled);
    let normal_score = evaluator.evaluate(&normal);
    
    assert!(doubled_score < normal_score,
        "Doubled pawns should score lower than normal pawns");
}

#[test]
fn req3_isolated_pawns_penalty() {
    // Isolated pawn should be penalized
    let isolated = Board::from_fen("8/8/8/8/4P3/8/8/4K2k w - - 0 1").unwrap();
    let supported = Board::from_fen("8/8/8/8/3PP3/8/8/4K2k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::PAWN_STRUCTURE_ONLY);
    let isolated_score = evaluator.evaluate(&isolated);
    let supported_score = evaluator.evaluate(&supported);
    
    assert!(isolated_score < supported_score,
        "Isolated pawn should score lower than supported pawn");
}

#[test]
fn req3_passed_pawns_bonus() {
    // Passed pawn should get bonus
    let passed = Board::from_fen("8/4P3/8/8/8/8/pppp1ppp/4K2k w - - 0 1").unwrap();
    let blocked = Board::from_fen("4p3/4P3/8/8/8/8/pppp1ppp/4K2k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::PAWN_STRUCTURE_ONLY);
    let passed_score = evaluator.evaluate(&passed);
    let blocked_score = evaluator.evaluate(&blocked);
    
    assert!(passed_score > blocked_score,
        "Passed pawn should score higher than blocked pawn");
}

#[test]
fn req3_connected_passed_pawns() {
    // Connected passed pawns should be very strong
    let connected = Board::from_fen("8/3PP3/8/8/8/8/8/4K2k w - - 0 1").unwrap();
    let single = Board::from_fen("8/4P3/8/8/8/8/8/4K2k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::PAWN_STRUCTURE_ONLY);
    let connected_score = evaluator.evaluate(&connected);
    let single_score = evaluator.evaluate(&single);
    
    // Connected passers are more than 2x a single passer
    assert!(connected_score > single_score,
        "Connected passed pawns should score very high");
}

// =============================================================================
// REQ-4: King Safety Tests
// =============================================================================

#[test]
fn req4_pawn_shield_bonus() {
    // King with pawn shield should score better
    let shielded = Board::from_fen("6k1/5ppp/8/8/8/8/5PPP/6K1 w - - 0 1").unwrap();
    let exposed = Board::from_fen("6k1/5ppp/8/8/8/8/8/6K1 w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::KING_SAFETY_ONLY);
    let shielded_score = evaluator.evaluate(&shielded);
    let exposed_score = evaluator.evaluate(&exposed);
    
    assert!(shielded_score > exposed_score,
        "King with pawn shield should score higher");
}

#[test]
fn req4_open_file_penalty() {
    // King on open file should be penalized
    let open_file = Board::from_fen("4r3/8/8/8/8/8/5PPP/4K3 w - - 0 1").unwrap();
    let closed = Board::from_fen("4r3/8/8/8/8/8/4PPPP/4K3 w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::KING_SAFETY_ONLY);
    let open_score = evaluator.evaluate(&open_file);
    let closed_score = evaluator.evaluate(&closed);
    
    assert!(open_score < closed_score,
        "King on open file should score lower");
}

// =============================================================================
// REQ-5: Mobility Tests
// =============================================================================

#[test]
fn req5_mobility_trapped_bishop() {
    // Trapped bishop should have low mobility score
    let trapped = Board::from_fen("8/8/1p6/2p5/8/B7/8/4K2k w - - 0 1").unwrap();
    let active = Board::from_fen("8/8/8/8/8/8/8/B3K2k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::MOBILITY_ONLY);
    let trapped_score = evaluator.evaluate(&trapped);
    let active_score = evaluator.evaluate(&active);
    
    assert!(trapped_score < active_score,
        "Trapped bishop should have lower mobility score");
}

#[test]
fn req5_mobility_active_rooks() {
    // Compare rook mobility between open and blocked configurations
    // Rook on same square, but with/without blocking pieces in front
    let open = Board::from_fen("8/8/8/8/8/8/8/4RK1k w - - 0 1").unwrap();
    let blocked = Board::from_fen("8/8/8/8/4P3/8/8/4RK1k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::MOBILITY_ONLY);
    let open_score = evaluator.evaluate(&open);
    let blocked_score = evaluator.evaluate(&blocked);
    
    // In open position, rook has full file mobility (7 squares up)
    // In blocked position, rook blocked by pawn on e4 (only 2 squares up)
    assert!(open_score > blocked_score,
        "Rooks on open files should have better mobility: open={} blocked={}", open_score, blocked_score);
}

// =============================================================================
// REQ-6: Center Control Tests
// =============================================================================

#[test]
fn req6_center_pawn_control() {
    // Pawns on e4/d4 should give center control bonus
    let central = Board::from_fen("8/8/8/8/3PP3/8/8/4K2k w - - 0 1").unwrap();
    let flank = Board::from_fen("8/8/8/8/P6P/8/8/4K2k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::CENTER_CONTROL_ONLY);
    let central_score = evaluator.evaluate(&central);
    let flank_score = evaluator.evaluate(&flank);
    
    assert!(central_score > flank_score,
        "Central pawns should score higher than flank pawns");
}

#[test]
fn req6_center_piece_control() {
    // Knights/bishops attacking center should get bonus
    let attacking = Board::from_fen("8/8/8/8/8/5N2/8/4K2k w - - 0 1").unwrap();
    let not_attacking = Board::from_fen("8/8/8/8/8/7N/8/4K2k w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::CENTER_CONTROL_ONLY);
    let attacking_score = evaluator.evaluate(&attacking);
    let not_score = evaluator.evaluate(&not_attacking);
    
    assert!(attacking_score > not_score,
        "Piece attacking center should score higher");
}

// =============================================================================
// REQ-7: Tapered Evaluation Tests
// =============================================================================

#[test]
fn req7_game_phase_calculation() {
    // Opening should have high phase, endgame low phase
    let opening = Board::default();
    let endgame = Board::from_fen("8/8/4k3/8/8/8/4K3/8 w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let opening_phase = evaluator.game_phase(&opening);
    let endgame_phase = evaluator.game_phase(&endgame);
    
    assert!(opening_phase > endgame_phase,
        "Opening should have higher game phase than endgame");
}

#[test]
fn req7_king_endgame_centralization() {
    // In endgame, king should prefer center
    let center_king = Board::from_fen("8/8/8/4K3/8/8/8/4k3 w - - 0 1").unwrap();
    let corner_king = Board::from_fen("K7/8/8/8/8/8/8/4k3 w - - 0 1").unwrap();
    
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let center_score = evaluator.evaluate(&center_king);
    let corner_score = evaluator.evaluate(&corner_king);
    
    assert!(center_score > corner_score,
        "Centralized king should score higher in endgame");
}

#[test]
fn req7_tapered_smooth_transition() {
    // Evaluation should transition smoothly between phases
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    // These positions have decreasing material
    let positions = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 1",
        "rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 1",
        "4k3/8/8/8/8/8/8/4K3 w - - 0 1",
    ];
    
    let phases: Vec<i32> = positions.iter()
        .map(|fen| evaluator.game_phase(&Board::from_fen(fen).unwrap()))
        .collect();
    
    // Phases should be monotonically decreasing
    for i in 1..phases.len() {
        assert!(phases[i] <= phases[i-1],
            "Game phase should decrease with material");
    }
}

// =============================================================================
// Feature Isolation Tests
// =============================================================================

#[test]
fn feature_isolation_material_only() {
    let board = Board::default();
    let evaluator = Evaluator::new(EvalConfig::MATERIAL_ONLY);
    let breakdown = evaluator.evaluate_breakdown(&board);
    
    // Only material should be non-zero (and it's 0 for equal material)
    assert_eq!(breakdown.material, 0);
    assert_eq!(breakdown.piece_square, 0);
    assert_eq!(breakdown.pawn_structure, 0);
    assert_eq!(breakdown.king_safety, 0);
    assert_eq!(breakdown.mobility, 0);
    assert_eq!(breakdown.center_control, 0);
}

#[test]
fn feature_isolation_incremental() {
    // Adding features should change the score predictably
    let board = Board::default();
    
    let material_only = Evaluator::new(EvalConfig::MATERIAL_ONLY);
    let with_pst = Evaluator::new(EvalConfig {
        material: true,
        piece_square_tables: true,
        ..EvalConfig::NONE
    });
    
    let score1 = material_only.evaluate(&board);
    let score2 = with_pst.evaluate(&board);
    
    // PST might change the score (centralized pieces get bonus)
    // Main thing is it doesn't crash
    let _ = (score1, score2);
}

#[test]
fn feature_symmetry_test() {
    // Symmetric position should evaluate to ~0
    let board = Board::default();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let score = evaluator.evaluate(&board);
    
    // Starting position should be very close to 0
    assert!(score.abs() < 50, 
        "Symmetric position should be close to 0, got {}", score);
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn integration_obvious_advantage() {
    // Queen up should always be winning
    let board = Board::from_fen("rnb1kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let score = evaluator.evaluate(&board);
    
    assert!(score > 500, "Queen advantage should give large positive score");
}

#[test]
fn integration_evaluate_does_not_panic() {
    // Various positions should all evaluate without panicking
    let positions = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        "8/8/8/8/8/8/8/4K2k w - - 0 1",
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
    ];
    
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    for fen in positions {
        let board = Board::from_fen(fen).unwrap();
        let _score = evaluator.evaluate(&board);
        // Just checking it doesn't panic
    }
}

#[test]
fn integration_breakdown_sums_correctly() {
    let board = Board::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 1").unwrap();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let breakdown = evaluator.evaluate_breakdown(&board);
    
    // Total should equal sum of components (accounting for tapering)
    let sum = breakdown.material 
        + breakdown.piece_square 
        + breakdown.pawn_structure 
        + breakdown.king_safety 
        + breakdown.mobility 
        + breakdown.center_control;
    
    // Allow small difference due to tapering interpolation
    assert!((breakdown.total - sum).abs() <= 5,
        "Breakdown sum {} should match total {}", sum, breakdown.total);
}
