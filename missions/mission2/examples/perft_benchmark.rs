use mission2_movegen::board::Board;
use mission2_movegen::board_ext::BoardExt;
use mission2_movegen::movegen::MoveGenerator;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

/// Perft function - recursively counts nodes at depth
fn perft(movegen: &MoveGenerator, board: &Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = movegen.generate_legal_moves(board);
    let mut nodes = 0u64;

    for m in moves.moves() {
        let mut new_board = board.clone();
        new_board.make_move_complete(*m);
        nodes += perft(movegen, &new_board, depth - 1);
    }

    nodes
}

fn main() {
    // Create/open log file
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("perft_benchmark.log")
        .expect("Failed to open log file");
    
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(log_file, "\n=== Perft Benchmark Run: {} ===", timestamp).unwrap();
    writeln!(log_file, "Build mode: RELEASE\n").unwrap();
    
    println!("=== Perft Benchmark ===");
    println!("Build mode: RELEASE\n");
    
    let movegen = MoveGenerator::new();
    
    // Starting Position
    println!("Starting Position (rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1)");
    writeln!(log_file, "Starting Position (rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1)").unwrap();
    
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("Failed to parse starting position");
    
    let expected_results = [20, 400, 8902, 197281, 4865609, 119060324];
    
    for depth in 1..=6 {
        let start = Instant::now();
        let result = perft(&movegen, &board, depth);
        let duration = start.elapsed();
        
        let status = if result == expected_results[depth as usize - 1] {
            "✓ PASS"
        } else {
            "✗ FAIL"
        };
        
        let output = format!(
            "  Depth {}: {} nodes in {:.2}s ({:.0} nodes/sec) {}",
            depth,
            result,
            duration.as_secs_f64(),
            result as f64 / duration.as_secs_f64(),
            status
        );
        
        println!("{}", output);
        writeln!(log_file, "{}", output).unwrap();
        
        if result != expected_results[depth as usize - 1] {
            let error = format!("    Expected: {}", expected_results[depth as usize - 1]);
            println!("{}", error);
            writeln!(log_file, "{}", error).unwrap();
        }
    }
    
    // Kiwipete Position
    println!("\nKiwipete Position (r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1)");
    writeln!(log_file, "\nKiwipete Position (r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1)").unwrap();
    
    let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
        .expect("Failed to parse Kiwipete position");
    
    let expected_results = [48, 2039, 97862, 4085603, 193690690];
    
    for depth in 1..=5 {
        let start = Instant::now();
        let result = perft(&movegen, &board, depth);
        let duration = start.elapsed();
        
        let status = if result == expected_results[depth as usize - 1] {
            "✓ PASS"
        } else {
            "✗ FAIL"
        };
        
        let output = format!(
            "  Depth {}: {} nodes in {:.2}s ({:.0} nodes/sec) {}",
            depth,
            result,
            duration.as_secs_f64(),
            result as f64 / duration.as_secs_f64(),
            status
        );
        
        println!("{}", output);
        writeln!(log_file, "{}", output).unwrap();
        
        if result != expected_results[depth as usize - 1] {
            let error = format!("    Expected: {}", expected_results[depth as usize - 1]);
            println!("{}", error);
            writeln!(log_file, "{}", error).unwrap();
        }
    }
    
    println!("\nBenchmark complete! Results logged to perft_benchmark.log");
    writeln!(log_file, "\n=== Benchmark Complete ===\n").unwrap();
}
