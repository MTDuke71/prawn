# Mission 8: Official v0.1 Release

## Overview
Create the first official release of Prawn chess engine with proper versioning, build metadata, and release artifacts.

## Requirements

### REQ-1: Version Information in UCI
**Description**: Display version and build timestamp in UCI `id` response

**Acceptance Criteria**:
- [x] `id name Prawn v0.1.0` with build timestamp
- [x] Build timestamp embedded at compile time
- [x] `uci` command shows version info

**Implementation**: Use `env!()` and build script for compile-time metadata

### REQ-2: Build Timestamp
**Description**: Embed compile-time timestamp in binary

**Acceptance Criteria**:
- [x] Timestamp generated during `cargo build`
- [x] Format: `YYYY-MM-DD HH:MM:SS UTC`
- [x] Accessible via constant or function

**Implementation**: `build.rs` script with `BUILT_TIMESTAMP` env var

### REQ-3: Version Constants
**Description**: Centralized version information

**Acceptance Criteria**:
- [x] `VERSION` constant: "0.1.0"
- [x] `ENGINE_NAME` constant: "Prawn"
- [x] `ENGINE_AUTHOR` constant

### REQ-4: Release Build Configuration
**Description**: Optimized release profile

**Acceptance Criteria**:
- [x] LTO (Link Time Optimization) enabled
- [x] Single codegen unit for maximum optimization
- [x] Native CPU target for best performance

---

## Traceability Matrix

| Requirement | Implementation | Tests | Status |
|-------------|----------------|-------|--------|
| REQ-1: UCI Version | `uci.rs` id response | Manual UCI test | ✅ |
| REQ-2: Build Timestamp | `build.rs` | Compile verification | ✅ |
| REQ-3: Version Constants | `lib.rs` constants | N/A | ✅ |
| REQ-4: Release Config | `Cargo.toml` profile | Build verification | ✅ |

---

## Architecture Notes

### Build Script (build.rs)
```rust
// Generates BUILT_TIMESTAMP at compile time
fn main() {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    println!("cargo:rustc-env=BUILT_TIMESTAMP={}", timestamp);
}
```

### UCI Output Format
```
id name Prawn v0.1.0 (built 2025-12-31 22:00:00 UTC)
id author [Author Name]
```

---

## Release Checklist

- [x] All tests pass (`cargo test --release`)
- [x] Zero clippy warnings
- [x] Version constants updated
- [x] Build timestamp working
- [x] Tournament tested (1.5/2 vs TSCP 181)
- [ ] Git tag created: `v0.1.0`
- [ ] Release notes written
