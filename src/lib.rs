// Library root

// Mission 1: Board Representation
pub mod board;

// Mission 2: Move Generation
pub mod attacks;
pub mod board_ext;
pub mod magic;
pub mod movegen;
pub mod moves;

// Re-export main types for convenience
pub use attacks::AttackTables;
pub use board_ext::BoardExt;
pub use magic::MagicTable;
pub use movegen::MoveGenerator;
pub use moves::{Move, MoveList, MoveType};
