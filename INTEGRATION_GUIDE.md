# Mission Integration Guide

## Integrating Completed Missions into Main Engine

After completing a mission, integrate the code into the main engine using these steps:

### Automated Integration (Recommended)

Run the integration script:

```powershell
.\scripts\integrate-mission.ps1 -MissionNumber 2
```

This will:
1. Copy all source files (*.rs) from `missions/mission2/src/` to `src/` (excluding lib.rs)
2. Copy all test files from `missions/mission2/tests/` to `tests/`
3. Automatically update imports from `mission2_movegen::` to `prawn::`

### Manual Steps After Integration

1. **Update `src/lib.rs`** to declare and re-export the new modules:

```rust
// Add module declarations
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
```

2. **Run quick tests** (skips depth 6 starting position and depth 5 kiwipete):

```bash
cargo test -- --skip perft_startpos_depth_6 --skip perft_kiwipete_depth_5
```

3. **Run full test suite** (includes all tests):

```bash
cargo test
```

4. **Commit the integration**:

```bash
git add src/ tests/
git commit -m "Integrate Mission X into main engine"
```

### Manual Integration (Alternative)

If you prefer manual control:

```powershell
# Copy source files
Copy-Item missions\mission2\src\*.rs src\ -Exclude lib.rs

# Copy test files
Copy-Item missions\mission2\tests\*.rs tests\

# Update test imports
Get-ChildItem tests\*.rs | ForEach-Object {
    (Get-Content $_.FullName) -replace 'mission2_movegen::', 'prawn::' | 
    Set-Content $_.FullName
}
```

## Test Configuration

### Quick Tests (Recommended for Development)
Skip the slowest tests that take minutes to run:

```bash
cargo test -- --skip perft_startpos_depth_6 --skip perft_kiwipete_depth_5
```

**Runtime**: ~1-2 seconds  
**Coverage**: All functionality validated at lower depths

### Full Test Suite
Run everything including deep perft validation:

```bash
cargo test
```

**Runtime**: ~7-10 minutes (includes depth 6 and 5 tests)  
**Coverage**: Comprehensive validation with 312+ million positions

### Continuous Integration
For CI/CD, use quick tests to keep build times reasonable:

```yaml
- name: Run tests
  run: cargo test -- --skip perft_startpos_depth_6 --skip perft_kiwipete_depth_5
```

## Mission Completion Checklist

- [ ] All mission requirements implemented
- [ ] All mission tests passing
- [ ] Mission-specific documentation updated
- [ ] Integration script run successfully
- [ ] `src/lib.rs` updated with new modules
- [ ] Quick test suite passes
- [ ] Full test suite passes (if time permits)
- [ ] Changes committed with clear message
- [ ] Mission marked as complete in tracking

## File Structure

```
prawn/
├── src/                    # Main engine source (integrated code)
│   ├── lib.rs             # Library root - update after each mission
│   ├── board.rs           # Mission 1: Board representation
│   ├── attacks.rs         # Mission 2: Attack generation
│   ├── board_ext.rs       # Mission 2: Board extensions
│   ├── magic.rs           # Mission 2: Magic bitboards
│   ├── movegen.rs         # Mission 2: Move generation
│   └── moves.rs           # Mission 2: Move types
├── tests/                  # Integrated test suite
├── missions/               # Mission-specific development
│   └── mission2/          # Kept for reference
└── scripts/
    └── integrate-mission.ps1  # Automation script
```
