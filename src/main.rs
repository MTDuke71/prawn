//! Prawn Chess Engine
//!
//! A chess engine written in Rust.

use prawn::board::{Board, Color, PieceType};
use prawn::{
    EvalConfig, Evaluator, GameState, Move, MoveGenerator, 
    SearchConfig, SearchLimits, Searcher, SearchParams, EngineOptions, InfoReporter
};
use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

const ENGINE_NAME: &str = "prawn 0.1";
const ENGINE_AUTHOR: &str = "MTDuke71";
const DEFAULT_DEPTH: u8 = 6;

fn main() {
    // Check for command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "bench" => run_benchmark(),
            _ => run_uci(),
        }
    } else {
        run_uci();
    }
}

/// Run the UCI protocol loop
fn run_uci() {
    let mut game = GameState::from_board(Board::default());
    let movegen = MoveGenerator::new();
    let evaluator = Evaluator::new(EvalConfig::FAST); // FAST excludes slow mobility eval
    let mut options = EngineOptions::default();
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let input = match line {
            Ok(s) => s,
            Err(_) => break,
        };

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");

        match command {
            "uci" => {
                println!("id name {}", ENGINE_NAME);
                println!("id author {}", ENGINE_AUTHOR);
                println!("{}", EngineOptions::print_options());
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "ucinewgame" => {
                game = GameState::from_board(Board::default());
            }
            "setoption" => {
                // Parse "setoption name X value Y"
                if let Some(name_start) = input.find("name ") {
                    let after_name = &input[name_start + 5..];
                    let (name, value) = if let Some(value_start) = after_name.find(" value ") {
                        (&after_name[..value_start], Some(&after_name[value_start + 7..]))
                    } else {
                        (after_name.trim(), None)
                    };
                    if let Some(val) = value {
                        options.set_option(name.trim(), val.trim());
                    }
                }
            }
            "position" => {
                game = parse_position(input, &movegen);
            }
            "go" => {
                if input.contains("perft") {
                    run_perft(input, &mut game, &movegen);
                } else {
                    // Reset stop flag before search
                    stop_flag.store(false, Ordering::Relaxed);
                    
                    // Parse search parameters
                    let params = SearchParams::parse(input);
                    
                    // Create search limits from parameters
                    let limits = create_search_limits(&params, game.board().side_to_move(), stop_flag.clone());
                    
                    // Create searcher with custom hash size
                    let mut searcher = Searcher::with_hash_size(
                        SearchConfig::DEFAULT, 
                        &movegen, 
                        &evaluator,
                        options.hash_size_mb
                    );
                    
                    // Set up info callback for iterative deepening output
                    let info_reporter = InfoReporter::new(true);
                    searcher.set_info_callback(move |result, time_ms| {
                        let hashfull = None; // Could get from searcher if accessible
                        info_reporter.report_depth(
                            result.depth,
                            result.seldepth,
                            result.score,
                            result.nodes,
                            time_ms,
                            &result.pv,
                            hashfull,
                        );
                    });
                    
                    // Run search
                    let result = searcher.search_with_limits(&mut game, limits);
                    
                    // Output best move
                    if let Some(best) = result.best_move {
                        // Try to get ponder move from PV
                        let ponder = result.pv.get(1).map(|m| format!(" ponder {}", m.to_uci())).unwrap_or_default();
                        println!("bestmove {}{}", best.to_uci(), ponder);
                    } else {
                        println!("bestmove 0000");
                    }
                }
            }
            "stop" => {
                stop_flag.store(true, Ordering::Relaxed);
            }
            "d" => {
                println!("{}", game.board());
                println!("FEN: {}", game.board().to_fen());
                let side = if game.board().side_to_move() == Color::White {
                    "White"
                } else {
                    "Black"
                };
                println!("Side to move: {}", side);
                println!("Hash: {:016x}", game.zobrist_hash());
            }
            "eval" => {
                let breakdown = evaluator.evaluate_breakdown(game.board());
                println!("=== Evaluation Breakdown ===");
                println!("Material:       {:+} cp", breakdown.material);
                println!("Piece-Square:   {:+} cp", breakdown.piece_square);
                println!("Pawn Structure: {:+} cp", breakdown.pawn_structure);
                println!("King Safety:    {:+} cp", breakdown.king_safety);
                println!("Mobility:       {:+} cp", breakdown.mobility);
                println!("Center Control: {:+} cp", breakdown.center_control);
                println!("----------------------------");
                println!("Total:          {:+} cp", breakdown.total);
            }
            "quit" => break,
            _ => {} // Ignore unknown commands
        }

        let _ = stdout.flush();
    }
}

/// Create SearchLimits from UCI SearchParams
fn create_search_limits(params: &SearchParams, side: Color, stop_flag: Arc<AtomicBool>) -> SearchLimits {
    // Fixed depth
    if let Some(depth) = params.depth {
        return SearchLimits {
            max_depth: depth,
            stop_flag: Some(stop_flag),
            start_time: Some(Instant::now()),
            ..Default::default()
        };
    }
    
    // Fixed movetime
    if let Some(movetime) = params.movetime {
        return SearchLimits::movetime(movetime, stop_flag);
    }
    
    // Infinite
    if params.infinite {
        return SearchLimits::infinite(stop_flag);
    }
    
    // Tournament time control
    if params.wtime.is_some() || params.btime.is_some() {
        let (time, inc) = match side {
            Color::White => (params.wtime.unwrap_or(60000), params.winc.unwrap_or(0)),
            Color::Black => (params.btime.unwrap_or(60000), params.binc.unwrap_or(0)),
        };
        
        // Time management formula
        let moves_to_go = params.movestogo.unwrap_or(40) as u64;
        let base_time = time / moves_to_go;
        let target = (base_time + (inc * 3 / 4)).min(time.saturating_sub(50));
        let max = (time / 10).min(time.saturating_sub(50)).max(target);
        
        return SearchLimits::time_control(target, max, stop_flag);
    }
    
    // Default: fixed depth
    SearchLimits {
        max_depth: DEFAULT_DEPTH,
        stop_flag: Some(stop_flag),
        start_time: Some(Instant::now()),
        ..Default::default()
    }
}

/// Parse a UCI position command
fn parse_position(input: &str, movegen: &MoveGenerator) -> GameState {
    let input = input.strip_prefix("position").unwrap_or(input).trim();

    // Find where moves start (if any)
    let (position_part, moves_part) = if let Some(idx) = input.find(" moves ") {
        (&input[..idx], Some(&input[idx + 7..]))
    } else {
        (input, None)
    };

    // Parse the position
    let mut game = if position_part.starts_with("startpos") {
        GameState::from_board(Board::default())
    } else if position_part.starts_with("fen ") {
        let fen = position_part.strip_prefix("fen ").unwrap_or("").trim();
        if let Ok(board) = Board::from_fen(fen) {
            GameState::from_board(board)
        } else {
            GameState::from_board(Board::default())
        }
    } else {
        GameState::from_board(Board::default())
    };

    // Apply moves if any
    if let Some(moves_str) = moves_part {
        for move_str in moves_str.split_whitespace() {
            if let Some(mv) = parse_move(move_str, &game, movegen) {
                game.make_move(mv);
            }
        }
    }

    game
}

/// Parse a UCI move string
fn parse_move(move_str: &str, game: &GameState, movegen: &MoveGenerator) -> Option<Move> {
    if move_str.len() < 4 {
        return None;
    }

    let from = parse_square(&move_str[0..2])?;
    let to = parse_square(&move_str[2..4])?;

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
    let moves = movegen.generate_legal_moves(game.board());

    for mv in moves.moves() {
        if mv.from().index() == from && mv.to().index() == to {
            if let Some(promo) = promotion {
                if mv.is_promotion() && mv.promotion() == Some(promo) {
                    return Some(*mv);
                }
            } else if !mv.is_promotion() || mv.promotion() == Some(PieceType::Queen) {
                // Non-promotion move, or default promotion to queen
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

/// Run perft command
fn run_perft(input: &str, game: &mut GameState, movegen: &MoveGenerator) {
    // Parse depth from "go perft N"
    let depth: u8 = input
        .split_whitespace()
        .skip_while(|&s| s != "perft")
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    let start = Instant::now();
    let mut total_nodes: u64 = 0;

    // Generate moves for divide
    let moves = movegen.generate_legal_moves(game.board());

    for mv in moves.moves() {
        game.make_move(*mv);
        let nodes = perft(game, movegen, depth - 1);
        game.unmake_move();

        println!("{}: {}", mv.to_uci(), nodes);
        total_nodes += nodes;
    }

    let elapsed = start.elapsed();
    let nps = if elapsed.as_secs_f64() > 0.0 {
        (total_nodes as f64 / elapsed.as_secs_f64()) as u64
    } else {
        0
    };

    println!();
    println!("Nodes: {}", total_nodes);
    println!("Time: {}ms", elapsed.as_millis());
    println!("NPS: {}", nps);
}

/// Perft function
fn perft(game: &mut GameState, movegen: &MoveGenerator, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = movegen.generate_legal_moves(game.board());

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes: u64 = 0;
    for mv in moves.moves() {
        game.make_move(*mv);
        nodes += perft(game, movegen, depth - 1);
        game.unmake_move();
    }

    nodes
}

/// Run benchmark
fn run_benchmark() {
    println!("{} - Benchmark", ENGINE_NAME);
    println!();

    let movegen = MoveGenerator::new();

    let positions = [
        (
            "Starting position",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            5,
        ),
        (
            "Kiwipete",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            4,
        ),
        ("Position 3", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 5),
    ];

    let mut total_nodes: u64 = 0;
    let total_start = Instant::now();

    for (name, fen, depth) in positions {
        let board = Board::from_fen(fen).unwrap();
        let mut game = GameState::from_board(board);

        let start = Instant::now();
        let nodes = perft(&mut game, &movegen, depth);
        let elapsed = start.elapsed();

        let nps = if elapsed.as_secs_f64() > 0.0 {
            (nodes as f64 / elapsed.as_secs_f64()) as u64
        } else {
            0
        };

        println!(
            "{} (depth {}): {} nodes in {:?} ({} nps)",
            name, depth, nodes, elapsed, nps
        );

        total_nodes += nodes;
    }

    let total_elapsed = total_start.elapsed();
    let total_nps = if total_elapsed.as_secs_f64() > 0.0 {
        (total_nodes as f64 / total_elapsed.as_secs_f64()) as u64
    } else {
        0
    };

    println!();
    println!(
        "Total: {} nodes in {:?} ({} nps)",
        total_nodes, total_elapsed, total_nps
    );
}
