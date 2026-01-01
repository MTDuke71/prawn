# Prawn Chess Engine - Test Plan

## Overview

This document outlines the comprehensive testing strategy for Prawn chess engine v0.1.0+. Testing is critical for chess engines as bugs can manifest as illegal moves, incorrect evaluations, or search failures that are difficult to debug.

---

## 1. Unit Tests

### 1.1 Board Representation (`board.rs`)
| Test ID | Description | Command |
|---------|-------------|---------|
| UT-B01 | FEN parsing - starting position | `cargo test --release fen_parsing` |
| UT-B02 | FEN parsing - complex positions | `cargo test --release fen` |
| UT-B03 | FEN generation roundtrip | `cargo test --release fen_roundtrip` |
| UT-B04 | Piece placement/removal | `cargo test --release piece` |
| UT-B05 | Castling rights tracking | `cargo test --release castling` |
| UT-B06 | En passant square tracking | `cargo test --release en_passant` |

### 1.2 Move Generation (`movegen.rs`)
| Test ID | Description | Command |
|---------|-------------|---------|
| UT-M01 | Pawn moves (single, double, capture) | `cargo test --release pawn` |
| UT-M02 | Knight moves | `cargo test --release knight` |
| UT-M03 | Bishop moves (sliding) | `cargo test --release bishop` |
| UT-M04 | Rook moves (sliding) | `cargo test --release rook` |
| UT-M05 | Queen moves | `cargo test --release queen` |
| UT-M06 | King moves | `cargo test --release king` |
| UT-M07 | Castling legality | `cargo test --release castle` |
| UT-M08 | En passant captures | `cargo test --release en_passant` |
| UT-M09 | Promotion moves | `cargo test --release promotion` |
| UT-M10 | Check detection | `cargo test --release check` |
| UT-M11 | Checkmate detection | `cargo test --release checkmate` |
| UT-M12 | Stalemate detection | `cargo test --release stalemate` |

### 1.3 Move Execution (`game_state.rs`)
| Test ID | Description | Command |
|---------|-------------|---------|
| UT-E01 | Make move updates board | `cargo test --release make_move` |
| UT-E02 | Unmake move restores state | `cargo test --release unmake` |
| UT-E03 | Zobrist hash consistency | `cargo test --release zobrist` |
| UT-E04 | Halfmove clock (50-move rule) | `cargo test --release halfmove` |
| UT-E05 | Threefold repetition | `cargo test --release repetition` |

### 1.4 Evaluation (`eval.rs`)
| Test ID | Description | Command |
|---------|-------------|---------|
| UT-V01 | Material counting | `cargo test --release material` |
| UT-V02 | Piece-square tables | `cargo test --release pst` |
| UT-V03 | Pawn structure | `cargo test --release pawn_structure` |
| UT-V04 | King safety | `cargo test --release king_safety` |
| UT-V05 | Mobility | `cargo test --release mobility` |
| UT-V06 | Symmetric evaluation | `cargo test --release symmetric` |

### 1.5 Search (`search.rs`)
| Test ID | Description | Command |
|---------|-------------|---------|
| UT-S01 | Alpha-beta correctness | `cargo test --release alpha_beta` |
| UT-S02 | Transposition table | `cargo test --release transposition` |
| UT-S03 | Move ordering | `cargo test --release move_ordering` |
| UT-S04 | Quiescence search | `cargo test --release quiescence` |
| UT-S05 | Iterative deepening | `cargo test --release iterative` |

### 1.6 UCI Protocol (`uci.rs`)
| Test ID | Description | Command |
|---------|-------------|---------|
| UT-U01 | UCI handshake | `cargo test --release uci_handshake` |
| UT-U02 | Position commands | `cargo test --release position` |
| UT-U03 | Go command parsing | `cargo test --release go_command` |
| UT-U04 | Stop command | `cargo test --release stop` |
| UT-U05 | Option handling | `cargo test --release setoption` |

**Run all unit tests:**
```bash
cargo test --release
```

---

## 2. Perft Tests (Move Generation Validation)

Perft (performance test) counts all leaf nodes at a given depth. Known correct values validate move generation.

### 2.1 Standard Positions

| Position | Depth | Expected Nodes | Command |
|----------|-------|----------------|---------|
| Starting | 1 | 20 | `echo "position startpos\ngo perft 1\nquit" \| prawn.exe` |
| Starting | 2 | 400 | `echo "position startpos\ngo perft 2\nquit" \| prawn.exe` |
| Starting | 3 | 8,902 | `echo "position startpos\ngo perft 3\nquit" \| prawn.exe` |
| Starting | 4 | 197,281 | `echo "position startpos\ngo perft 4\nquit" \| prawn.exe` |
| Starting | 5 | 4,865,609 | `echo "position startpos\ngo perft 5\nquit" \| prawn.exe` |
| Starting | 6 | 119,060,324 | `echo "position startpos\ngo perft 6\nquit" \| prawn.exe` |

### 2.2 Complex Positions (Kiwipete, etc.)

| Position | FEN | Depth | Expected |
|----------|-----|-------|----------|
| Kiwipete | `r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -` | 4 | 4,085,603 |
| Position 3 | `8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -` | 5 | 674,624 |
| Position 4 | `r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq -` | 4 | 422,333 |
| Position 5 | `rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ -` | 4 | 2,103,487 |

### 2.3 Edge Case Positions

| Test Case | FEN | Notes |
|-----------|-----|-------|
| En passant pin | `8/8/8/2k5/3Pp3/8/8/4K2R b - d3 0 1` | EP capture leaves king in check |
| Castling through check | `r3k2r/8/8/8/8/8/8/R3K2R w KQkq -` | Cannot castle through attacked square |
| Double check | `rnbqkb1r/pppp1ppp/5n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq -` | Only king can move |
| Promotion with check | `8/P7/8/8/8/8/8/k1K5 w - -` | All four promotion types |

---

## 3. Integration Tests

### 3.1 UCI Protocol Integration

```bash
# Test 1: Full game simulation
echo "uci
isready
ucinewgame
position startpos moves e2e4 e7e5 g1f3 b8c6
go depth 8
quit" | prawn.exe

# Test 2: Time control
echo "uci
isready
position startpos
go wtime 60000 btime 60000 winc 1000 binc 1000
quit" | prawn.exe

# Test 3: Stop command (needs async)
# Start search, send stop after 1 second
```

### 3.2 Hash Table Integration
```bash
# Test different hash sizes
echo "uci
setoption name Hash value 16
setoption name Hash value 128
setoption name Hash value 512
isready
quit" | prawn.exe
```

---

## 4. Tactical Test Suites

### 4.1 Mate-in-N Puzzles

| Puzzle | FEN | Solution | Depth |
|--------|-----|----------|-------|
| Scholar's Mate | `r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq -` | Qxf7# | 1 |
| Back Rank | `6k1/5ppp/8/8/8/8/8/R3K3 w Q -` | Ra8# | 1 |
| Smothered Mate | `r1b1kb1r/pppp1ppp/2n2n2/4p3/2B1P2q/2N2N2/PPPP1PPP/R1BQK2R b KQkq -` | Qxf2# | 1 |

### 4.2 Tactical Motifs

| Category | Test Count | Description |
|----------|------------|-------------|
| Forks | 10 | Knight/bishop/queen forks |
| Pins | 10 | Absolute and relative pins |
| Skewers | 10 | Attacking through pieces |
| Discovered attacks | 10 | Moving to reveal attack |
| Deflection | 10 | Removing defender |

---

## 5. Performance Benchmarks

### 5.1 Speed Tests

```bash
# Built-in benchmark
prawn.exe bench

# Expected output metrics:
# - Nodes per second (target: >3M nps)
# - Time per position
# - Total nodes searched
```

### 5.2 Performance Regression Tests

| Metric | Baseline (v0.1.0) | Acceptable Range |
|--------|-------------------|------------------|
| NPS (release) | 3.3M | >3.0M |
| Perft 6 time | ~35s | <40s |
| Depth 10 (startpos) | ~500ms | <600ms |
| Memory (64MB hash) | ~70MB | <100MB |

---

## 6. Tournament Testing

### 6.1 Self-Play

```bash
# Using cutechess-cli
cutechess-cli \
  -engine name=Prawn cmd=prawn.exe \
  -engine name=Prawn cmd=prawn.exe \
  -each tc=1+0.1 \
  -games 100 \
  -pgnout selfplay.pgn \
  -recover
```

### 6.2 Against Reference Engines

| Opponent | Expected Score | Time Control | Games |
|----------|----------------|--------------|-------|
| TSCP 181 (~2200) | 50-75% | 2+2 | 100 |
| Crafty (~2400) | 30-50% | 2+2 | 100 |
| Stockfish L1 | 10-30% | 2+2 | 100 |

### 6.3 CCRL/CEGT Testing Protocol

For official rating:
- 40 moves in 4 minutes (40/4)
- Or 40 moves in 15 minutes (40/15)
- Minimum 200 games per opponent
- Use standard opening books

---

## 7. Stress Testing

### 7.1 Long Games
- Play 1000+ move games to test for memory leaks
- Verify repetition detection over many moves

### 7.2 Extreme Time Controls
- Bullet: 1+0 (1 minute, no increment)
- Ultrabullet: 15+0 (15 seconds total)
- Long: 60+0 (60 minutes, no increment)

### 7.3 Hash Collision Testing
- Play many games with small hash (1MB)
- Verify no crashes or illegal moves

---

## 8. Automated Test Commands

### Daily Regression Suite
```bash
# Run all tests
cargo test --release

# Run perft validation
cargo test --release perft

# Run clippy
cargo clippy --release -- -D warnings

# Run benchmark
./target/release/prawn.exe bench
```

### Pre-Release Checklist
```bash
# 1. All unit tests pass
cargo test --release

# 2. Zero clippy warnings  
cargo clippy --release -- -D warnings

# 3. Perft validation (depth 5+)
# Manual verification of node counts

# 4. Tournament test (minimum 20 games)
# Against known opponent

# 5. Performance benchmark
./target/release/prawn.exe bench

# 6. Memory leak check (optional)
# Run long game and monitor memory
```

---

## 9. Bug Reproduction Template

When reporting bugs, include:

```markdown
### Bug Report

**Version**: Prawn v0.1.0 (built YYYY-MM-DD)
**OS**: Windows 11 / Linux / macOS

**Steps to Reproduce**:
1. Start engine
2. Send commands:
   ```
   uci
   position fen [FEN]
   go depth 10
   ```
3. Observe [unexpected behavior]

**Expected**: [what should happen]
**Actual**: [what happened]

**FEN**: [position FEN if applicable]
**PGN**: [game PGN if applicable]
```

---

## 10. Test Results Log

| Date | Version | Unit Tests | Perft | Tournament | Notes |
|------|---------|------------|-------|------------|-------|
| 2025-12-31 | v0.1.0 | ✅ 100% | ✅ Depth 6 | 1.5/2 vs TSCP | First release |

---

## Appendix A: Test Tools

| Tool | Purpose | Link |
|------|---------|------|
| cutechess-cli | Tournament management | https://github.com/cutechess/cutechess |
| Arena | GUI testing | http://www.playwitharena.de/ |
| Stockfish | Reference engine | https://stockfishchess.org/ |
| EPD Test Suites | Tactical testing | chessprogramming.org |

## Appendix B: Reference Perft Values

Source: https://www.chessprogramming.org/Perft_Results

---

*Last Updated: 2025-12-31*
*Document Version: 1.0*
