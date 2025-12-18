// Mission 2: Move Generation
// High-performance legal move generation using Magic Bitboards

// Re-export from Mission 1
pub use prawn::board;

// Mission 2 modules
pub mod attacks;
pub mod board_ext;
pub mod magic;
pub mod movegen;
pub mod moves;

// Re-export main types
pub use attacks::AttackTables;
pub use magic::MagicTable;
pub use movegen::MoveGenerator;
pub use moves::{Move, MoveList, MoveType};
