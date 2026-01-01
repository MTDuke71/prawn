//! Move Ordering
//! 
//! Order moves to maximize alpha-beta cutoffs.
//! Better move ordering = searching fewer nodes.

use crate::board::{Board, PieceType};
use crate::Move;

/// Score for move ordering (higher = search first)
#[derive(Debug, Clone, Copy)]
pub struct MoveScore {
    pub mv: Move,
    pub score: i32,
}

/// Piece values for MVV-LVA ordering
const PIECE_VALUES: [i32; 7] = [
    0,    // None
    100,  // Pawn
    320,  // Knight
    330,  // Bishop
    500,  // Rook
    900,  // Queen
    20000, // King
];

/// MVV-LVA score: captures of valuable pieces by less valuable attackers first
#[inline]
pub fn mvv_lva_score(board: &Board, mv: Move) -> i32 {
    if let Some(captured) = mv.captured() {
        let victim_value = PIECE_VALUES[captured.piece_type() as usize];
        let attacker = board.piece_at(mv.from());
        let attacker_value = attacker
            .map(|p| PIECE_VALUES[p.piece_type() as usize])
            .unwrap_or(100);
        
        // MVV-LVA: maximize victim value, minimize attacker value
        // Score = victim * 10 - attacker (so QxP scores higher than PxQ)
        // Actually: we want PxQ highest, so: victim - attacker/100
        victim_value * 10 - attacker_value
    } else {
        0
    }
}

/// Killer moves storage (2 killers per ply)
pub struct KillerMoves {
    killers: [[Option<Move>; 2]; 128],
}

impl KillerMoves {
    pub fn new() -> Self {
        Self {
            killers: [[None; 2]; 128],
        }
    }
    
    /// Add a killer move at the given ply
    #[inline]
    pub fn add(&mut self, ply: usize, mv: Move) {
        if ply >= 128 {
            return;
        }
        
        // Don't add captures as killers
        if mv.captured().is_some() {
            return;
        }
        
        // Shift killers and add new one
        if self.killers[ply][0] != Some(mv) {
            self.killers[ply][1] = self.killers[ply][0];
            self.killers[ply][0] = Some(mv);
        }
    }
    
    /// Check if a move is a killer at this ply
    #[inline]
    pub fn is_killer(&self, ply: usize, mv: Move) -> bool {
        if ply >= 128 {
            return false;
        }
        self.killers[ply][0] == Some(mv) || self.killers[ply][1] == Some(mv)
    }
    
    /// Clear killers
    pub fn clear(&mut self) {
        self.killers = [[None; 2]; 128];
    }
}

impl Default for KillerMoves {
    fn default() -> Self {
        Self::new()
    }
}

/// History heuristic storage
pub struct HistoryTable {
    // [color][from][to] scores
    table: [[[i32; 64]; 64]; 2],
}

impl HistoryTable {
    pub fn new() -> Self {
        Self {
            table: [[[0; 64]; 64]; 2],
        }
    }
    
    /// Add a history bonus for a move that caused a cutoff
    #[inline]
    pub fn add(&mut self, color: usize, mv: Move, depth: i32) {
        let bonus = depth * depth;
        let from = mv.from() as usize;
        let to = mv.to() as usize;
        
        // Age and add (prevent overflow)
        let current = &mut self.table[color][from][to];
        *current = (*current + bonus).min(100_000);
    }
    
    /// Get history score for a move
    #[inline]
    pub fn get(&self, color: usize, mv: Move) -> i32 {
        self.table[color][mv.from() as usize][mv.to() as usize]
    }
    
    /// Age history scores (call periodically)
    #[allow(dead_code)]
    pub fn age(&mut self) {
        for color in 0..2 {
            for from in 0..64 {
                for to in 0..64 {
                    self.table[color][from][to] /= 2;
                }
            }
        }
    }
    
    /// Clear history
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.table = [[[0; 64]; 64]; 2];
    }
}

impl Default for HistoryTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Order moves for best alpha-beta performance
/// Returns moves sorted by score (highest first)
pub fn order_moves(
    board: &Board,
    moves: &[Move],
    tt_move: Option<Move>,
    killers: Option<&KillerMoves>,
    history: Option<&HistoryTable>,
    ply: usize,
) -> Vec<MoveScore> {
    let color = board.side_to_move() as usize;
    
    let mut scored: Vec<MoveScore> = moves
        .iter()
        .map(|&mv| {
            let mut score = 0i32;
            
            // TT move gets highest priority
            if tt_move == Some(mv) {
                score += 10_000_000;
            }
            
            // Captures: MVV-LVA
            if mv.captured().is_some() {
                score += 1_000_000 + mvv_lva_score(board, mv);
            }
            
            // Promotions
            if mv.promotion().is_some() {
                score += 900_000;
                if mv.promotion() == Some(PieceType::Queen) {
                    score += 50_000;
                }
            }
            
            // Killer moves
            if let Some(k) = killers {
                if k.is_killer(ply, mv) {
                    score += 500_000;
                }
            }
            
            // History heuristic
            if let Some(h) = history {
                score += h.get(color, mv);
            }
            
            MoveScore { mv, score }
        })
        .collect();
    
    // Sort by score descending
    scored.sort_unstable_by(|a, b| b.score.cmp(&a.score));
    
    scored
}

/// Faster: pick the best move without full sort (for when we might get cutoff early)
#[inline]
pub fn pick_best(scored_moves: &mut [MoveScore], start_idx: usize) {
    if start_idx >= scored_moves.len() {
        return;
    }
    
    let mut best_idx = start_idx;
    let mut best_score = scored_moves[start_idx].score;
    
    for i in (start_idx + 1)..scored_moves.len() {
        if scored_moves[i].score > best_score {
            best_score = scored_moves[i].score;
            best_idx = i;
        }
    }
    
    if best_idx != start_idx {
        scored_moves.swap(start_idx, best_idx);
    }
}
