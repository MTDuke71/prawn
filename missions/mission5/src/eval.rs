//! Core evaluation structures and the main Evaluator

use prawn::board::{Board, Color, PieceType};
use crate::{material, pst, pawn_structure, king_safety, mobility, center_control};

/// Configuration for which evaluation features are enabled.
/// Each feature can be toggled independently to measure its strength contribution.
#[derive(Debug, Clone, Copy)]
pub struct EvalConfig {
    pub material: bool,
    pub piece_square_tables: bool,
    pub pawn_structure: bool,
    pub king_safety: bool,
    pub mobility: bool,
    pub center_control: bool,
    pub tapered_eval: bool,
}

impl EvalConfig {
    /// All features disabled
    pub const NONE: EvalConfig = EvalConfig {
        material: false,
        piece_square_tables: false,
        pawn_structure: false,
        king_safety: false,
        mobility: false,
        center_control: false,
        tapered_eval: false,
    };
    
    /// All features enabled
    pub const ALL: EvalConfig = EvalConfig {
        material: true,
        piece_square_tables: true,
        pawn_structure: true,
        king_safety: true,
        mobility: true,
        center_control: true,
        tapered_eval: true,
    };
    
    /// Only material counting
    pub const MATERIAL_ONLY: EvalConfig = EvalConfig {
        material: true,
        piece_square_tables: false,
        pawn_structure: false,
        king_safety: false,
        mobility: false,
        center_control: false,
        tapered_eval: false,
    };
    
    /// Only piece-square tables
    pub const PST_ONLY: EvalConfig = EvalConfig {
        material: false,
        piece_square_tables: true,
        pawn_structure: false,
        king_safety: false,
        mobility: false,
        center_control: false,
        tapered_eval: false,
    };
    
    /// Only pawn structure evaluation
    pub const PAWN_STRUCTURE_ONLY: EvalConfig = EvalConfig {
        material: false,
        piece_square_tables: false,
        pawn_structure: true,
        king_safety: false,
        mobility: false,
        center_control: false,
        tapered_eval: false,
    };
    
    /// Only king safety evaluation
    pub const KING_SAFETY_ONLY: EvalConfig = EvalConfig {
        material: false,
        piece_square_tables: false,
        pawn_structure: false,
        king_safety: true,
        mobility: false,
        center_control: false,
        tapered_eval: false,
    };
    
    /// Only mobility evaluation
    pub const MOBILITY_ONLY: EvalConfig = EvalConfig {
        material: false,
        piece_square_tables: false,
        pawn_structure: false,
        king_safety: false,
        mobility: true,
        center_control: false,
        tapered_eval: false,
    };
    
    /// Only center control evaluation
    pub const CENTER_CONTROL_ONLY: EvalConfig = EvalConfig {
        material: false,
        piece_square_tables: false,
        pawn_structure: false,
        king_safety: false,
        mobility: false,
        center_control: true,
        tapered_eval: false,
    };
}

/// Breakdown of evaluation score by component.
/// Useful for debugging and understanding position assessment.
#[derive(Debug, Clone, Copy, Default)]
pub struct EvalBreakdown {
    pub material: i32,
    pub piece_square: i32,
    pub pawn_structure: i32,
    pub king_safety: i32,
    pub mobility: i32,
    pub center_control: i32,
    pub total: i32,
}

/// The main evaluator that computes position scores.
#[derive(Debug, Clone)]
pub struct Evaluator {
    config: EvalConfig,
}

impl Evaluator {
    /// Create a new evaluator with the given configuration.
    pub fn new(config: EvalConfig) -> Self {
        Self { config }
    }
    
    /// Evaluate the position and return a score in centipawns.
    /// Positive scores favor the side to move.
    pub fn evaluate(&self, board: &Board) -> i32 {
        self.evaluate_breakdown(board).total
    }
    
    /// Evaluate the position and return a detailed breakdown.
    pub fn evaluate_breakdown(&self, board: &Board) -> EvalBreakdown {
        let mut breakdown = EvalBreakdown::default();
        let phase = if self.config.tapered_eval {
            self.game_phase(board)
        } else {
            128 // Midgame value
        };
        
        if self.config.material {
            breakdown.material = material::evaluate(board);
        }
        
        if self.config.piece_square_tables {
            breakdown.piece_square = pst::evaluate(board, phase);
        }
        
        if self.config.pawn_structure {
            breakdown.pawn_structure = pawn_structure::evaluate(board);
        }
        
        if self.config.king_safety {
            breakdown.king_safety = king_safety::evaluate(board, phase);
        }
        
        if self.config.mobility {
            breakdown.mobility = mobility::evaluate(board);
        }
        
        if self.config.center_control {
            breakdown.center_control = center_control::evaluate(board);
        }
        
        breakdown.total = breakdown.material
            + breakdown.piece_square
            + breakdown.pawn_structure
            + breakdown.king_safety
            + breakdown.mobility
            + breakdown.center_control;
        
        breakdown
    }
    
    /// Calculate the game phase (0 = endgame, 256 = opening).
    /// Used for tapered evaluation.
    pub fn game_phase(&self, board: &Board) -> i32 {
        // Phase values for each piece type
        const PAWN_PHASE: i32 = 0;
        const KNIGHT_PHASE: i32 = 1;
        const BISHOP_PHASE: i32 = 1;
        const ROOK_PHASE: i32 = 2;
        const QUEEN_PHASE: i32 = 4;
        
        // Total phase at game start
        const TOTAL_PHASE: i32 = PAWN_PHASE * 16 
            + KNIGHT_PHASE * 4 
            + BISHOP_PHASE * 4 
            + ROOK_PHASE * 4 
            + QUEEN_PHASE * 2;
        
        let mut phase = TOTAL_PHASE;
        
        // Count pieces and subtract from total phase
        let white_knights = board.piece_bitboard(Color::White, PieceType::Knight).count_ones() as i32;
        let black_knights = board.piece_bitboard(Color::Black, PieceType::Knight).count_ones() as i32;
        let white_bishops = board.piece_bitboard(Color::White, PieceType::Bishop).count_ones() as i32;
        let black_bishops = board.piece_bitboard(Color::Black, PieceType::Bishop).count_ones() as i32;
        let white_rooks = board.piece_bitboard(Color::White, PieceType::Rook).count_ones() as i32;
        let black_rooks = board.piece_bitboard(Color::Black, PieceType::Rook).count_ones() as i32;
        let white_queens = board.piece_bitboard(Color::White, PieceType::Queen).count_ones() as i32;
        let black_queens = board.piece_bitboard(Color::Black, PieceType::Queen).count_ones() as i32;
        
        phase -= (white_knights + black_knights) * KNIGHT_PHASE;
        phase -= (white_bishops + black_bishops) * BISHOP_PHASE;
        phase -= (white_rooks + black_rooks) * ROOK_PHASE;
        phase -= (white_queens + black_queens) * QUEEN_PHASE;
        
        // Normalize to 0-256 range
        // 256 = opening (all pieces), 0 = endgame (no pieces)
        ((TOTAL_PHASE - phase) * 256) / TOTAL_PHASE
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new(EvalConfig::ALL)
    }
}
