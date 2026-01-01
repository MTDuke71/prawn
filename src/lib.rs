// Library root

// Version and build information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ENGINE_NAME: &str = "Prawn";
pub const ENGINE_AUTHOR: &str = "MTDuke71";
pub const BUILD_TIMESTAMP: &str = env!("BUILT_TIMESTAMP");

/// Returns the full engine identification string
pub fn engine_id() -> String {
    format!("{} v{} (built {})", ENGINE_NAME, VERSION, BUILD_TIMESTAMP)
}

// Mission 1: Board Representation
pub mod board;

// Mission 2: Move Generation
pub mod attacks;
pub mod board_ext;
pub mod magic;
pub mod movegen;
pub mod moves;

// Mission 3: Move Execution
pub mod game_state;
pub mod undo_info;
pub mod zobrist;

// Mission 4: Basic UCI / Debug Shell
pub mod uci;

// Mission 5: Position Evaluation
pub mod eval;
mod material;
mod pst;
mod pawn_structure;
mod king_safety;
mod mobility;
mod center_control;

// Mission 6: Search Algorithm
pub mod search;
pub mod transposition;
pub mod move_ordering;

// Re-export main types for convenience
pub use attacks::AttackTables;
pub use board_ext::BoardExt;
pub use magic::MagicTable;
pub use movegen::MoveGenerator;
pub use moves::{Move, MoveList, MoveType};

// Mission 3 re-exports
pub use game_state::GameState;
pub use undo_info::UndoInfo;
pub use zobrist::ZobristHasher;

// Mission 4 re-exports
pub use uci::UciHandler;

// Mission 5 re-exports
pub use eval::{EvalConfig, EvalBreakdown, Evaluator};

// Mission 6 re-exports
pub use search::{Searcher, SearchConfig, SearchResult, SearchLimits, MoveOrderingConfig, MATE_SCORE};
pub use transposition::{TranspositionTable, TTEntry, Bound};
pub use move_ordering::MoveScore;

// Mission 7 re-exports (Full UCI)
pub use uci::{SearchParams, EngineOptions, TimeManager, InfoReporter};
