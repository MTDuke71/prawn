# Mission 2 - Perft Benchmark Results

## Overview
This file contains performance benchmark results for the Mission 2 move generator using the perft (performance test) algorithm.

## Running the Benchmark

To run the benchmark yourself in **release mode** (optimized):

```bash
cd missions/mission2
cargo run --release --example perft_benchmark
```

Or build and run directly:

```bash
cargo build --release --example perft_benchmark
./target/release/examples/perft_benchmark
```

⚠️ **Important**: Always run benchmarks in release mode (`--release` flag). Debug mode is 30-40x slower!

## Latest Results

**Date**: December 17, 2025  
**Build Mode**: Release (optimized)

### Starting Position
Position: `rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1`

| Depth | Nodes | Time | Nodes/sec | Status |
|-------|-------|------|-----------|--------|
| 1 | 20 | 0.00s | ~5.3M | ✓ PASS |
| 2 | 400 | 0.00s | ~21.7M | ✓ PASS |
| 3 | 8,902 | 0.00s | ~23.0M | ✓ PASS |
| 4 | 197,281 | 0.01s | ~27.4M | ✓ PASS |
| 5 | 4,865,609 | 0.17s | ~29.3M | ✓ PASS |
| 6 | 119,060,324 | 4.35s | ~27.4M | ✓ PASS |

### Kiwipete Position
Position: `r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1`

| Depth | Nodes | Time | Nodes/sec | Status |
|-------|-------|------|-----------|--------|
| 1 | 48 | 0.00s | ~8.3M | ✓ PASS |
| 2 | 2,039 | 0.00s | ~24.1M | ✓ PASS |
| 3 | 97,862 | 0.00s | ~26.9M | ✓ PASS |
| 4 | 4,085,603 | 0.15s | ~27.2M | ✓ PASS |
| 5 | 193,690,690 | 6.80s | ~28.5M | ✓ PASS |

## Performance Comparison

### Debug vs Release Mode

| Test | Debug Time | Release Time | Speedup |
|------|-----------|--------------|---------|
| Starting Depth 6 | 136.73s | 4.35s | **31.4x faster** |
| Kiwipete Depth 5 | 182.08s | 6.80s | **26.8x faster** |

**Average throughput**: ~27.5 million nodes/second in release mode

## Validation

All perft tests **PASS** with expected node counts, confirming:
- ✅ Correct move generation for all piece types
- ✅ En passant captures handled correctly
- ✅ Castling rights updated properly
- ✅ Check/checkmate detection accurate
- ✅ Pinned piece handling correct
- ✅ Promotions working as expected

## Logs

Results are automatically appended to `perft_benchmark.log` with timestamps for each run.
