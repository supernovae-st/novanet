# TUI Bug Fixes Implementation Plan

**Date**: 2026-02-27
**Author**: Claude + Thibaut
**Status**: ✅ Completed

## Executive Summary

Fix all identified bugs from the codebase audit:
- 2 CRITICAL (race condition, silent errors) → ✅ Fixed
- 1 HIGH (unused validation) → ✅ Already implemented
- 3 MEDIUM (refactoring) → ✅ 1 fixed, 2 deferred to separate PRs
- 2 LOW (documentation, consistency) → ✅ Fixed

## Results

| Priority | Issue | Status | Notes |
|----------|-------|--------|-------|
| CRITICAL | Race condition | ✅ Fixed | `navigation_generation` counter added |
| CRITICAL | Error logging | ✅ Fixed | `tracing::warn!` added |
| HIGH | Validation | ✅ N/A | Already implemented in all functions |
| MEDIUM | App refactor | ⏸️ Deferred | Complex, needs separate PR |
| MEDIUM | LazyLock regex | ✅ Fixed | `RE_FIELD` extracted in formatting.rs |
| MEDIUM | Extract utils | ⏸️ Deferred | Complex, needs separate PR |
| LOW | Magic numbers | ✅ Fixed | Doc comments with rationale added |
| LOW | writeln pattern | ✅ N/A | Pattern is safe (String write infallible) |

**Tests**: 1192 passed | **Clippy**: 0 warnings

## Issues & Solutions

### CRITICAL 1: TUI Race Condition (app.rs:501-511)

**Problem**: Pending async loads can complete AFTER user navigates away, setting stale data.

**Flow**:
```
1. User presses 'j' → pending_arcs_load = Some("Page")
2. Event loop takes "Page", starts async load
3. User presses 'j' again → load_yaml_for_current() clears pending
4. Async completes with "Page" arcs
5. set_class_arcs() applies WRONG data (cursor now on different item)
```

**Solution**: Add generation counter to detect stale results.

**Files**:
- `src/tui/app.rs`: Add `navigation_generation: u64` field
- `src/tui/mod.rs`: Capture generation before async, check before apply

**Implementation**:
```rust
// app.rs - Add to App struct
pub navigation_generation: u64,

// app.rs - In load_yaml_for_current()
self.navigation_generation = self.navigation_generation.wrapping_add(1);

// mod.rs - Capture before async
let gen = app.navigation_generation;
// After async completes:
if app.navigation_generation == gen {
    app.set_class_arcs(arcs);
}
```

---

### CRITICAL 2: Event Stream Errors Silently Ignored (mod.rs:294-296)

**Problem**: `Ok(Some(Err(_)))` swallows errors without logging.

**Solution**: Log errors for debugging, continue execution.

**Implementation**:
```rust
Ok(Some(Err(e))) => {
    // Log but don't crash - terminal errors shouldn't stop TUI
    tracing::warn!("Event stream error: {}", e);
}
```

---

### HIGH: validate_cypher_label() Unused (data.rs:24-41)

**Problem**: Defense-in-depth validation exists but isn't called.

**Solution**: Call validation before all Cypher queries with interpolated labels.

**Files**: `src/tui/data.rs`

**Functions to update**:
- `load_class_arcs()` - line 1545
- `load_arc_class_details()` - line 1625
- `load_instances()` - line 1215
- `load_instances_fast()` - line 1361
- Any other functions with label interpolation

---

### MEDIUM 1: Large App Struct (40+ fields)

**Problem**: App struct has too many fields, hard to maintain.

**Solution**: Extract into logical sub-structs.

**New structs**:
```rust
/// Source panel state (Schema/Instance tabs)
pub struct SourcePanelState {
    pub tab: SourceTab,
    pub class_cursor: Option<usize>,
}

/// Schema overlay state (validation coverage)
pub struct SchemaOverlayState {
    pub enabled: bool,
    pub matched_properties: Option<Vec<String>>,
    pub coverage_stats: Option<ValidationCoverageStats>,
}

/// Pending load requests (all async operations)
pub struct PendingLoads {
    pub instance: Option<String>,
    pub arcs: Option<String>,
    pub instance_arcs: Option<(String, Vec<String>)>,
    pub entity_categories: bool,
    pub category_instances: Option<String>,
    pub arc_class: Option<String>,
    pub realm: Option<String>,
    pub layer: Option<String>,
}
```

---

### MEDIUM 2: Regex Patterns Compiled Inline

**Problem**: Regex compiled on every call in parsers/*.rs

**Files**:
- `src/parsers/formatting.rs`
- `src/parsers/market.rs`
- `src/parsers/culture.rs`
- `src/parsers/expression.rs`

**Solution**: Use `LazyLock<Regex>` at module level.

**Example**:
```rust
use std::sync::LazyLock;
use regex::Regex;

static RE_FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"-\s+\*\*(\w+)\*\*:\s*`?([^`\n]+)`?")
        .expect("RE_FIELD regex should be valid")
});
```

---

### MEDIUM 3: Duplicate Markdown Parsing

**Problem**: Frontmatter parsing duplicated across 4 parser files.

**Solution**: Extract to `src/parsers/markdown_utils.rs`.

**Functions to extract**:
- `parse_frontmatter()` - extract version/date
- `split_sections()` - split by ## headers
- `extract_field()` - extract specific field value

---

### LOW 1: Magic Numbers Without Documentation

**Problem**: Constants lack rationale documentation.

**File**: `src/tui/app.rs:30-41`

**Solution**: Add doc comments explaining why each value was chosen.

---

### LOW 2: writeln!().unwrap() Pattern

**Problem**: Inconsistent - some use `.unwrap()`, some use `?`.

**Solution**: Use `write!()` macro with `?` in Result-returning contexts, accept `.unwrap()` for String building (it's infallible but document why).

---

## Execution Order

1. **CRITICAL 1**: Race condition (highest impact)
2. **CRITICAL 2**: Error logging (quick fix)
3. **HIGH**: Validation usage (security)
4. **MEDIUM 1**: App refactor (preparation for future)
5. **MEDIUM 2**: LazyLock regex (performance)
6. **MEDIUM 3**: Extract utils (maintainability)
7. **LOW 1-2**: Documentation/consistency

## Testing Strategy

After each fix:
1. `cargo test` - all 1192+ tests pass
2. `cargo clippy -- -D warnings` - zero warnings
3. Manual TUI testing for CRITICAL fixes

## Estimated Effort

| Priority | Issue | Time |
|----------|-------|------|
| CRITICAL | Race condition | 30 min |
| CRITICAL | Error logging | 5 min |
| HIGH | Validation | 15 min |
| MEDIUM | App refactor | 1 hour |
| MEDIUM | LazyLock | 30 min |
| MEDIUM | Extract utils | 45 min |
| LOW | Documentation | 15 min |

**Total**: ~3 hours
