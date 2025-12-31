# Mission 4: Basic UCI / Debug Shell

## Overview

Minimal UCI (Universal Chess Interface) implementation for interactive debugging and testing. This enables interactive perft testing, position setup debugging, and play-testing with random moves before evaluation/search are implemented.

## Requirements

### REQ-1: UCI Handshake
**Description**: Implement the basic UCI protocol handshake.

**Acceptance Criteria**:
- [x] Respond to `uci` with engine id and `uciok`
- [x] Respond to `isready` with `readyok`
- [x] Engine name and author reported correctly

**Tests**: `req1_uci_handshake`, `req1_isready_readyok`

---

### REQ-2: Position Setup
**Description**: Parse and apply position commands.

**Acceptance Criteria**:
- [x] `position startpos` sets initial position
- [x] `position startpos moves e2e4 e7e5` applies moves
- [x] `position fen <fen>` sets position from FEN
- [x] `position fen <fen> moves ...` sets FEN then applies moves

**Tests**: `req2_position_startpos`, `req2_position_fen`, `req2_position_with_moves`

---

### REQ-3: Position Display (Debug)
**Description**: Display current position for debugging.

**Acceptance Criteria**:
- [x] `d` command prints board in ASCII
- [x] Shows FEN string
- [x] Shows side to move
- [x] Shows castling rights
- [x] Shows hash key

**Tests**: `req3_display_command`

---

### REQ-4: Perft Command
**Description**: Run perft from current position.

**Acceptance Criteria**:
- [x] `go perft <depth>` runs perft and prints node count
- [x] Shows divide output (nodes per move)
- [x] Shows total nodes and time taken

**Tests**: `req4_perft_command`, `req4_perft_startpos`

---

### REQ-5: Basic Go Command
**Description**: Return a legal move when asked to search.

**Acceptance Criteria**:
- [x] `go` returns `bestmove <move>` with a legal move
- [x] Move is in UCI format (e.g., `e2e4`, `e7e8q`)
- [x] Returns first legal move (placeholder until search implemented)

**Tests**: `req5_go_returns_legal_move`, `req5_bestmove_format`

---

### REQ-6: Quit Command
**Description**: Gracefully terminate the engine.

**Acceptance Criteria**:
- [x] `quit` terminates the UCI loop
- [x] Clean exit without errors

**Tests**: `req6_quit_command`

---

## Architecture

### Module Structure
```
src/
├── uci.rs          # UCI protocol handler
└── main.rs         # Entry point, starts UCI loop
```

### Key Components

#### `UciHandler`
Manages UCI state and command processing:
```rust
pub struct UciHandler {
    game: GameState,
    // Future: search, options, etc.
}
```

### Command Flow
```
stdin -> parse_command() -> execute_command() -> stdout
```

## Traceability Matrix

| Requirement | Implementation | Tests | Status |
|-------------|----------------|-------|--------|
| REQ-1: UCI handshake | `UciHandler::cmd_uci()` | `req1_*` | ✅ |
| REQ-2: Position setup | `UciHandler::cmd_position()` | `req2_*` | ✅ |
| REQ-3: Debug display | `UciHandler::cmd_display()` | `req3_*` | ✅ |
| REQ-4: Perft command | `UciHandler::cmd_perft()` | `req4_*` | ✅ |
| REQ-5: Basic go | `UciHandler::cmd_go()` | `req5_*` | ✅ |
| REQ-6: Quit command | `UciHandler::cmd_quit()` | `req6_*` | ✅ |

## Usage Examples

### Interactive Session
```
$ ./prawn
uci
id name prawn 0.1
id author MTDuke71
uciok
isready
readyok
position startpos
d
 +---+---+---+---+---+---+---+---+
 | r | n | b | q | k | b | n | r | 8
 +---+---+---+---+---+---+---+---+
 | p | p | p | p | p | p | p | p | 7
 +---+---+---+---+---+---+---+---+
 |   |   |   |   |   |   |   |   | 6
 +---+---+---+---+---+---+---+---+
 |   |   |   |   |   |   |   |   | 5
 +---+---+---+---+---+---+---+---+
 |   |   |   |   |   |   |   |   | 4
 +---+---+---+---+---+---+---+---+
 |   |   |   |   |   |   |   |   | 3
 +---+---+---+---+---+---+---+---+
 | P | P | P | P | P | P | P | P | 2
 +---+---+---+---+---+---+---+---+
 | R | N | B | Q | K | B | N | R | 1
 +---+---+---+---+---+---+---+---+
   a   b   c   d   e   f   g   h

FEN: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Side to move: White
go perft 5
a2a3: 181046
a2a4: 217832
...
Nodes: 4865609
Time: 142ms
go
bestmove a2a3
quit
```

## Integration

This mission uses:
- Mission 1: Board representation (`Board`)
- Mission 2: Move generation (`MoveGenerator`, `MoveList`)
- Mission 3: Move execution (`GameState`)

## Future Enhancements (Mission 7)
- [ ] `go depth N` with actual search
- [ ] `go movetime N` time-limited search
- [ ] `stop` command for async termination
- [ ] `info` output during search
- [ ] `setoption` for configuration
