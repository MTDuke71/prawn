// Game state with move execution capabilities
// REQ-1: Make move, REQ-2: Unmake move, REQ-3: Move history, REQ-7: Repetition detection
// OPTIMIZED: Fixed-size arrays, no heap allocation, inline hot paths

use crate::board::{Board, Color, Piece, PieceType, Square};
use crate::undo_info::UndoInfo;
use crate::zobrist::{ZobristHasher, ZOBRIST};
use crate::Move;

/// Maximum game length (plies). 1024 plies = 512 full moves, very generous
pub const MAX_PLY: usize = 1024;

/// Game state with move execution and history tracking
/// Requirements Satisfied: REQ-1 through REQ-7
/// OPTIMIZED: Fixed-size arrays, no heap allocation during play
pub struct GameState {
    board: Board,
    current_hash: u64,
    initial_hash: u64, // Hash of starting position (for repetition detection)
    // Fixed-size arrays for history - no heap allocation during search
    move_history: [Move; MAX_PLY],
    undo_stack: [UndoInfo; MAX_PLY],
    position_hashes: [u64; MAX_PLY], // For repetition detection (hash after each move)
    ply: usize,                      // Current ply count
}

impl GameState {
    /// Create a new game state
    /// Requirements Satisfied: REQ-1, REQ-6
    #[inline]
    pub fn new(board: Board, _zobrist: ZobristHasher) -> Self {
        let current_hash = ZOBRIST.hash_board(&board);

        GameState {
            board,
            current_hash,
            initial_hash: current_hash, // Store for repetition detection
            move_history: [Move::NULL; MAX_PLY],
            undo_stack: [UndoInfo::EMPTY; MAX_PLY],
            position_hashes: [0; MAX_PLY], // Will be filled as moves are made
            ply: 0,
        }
    }

    /// Create from just a board (convenience method)
    #[inline]
    pub fn from_board(board: Board) -> Self {
        Self::new(board, ZobristHasher::new())
    }

    /// Get reference to the board
    #[inline(always)]
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Get mutable reference to the board
    #[inline(always)]
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Get current Zobrist hash
    /// Requirements Satisfied: REQ-6
    #[inline(always)]
    pub fn zobrist_hash(&self) -> u64 {
        self.current_hash
    }

    /// Get move history as slice
    /// Requirements Satisfied: REQ-3
    #[inline(always)]
    pub fn move_history(&self) -> &[Move] {
        &self.move_history[..self.ply]
    }

    /// Get current ply count
    #[inline(always)]
    pub fn ply(&self) -> usize {
        self.ply
    }

    /// Make a move on the board
    /// Requirements Satisfied: REQ-1, REQ-4, REQ-5, REQ-6
    #[inline]
    pub fn make_move(&mut self, m: Move) {
        debug_assert!(self.ply < MAX_PLY, "Game too long!");

        // Save undo information
        self.undo_stack[self.ply] = UndoInfo::new(
            m.captured(),
            self.board.castling_rights(),
            self.board.en_passant_square(),
            self.board.halfmove_clock(),
            self.current_hash,
        );

        let from = m.from();
        let to = m.to();
        let moving_piece = self.board.piece_at(from);

        // Update Zobrist hash - remove old state
        self.update_hash_before_move(&m);

        // REQ-4: Update halfmove clock
        let is_pawn_move = moving_piece
            .map(|p| p.piece_type() == PieceType::Pawn)
            .unwrap_or(false);
        let is_capture = m.is_capture();

        if is_pawn_move || is_capture {
            self.board.set_halfmove_clock(0);
        } else {
            self.board
                .set_halfmove_clock(self.board.halfmove_clock() + 1);
        }

        // Clear en passant square by default
        self.board.set_en_passant_square(None);

        // Execute the move based on type
        match m.move_type() {
            crate::MoveType::Quiet => {
                self.execute_quiet_move(from, to);
            }
            crate::MoveType::DoublePawnPush => {
                self.execute_double_pawn_push(from, to);
            }
            crate::MoveType::Capture => {
                self.execute_capture(from, to);
            }
            crate::MoveType::EnPassant => {
                self.execute_en_passant(from, to);
            }
            crate::MoveType::KingsideCastle => {
                self.execute_kingside_castle();
            }
            crate::MoveType::QueensideCastle => {
                self.execute_queenside_castle();
            }
            crate::MoveType::Promotion => {
                self.execute_promotion(from, to, m.promotion().unwrap());
            }
            crate::MoveType::CapturePromotion => {
                self.execute_capture_promotion(from, to, m.promotion().unwrap());
            }
        }

        // Update castling rights based on king/rook moves
        self.update_castling_rights(from, to, moving_piece);

        // REQ-5: Update fullmove counter
        let side = self.board.side_to_move();
        self.board.swap_side();

        if side == Color::Black {
            self.board
                .set_fullmove_number(self.board.fullmove_number() + 1);
        }

        // Update Zobrist hash - add new state
        self.update_hash_after_move(&m);

        // REQ-3: Track move in history
        self.move_history[self.ply] = m;

        // REQ-7: Track position for repetition detection
        self.position_hashes[self.ply] = self.current_hash;
        self.ply += 1;
    }

    /// Unmake the last move
    /// Requirements Satisfied: REQ-2, REQ-6, REQ-7
    #[inline]
    pub fn unmake_move(&mut self) {
        if self.ply == 0 {
            return;
        }

        self.ply -= 1;
        let undo = self.undo_stack[self.ply];
        let m = self.move_history[self.ply];

        // Restore Zobrist hash
        self.current_hash = undo.zobrist_hash;

        let from = m.from();
        let to = m.to();

        // Undo the move based on type
        match m.move_type() {
            crate::MoveType::Quiet | crate::MoveType::DoublePawnPush => {
                self.undo_quiet_move(from, to);
            }
            crate::MoveType::Capture => {
                self.undo_capture(from, to, undo.captured_piece.unwrap());
            }
            crate::MoveType::EnPassant => {
                self.undo_en_passant(from, to);
            }
            crate::MoveType::KingsideCastle => {
                self.undo_kingside_castle();
            }
            crate::MoveType::QueensideCastle => {
                self.undo_queenside_castle();
            }
            crate::MoveType::Promotion => {
                self.undo_promotion(from, to);
            }
            crate::MoveType::CapturePromotion => {
                self.undo_capture_promotion(from, to, undo.captured_piece.unwrap());
            }
        }

        // Restore game state
        self.board.set_castling_rights(undo.castling_rights);
        self.board.set_en_passant_square(undo.en_passant_square);
        self.board.set_halfmove_clock(undo.halfmove_clock);
        self.board.swap_side(); // Switch back

        // Restore fullmove number if it was Black's move
        if self.board.side_to_move() == Color::Black {
            self.board
                .set_fullmove_number(self.board.fullmove_number() - 1);
        }
    }

    /// Check if current position is a threefold repetition
    /// Requirements Satisfied: REQ-7
    /// OPTIMIZED: Linear scan through position history (better cache locality than HashMap)
    #[inline]
    pub fn is_threefold_repetition(&self) -> bool {
        let target = self.current_hash;
        let mut count = 1u32; // Current position counts as 1

        // Check initial position (before any moves)
        if self.initial_hash == target {
            count += 1;
            if count >= 3 {
                return true;
            }
        }

        // Check all previous positions in history
        // position_hashes[i] contains hash after move i was made (0-indexed)
        // We only care about positions with the same side to move, which are
        // every 4 plies apart (same position can only repeat after 2 full moves minimum)
        // But for simplicity and correctness, check all positions
        for i in 0..self.ply {
            if self.position_hashes[i] == target {
                count += 1;
                if count >= 3 {
                    return true;
                }
            }
        }

        false
    }

    /// Check if 50-move rule applies
    /// Requirements Satisfied: REQ-4
    #[inline(always)]
    pub fn is_fifty_move_rule(&self) -> bool {
        self.board.halfmove_clock() >= 100
    }

    // ========================================================================
    // Move execution helpers
    // ========================================================================

    #[inline(always)]
    fn execute_quiet_move(&mut self, from: Square, to: Square) {
        if let Some(piece) = self.board.piece_at(from) {
            self.board.clear_piece(from);
            self.board.set_piece(to, piece);
        }
    }

    #[inline(always)]
    fn execute_double_pawn_push(&mut self, from: Square, to: Square) {
        if let Some(piece) = self.board.piece_at(from) {
            self.board.clear_piece(from);
            self.board.set_piece(to, piece);

            // Set en passant square
            let ep_rank = if piece.color() == Color::White { 2 } else { 5 };
            let ep_square = Square::from_index(ep_rank * 8 + from.file()).unwrap();
            self.board.set_en_passant_square(Some(ep_square));
        }
    }

    #[inline(always)]
    fn execute_capture(&mut self, from: Square, to: Square) {
        if let Some(piece) = self.board.piece_at(from) {
            self.board.clear_piece(to); // Remove captured piece
            self.board.clear_piece(from);
            self.board.set_piece(to, piece);
        }
    }

    #[inline(always)]
    fn execute_en_passant(&mut self, from: Square, to: Square) {
        if let Some(piece) = self.board.piece_at(from) {
            // Remove capturing pawn
            self.board.clear_piece(from);
            // Place pawn on destination
            self.board.set_piece(to, piece);
            // Remove captured pawn (on same rank as from square, same file as to square)
            let captured_square = Square::from_index(from.rank() * 8 + to.file()).unwrap();
            self.board.clear_piece(captured_square);
        }
    }

    #[inline(always)]
    fn execute_kingside_castle(&mut self) {
        let color = self.board.side_to_move();
        let (king_from, king_to, rook_from, rook_to) = match color {
            Color::White => (Square::E1, Square::G1, Square::H1, Square::F1),
            Color::Black => (Square::E8, Square::G8, Square::H8, Square::F8),
        };

        let king = self.board.piece_at(king_from).unwrap();
        let rook = self.board.piece_at(rook_from).unwrap();

        self.board.clear_piece(king_from);
        self.board.clear_piece(rook_from);
        self.board.set_piece(king_to, king);
        self.board.set_piece(rook_to, rook);
    }

    #[inline(always)]
    fn execute_queenside_castle(&mut self) {
        let color = self.board.side_to_move();
        let (king_from, king_to, rook_from, rook_to) = match color {
            Color::White => (Square::E1, Square::C1, Square::A1, Square::D1),
            Color::Black => (Square::E8, Square::C8, Square::A8, Square::D8),
        };

        let king = self.board.piece_at(king_from).unwrap();
        let rook = self.board.piece_at(rook_from).unwrap();

        self.board.clear_piece(king_from);
        self.board.clear_piece(rook_from);
        self.board.set_piece(king_to, king);
        self.board.set_piece(rook_to, rook);
    }

    #[inline(always)]
    fn execute_promotion(&mut self, from: Square, to: Square, promotion: PieceType) {
        let color = self.board.side_to_move();
        self.board.clear_piece(from);
        self.board
            .set_piece(to, Piece::from_type_and_color(promotion, color));
    }

    #[inline(always)]
    fn execute_capture_promotion(&mut self, from: Square, to: Square, promotion: PieceType) {
        let color = self.board.side_to_move();
        self.board.clear_piece(to); // Remove captured piece
        self.board.clear_piece(from);
        self.board
            .set_piece(to, Piece::from_type_and_color(promotion, color));
    }

    // ========================================================================
    // Move undo helpers
    // ========================================================================

    #[inline(always)]
    fn undo_quiet_move(&mut self, from: Square, to: Square) {
        if let Some(piece) = self.board.piece_at(to) {
            self.board.clear_piece(to);
            self.board.set_piece(from, piece);
        }
    }

    #[inline(always)]
    fn undo_capture(&mut self, from: Square, to: Square, captured: Piece) {
        if let Some(piece) = self.board.piece_at(to) {
            self.board.clear_piece(to);
            self.board.set_piece(from, piece);
            self.board.set_piece(to, captured);
        }
    }

    #[inline(always)]
    fn undo_en_passant(&mut self, from: Square, to: Square) {
        // During unmake, side_to_move() is the opponent of who made the move
        // So we need to use side_to_move() for the captured pawn color
        let captured_pawn_color = self.board.side_to_move();
        let moving_pawn_color = captured_pawn_color.opponent();

        if let Some(_piece) = self.board.piece_at(to) {
            self.board.clear_piece(to);
            self.board.set_piece(
                from,
                Piece::from_type_and_color(PieceType::Pawn, moving_pawn_color),
            );
            // Restore captured pawn (on same rank as from square, same file as to square)
            let captured_square = Square::from_index(from.rank() * 8 + to.file()).unwrap();
            self.board.set_piece(
                captured_square,
                Piece::from_type_and_color(PieceType::Pawn, captured_pawn_color),
            );
        }
    }

    #[inline(always)]
    fn undo_kingside_castle(&mut self) {
        // During unmake, side_to_move() is the opponent of who castled
        let castle_color = self.board.side_to_move().opponent();
        let (king_from, king_to, rook_from, rook_to) = match castle_color {
            Color::White => (Square::E1, Square::G1, Square::H1, Square::F1),
            Color::Black => (Square::E8, Square::G8, Square::H8, Square::F8),
        };

        let king = self.board.piece_at(king_to).unwrap();
        let rook = self.board.piece_at(rook_to).unwrap();

        self.board.clear_piece(king_to);
        self.board.clear_piece(rook_to);
        self.board.set_piece(king_from, king);
        self.board.set_piece(rook_from, rook);
    }

    #[inline(always)]
    fn undo_queenside_castle(&mut self) {
        // During unmake, side_to_move() is the opponent of who castled
        let castle_color = self.board.side_to_move().opponent();
        let (king_from, king_to, rook_from, rook_to) = match castle_color {
            Color::White => (Square::E1, Square::C1, Square::A1, Square::D1),
            Color::Black => (Square::E8, Square::C8, Square::A8, Square::D8),
        };

        let king = self.board.piece_at(king_to).unwrap();
        let rook = self.board.piece_at(rook_to).unwrap();

        self.board.clear_piece(king_to);
        self.board.clear_piece(rook_to);
        self.board.set_piece(king_from, king);
        self.board.set_piece(rook_from, rook);
    }

    #[inline(always)]
    fn undo_promotion(&mut self, from: Square, to: Square) {
        // During unmake, side_to_move() is the opponent of who promoted
        let promoting_color = self.board.side_to_move().opponent();
        self.board.clear_piece(to);
        self.board.set_piece(
            from,
            Piece::from_type_and_color(PieceType::Pawn, promoting_color),
        );
    }

    #[inline(always)]
    fn undo_capture_promotion(&mut self, from: Square, to: Square, captured: Piece) {
        // During unmake, side_to_move() is the opponent of who promoted
        let promoting_color = self.board.side_to_move().opponent();
        self.board.clear_piece(to);
        self.board.set_piece(
            from,
            Piece::from_type_and_color(PieceType::Pawn, promoting_color),
        );
        self.board.set_piece(to, captured);
    }

    // ========================================================================
    // Castling rights update
    // ========================================================================

    #[inline(always)]
    fn update_castling_rights(&mut self, from: Square, to: Square, moving_piece: Option<Piece>) {
        if let Some(piece) = moving_piece {
            let color = piece.color();
            match piece.piece_type() {
                PieceType::King => {
                    // King move: clear both castling rights for this color
                    match color {
                        Color::White => self.board.clear_castling_rights(0b0011),
                        Color::Black => self.board.clear_castling_rights(0b1100),
                    }
                }
                PieceType::Rook => {
                    // Rook move: clear castling right for that side
                    match (color, from) {
                        (Color::White, Square::H1) => self.board.clear_castling_rights(0b0001),
                        (Color::White, Square::A1) => self.board.clear_castling_rights(0b0010),
                        (Color::Black, Square::H8) => self.board.clear_castling_rights(0b0100),
                        (Color::Black, Square::A8) => self.board.clear_castling_rights(0b1000),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Also clear castling rights if rook is captured
        match to {
            Square::H1 => self.board.clear_castling_rights(0b0001),
            Square::A1 => self.board.clear_castling_rights(0b0010),
            Square::H8 => self.board.clear_castling_rights(0b0100),
            Square::A8 => self.board.clear_castling_rights(0b1000),
            _ => {}
        }
    }

    // ========================================================================
    // Zobrist hash updates
    // ========================================================================

    #[inline(always)]
    fn update_hash_before_move(&mut self, m: &Move) {
        let from = m.from();
        let to = m.to();

        // Remove piece from source square
        if let Some(piece) = self.board.piece_at(from) {
            self.current_hash ^= ZOBRIST.hash_piece(piece, from);
        }

        // Remove captured piece (if any)
        if m.is_capture() && !matches!(m.move_type(), crate::MoveType::EnPassant) {
            if let Some(captured) = self.board.piece_at(to) {
                self.current_hash ^= ZOBRIST.hash_piece(captured, to);
            }
        }

        // Handle en passant capture
        if matches!(m.move_type(), crate::MoveType::EnPassant) {
            let captured_square = Square::from_index(from.rank() * 8 + to.file()).unwrap();
            if let Some(captured) = self.board.piece_at(captured_square) {
                self.current_hash ^= ZOBRIST.hash_piece(captured, captured_square);
            }
        }

        // Remove old castling rights
        self.current_hash ^= ZOBRIST.castling[self.board.castling_rights() as usize];

        // Remove old en passant
        if let Some(ep_square) = self.board.en_passant_square() {
            self.current_hash ^= ZOBRIST.en_passant[ep_square.file() as usize];
        }

        // Toggle side to move (only once, here before the move)
        self.current_hash ^= ZOBRIST.side_to_move;
    }

    #[inline(always)]
    fn update_hash_after_move(&mut self, m: &Move) {
        let to = m.to();

        // Add piece to destination square
        if let Some(piece) = self.board.piece_at(to) {
            self.current_hash ^= ZOBRIST.hash_piece(piece, to);
        }

        // Handle castling rook move
        if m.is_castle() {
            let color = self.board.side_to_move().opponent();
            let (rook_from, rook_to) = match m.move_type() {
                crate::MoveType::KingsideCastle => match color {
                    Color::White => (Square::H1, Square::F1),
                    Color::Black => (Square::H8, Square::F8),
                },
                crate::MoveType::QueensideCastle => match color {
                    Color::White => (Square::A1, Square::D1),
                    Color::Black => (Square::A8, Square::D8),
                },
                _ => unreachable!(),
            };

            // Remove rook from old square, add to new square
            let rook = Piece::from_type_and_color(PieceType::Rook, color);
            self.current_hash ^= ZOBRIST.hash_piece(rook, rook_from);
            self.current_hash ^= ZOBRIST.hash_piece(rook, rook_to);
        }

        // Add new castling rights
        self.current_hash ^= ZOBRIST.castling[self.board.castling_rights() as usize];

        // Add new en passant
        if let Some(ep_square) = self.board.en_passant_square() {
            self.current_hash ^= ZOBRIST.en_passant[ep_square.file() as usize];
        }

        // Note: side_to_move toggle is already done in update_hash_before_move
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_unmake_preserves_hash() {
        let board = Board::starting_position();
        let mut game = GameState::from_board(board);

        let original_hash = game.zobrist_hash();

        // Make and unmake a move
        let m = Move::new_quiet(Square::E2, Square::E4);
        game.make_move(m);
        game.unmake_move();

        assert_eq!(game.zobrist_hash(), original_hash);
    }
}
