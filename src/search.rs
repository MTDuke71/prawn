//! Search Algorithm
//! 
//! Modular negamax search with alpha-beta pruning and various enhancements.
//! Each feature can be toggled for measuring ELO impact.

use crate::board::{Board, PieceType};
use crate::{Evaluator, GameState, Move, MoveGenerator};

use crate::transposition::{TranspositionTable, Bound};
use crate::move_ordering::{order_moves, pick_best, KillerMoves, HistoryTable, MoveScore};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// Mate score constants
pub const MATE_SCORE: i32 = 30000;

/// Configuration for move ordering features
#[derive(Debug, Clone, Copy)]
pub struct MoveOrderingConfig {
    pub mvv_lva: bool,
    pub killer_moves: bool,
    pub history_heuristic: bool,
    pub tt_move: bool,
}

impl MoveOrderingConfig {
    pub const NONE: Self = Self {
        mvv_lva: false,
        killer_moves: false,
        history_heuristic: false,
        tt_move: false,
    };
    
    pub const MVV_LVA_ONLY: Self = Self {
        mvv_lva: true,
        killer_moves: false,
        history_heuristic: false,
        tt_move: false,
    };
    
    pub const ALL: Self = Self {
        mvv_lva: true,
        killer_moves: true,
        history_heuristic: true,
        tt_move: true,
    };
}

/// Configuration for search features (all toggleable for ELO measurement)
#[derive(Debug, Clone, Copy)]
pub struct SearchConfig {
    /// Use alpha-beta pruning (vs plain negamax)
    pub alpha_beta: bool,
    /// Use iterative deepening
    pub iterative_deepening: bool,
    /// Use quiescence search
    pub quiescence: bool,
    /// Use transposition table
    pub transposition_table: bool,
    /// Move ordering configuration
    pub move_ordering: MoveOrderingConfig,
    /// Use null move pruning
    pub null_move_pruning: bool,
    /// Use late move reductions
    pub lmr: bool,
    /// Use aspiration windows
    pub aspiration_windows: bool,
    /// Null move reduction depth
    pub null_move_r: u8,
    /// LMR threshold (search this many moves at full depth)
    pub lmr_threshold: usize,
    /// Quiescence search depth limit
    pub qs_depth_limit: i32,
}

impl SearchConfig {
    /// Plain negamax - baseline for comparison
    pub const NEGAMAX_ONLY: Self = Self {
        alpha_beta: false,
        iterative_deepening: false,
        quiescence: false,
        transposition_table: false,
        move_ordering: MoveOrderingConfig::NONE,
        null_move_pruning: false,
        lmr: false,
        aspiration_windows: false,
        null_move_r: 2,
        lmr_threshold: 4,
        qs_depth_limit: 8,
    };
    
    /// Alpha-beta only
    pub const ALPHA_BETA_ONLY: Self = Self {
        alpha_beta: true,
        iterative_deepening: false,
        quiescence: false,
        transposition_table: false,
        move_ordering: MoveOrderingConfig::NONE,
        null_move_pruning: false,
        lmr: false,
        aspiration_windows: false,
        null_move_r: 2,
        lmr_threshold: 4,
        qs_depth_limit: 8,
    };
    
    /// All features enabled
    pub const ALL: Self = Self {
        alpha_beta: true,
        iterative_deepening: true,
        quiescence: true,
        transposition_table: true,
        move_ordering: MoveOrderingConfig::ALL,
        null_move_pruning: true,
        lmr: true,
        aspiration_windows: true,
        null_move_r: 3,
        lmr_threshold: 4,
        qs_depth_limit: 8,
    };
    
    /// Default config - good balance of speed and strength
    pub const DEFAULT: Self = Self {
        alpha_beta: true,
        iterative_deepening: true,
        quiescence: true,
        transposition_table: true,
        move_ordering: MoveOrderingConfig::ALL,
        null_move_pruning: true,
        lmr: true,
        aspiration_windows: false,
        null_move_r: 3,
        lmr_threshold: 4,
        qs_depth_limit: 8,
    };
    
    /// Create config with iterative deepening
    pub fn with_iterative_deepening() -> Self {
        Self {
            alpha_beta: true,
            iterative_deepening: true,
            quiescence: false,
            transposition_table: false,
            move_ordering: MoveOrderingConfig::MVV_LVA_ONLY,
            null_move_pruning: false,
            lmr: false,
            aspiration_windows: false,
            null_move_r: 2,
            lmr_threshold: 4,
            qs_depth_limit: 8,
        }
    }
    
    /// Create config with quiescence search
    pub fn with_quiescence() -> Self {
        Self {
            alpha_beta: true,
            iterative_deepening: false,
            quiescence: true,
            transposition_table: false,
            move_ordering: MoveOrderingConfig::MVV_LVA_ONLY,
            null_move_pruning: false,
            lmr: false,
            aspiration_windows: false,
            null_move_r: 2,
            lmr_threshold: 4,
            qs_depth_limit: 8,
        }
    }
    
    /// Create config with transposition table
    pub fn with_tt() -> Self {
        Self {
            alpha_beta: true,
            iterative_deepening: true,
            quiescence: false,
            transposition_table: true,
            move_ordering: MoveOrderingConfig::ALL,
            null_move_pruning: false,
            lmr: false,
            aspiration_windows: false,
            null_move_r: 2,
            lmr_threshold: 4,
            qs_depth_limit: 8,
        }
    }
    
    /// Create config with MVV-LVA move ordering
    pub fn with_mvv_lva() -> Self {
        Self {
            alpha_beta: true,
            iterative_deepening: false,
            quiescence: false,
            transposition_table: false,
            move_ordering: MoveOrderingConfig::MVV_LVA_ONLY,
            null_move_pruning: false,
            lmr: false,
            aspiration_windows: false,
            null_move_r: 2,
            lmr_threshold: 4,
            qs_depth_limit: 8,
        }
    }
    
    /// Create config with killer moves
    pub fn with_killers() -> Self {
        Self {
            alpha_beta: true,
            iterative_deepening: false,
            quiescence: false,
            transposition_table: false,
            move_ordering: MoveOrderingConfig {
                mvv_lva: true,
                killer_moves: true,
                history_heuristic: false,
                tt_move: false,
            },
            null_move_pruning: false,
            lmr: false,
            aspiration_windows: false,
            null_move_r: 2,
            lmr_threshold: 4,
            qs_depth_limit: 8,
        }
    }
    
    /// Create config with null move pruning
    pub fn with_null_move() -> Self {
        Self {
            alpha_beta: true,
            iterative_deepening: false,
            quiescence: false,
            transposition_table: false,
            move_ordering: MoveOrderingConfig::MVV_LVA_ONLY,
            null_move_pruning: true,
            lmr: false,
            aspiration_windows: false,
            null_move_r: 3,
            lmr_threshold: 4,
            qs_depth_limit: 8,
        }
    }
    
    /// Create config with LMR
    pub fn with_lmr() -> Self {
        Self {
            alpha_beta: true,
            iterative_deepening: false,
            quiescence: false,
            transposition_table: false,
            move_ordering: MoveOrderingConfig::MVV_LVA_ONLY,
            null_move_pruning: true,
            lmr: true,
            aspiration_windows: false,
            null_move_r: 3,
            lmr_threshold: 4,
            qs_depth_limit: 8,
        }
    }
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Best move found
    pub best_move: Option<Move>,
    /// Score of the position
    pub score: i32,
    /// Depth searched
    pub depth: u8,
    /// Selective depth (max depth reached in quiescence)
    pub seldepth: u8,
    /// Nodes searched
    pub nodes: u64,
    /// Principal variation
    pub pv: Vec<Move>,
}

/// Search limits for time management
#[derive(Debug, Clone)]
pub struct SearchLimits {
    /// Maximum depth to search
    pub max_depth: u8,
    /// Stop flag for async termination
    pub stop_flag: Option<Arc<AtomicBool>>,
    /// Start time of search
    pub start_time: Option<Instant>,
    /// Target time in ms (soft limit)
    pub target_time_ms: Option<u64>,
    /// Maximum time in ms (hard limit)  
    pub max_time_ms: Option<u64>,
    /// Node limit
    pub node_limit: Option<u64>,
    /// Infinite search mode
    pub infinite: bool,
}

impl Default for SearchLimits {
    fn default() -> Self {
        Self {
            max_depth: 64,
            stop_flag: None,
            start_time: None,
            target_time_ms: None,
            max_time_ms: None,
            node_limit: None,
            infinite: false,
        }
    }
}

impl SearchLimits {
    /// Create limits for fixed depth search
    pub fn depth(depth: u8) -> Self {
        Self {
            max_depth: depth,
            ..Default::default()
        }
    }
    
    /// Create limits for fixed time search
    pub fn movetime(time_ms: u64, stop_flag: Arc<AtomicBool>) -> Self {
        Self {
            max_depth: 64,
            stop_flag: Some(stop_flag),
            start_time: Some(Instant::now()),
            target_time_ms: Some(time_ms),
            max_time_ms: Some(time_ms),
            node_limit: None,
            infinite: false,
        }
    }
    
    /// Create limits for tournament time control
    pub fn time_control(
        target_ms: u64, 
        max_ms: u64, 
        stop_flag: Arc<AtomicBool>
    ) -> Self {
        Self {
            max_depth: 64,
            stop_flag: Some(stop_flag),
            start_time: Some(Instant::now()),
            target_time_ms: Some(target_ms),
            max_time_ms: Some(max_ms),
            node_limit: None,
            infinite: false,
        }
    }
    
    /// Create limits for infinite search
    pub fn infinite(stop_flag: Arc<AtomicBool>) -> Self {
        Self {
            max_depth: 64,
            stop_flag: Some(stop_flag),
            start_time: Some(Instant::now()),
            target_time_ms: None,
            max_time_ms: None,
            node_limit: None,
            infinite: true,
        }
    }
    
    /// Check if we should stop searching
    #[inline]
    pub fn should_stop(&self) -> bool {
        // Check external stop flag
        if let Some(ref flag) = self.stop_flag {
            if flag.load(Ordering::Relaxed) {
                return true;
            }
        }
        
        // Check hard time limit
        if let (Some(start), Some(max_ms)) = (self.start_time, self.max_time_ms) {
            if start.elapsed().as_millis() as u64 >= max_ms {
                return true;
            }
        }
        
        false
    }
    
    /// Check if we can start another iteration
    #[inline]
    pub fn can_start_iteration(&self) -> bool {
        // Check external stop flag
        if let Some(ref flag) = self.stop_flag {
            if flag.load(Ordering::Relaxed) {
                return false;
            }
        }
        
        // Infinite mode - always continue
        if self.infinite {
            return true;
        }
        
        // Check target time limit
        if let (Some(start), Some(target_ms)) = (self.start_time, self.target_time_ms) {
            if start.elapsed().as_millis() as u64 >= target_ms {
                return false;
            }
        }
        
        true
    }
    
    /// Get elapsed time in ms
    pub fn elapsed_ms(&self) -> u64 {
        self.start_time
            .map(|s| s.elapsed().as_millis() as u64)
            .unwrap_or(0)
    }
}

/// The search engine
pub struct Searcher<'a> {
    config: SearchConfig,
    movegen: &'a MoveGenerator,
    evaluator: &'a Evaluator,
    tt: Option<TranspositionTable>,
    killers: KillerMoves,
    history: HistoryTable,
    nodes: u64,
    ply: usize,
    seldepth: u8,
    pv_table: [[Option<Move>; 128]; 128],
    pv_length: [usize; 128],
    limits: SearchLimits,
    /// Callback for reporting search info
    #[allow(clippy::type_complexity)]
    info_callback: Option<Box<dyn Fn(&SearchResult, u64) + 'a>>,
}

impl<'a> Searcher<'a> {
    /// Create a new searcher
    pub fn new(config: SearchConfig, movegen: &'a MoveGenerator, evaluator: &'a Evaluator) -> Self {
        let tt = if config.transposition_table {
            Some(TranspositionTable::new(64)) // 64 MB default
        } else {
            None
        };
        
        Self {
            config,
            movegen,
            evaluator,
            tt,
            killers: KillerMoves::new(),
            history: HistoryTable::new(),
            nodes: 0,
            ply: 0,
            seldepth: 0,
            pv_table: [[None; 128]; 128],
            pv_length: [0; 128],
            limits: SearchLimits::default(),
            info_callback: None,
        }
    }
    
    /// Create a new searcher with custom hash size
    pub fn with_hash_size(config: SearchConfig, movegen: &'a MoveGenerator, evaluator: &'a Evaluator, hash_mb: usize) -> Self {
        let tt = if config.transposition_table && hash_mb > 0 {
            Some(TranspositionTable::new(hash_mb))
        } else {
            None
        };
        
        Self {
            config,
            movegen,
            evaluator,
            tt,
            killers: KillerMoves::new(),
            history: HistoryTable::new(),
            nodes: 0,
            ply: 0,
            seldepth: 0,
            pv_table: [[None; 128]; 128],
            pv_length: [0; 128],
            limits: SearchLimits::default(),
            info_callback: None,
        }
    }
    
    /// Set a callback for info output during search
    pub fn set_info_callback<F: Fn(&SearchResult, u64) + 'a>(&mut self, callback: F) {
        self.info_callback = Some(Box::new(callback));
    }
    
    /// Clear the transposition table
    pub fn clear_tt(&mut self) {
        if let Some(ref mut tt) = self.tt {
            tt.clear();
        }
    }
    
    /// Get hashfull (permill of TT entries used)
    pub fn hashfull(&self) -> Option<u32> {
        self.tt.as_ref().map(|tt| tt.hashfull() as u32)
    }
    
    /// Search the position to the given depth (simple API)
    pub fn search(&mut self, game: &mut GameState, depth: u8) -> SearchResult {
        let limits = SearchLimits::depth(depth);
        self.search_with_limits(game, limits)
    }
    
    /// Search with full time/depth limits (full API for UCI)
    pub fn search_with_limits(&mut self, game: &mut GameState, limits: SearchLimits) -> SearchResult {
        self.nodes = 0;
        self.seldepth = 0;
        self.limits = limits;
        self.killers.clear();
        
        if let Some(ref mut tt) = self.tt {
            tt.new_search();
        }
        
        if self.config.iterative_deepening {
            self.iterative_deepening(game, self.limits.max_depth)
        } else {
            self.search_root(game, self.limits.max_depth)
        }
    }
    
    /// Iterative deepening search
    fn iterative_deepening(&mut self, game: &mut GameState, max_depth: u8) -> SearchResult {
        let mut best_result = SearchResult {
            best_move: None,
            score: 0,
            depth: 0,
            seldepth: 0,
            nodes: 0,
            pv: Vec::new(),
        };
        
        for depth in 1..=max_depth {
            // Check if we can start another iteration
            if depth > 1 && !self.limits.can_start_iteration() {
                break;
            }
            
            let result = self.search_root(game, depth);
            
            // If we were stopped mid-search, don't use partial results
            if self.limits.should_stop() && depth > 1 {
                break;
            }
            
            // Update best result
            best_result = SearchResult {
                best_move: result.best_move,
                score: result.score,
                depth,
                seldepth: self.seldepth,
                nodes: self.nodes,
                pv: result.pv,
            };
            
            // Report info at each depth
            if let Some(ref callback) = self.info_callback {
                callback(&best_result, self.limits.elapsed_ms());
            }
        }
        
        best_result
    }
    
    /// Root search
    fn search_root(&mut self, game: &mut GameState, depth: u8) -> SearchResult {
        self.ply = 0;
        self.pv_length[0] = 0;
        
        let alpha = -MATE_SCORE;
        let beta = MATE_SCORE;
        
        let moves = self.movegen.generate_legal_moves(game.board());
        
        if moves.is_empty() {
            // No legal moves
            let in_check = self.movegen.is_in_check(game.board(), game.board().side_to_move());
            let score = if in_check { -MATE_SCORE } else { 0 };
            return SearchResult {
                best_move: None,
                score,
                depth,
                seldepth: 0,
                nodes: self.nodes,
                pv: Vec::new(),
            };
        }
        
        // Get TT move for ordering
        let tt_move = self.tt.as_ref().and_then(|tt| {
            tt.probe_copy(game.zobrist_hash()).and_then(|e| e.best_move)
        });
        
        // Order moves
        let scored_moves = self.order_moves(game.board(), moves.moves(), tt_move);
        
        let mut best_move = None;
        let mut best_score = -MATE_SCORE;
        let mut alpha = alpha;
        
        for ms in &scored_moves {
            // Check for stop
            if self.limits.should_stop() {
                break;
            }
            
            let mv = ms.mv;
            game.make_move(mv);
            self.ply += 1;
            
            let score = if self.config.alpha_beta {
                -self.alpha_beta(game, depth - 1, -beta, -alpha, true)
            } else {
                -self.negamax(game, depth - 1)
            };
            
            self.ply -= 1;
            game.unmake_move();
            
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
                
                // Update PV
                self.pv_table[0][0] = Some(mv);
                for j in 0..self.pv_length[1] {
                    self.pv_table[0][j + 1] = self.pv_table[1][j];
                }
                self.pv_length[0] = self.pv_length[1] + 1;
                
                if score > alpha {
                    alpha = score;
                }
            }
        }
        
        // Store in TT
        if let Some(ref mut tt) = self.tt {
            tt.store(game.zobrist_hash(), best_score, depth, Bound::Exact, best_move);
        }
        
        // Extract PV
        let pv: Vec<Move> = (0..self.pv_length[0])
            .filter_map(|i| self.pv_table[0][i])
            .collect();
        
        SearchResult {
            best_move,
            score: best_score,
            depth,
            seldepth: self.seldepth,
            nodes: self.nodes,
            pv,
        }
    }
    
    /// Plain negamax (for comparison)
    fn negamax(&mut self, game: &mut GameState, depth: u8) -> i32 {
        self.nodes += 1;
        
        if depth == 0 {
            return self.evaluator.evaluate(game.board());
        }
        
        let moves = self.movegen.generate_legal_moves(game.board());
        
        if moves.is_empty() {
            let in_check = self.movegen.is_in_check(game.board(), game.board().side_to_move());
            return if in_check {
                -MATE_SCORE + self.ply as i32
            } else {
                0
            };
        }
        
        let mut best_score = -MATE_SCORE;
        
        for mv in moves.moves() {
            game.make_move(*mv);
            self.ply += 1;
            
            let score = -self.negamax(game, depth - 1);
            
            self.ply -= 1;
            game.unmake_move();
            
            if score > best_score {
                best_score = score;
            }
        }
        
        best_score
    }
    
    /// Alpha-beta search
    fn alpha_beta(&mut self, game: &mut GameState, depth: u8, mut alpha: i32, beta: i32, do_null: bool) -> i32 {
        self.nodes += 1;
        
        // Guard against exceeding array bounds (max ply = 127)
        if self.ply >= 127 {
            return self.evaluator.evaluate(game.board());
        }
        
        self.pv_length[self.ply] = 0;
        
        // Check for stop (every 1024 nodes to reduce overhead)
        if (self.nodes & 1023) == 0 && self.limits.should_stop() {
            return alpha;
        }
        
        // Check for repetition (simple: draw)
        // TODO: implement proper repetition detection
        
        // Leaf node
        if depth == 0 {
            return if self.config.quiescence {
                self.quiescence(game, alpha, beta, 0)
            } else {
                self.evaluator.evaluate(game.board())
            };
        }
        
        let in_check = self.movegen.is_in_check(game.board(), game.board().side_to_move());
        
        // Check TT
        let tt_move = if let Some(ref mut tt) = self.tt {
            if let Some(entry) = tt.probe(game.zobrist_hash()) {
                if entry.depth >= depth {
                    match entry.bound {
                        Bound::Exact => return entry.score,
                        Bound::Lower if entry.score >= beta => return entry.score,
                        Bound::Upper if entry.score <= alpha => return entry.score,
                        _ => {}
                    }
                }
                entry.best_move
            } else {
                None
            }
        } else {
            None
        };
        
        // Null move pruning
        if self.config.null_move_pruning 
            && do_null 
            && !in_check 
            && depth > self.config.null_move_r + 1  // Must have enough depth after reduction
            && self.has_non_pawn_material(game.board())
        {
            let old_ep = game.make_null_move();
            self.ply += 1;
            
            let reduced_depth = depth - 1 - self.config.null_move_r;
            let score = -self.alpha_beta(game, reduced_depth, -beta, -beta + 1, false);
            
            self.ply -= 1;
            game.unmake_null_move(old_ep);
            
            if score >= beta {
                return beta;
            }
        }
        
        // Generate and order moves
        let moves = self.movegen.generate_legal_moves(game.board());
        
        if moves.is_empty() {
            return if in_check {
                -MATE_SCORE + self.ply as i32
            } else {
                0
            };
        }
        
        let mut scored_moves = self.order_moves(game.board(), moves.moves(), tt_move);
        
        let mut best_move = None;
        let mut best_score = -MATE_SCORE;
        
        #[allow(clippy::explicit_counter_loop)]
        for (moves_searched, i) in (0..scored_moves.len()).enumerate() {
            // Incremental sorting: pick best for this iteration
            pick_best(&mut scored_moves, i);
            let mv = scored_moves[i].mv;
            
            game.make_move(mv);
            self.ply += 1;
            
            let mut score;
            
            // Late Move Reduction
            if self.config.lmr 
                && moves_searched >= self.config.lmr_threshold 
                && depth >= 3 
                && !in_check
                && mv.captured().is_none()
                && mv.promotion().is_none()
            {
                // Reduced search
                score = -self.alpha_beta(game, depth - 2, -alpha - 1, -alpha, true);
                
                // Re-search if it might be good
                if score > alpha {
                    score = -self.alpha_beta(game, depth - 1, -beta, -alpha, true);
                }
            } else {
                score = -self.alpha_beta(game, depth - 1, -beta, -alpha, true);
            }
            
            self.ply -= 1;
            game.unmake_move();
            
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
                
                if score > alpha {
                    alpha = score;
                    
                    // Update PV
                    self.pv_table[self.ply][0] = Some(mv);
                    for j in 0..self.pv_length[self.ply + 1] {
                        self.pv_table[self.ply][j + 1] = self.pv_table[self.ply + 1][j];
                    }
                    self.pv_length[self.ply] = self.pv_length[self.ply + 1] + 1;
                    
                    if score >= beta {
                        // Beta cutoff
                        if mv.captured().is_none() {
                            self.killers.add(self.ply, mv);
                            self.history.add(game.board().side_to_move() as usize, mv, depth as i32);
                        }
                        
                        // Store in TT
                        if let Some(ref mut tt) = self.tt {
                            tt.store(game.zobrist_hash(), score, depth, Bound::Lower, best_move);
                        }
                        
                        return beta;
                    }
                }
            }
        }
        
        // Store in TT
        if let Some(ref mut tt) = self.tt {
            let bound = if best_score <= alpha { Bound::Upper } else { Bound::Exact };
            tt.store(game.zobrist_hash(), best_score, depth, bound, best_move);
        }
        
        best_score
    }
    
    /// Quiescence search - search captures until position is quiet
    fn quiescence(&mut self, game: &mut GameState, mut alpha: i32, beta: i32, qs_depth: i32) -> i32 {
        self.nodes += 1;
        
        // Track selective depth
        let current_depth = self.ply as u8 + qs_depth as u8;
        if current_depth > self.seldepth {
            self.seldepth = current_depth;
        }
        
        // Stand pat
        let stand_pat = self.evaluator.evaluate(game.board());
        
        if stand_pat >= beta {
            return beta;
        }
        
        if alpha < stand_pat {
            alpha = stand_pat;
        }
        
        // Depth limit for quiescence
        if qs_depth >= self.config.qs_depth_limit {
            return alpha;
        }
        
        // Generate captures only
        let moves = self.movegen.generate_legal_moves(game.board());
        
        // Filter to captures and promotions
        let captures: Vec<Move> = moves.moves()
            .iter()
            .filter(|m| m.captured().is_some() || m.promotion().is_some())
            .copied()
            .collect();
        
        if captures.is_empty() {
            return alpha;
        }
        
        // Order captures by MVV-LVA
        let mut scored = self.order_moves(game.board(), &captures, None);
        
        for i in 0..scored.len() {
            pick_best(&mut scored, i);
            let mv = scored[i].mv;
            
            game.make_move(mv);
            self.ply += 1;
            
            let score = -self.quiescence(game, -beta, -alpha, qs_depth + 1);
            
            self.ply -= 1;
            game.unmake_move();
            
            if score >= beta {
                return beta;
            }
            
            if score > alpha {
                alpha = score;
            }
        }
        
        alpha
    }
    
    /// Order moves based on config
    fn order_moves(&self, board: &Board, moves: &[Move], tt_move: Option<Move>) -> Vec<MoveScore> {
        let killers = if self.config.move_ordering.killer_moves {
            Some(&self.killers)
        } else {
            None
        };
        
        let history = if self.config.move_ordering.history_heuristic {
            Some(&self.history)
        } else {
            None
        };
        
        let tt_mv = if self.config.move_ordering.tt_move {
            tt_move
        } else {
            None
        };
        
        if self.config.move_ordering.mvv_lva || tt_mv.is_some() || killers.is_some() || history.is_some() {
            order_moves(board, moves, tt_mv, killers, history, self.ply)
        } else {
            // No ordering - just wrap moves
            moves.iter().map(|&mv| MoveScore { mv, score: 0 }).collect()
        }
    }
    
    /// Check if position has non-pawn material (for null move pruning)
    fn has_non_pawn_material(&self, board: &Board) -> bool {
        let us = board.side_to_move();
        let our_pieces = board.occupancy(us);
        let pawns = board.piece_bitboard(us, PieceType::Pawn);
        let kings = board.piece_bitboard(us, PieceType::King);
        
        // Has pieces other than pawns and king
        (our_pieces & !(pawns | kings)).count_ones() > 0
    }
}
