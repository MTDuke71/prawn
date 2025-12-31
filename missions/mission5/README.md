# Mission 5: Position Evaluation

## Overview

Static evaluation function that assigns a numerical score to any chess position. Designed with **feature isolation** - each evaluation component can be independently enabled/disabled to measure its strength contribution.

## Design Philosophy

**Modular Feature Architecture**: Each evaluation term is:
1. Independently toggleable via `EvalConfig`
2. Separately scored for analysis
3. Tunable without affecting other features

This enables:
- Measuring Elo gain per feature
- A/B testing improvements
- Isolating bugs to specific components
- Incremental optimization

## Requirements

### REQ-1: Material Counting
**Description**: Count material value for both sides.

**Acceptance Criteria**:
- [x] Standard piece values (P=100, N=320, B=330, R=500, Q=900)
- [x] Return score from side-to-move perspective
- [x] Can be toggled independently

**Tests**: `req1_material_count_basic`, `req1_material_advantage`, `req1_material_symmetric`

---

### REQ-2: Piece-Square Tables
**Description**: Positional bonuses based on piece location.

**Acceptance Criteria**:
- [x] Separate tables for each piece type
- [x] Mirrored for black pieces
- [x] Opening/middlegame tables
- [x] Can be toggled independently

**Tests**: `req2_pst_knight_center`, `req2_pst_pawn_advancement`, `req2_pst_king_safety`

---

### REQ-3: Pawn Structure Evaluation
**Description**: Evaluate pawn formations.

**Acceptance Criteria**:
- [x] Doubled pawns penalty
- [x] Isolated pawns penalty
- [x] Passed pawns bonus
- [x] Can be toggled independently

**Tests**: `req3_doubled_pawns`, `req3_isolated_pawns`, `req3_passed_pawns`

---

### REQ-4: King Safety Evaluation
**Description**: Evaluate king vulnerability.

**Acceptance Criteria**:
- [x] Pawn shield bonus
- [x] Open file near king penalty
- [x] Can be toggled independently

**Tests**: `req4_pawn_shield`, `req4_open_file_penalty`

---

### REQ-5: Mobility Evaluation
**Description**: Count available moves as a proxy for piece activity.

**Acceptance Criteria**:
- [x] Count legal moves per piece type
- [x] Weight by piece type importance
- [x] Can be toggled independently

**Tests**: `req5_mobility_trapped_piece`, `req5_mobility_active_pieces`

---

### REQ-6: Center Control Evaluation
**Description**: Reward control of central squares.

**Acceptance Criteria**:
- [x] Bonus for pieces attacking e4, d4, e5, d5
- [x] Bonus for pawns controlling center
- [x] Can be toggled independently

**Tests**: `req6_center_control`, `req6_center_pawns`

---

### REQ-7: Tapered Evaluation
**Description**: Blend opening/endgame evaluations based on game phase.

**Acceptance Criteria**:
- [x] Calculate game phase from remaining material
- [x] Interpolate between middlegame and endgame scores
- [x] King centralization bonus in endgame
- [x] Can be toggled independently

**Tests**: `req7_tapered_middlegame`, `req7_tapered_endgame`, `req7_king_endgame`

---

## Architecture

### Feature Configuration
```rust
/// Configuration for which evaluation features are enabled
#[derive(Clone, Copy)]
pub struct EvalConfig {
    pub material: bool,
    pub piece_square_tables: bool,
    pub pawn_structure: bool,
    pub king_safety: bool,
    pub mobility: bool,
    pub center_control: bool,
    pub tapered_eval: bool,
}

impl EvalConfig {
    /// All features enabled (default)
    pub const ALL: Self = Self { /* all true */ };
    
    /// Only material counting (baseline)
    pub const MATERIAL_ONLY: Self = Self { material: true, /* rest false */ };
}
```

### Evaluation Breakdown
```rust
/// Detailed breakdown of evaluation components
pub struct EvalBreakdown {
    pub material: i32,
    pub piece_square: i32,
    pub pawn_structure: i32,
    pub king_safety: i32,
    pub mobility: i32,
    pub center_control: i32,
    pub total: i32,
}
```

### Main Evaluator
```rust
pub struct Evaluator {
    config: EvalConfig,
}

impl Evaluator {
    pub fn evaluate(&self, board: &Board) -> i32;
    pub fn evaluate_breakdown(&self, board: &Board) -> EvalBreakdown;
}
```

## Traceability Matrix

| Requirement | Implementation | Tests | Status |
|-------------|----------------|-------|--------|
| REQ-1: Material | `Evaluator::material()` | `req1_*` | 🔲 |
| REQ-2: PST | `Evaluator::piece_square()` | `req2_*` | 🔲 |
| REQ-3: Pawns | `Evaluator::pawn_structure()` | `req3_*` | 🔲 |
| REQ-4: King safety | `Evaluator::king_safety()` | `req4_*` | 🔲 |
| REQ-5: Mobility | `Evaluator::mobility()` | `req5_*` | 🔲 |
| REQ-6: Center | `Evaluator::center_control()` | `req6_*` | 🔲 |
| REQ-7: Tapered | `Evaluator::tapered_eval()` | `req7_*` | 🔲 |

## Piece Values

Standard centipawn values:
- Pawn: 100
- Knight: 320
- Bishop: 330
- Rook: 500
- Queen: 900
- King: 20000 (for MVV-LVA, not evaluation)

## Performance Considerations

- **Incremental updates**: Future enhancement - update eval incrementally on make/unmake
- **Lazy evaluation**: Some features (mobility) are expensive; consider lazy eval in search
- **SIMD**: PST lookups could use SIMD for batch evaluation

## Usage Examples

```rust
// Full evaluation
let evaluator = Evaluator::new(EvalConfig::ALL);
let score = evaluator.evaluate(&board);

// Material only (for testing baseline)
let evaluator = Evaluator::new(EvalConfig::MATERIAL_ONLY);
let score = evaluator.evaluate(&board);

// Get breakdown for analysis
let breakdown = evaluator.evaluate_breakdown(&board);
println!("Material: {}, PST: {}, Total: {}", 
    breakdown.material, breakdown.piece_square, breakdown.total);

// Custom config - test specific feature
let config = EvalConfig {
    material: true,
    piece_square_tables: true,
    pawn_structure: true,  // Testing this feature
    ..EvalConfig::MATERIAL_ONLY
};
```

## Integration

This mission uses:
- Mission 1: Board representation
- Mission 2: Move generation (for mobility)

Used by:
- Mission 6: Search algorithm (for position scoring)

## Testing Strategy

Each feature should be tested:
1. **In isolation** - only that feature enabled
2. **Incrementally** - added to baseline
3. **Symmetry** - mirrored positions should have equal scores
