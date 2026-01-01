# Mission 6: Search Algorithm

## Overview
Implement a modular, high-performance search algorithm with toggleable features for measuring ELO gains.

## Requirements

### REQ-1: Negamax Search (Baseline)
**Description**: Basic negamax search framework
**Acceptance Criteria**:
- [ ] Recursive depth-limited search
- [ ] Returns score from side-to-move perspective
- [ ] Handles checkmate/stalemate terminal nodes
**Performance**: Baseline for comparison

### REQ-2: Alpha-Beta Pruning
**Description**: Prune branches that cannot affect the result
**Acceptance Criteria**:
- [ ] Correct pruning (same result as plain negamax)
- [ ] Tracks nodes searched for measuring efficiency
- [ ] Significant reduction in nodes vs plain negamax
**Performance**: ~90%+ node reduction at depth 6

### REQ-3: Iterative Deepening
**Description**: Search progressively deeper, using previous results
**Acceptance Criteria**:
- [ ] Search depth 1, then 2, then 3...
- [ ] Returns best move from deepest completed search
- [ ] Can be interrupted at any depth (for time control)
- [ ] Reports info at each depth (for UCI)
**Performance**: Minimal overhead vs direct search

### REQ-4: Quiescence Search
**Description**: Extend search at tactical positions to avoid horizon effect
**Acceptance Criteria**:
- [ ] Searches captures at leaf nodes
- [ ] Stand-pat evaluation (option to not capture)
- [ ] Configurable depth limit
**Performance**: Better tactical accuracy, some speed cost

### REQ-5: Transposition Table
**Description**: Cache positions to avoid re-searching identical positions
**Acceptance Criteria**:
- [ ] Fixed-size hash table using Zobrist keys
- [ ] Store: score, depth, bound type (exact/lower/upper), best move
- [ ] Correct retrieval and cutoffs
- [ ] Replacement strategy (depth-preferred or always-replace)
**Performance**: Significant node reduction in positions with transpositions

### REQ-6: Move Ordering
**Description**: Search better moves first for more alpha-beta cutoffs
**Acceptance Criteria**:
- [ ] MVV-LVA (Most Valuable Victim - Least Valuable Attacker) for captures
- [ ] Killer moves (quiet moves that caused cutoffs)
- [ ] History heuristic (moves that historically cause cutoffs)
- [ ] TT move first (from transposition table)
**Performance**: Dramatically improves alpha-beta efficiency

### REQ-7: Null Move Pruning
**Description**: Skip a turn to prove position is winning
**Acceptance Criteria**:
- [ ] Reduced depth search after passing
- [ ] Verification search to avoid zugzwang errors
- [ ] Disabled in endgames and when in check
**Performance**: Large node reduction in non-tactical positions

### REQ-8: Late Move Reduction (LMR)
**Description**: Reduce search depth for moves unlikely to be good
**Acceptance Criteria**:
- [ ] Full search for first N moves
- [ ] Reduced search for later moves
- [ ] Re-search if reduced search returns surprisingly good score
**Performance**: Deeper effective search depth

### REQ-9: Principal Variation Tracking
**Description**: Track the best line of play
**Acceptance Criteria**:
- [ ] Extract PV for UCI info output
- [ ] PV-node vs non-PV-node distinction
**Performance**: Minimal overhead

## Modular Configuration

```rust
pub struct SearchConfig {
    pub alpha_beta: bool,        // REQ-2
    pub iterative_deepening: bool, // REQ-3
    pub quiescence: bool,        // REQ-4
    pub transposition_table: bool, // REQ-5
    pub move_ordering: MoveOrderingConfig, // REQ-6
    pub null_move_pruning: bool, // REQ-7
    pub lmr: bool,               // REQ-8
    pub aspiration_windows: bool, // Advanced
}

pub struct MoveOrderingConfig {
    pub mvv_lva: bool,
    pub killer_moves: bool,
    pub history_heuristic: bool,
    pub tt_move: bool,
}
```

## Performance Targets
- Baseline negamax at depth 6: measure nodes
- With alpha-beta: >90% node reduction
- With move ordering: additional 50%+ reduction
- With TT: 20-40% reduction (position dependent)
- With LMR: reach depth 8-10 in same time as depth 6

## Testing Strategy
1. Correctness tests: same best move as reference at low depths
2. Efficiency tests: count nodes, verify reductions
3. Tactical tests: find mate-in-N, find winning captures
4. Benchmark: nodes per second, effective depth

## Dependencies
- Mission 2: Move generation
- Mission 3: Game state (make/unmake move)
- Mission 5: Position evaluation
