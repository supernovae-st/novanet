# v9.8 Polish Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add tracing instrumentation and TUI caching for production observability and performance.

**Architecture:** Minimal changes — add `#[instrument]` macros to key functions, add YAML content cache to TUI.

**Tech Stack:** Rust (tracing, ratatui)

---

## Summary

| Batch | Tasks | Est. Time | Priority |
|-------|-------|-----------|----------|
| 1 | Add tracing instrumentation | 30min | P2 |
| 2 | Add TUI YAML caching | 30min | P2 |
| 3 | Verify & commit | 10min | P1 |

**Total**: ~70min

---

## Batch 1: Tracing Instrumentation

### Task 1.1: Add #[instrument] to db.rs functions

**File:** `tools/novanet/src/commands/db.rs`

The file already imports tracing. Add `#[instrument]` to key functions.

**Step 1: Read current file**

Check which functions exist and which already have instrumentation.

**Step 2: Add #[instrument] to run_seed**

```rust
#[instrument(skip(db), fields(root = %root.display()))]
pub async fn run_seed(db: &Db, root: &Path) -> crate::Result<()> {
```

**Step 3: Add #[instrument] to run_reset**

```rust
#[instrument(skip(db), fields(root = %root.display()))]
pub async fn run_reset(db: &Db, root: &Path) -> crate::Result<()> {
```

**Step 4: Verify compilation**

```bash
cd tools/novanet && cargo check
```

---

### Task 1.2: Add #[instrument] to schema.rs functions

**File:** `tools/novanet/src/commands/schema.rs`

**Step 1: Add tracing import if not present**

```rust
use tracing::{info, instrument};
```

**Step 2: Add #[instrument] to run_generate**

```rust
#[instrument(skip_all, fields(root = %root.display(), dry_run))]
pub fn run_generate(root: &Path, dry_run: bool) -> crate::Result<()> {
```

**Step 3: Add #[instrument] to run_validate**

```rust
#[instrument(skip_all, fields(root = %root.display(), strict))]
pub fn run_validate(root: &Path, strict: bool) -> crate::Result<()> {
```

**Step 4: Verify compilation**

```bash
cd tools/novanet && cargo check
```

---

### Task 1.3: Add #[instrument] to read.rs functions

**File:** `tools/novanet/src/commands/read.rs`

**Step 1: Add tracing import**

```rust
use tracing::instrument;
```

**Step 2: Add #[instrument] to run_data**

```rust
#[instrument(skip(db))]
pub async fn run_data(db: &Db, format: OutputFormat) -> crate::Result<()> {
```

**Step 3: Add #[instrument] to run_meta, run_overlay, run_query**

Similar pattern for each function.

**Step 4: Verify compilation**

```bash
cd tools/novanet && cargo check
```

---

### Task 1.4: Add #[instrument] to search.rs

**File:** `tools/novanet/src/commands/search.rs`

**Step 1: Add tracing import**

```rust
use tracing::instrument;
```

**Step 2: Add #[instrument] to run_search**

```rust
#[instrument(skip(db), fields(query = %query, limit))]
pub async fn run_search(
    db: &Db,
    query: &str,
    kind: Option<&str>,
    limit: i64,
    format: OutputFormat,
) -> crate::Result<()> {
```

**Step 3: Verify compilation**

```bash
cd tools/novanet && cargo check
```

---

## Batch 2: TUI YAML Caching

### Task 2.1: Add yaml_cache to App struct

**File:** `tools/novanet/src/tui/app.rs`

**Step 1: Add HashMap import**

```rust
use std::collections::HashMap;
```

**Step 2: Add yaml_cache field to App struct**

Find the App struct and add:

```rust
pub struct App {
    // ... existing fields ...

    /// Cache of YAML file contents (path -> content).
    /// Avoids re-reading files on every scroll/navigation.
    pub yaml_cache: HashMap<String, String>,
}
```

**Step 3: Initialize yaml_cache in App::new()**

```rust
yaml_cache: HashMap::new(),
```

---

### Task 2.2: Update YAML loading to use cache

**File:** `tools/novanet/src/tui/app.rs`

**Step 1: Find the YAML loading code**

Look for where `yaml_content` is set from file reads.

**Step 2: Add cache check**

```rust
pub fn load_yaml_for_selection(&mut self) {
    let path = self.current_yaml_path();

    // Check cache first
    if let Some(cached) = self.yaml_cache.get(&path) {
        if self.yaml_content != *cached {
            self.yaml_content = cached.clone();
            self.yaml_line_count = self.yaml_content.lines().count();
        }
        return;
    }

    // Load from disk
    let full_path = self.root.join(&path);
    let content = std::fs::read_to_string(&full_path).unwrap_or_default();

    // Update cache
    self.yaml_cache.insert(path.clone(), content.clone());

    // Update display
    self.yaml_content = content;
    self.yaml_line_count = self.yaml_content.lines().count();
}
```

**Step 3: Verify compilation**

```bash
cd tools/novanet && cargo check
```

---

### Task 2.3: Add cache invalidation (optional)

**File:** `tools/novanet/src/tui/app.rs`

**Step 1: Add clear_cache method**

```rust
/// Clear YAML cache (useful after external file modifications).
pub fn clear_yaml_cache(&mut self) {
    self.yaml_cache.clear();
}
```

**Step 2: Verify compilation**

```bash
cd tools/novanet && cargo check
```

---

## Batch 3: Verify & Commit

### Task 3.1: Run full test suite

**Step 1: Rust tests**

```bash
cd tools/novanet && cargo test --quiet
```

Expected: All tests pass.

**Step 2: Clippy**

```bash
cd tools/novanet && cargo clippy -- -D warnings
```

Expected: Zero warnings.

---

### Task 3.2: Test tracing output

**Step 1: Run with RUST_LOG**

```bash
cd /Users/thibaut/supernovae-st/novanet-hq && RUST_LOG=debug cargo run --manifest-path tools/novanet/Cargo.toml -- schema validate 2>&1 | head -20
```

Expected: Debug output with function spans visible.

---

### Task 3.3: Commit changes

**Step 1: Stage and commit**

```bash
git add -A && git commit -m "$(cat <<'EOF'
perf(rust): add tracing instrumentation + TUI YAML caching

- Add #[instrument] to db, schema, read, search commands
- Add yaml_cache HashMap to TUI App for content caching
- Reduces file I/O on TUI navigation

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>
EOF
)"
```

---

## Verification Checklist

After completion:

- [ ] `cargo test` passes
- [ ] `cargo clippy -- -D warnings` has zero warnings
- [ ] `RUST_LOG=debug cargo run -- schema validate` shows function spans
- [ ] TUI navigation doesn't re-read YAML files unnecessarily

---

## Notes

- tracing is already a dependency in Cargo.toml
- HashMap is from std, no new dependencies needed
- Cache uses path as key, content as value
- Cache is never persisted (in-memory only, cleared on exit)
