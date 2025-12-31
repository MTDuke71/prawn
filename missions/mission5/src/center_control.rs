//! Center Control Evaluation
//!
//! Evaluates control of the central squares (d4, d5, e4, e5).

use prawn::board::{Board, Color, PieceType, Square, BitboardOps};
use prawn::{AttackTables, MagicTable};

// Center squares: d4, e4, d5, e5
const CENTER_SQUARES: u64 = 
    (1u64 << Square::D4.index()) |
    (1u64 << Square::E4.index()) |
    (1u64 << Square::D5.index()) |
    (1u64 << Square::E5.index());

// Extended center: c3-f3, c4-f4, c5-f5, c6-f6
const EXTENDED_CENTER: u64 = 
    (1u64 << Square::C3.index()) |
    (1u64 << Square::D3.index()) |
    (1u64 << Square::E3.index()) |
    (1u64 << Square::F3.index()) |
    (1u64 << Square::C4.index()) |
    (1u64 << Square::D4.index()) |
    (1u64 << Square::E4.index()) |
    (1u64 << Square::F4.index()) |
    (1u64 << Square::C5.index()) |
    (1u64 << Square::D5.index()) |
    (1u64 << Square::E5.index()) |
    (1u64 << Square::F5.index()) |
    (1u64 << Square::C6.index()) |
    (1u64 << Square::D6.index()) |
    (1u64 << Square::E6.index()) |
    (1u64 << Square::F6.index());

// Bonuses in centipawns
const PAWN_CENTER_BONUS: i32 = 20;
const PAWN_EXTENDED_CENTER_BONUS: i32 = 10;
const PIECE_CENTER_ATTACK_BONUS: i32 = 5;

/// Evaluate center control.
/// Returns score from the perspective of the side to move.
pub fn evaluate(board: &Board) -> i32 {
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
    
    // Pawns in center
    let pawns = board.piece_bitboard(color, PieceType::Pawn);
    let pawns_in_center = (pawns & CENTER_SQUARES).count_ones() as i32;
    let pawns_in_extended = (pawns & EXTENDED_CENTER & !CENTER_SQUARES).count_ones() as i32;
    
    score += pawns_in_center * PAWN_CENTER_BONUS;
    score += pawns_in_extended * PAWN_EXTENDED_CENTER_BONUS;
    
    // Pieces attacking center
    let center_attacks = count_center_attacks(board, color, occupied, attacks, magic);
    score += center_attacks * PIECE_CENTER_ATTACK_BONUS;
    
    score
}

/// Count how many times our pieces attack the center
fn count_center_attacks(board: &Board, color: Color, occupied: u64, attacks: &AttackTables, magic: &MagicTable) -> i32 {
    let mut attack_count = 0i32;
    
    // Knights attacking center
    let mut knights = board.piece_bitboard(color, PieceType::Knight);
    while knights != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(knights);
        knights = new_bb;
        if let Some(square) = sq {
            let knight_attacks = attacks.knight_attacks(square);
            attack_count += (knight_attacks & CENTER_SQUARES).count_ones() as i32;
        }
    }
    
    // Bishops attacking center
    let mut bishops = board.piece_bitboard(color, PieceType::Bishop);
    while bishops != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(bishops);
        bishops = new_bb;
        if let Some(square) = sq {
            let bishop_attacks = magic.bishop_attacks(square, occupied);
            attack_count += (bishop_attacks & CENTER_SQUARES).count_ones() as i32;
        }
    }
    
    // Rooks attacking center (less important)
    let mut rooks = board.piece_bitboard(color, PieceType::Rook);
    while rooks != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(rooks);
        rooks = new_bb;
        if let Some(square) = sq {
            let rook_attacks = magic.rook_attacks(square, occupied);
            attack_count += (rook_attacks & CENTER_SQUARES).count_ones() as i32 / 2; // Half value for rooks
        }
    }
    
    // Queens attacking center
    let mut queens = board.piece_bitboard(color, PieceType::Queen);
    while queens != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(queens);
        queens = new_bb;
        if let Some(square) = sq {
            let queen_attacks = magic.queen_attacks(square, occupied);
            attack_count += (queen_attacks & CENTER_SQUARES).count_ones() as i32 / 2; // Half value
        }
    }
    
    attack_count
}
