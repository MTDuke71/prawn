//! UCI Protocol Handler
//!
//! Implements the Universal Chess Interface protocol for GUI communication.
//!
//! # Requirements Satisfied
//! - REQ-1: UCI handshake
//! - REQ-2: Position setup
//! - REQ-3: Position display
//! - REQ-4: Perft command
//! - REQ-5: Basic go command
//! - REQ-6: Quit command

use prawn::board::{Board, Color, PieceType};
use prawn::{GameState, Move, MoveGenerator};
use std::time::Instant;

const ENGINE_NAME: &str = "prawn 0.1";
const ENGINE_AUTHOR: &str = "MTDuke71";

/// UCI protocol handler
///
/// Manages UCI state and processes commands from the GUI.
pub struct UciHandler {
    /// Current game state
    game: GameState,
    /// Move generator
    movegen: MoveGenerator,
    /// Flag indicating the engine should quit
    quit: bool,
}

impl UciHandler {
    /// Creates a new UCI handler with starting position
    #[inline]
    pub fn new() -> Self {
        Self {
            game: GameState::from_board(Board::default()),
            movegen: MoveGenerator::new(),
            quit: false,
        }
    }

    /// Process a UCI command and return the response
    ///
    /// # Arguments
    /// * `input` - The UCI command string
    ///
    /// # Returns
    /// The response string to send to the GUI
    pub fn process_command(&mut self, input: &str) -> String {
        let input = input.trim();
        if input.is_empty() {
            return String::new();
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");

        match command {
            "uci" => self.cmd_uci(),
            "isready" => self.cmd_isready(),
            "ucinewgame" => self.cmd_ucinewgame(),
            "position" => self.cmd_position(input),
            "go" => self.cmd_go(input),
            "d" => self.cmd_display(),
            "quit" => self.cmd_quit(),
            _ => String::new(), // Unknown commands silently ignored per UCI spec
        }
    }

    /// Returns true if the engine should quit
    #[inline]
    pub fn should_quit(&self) -> bool {
        self.quit
    }

    /// Returns the current position's FEN string
    #[inline]
    pub fn current_fen(&self) -> String {
        self.game.board().to_fen()
    }

    // =========================================================================
    // REQ-1: UCI Handshake
    // =========================================================================

    /// Handle 'uci' command - identify the engine
    fn cmd_uci(&self) -> String {
        format!(
            "id name {}\nid author {}\nuciok",
            ENGINE_NAME, ENGINE_AUTHOR
        )
    }

    /// Handle 'isready' command - synchronization
    fn cmd_isready(&self) -> String {
        "readyok".to_string()
    }

    // =========================================================================
    // REQ-2: Position Setup
    // =========================================================================

    /// Handle 'ucinewgame' command - reset state
    fn cmd_ucinewgame(&mut self) -> String {
        self.game = GameState::from_board(Board::default());
        String::new()
    }

    /// Handle 'position' command - set up board position
    ///
    /// Formats:
    /// - `position startpos`
    /// - `position startpos moves e2e4 e7e5 ...`
    /// - `position fen <fen>`
    /// - `position fen <fen> moves e2e4 e7e5 ...`
    fn cmd_position(&mut self, input: &str) -> String {
        let input = input.strip_prefix("position").unwrap_or(input).trim();

        // Find where moves start (if any)
        let (position_part, moves_part) = if let Some(idx) = input.find(" moves ") {
            (&input[..idx], Some(&input[idx + 7..]))
        } else {
            (input, None)
        };

        // Parse the position
        if position_part.starts_with("startpos") {
            self.game = GameState::from_board(Board::default());
        } else if position_part.starts_with("fen ") {
            let fen = position_part.strip_prefix("fen ").unwrap_or("").trim();
            if let Ok(board) = Board::from_fen(fen) {
                self.game = GameState::from_board(board);
            }
        }

        // Apply moves if any
        if let Some(moves_str) = moves_part {
            for move_str in moves_str.split_whitespace() {
                if let Some(mv) = self.parse_move(move_str) {
                    self.game.make_move(mv);
                }
            }
        }

        String::new()
    }

    /// Parse a UCI move string (e.g., "e2e4", "e7e8q") into a Move
    fn parse_move(&self, move_str: &str) -> Option<Move> {
        if move_str.len() < 4 {
            return None;
        }

        let from = Self::parse_square(&move_str[0..2])?;
        let to = Self::parse_square(&move_str[2..4])?;

        // Check for promotion
        let promotion = if move_str.len() > 4 {
            match move_str.chars().nth(4)? {
                'q' => Some(PieceType::Queen),
                'r' => Some(PieceType::Rook),
                'b' => Some(PieceType::Bishop),
                'n' => Some(PieceType::Knight),
                _ => None,
            }
        } else {
            None
        };

        // Generate legal moves and find the matching one
        let moves = self.movegen.generate_legal_moves(self.game.board());

        for mv in moves.moves() {
            if mv.from().index() == from && mv.to().index() == to {
                // Check promotion matches
                if let Some(promo) = promotion {
                    if mv.is_promotion() && mv.promotion() == Some(promo) {
                        return Some(*mv);
                    }
                } else if !mv.is_promotion() {
                    return Some(*mv);
                } else if mv.promotion() == Some(PieceType::Queen) {
                    // Default promotion to queen
                    return Some(*mv);
                }
            }
        }

        None
    }

    /// Parse a square string (e.g., "e2") into a square index
    fn parse_square(s: &str) -> Option<usize> {
        let mut chars = s.chars();
        let file = chars.next()?;
        let rank = chars.next()?;

        if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
            return None;
        }

        let file_idx = (file as u8 - b'a') as usize;
        let rank_idx = (rank as u8 - b'1') as usize;

        Some(rank_idx * 8 + file_idx)
    }

    // =========================================================================
    // REQ-3: Position Display
    // =========================================================================

    /// Handle 'd' command - display current position
    fn cmd_display(&self) -> String {
        let board = self.game.board();
        let mut output = String::new();

        // Board display
        output.push_str(&format!("{}\n", board));

        // FEN
        output.push_str(&format!("FEN: {}\n", board.to_fen()));

        // Side to move
        let side = if board.side_to_move() == Color::White {
            "White"
        } else {
            "Black"
        };
        output.push_str(&format!("Side to move: {}\n", side));

        // Hash
        output.push_str(&format!("Hash: {:016x}", self.game.zobrist_hash()));

        output
    }

    // =========================================================================
    // REQ-4: Perft Command
    // =========================================================================

    /// Handle 'go perft N' command
    fn cmd_go(&mut self, input: &str) -> String {
        // Check for perft
        if input.contains("perft") {
            return self.cmd_perft(input);
        }

        // Basic go - return first legal move
        self.cmd_go_basic()
    }

    /// Run perft and return divide output
    fn cmd_perft(&mut self, input: &str) -> String {
        // Parse depth from "go perft N"
        let depth: u8 = input
            .split_whitespace()
            .skip_while(|&s| s != "perft")
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);

        let start = Instant::now();
        let mut total_nodes: u64 = 0;
        let mut output = String::new();

        // Generate moves for divide
        let moves = self.movegen.generate_legal_moves(self.game.board());

        for mv in moves.moves() {
            self.game.make_move(*mv);
            let nodes = self.perft(depth - 1);
            self.game.unmake_move();

            output.push_str(&format!("{}: {}\n", mv.to_uci(), nodes));
            total_nodes += nodes;
        }

        let elapsed = start.elapsed();
        let nps = if elapsed.as_secs_f64() > 0.0 {
            (total_nodes as f64 / elapsed.as_secs_f64()) as u64
        } else {
            0
        };

        output.push_str(&format!("\nNodes: {}\n", total_nodes));
        output.push_str(&format!("Time: {}ms\n", elapsed.as_millis()));
        output.push_str(&format!("NPS: {}", nps));

        output
    }

    /// Perft recursive function
    fn perft(&mut self, depth: u8) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = self.movegen.generate_legal_moves(self.game.board());

        if depth == 1 {
            return moves.len() as u64;
        }

        let mut nodes: u64 = 0;
        for mv in moves.moves() {
            self.game.make_move(*mv);
            nodes += self.perft(depth - 1);
            self.game.unmake_move();
        }

        nodes
    }

    // =========================================================================
    // REQ-5: Basic Go Command
    // =========================================================================

    /// Basic go command - returns first legal move
    fn cmd_go_basic(&self) -> String {
        let moves = self.movegen.generate_legal_moves(self.game.board());

        if !moves.is_empty() {
            let mv = &moves.moves()[0];
            format!("bestmove {}", mv.to_uci())
        } else {
            // No legal moves - checkmate or stalemate
            "bestmove 0000".to_string()
        }
    }

    // =========================================================================
    // REQ-6: Quit Command
    // =========================================================================

    /// Handle 'quit' command
    fn cmd_quit(&mut self) -> String {
        self.quit = true;
        String::new()
    }
}

impl Default for UciHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_square() {
        assert_eq!(UciHandler::parse_square("a1"), Some(0));
        assert_eq!(UciHandler::parse_square("h1"), Some(7));
        assert_eq!(UciHandler::parse_square("a8"), Some(56));
        assert_eq!(UciHandler::parse_square("h8"), Some(63));
        assert_eq!(UciHandler::parse_square("e4"), Some(28));
        assert_eq!(UciHandler::parse_square("d5"), Some(35));
    }
}
