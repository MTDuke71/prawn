// Zobrist hashing for position identification
// REQ-6: Zobrist hashing for position identification
// OPTIMIZED: Static tables, no allocation, O(1) operations

use crate::board::{Board, Color, Piece, PieceType, Square};
use std::sync::LazyLock;

/// Static Zobrist tables - initialized once, shared everywhere
/// No copying, no allocation overhead
pub static ZOBRIST: LazyLock<ZobristTables> = LazyLock::new(ZobristTables::init);

/// Pre-computed Zobrist random numbers
pub struct ZobristTables {
    // Random numbers for each piece on each square: [piece][square]
    // piece index: 0-5 = White P,N,B,R,Q,K; 6-11 = Black P,N,B,R,Q,K
    pub piece_square: [[u64; 64]; 12],
    pub side_to_move: u64,
    pub castling: [u64; 16],
    pub en_passant: [u64; 8],
}

impl ZobristTables {
    /// Initialize Zobrist tables with deterministic PRNG
    fn init() -> Self {
        // Simple xorshift64 PRNG for deterministic initialization
        let mut seed: u64 = 0x98f107c5a6b3d2e1; // Fixed seed
        let mut next_rand = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };

        let mut tables = ZobristTables {
            piece_square: [[0; 64]; 12],
            side_to_move: 0,
            castling: [0; 16],
            en_passant: [0; 8],
        };

        // Initialize piece-square random numbers
        for piece in 0..12 {
            for square in 0..64 {
                tables.piece_square[piece][square] = next_rand();
            }
        }

        tables.side_to_move = next_rand();

        for i in 0..16 {
            tables.castling[i] = next_rand();
        }

        for i in 0..8 {
            tables.en_passant[i] = next_rand();
        }

        tables
    }

    /// Get piece index (0-11) for Zobrist lookup
    #[inline(always)]
    pub fn piece_index(piece: Piece) -> usize {
        let color_offset = if piece.color() == Color::White { 0 } else { 6 };
        let piece_offset = match piece.piece_type() {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        };
        color_offset + piece_offset
    }

    /// Hash a piece on a square
    #[inline(always)]
    pub fn hash_piece(&self, piece: Piece, square: Square) -> u64 {
        self.piece_square[Self::piece_index(piece)][square.index()]
    }

    /// Hash the board from scratch
    pub fn hash_board(&self, board: &Board) -> u64 {
        let mut hash = 0u64;

        // Hash all pieces using mailbox for speed
        for sq in 0..64 {
            if let Some(square) = Square::from_index(sq) {
                if let Some(piece) = board.piece_at(square) {
                    hash ^= self.hash_piece(piece, square);
                }
            }
        }

        // Hash side to move
        if board.side_to_move() == Color::Black {
            hash ^= self.side_to_move;
        }

        // Hash castling rights
        hash ^= self.castling[board.castling_rights() as usize];

        // Hash en passant
        if let Some(ep_square) = board.en_passant_square() {
            hash ^= self.en_passant[ep_square.file() as usize];
        }

        hash
    }
}

// Keep ZobristHasher for API compatibility but it just wraps the static tables
#[derive(Clone)]
pub struct ZobristHasher;

impl ZobristHasher {
    #[inline(always)]
    pub fn new() -> Self {
        ZobristHasher
    }

    #[inline(always)]
    pub fn hash_board(&self, board: &Board) -> u64 {
        ZOBRIST.hash_board(board)
    }

    #[inline(always)]
    pub fn piece_square_hash(&self, piece: Piece, square: Square) -> u64 {
        ZOBRIST.hash_piece(piece, square)
    }

    #[inline(always)]
    pub fn side_to_move_hash(&self) -> u64 {
        ZOBRIST.side_to_move
    }

    #[inline(always)]
    pub fn castling_hash(&self, board: &Board) -> u64 {
        ZOBRIST.castling[board.castling_rights() as usize]
    }

    #[inline(always)]
    pub fn en_passant_hash(&self, file: u8) -> u64 {
        ZOBRIST.en_passant[file as usize]
    }
}

impl Default for ZobristHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zobrist_deterministic() {
        let board = Board::starting_position();
        let hash1 = ZOBRIST.hash_board(&board);
        let hash2 = ZOBRIST.hash_board(&board);
        assert_eq!(hash1, hash2, "Same position should have same hash");
    }

    #[test]
    fn test_zobrist_different_positions() {
        let board1 = Board::starting_position();
        let board2 = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
            .expect("Valid FEN");

        let hash1 = ZOBRIST.hash_board(&board1);
        let hash2 = ZOBRIST.hash_board(&board2);

        assert_ne!(
            hash1, hash2,
            "Different positions should have different hashes"
        );
    }
}
