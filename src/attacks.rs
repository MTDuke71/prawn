// Pre-computed attack tables for non-sliding pieces

use crate::board::{Color, Square};

/// Attack tables for all piece types
pub struct AttackTables {
    pub pawn_attacks: [[u64; 64]; 2], // [color][square]
    pub knight_attacks: [u64; 64],
    pub king_attacks: [u64; 64],
}

impl AttackTables {
    /// Initialize all attack tables
    pub fn new() -> Self {
        let mut tables = AttackTables {
            pawn_attacks: [[0; 64]; 2],
            knight_attacks: [0; 64],
            king_attacks: [0; 64],
        };

        // Initialize pawn attacks
        for square in 0..64 {
            if let Some(sq) = Square::from_index(square) {
                tables.pawn_attacks[Color::White as usize][square as usize] =
                    generate_pawn_attacks(sq, Color::White);
                tables.pawn_attacks[Color::Black as usize][square as usize] =
                    generate_pawn_attacks(sq, Color::Black);
            }
        }

        // Initialize knight attacks
        for square in 0..64 {
            if let Some(sq) = Square::from_index(square) {
                tables.knight_attacks[square as usize] = generate_knight_attacks(sq);
            }
        }

        // Initialize king attacks
        for square in 0..64 {
            if let Some(sq) = Square::from_index(square) {
                tables.king_attacks[square as usize] = generate_king_attacks(sq);
            }
        }

        tables
    }

    /// Get pawn attacks for a square and color
    #[inline(always)]
    pub fn pawn_attacks(&self, square: Square, color: Color) -> u64 {
        self.pawn_attacks[color as usize][square.index()]
    }

    /// Get knight attacks for a square
    #[inline(always)]
    pub fn knight_attacks(&self, square: Square) -> u64 {
        self.knight_attacks[square.index()]
    }

    /// Get king attacks for a square
    #[inline(always)]
    pub fn king_attacks(&self, square: Square) -> u64 {
        self.king_attacks[square.index()]
    }
}

impl Default for AttackTables {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate pawn attack bitboard for a square
fn generate_pawn_attacks(square: Square, color: Color) -> u64 {
    let mut attacks = 0u64;
    let rank = square.rank() as i8;
    let file = square.file() as i8;

    match color {
        Color::White => {
            // White pawns attack diagonally upward
            if rank < 7 {
                // North-west
                if file > 0 {
                    attacks |= 1u64 << ((rank + 1) * 8 + (file - 1));
                }
                // North-east
                if file < 7 {
                    attacks |= 1u64 << ((rank + 1) * 8 + (file + 1));
                }
            }
        }
        Color::Black => {
            // Black pawns attack diagonally downward
            if rank > 0 {
                // South-west
                if file > 0 {
                    attacks |= 1u64 << ((rank - 1) * 8 + (file - 1));
                }
                // South-east
                if file < 7 {
                    attacks |= 1u64 << ((rank - 1) * 8 + (file + 1));
                }
            }
        }
    }

    attacks
}

/// Generate knight attack bitboard for a square
fn generate_knight_attacks(square: Square) -> u64 {
    let mut attacks = 0u64;
    let rank = square.rank() as i8;
    let file = square.file() as i8;

    // All 8 possible knight moves (L-shaped)
    let knight_moves = [
        (2, 1),   // Up 2, Right 1
        (2, -1),  // Up 2, Left 1
        (-2, 1),  // Down 2, Right 1
        (-2, -1), // Down 2, Left 1
        (1, 2),   // Up 1, Right 2
        (1, -2),  // Up 1, Left 2
        (-1, 2),  // Down 1, Right 2
        (-1, -2), // Down 1, Left 2
    ];

    for (rank_offset, file_offset) in knight_moves.iter() {
        let new_rank = rank + rank_offset;
        let new_file = file + file_offset;

        if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
            attacks |= 1u64 << (new_rank * 8 + new_file);
        }
    }

    attacks
}

/// Generate king attack bitboard for a square
fn generate_king_attacks(square: Square) -> u64 {
    let mut attacks = 0u64;
    let rank = square.rank() as i8;
    let file = square.file() as i8;

    // All 8 possible king moves (adjacent squares)
    let king_moves = [
        (1, 0),   // North
        (1, 1),   // North-East
        (0, 1),   // East
        (-1, 1),  // South-East
        (-1, 0),  // South
        (-1, -1), // South-West
        (0, -1),  // West
        (1, -1),  // North-West
    ];

    for (rank_offset, file_offset) in king_moves.iter() {
        let new_rank = rank + rank_offset;
        let new_file = file + file_offset;

        if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
            attacks |= 1u64 << (new_rank * 8 + new_file);
        }
    }

    attacks
}

/// Get pawn pushes (forward moves, not attacks)
pub fn pawn_pushes(square: Square, color: Color, occupancy: u64) -> u64 {
    let mut pushes = 0u64;
    let rank = square.rank() as i8;
    let file = square.file() as i8;

    match color {
        Color::White => {
            // Single push
            if rank < 7 {
                let target = 1u64 << ((rank + 1) * 8 + file);
                if (occupancy & target) == 0 {
                    pushes |= target;

                    // Double push from starting rank
                    if rank == 1 {
                        let double_target = 1u64 << ((rank + 2) * 8 + file);
                        if (occupancy & double_target) == 0 {
                            pushes |= double_target;
                        }
                    }
                }
            }
        }
        Color::Black => {
            // Single push
            if rank > 0 {
                let target = 1u64 << ((rank - 1) * 8 + file);
                if (occupancy & target) == 0 {
                    pushes |= target;

                    // Double push from starting rank
                    if rank == 6 {
                        let double_target = 1u64 << ((rank - 2) * 8 + file);
                        if (occupancy & double_target) == 0 {
                            pushes |= double_target;
                        }
                    }
                }
            }
        }
    }

    pushes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_attacks_white() {
        let tables = AttackTables::new();

        // White pawn on E4 attacks D5 and F5
        let attacks = tables.pawn_attacks(Square::E4, Color::White);
        assert_eq!(attacks.count_ones(), 2);
        assert_ne!(attacks & (1u64 << Square::D5.index()), 0);
        assert_ne!(attacks & (1u64 << Square::F5.index()), 0);
    }

    #[test]
    fn test_pawn_attacks_black() {
        let tables = AttackTables::new();

        // Black pawn on E5 attacks D4 and F4
        let attacks = tables.pawn_attacks(Square::E5, Color::Black);
        assert_eq!(attacks.count_ones(), 2);
        assert_ne!(attacks & (1u64 << Square::D4.index()), 0);
        assert_ne!(attacks & (1u64 << Square::F4.index()), 0);
    }

    #[test]
    fn test_pawn_attacks_edge() {
        let tables = AttackTables::new();

        // White pawn on A4 attacks only B5 (not off board)
        let attacks = tables.pawn_attacks(Square::A4, Color::White);
        assert_eq!(attacks.count_ones(), 1);
        assert_ne!(attacks & (1u64 << Square::B5.index()), 0);
    }

    #[test]
    fn test_knight_attacks_center() {
        let tables = AttackTables::new();

        // Knight on E4 can move to 8 squares
        let attacks = tables.knight_attacks(Square::E4);
        assert_eq!(attacks.count_ones(), 8);
    }

    #[test]
    fn test_knight_attacks_corner() {
        let tables = AttackTables::new();

        // Knight on A1 can move to only 2 squares
        let attacks = tables.knight_attacks(Square::A1);
        assert_eq!(attacks.count_ones(), 2);
        assert_ne!(attacks & (1u64 << Square::B3.index()), 0);
        assert_ne!(attacks & (1u64 << Square::C2.index()), 0);
    }

    #[test]
    fn test_king_attacks_center() {
        let tables = AttackTables::new();

        // King on E4 can move to 8 adjacent squares
        let attacks = tables.king_attacks(Square::E4);
        assert_eq!(attacks.count_ones(), 8);
    }

    #[test]
    fn test_king_attacks_corner() {
        let tables = AttackTables::new();

        // King on A1 can move to only 3 squares
        let attacks = tables.king_attacks(Square::A1);
        assert_eq!(attacks.count_ones(), 3);
    }

    #[test]
    fn test_pawn_pushes_white() {
        // Empty board
        let pushes = pawn_pushes(Square::E2, Color::White, 0);
        // Can push to E3 and E4
        assert_eq!(pushes.count_ones(), 2);
        assert_ne!(pushes & (1u64 << Square::E3.index()), 0);
        assert_ne!(pushes & (1u64 << Square::E4.index()), 0);
    }

    #[test]
    fn test_pawn_pushes_blocked() {
        // Piece on E3
        let occupancy = 1u64 << Square::E3.index();
        let pushes = pawn_pushes(Square::E2, Color::White, occupancy);
        // Cannot push (blocked)
        assert_eq!(pushes.count_ones(), 0);
    }
}
