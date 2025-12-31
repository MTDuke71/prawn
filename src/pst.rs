//! Piece-Square Tables (PST)
//!
//! Position-dependent piece values that encourage good piece placement.

use crate::board::{Board, Color, PieceType, Square, BitboardOps};

/// Evaluate piece-square tables.
/// Returns score from the perspective of the side to move.
pub fn evaluate(board: &Board, phase: i32) -> i32 {
    let white_score = evaluate_side(board, Color::White, phase);
    let black_score = evaluate_side(board, Color::Black, phase);
    
    let score = white_score - black_score;
    
    if board.side_to_move() == Color::White {
        score
    } else {
        -score
    }
}

fn evaluate_side(board: &Board, color: Color, phase: i32) -> i32 {
    let mut mg_score = 0i32;
    let mut eg_score = 0i32;
    
    // Pawns
    let mut pawns = board.piece_bitboard(color, PieceType::Pawn);
    while pawns != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(pawns);
        pawns = new_bb;
        if let Some(square) = sq {
            let idx = pst_index(square, color);
            mg_score += PAWN_MG[idx];
            eg_score += PAWN_EG[idx];
        }
    }
    
    // Knights
    let mut knights = board.piece_bitboard(color, PieceType::Knight);
    while knights != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(knights);
        knights = new_bb;
        if let Some(square) = sq {
            let idx = pst_index(square, color);
            mg_score += KNIGHT_MG[idx];
            eg_score += KNIGHT_EG[idx];
        }
    }
    
    // Bishops
    let mut bishops = board.piece_bitboard(color, PieceType::Bishop);
    while bishops != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(bishops);
        bishops = new_bb;
        if let Some(square) = sq {
            let idx = pst_index(square, color);
            mg_score += BISHOP_MG[idx];
            eg_score += BISHOP_EG[idx];
        }
    }
    
    // Rooks
    let mut rooks = board.piece_bitboard(color, PieceType::Rook);
    while rooks != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(rooks);
        rooks = new_bb;
        if let Some(square) = sq {
            let idx = pst_index(square, color);
            mg_score += ROOK_MG[idx];
            eg_score += ROOK_EG[idx];
        }
    }
    
    // Queens
    let mut queens = board.piece_bitboard(color, PieceType::Queen);
    while queens != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(queens);
        queens = new_bb;
        if let Some(square) = sq {
            let idx = pst_index(square, color);
            mg_score += QUEEN_MG[idx];
            eg_score += QUEEN_EG[idx];
        }
    }
    
    // Kings
    let mut kings = board.piece_bitboard(color, PieceType::King);
    while kings != 0 {
        let (new_bb, sq) = BitboardOps::pop_bit(kings);
        kings = new_bb;
        if let Some(square) = sq {
            let idx = pst_index(square, color);
            mg_score += KING_MG[idx];
            eg_score += KING_EG[idx];
        }
    }
    
    // Interpolate between middlegame and endgame
    // phase: 256 = opening, 0 = endgame
    ((mg_score * phase) + (eg_score * (256 - phase))) / 256
}

/// Convert square to PST index, flipping for black
fn pst_index(square: Square, color: Color) -> usize {
    let idx = square.index();
    match color {
        // For white, flip vertically because tables are written with rank 8 at top
        Color::White => idx ^ 56,
        // For black, use direct index (already flipped from white's perspective)
        Color::Black => idx,
    }
}

// Piece-square tables from white's perspective
// Values are in centipawns, indexed a1=0, h1=7, a2=8, etc.

#[rustfmt::skip]
const PAWN_MG: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
    50,  50,  50,  50,  50,  50,  50,  50,
    10,  10,  20,  30,  30,  20,  10,  10,
     5,   5,  10,  25,  25,  10,   5,   5,
     0,   0,   0,  20,  20,   0,   0,   0,
     5,  -5, -10,   0,   0, -10,  -5,   5,
     5,  10,  10, -20, -20,  10,  10,   5,
     0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
const PAWN_EG: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
    80,  80,  80,  80,  80,  80,  80,  80,
    50,  50,  50,  50,  50,  50,  50,  50,
    30,  30,  30,  30,  30,  30,  30,  30,
    20,  20,  20,  20,  20,  20,  20,  20,
    10,  10,  10,  10,  10,  10,  10,  10,
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
const KNIGHT_MG: [i32; 64] = [
   -50, -40, -30, -30, -30, -30, -40, -50,
   -40, -20,   0,   0,   0,   0, -20, -40,
   -30,   0,  10,  15,  15,  10,   0, -30,
   -30,   5,  15,  20,  20,  15,   5, -30,
   -30,   0,  15,  20,  20,  15,   0, -30,
   -30,   5,  10,  15,  15,  10,   5, -30,
   -40, -20,   0,   5,   5,   0, -20, -40,
   -50, -40, -30, -30, -30, -30, -40, -50,
];

#[rustfmt::skip]
const KNIGHT_EG: [i32; 64] = [
   -50, -40, -30, -30, -30, -30, -40, -50,
   -40, -20,   0,   0,   0,   0, -20, -40,
   -30,   0,  10,  15,  15,  10,   0, -30,
   -30,   5,  15,  20,  20,  15,   5, -30,
   -30,   0,  15,  20,  20,  15,   0, -30,
   -30,   5,  10,  15,  15,  10,   5, -30,
   -40, -20,   0,   5,   5,   0, -20, -40,
   -50, -40, -30, -30, -30, -30, -40, -50,
];

#[rustfmt::skip]
const BISHOP_MG: [i32; 64] = [
   -20, -10, -10, -10, -10, -10, -10, -20,
   -10,   0,   0,   0,   0,   0,   0, -10,
   -10,   0,   5,  10,  10,   5,   0, -10,
   -10,   5,   5,  10,  10,   5,   5, -10,
   -10,   0,  10,  10,  10,  10,   0, -10,
   -10,  10,  10,  10,  10,  10,  10, -10,
   -10,   5,   0,   0,   0,   0,   5, -10,
   -20, -10, -10, -10, -10, -10, -10, -20,
];

#[rustfmt::skip]
const BISHOP_EG: [i32; 64] = [
   -20, -10, -10, -10, -10, -10, -10, -20,
   -10,   0,   0,   0,   0,   0,   0, -10,
   -10,   0,   5,  10,  10,   5,   0, -10,
   -10,   5,   5,  10,  10,   5,   5, -10,
   -10,   0,  10,  10,  10,  10,   0, -10,
   -10,  10,  10,  10,  10,  10,  10, -10,
   -10,   5,   0,   0,   0,   0,   5, -10,
   -20, -10, -10, -10, -10, -10, -10, -20,
];

#[rustfmt::skip]
const ROOK_MG: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
     5,  10,  10,  10,  10,  10,  10,   5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
     0,   0,   0,   5,   5,   0,   0,   0,
];

#[rustfmt::skip]
const ROOK_EG: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
     5,  10,  10,  10,  10,  10,  10,   5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
     0,   0,   0,   5,   5,   0,   0,   0,
];

#[rustfmt::skip]
const QUEEN_MG: [i32; 64] = [
   -20, -10, -10,  -5,  -5, -10, -10, -20,
   -10,   0,   0,   0,   0,   0,   0, -10,
   -10,   0,   5,   5,   5,   5,   0, -10,
    -5,   0,   5,   5,   5,   5,   0,  -5,
     0,   0,   5,   5,   5,   5,   0,  -5,
   -10,   5,   5,   5,   5,   5,   0, -10,
   -10,   0,   5,   0,   0,   0,   0, -10,
   -20, -10, -10,  -5,  -5, -10, -10, -20,
];

#[rustfmt::skip]
const QUEEN_EG: [i32; 64] = [
   -20, -10, -10,  -5,  -5, -10, -10, -20,
   -10,   0,   0,   0,   0,   0,   0, -10,
   -10,   0,   5,   5,   5,   5,   0, -10,
    -5,   0,   5,   5,   5,   5,   0,  -5,
     0,   0,   5,   5,   5,   5,   0,  -5,
   -10,   5,   5,   5,   5,   5,   0, -10,
   -10,   0,   5,   0,   0,   0,   0, -10,
   -20, -10, -10,  -5,  -5, -10, -10, -20,
];

#[rustfmt::skip]
const KING_MG: [i32; 64] = [
   -30, -40, -40, -50, -50, -40, -40, -30,
   -30, -40, -40, -50, -50, -40, -40, -30,
   -30, -40, -40, -50, -50, -40, -40, -30,
   -30, -40, -40, -50, -50, -40, -40, -30,
   -20, -30, -30, -40, -40, -30, -30, -20,
   -10, -20, -20, -20, -20, -20, -20, -10,
    20,  20,   0,   0,   0,   0,  20,  20,
    20,  30,  10,   0,   0,  10,  30,  20,
];

#[rustfmt::skip]
const KING_EG: [i32; 64] = [
   -50, -40, -30, -20, -20, -30, -40, -50,
   -30, -20, -10,   0,   0, -10, -20, -30,
   -30, -10,  20,  30,  30,  20, -10, -30,
   -30, -10,  30,  40,  40,  30, -10, -30,
   -30, -10,  30,  40,  40,  30, -10, -30,
   -30, -10,  20,  30,  30,  20, -10, -30,
   -30, -30,   0,   0,   0,   0, -30, -30,
   -50, -30, -30, -30, -30, -30, -30, -50,
];
