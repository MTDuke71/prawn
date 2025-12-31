// Undo information for move unmake
// REQ-2: Unmake move (restore previous state)
// OPTIMIZED: Copy trait, const constructor for fixed arrays

use crate::board::{Piece, Square};

/// Stores all information needed to unmake a move
/// Requirements Satisfied: REQ-2
/// OPTIMIZED: Copy for fast pass-by-value, const EMPTY for array initialization
#[derive(Debug, Clone, Copy)]
pub struct UndoInfo {
    /// The piece that was captured (if any)
    pub captured_piece: Option<Piece>,
    /// Castling rights before the move
    pub castling_rights: u8,
    /// En passant square before the move
    pub en_passant_square: Option<Square>,
    /// Halfmove clock before the move
    pub halfmove_clock: u32,
    /// Zobrist hash before the move
    pub zobrist_hash: u64,
}

impl UndoInfo {
    /// Empty undo info for array initialization
    pub const EMPTY: UndoInfo = UndoInfo {
        captured_piece: None,
        castling_rights: 0,
        en_passant_square: None,
        halfmove_clock: 0,
        zobrist_hash: 0,
    };

    /// Create new undo information
    #[inline(always)]
    pub fn new(
        captured_piece: Option<Piece>,
        castling_rights: u8,
        en_passant_square: Option<Square>,
        halfmove_clock: u32,
        zobrist_hash: u64,
    ) -> Self {
        UndoInfo {
            captured_piece,
            castling_rights,
            en_passant_square,
            halfmove_clock,
            zobrist_hash,
        }
    }
}
