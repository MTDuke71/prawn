//! Mobility Evaluation
//!
//! Evaluates piece mobility (number of legal moves available).

use prawn::board::{Board, Color, PieceType, BitboardOps};
use prawn::{AttackTables, MagicTable};

// Mobility bonuses per square available (centipawns)
const KNIGHT_MOBILITY: i32 = 4;
const BISHOP_MOBILITY: i32 = 5;
const ROOK_MOBILITY: i32 = 2;
const QUEEN_MOBILITY: i32 = 1;

/// Evaluate piece mobility.
/// Returns score from the perspective of the side to move.
pub fn evaluate(board: &Board) -> i32 {
    // Create attack tables for computing mobility
    let attack_tables = AttackTables::new();
    let magic_table = MagicTable::new();
    
    let white_score = evaluate_side(board, Color::White, &attack_tables, &magic_table);
    let black_score = evaluate_side(board, Color::Black, &attack_tables, &magic_table);
    
    let score = white_score - black_score;
    
    if board.side_to_move() == Color::White {
        score
    } else {
        -score
    }
}

fn evaluate_side(board: &Board, color: Color, attacks: &AttackTables, magic: &MagicTable) -> i32 {
    let mut score = 0i32;
    
    let occupied = board.all_occupancy();
    let friendly = board.occupancy(color);
    
    // Knight mobility
    let mut knights = board.piece_bitboard(color, PieceType::Knight);
    while knights != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(knights);
        knights = new_bb;
        if let Some(square) = sq {
            let knight_attacks = attacks.knight_attacks(square);
            let mobility = (knight_attacks & !friendly).count_ones() as i32;
            score += mobility * KNIGHT_MOBILITY;
        }
    }
    
    // Bishop mobility
    let mut bishops = board.piece_bitboard(color, PieceType::Bishop);
    while bishops != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(bishops);
        bishops = new_bb;
        if let Some(square) = sq {
            let bishop_attacks = magic.bishop_attacks(square, occupied);
            let mobility = (bishop_attacks & !friendly).count_ones() as i32;
            score += mobility * BISHOP_MOBILITY;
        }
    }
    
    // Rook mobility
    let mut rooks = board.piece_bitboard(color, PieceType::Rook);
    while rooks != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(rooks);
        rooks = new_bb;
        if let Some(square) = sq {
            let rook_attacks = magic.rook_attacks(square, occupied);
            let mobility = (rook_attacks & !friendly).count_ones() as i32;
            score += mobility * ROOK_MOBILITY;
        }
    }
    
    // Queen mobility (combine bishop + rook)
    let mut queens = board.piece_bitboard(color, PieceType::Queen);
    while queens != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(queens);
        queens = new_bb;
        if let Some(square) = sq {
            let queen_attacks = magic.queen_attacks(square, occupied);
            let mobility = (queen_attacks & !friendly).count_ones() as i32;
            score += mobility * QUEEN_MOBILITY;
        }
    }
    
    score
}
