# Mission 3: Move Execution

## Overview
This mission implements the move execution system, enabling the chess engine to apply and undo moves while maintaining complete game state. This includes move history, game counters (halfmove clock, fullmove counter), Zobrist hashing for position identification, and threefold repetition detection.

## Requirements

### REQ-1: Make Move (Update Board State)
**Description**: Implement the ability to apply a move to the board, updating all relevant state including piece positions, side to move, castling rights, en passant square, and move counters.

**Acceptance Criteria**:
- [ ] Piece is moved from source square to destination square
- [ ] Captures remove the captured piece from the board
- [ ] Special moves handled correctly (castling, en passant, promotion)
- [ ] Side to move is switched
- [ ] Castling rights updated when king/rook moves or rook is captured
- [ ] En passant square set for double pawn pushes
- [ ] Halfmove clock incremented (or reset on pawn move/capture)
- [ ] Fullmove counter incremented after Black's move

**Performance**: O(1) time complexity

**Dependencies**: Mission 2 (Move representation and move generation)

**Tests**: `req1_make_move_updates_board`, `req1_make_move_capture`, `req1_make_move_castling`, `req1_make_move_en_passant`, `req1_make_move_promotion`

---

### REQ-2: Unmake Move (Restore Previous State)
**Description**: Implement the ability to undo a move, restoring the board to its exact previous state. This is critical for search algorithms which need to explore move trees efficiently.

**Acceptance Criteria**:
- [ ] Piece is moved back to original square
- [ ] Captured pieces are restored
- [ ] Special moves are undone correctly (castling, en passant, promotion)
- [ ] Side to move is switched back
- [ ] All game state restored (castling rights, en passant, clocks, counters)
- [ ] Multiple unmakes can be performed in sequence (undo move history)

**Performance**: O(1) time complexity

**Dependencies**: REQ-1

**Tests**: `req2_unmake_move_restores_state`, `req2_unmake_move_capture`, `req2_unmake_move_castling`, `req2_unmake_move_en_passant`, `req2_make_unmake_sequence`

---

### REQ-3: Move History Tracking
**Description**: Maintain a history of all moves played in the game, enabling move unmake and game replay functionality.

**Acceptance Criteria**:
- [ ] Each move is stored with complete state information (captured piece, castling rights, etc.)
- [ ] History can be traversed forward and backward
- [ ] History supports arbitrary depth (limited only by memory)
- [ ] Position can be restored to any point in history

**Performance**: O(1) push/pop operations

**Dependencies**: REQ-1, REQ-2

**Tests**: `req3_move_history_tracking`, `req3_move_history_depth`, `req3_restore_from_history`

---

### REQ-4: Halfmove Clock (50-Move Rule)
**Description**: Track the number of halfmoves since the last pawn move or capture. Used to enforce the 50-move rule (game is drawn if 50 moves occur without pawn move or capture).

**Acceptance Criteria**:
- [ ] Halfmove clock increments on each move
- [ ] Halfmove clock resets to 0 on pawn moves
- [ ] Halfmove clock resets to 0 on captures
- [ ] Halfmove clock is preserved/restored on make/unmake
- [ ] Can detect 50-move rule condition (halfmove clock >= 100)

**Performance**: O(1)

**Dependencies**: REQ-1

**Tests**: `req4_halfmove_clock_increment`, `req4_halfmove_clock_reset_pawn`, `req4_halfmove_clock_reset_capture`, `req4_fifty_move_rule`

---

### REQ-5: Fullmove Counter
**Description**: Track the fullmove number, starting at 1 and incrementing after each Black move.

**Acceptance Criteria**:
- [ ] Fullmove number starts at 1
- [ ] Fullmove number increments after Black's move
- [ ] Fullmove number is preserved/restored on make/unmake
- [ ] Fullmove number correctly reflected in FEN output

**Performance**: O(1)

**Dependencies**: REQ-1

**Tests**: `req5_fullmove_counter_starts_one`, `req5_fullmove_counter_increment`, `req5_fullmove_counter_unmake`

---

### REQ-6: Zobrist Hashing for Position Identification
**Description**: Implement Zobrist hashing to generate unique 64-bit hash values for chess positions. This enables efficient position comparison and is essential for transposition tables and repetition detection.

**Acceptance Criteria**:
- [ ] Each position has a unique hash (collision probability negligible)
- [ ] Hash is incrementally updated during make/unmake (no full recalculation)
- [ ] Hash incorporates piece positions, side to move, castling rights, en passant
- [ ] Hash can be computed from scratch for validation
- [ ] Random numbers used for Zobrist initialization are deterministic (seeded)

**Performance**: O(1) incremental update, O(n) full calculation where n = number of pieces

**Dependencies**: REQ-1, REQ-2

**Tests**: `req6_zobrist_hash_uniqueness`, `req6_zobrist_incremental_update`, `req6_zobrist_same_position_same_hash`, `req6_zobrist_different_position_different_hash`

---

### REQ-7: Threefold Repetition Detection
**Description**: Detect when the same position has occurred three times in a game (threefold repetition), which allows a draw to be claimed.

**Acceptance Criteria**:
- [ ] Track position hashes throughout the game
- [ ] Detect when a position occurs for the third time
- [ ] Only count positions with same side to move, castling rights, en passant
- [ ] Repetitions need not be consecutive
- [ ] Detection works correctly with make/unmake

**Performance**: O(1) per move (using hash map)

**Dependencies**: REQ-3, REQ-6

**Tests**: `req7_threefold_repetition`, `req7_threefold_non_consecutive`, `req7_no_false_positives`

---

## Architecture

### Data Structures

```rust
/// Stores all information needed to unmake a move
pub struct UndoInfo {
    pub captured_piece: Option<Piece>,
    pub castling_rights: u8,
    pub en_passant_square: Option<Square>,
    pub halfmove_clock: u32,
    pub zobrist_hash: u64,
}

/// Game state with move execution capabilities
pub struct GameState {
    board: Board,
    zobrist: ZobristHasher,
    move_history: Vec<Move>,
    undo_stack: Vec<UndoInfo>,
    position_history: HashMap<u64, u32>, // hash -> count
}
```

### Key Algorithms

- **Zobrist Hashing**: XOR-based incremental hashing for position identification
- **Move Make/Unmake**: Efficient state updates with full restoration capability
- **Repetition Detection**: Hash-based position tracking with occurrence counting

## Traceability Matrix

| Requirement | Implementation | Tests | Status |
|-------------|----------------|-------|--------|
| REQ-1: Make move | `GameState::make_move()` | `req1_make_move_updates_board`, `req1_make_move_capture`, `req1_make_move_castling`, `req1_make_move_en_passant`, `req1_make_move_promotion` | ✅ |
| REQ-2: Unmake move | `GameState::unmake_move()` | `req2_unmake_move_restores_state`, `req2_unmake_move_capture`, `req2_unmake_move_castling`, `req2_unmake_move_en_passant`, `req2_make_unmake_sequence` | ✅ |
| REQ-3: Move history | `GameState::move_history` | `req3_move_history_tracking`, `req3_move_history_depth`, `req3_restore_from_history` | ✅ |
| REQ-4: Halfmove clock | `GameState::make_move()` | `req4_halfmove_clock_increment`, `req4_halfmove_clock_reset_pawn`, `req4_halfmove_clock_reset_capture`, `req4_fifty_move_rule` | ✅ |
| REQ-5: Fullmove counter | `GameState::make_move()` | `req5_fullmove_counter_starts_one`, `req5_fullmove_counter_increment`, `req5_fullmove_counter_unmake` | ✅ |
| REQ-6: Zobrist hashing | `ZobristHasher` | `req6_zobrist_hash_uniqueness`, `req6_zobrist_incremental_update`, `req6_zobrist_same_position_same_hash`, `req6_zobrist_different_position_different_hash` | ✅ |
| REQ-7: Repetition detection | `GameState::is_threefold_repetition()` | `req7_threefold_repetition`, `req7_threefold_non_consecutive`, `req7_no_false_positives` | ✅ |

## Usage Examples

```rust
use mission3_move_execution::{GameState, ZobristHasher};
use prawn::board::Board;
use mission2_movegen::Move;

// Create a new game state
let board = Board::starting_position();
let zobrist = ZobristHasher::new();
let mut game = GameState::new(board, zobrist);

// Make a move
let e2_e4 = Move::new_quiet(Square::E2, Square::E4);
game.make_move(e2_e4);

// Check for repetitions
if game.is_threefold_repetition() {
    println!("Draw by threefold repetition!");
}

// Unmake the move
game.unmake_move();
```

## Integration

This mission is used by:
- Mission 5 (Search algorithm needs make/unmake for tree exploration)
- Mission 6 (UCI protocol needs position setup and move execution)
- Future missions (evaluation, time management, etc.)

## Future Enhancements
- [ ] Perft debugging with move path tracking
- [ ] Move legality validation during make_move
- [ ] Efficient hash table for larger position histories
- [ ] Support for 960 (Chess960/Fischer Random)
