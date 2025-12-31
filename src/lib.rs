// Library root

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
