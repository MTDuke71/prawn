//! Mission 5: Position Evaluation
//!
//! Modular static position evaluation with independently toggleable features.
//! Each evaluation component can be enabled/disabled to measure its strength contribution.

mod eval;
mod material;
mod pst;
mod pawn_structure;
mod king_safety;
mod mobility;
mod center_control;

pub use eval::{EvalConfig, EvalBreakdown, Evaluator};
