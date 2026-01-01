//! UCI Protocol Handler
//!
//! Implements the Universal Chess Interface protocol for GUI communication.
//!
//! # Mission 4 Requirements (Basic UCI)
//! - REQ-1: UCI handshake
//! - REQ-2: Position setup
//! - REQ-3: Position display
//! - REQ-4: Perft command
//! - REQ-5: Basic go command
//! - REQ-6: Quit command
//!
//! # Mission 7 Requirements (Full UCI)
//! - REQ-1: Go commands with time control (depth, movetime, wtime/btime, infinite)
//! - REQ-2: Stop command (async search termination)
//! - REQ-3: Info output during search
//! - REQ-4: Bestmove with ponder
//! - REQ-5: UCI options (Hash)

use crate::board::{Board, Color, PieceType};
use crate::{GameState, Move, MoveGenerator};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const ENGINE_NAME: &str = "prawn 0.1";
const ENGINE_AUTHOR: &str = "MTDuke71";

// =============================================================================
// Mission 7: Search Parameters
// =============================================================================

/// Search parameters parsed from UCI go command
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    /// Fixed depth limit
    pub depth: Option<u8>,
    /// Fixed time in milliseconds
    pub movetime: Option<u64>,
    /// White time remaining in ms
    pub wtime: Option<u64>,
    /// Black time remaining in ms
    pub btime: Option<u64>,
    /// White increment per move in ms
    pub winc: Option<u64>,
    /// Black increment per move in ms
    pub binc: Option<u64>,
    /// Moves to go until next time control
    pub movestogo: Option<u32>,
    /// Search until stop command
    pub infinite: bool,
    /// Pondering mode
    pub ponder: bool,
    /// Node limit (optional)
    pub nodes: Option<u64>,
}

impl SearchParams {
    /// Parse go command arguments
    pub fn parse(args: &str) -> Self {
        let mut params = Self::default();
        let parts: Vec<&str> = args.split_whitespace().collect();
        
        let mut i = 0;
        while i < parts.len() {
            match parts[i] {
                "depth" => {
                    if i + 1 < parts.len() {
                        params.depth = parts[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "movetime" => {
                    if i + 1 < parts.len() {
                        params.movetime = parts[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "wtime" => {
                    if i + 1 < parts.len() {
                        params.wtime = parts[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "btime" => {
                    if i + 1 < parts.len() {
                        params.btime = parts[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "winc" => {
                    if i + 1 < parts.len() {
                        params.winc = parts[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "binc" => {
                    if i + 1 < parts.len() {
                        params.binc = parts[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "movestogo" => {
                    if i + 1 < parts.len() {
                        params.movestogo = parts[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "nodes" => {
                    if i + 1 < parts.len() {
                        params.nodes = parts[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "infinite" => {
                    params.infinite = true;
                }
                "ponder" => {
                    params.ponder = true;
                }
                _ => {}
            }
            i += 1;
        }
        
        params
    }
    
    /// Returns true if no time control is specified
    pub fn is_depth_only(&self) -> bool {
        self.depth.is_some() 
            && self.movetime.is_none() 
            && self.wtime.is_none() 
            && !self.infinite
    }
}

// =============================================================================
// Mission 7: Engine Options
// =============================================================================

/// Engine configuration options
#[derive(Debug, Clone)]
pub struct EngineOptions {
    /// Hash table size in MB
    pub hash_size_mb: usize,
}

impl Default for EngineOptions {
    fn default() -> Self {
        Self {
            hash_size_mb: 64,
        }
    }
}

impl EngineOptions {
    /// Print UCI options during handshake
    pub fn print_options() -> String {
        "option name Hash type spin default 64 min 1 max 4096".to_string()
    }
    
    /// Apply a setoption command
    pub fn set_option(&mut self, name: &str, value: &str) -> bool {
        if name.to_lowercase() == "hash" {
            if let Ok(size) = value.parse::<usize>() {
                self.hash_size_mb = size.clamp(1, 4096);
                return true;
            }
        }
        false
    }
}

// =============================================================================
// Mission 7: Time Management
// =============================================================================

/// Time manager for search
#[derive(Debug)]
pub struct TimeManager {
    /// When search started
    start_time: Instant,
    /// Target search time in ms (soft limit)
    target_time_ms: u64,
    /// Maximum search time in ms (hard limit)
    max_time_ms: u64,
    /// Stop flag for async termination
    stop_flag: Arc<AtomicBool>,
}

impl TimeManager {
    /// Create a new time manager from search parameters
    pub fn new(params: &SearchParams, side: Color, stop_flag: Arc<AtomicBool>) -> Self {
        let (target_time_ms, max_time_ms) = Self::calculate_time(params, side);
        
        Self {
            start_time: Instant::now(),
            target_time_ms,
            max_time_ms,
            stop_flag,
        }
    }
    
    /// Calculate target and max time from parameters
    fn calculate_time(params: &SearchParams, side: Color) -> (u64, u64) {
        // Infinite search or depth-limited
        if params.infinite || params.depth.is_some() {
            return (u64::MAX, u64::MAX);
        }
        
        // Fixed movetime
        if let Some(movetime) = params.movetime {
            return (movetime, movetime);
        }
        
        // Tournament time control
        let (time, inc) = match side {
            Color::White => (params.wtime, params.winc),
            Color::Black => (params.btime, params.binc),
        };
        
        let time = time.unwrap_or(60000);
        let inc = inc.unwrap_or(0);
        
        // Time management formula:
        // - Assume ~40 moves per game
        // - Use more time with increment available
        let moves_to_go = params.movestogo.unwrap_or(40) as u64;
        
        // Base time per move
        let base_time = time / moves_to_go;
        
        // Add some increment
        let target = base_time + (inc * 3 / 4);
        
        // Don't use more than 10% of remaining time
        let max = time / 10;
        
        // Safety margin - leave at least 50ms
        let target = target.min(time.saturating_sub(50));
        let max = max.min(time.saturating_sub(50));
        
        (target, max.max(target))
    }
    
    /// Check if we should stop searching
    pub fn should_stop(&self) -> bool {
        // Check external stop flag
        if self.stop_flag.load(Ordering::Relaxed) {
            return true;
        }
        
        // Check hard time limit
        self.elapsed_ms() >= self.max_time_ms
    }
    
    /// Check if we have enough time to start another iteration
    pub fn can_start_iteration(&self) -> bool {
        if self.stop_flag.load(Ordering::Relaxed) {
            return false;
        }
        
        // Allow new iteration if under target time
        self.elapsed_ms() < self.target_time_ms
    }
    
    /// Get elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }
    
    /// Get elapsed duration
    #[allow(dead_code)]
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Signal stop
    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }
    
    /// Get the stop flag for sharing
    pub fn stop_flag(&self) -> Arc<AtomicBool> {
        self.stop_flag.clone()
    }
}

// =============================================================================
// Mission 7: Info Reporter
// =============================================================================

/// Info output during search
pub struct InfoReporter {
    /// Whether to report info (can be disabled for tests)
    pub enabled: bool,
}

impl InfoReporter {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    
    /// Report search progress
    #[allow(clippy::too_many_arguments)]
    pub fn report_depth(
        &self,
        depth: u8,
        seldepth: u8,
        score: i32,
        nodes: u64,
        time_ms: u64,
        pv: &[Move],
        hashfull: Option<u32>,
    ) {
        if !self.enabled {
            return;
        }
        
        let nps = if time_ms > 0 {
            nodes * 1000 / time_ms
        } else {
            0
        };
        
        let pv_str: String = pv.iter()
            .map(|m| m.to_uci())
            .collect::<Vec<_>>()
            .join(" ");
        
        // Check for mate score
        let score_str = if score.abs() > 29000 {
            let mate_in = (30000 - score.abs() + 1) / 2;
            if score > 0 {
                format!("mate {}", mate_in)
            } else {
                format!("mate -{}", mate_in)
            }
        } else {
            format!("cp {}", score)
        };
        
        let hashfull_str = if let Some(hf) = hashfull {
            format!(" hashfull {}", hf)
        } else {
            String::new()
        };
        
        println!(
            "info depth {} seldepth {} score {} nodes {} nps {} time {}{}{}",
            depth,
            seldepth,
            score_str,
            nodes,
            nps,
            time_ms,
            hashfull_str,
            if pv_str.is_empty() { String::new() } else { format!(" pv {}", pv_str) }
        );
    }
}

// =============================================================================
// UCI Handler
// =============================================================================

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
    /// Engine options
    options: EngineOptions,
    /// Stop flag for search termination
    stop_flag: Arc<AtomicBool>,
}

impl UciHandler {
    /// Creates a new UCI handler with starting position
    #[inline]
    pub fn new() -> Self {
        Self {
            game: GameState::from_board(Board::default()),
            movegen: MoveGenerator::new(),
            quit: false,
            options: EngineOptions::default(),
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }
    
    /// Get the stop flag for external control
    pub fn stop_flag(&self) -> Arc<AtomicBool> {
        self.stop_flag.clone()
    }
    
    /// Get current options
    pub fn options(&self) -> &EngineOptions {
        &self.options
    }
    
    /// Get current game state
    pub fn game(&self) -> &GameState {
        &self.game
    }
    
    /// Get mutable game state
    pub fn game_mut(&mut self) -> &mut GameState {
        &mut self.game
    }
    
    /// Get move generator
    pub fn movegen(&self) -> &MoveGenerator {
        &self.movegen
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
            "stop" => self.cmd_stop(),
            "setoption" => self.cmd_setoption(input),
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
    
    /// Reset stop flag before starting search
    pub fn reset_stop_flag(&self) {
        self.stop_flag.store(false, Ordering::Relaxed);
    }

    // =========================================================================
    // REQ-1: UCI Handshake
    // =========================================================================

    /// Handle 'uci' command - identify the engine with options
    fn cmd_uci(&self) -> String {
        format!(
            "id name {}\nid author {}\n{}\nuciok",
            ENGINE_NAME, ENGINE_AUTHOR, EngineOptions::print_options()
        )
    }

    /// Handle 'isready' command - synchronization
    fn cmd_isready(&self) -> String {
        "readyok".to_string()
    }
    
    // =========================================================================
    // Mission 7 REQ-2: Stop Command
    // =========================================================================
    
    /// Handle 'stop' command - terminate search
    fn cmd_stop(&self) -> String {
        self.stop_flag.store(true, Ordering::Relaxed);
        String::new()
    }
    
    // =========================================================================
    // Mission 7 REQ-5: UCI Options
    // =========================================================================
    
    /// Handle 'setoption' command
    /// Format: setoption name <name> [value <value>]
    fn cmd_setoption(&mut self, input: &str) -> String {
        // Parse "setoption name X value Y"
        let input = input.strip_prefix("setoption").unwrap_or(input).trim();
        
        if let Some(name_start) = input.find("name ") {
            let after_name = &input[name_start + 5..];
            
            // Find where value starts (if any)
            let (name, value) = if let Some(value_start) = after_name.find(" value ") {
                (&after_name[..value_start], Some(&after_name[value_start + 7..]))
            } else {
                (after_name.trim(), None)
            };
            
            if let Some(val) = value {
                self.options.set_option(name.trim(), val.trim());
            }
        }
        
        String::new()
    }

    // =========================================================================
    // REQ-2: Position Setup
    // =========================================================================

    /// Handle 'ucinewgame' command - reset state
    fn cmd_ucinewgame(&mut self) -> String {
        self.game = GameState::from_board(Board::default());
        // Note: Hash table clearing is handled by caller when creating new searcher
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
    
    // =========================================================================
    // Mission 7 REQ-1: Go Commands Tests
    // =========================================================================
    
    #[test]
    fn req1_parse_go_depth() {
        let params = SearchParams::parse("depth 8");
        assert_eq!(params.depth, Some(8));
        assert!(!params.infinite);
    }
    
    #[test]
    fn req1_parse_go_movetime() {
        let params = SearchParams::parse("movetime 1000");
        assert_eq!(params.movetime, Some(1000));
    }
    
    #[test]
    fn req1_parse_go_wtime_btime() {
        let params = SearchParams::parse("wtime 300000 btime 300000 winc 2000 binc 2000");
        assert_eq!(params.wtime, Some(300000));
        assert_eq!(params.btime, Some(300000));
        assert_eq!(params.winc, Some(2000));
        assert_eq!(params.binc, Some(2000));
    }
    
    #[test]
    fn req1_parse_go_infinite() {
        let params = SearchParams::parse("infinite");
        assert!(params.infinite);
    }
    
    #[test]
    fn req1_parse_go_complex() {
        let params = SearchParams::parse("wtime 60000 btime 55000 winc 1000 binc 1000 movestogo 20");
        assert_eq!(params.wtime, Some(60000));
        assert_eq!(params.btime, Some(55000));
        assert_eq!(params.winc, Some(1000));
        assert_eq!(params.binc, Some(1000));
        assert_eq!(params.movestogo, Some(20));
    }
    
    #[test]
    fn req1_time_manager_fixed_movetime() {
        let params = SearchParams {
            movetime: Some(1000),
            ..Default::default()
        };
        let stop = Arc::new(AtomicBool::new(false));
        let tm = TimeManager::new(&params, Color::White, stop);
        
        assert_eq!(tm.target_time_ms, 1000);
        assert_eq!(tm.max_time_ms, 1000);
    }
    
    #[test]
    fn req1_time_manager_tournament_control() {
        let params = SearchParams {
            wtime: Some(60000),
            btime: Some(60000),
            winc: Some(1000),
            binc: Some(1000),
            ..Default::default()
        };
        let stop = Arc::new(AtomicBool::new(false));
        let tm = TimeManager::new(&params, Color::White, stop);
        
        // Should get reasonable target time
        assert!(tm.target_time_ms > 0);
        assert!(tm.target_time_ms < 60000);
    }
    
    #[test]
    fn req1_time_manager_infinite() {
        let params = SearchParams {
            infinite: true,
            ..Default::default()
        };
        let stop = Arc::new(AtomicBool::new(false));
        let tm = TimeManager::new(&params, Color::White, stop);
        
        assert_eq!(tm.target_time_ms, u64::MAX);
        assert_eq!(tm.max_time_ms, u64::MAX);
    }
    
    // =========================================================================
    // Mission 7 REQ-2: Stop Command Tests
    // =========================================================================
    
    #[test]
    fn req2_stop_flag_terminates() {
        let params = SearchParams {
            infinite: true,
            ..Default::default()
        };
        let stop = Arc::new(AtomicBool::new(false));
        let tm = TimeManager::new(&params, Color::White, stop.clone());
        
        assert!(!tm.should_stop());
        
        stop.store(true, Ordering::Relaxed);
        assert!(tm.should_stop());
    }
    
    #[test]
    fn req2_stop_command_sets_flag() {
        let mut handler = UciHandler::new();
        
        // Stop flag should be false initially
        assert!(!handler.stop_flag().load(Ordering::Relaxed));
        
        // Process stop command
        handler.process_command("stop");
        
        // Stop flag should now be true
        assert!(handler.stop_flag().load(Ordering::Relaxed));
    }
    
    #[test]
    fn req2_reset_stop_flag() {
        let handler = UciHandler::new();
        
        // Set stop flag
        handler.stop_flag().store(true, Ordering::Relaxed);
        assert!(handler.stop_flag().load(Ordering::Relaxed));
        
        // Reset it
        handler.reset_stop_flag();
        assert!(!handler.stop_flag().load(Ordering::Relaxed));
    }
    
    // =========================================================================
    // Mission 7 REQ-3: Info Output Tests
    // =========================================================================
    
    #[test]
    fn req3_info_mate_score_format() {
        // This tests the logic for mate score formatting
        let score: i32 = 29998; // Mate in 1
        let mate_in = (30000 - score.abs() + 1) / 2;
        assert_eq!(mate_in, 1);
        
        let score: i32 = 29996; // Mate in 2
        let mate_in = (30000 - score.abs() + 1) / 2;
        assert_eq!(mate_in, 2);
        
        let score: i32 = -29998; // Getting mated in 1
        let mate_in = (30000 - score.abs() + 1) / 2;
        assert_eq!(mate_in, 1);
    }
    
    // =========================================================================
    // Mission 7 REQ-5: UCI Options Tests
    // =========================================================================
    
    #[test]
    fn req5_option_hash_default() {
        let opts = EngineOptions::default();
        assert_eq!(opts.hash_size_mb, 64);
    }
    
    #[test]
    fn req5_option_hash_set() {
        let mut opts = EngineOptions::default();
        
        assert!(opts.set_option("Hash", "128"));
        assert_eq!(opts.hash_size_mb, 128);
        
        // Test clamping
        assert!(opts.set_option("Hash", "10000"));
        assert_eq!(opts.hash_size_mb, 4096);
        
        assert!(opts.set_option("Hash", "0"));
        assert_eq!(opts.hash_size_mb, 1);
    }
    
    #[test]
    fn req5_setoption_command() {
        let mut handler = UciHandler::new();
        
        handler.process_command("setoption name Hash value 128");
        assert_eq!(handler.options().hash_size_mb, 128);
    }
    
    #[test]
    fn req5_uci_reports_options() {
        let handler = UciHandler::new();
        let response = handler.cmd_uci();
        
        assert!(response.contains("option name Hash"));
        assert!(response.contains("type spin"));
        assert!(response.contains("default 64"));
    }
}
