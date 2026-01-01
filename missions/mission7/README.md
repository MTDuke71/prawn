# Mission 7: Full UCI Protocol

## Overview
Complete UCI (Universal Chess Interface) implementation with search integration for GUI and tournament compatibility.

## Requirements

### REQ-1: Go Commands with Time Control
**Description**: Parse and handle all UCI go command variations for search control.

**Acceptance Criteria**:
- [x] `go depth N` - Search to fixed depth
- [ ] `go movetime N` - Search for N milliseconds
- [ ] `go wtime N btime N [winc N] [binc N]` - Tournament time control
- [ ] `go infinite` - Search until stop command
- [ ] `go nodes N` - Search N nodes (optional)
- [ ] `go ponder` - Ponder during opponent's time

**Tests**: `req1_go_depth`, `req1_go_movetime`, `req1_go_wtime_btime`, `req1_go_infinite`

---

### REQ-2: Stop Command (Async Search Termination)
**Description**: Ability to stop search immediately and return best move found so far.

**Acceptance Criteria**:
- [ ] `stop` command terminates search immediately
- [ ] Returns best move found at interrupted depth
- [ ] Thread-safe stop flag using `AtomicBool`
- [ ] Graceful handling of stop during any search phase

**Performance**: Stop response < 10ms

**Tests**: `req2_stop_terminates_search`, `req2_stop_returns_best_move`

---

### REQ-3: Info Output During Search
**Description**: Send info strings during search with statistics and PV.

**Acceptance Criteria**:
- [ ] Output info at each iteration of iterative deepening
- [ ] Include: `depth`, `seldepth`, `score cp/mate`, `nodes`, `nps`, `time`, `pv`
- [ ] Support `score mate N` format when mate found
- [ ] Include `hashfull` from transposition table
- [ ] Output `currmove` and `currmovenumber` (optional)

**Format**: `info depth 6 seldepth 12 score cp 35 nodes 58000 nps 3600000 time 16 pv e2e4 e7e5 g1f3`

**Tests**: `req3_info_depth_score`, `req3_info_mate_score`, `req3_info_pv_format`

---

### REQ-4: Bestmove Output
**Description**: Output best move in correct UCI format with optional ponder move.

**Acceptance Criteria**:
- [x] Output `bestmove <move>` after search
- [ ] Support `bestmove <move> ponder <move>` format
- [ ] Handle null move (0000) for no legal moves
- [ ] Correct promotion format (e.g., e7e8q)

**Tests**: `req4_bestmove_format`, `req4_bestmove_with_ponder`

---

### REQ-5: UCI Options
**Description**: Support configurable engine options via setoption command.

**Acceptance Criteria**:
- [ ] `option name Hash type spin default 64 min 1 max 4096`
- [ ] `option name Threads type spin default 1 min 1 max 1` (single-threaded initially)
- [ ] `setoption name Hash value N` - Resize transposition table
- [ ] Report options in UCI handshake
- [ ] Clear Hash on `ucinewgame`

**Tests**: `req5_option_hash`, `req5_setoption_hash`, `req5_ucinewgame_clears_hash`

---

### REQ-6: UCI_Chess960 Support (Optional)
**Description**: Support for Fischer Random Chess (Chess960).

**Acceptance Criteria**:
- [ ] `option name UCI_Chess960 type check default false`
- [ ] Parse Chess960 castling in FEN
- [ ] Correct castling move format for Chess960

**Tests**: `req6_chess960_option`, `req6_chess960_fen`

---

## Architecture

### Data Structures

```rust
/// Search parameters from UCI go command
pub struct SearchParams {
    pub depth: Option<u8>,        // Fixed depth limit
    pub movetime: Option<u64>,    // Fixed time in ms
    pub wtime: Option<u64>,       // White time remaining
    pub btime: Option<u64>,       // Black time remaining
    pub winc: Option<u64>,        // White increment
    pub binc: Option<u64>,        // Black increment
    pub infinite: bool,           // Search until stop
    pub ponder: bool,             // Pondering mode
}

/// Engine options
pub struct EngineOptions {
    pub hash_size_mb: usize,      // TT size in MB
    pub threads: usize,           // Number of threads (future)
}
```

### Key Changes to Searcher

1. Add `stop: Arc<AtomicBool>` for async termination
2. Add callback/channel for info output during search
3. Check stop flag in search loop
4. Time management integration

### Time Management Algorithm

```rust
fn calculate_search_time(params: &SearchParams, side: Color) -> u64 {
    if params.infinite { return u64::MAX; }
    if let Some(movetime) = params.movetime { return movetime; }
    
    let (time, inc) = match side {
        White => (params.wtime, params.winc),
        Black => (params.btime, params.binc),
    };
    
    // Simple formula: use 2.5% of remaining time + increment
    let time = time.unwrap_or(60000);
    let inc = inc.unwrap_or(0);
    
    (time / 40) + inc
}
```

## Traceability Matrix

| Requirement | Implementation | Tests | Status |
|-------------|----------------|-------|--------|
| REQ-1: Go commands | `parse_go()` | `req1_*` | 🔨 In Progress |
| REQ-2: Stop command | `Searcher::stop` | `req2_*` | 🔨 In Progress |
| REQ-3: Info output | `InfoReporter` | `req3_*` | 🔨 In Progress |
| REQ-4: Bestmove | `bestmove_output()` | `req4_*` | ✅ Partial |
| REQ-5: UCI options | `EngineOptions` | `req5_*` | 🔨 In Progress |
| REQ-6: Chess960 | N/A | `req6_*` | ⏳ Deferred |

## Implementation Plan

1. **Phase 1**: Add stop flag and time tracking to Searcher
2. **Phase 2**: Parse all go command parameters
3. **Phase 3**: Implement time management algorithm
4. **Phase 4**: Add info output during iterative deepening
5. **Phase 5**: Implement UCI options
6. **Phase 6**: Threaded search (for stop command)

## Usage Examples

```
# Fixed depth search
position startpos
go depth 8

# Fixed time search (1 second)
position startpos
go movetime 1000

# Tournament time control
position startpos moves e2e4 e7e5
go wtime 300000 btime 300000 winc 2000 binc 2000

# Infinite analysis
position fen r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
go infinite
# ... wait ...
stop

# Set hash size
setoption name Hash value 128
```

## Dependencies
- Mission 6: Search Algorithm (complete)
- Mission 5: Position Evaluation (complete)
- Mission 4: Basic UCI/Debug Shell (complete)

## Performance Targets
- Stop response: < 10ms
- Info output overhead: < 1% of search time
- Time management accuracy: ±10% of target time
