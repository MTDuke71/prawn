//! King Safety Evaluation
//!
//! Evaluates pawn shield and open file exposure.

use crate::board::{Board, Color, PieceType, Square, BitboardOps};

// Bonuses and penalties in centipawns
const PAWN_SHIELD_BONUS: i32 = 10;
const OPEN_FILE_PENALTY: i32 = -20;
const SEMI_OPEN_FILE_PENALTY: i32 = -10;

/// Evaluate king safety.
/// Returns score from the perspective of the side to move.
pub fn evaluate(board: &Board, phase: i32) -> i32 {
    // King safety matters more in middlegame
    let mg_weight = phase;
    
    let white_score = evaluate_side(board, Color::White);
    let black_score = evaluate_side(board, Color::Black);
    
    let raw_score = white_score - black_score;
    
    // Scale by game phase (more important in middlegame)
    let score = (raw_score * mg_weight) / 256;
    
    if board.side_to_move() == Color::White {
        score
    } else {
        -score
    }
}

fn evaluate_side(board: &Board, color: Color) -> i32 {
    let mut score = 0i32;
    
    let king_bb = board.piece_bitboard(color, PieceType::King);
    if king_bb == 0 {
        return 0; // No king (shouldn't happen)
    }
    
    let (_, king_sq_opt) = BitboardOps::pop_bit(king_bb);
    let king_sq: Square = match king_sq_opt {
        Some(sq) => sq,
        None => return 0,
    };
    
    let king_file = king_sq.file() as usize;
    let our_pawns = board.piece_bitboard(color, PieceType::Pawn);
    let their_pawns = board.piece_bitboard(color.opponent(), PieceType::Pawn);
    
    // Evaluate pawn shield (pawns in front of king)
    score += evaluate_pawn_shield(our_pawns, king_sq, color);
    
    // Evaluate open files near king
    score += evaluate_king_file_safety(our_pawns, their_pawns, king_file);
    
    score
}

/// Evaluate pawn shield quality
fn evaluate_pawn_shield(our_pawns: u64, king_sq: Square, color: Color) -> i32 {
    let mut score = 0i32;
    let king_file = king_sq.file() as usize;
    let king_rank = king_sq.rank() as usize;
    
    // Check files around the king (current file and adjacent)
    for file_offset in -1i32..=1 {
        let file = king_file as i32 + file_offset;
        if !(0..=7).contains(&file) {
            continue;
        }
        
        let file_mask = BitboardOps::FILE_A << file;
        let shield_pawns = our_pawns & file_mask;
        
        // For white, shield pawns are on ranks 2-3 in front of king
        // For black, shield pawns are on ranks 6-7 in front of king
        let shield_rank_mask = match color {
            Color::White => {
                if king_rank <= 1 {
                    BitboardOps::RANK_2 | BitboardOps::RANK_3
                } else {
                    0
                }
            }
            Color::Black => {
                if king_rank >= 6 {
                    BitboardOps::RANK_6 | BitboardOps::RANK_7
                } else {
                    0
                }
            }
        };
        
        if (shield_pawns & shield_rank_mask) != 0 {
            score += PAWN_SHIELD_BONUS;
        }
    }
    
    score
}

/// Evaluate open/semi-open files near king
fn evaluate_king_file_safety(our_pawns: u64, their_pawns: u64, king_file: usize) -> i32 {
    let mut score = 0i32;
    
    // Check king's file and adjacent files
    for file_offset in -1i32..=1 {
        let file = king_file as i32 + file_offset;
        if !(0..=7).contains(&file) {
            continue;
        }
        
        let file_mask = BitboardOps::FILE_A << file;
        let our_pawns_on_file = (our_pawns & file_mask) != 0;
        let their_pawns_on_file = (their_pawns & file_mask) != 0;
        
        if !our_pawns_on_file && !their_pawns_on_file {
            // Open file - dangerous!
            score += OPEN_FILE_PENALTY;
        } else if !our_pawns_on_file {
            // Semi-open file (only enemy pawns) - somewhat dangerous
            score += SEMI_OPEN_FILE_PENALTY;
        }
    }
    
    score
}
