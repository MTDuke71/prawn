//! Mission 6: Search Algorithm
//! 
//! Modular, high-performance search with toggleable features for measuring ELO gains.

mod search;
mod transposition;
mod move_ordering;

pub use search::{Searcher, SearchConfig, SearchResult, MoveOrderingConfig};
pub use transposition::{TranspositionTable, TTEntry, Bound};
pub use move_ordering::{order_moves, MoveScore};
