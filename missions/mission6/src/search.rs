//! Search Algorithm
//! 
//! Modular negamax search with alpha-beta pruning and various enhancements.
//! Each feature can be toggled for measuring ELO impact.

use prawn::board::{Board, PieceType};
use prawn::{Evaluator, GameState, Move, MoveGenerator};

use crate::transposition::{TranspositionTable, Bound};
use crate::move_ordering::{order_moves, pick_best, KillerMoves, HistoryTable, MoveScore};

/// Mate score constants
pub const MATE_SCORE: i32 = 30000;
pub const MATE_THRESHOLD: i32 = 29000;

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
    /// Nodes searched
    pub nodes: u64,
    /// Principal variation
    pub pv: Vec<Move>,
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
    pv_table: [[Option<Move>; 128]; 128],
    pv_length: [usize; 128],
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
            pv_table: [[None; 128]; 128],
            pv_length: [0; 128],
        }
    }
    
    /// Search the position to the given depth
    pub fn search(&mut self, game: &mut GameState, depth: u8) -> SearchResult {
        self.nodes = 0;
        self.killers.clear();
        
        if let Some(ref mut tt) = self.tt {
            tt.new_search();
        }
        
        if self.config.iterative_deepening {
            self.iterative_deepening(game, depth)
        } else {
            self.search_root(game, depth)
        }
    }
    
    /// Iterative deepening search
    fn iterative_deepening(&mut self, game: &mut GameState, max_depth: u8) -> SearchResult {
        let mut best_result = SearchResult {
            best_move: None,
            score: 0,
            depth: 0,
            nodes: 0,
            pv: Vec::new(),
        };
        
        for depth in 1..=max_depth {
            let result = self.search_root(game, depth);
            
            // Update best result
            best_result = SearchResult {
                best_move: result.best_move,
                score: result.score,
                depth,
                nodes: self.nodes,
                pv: result.pv,
            };
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
                nodes: self.nodes,
                pv: Vec::new(),
            };
        }
        
        // Get TT move for ordering
        let tt_move = self.tt.as_ref().and_then(|tt| {
            tt.probe_copy(game.zobrist_hash()).and_then(|e| e.best_move)
        });
        
        // Order moves
        let mut scored_moves = self.order_moves(game.board(), moves.moves(), tt_move);
        
        let mut best_move = None;
        let mut best_score = -MATE_SCORE;
        let mut alpha = alpha;
        
        for (i, ms) in scored_moves.iter().enumerate() {
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
            && depth >= 3
            && self.has_non_pawn_material(game.board())
        {
            let old_ep = game.make_null_move();
            self.ply += 1;
            
            let score = -self.alpha_beta(game, depth - 1 - self.config.null_move_r, -beta, -beta + 1, false);
            
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
        let mut moves_searched = 0;
        
        for i in 0..scored_moves.len() {
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
            
            moves_searched += 1;
            
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
    #[allow(dead_code)]
    fn has_non_pawn_material(&self, board: &Board) -> bool {
        let us = board.side_to_move();
        let our_pieces = board.occupancy(us);
        let pawns = board.piece_bitboard(us, PieceType::Pawn);
        let kings = board.piece_bitboard(us, PieceType::King);
        
        // Has pieces other than pawns and king
        (our_pieces & !(pawns | kings)).count_ones() > 0
    }
}
