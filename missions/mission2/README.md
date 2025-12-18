# Mission 2: Move Generation

## Overview
Implements high-performance legal move generation for all chess pieces using Magic Bitboards for sliding pieces (bishops, rooks, queens). This is the most performance-critical component of the chess engine.

## Requirements

### REQ-1: Pawn Move Generation
**Description**: Generate all legal pawn moves including single push, double push, captures, and en passant.

**Acceptance Criteria**:
- [ ] Single square forward moves (when square is empty)
- [ ] Double square forward moves from starting rank (when both squares empty)
- [ ] Diagonal captures (left and right)
- [ ] En passant captures (when valid)
- [ ] Pawn promotion handling (to Queen, Rook, Bishop, Knight)
- [ ] Cannot move through or land on own pieces

**Performance**: O(1) per pawn using pre-computed attack tables

**Dependencies**: Mission 1 (Board Representation)

**Tests**: `req1_pawn_single_move`, `req1_pawn_double_move`, `req1_pawn_capture`, `req1_en_passant`, `req1_pawn_promotion`

---

### REQ-2: Knight Move Generation
**Description**: Generate all legal knight moves (L-shaped pattern).

**Acceptance Criteria**:
- [ ] All 8 possible L-shaped moves
- [ ] Cannot land on own pieces
- [ ] Works correctly at board edges

**Performance**: O(1) using pre-computed attack table

**Dependencies**: Mission 1

**Tests**: `req2_knight_moves`, `req2_knight_edge_cases`

---

### REQ-3: Bishop Move Generation
**Description**: Generate all legal bishop moves using Magic Bitboards.

**Acceptance Criteria**:
- [ ] All diagonal moves in 4 directions
- [ ] Stops at first blocker (enemy or friendly)
- [ ] Cannot jump over pieces
- [ ] Uses Magic Bitboard lookup for O(1) performance

**Performance**: O(1) using magic bitboard lookup

**Dependencies**: Mission 1

**Tests**: `req3_bishop_moves`, `req3_bishop_blocked`, `req3_bishop_captures`

---

### REQ-4: Rook Move Generation
**Description**: Generate all legal rook moves using Magic Bitboards.

**Acceptance Criteria**:
- [ ] All rank and file moves in 4 directions
- [ ] Stops at first blocker (enemy or friendly)
- [ ] Cannot jump over pieces
- [ ] Uses Magic Bitboard lookup for O(1) performance

**Performance**: O(1) using magic bitboard lookup

**Dependencies**: Mission 1

**Tests**: `req4_rook_moves`, `req4_rook_blocked`, `req4_rook_captures`

---

### REQ-5: Queen Move Generation
**Description**: Generate all legal queen moves (combination of bishop + rook).

**Acceptance Criteria**:
- [ ] All diagonal, rank, and file moves
- [ ] Uses bishop and rook magic bitboard lookups

**Performance**: O(1) using magic bitboard lookup

**Dependencies**: Mission 1, REQ-3, REQ-4

**Tests**: `req5_queen_moves`, `req5_queen_all_directions`

---

### REQ-6: King Move Generation
**Description**: Generate all legal king moves (single square in 8 directions).

**Acceptance Criteria**:
- [ ] All 8 adjacent square moves
- [ ] Cannot land on own pieces
- [ ] Works correctly at board edges

**Performance**: O(1) using pre-computed attack table

**Dependencies**: Mission 1

**Tests**: `req6_king_moves`, `req6_king_edge_cases`

---

### REQ-7: Castling
**Description**: Implement castling move generation with all legality checks.

**Acceptance Criteria**:
- [ ] Kingside castling (O-O)
- [ ] Queenside castling (O-O-O)
- [ ] Check castling rights from FEN
- [ ] King and rook must not have moved
- [ ] Squares between king and rook must be empty
- [ ] King cannot be in check
- [ ] King cannot move through check
- [ ] King cannot land in check

**Performance**: O(1) with pre-computed attack detection

**Dependencies**: Mission 1, REQ-9 (check detection)

**Tests**: `req7_kingside_castle`, `req7_queenside_castle`, `req7_castle_blocked`, `req7_castle_through_check`

---

### REQ-8: Move Validation (No Self-Check)
**Description**: Filter out moves that leave own king in check.

**Acceptance Criteria**:
- [ ] Make move on copy of board
- [ ] Check if own king is in check
- [ ] Only return moves that don't leave king in check
- [ ] Efficient implementation using magic bitboards for attack detection

**Performance**: O(n) where n is number of pseudo-legal moves

**Dependencies**: Mission 1, REQ-9

**Tests**: `req8_illegal_move_into_check`, `req8_pinned_piece`, `req8_discovered_check`

---

### REQ-9: Check Detection
**Description**: Determine if a king is under attack.

**Acceptance Criteria**:
- [ ] Detect attacks from all piece types
- [ ] Use magic bitboards for sliding piece attacks
- [ ] Fast O(1) detection per piece type

**Performance**: O(1) per attacking piece type

**Dependencies**: Mission 1, REQ-1 through REQ-6

**Tests**: `req9_check_detection`, `req9_multiple_attackers`, `req9_no_check`

---

### REQ-10: Checkmate Detection
**Description**: Determine if current position is checkmate.

**Acceptance Criteria**:
- [ ] King must be in check
- [ ] No legal moves available for side to move
- [ ] Returns true only if both conditions met

**Performance**: O(n) where n is number of pieces

**Dependencies**: REQ-8, REQ-9

**Tests**: `req10_checkmate_detected`, `req10_scholars_mate`, `req10_back_rank_mate`, `req10_not_checkmate_can_block`

---

### REQ-11: Stalemate Detection
**Description**: Determine if current position is stalemate.

**Acceptance Criteria**:
- [ ] King must NOT be in check
- [ ] No legal moves available for side to move
- [ ] Returns true only if both conditions met

**Performance**: O(n) where n is number of pieces

**Dependencies**: REQ-8, REQ-9

**Tests**: `req11_stalemate_detected`, `req11_not_stalemate_has_moves`, `req11_not_stalemate_in_check`

---

## Architecture

### Magic Bitboards

Magic Bitboards provide O(1) lookup for sliding piece moves (bishops, rooks, queens).

**Concept**:
1. Pre-compute all possible attack patterns for each square
2. Use magic multiplication and shift to hash occupancy to attack pattern
3. Store attack patterns in lookup table

**Data Structures**:
```rust
pub struct Magic {
    mask: u64,        // Relevant occupancy bits
    magic: u64,       // Magic number for hashing
    shift: u8,        // Shift amount
    offset: u32,      // Offset into attack table
}

pub struct MagicTable {
    magics: [Magic; 64],
    attacks: Vec<u64>,  // Pre-computed attack bitboards
}
```

**Usage**:
```rust
fn bishop_attacks(square: Square, occupancy: u64, magic_table: &MagicTable) -> u64 {
    let magic = &magic_table.magics[square.index()];
    let relevant_occupancy = occupancy & magic.mask;
    let index = ((relevant_occupancy.wrapping_mul(magic.magic)) >> magic.shift) as usize;
    magic_table.attacks[magic.offset as usize + index]
}
```

### Move Structure

```rust
pub struct Move {
    from: Square,
    to: Square,
    move_type: MoveType,
}

pub enum MoveType {
    Quiet,
    DoublePawnPush,
    KingsideCastle,
    QueensideCastle,
    Capture(Piece),
    EnPassant,
    Promotion(PieceType),
    CapturePromotion(Piece, PieceType),
}
```

### Move Generation Flow

```
generate_legal_moves()
  ├─> generate_pseudo_legal_moves()
  │     ├─> generate_pawn_moves()
  │     ├─> generate_knight_moves()
  │     ├─> generate_bishop_moves()  [Magic Bitboard]
  │     ├─> generate_rook_moves()    [Magic Bitboard]
  │     ├─> generate_queen_moves()   [Magic Bitboard]
  │     ├─> generate_king_moves()
  │     └─> generate_castling_moves()
  └─> filter_illegal_moves()  [Check if leaves king in check]
```

## Traceability Matrix

| Requirement | Implementation | Tests | Benchmarks | Status |
|-------------|----------------|-------|------------|--------|
| REQ-1: Pawn moves | `generate_pawn_moves()` | `req1_*` | `pawn_movegen` | ⏳ |
| REQ-2: Knight moves | `generate_knight_moves()` | `req2_*` | `knight_movegen` | ⏳ |
| REQ-3: Bishop moves | `generate_bishop_moves()` | `req3_*` | `bishop_movegen` | ⏳ |
| REQ-4: Rook moves | `generate_rook_moves()` | `req4_*` | `rook_movegen` | ⏳ |
| REQ-5: Queen moves | `generate_queen_moves()` | `req5_*` | `queen_movegen` | ⏳ |
| REQ-6: King moves | `generate_king_moves()` | `req6_*` | `king_movegen` | ⏳ |
| REQ-7: Castling | `generate_castling_moves()` | `req7_*` | N/A | ⏳ |
| REQ-8: Move validation | `filter_illegal_moves()` | `req8_*` | N/A | ⏳ |
| REQ-9: Check detection | `is_square_attacked()` | `req9_*` | `check_detection` | ⏳ |
| REQ-10: Checkmate | `is_checkmate()` | `req10_*` | N/A | ⏳ |
| REQ-11: Stalemate | `is_stalemate()` | `req11_*` | N/A | ⏳ |

## Perft Testing

Perft (performance test) validates move generation correctness by counting all nodes at each depth.

**Standard Positions**:
```
Starting Position:
Depth 1: 20
Depth 2: 400
Depth 3: 8,902
Depth 4: 197,281
Depth 5: 4,865,609
Depth 6: 119,060,324

Kiwipete Position:
r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Depth 1: 48
Depth 2: 2,039
Depth 3: 97,862
Depth 4: 4,085,603
Depth 5: 193,690,690
```

## Performance Targets

- Pawn move generation: < 50 ns per pawn
- Knight move generation: < 10 ns per knight
- Magic bitboard lookup: < 5 ns per lookup
- Full position move generation: < 10 μs for middlegame position
- Perft(6) from starting position: < 5 seconds

## Integration

This mission is used by:
- Mission 3: Move Execution (needs move representation)
- Mission 5: Search Algorithm (needs fast move generation)

## Future Enhancements
- [ ] Staged move generation (tactical moves first)
- [ ] Move ordering hints in Move structure
- [ ] SIMD optimization for attack detection
