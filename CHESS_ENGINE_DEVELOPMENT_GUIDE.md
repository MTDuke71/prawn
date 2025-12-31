# Chess Engine Development Guide
## V-Cycle Methodology for AI-Assisted Chess Engine Development

**Purpose**: This guide establishes the disciplined, test-driven development approach for building a production-quality chess engine in Rust, following the same rigorous V-Cycle methodology proven successful in the Mission series of this repository.

---

## 📋 Core Principles

### 1. **Requirements-First Development**
- Every feature MUST have a numbered requirement (REQ-1, REQ-2, etc.)
- Requirements documented BEFORE implementation
- Each requirement maps to specific tests
- Traceability matrix maintained in README.md

### 2. **Test-Driven Development (TDD)**
- Tests written BEFORE implementation code
- Test names MUST include requirement ID: `req1_board_initialization`, `req2_legal_move_generation`
- Zero tolerance for failing tests
- All code paths covered by tests

### 3. **Incremental Mission-Based Structure**
- Chess engine divided into "missions" (modules/components)
- Each mission is self-contained and independently testable
- Missions build upon validated previous missions
- Clear dependency graph between missions

### 4. **Quality Gates**
- `cargo clippy --workspace -- -D warnings` MUST pass (zero warnings)
- `cargo fmt --all --check` MUST pass
- `cargo test --workspace` MUST pass (100% test success)
- Documentation complete for all public APIs

### 5. **Performance is Critical**
Chess engines are fundamentally about **speed**. The ability to search deeper directly translates to stronger play. Every design decision must consider performance impact.

**Speed Design Principles:**
- **O(1) over O(n)**: Prefer constant-time operations (e.g., mailbox array for piece lookup vs iterating bitboards)
- **Zero allocation in hot paths**: Use fixed-size arrays, avoid `Vec`, `HashMap`, `String` during search
- **Inline critical functions**: Use `#[inline(always)]` on functions called millions of times per second
- **Copy over Clone**: Small structs should be `Copy` for pass-by-value efficiency
- **Static over instance**: Shared lookup tables (magic bitboards, Zobrist keys) should be `static`
- **Cache locality**: Keep frequently-accessed data together in memory

**Performance Targets:**
- Move generation: >30 million nodes/second (release mode)
- Make/unmake move: <100 nanoseconds per operation
- Perft verification: Use to validate both correctness AND speed

**Always benchmark in release mode**: Debug builds are 20-40x slower and give misleading performance data.

---

## 🎯 Chess Engine Mission Structure

### Suggested Mission Breakdown

#### **Mission 1: Board Representation**
**Focus**: Core data structure for chess board state
**Performance**: O(1) piece lookup via hybrid bitboard + mailbox array
- REQ-1: 8x8 board representation with piece placement
- REQ-2: Bitboard support for efficient move generation
- REQ-3: Piece type encoding (pawn, knight, bishop, rook, queen, king)
- REQ-4: Color assignment (white/black)
- REQ-5: FEN (Forsyth-Edwards Notation) parsing and generation
- REQ-6: Board display (ASCII and Unicode)

**Tests**:
- `req1_empty_board_initialization`
- `req2_bitboard_operations`
- `req3_piece_placement`
- `req5_fen_parsing`
- `req5_fen_generation`

#### **Mission 2: Move Generation**
**Focus**: Legal move calculation for all piece types
**Performance**: Magic bitboards for sliding pieces, >30M nodes/sec target
- REQ-1: Pawn moves (single, double, capture, en passant)
- REQ-2: Knight moves (L-shaped)
- REQ-3: Bishop moves (diagonal)
- REQ-4: Rook moves (rank/file)
- REQ-5: Queen moves (combination)
- REQ-6: King moves (single square)
- REQ-7: Castling (kingside, queenside, legality checks)
- REQ-8: Move validation (no self-check)
- REQ-9: Check detection
- REQ-10: Checkmate detection
- REQ-11: Stalemate detection

**Tests**:
- `req1_pawn_single_move`
- `req1_pawn_double_move`
- `req1_pawn_capture`
- `req1_en_passant`
- `req7_kingside_castle`
- `req8_illegal_move_into_check`
- `req10_checkmate_detection`

#### **Mission 3: Move Execution**
**Focus**: Applying moves and maintaining game state
**Performance**: Zero allocation (fixed arrays), static Zobrist tables, <100ns make/unmake
- REQ-1: Make move (update board state)
- REQ-2: Unmake move (restore previous state)
- REQ-3: Move history tracking
- REQ-4: Halfmove clock (50-move rule)
- REQ-5: Fullmove counter
- REQ-6: Zobrist hashing for position identification
- REQ-7: Threefold repetition detection

**Tests**:
- `req1_make_move_updates_board`
- `req2_unmake_move_restores_state`
- `req6_zobrist_hash_uniqueness`
- `req7_threefold_repetition`

#### **Mission 4: Basic UCI / Debug Shell**
**Focus**: Minimal UCI for interactive debugging and testing
**Rationale**: Early UCI enables interactive perft testing, position setup debugging, and play-testing with random moves before evaluation/search are implemented.
- REQ-1: UCI handshake (uci, uciok, isready, readyok)
- REQ-2: Position setup (position startpos, position fen)
- REQ-3: Position display (d command for debugging)
- REQ-4: Perft command (go perft N)
- REQ-5: Basic go command (returns random legal move)
- REQ-6: Quit command

**Tests**:
- `req1_uci_handshake`
- `req2_position_startpos`
- `req2_position_fen`
- `req4_perft_command`
- `req5_go_returns_legal_move`

#### **Mission 5: Position Evaluation**
**Focus**: Static evaluation function
**Performance**: Incremental updates where possible, avoid recomputation
- REQ-1: Material counting (piece values)
- REQ-2: Piece-square tables (positional bonuses)
- REQ-3: Pawn structure evaluation
- REQ-4: King safety evaluation
- REQ-5: Mobility evaluation
- REQ-6: Center control evaluation
- REQ-7: Tapered evaluation (opening/endgame)

**Tests**:
- `req1_material_count_basic`
- `req2_piece_square_bonus`
- `req7_tapered_eval_endgame`

#### **Mission 6: Search Algorithm**
**Focus**: Minimax search with alpha-beta pruning
**Performance**: Move ordering critical for cutoffs, TT for avoiding re-search
- REQ-1: Minimax search (basic)
- REQ-2: Alpha-beta pruning
- REQ-3: Iterative deepening
- REQ-4: Quiescence search
- REQ-5: Transposition table
- REQ-6: Move ordering (MVV-LVA, killer moves, history heuristic)
- REQ-7: Null move pruning
- REQ-8: Late move reduction (LMR)
- REQ-9: Principal variation (PV) tracking

**Tests**:
- `req1_minimax_depth_1`
- `req2_alpha_beta_prune_count`
- `req5_transposition_table_hit`
- `req6_move_ordering_improves_cutoffs`
#### **Mission 7: Full UCI Protocol**
**Focus**: Complete UCI implementation with search integration
- REQ-1: Go depth/movetime/infinite commands
- REQ-2: Stop command (async search termination)
- REQ-3: Info output (depth, score, nodes, nps, pv)
- REQ-4: Bestmove with ponder move
- REQ-5: Option handling (Hash, Threads, etc.)
- REQ-6: UCI_Chess960 support (optional)

**Tests**:
- `req1_go_depth_command`
- `req2_stop_terminates_search`
- `req3_info_output_format`
- `req5_setoption_hash`

#### **Mission 8: Time Management**
**Focus**: Efficient time allocation during search
- REQ-1: Fixed depth search
- REQ-2: Fixed time search
- REQ-3: Tournament time control
- REQ-4: Increment handling
- REQ-5: Time allocation algorithm
- REQ-6: Time safety margin

**Tests**:
- `req2_fixed_time_terminates`
- `req3_tournament_time_allocation`

#### **Mission 9: Opening Book** (Optional)
**Focus**: Opening theory database
- REQ-1: Polyglot book format parsing
- REQ-2: Opening move selection
- REQ-3: Book move weights

#### **Mission 10: Endgame Tablebases** (Optional)
**Focus**: Perfect endgame play
- REQ-1: Syzygy tablebase probing
- REQ-2: DTZ (distance-to-zero) evaluation
- REQ-3: WDL (win-draw-loss) evaluation

---

## 📐 V-Cycle Development Workflow

### Phase 1: Requirements Definition
```markdown
# missions/MissionX/README.md

## Mission X: [Component Name]

### Requirements

#### REQ-1: [Requirement Title]
**Description**: Detailed description of what must be implemented

**Acceptance Criteria**:
- [ ] Criterion 1
- [ ] Criterion 2

**Performance Requirements**: O(1) lookup, O(n log n) sort, etc.

**Dependencies**: Requires Mission Y REQ-Z
```

### Phase 2: Test Creation
```rust
// missions/MissionX/tests/unit_tests.rs

#[test]
fn req1_descriptive_test_name() {
    // Arrange
    let board = Board::new();
    
    // Act
    let result = board.is_valid();
    
    // Assert
    assert!(result, "Board should be valid after initialization");
}

#[test]
fn req1_edge_case_test() {
    // Test edge cases for REQ-1
}

#[test]
fn req2_next_requirement_test() {
    // Tests for REQ-2
}
```

### Phase 3: Implementation
```rust
// missions/MissionX/src/lib.rs

/// Represents a chess board state
/// 
/// # Requirements Satisfied
/// - REQ-1: 8x8 board representation
/// - REQ-3: Piece type encoding
/// 
/// # Examples
/// ```
/// use mission1_board::Board;
/// 
/// let board = Board::new();
/// assert!(board.is_valid());
/// ```
pub struct Board {
    // Implementation
}

impl Board {
    /// Creates a new board in starting position
    /// 
    /// # Requirements Satisfied: REQ-1
    pub fn new() -> Self {
        // Implementation
    }
}
```

### Phase 4: Verification
```bash
# Run all quality checks
cargo test -p missionX              # All tests pass
cargo clippy -p missionX -- -D warnings  # Zero warnings
cargo fmt --check -p missionX       # Properly formatted
cargo doc -p missionX --no-deps     # Documentation builds
```

### Phase 5: Validation
```markdown
# missions/MissionX/README.md

## Traceability Matrix

| Requirement | Implementation | Tests | Status |
|-------------|----------------|-------|--------|
| REQ-1: Board init | `Board::new()` | `req1_empty_board`, `req1_starting_position` | ✅ |
| REQ-2: Bitboards | `Board::bitboards` | `req2_bitboard_ops` | ✅ |
| REQ-5: FEN parse | `Board::from_fen()` | `req5_fen_valid`, `req5_fen_invalid` | ✅ |
```

---

## 🏗️ Repository Structure

```
rust_chess_engine/
├── Cargo.toml                 # Workspace configuration
├── README.md                  # Project overview
├── .github/
│   ├── copilot-instructions.md
│   └── workflows/
│       ├── test.yml          # CI for all tests
│       └── clippy.yml        # CI for linting
├── missions/
│   ├── Mission1_Board/
│   │   ├── Cargo.toml
│   │   ├── README.md         # REQ-1 through REQ-N
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── board.rs
│   │   │   └── pieces.rs
│   │   ├── tests/
│   │   │   ├── unit_tests.rs
│   │   │   └── integration_tests.rs
│   │   ├── benches/
│   │   │   └── board_benchmarks.rs
│   │   └── examples/
│   │       └── board_display.rs
│   ├── Mission2_MoveGen/
│   ├── Mission3_MoveExec/
│   ├── Mission4_Evaluation/
│   ├── Mission5_Search/
│   ├── Mission6_UCI/
│   └── Mission7_TimeManagement/
├── engine/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs           # Main engine binary (integrates all missions)
└── perft_tests/              # Perft testing suite for move generation validation
    ├── positions.txt
    └── perft_runner.rs
```

---

## 🧪 Testing Standards

### Test Naming Convention
```rust
// Format: req{N}_{description}
#[test]
fn req1_board_starts_empty() { }

#[test]
fn req5_fen_parses_starting_position() { }

#[test]
fn req10_checkmate_detected_scholars_mate() { }

// Edge cases
#[test]
fn test_fen_invalid_format_returns_error() { }

// Performance tests
#[test]
fn req2_bitboard_operations_under_100ns() { }
```

### Test Coverage Requirements
- Every public function has at least one test
- Every requirement has at least one test
- Edge cases tested (empty board, full board, boundary conditions)
- Error conditions tested
- Performance requirements validated with benchmarks

### Perft Testing
**Critical for chess engines** - validates move generation correctness:

```rust
// perft_tests/perft_runner.rs

#[test]
fn perft_starting_position_depth_5() {
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    assert_eq!(perft(&board, 5), 4_865_609);
}

#[test]
fn perft_kiwipete_depth_4() {
    let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    assert_eq!(perft(&board, 4), 4_085_603);
}
```

---

## 📊 Quality Standards

### Code Quality
- **Zero clippy warnings** with `-D warnings`
- **Formatted** with `rustfmt`
- **Documented** - all public APIs have rustdoc with examples
- **Type-safe** - leverage Rust's type system (no `unwrap()` in production code)
- **Error handling** - use `Result<T, E>` with `anyhow` or `thiserror`

### Performance Standards
```rust
// Document performance requirements
/// Generates all legal moves for the current position
/// 
/// # Requirements Satisfied: REQ-1, REQ-2, REQ-3
/// 
/// # Performance
/// - Time Complexity: O(n) where n is number of pieces
/// - Target: < 100μs for typical middlegame position
/// - Benchmark: See `benches/move_generation.rs`
pub fn generate_moves(&self) -> Vec<Move> {
    // Implementation
}
```

### Benchmark All Critical Paths
```rust
// missions/Mission2_MoveGen/benches/move_generation.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_move_generation(c: &mut Criterion) {
    let board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    
    c.bench_function("generate_moves_kiwipete", |b| {
        b.iter(|| {
            black_box(board.generate_moves())
        });
    });
}

criterion_group!(benches, benchmark_move_generation);
criterion_main!(benches);
```

---

## 🤖 AI Development Guidelines

### When Creating a New Mission

**Prompt Template**:
```
Create Mission X: [Component Name] following the V-Cycle methodology.

Requirements:
- REQ-1: [First requirement]
- REQ-2: [Second requirement]
...

Follow the structure in .github/CHESS_ENGINE_DEVELOPMENT_GUIDE.md:
1. Create missions/MissionX/README.md with all requirements
2. Create tests/unit_tests.rs with req{N}_* test functions
3. Implement src/lib.rs with /// Requirements Satisfied comments
4. Add examples demonstrating usage
5. Create benchmarks for performance-critical code
6. Update traceability matrix

Ensure:
- All tests pass
- Zero clippy warnings
- Complete documentation
- Perft tests if move generation related
```

### When Implementing a Requirement

**Prompt Template**:
```
Implement REQ-X: [Requirement Title] for Mission Y.

Requirement Details:
[Paste from README.md]

Test-First Approach:
1. Write failing test: req{X}_{description}
2. Implement minimum code to pass test
3. Refactor for clarity
4. Add edge case tests
5. Document with rustdoc

Dependencies: [List required missions/requirements]
Performance Target: [O(n), < Xμs, etc.]
```

### When Debugging a Test Failure

**Prompt Template**:
```
Test `reqX_{test_name}` is failing in Mission Y.

Error:
[Paste error message]

Context:
- Requirement: REQ-X: [Title]
- Expected: [Expected behavior]
- Actual: [Actual behavior]

Please:
1. Identify root cause
2. Fix implementation to pass test
3. Ensure no regressions (all other tests still pass)
4. Update documentation if behavior changed
```

---

## 📚 Documentation Standards

### Mission README Template
```markdown
# Mission X: [Component Name]

## Overview
Brief description of this mission's purpose and scope.

## Requirements

### REQ-1: [Requirement Title]
**Description**: Detailed description

**Acceptance Criteria**:
- [ ] Criterion 1
- [ ] Criterion 2

**Performance**: O(1) lookup

**Dependencies**: Mission Y REQ-Z

**Tests**: `req1_test1`, `req1_test2`

---

### REQ-2: [Next Requirement]
...

## Architecture

### Data Structures
```rust
pub struct Board { ... }
```

### Key Algorithms
- Algorithm 1: Description
- Algorithm 2: Description

## Traceability Matrix

| Requirement | Implementation | Tests | Benchmarks | Status |
|-------------|----------------|-------|------------|--------|
| REQ-1 | `Board::new()` | `req1_*` | N/A | ✅ |
| REQ-2 | `Board::bitboards` | `req2_*` | `bitboard_ops` | ✅ |

## Usage Examples

See `examples/` directory.

## Performance Benchmarks

Run with: `cargo bench -p missionX`

Expected results:
- Operation 1: < 100ns
- Operation 2: < 1μs

## Integration

This mission is used by:
- Mission Y (move generation needs board representation)
- Mission Z (evaluation needs board state)

## Future Enhancements
- [ ] Enhancement 1
- [ ] Enhancement 2
```

### Rustdoc Standards
```rust
/// Represents a move in algebraic notation
/// 
/// # Requirements Satisfied
/// - REQ-3: Move representation
/// - REQ-4: Algebraic notation parsing
/// 
/// # Examples
/// ```
/// use chess_engine::Move;
/// 
/// let m = Move::from_uci("e2e4").unwrap();
/// assert_eq!(m.from(), Square::E2);
/// assert_eq!(m.to(), Square::E4);
/// ```
/// 
/// # Performance
/// - Parsing: O(1)
/// - Validation: O(1)
pub struct Move {
    // Fields
}
```

---

## 🔍 Chess-Specific Testing Requirements

### Perft (Performance Test)
**Required for move generation validation**:

```rust
fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    
    let moves = board.generate_moves();
    let mut nodes = 0;
    
    for m in moves {
        let mut new_board = board.clone();
        new_board.make_move(m);
        nodes += perft(&new_board, depth - 1);
    }
    
    nodes
}

// Known perft values from chessprogramming.org
#[test]
fn perft_startpos_depth_6() {
    let board = Board::new();
    assert_eq!(perft(&board, 6), 119_060_324);
}
```

### Tactical Test Suite
**EPD format positions to validate search**:

```rust
#[test]
fn tactical_test_mate_in_2() {
    // Position: White to move and mate in 2
    let board = Board::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 0 1").unwrap();
    let best_move = engine.search(&board, SearchParams::mate_in(2));
    assert_eq!(best_move.to_uci(), "h5f7"); // Qxf7#
}
```

---

## 🚀 Deployment & Release

### Version Management
- Semantic versioning: `MAJOR.MINOR.PATCH`
- Each mission has independent version
- Engine version tracks latest mission versions

### Release Checklist
- [ ] All tests pass (`cargo test --workspace`)
- [ ] Zero clippy warnings (`cargo clippy --workspace -- -D warnings`)
- [ ] Documentation complete (`cargo doc --workspace --no-deps`)
- [ ] Benchmarks run and within targets
- [ ] Perft tests pass for all known positions
- [ ] Tactical test suite passes
- [ ] UCI protocol compliance verified
- [ ] README.md updated with new features
- [ ] CHANGELOG.md updated
- [ ] Git tag created: `vX.Y.Z`

---

## 📖 Learning Resources

### Chess Programming Wiki
- https://www.chessprogramming.org/
- Essential reference for all algorithms and techniques

### Perft Positions
- https://www.chessprogramming.org/Perft_Results
- Validation data for move generation

### UCI Protocol
- http://wbec-ridderkerk.nl/html/UCIProtocol.html
- Full specification

### Test Positions
- https://www.chessprogramming.org/Test-Positions
- Tactical test suites (WAC, Nolot, etc.)

---

## 🎯 Success Criteria

A mission is considered complete when:

1. ✅ All requirements have numbered IDs
2. ✅ All requirements have tests named `req{N}_*`
3. ✅ All tests pass (100% success rate)
4. ✅ Zero clippy warnings
5. ✅ All public APIs documented with rustdoc
6. ✅ Traceability matrix complete
7. ✅ Performance benchmarks meet targets
8. ✅ Examples demonstrate all major features
9. ✅ Integration with previous missions validated
10. ✅ README.md complete with usage instructions

---

## 🔧 Troubleshooting

### Common Issues

**Test Failures**:
- Review requirement definition - is it clear?
- Check test expectations - are they correct?
- Verify implementation matches specification
- Add debug output to understand actual behavior

**Performance Issues**:
- Profile with `cargo flamegraph`
- Check algorithm complexity - O(n²) when O(n log n) possible?
- Consider better data structures (Vec → HashMap, etc.)
- Add benchmarks to track improvements

**Integration Problems**:
- Review dependency graph - circular dependencies?
- Check version compatibility between missions
- Verify trait implementations are complete
- Use integration tests to catch interface issues

---

## 📝 Example: Mission 1 Walkthrough

### Step 1: Create README with Requirements
```markdown
# Mission 1: Board Representation

## REQ-1: 8x8 Board Initialization
Initialize an empty chess board with 64 squares.

**Tests**: `req1_empty_board`, `req1_board_dimensions`
```

### Step 2: Write Failing Tests
```rust
#[test]
fn req1_empty_board() {
    let board = Board::new();
    assert_eq!(board.piece_at(Square::E4), None);
}
```

### Step 3: Implement Minimum Code
```rust
pub struct Board {
    squares: [Option<Piece>; 64],
}

impl Board {
    /// Creates empty board (REQ-1)
    pub fn new() -> Self {
        Self {
            squares: [None; 64],
        }
    }
    
    pub fn piece_at(&self, sq: Square) -> Option<Piece> {
        self.squares[sq.index()]
    }
}
```

### Step 4: Verify
```bash
cargo test -p mission1_board
cargo clippy -p mission1_board -- -D warnings
```

### Step 5: Update Traceability Matrix
```markdown
| REQ-1 | `Board::new()` | `req1_empty_board` | ✅ |
```

---

## 🎓 Final Notes

This guide ensures that the chess engine is built with:
- **Clarity**: Every requirement is explicit and testable
- **Quality**: Zero tolerance for warnings or failing tests
- **Traceability**: Every line of code maps to a requirement
- **Incrementality**: Missions build on validated foundations
- **Documentation**: Future maintainers understand design decisions
- **Performance**: Benchmarks ensure the engine is competitive

Follow this guide strictly, and you'll produce a chess engine that is:
- ✅ Well-tested and reliable
- ✅ Easy to understand and maintain
- ✅ Performance-optimized
- ✅ Production-ready
- ✅ A portfolio-worthy project demonstrating Rust mastery

**Happy chess engine building! ♟️**
