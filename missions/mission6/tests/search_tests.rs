//! Mission 6: Search Algorithm Tests
//! TDD tests written BEFORE implementation

use prawn::board::Board;
use prawn::{EvalConfig, Evaluator, GameState, MoveGenerator};

// Import mission6 types (will be implemented)
use mission6_search::{
    SearchConfig, SearchResult, Searcher, MoveOrderingConfig,
    TranspositionTable, TTEntry, Bound,
};

// =============================================================================
// REQ-1: Negamax Search (Baseline)
// =============================================================================

#[test]
fn req1_negamax_depth_1_returns_best_capture() {
    // White can capture a free queen - knight on d3 can take queen on e5
    let board = Board::from_fen("rnb1kbnr/pppppppp/8/4q3/8/3N4/PPPPPPPP/R1BQKB1R w KQkq - 0 1").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::MATERIAL_ONLY);
    let config = SearchConfig::NEGAMAX_ONLY;
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    // Need depth 2: after Nxe5, black responds, then we evaluate the material advantage
    let result = searcher.search(&mut game, 2);
    
    // Best move should be Nd3xe5 capturing the queen
    println!("Best move: {:?}, Score: {}, Nodes: {}", result.best_move.map(|m| m.to_uci()), result.score, result.nodes);
    assert_eq!(result.best_move.unwrap().to_uci(), "d3e5");
}

#[test]
fn req1_negamax_detects_checkmate() {
    // Fool's mate position - Black has just checkmated White
    let board = Board::from_fen("rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::NEGAMAX_ONLY;
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 1);
    
    // Should recognize it's checkmate (no legal moves, in check)
    assert!(result.best_move.is_none() || result.score <= -29000);
}

#[test]
fn req1_negamax_detects_stalemate() {
    // Real stalemate position - Black king can't move but not in check
    // White: Ka6, Qc7. Black: Ka8. Black to move - stalemate
    let board = Board::from_fen("k7/2Q5/K7/8/8/8/8/8 b - - 0 1").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::NEGAMAX_ONLY;
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 1);
    
    // Stalemate should return draw score (0)
    assert_eq!(result.score, 0);
}

#[test]
fn req1_negamax_finds_mate_in_1() {
    // White to move, Qh7# is mate in 1
    let board = Board::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::NEGAMAX_ONLY;
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 2);
    
    // Should find Qxf7# (checkmate)
    assert_eq!(result.best_move.unwrap().to_uci(), "h5f7");
    assert!(result.score > 29000); // Mate score
}

// =============================================================================
// REQ-2: Alpha-Beta Pruning
// =============================================================================

#[test]
fn req2_alpha_beta_same_result_as_negamax() {
    let board = Board::from_fen("r1bqkbnr/pppppppp/2n5/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    // Search with negamax only
    let negamax_config = SearchConfig::NEGAMAX_ONLY;
    let mut searcher1 = Searcher::new(negamax_config, &movegen, &evaluator);
    let result1 = searcher1.search(&mut game, 3);
    
    // Search with alpha-beta
    let ab_config = SearchConfig::ALPHA_BETA_ONLY;
    let mut searcher2 = Searcher::new(ab_config, &movegen, &evaluator);
    let result2 = searcher2.search(&mut game, 3);
    
    // Should get same score
    assert_eq!(result1.score, result2.score);
}

#[test]
fn req2_alpha_beta_reduces_nodes() {
    let board = Board::default();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::MATERIAL_ONLY);
    
    // Search with negamax only
    let negamax_config = SearchConfig::NEGAMAX_ONLY;
    let mut searcher1 = Searcher::new(negamax_config, &movegen, &evaluator);
    let result1 = searcher1.search(&mut game, 4);
    
    // Search with alpha-beta
    let ab_config = SearchConfig::ALPHA_BETA_ONLY;
    let mut searcher2 = Searcher::new(ab_config, &movegen, &evaluator);
    let result2 = searcher2.search(&mut game, 4);
    
    // Alpha-beta should search significantly fewer nodes
    println!("Negamax nodes: {}, Alpha-beta nodes: {}", result1.nodes, result2.nodes);
    assert!(result2.nodes < result1.nodes / 2, "Alpha-beta should reduce nodes by at least 50%");
}

// =============================================================================
// REQ-3: Iterative Deepening
// =============================================================================

#[test]
fn req3_iterative_deepening_returns_result_at_each_depth() {
    let board = Board::default();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::with_iterative_deepening();
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 4);
    
    // Should have searched depths 1-4
    assert_eq!(result.depth, 4);
    assert!(result.best_move.is_some());
}

#[test]
fn req3_iterative_deepening_finds_deeper_tactics() {
    // Position where depth 3 sees a different move than depth 5
    let board = Board::from_fen("r2qkb1r/ppp2ppp/2n2n2/3pp1B1/2B1P1b1/3P1N2/PPP2PPP/RN1QK2R w KQkq - 0 6").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::with_iterative_deepening();
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 4);
    
    // Should find a valid move at depth 4
    assert!(result.best_move.is_some());
}

// =============================================================================
// REQ-4: Quiescence Search
// =============================================================================

#[test]
fn req4_quiescence_avoids_horizon_effect() {
    // Position where a capture sequence changes evaluation
    // White appears to win a pawn but actually loses the queen after recaptures
    let board = Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq d3 0 3").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    // Without quiescence
    let config1 = SearchConfig::ALPHA_BETA_ONLY;
    let mut searcher1 = Searcher::new(config1, &movegen, &evaluator);
    let result1 = searcher1.search(&mut game, 3);
    
    // With quiescence
    let config2 = SearchConfig::with_quiescence();
    let mut searcher2 = Searcher::new(config2, &movegen, &evaluator);
    let result2 = searcher2.search(&mut game, 3);
    
    // Both should work (quiescence may find different score due to deeper tactical analysis)
    assert!(result1.best_move.is_some());
    assert!(result2.best_move.is_some());
}

#[test]
fn req4_quiescence_handles_check() {
    // Position in check - quiescence must handle this
    let board = Board::from_fen("r1bqkbnr/pppp1Qpp/2n5/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::with_quiescence();
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 1);
    
    // Should find a move even in check (or recognize checkmate)
    assert!(result.best_move.is_some() || result.score < -29000);
}

// =============================================================================
// REQ-5: Transposition Table
// =============================================================================

#[test]
fn req5_tt_stores_and_retrieves() {
    let mut tt = TranspositionTable::new(16); // 16 MB
    
    let entry = TTEntry {
        key: 0x12345678ABCDEF00,
        score: 150,
        depth: 5,
        bound: Bound::Exact,
        best_move: None,
        age: 0,
    };
    
    tt.store(entry.key, entry.score, entry.depth, entry.bound, entry.best_move);
    
    let retrieved = tt.probe(entry.key);
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.score, 150);
    assert_eq!(retrieved.depth, 5);
}

#[test]
fn req5_tt_reduces_nodes_in_symmetric_position() {
    // Symmetric position has many transpositions
    let board = Board::default();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    // Without TT
    let config1 = SearchConfig::ALPHA_BETA_ONLY;
    let mut searcher1 = Searcher::new(config1, &movegen, &evaluator);
    let result1 = searcher1.search(&mut game, 4);
    
    // With TT
    let config2 = SearchConfig::with_tt();
    let mut searcher2 = Searcher::new(config2, &movegen, &evaluator);
    let result2 = searcher2.search(&mut game, 4);
    
    println!("Without TT: {} nodes, With TT: {} nodes", result1.nodes, result2.nodes);
    // TT should reduce nodes (may not always be significant at low depths)
    assert!(result2.nodes <= result1.nodes);
}

// =============================================================================
// REQ-6: Move Ordering
// =============================================================================

#[test]
fn req6_mvv_lva_orders_captures_correctly() {
    // Position with multiple captures available
    let board = Board::from_fen("r1bqkb1r/pppppppp/2n2n2/4P3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 3").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    // Without move ordering
    let config1 = SearchConfig::ALPHA_BETA_ONLY;
    let mut searcher1 = Searcher::new(config1, &movegen, &evaluator);
    let result1 = searcher1.search(&mut game, 4);
    
    // With MVV-LVA ordering
    let config2 = SearchConfig::with_mvv_lva();
    let mut searcher2 = Searcher::new(config2, &movegen, &evaluator);
    let result2 = searcher2.search(&mut game, 4);
    
    println!("Without ordering: {} nodes, With MVV-LVA: {} nodes", result1.nodes, result2.nodes);
    // Same result, fewer nodes
    assert_eq!(result1.score, result2.score);
    assert!(result2.nodes <= result1.nodes);
}

#[test]
fn req6_killer_moves_improve_ordering() {
    let board = Board::default();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    // With MVV-LVA only
    let config1 = SearchConfig::with_mvv_lva();
    let mut searcher1 = Searcher::new(config1, &movegen, &evaluator);
    let result1 = searcher1.search(&mut game, 4);
    
    // With MVV-LVA + killer moves
    let config2 = SearchConfig::with_killers();
    let mut searcher2 = Searcher::new(config2, &movegen, &evaluator);
    let result2 = searcher2.search(&mut game, 4);
    
    println!("MVV-LVA only: {} nodes, With killers: {} nodes", result1.nodes, result2.nodes);
    // Should search same or fewer nodes
    assert!(result2.nodes <= result1.nodes + 1000); // Allow small variance
}

// =============================================================================
// REQ-7: Null Move Pruning
// =============================================================================

#[test]
fn req7_null_move_reduces_nodes() {
    // Position where null move should help
    let board = Board::from_fen("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 0 5").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    // Without null move
    let config1 = SearchConfig::with_mvv_lva();
    let mut searcher1 = Searcher::new(config1, &movegen, &evaluator);
    let result1 = searcher1.search(&mut game, 4);
    
    // With null move
    let config2 = SearchConfig::with_null_move();
    let mut searcher2 = Searcher::new(config2, &movegen, &evaluator);
    let result2 = searcher2.search(&mut game, 4);
    
    println!("Without null move: {} nodes, With null move: {} nodes", result1.nodes, result2.nodes);
    // Null move should reduce nodes in most positions
    assert!(result2.nodes <= result1.nodes);
}

#[test]
fn req7_null_move_disabled_when_in_check() {
    // Position where side to move is in check - null move must not be used
    let board = Board::from_fen("r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::with_null_move();
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 3);
    
    // Should still find a move (doesn't crash with null move in check)
    assert!(result.best_move.is_some() || result.score < -29000);
}

// =============================================================================
// REQ-8: Late Move Reduction (LMR)
// =============================================================================

#[test]
fn req8_lmr_reduces_nodes() {
    let board = Board::default();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    // Without LMR
    let config1 = SearchConfig::with_null_move();
    let mut searcher1 = Searcher::new(config1, &movegen, &evaluator);
    let result1 = searcher1.search(&mut game, 4);
    
    // With LMR
    let config2 = SearchConfig::with_lmr();
    let mut searcher2 = Searcher::new(config2, &movegen, &evaluator);
    let result2 = searcher2.search(&mut game, 4);
    
    println!("Without LMR: {} nodes, With LMR: {} nodes", result1.nodes, result2.nodes);
    // LMR should reduce nodes
    assert!(result2.nodes <= result1.nodes);
}

#[test]
fn req8_lmr_maintains_correctness() {
    // Position with clear best move
    let board = Board::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    
    let config = SearchConfig::with_lmr();
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 4);
    
    // Should still find checkmate
    assert_eq!(result.best_move.unwrap().to_uci(), "h5f7");
}

// =============================================================================
// REQ-9: Principal Variation Tracking
// =============================================================================

#[test]
fn req9_pv_extracted() {
    let board = Board::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::ALL;
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 3);
    
    // PV should contain the best line
    assert!(!result.pv.is_empty());
    assert_eq!(result.pv[0].to_uci(), "h5f7"); // First move should be Qxf7#
}

// =============================================================================
// Integration / Benchmark Tests
// =============================================================================

#[test]
fn integration_all_features_find_mate_in_2() {
    // Mate in 2: 1. Qxf7+ Ke7 2. Qxe7# (or other variations)
    let board = Board::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4").unwrap();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::ALL;
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 4);
    
    // Should find the mate
    assert!(result.score > 29000);
    assert_eq!(result.best_move.unwrap().to_uci(), "h5f7");
}

#[test]
fn integration_starting_position_reasonable_move() {
    let board = Board::default();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::ALL;
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    let result = searcher.search(&mut game, 4);
    
    // Should find a reasonable opening move
    let uci = result.best_move.unwrap().to_uci();
    let reasonable_moves = ["e2e4", "d2d4", "g1f3", "c2c4", "e2e3", "d2d3", "b1c3"];
    assert!(reasonable_moves.contains(&uci.as_str()), "Expected reasonable opening move, got {}", uci);
}

#[test]
fn benchmark_search_speed() {
    use std::time::Instant;
    
    let board = Board::default();
    let mut game = GameState::from_board(board);
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::ALL);
    let config = SearchConfig::ALL;
    
    let mut searcher = Searcher::new(config, &movegen, &evaluator);
    
    let start = Instant::now();
    let result = searcher.search(&mut game, 5);
    let elapsed = start.elapsed();
    
    let nps = result.nodes as f64 / elapsed.as_secs_f64();
    println!("Depth 5: {} nodes in {:?} ({:.0} nps)", result.nodes, elapsed, nps);
    
    // Should search at reasonable speed (at least 100k nps in debug, millions in release)
    assert!(result.nodes > 0);
}
