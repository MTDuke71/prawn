// Mission 1: Board Representation - Bitboard Implementation
// REQ-1: 8x8 Board Representation with Bitboards

use std::fmt;

// ============================================================================
// Types and Constants
// ============================================================================

/// Bitboard type: 64-bit integer where each bit represents a square
/// Bit 0 = A1, Bit 1 = B1, ..., Bit 7 = H1, Bit 8 = A2, ..., Bit 63 = H8
pub type Bitboard = u64;

/// Square enumeration (0-63)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    /// Convert square to index (0-63)
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Create square from index
    pub fn from_index(index: u8) -> Option<Self> {
        if index < 64 {
            Some(unsafe { std::mem::transmute::<u8, Square>(index) })
        } else {
            None
        }
    }

    /// Get rank (0-7) from square
    pub const fn rank(self) -> u8 {
        (self as u8) / 8
    }

    /// Get file (0-7) from square
    pub const fn file(self) -> u8 {
        (self as u8) % 8
    }
}

/// Color enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    /// Get opponent color
    pub const fn opponent(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// Piece type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Complete piece enumeration (type + color)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    /// Get piece type
    pub const fn piece_type(self) -> PieceType {
        match self {
            Piece::WhitePawn | Piece::BlackPawn => PieceType::Pawn,
            Piece::WhiteKnight | Piece::BlackKnight => PieceType::Knight,
            Piece::WhiteBishop | Piece::BlackBishop => PieceType::Bishop,
            Piece::WhiteRook | Piece::BlackRook => PieceType::Rook,
            Piece::WhiteQueen | Piece::BlackQueen => PieceType::Queen,
            Piece::WhiteKing | Piece::BlackKing => PieceType::King,
        }
    }

    /// Get piece color
    pub const fn color(self) -> Color {
        match self {
            Piece::WhitePawn
            | Piece::WhiteKnight
            | Piece::WhiteBishop
            | Piece::WhiteRook
            | Piece::WhiteQueen
            | Piece::WhiteKing => Color::White,
            Piece::BlackPawn
            | Piece::BlackKnight
            | Piece::BlackBishop
            | Piece::BlackRook
            | Piece::BlackQueen
            | Piece::BlackKing => Color::Black,
        }
    }

    /// Create piece from type and color
    pub const fn from_type_and_color(piece_type: PieceType, color: Color) -> Piece {
        match (piece_type, color) {
            (PieceType::Pawn, Color::White) => Piece::WhitePawn,
            (PieceType::Knight, Color::White) => Piece::WhiteKnight,
            (PieceType::Bishop, Color::White) => Piece::WhiteBishop,
            (PieceType::Rook, Color::White) => Piece::WhiteRook,
            (PieceType::Queen, Color::White) => Piece::WhiteQueen,
            (PieceType::King, Color::White) => Piece::WhiteKing,
            (PieceType::Pawn, Color::Black) => Piece::BlackPawn,
            (PieceType::Knight, Color::Black) => Piece::BlackKnight,
            (PieceType::Bishop, Color::Black) => Piece::BlackBishop,
            (PieceType::Rook, Color::Black) => Piece::BlackRook,
            (PieceType::Queen, Color::Black) => Piece::BlackQueen,
            (PieceType::King, Color::Black) => Piece::BlackKing,
        }
    }

    /// Get FEN character for piece
    pub fn to_fen_char(self) -> char {
        match self {
            Piece::WhitePawn => 'P',
            Piece::WhiteKnight => 'N',
            Piece::WhiteBishop => 'B',
            Piece::WhiteRook => 'R',
            Piece::WhiteQueen => 'Q',
            Piece::WhiteKing => 'K',
            Piece::BlackPawn => 'p',
            Piece::BlackKnight => 'n',
            Piece::BlackBishop => 'b',
            Piece::BlackRook => 'r',
            Piece::BlackQueen => 'q',
            Piece::BlackKing => 'k',
        }
    }

    /// Parse FEN character to piece
    pub fn from_fen_char(c: char) -> Option<Piece> {
        match c {
            'P' => Some(Piece::WhitePawn),
            'N' => Some(Piece::WhiteKnight),
            'B' => Some(Piece::WhiteBishop),
            'R' => Some(Piece::WhiteRook),
            'Q' => Some(Piece::WhiteQueen),
            'K' => Some(Piece::WhiteKing),
            'p' => Some(Piece::BlackPawn),
            'n' => Some(Piece::BlackKnight),
            'b' => Some(Piece::BlackBishop),
            'r' => Some(Piece::BlackRook),
            'q' => Some(Piece::BlackQueen),
            'k' => Some(Piece::BlackKing),
            _ => None,
        }
    }
}

// ============================================================================
// Bitboard Utilities (REQ-2)
// ============================================================================

/// Bitboard utility functions and constants
pub struct BitboardOps;

impl BitboardOps {
    // File masks
    pub const FILE_A: u64 = 0x0101010101010101;
    pub const FILE_B: u64 = 0x0202020202020202;
    pub const FILE_C: u64 = 0x0404040404040404;
    pub const FILE_D: u64 = 0x0808080808080808;
    pub const FILE_E: u64 = 0x1010101010101010;
    pub const FILE_F: u64 = 0x2020202020202020;
    pub const FILE_G: u64 = 0x4040404040404040;
    pub const FILE_H: u64 = 0x8080808080808080;

    // Rank masks
    pub const RANK_1: u64 = 0x00000000000000FF;
    pub const RANK_2: u64 = 0x000000000000FF00;
    pub const RANK_3: u64 = 0x0000000000FF0000;
    pub const RANK_4: u64 = 0x00000000FF000000;
    pub const RANK_5: u64 = 0x000000FF00000000;
    pub const RANK_6: u64 = 0x0000FF0000000000;
    pub const RANK_7: u64 = 0x00FF000000000000;
    pub const RANK_8: u64 = 0xFF00000000000000;

    /// Set bit at square
    pub fn set_bit(bb: u64, square: Square) -> u64 {
        bb | (1u64 << square.index())
    }

    /// Clear bit at square
    pub fn clear_bit(bb: u64, square: Square) -> u64 {
        bb & !(1u64 << square.index())
    }

    /// Test if bit is set at square
    pub fn get_bit(bb: u64, square: Square) -> bool {
        (bb & (1u64 << square.index())) != 0
    }

    /// Count number of set bits
    pub fn count_bits(bb: u64) -> u32 {
        bb.count_ones()
    }

    /// Pop least significant bit and return new bitboard and square
    pub fn pop_bit(bb: u64) -> (u64, Option<Square>) {
        if bb == 0 {
            return (0, None);
        }
        let lsb = bb.trailing_zeros() as u8;
        let new_bb = bb & (bb - 1); // Clear LSB
        (new_bb, Square::from_index(lsb))
    }

    /// Convert square to bitboard with single bit set
    pub fn square_to_bitboard(square: Square) -> u64 {
        1u64 << square.index()
    }
}

// ============================================================================
// Board Structure (REQ-1)
// ============================================================================

/// Chess board representation using bitboards
#[derive(Clone)]
pub struct Board {
    // Piece bitboards: [color][piece_type]
    pawns: [u64; 2],
    knights: [u64; 2],
    bishops: [u64; 2],
    rooks: [u64; 2],
    queens: [u64; 2],
    kings: [u64; 2],

    // Game state
    side_to_move: Color,
    castling_rights: u8, // KQkq bits
    en_passant_square: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl Board {
    /// Create empty board
    pub fn new() -> Self {
        Board {
            pawns: [0, 0],
            knights: [0, 0],
            bishops: [0, 0],
            rooks: [0, 0],
            queens: [0, 0],
            kings: [0, 0],
            side_to_move: Color::White,
            castling_rights: 0,
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Get bitboard for specific piece type and color
    pub fn piece_bitboard(&self, color: Color, piece_type: PieceType) -> u64 {
        let idx = color as usize;
        match piece_type {
            PieceType::Pawn => self.pawns[idx],
            PieceType::Knight => self.knights[idx],
            PieceType::Bishop => self.bishops[idx],
            PieceType::Rook => self.rooks[idx],
            PieceType::Queen => self.queens[idx],
            PieceType::King => self.kings[idx],
        }
    }

    /// Get occupancy bitboard for a color
    pub fn occupancy(&self, color: Color) -> u64 {
        let idx = color as usize;
        self.pawns[idx]
            | self.knights[idx]
            | self.bishops[idx]
            | self.rooks[idx]
            | self.queens[idx]
            | self.kings[idx]
    }

    /// Get all occupancy bitboard
    pub fn all_occupancy(&self) -> u64 {
        self.occupancy(Color::White) | self.occupancy(Color::Black)
    }

    /// Set piece at square
    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        let color_idx = piece.color() as usize;
        let bb = match piece.piece_type() {
            PieceType::Pawn => &mut self.pawns[color_idx],
            PieceType::Knight => &mut self.knights[color_idx],
            PieceType::Bishop => &mut self.bishops[color_idx],
            PieceType::Rook => &mut self.rooks[color_idx],
            PieceType::Queen => &mut self.queens[color_idx],
            PieceType::King => &mut self.kings[color_idx],
        };
        *bb = BitboardOps::set_bit(*bb, square);
    }

    /// Clear piece at square
    pub fn clear_piece(&mut self, square: Square) {
        for color_idx in 0..2 {
            self.pawns[color_idx] = BitboardOps::clear_bit(self.pawns[color_idx], square);
            self.knights[color_idx] = BitboardOps::clear_bit(self.knights[color_idx], square);
            self.bishops[color_idx] = BitboardOps::clear_bit(self.bishops[color_idx], square);
            self.rooks[color_idx] = BitboardOps::clear_bit(self.rooks[color_idx], square);
            self.queens[color_idx] = BitboardOps::clear_bit(self.queens[color_idx], square);
            self.kings[color_idx] = BitboardOps::clear_bit(self.kings[color_idx], square);
        }
    }

    /// Get piece at square
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        for color in [Color::White, Color::Black] {
            for piece_type in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ] {
                if BitboardOps::get_bit(self.piece_bitboard(color, piece_type), square) {
                    return Some(Piece::from_type_and_color(piece_type, color));
                }
            }
        }
        None
    }

    /// Get side to move
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Check if can castle kingside
    pub fn can_castle_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => (self.castling_rights & 0b0001) != 0,
            Color::Black => (self.castling_rights & 0b0100) != 0,
        }
    }

    /// Check if can castle queenside
    pub fn can_castle_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => (self.castling_rights & 0b0010) != 0,
            Color::Black => (self.castling_rights & 0b1000) != 0,
        }
    }

    /// Get en passant square
    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    /// Set en passant square
    pub fn set_en_passant_square(&mut self, square: Option<Square>) {
        self.en_passant_square = square;
    }

    /// Clear castling rights
    pub fn clear_castling_rights(&mut self, mask: u8) {
        self.castling_rights &= !mask;
    }

    /// Make a move without legality checking (for internal use)
    pub fn make_move_unchecked(&mut self, from: Square, to: Square) {
        if let Some(piece) = self.piece_at(from) {
            self.clear_piece(from);
            self.clear_piece(to); // Clear destination (handles captures)
            self.set_piece(to, piece);
        }
    }

    /// Swap side to move
    pub fn swap_side(&mut self) {
        self.side_to_move = self.side_to_move.opponent();
    }

    // REQ-5: FEN Parsing and Generation

    /// Parse FEN string to create board
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err(format!(
                "Invalid FEN: expected 6 parts, got {}",
                parts.len()
            ));
        }

        let mut board = Board::new();

        // Parse piece placement
        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err("Invalid FEN: expected 8 ranks".to_string());
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let rank = 7 - rank_idx; // FEN starts from rank 8
            let mut file = 0;

            for c in rank_str.chars() {
                if let Some(digit) = c.to_digit(10) {
                    file += digit as usize;
                } else if let Some(piece) = Piece::from_fen_char(c) {
                    let square_idx = rank * 8 + file;
                    if let Some(square) = Square::from_index(square_idx as u8) {
                        board.set_piece(square, piece);
                    }
                    file += 1;
                } else {
                    return Err(format!("Invalid FEN character: {}", c));
                }
            }
        }

        // Parse side to move
        board.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid side to move".to_string()),
        };

        // Parse castling rights
        board.castling_rights = 0;
        if parts[2] != "-" {
            for c in parts[2].chars() {
                match c {
                    'K' => board.castling_rights |= 0b0001,
                    'Q' => board.castling_rights |= 0b0010,
                    'k' => board.castling_rights |= 0b0100,
                    'q' => board.castling_rights |= 0b1000,
                    _ => return Err(format!("Invalid castling character: {}", c)),
                }
            }
        }

        // Parse en passant square
        board.en_passant_square = if parts[3] == "-" {
            None
        } else {
            // Parse algebraic notation (e.g., "e3")
            let chars: Vec<char> = parts[3].chars().collect();
            if chars.len() == 2 {
                let file = (chars[0] as u8).wrapping_sub(b'a');
                let rank = (chars[1] as u8).wrapping_sub(b'1');
                if file < 8 && rank < 8 {
                    Square::from_index(rank * 8 + file)
                } else {
                    None
                }
            } else {
                None
            }
        };

        // Parse halfmove clock
        board.halfmove_clock = parts[4].parse().unwrap_or(0);

        // Parse fullmove number
        board.fullmove_number = parts[5].parse().unwrap_or(1);

        Ok(board)
    }

    /// Generate FEN string from board
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Piece placement
        for rank in (0..8).rev() {
            let mut empty_count = 0;
            for file in 0..8 {
                let square = Square::from_index((rank * 8 + file) as u8).unwrap();
                if let Some(piece) = self.piece_at(square) {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(piece.to_fen_char());
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        // Side to move
        fen.push(' ');
        fen.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        // Castling rights
        fen.push(' ');
        if self.castling_rights == 0 {
            fen.push('-');
        } else {
            if (self.castling_rights & 0b0001) != 0 {
                fen.push('K');
            }
            if (self.castling_rights & 0b0010) != 0 {
                fen.push('Q');
            }
            if (self.castling_rights & 0b0100) != 0 {
                fen.push('k');
            }
            if (self.castling_rights & 0b1000) != 0 {
                fen.push('q');
            }
        }

        // En passant square
        fen.push(' ');
        if let Some(ep_square) = self.en_passant_square {
            let file = (b'a' + ep_square.file()) as char;
            let rank = (b'1' + ep_square.rank()) as char;
            fen.push(file);
            fen.push(rank);
        } else {
            fen.push('-');
        }

        // Halfmove clock and fullmove number
        fen.push_str(&format!(
            " {} {}",
            self.halfmove_clock, self.fullmove_number
        ));

        fen
    }
}

// Default implementation: standard starting position
impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Default FEN should always be valid")
    }
}

// REQ-6: Board Display
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  +---+---+---+---+---+---+---+---+")?;

        for rank in (0..8).rev() {
            write!(f, "{} |", rank + 1)?;
            for file in 0..8 {
                let square = Square::from_index((rank * 8 + file) as u8).unwrap();
                let piece_char = if let Some(piece) = self.piece_at(square) {
                    match piece {
                        Piece::WhitePawn => " ♙ ",
                        Piece::WhiteKnight => " ♘ ",
                        Piece::WhiteBishop => " ♗ ",
                        Piece::WhiteRook => " ♖ ",
                        Piece::WhiteQueen => " ♕ ",
                        Piece::WhiteKing => " ♔ ",
                        Piece::BlackPawn => " ♟ ",
                        Piece::BlackKnight => " ♞ ",
                        Piece::BlackBishop => " ♝ ",
                        Piece::BlackRook => " ♜ ",
                        Piece::BlackQueen => " ♛ ",
                        Piece::BlackKing => " ♚ ",
                    }
                } else {
                    "   "
                };
                write!(f, "{}|", piece_char)?;
            }
            writeln!(f)?;
            writeln!(f, "  +---+---+---+---+---+---+---+---+")?;
        }

        writeln!(f, "    a   b   c   d   e   f   g   h")?;
        Ok(())
    }
}
