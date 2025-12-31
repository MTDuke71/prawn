//! Material evaluation
//!
//! Basic piece counting with standard piece values.

use crate::board::{Board, Color, PieceType};

// Standard piece values in centipawns
pub const PAWN_VALUE: i32 = 100;
pub const KNIGHT_VALUE: i32 = 320;
pub const BISHOP_VALUE: i32 = 330;
pub const ROOK_VALUE: i32 = 500;
pub const QUEEN_VALUE: i32 = 900;

/// Evaluate material balance.
/// Returns score from the perspective of the side to move.
pub fn evaluate(board: &Board) -> i32 {
    let white_material = count_material(board, Color::White);
    let black_material = count_material(board, Color::Black);
    
    let score = white_material - black_material;
    
    // Return from perspective of side to move
    if board.side_to_move() == Color::White {
        score
    } else {
        -score
    }
}

/// Count total material for one side
fn count_material(board: &Board, color: Color) -> i32 {
    let pawns = board.piece_bitboard(color, PieceType::Pawn).count_ones() as i32;
    let knights = board.piece_bitboard(color, PieceType::Knight).count_ones() as i32;
    let bishops = board.piece_bitboard(color, PieceType::Bishop).count_ones() as i32;
    let rooks = board.piece_bitboard(color, PieceType::Rook).count_ones() as i32;
    let queens = board.piece_bitboard(color, PieceType::Queen).count_ones() as i32;
    
    pawns * PAWN_VALUE
        + knights * KNIGHT_VALUE
        + bishops * BISHOP_VALUE
        + rooks * ROOK_VALUE
        + queens * QUEEN_VALUE
}
