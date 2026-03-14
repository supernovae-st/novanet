# Rust Cleanup: 22 Audit Findings Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task.

**Goal:** Address all 22 findings from the 3-agent codebase audit (2 CRITICAL, 10 IMPORTANT, 10 MINOR)

**Architecture:** Surgical fixes organized by risk level. Critical security fixes first, then MCP server architecture improvements, then TUI cleanup, then minor polish. Each task is independently committable.

**Tech Stack:** Rust 1.86+, Edition 2024, neo4rs 0.8, rmcp 0.16, moka 0.12

**Scope:** `tools/novanet-mcp/` (MCP server) and `tools/novanet/` (CLI+TUI)

---

## Task 1: CRITICAL — Extract APOC Blocklist Constant (C-1)

**Finding:** The `dangerous_apoc` array is duplicated identically between `validate_read_only()` (line 425) and `validate_write_safe()` (line 500) in `pool.rs`. This is a security DRY violation — if one list is updated but not the other, a bypass becomes possible.

**Files:**
- Modify: `tools/novanet-mcp/src/neo4j/pool.rs:425-522`

**Step 1: Extract the shared constant**

At module level (before the functions), add:

```rust
/// Dangerous APOC procedures blocked in both read and write contexts.
/// SECURITY: This is the single source of truth — update here only.
const DANGEROUS_APOC: &[&str] = &[
    // Dynamic Cypher execution
    "APOC.CYPHER.RUN",
    "APOC.CYPHER.DOIT",
    "APOC.CYPHER.RUNMANY",
    "APOC.CYPHER.PARALLEL",
    // Periodic/scheduled execution
    "APOC.PERIODIC.COMMIT",
    "APOC.PERIODIC.ITERATE",
    "APOC.PERIODIC.SUBMIT",
    "APOC.PERIODIC.REPEAT",
    // File system access
    "APOC.EXPORT",
    "APOC.IMPORT",
    "APOC.LOAD.CSV",
    "APOC.LOAD.JSON",
    "APOC.LOAD.XML",
    // Schema modifications
    "APOC.SCHEMA.ASSERT",
    "APOC.TRIGGER",
    // Database operations
    "APOC.SYSTEMDB",
];
```

**Step 2: Replace both inline arrays**

In `validate_read_only()` (~line 424-456), replace the `let dangerous_apoc = [...]` block and the for loop with:

```rust
// Step 3: Block dangerous APOC procedures
for proc in DANGEROUS_APOC {
    if upper.contains(proc) {
        return Err(Error::invalid_cypher(format!(
            "Dangerous APOC procedure not allowed: {}",
            proc
        )));
    }
}
```

Do the same in `validate_write_safe()` (~line 499-529).

**Step 3: Run tests**

Run: `cargo test --lib -p novanet-mcp -- pool`
Expected: All existing pool tests pass (the behavior is identical)

**Step 4: Commit**

```bash
git add tools/novanet-mcp/src/neo4j/pool.rs
git commit -m "fix(mcp): extract APOC blocklist to single constant (C-1 security DRY)"
```

---

## Task 2: CRITICAL — Remove once_cell, Use std::sync::LazyLock (C-2 + M-1)

**Finding:** `validation.rs` uses `once_cell::sync::Lazy` but the project targets Rust 1.86 (LazyLock stable since 1.80). Also removes the `once_cell` dependency entirely.

**Files:**
- Modify: `tools/novanet-mcp/src/validation.rs:1-18`
- Modify: `tools/novanet-mcp/Cargo.toml:104` (remove once_cell)

**Step 1: Replace once_cell with std::sync::LazyLock in validation.rs**

Change the imports and statics:

```rust
// OLD (line 6):
use once_cell::sync::Lazy;
// NEW:
use std::sync::LazyLock;

// OLD (lines 11-12):
static CLASS_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Z][A-Za-z0-9]*$").expect("Invalid regex"));
// NEW:
static CLASS_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Z][A-Za-z0-9]*$").expect("Invalid regex"));

// OLD (lines 17-18):
static ARC_CLASS_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Z][A-Z0-9_]*$").expect("Invalid regex"));
// NEW:
static ARC_CLASS_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Z][A-Z0-9_]*$").expect("Invalid regex"));
```

**Step 2: Check for other once_cell usages**

Run: `grep -rn "once_cell" tools/novanet-mcp/src/`
If no other usages, proceed to step 3.

**Step 3: Remove once_cell from Cargo.toml**

Delete line 104 from `tools/novanet-mcp/Cargo.toml`:
```toml
once_cell = "1.19"
```

Update the comment block above it (lines 99-105) to just say:
```toml
# ═══════════════════════════════════════════════════════════════════════════════
# Validation
# regex 1.10: Input validation to prevent Cypher injection
# ═══════════════════════════════════════════════════════════════════════════════
regex = "1.10"
```

**Step 4: Run tests**

Run: `cargo test --lib -p novanet-mcp -- validation`
Expected: All 5 validation tests pass

Run: `cargo build -p novanet-mcp`
Expected: Compiles without once_cell

**Step 5: Commit**

```bash
git add tools/novanet-mcp/src/validation.rs tools/novanet-mcp/Cargo.toml
git commit -m "refactor(mcp): replace once_cell with std::sync::LazyLock (M-1)"
```

---

## Task 3: Restrict Module Visibility (I-2)

**Finding:** All 13 modules in `lib.rs` are `pub` but most are internal implementation details. Only `error`, `server` types need to be public.

**Files:**
- Modify: `tools/novanet-mcp/src/lib.rs:27-39`

**Step 1: Change visibility**

```rust
// Only these need to be pub (re-exported below or used externally):
pub mod error;
pub mod server;

// Internal modules — pub(crate) only:
pub(crate) mod activation;
pub(crate) mod cache;
pub(crate) mod hints;
pub(crate) mod metrics;
pub(crate) mod neo4j;
pub(crate) mod prompts;
pub(crate) mod resources;
pub(crate) mod schema_cache;
pub(crate) mod tokens;
pub(crate) mod tools;
pub(crate) mod validation;
```

**Step 2: Build to verify**

Run: `cargo build -p novanet-mcp`
Expected: Compiles. If any external usage errors appear, keep that module `pub`.

**Step 3: Run all tests**

Run: `cargo test -p novanet-mcp`
Expected: All 465+ tests pass (tests are within the crate, so `pub(crate)` is sufficient)

**Step 4: Commit**

```bash
git add tools/novanet-mcp/src/lib.rs
git commit -m "refactor(mcp): restrict module visibility to pub(crate) (I-2)"
```

---

## Task 4: Lock-Free Stats with AtomicU64 (I-4)

**Finding:** `state.rs` uses `RwLock<ServerStats>` for 3 simple u64 counters. Acquiring a write lock for each counter increment is unnecessary overhead — `AtomicU64` provides lock-free increments.

**Files:**
- Modify: `tools/novanet-mcp/src/server/state.rs:19-56` (struct + methods)

**Step 1: Replace RwLock with atomics**

Change imports (line 27):
```rust
// OLD:
use parking_lot::RwLock;
// NEW:
use std::sync::atomic::{AtomicU64, Ordering};
```

Change `StateInner` (line 47):
```rust
// OLD:
pub stats: RwLock<ServerStats>,
// NEW:
pub stats: ServerStats,
```

Change `ServerStats` (lines 50-56):
```rust
/// Server statistics (lock-free atomic counters)
#[derive(Debug, Default)]
pub struct ServerStats {
    pub queries_executed: AtomicU64,
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
}
```

Remove `Clone` derive from `ServerStats` (atomics don't implement Clone).

**Step 2: Update the accessor methods**

Find the `record_query()`, `record_cache_hit()`, `record_cache_miss()` methods and replace write lock pattern with atomic increment. Also update any `get_stats()` or similar reader.

```rust
pub fn record_query(&self) {
    self.inner.stats.queries_executed.fetch_add(1, Ordering::Relaxed);
}

pub fn record_cache_hit(&self) {
    self.inner.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
}

pub fn record_cache_miss(&self) {
    self.inner.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
}
```

For any stats reader (if one returns `ServerStats`), return a snapshot struct:

```rust
pub fn stats_snapshot(&self) -> (u64, u64, u64) {
    (
        self.inner.stats.queries_executed.load(Ordering::Relaxed),
        self.inner.stats.cache_hits.load(Ordering::Relaxed),
        self.inner.stats.cache_misses.load(Ordering::Relaxed),
    )
}
```

**Step 3: Update StateInner initialization**

Where `stats: RwLock::new(ServerStats::default())` is used, change to:
```rust
stats: ServerStats::default(),
```

**Step 4: Check if parking_lot is still needed**

Run: `grep -rn "parking_lot" tools/novanet-mcp/src/ --include="*.rs"`
If no remaining usages, remove `parking_lot` from Cargo.toml. If other files use it, keep it.

**Step 5: Run tests**

Run: `cargo test -p novanet-mcp`
Expected: All tests pass

**Step 6: Commit**

```bash
git add tools/novanet-mcp/src/server/state.rs
# Also add Cargo.toml if parking_lot was removed
git commit -m "perf(mcp): replace RwLock<ServerStats> with AtomicU64 (I-4)"
```

---

## Task 5: DRY Config Env Parsing (I-3)

**Finding:** `config.rs` repeats the same 3-line env var parsing pattern 10+ times. Extract a helper.

**Files:**
- Modify: `tools/novanet-mcp/src/server/config.rs`

**Step 1: Add helper function**

At the top of the file (after imports):

```rust
/// Parse an environment variable with a default value.
/// Pattern: read env var → parse to T → fallback to default.
fn env_or<T: std::str::FromStr>(var: &str, default: T) -> T {
    std::env::var(var)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}
```

**Step 2: Replace all repetitive parsing**

Replace each occurrence of the 3-line pattern. Example:

```rust
// OLD:
pool_size: std::env::var("NOVANET_MCP_NEO4J_POOL_SIZE")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(16),

// NEW:
pool_size: env_or("NOVANET_MCP_NEO4J_POOL_SIZE", 16),
```

Apply to all ~10 numeric env vars: pool_size, fetch_size, max_retries, retry_base_delay (inside Duration::from_millis), circuit_breaker_threshold, circuit_breaker_reset_timeout (inside Duration::from_secs), cache_max_entries, cache_ttl (inside Duration::from_secs), default_token_budget, max_hops, evidence_packet_size.

For Duration wrappers, use the helper inside the Duration constructor:

```rust
retry_base_delay: Duration::from_millis(env_or("NOVANET_MCP_RETRY_BASE_DELAY_MS", 100)),
```

**Step 3: Eliminate Default impl duplication**

The `Default` impl (lines 134-159) duplicates all default values. Use `from_env()` pattern or define defaults as constants:

```rust
impl Default for Config {
    fn default() -> Self {
        // Set the password env var for testing contexts
        Self {
            neo4j_uri: "bolt://localhost:7687".to_string(),
            neo4j_user: "neo4j".to_string(),
            neo4j_password: "novanetpassword".to_string(),
            pool_size: 16,
            fetch_size: 500,
            max_retries: 3,
            retry_base_delay: Duration::from_millis(100),
            circuit_breaker_threshold: 5,
            circuit_breaker_reset_timeout: Duration::from_secs(30),
            cache_max_entries: 10000,
            cache_ttl: Duration::from_secs(300),
            default_token_budget: 100_000,
            max_hops: 5,
            evidence_packet_size: 200,
            spreading_config_path: None,
        }
    }
}
```

Keep the Default impl as-is (it's already clean), but the `from_env()` now uses `env_or()` to avoid repetition.

**Step 4: Run tests**

Run: `cargo test -p novanet-mcp -- config`
Expected: `test_default_config` passes

**Step 5: Commit**

```bash
git add tools/novanet-mcp/src/server/config.rs
git commit -m "refactor(mcp): extract env_or helper for config parsing (I-3)"
```

---

## Task 6: Split pool.rs — Extract Cypher Validation (I-5)

**Finding:** `pool.rs` is 1,014 lines with mixed responsibilities: connection pool, query execution, Cypher validation, comment stripping. Extract validation to its own module.

**Files:**
- Create: `tools/novanet-mcp/src/neo4j/cypher_guard.rs`
- Modify: `tools/novanet-mcp/src/neo4j/pool.rs`
- Modify: `tools/novanet-mcp/src/neo4j/mod.rs`

**Step 1: Create `cypher_guard.rs`**

Move these functions from `pool.rs` to the new file:
- `DANGEROUS_APOC` constant (from Task 1)
- `WRITE_KEYWORDS` constant
- `validate_read_only()`
- `validate_write_safe()`
- `strip_cypher_comments()`
- `contains_keyword_with_boundary()`
- `starts_with_keyword()`
- `contains_foreach_write()`
- All related tests (the `#[cfg(test)]` block for validation)

The new file should have:
```rust
//! Cypher query validation and sanitization
//!
//! Security-critical module that validates Cypher queries before execution.
//! Prevents injection attacks, blocks dangerous APOC procedures, and enforces
//! read-only or write-safe constraints.

use crate::error::{Error, Result};

// ... moved functions and constants ...
```

**Step 2: Update pool.rs imports**

In `pool.rs`, replace the direct function calls with:
```rust
use super::cypher_guard::{validate_read_only, validate_write_safe};
```

Remove the moved functions and constants from pool.rs.

**Step 3: Update neo4j/mod.rs**

Add the new module:
```rust
pub(crate) mod cypher_guard;
```

**Step 4: Run tests**

Run: `cargo test -p novanet-mcp -- cypher_guard`
Run: `cargo test -p novanet-mcp -- pool`
Expected: All tests pass from their new locations

**Step 5: Commit**

```bash
git add tools/novanet-mcp/src/neo4j/cypher_guard.rs tools/novanet-mcp/src/neo4j/pool.rs tools/novanet-mcp/src/neo4j/mod.rs
git commit -m "refactor(mcp): extract Cypher validation to cypher_guard module (I-5)"
```

---

## Task 7: Reduce Handler Boilerplate (I-6)

**Finding:** `handler.rs` has 8 tool methods with identical 5-line bodies: execute → serialize → wrap. Extract a helper.

**Files:**
- Modify: `tools/novanet-mcp/src/server/handler.rs`

**Step 1: Add helper function**

Add at the bottom of the file (outside the impl blocks):

```rust
/// Execute a tool and serialize the result to a CallToolResult.
/// Shared by all 8 tool handlers.
async fn execute_and_serialize<F, T, Fut>(f: F) -> Result<CallToolResult, McpError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = crate::Result<T>>,
    T: serde::Serialize,
{
    let result = f().await.map_err(McpError::from)?;
    let json = serde_json::to_string_pretty(&result)
        .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}
```

**Step 2: Simplify each tool method**

Replace each 5-line body with a one-liner. Example:

```rust
async fn novanet_query(
    &self,
    params: Parameters<QueryParams>,
) -> Result<CallToolResult, McpError> {
    execute_and_serialize(|| crate::tools::query::execute(&self.state, params.0)).await
}
```

Do this for all 8 tools: query, describe, search, introspect, context, write, audit, batch.

**Step 3: Verify it compiles**

Run: `cargo build -p novanet-mcp`
Expected: Compiles. The `#[tool(...)]` macro should work fine since the function signature is unchanged.

**Step 4: Run tests**

Run: `cargo test -p novanet-mcp`
Expected: All tests pass

**Step 5: Commit**

```bash
git add tools/novanet-mcp/src/server/handler.rs
git commit -m "refactor(mcp): extract execute_and_serialize handler helper (I-6)"
```

---

## Task 8: TUI Dead Trait Code Cleanup (I-9 + M-10)

**Finding:** `icons.rs` still has `trait_icons()` and `trait_terminal_icon()` functions. `theme.rs` still has the entire `traits` module with border chars and colors. Traits were deprecated in v0.19.0 (ADR-024). Also `icons.rs` header comment references outdated information.

**Files:**
- Modify: `tools/novanet/src/tui/icons.rs`
- Modify: `tools/novanet/src/tui/theme.rs`

**Step 1: Check for callers**

Run: `grep -rn "trait_icons\|trait_terminal_icon\|traits::border_char\|traits::color" tools/novanet/src/`

If no callers exist outside the modules themselves, proceed to delete.

**Step 2: Remove dead trait functions from icons.rs**

Remove:
- `trait_icons()` function (lines ~146-155)
- `trait_terminal_icon()` function (lines ~249-258)
- Any unused `TRAITS_*` constants that were only referenced by these functions

Fix the module header comment to reflect current state (remove trait references).

**Step 3: Remove dead trait module from theme.rs**

Remove the entire `pub mod traits` block (lines ~255-330+) including:
- Border character constants
- Hex color constants
- `border_char()` function
- `color()` function

Remove the header comment line 9 referencing `traits/*.yaml` since traits were deleted.

**Step 4: Run tests**

Run: `cargo test -p novanet`
Expected: All 1,047+ tests pass (if any test references trait functions, fix the test too — it's testing dead code)

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/icons.rs tools/novanet/src/tui/theme.rs
git commit -m "refactor(tui): remove dead trait icons and colors (I-9, ADR-024 deprecated)"
```

---

## Task 9: Minor MCP Fixes Batch (M-2, M-5, M-6, M-7)

**Finding:** Four small, independent issues that can be fixed quickly.

**Files:**
- Check: `tools/novanet-mcp/src/tools/query.rs` (M-2: unnecessary clone)
- Modify: `tools/novanet-mcp/CLAUDE.md` (M-5: dashmap reference)
- Check: `tools/novanet-mcp/src/tokens/` (M-6: estimate_tokens allocation)
- Check: `tools/novanet-mcp/Cargo.toml` (M-7: duplicate release profile)

**Step 1: M-5 — Fix CLAUDE.md dashmap reference**

In `tools/novanet-mcp/CLAUDE.md`, the Key Dependencies table lists `dashmap` but it may no longer be in Cargo.toml. Check:

Run: `grep "dashmap" tools/novanet-mcp/Cargo.toml`

If dashmap is NOT in Cargo.toml, remove the dashmap row from the dependencies table in CLAUDE.md. Also update the tokio version in the table to match Cargo.toml (1.50, not 1.43).

**Step 2: M-2 — Check for unnecessary clone in query.rs**

Run: `grep -n "\.clone()" tools/novanet-mcp/src/tools/query.rs`

If there's an unnecessary `.clone()` that could be removed (e.g., cloning a value that's only used once), remove it.

**Step 3: M-7 — Check for duplicate release profile**

Run: `grep -c "\[profile.release\]" tools/novanet-mcp/Cargo.toml`

If count > 1, remove the duplicate. Current file shows only one at line 116, so this may already be clean.

**Step 4: M-6 — Check estimate_tokens for unnecessary allocations**

Run: `grep -n "estimate_tokens\|estimate" tools/novanet-mcp/src/tokens/`

Look for any `String::new()` or `format!()` that could be avoided in the fast estimation path.

**Step 5: Run tests**

Run: `cargo test -p novanet-mcp`
Expected: All tests pass

**Step 6: Commit**

```bash
git add tools/novanet-mcp/CLAUDE.md
# Add any other changed files
git commit -m "docs(mcp): fix outdated CLAUDE.md references (M-5)"
```

If code changes were made:
```bash
git commit -m "refactor(mcp): minor cleanup — unnecessary clones and allocations (M-2, M-6)"
```

---

## Task 10: Update CLAUDE.md Tokio Version (Docs Accuracy)

**Finding:** CLAUDE.md Key Dependencies table shows `tokio 1.43` but Cargo.toml has `1.50`.

**Files:**
- Modify: `tools/novanet-mcp/CLAUDE.md`

**Step 1: Update the dependency table**

Find the Key Dependencies table and update:
- `tokio` version: `1.43` → `1.50`
- Remove `dashmap` row if the crate is not in Cargo.toml
- Verify all other versions match Cargo.toml

**Step 2: Commit**

```bash
git add tools/novanet-mcp/CLAUDE.md
git commit -m "docs(mcp): update dependency versions in CLAUDE.md"
```

---

## Deferred Items (Not In Scope)

The following findings are **intentionally deferred** with rationale:

| Finding | Rationale |
|---------|-----------|
| **I-1** Cross-crate Neo4j duplication | CLI `db.rs` (200 lines) and MCP `pool.rs` are in separate crates with different concerns. CLI does seed/reset, MCP does runtime queries. Not true duplication. |
| **I-7** Split monolithic test files | Test files co-located with their modules is Rust convention. Splitting only adds complexity. |
| **I-8** Cross-crate error duplication | Separate crates with independent error types is intentional. Shared error crate adds coupling. |
| **I-10** Split data.rs monolith | data.rs (4,128 lines) is the TUI's unified tree data model. Splitting would fragment the tree logic across modules with heavy cross-references. Revisit when adding new TUI features. |
| **M-3** Split search.rs | 1,127 lines is large but the modes (fulltext, property, hybrid, walk, triggers) share types and helpers. Splitting would create circular dependencies. |
| **M-4** Hash library divergence | ahash is used intentionally for cache keys (performance). Not a consistency issue. |
| **M-8** CLI row collection pattern | Minor optimization, low value for the effort. |
| **M-9** Reserved dead code structs | May be intentional placeholders. Verify before removing. |

---

## Execution Order

```
Task 1: CRITICAL — APOC blocklist constant (pool.rs)          [5 min]
Task 2: CRITICAL — LazyLock migration (validation.rs)          [5 min]
Task 3: Module visibility restriction (lib.rs)                 [5 min]
Task 4: AtomicU64 stats (state.rs)                            [10 min]
Task 5: DRY config parsing (config.rs)                        [10 min]
Task 6: Split pool.rs (cypher_guard.rs extraction)            [15 min]
Task 7: Handler boilerplate reduction (handler.rs)            [10 min]
Task 8: TUI dead trait cleanup (icons.rs + theme.rs)          [10 min]
Task 9: Minor MCP fixes batch (M-2, M-5, M-6, M-7)          [10 min]
Task 10: CLAUDE.md accuracy (docs)                            [5 min]
```

**Total: 10 tasks, ~85 min estimated**

**Verification after all tasks:**

```bash
# MCP server
cargo test -p novanet-mcp           # All 465+ tests pass
cargo clippy -p novanet-mcp -- -D warnings  # Zero warnings

# CLI + TUI
cargo test -p novanet               # All 1,047+ tests pass
cargo clippy -p novanet -- -D warnings      # Zero warnings
```
