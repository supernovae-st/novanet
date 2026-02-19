# TUI Audit Cleanup Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Clean up NovaNet Rust TUI based on 10-agent audit findings - remove dead code, fix performance issues, modernize patterns, and improve test coverage.

**Architecture:** Four-phase approach: (1) Quick wins - dead code removal and duplication fix, (2) Performance - lazy regex, Vec capacity, parallel async, (3) Modernization - tracing migration and CLI extraction, (4) Testing - integration tests for untested modules.

**Tech Stack:** Rust 1.86+, tokio, neo4rs, ratatui, std::sync::LazyLock

---

## Phase 1: Quick Wins (1 hour)

### Task 1.1: Remove Dead Function `clear_yaml_cache`

**Files:**
- Modify: `src/tui/app.rs:391-394`

**Step 1: Verify function is unused**

Run:
```bash
grep -rn "clear_yaml_cache" src/
```
Expected: Only definition at app.rs:391, no calls

**Step 2: Remove the function**

Delete lines 390-394 in `src/tui/app.rs`:
```rust
// DELETE THIS:
    /// Clear YAML cache (useful after external file modifications).
    #[allow(dead_code)]
    pub fn clear_yaml_cache(&mut self) {
        self.yaml_cache.clear();
    }
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 4: Commit**

```bash
git add src/tui/app.rs
git commit -m "refactor(tui): remove unused clear_yaml_cache function"
```

---

### Task 1.2: Remove Dead Function `set_atlas_layer_counts`

**Files:**
- Modify: `src/tui/app.rs:1222` (approximate line)

**Step 1: Verify function is unused**

Run:
```bash
grep -rn "set_atlas_layer_counts" src/
```
Expected: Only definition, no calls

**Step 2: Remove the function**

Find and delete the `set_atlas_layer_counts` function and its `#[allow(dead_code)]` marker.

**Step 3: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 4: Commit**

```bash
git add src/tui/app.rs
git commit -m "refactor(tui): remove unused set_atlas_layer_counts function"
```

---

### Task 1.3: Remove Dead Function `clear_instances`

**Files:**
- Modify: `src/tui/data.rs:1501` (approximate line)

**Step 1: Verify function is unused**

Run:
```bash
grep -rn "clear_instances" src/
```
Expected: Only definition, no calls

**Step 2: Remove the function**

Find and delete the `clear_instances` function and its `#[allow(dead_code)]` marker.

**Step 3: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 4: Commit**

```bash
git add src/tui/data.rs
git commit -m "refactor(tui): remove unused clear_instances function"
```

---

### Task 1.4: Remove Dead Function `load_trait_details` and Related Structs

**Files:**
- Modify: `src/tui/data.rs:296-305` (TraitKindInfo, TraitDetails structs)
- Modify: `src/tui/data.rs:1019` (load_trait_details function)

**Step 1: Verify items are unused**

Run:
```bash
grep -rn "load_trait_details\|TraitKindInfo\|TraitDetails" src/
```
Expected: Only definitions, no usages

**Step 2: Remove structs TraitKindInfo and TraitDetails**

Find and delete around lines 296-315:
```rust
// DELETE THIS:
#[allow(dead_code)]
pub struct TraitKindInfo {
    // ...
}

#[allow(dead_code)]
pub struct TraitDetails {
    // ...
}
```

**Step 3: Remove load_trait_details function**

Find and delete the async function `load_trait_details` (around line 1019).

**Step 4: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 5: Commit**

```bash
git add src/tui/data.rs
git commit -m "refactor(tui): remove unused TraitKindInfo, TraitDetails, load_trait_details"
```

---

### Task 1.5: Remove Dead Function `render_instance_arcs`

**Files:**
- Modify: `src/tui/ui.rs:1669` (approximate line)

**Step 1: Verify function is unused**

Run:
```bash
grep -rn "render_instance_arcs" src/
```
Expected: Only definition, no calls

**Step 2: Remove the function**

Find and delete the `render_instance_arcs` function and its `#[allow(dead_code)]` marker.

**Step 3: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 4: Commit**

```bash
git add src/tui/ui.rs
git commit -m "refactor(tui): remove unused render_instance_arcs function"
```

---

### Task 1.6: Remove False Positive Dead Code Marker

**Files:**
- Modify: `src/retrieval/meta.rs:42`

**Step 1: Verify field IS used**

Run:
```bash
grep -n "self.graph" src/retrieval/meta.rs
```
Expected: Multiple usages (lines 79, 100, 126)

**Step 2: Remove the #[allow(dead_code)] marker**

Change from:
```rust
#[allow(dead_code)]
graph: Arc<Graph>,
```

To:
```rust
graph: Arc<Graph>,
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: No errors, no warnings about dead_code

**Step 4: Commit**

```bash
git add src/retrieval/meta.rs
git commit -m "refactor(retrieval): remove incorrect dead_code marker from graph field"
```

---

### Task 1.7: Consolidate Duplicate `escape_cypher` Function

**Files:**
- Modify: `src/generators/cypher_utils.rs` (add function)
- Modify: `src/generators/slugification.rs:294-296` (remove, use import)
- Modify: `src/generators/formatting.rs:208-210` (remove, use import)

**Step 1: Add escape_cypher to cypher_utils.rs**

Add to `src/generators/cypher_utils.rs`:
```rust
/// Escape single quotes and newlines for Cypher string literals.
pub fn escape_cypher(s: &str) -> String {
    s.replace('\'', "\\'").replace('\n', "\\n")
}
```

**Step 2: Update slugification.rs to use shared function**

In `src/generators/slugification.rs`:
- Add import: `use super::cypher_utils::escape_cypher;`
- Remove local `escape_cypher` function (lines 294-296)

**Step 3: Update formatting.rs to use shared function**

In `src/generators/formatting.rs`:
- Add import: `use super::cypher_utils::escape_cypher;`
- Remove local `escape_cypher` function (around line 208-210)

**Step 4: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 5: Run tests**

Run: `cargo test`
Expected: All 246 tests pass

**Step 6: Commit**

```bash
git add src/generators/cypher_utils.rs src/generators/slugification.rs src/generators/formatting.rs
git commit -m "refactor(generators): consolidate duplicate escape_cypher into cypher_utils"
```

---

### Task 1.8: Fix Rustdoc Warnings - Escape Brackets

**Files:**
- Modify: `src/tui/atlas/state.rs:6-16`
- Modify: `src/tui/mod.rs:10`

**Step 1: Fix atlas/state.rs doc comments**

Change bracket notation from `[a]` to `(a)` or escaped `\[a\]`:

```rust
// Change from:
/// [a] Spreading Activation — Cognitive science math behind context assembly

// To:
/// (a) Spreading Activation — Cognitive science math behind context assembly
```

Apply to all variants: `[a]`, `[b]`, `[c]`, `[v]`, `[e]`, `[r]`

**Step 2: Fix tui/mod.rs doc comment**

Fix the `:OF_KIND` reference - wrap in backticks:

```rust
// Change from:
/// - :OF_KIND relationships

// To:
/// - `OF_KIND` relationships
```

**Step 3: Verify rustdoc**

Run: `cargo doc --no-deps 2>&1 | grep -c "unresolved link"`
Expected: 0

**Step 4: Commit**

```bash
git add src/tui/atlas/state.rs src/tui/mod.rs
git commit -m "docs(tui): fix rustdoc warnings - escape brackets in doc comments"
```

---

## Phase 2: Performance Optimizations (2 hours)

### Task 2.1: Add LazyLock for Regex in slugification.rs

**Files:**
- Modify: `src/parsers/slugification.rs`

**Step 1: Add LazyLock import**

At top of file:
```rust
use std::sync::LazyLock;
use regex::Regex;
```

**Step 2: Create static regex constants**

Add after imports:
```rust
static FRONTMATTER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)^---\n(.*?)\n---").expect("valid frontmatter regex")
});

static SLUG_RULE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*Slug Rule\*\*:\s*(\w+)").expect("valid slug rule regex")
});

static STOPWORD_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\|\s*([^\|]+?)\s*\|\s*(article|preposition|conjunction|pronoun|verb|contraction|demonstrative|auxiliary|possessive|interrogative|negation|adverb|particle_\w+|honorific|classifier|copula|proper_noun|currency|relative_pronoun|indefinite|quantifier|interjection|abbreviation|filler|honorific_suffix)\s*\|").expect("valid stopword regex")
});

static CHAR_TRANSFORM_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\|\s*([^\|]+?)\s*\|\s*(\w+)\s*\|\s*([^\|]+?)\s*\|").expect("valid char transform regex")
});

static SCRIPT_CONFIG_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\|\s*([^\|]+?)\s*\|\s*([^\|]+?)\s*\|\s*([^\|]+?)\s*\|").expect("valid script config regex")
});

static EXAMPLE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\|\s*([^\|]+?)\s*\|\s*([^\|]+?)\s*\|").expect("valid example regex")
});
```

**Step 3: Replace Regex::new().unwrap() calls with static references**

Find each `Regex::new(...).unwrap()` and replace with the corresponding static:
- Line 330: `let re = Regex::new(...)` → `let re = &*FRONTMATTER_RE;`
- Line 344: Use `&*SLUG_RULE_RE`
- Line 359: Use `&*STOPWORD_RE`
- Line 389: Use `&*CHAR_TRANSFORM_RE`
- Line 440: Use `&*SCRIPT_CONFIG_RE`
- Line 487: Use `&*EXAMPLE_RE`

**Step 4: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 5: Run tests**

Run: `cargo test slugification`
Expected: All slugification tests pass

**Step 6: Commit**

```bash
git add src/parsers/slugification.rs
git commit -m "perf(parsers): use LazyLock for regex compilation in slugification"
```

---

### Task 2.2: Add LazyLock for Regex in formatting.rs

**Files:**
- Modify: `src/parsers/formatting.rs`

**Step 1: Add LazyLock import**

At top of file:
```rust
use std::sync::LazyLock;
use regex::Regex;
```

**Step 2: Create static regex constants**

Add after imports (for the ~29 regex patterns):
```rust
static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"template_version:\s*(.+)").expect("valid version regex")
});

static DATE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"last_updated:\s*(.+)").expect("valid date regex")
});

static DATA_SOURCES_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*Data Sources\*\*:\s*(.+)").expect("valid data sources regex")
});

static SECTION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"##\s+\d+\.\s+(.+)").expect("valid section regex")
});

static FIELD_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"-\s+\*\*(\w+)\*\*:\s*`?([^`\n]+)`?").expect("valid field regex")
});

// Add more as needed for each unique regex pattern...
```

**Step 3: Replace Regex::new().unwrap() calls**

Replace each inline regex with the static reference.

**Step 4: Verify compilation and tests**

Run: `cargo check && cargo test formatting`
Expected: All pass

**Step 5: Commit**

```bash
git add src/parsers/formatting.rs
git commit -m "perf(parsers): use LazyLock for regex compilation in formatting"
```

---

### Task 2.3: Add Vec::with_capacity in data.rs

**Files:**
- Modify: `src/tui/data.rs:648-682`

**Step 1: Find Vec constructions without capacity**

Run:
```bash
grep -n "let mut.*= Vec::new()" src/tui/data.rs | head -10
```

**Step 2: Add capacity hints**

For each `Vec::new()` where size is known or estimable:
```rust
// Change from:
let mut kinds = Vec::new();

// To (when iterating over known collection):
let mut kinds = Vec::with_capacity(layer.kinds.len());
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 4: Run tests**

Run: `cargo test`
Expected: All pass

**Step 5: Commit**

```bash
git add src/tui/data.rs
git commit -m "perf(tui): add Vec::with_capacity for known-size collections"
```

---

### Task 2.4: Parallelize Neo4j Queries with join!

**Files:**
- Modify: `src/tui/mod.rs:161-219`

**Step 1: Identify sequential loads that can run in parallel**

Current pattern (sequential):
```rust
if let Some(kind_label) = app.take_pending_instance_load() {
    if let Ok(instances) = TaxonomyTree::load_instances(db, &kind_label).await {
        app.tree.set_instances(&kind_label, instances);
    }
}
if let Some(kind_label) = app.take_pending_arcs_load() {
    if let Ok(arcs) = TaxonomyTree::load_kind_arcs(db, &kind_label).await {
        app.set_kind_arcs(arcs);
    }
}
// ... more sequential loads
```

**Step 2: Group independent loads with join!**

Note: Some loads depend on each other, so group only independent ones:
```rust
use tokio::join;

// Group 1: Instance and Arcs loads (independent)
let instance_load = app.take_pending_instance_load();
let arcs_load = app.take_pending_arcs_load();

if instance_load.is_some() || arcs_load.is_some() {
    let (inst_result, arcs_result) = join!(
        async {
            if let Some(kind_label) = &instance_load {
                TaxonomyTree::load_instances(db, kind_label).await.ok()
            } else {
                None
            }
        },
        async {
            if let Some(kind_label) = &arcs_load {
                TaxonomyTree::load_kind_arcs(db, kind_label).await.ok()
            } else {
                None
            }
        }
    );

    if let (Some(kind_label), Some(instances)) = (&instance_load, inst_result) {
        app.tree.set_instances(kind_label, instances);
    }
    if let Some(arcs) = arcs_result {
        app.set_kind_arcs(arcs);
    }
}
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 4: Manual test TUI**

Run: `cargo run -- tui`
Verify: Navigation still works, no hangs

**Step 5: Commit**

```bash
git add src/tui/mod.rs
git commit -m "perf(tui): parallelize independent Neo4j queries with tokio::join!"
```

---

### Task 2.5: Fix String Allocation in search.rs

**Files:**
- Modify: `src/commands/search.rs:14-15, 26-29`

**Step 1: Read current implementation**

```bash
sed -n '10,35p' src/commands/search.rs
```

**Step 2: Add String::with_capacity where building queries**

```rust
// Change from:
let mut cypher = String::from("MATCH (n) WHERE ...");

// To:
let mut cypher = String::with_capacity(512); // Estimate query size
cypher.push_str("MATCH (n) WHERE ...");
```

**Step 3: Verify and test**

Run: `cargo check && cargo test search`
Expected: Pass

**Step 4: Commit**

```bash
git add src/commands/search.rs
git commit -m "perf(commands): add String::with_capacity in search query building"
```

---

## Phase 3: Modernization (3 hours)

### Task 3.1: Add `serialize_json` Helper to cypher_utils.rs

**Files:**
- Modify: `src/generators/cypher_utils.rs`

**Step 1: Add the generic helper**

```rust
use serde::Serialize;

/// Serialize data to JSON string with fallback.
pub fn serialize_json<T: Serialize>(data: &T, fallback: &str) -> String {
    serde_json::to_string(data).unwrap_or_else(|_| fallback.to_string())
}
```

**Step 2: Update formatting.rs to use helper**

Replace repeated patterns:
```rust
// From:
let json = serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string());

// To:
use crate::generators::cypher_utils::serialize_json;
let json = serialize_json(&data, "{}");
```

**Step 3: Update slugification.rs similarly**

**Step 4: Verify and test**

Run: `cargo check && cargo test`
Expected: All pass

**Step 5: Commit**

```bash
git add src/generators/cypher_utils.rs src/generators/formatting.rs src/generators/slugification.rs
git commit -m "refactor(generators): add serialize_json helper, reduce boilerplate"
```

---

### Task 3.2: Add Missing Doc Comment to Severity Enum

**Files:**
- Modify: `src/commands/schema.rs:154`

**Step 1: Add doc comment**

```rust
/// Severity level for schema validation issues.
#[derive(Debug, PartialEq, Eq)]
pub enum Severity {
    /// Critical issue that must be fixed before proceeding.
    Error,
    /// Non-critical issue that should be addressed.
    Warning,
}
```

**Step 2: Verify rustdoc**

Run: `cargo doc --no-deps 2>&1 | grep "Severity"`
Expected: No warnings

**Step 3: Commit**

```bash
git add src/commands/schema.rs
git commit -m "docs(schema): add doc comments to Severity enum"
```

---

### Task 3.3: Migrate Critical eprintln! to tracing (Partial)

**Files:**
- Modify: `src/commands/node.rs`
- Modify: `src/commands/arc.rs`

**Step 1: Add tracing imports where missing**

```rust
use tracing::{info, warn};
```

**Step 2: Replace user-facing messages**

```rust
// From:
eprintln!("Created node: {}", key);

// To:
info!(key = %key, kind = %kind, "created node");
```

**Step 3: Verify compilation**

Run: `cargo check`
Expected: No errors

**Step 4: Commit**

```bash
git add src/commands/node.rs src/commands/arc.rs
git commit -m "refactor(commands): migrate node/arc eprintln to tracing"
```

---

## Phase 4: Testing (4 hours)

### Task 4.1: Add Integration Test for db.rs Connection

**Files:**
- Create: `tests/integration_db.rs`

**Step 1: Create integration test file**

```rust
//! Integration tests for db.rs (require running Neo4j)

use novanet::db::Db;

#[tokio::test]
#[ignore] // Requires Neo4j: run with `cargo test -- --ignored`
async fn test_db_connection_success() {
    let db = Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword").await;
    assert!(db.is_ok(), "Should connect to Neo4j");
}

#[tokio::test]
#[ignore]
async fn test_db_connection_failure() {
    let db = Db::connect("bolt://invalid:9999", "neo4j", "wrong").await;
    assert!(db.is_err(), "Should fail on invalid URI");
}

#[tokio::test]
#[ignore]
async fn test_db_execute_simple_query() {
    let db = Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword")
        .await
        .expect("connect");

    let rows = db.execute("RETURN 1 AS n").await;
    assert!(rows.is_ok());
    assert_eq!(rows.unwrap().len(), 1);
}
```

**Step 2: Verify tests compile**

Run: `cargo test --no-run`
Expected: Compiles

**Step 3: Run with Neo4j (if available)**

Run: `cargo test -- --ignored`
Expected: Pass (if Neo4j running)

**Step 4: Commit**

```bash
git add tests/integration_db.rs
git commit -m "test(db): add integration tests for Neo4j connection"
```

---

### Task 4.2: Add Unit Tests for locale.rs escape_cypher

**Files:**
- Modify: `src/commands/locale.rs` (add tests module)

**Step 1: Add tests module**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_cypher_quotes() {
        assert_eq!(escape_cypher("it's"), "it\\'s");
    }

    #[test]
    fn test_escape_cypher_newlines() {
        assert_eq!(escape_cypher("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_escape_cypher_injection_attempt() {
        assert_eq!(escape_cypher("'; DROP TABLE--"), "\\'; DROP TABLE--");
    }

    #[test]
    fn test_escape_cypher_empty() {
        assert_eq!(escape_cypher(""), "");
    }
}
```

**Step 2: Run tests**

Run: `cargo test locale`
Expected: All pass

**Step 3: Commit**

```bash
git add src/commands/locale.rs
git commit -m "test(locale): add unit tests for escape_cypher"
```

---

### Task 4.3: Add Tests for Retrieval Engine Budget Exhaustion

**Files:**
- Modify: `src/retrieval/engine.rs` (add tests)

**Step 1: Add test for budget exhaustion**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_request_default_budget() {
        let request = ContextRequest::default();
        assert!(request.budget > 0, "Default budget should be positive");
    }

    #[test]
    fn test_context_window_respects_budget() {
        let mut window = ContextWindow::new(100); // 100 token budget

        // Simulate adding nodes until budget exhausted
        for i in 0..20 {
            if !window.try_add_node(&format!("node_{}", i), 10) {
                break;
            }
        }

        assert!(window.token_count() <= 100, "Should not exceed budget");
    }
}
```

**Step 2: Run tests**

Run: `cargo test engine`
Expected: Pass

**Step 3: Commit**

```bash
git add src/retrieval/engine.rs
git commit -m "test(retrieval): add budget exhaustion tests for context engine"
```

---

## Final Verification

### Task 5.1: Full Test Suite

**Step 1: Run all tests**

Run: `cargo test`
Expected: All tests pass (should be 250+ now)

**Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

**Step 3: Run rustdoc**

Run: `cargo doc --no-deps 2>&1 | grep -c "warning"`
Expected: 0

**Step 4: Final commit**

```bash
git add -A
git commit -m "chore: complete TUI audit cleanup (4 phases)"
```

---

## Summary

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Phase 1: Quick Wins | 8 tasks | 1 hour |
| Phase 2: Performance | 5 tasks | 2 hours |
| Phase 3: Modernization | 3 tasks | 1 hour |
| Phase 4: Testing | 3 tasks | 2 hours |
| **Total** | **19 tasks** | **6 hours** |

Each task follows TDD where applicable, with exact file paths and verification steps.
