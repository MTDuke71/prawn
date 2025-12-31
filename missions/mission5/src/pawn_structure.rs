//! Pawn Structure Evaluation
//!
//! Evaluates doubled, isolated, passed, and connected pawns.

use prawn::board::{Board, Color, PieceType, Square, BitboardOps};

// Bonuses and penalties in centipawns
const DOUBLED_PAWN_PENALTY: i32 = -10;
const ISOLATED_PAWN_PENALTY: i32 = -20;
const PASSED_PAWN_BONUS: [i32; 8] = [0, 10, 20, 40, 60, 90, 130, 0]; // By rank
const CONNECTED_PASSER_BONUS: i32 = 20;

/// Evaluate pawn structure.
/// Returns score from the perspective of the side to move.
pub fn evaluate(board: &Board) -> i32 {
    let white_score = evaluate_side(board, Color::White);
    let black_score = evaluate_side(board, Color::Black);
    
    let score = white_score - black_score;
    
    if board.side_to_move() == Color::White {
        score
    } else {
        -score
    }
}

fn evaluate_side(board: &Board, color: Color) -> i32 {
    let mut score = 0i32;
    let our_pawns = board.piece_bitboard(color, PieceType::Pawn);
    let their_pawns = board.piece_bitboard(color.opponent(), PieceType::Pawn);
    
    let mut pawns = our_pawns;
    while pawns != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(pawns);
        pawns = new_bb;
        
        if let Some(square) = sq {
            let file = square.file() as usize;
            let rank = square.rank() as usize;
            
            // Check for doubled pawns (another pawn on same file)
            if is_doubled(our_pawns, square, color) {
                score += DOUBLED_PAWN_PENALTY;
            }
            
            // Check for isolated pawns (no pawns on adjacent files)
            if is_isolated(our_pawns, file) {
                score += ISOLATED_PAWN_PENALTY;
            }
            
            // Check for passed pawns (no enemy pawns in front or adjacent)
            if is_passed(our_pawns, their_pawns, square, color) {
                let bonus_rank = if color == Color::White { rank } else { 7 - rank };
                score += PASSED_PAWN_BONUS[bonus_rank];
                
                // Extra bonus for connected passed pawns
                if is_connected_passer(our_pawns, their_pawns, square, color) {
                    score += CONNECTED_PASSER_BONUS;
                }
            }
        }
    }
    
    score
}

/// Check if a pawn is doubled (another pawn of same color on same file)
fn is_doubled(our_pawns: u64, square: Square, color: Color) -> bool {
    let file_mask = BitboardOps::FILE_A << square.file();
    let pawns_on_file = our_pawns & file_mask;
    let pawn_count = pawns_on_file.count_ones();
    
    if pawn_count <= 1 {
        return false;
    }
    
    // For white, doubled means there's a pawn behind us (lower rank)
    // For black, doubled means there's a pawn behind us (higher rank)
    let sq_bb = 1u64 << square.index();
    match color {
        Color::White => {
            // Check for pawns on lower ranks (behind us)
            let behind_mask = sq_bb - 1;
            (pawns_on_file & behind_mask) != 0
        }
        Color::Black => {
            // Check for pawns on higher ranks (behind us for black)
            let behind_mask = !sq_bb & !(sq_bb - 1);
            (pawns_on_file & behind_mask) != 0
        }
    }
}

/// Check if a pawn is isolated (no friendly pawns on adjacent files)
fn is_isolated(our_pawns: u64, file: usize) -> bool {
    let adjacent_files = get_adjacent_files_mask(file);
    (our_pawns & adjacent_files) == 0
}

/// Check if a pawn is passed (no enemy pawns can block or capture it)
fn is_passed(_our_pawns: u64, their_pawns: u64, square: Square, color: Color) -> bool {
    let file = square.file() as usize;
    let rank = square.rank() as usize;
    
    // Get mask for files the enemy could attack from (same file + adjacent)
    let mut attack_files = BitboardOps::FILE_A << file;
    if file > 0 {
        attack_files |= BitboardOps::FILE_A << (file - 1);
    }
    if file < 7 {
        attack_files |= BitboardOps::FILE_A << (file + 1);
    }
    
    // Get mask for ranks in front of the pawn
    let forward_ranks = match color {
        Color::White => {
            // Ranks above this one
            if rank >= 7 { 0 } else { !((1u64 << ((rank + 1) * 8)) - 1) }
        }
        Color::Black => {
            // Ranks below this one  
            (1u64 << (rank * 8)) - 1
        }
    };
    
    // A pawn is passed if no enemy pawns in forward attacking squares
    (their_pawns & attack_files & forward_ranks) == 0
}

/// Check if a passed pawn has a friendly passer on adjacent file
fn is_connected_passer(our_pawns: u64, their_pawns: u64, square: Square, color: Color) -> bool {
    let file = square.file() as usize;
    let adjacent_files = get_adjacent_files_mask(file);
    
    // Check adjacent files for friendly pawns
    let mut adjacent_pawns = our_pawns & adjacent_files;
    
    while adjacent_pawns != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(adjacent_pawns);
        adjacent_pawns = new_bb;
        
        if let Some(adj_square) = sq {
            // Check if this adjacent pawn is also passed
            if is_passed(our_pawns, their_pawns, adj_square, color) {
                return true;
            }
        }
    }
    
    false
}

/// Get mask of adjacent files
fn get_adjacent_files_mask(file: usize) -> u64 {
    let mut mask = 0u64;
    if file > 0 {
        mask |= BitboardOps::FILE_A << (file - 1);
    }
    if file < 7 {
        mask |= BitboardOps::FILE_A << (file + 1);
    }
    mask
}
