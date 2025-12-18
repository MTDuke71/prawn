// Move representation and encoding

use crate::board::{Piece, PieceType, Square};

/// Represents a chess move
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    data: u32, // Compact move encoding
}

// Move encoding (32 bits):
// Bits 0-5:   from square (0-63)
// Bits 6-11:  to square (0-63)
// Bits 12-15: move type (see MoveType)
// Bits 16-19: captured piece (if any)
// Bits 20-22: promotion piece type (if any)

impl Move {
    /// Create a quiet move (no capture, no special)
    pub fn new_quiet(from: Square, to: Square) -> Self {
        Move {
            data: (from.index() as u32) | ((to.index() as u32) << 6) | (MoveType::Quiet as u32) << 12,
        }
    }

    /// Create a capture move
    pub fn new_capture(from: Square, to: Square, captured: Piece) -> Self {
        Move {
            data: (from.index() as u32)
                | ((to.index() as u32) << 6)
                | ((MoveType::Capture as u32) << 12)
                | ((captured as u32) << 16),
        }
    }

    /// Create a double pawn push
    pub fn new_double_pawn_push(from: Square, to: Square) -> Self {
        Move {
            data: (from.index() as u32)
                | ((to.index() as u32) << 6)
                | ((MoveType::DoublePawnPush as u32) << 12),
        }
    }

    /// Create an en passant capture
    pub fn new_en_passant(from: Square, to: Square) -> Self {
        Move {
            data: (from.index() as u32)
                | ((to.index() as u32) << 6)
                | ((MoveType::EnPassant as u32) << 12),
        }
    }

    /// Create a kingside castle
    pub fn new_kingside_castle(from: Square, to: Square) -> Self {
        Move {
            data: (from.index() as u32)
                | ((to.index() as u32) << 6)
                | ((MoveType::KingsideCastle as u32) << 12),
        }
    }

    /// Create a queenside castle
    pub fn new_queenside_castle(from: Square, to: Square) -> Self {
        Move {
            data: (from.index() as u32)
                | ((to.index() as u32) << 6)
                | ((MoveType::QueensideCastle as u32) << 12),
        }
    }

    /// Create a promotion
    pub fn new_promotion(from: Square, to: Square, promotion: PieceType) -> Self {
        Move {
            data: (from.index() as u32)
                | ((to.index() as u32) << 6)
                | ((MoveType::Promotion as u32) << 12)
                | ((piece_type_to_u32(promotion)) << 20),
        }
    }

    /// Create a capture promotion
    pub fn new_capture_promotion(from: Square, to: Square, captured: Piece, promotion: PieceType) -> Self {
        Move {
            data: (from.index() as u32)
                | ((to.index() as u32) << 6)
                | ((MoveType::CapturePromotion as u32) << 12)
                | ((captured as u32) << 16)
                | ((piece_type_to_u32(promotion)) << 20),
        }
    }

    /// Get from square
    pub fn from(&self) -> Square {
        Square::from_index((self.data & 0x3F) as u8).unwrap()
    }

    /// Get to square
    pub fn to(&self) -> Square {
        Square::from_index(((self.data >> 6) & 0x3F) as u8).unwrap()
    }

    /// Get move type
    pub fn move_type(&self) -> MoveType {
        unsafe { std::mem::transmute::<u8, MoveType>(((self.data >> 12) & 0xF) as u8) }
    }

    /// Get captured piece (if any)
    pub fn captured(&self) -> Option<Piece> {
        if self.is_capture() {
            Some(unsafe { std::mem::transmute::<u8, Piece>(((self.data >> 16) & 0xF) as u8) })
        } else {
            None
        }
    }

    /// Get promotion piece type (if any)
    pub fn promotion(&self) -> Option<PieceType> {
        if self.is_promotion() {
            Some(u32_to_piece_type((self.data >> 20) & 0x7))
        } else {
            None
        }
    }

    /// Check if move is a capture
    pub fn is_capture(&self) -> bool {
        matches!(
            self.move_type(),
            MoveType::Capture | MoveType::EnPassant | MoveType::CapturePromotion
        )
    }

    /// Check if move is a promotion
    pub fn is_promotion(&self) -> bool {
        matches!(
            self.move_type(),
            MoveType::Promotion | MoveType::CapturePromotion
        )
    }

    /// Check if move is a castling move
    pub fn is_castle(&self) -> bool {
        matches!(
            self.move_type(),
            MoveType::KingsideCastle | MoveType::QueensideCastle
        )
    }

    /// Convert move to UCI notation (e.g., "e2e4", "e7e8q")
    pub fn to_uci(&self) -> String {
        let from_file = (b'a' + self.from().file()) as char;
        let from_rank = (b'1' + self.from().rank()) as char;
        let to_file = (b'a' + self.to().file()) as char;
        let to_rank = (b'1' + self.to().rank()) as char;

        let mut uci = format!("{}{}{}{}", from_file, from_rank, to_file, to_rank);

        if let Some(promo) = self.promotion() {
            uci.push(match promo {
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                _ => ' ',
            });
        }

        uci
    }
}

/// Move type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MoveType {
    Quiet = 0,
    DoublePawnPush = 1,
    KingsideCastle = 2,
    QueensideCastle = 3,
    Capture = 4,
    EnPassant = 5,
    Promotion = 6,
    CapturePromotion = 7,
}

fn piece_type_to_u32(pt: PieceType) -> u32 {
    match pt {
        PieceType::Knight => 0,
        PieceType::Bishop => 1,
        PieceType::Rook => 2,
        PieceType::Queen => 3,
        _ => 0,
    }
}

fn u32_to_piece_type(val: u32) -> PieceType {
    match val {
        0 => PieceType::Knight,
        1 => PieceType::Bishop,
        2 => PieceType::Rook,
        3 => PieceType::Queen,
        _ => PieceType::Knight,
    }
}

/// Move list with pre-allocated capacity
pub struct MoveList {
    moves: Vec<Move>,
}

impl MoveList {
    /// Create new move list with capacity for typical position
    pub fn new() -> Self {
        MoveList {
            moves: Vec::with_capacity(218), // Max possible moves in a position
        }
    }

    /// Add a move to the list
    pub fn push(&mut self, m: Move) {
        self.moves.push(m);
    }

    /// Get all moves
    pub fn moves(&self) -> &[Move] {
        &self.moves
    }

    /// Get number of moves
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    /// Clear the list
    pub fn clear(&mut self) {
        self.moves.clear();
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiet_move() {
        let m = Move::new_quiet(Square::E2, Square::E4);
        assert_eq!(m.from(), Square::E2);
        assert_eq!(m.to(), Square::E4);
        assert_eq!(m.move_type(), MoveType::Quiet);
        assert!(!m.is_capture());
        assert_eq!(m.to_uci(), "e2e4");
    }

    #[test]
    fn test_capture_move() {
        let m = Move::new_capture(Square::E4, Square::D5, Piece::BlackPawn);
        assert_eq!(m.from(), Square::E4);
        assert_eq!(m.to(), Square::D5);
        assert_eq!(m.move_type(), MoveType::Capture);
        assert!(m.is_capture());
        assert_eq!(m.captured(), Some(Piece::BlackPawn));
    }

    #[test]
    fn test_promotion_move() {
        let m = Move::new_promotion(Square::E7, Square::E8, PieceType::Queen);
        assert_eq!(m.from(), Square::E7);
        assert_eq!(m.to(), Square::E8);
        assert!(m.is_promotion());
        assert_eq!(m.promotion(), Some(PieceType::Queen));
        assert_eq!(m.to_uci(), "e7e8q");
    }

    #[test]
    fn test_castle_move() {
        let m = Move::new_kingside_castle(Square::E1, Square::G1);
        assert!(m.is_castle());
        assert_eq!(m.move_type(), MoveType::KingsideCastle);
    }
}
