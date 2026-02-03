# V10 Verification, Optimization & Unification Plan

**Date**: 2026-02-03
**Status**: Draft
**Scope**: NovaNet Rust binary (`tools/novanet/`)

---

## Executive Summary

This plan consolidates findings from the Ralph Wiggum checkpoint (code explorer, code reviewer, architect, security) into actionable phases for achieving production-grade code quality.

---

## Phase 1: Critical Security Fixes (DONE)

| Task | Status | Impact |
|------|--------|--------|
| Update `bytes` 1.11.0 → 1.11.1 (CVE fix) | DONE | Memory safety |
| Add Neo4j identifier validation in `db.rs` | DONE | Injection defense |

**Remaining**: Migrate from `serde_yml` to `serde_yaml` (HIGH priority - unsound library)

---

## Phase 2: Code Deduplication (DONE)

| Task | Status | Lines Saved |
|------|--------|-------------|
| Create `cypher_utils.rs` module | DONE | ~60 lines |
| Extract `cypher_str()` | DONE | 3 implementations → 1 |
| Extract `cypher_list_ref/owned()` | DONE | 2 implementations → 2 (typed) |
| Extract `write_merge_meta()` | DONE | Shared across organizing.rs |
| Harmonize REALM_ORDER constants | DONE | Bug prevention |

---

## Phase 3: Debugging Infrastructure

### 3.1 Structured Logging
```rust
// Current: eprintln!("...")
// Target: tracing with structured fields

use tracing::{info, debug, warn, error, instrument};

#[instrument(skip(db), fields(root = %root.display()))]
pub async fn run_seed(db: &Db, root: &Path) -> crate::Result<()> {
    info!("Starting database seed");
    // ...
    debug!(file = %path.display(), "Executing Cypher file");
}
```

**Files to update**:
- `commands/db.rs` - seed/migrate/reset operations
- `commands/schema.rs` - generate/validate operations
- `tui/runtime.rs` - TUI event loop
- `db.rs` - connection pool

### 3.2 Error Context Enhancement
```rust
// Current
#[error("generator failed: {generator}")]
Generator { generator: String, detail: String },

// Target
#[error("generator '{generator}' failed: {detail}")]
Generator { generator: String, detail: String },
```

### 3.3 Debug Mode Features
```toml
# Cargo.toml - add debug feature
[features]
debug = ["tracing-subscriber/fmt", "color-eyre/track-caller"]
```

---

## Phase 4: Performance Optimization

### 4.1 Parallel YAML Processing
```rust
// Current: Sequential node loading
let mut nodes = Vec::new();
for path in paths {
    nodes.push(parse_node(&path)?);
}

// Target: Parallel with rayon (already a dependency)
use rayon::prelude::*;
let nodes: Result<Vec<_>, _> = paths
    .par_iter()
    .map(|path| parse_node(path))
    .collect();
```

**Impact**: 44 nodes × ~5ms each = ~220ms → ~50ms (4x speedup)

### 4.2 String Allocation Reduction
```rust
// Current: Multiple allocations
fn cypher_str(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()  // allocation
        .join(" ")            // allocation
        .replace('\'', "\\'") // allocation
}

// Target: Single allocation with capacity hint
fn cypher_str(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut first = true;
    for word in s.split_whitespace() {
        if !first { result.push(' '); }
        first = false;
        for c in word.chars() {
            if c == '\'' { result.push_str("\\'"); }
            else { result.push(c); }
        }
    }
    result
}
```

### 4.3 Lazy Initialization
```rust
// TUI: Defer Neo4j connection until needed
pub struct App {
    db: OnceCell<Db>,  // Lazy connection
}
```

---

## Phase 5: Security Hardening

### 5.1 Dependency Migration
```toml
# Current (unsound)
serde_yml = "0.0.12"

# Target (maintained)
serde_yaml = "0.9"
```

**Migration steps**:
1. Update Cargo.toml
2. Replace `serde_yml::from_str` → `serde_yaml::from_str`
3. Update error handling (different error type)
4. Run full test suite

### 5.2 Input Validation
```rust
// Add to commands/node.rs
fn validate_props_keys(props: &serde_json::Value) -> crate::Result<()> {
    if let serde_json::Value::Object(map) = props {
        for key in map.keys() {
            if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err(NovaNetError::Validation(
                    format!("Invalid property key: {key}")
                ));
            }
        }
    }
    Ok(())
}
```

### 5.3 Search Limit Bounds
```rust
// main.rs - add range validation
#[arg(long, default_value_t = 50, value_parser = clap::value_parser!(i64).range(1..=10000))]
limit: i64,
```

---

## Phase 6: Nomenclature Unification

### 6.1 v9 Terminology Audit
| Old Term | New Term | Status |
|----------|----------|--------|
| Scope | Realm | DONE |
| Subcategory | Layer | DONE |
| NodeTypeMeta | Kind | DONE |
| DataMode | NavigationMode | DONE |
| IN_SUBCATEGORY | OF_KIND | DONE |

### 6.2 Code Comments Cleanup
```bash
# Search for legacy terms in comments
grep -rn "subcategor\|NodeTypeMeta\|DataMode" src/
```

### 6.3 Documentation Sync
- [ ] Update CLAUDE.md with final v9 terminology
- [ ] Update KEYBINDINGS.md
- [ ] Update README.md (if exists)
- [ ] Generate fresh Mermaid diagrams

---

## Phase 7: Architecture Unification

### 7.1 Generator Pattern Consistency
```rust
// Target: All generators use cypher_utils
use super::cypher_utils::{cypher_str, cypher_list_ref, write_section_header};
```

### 7.2 Parser Pattern Consistency
```rust
// Create src/parsers/utils.rs
pub fn load_yaml<T: DeserializeOwned>(path: &Path) -> crate::Result<T> {
    if !path.exists() {
        return Err(NovaNetError::Validation(...));
    }
    let content = std::fs::read_to_string(path)?;
    serde_yaml::from_str(&content).map_err(|e| NovaNetError::Schema { ... })
}
```

### 7.3 Error Hierarchy
```rust
// Enhance NovaNetError with source chaining
#[derive(Error, Debug)]
pub enum NovaNetError {
    #[error("file not found: {path}")]
    FileNotFound { path: String },

    #[error("schema error in {path}")]
    Schema {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },

    // ... existing variants
}
```

---

## Phase 8: Testing Enhancement

### 8.1 Snapshot Testing
```rust
// Use insta for generator output validation
#[test]
fn generate_kinds_snapshot() {
    let nodes = vec![/* test data */];
    let output = generate_kind(&nodes).unwrap();
    insta::assert_snapshot!(output);
}
```

### 8.2 Property-Based Testing
```toml
# Cargo.toml dev-dependencies
proptest = "1.4"
```

```rust
proptest! {
    #[test]
    fn cypher_str_never_contains_unescaped_quotes(s in ".*") {
        let result = cypher_str(&s);
        assert!(!result.contains("'") || result.contains("\\'"));
    }
}
```

### 8.3 Integration Test Coverage
- [ ] `db seed` with mock Neo4j
- [ ] `schema generate --dry-run` full pipeline
- [ ] TUI navigation smoke tests

---

## Execution Priority

| Phase | Priority | Effort | Impact |
|-------|----------|--------|--------|
| Phase 1 | DONE | - | - |
| Phase 2 | DONE | - | - |
| Phase 5.1 (serde_yaml migration) | P0 | 2h | Security |
| Phase 3.2 (error display) | P1 | 30min | UX |
| Phase 5.3 (search limit) | P1 | 15min | DoS prevention |
| Phase 6 (nomenclature) | P2 | 1h | Consistency |
| Phase 4.1 (parallel YAML) | P2 | 2h | Performance |
| Phase 7 (parsers/utils) | P2 | 2h | Maintainability |
| Phase 8 (testing) | P3 | 4h | Reliability |
| Phase 3.1 (structured logging) | P3 | 3h | Observability |
| Phase 4.2-4.3 (micro-opts) | P4 | 2h | Marginal |

---

## Verification Checklist

### Pre-Commit
- [ ] `cargo fmt --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo run -- schema validate`

### Pre-Release
- [ ] `cargo audit` (security vulnerabilities)
- [ ] `cargo test --all-features`
- [ ] `cargo run -- schema generate` (7 artifacts)
- [ ] Manual TUI smoke test

### Post-Refactor
- [ ] Run Ralph Wiggum checkpoint
- [ ] Verify no legacy terminology in code
- [ ] Confirm test count stable or increased

---

## Metrics

| Metric | Before | Target | Current |
|--------|--------|--------|---------|
| Tests | 180 | 200+ | 185 |
| Clippy warnings | 0 | 0 | 0 |
| CVEs | 1 | 0 | 0 |
| Dead code items | 5 | 0 | 1 |
| Code duplication | HIGH | LOW | LOW |
| Unsound deps | 1 | 0 | 1 (serde_yml) |

---

## Notes

### Context7 Integration
Use Context7 MCP for up-to-date Rust documentation:
- `ratatui` TUI patterns
- `neo4rs` async patterns
- `tokio` runtime best practices

### Rust Best Practices
Apply spn-rust skills for:
- Error handling patterns (`rust-core`)
- Async patterns (`rust-async`)
- Security audit (`rust-security`)
- Performance optimization (`rust-perf`)

---

---

## Detailed Implementation Plan (Approved)

### Batch 1: P0 Security (serde_yaml migration)

**Task 1.1: Update Cargo.toml**
```bash
# Replace serde_yml with serde_yaml
sed -i '' 's/serde_yml = "0.0.12"/serde_yaml = "0.9"/' Cargo.toml
```

**Task 1.2: Update imports in all parsers**
```rust
// Files to update:
// - src/parsers/yaml_node.rs
// - src/parsers/relations.rs
// - src/parsers/organizing.rs
// - src/parsers/views.rs

// Before:
use serde_yml;
serde_yml::from_str(&content)?

// After:
use serde_yaml;
serde_yaml::from_str(&content)?
```

**Task 1.3: Update error handling**
```rust
// serde_yaml::Error has different structure
// Update NovaNetError::Schema variant if needed
```

**Task 1.4: Verify**
```bash
cargo build && cargo test && cargo clippy -- -D warnings
```

---

### Batch 2: Quick Wins (30 min total)

**Task 2.1: Generator error display improvement**
```rust
// src/error.rs - line 18
// Before:
#[error("generator failed: {generator}")]
Generator { generator: String, detail: String },

// After:
#[error("generator '{generator}' failed: {detail}")]
Generator { generator: String, detail: String },
```

**Task 2.2: Search limit bounds**
```rust
// src/main.rs - SearchArgs struct
// Before:
#[arg(long, default_value_t = 50)]
limit: i64,

// After:
#[arg(long, default_value_t = 50, value_parser = clap::value_parser!(i64).range(1..=10000))]
limit: i64,
```

**Task 2.3: Remove default password from code**
```rust
// src/db.rs - move to env-only
// Current: password defaults to "novanetpassword"
// Target: require NEO4J_PASSWORD env var, fail with clear error if missing
```

---

### Batch 3: Parser Utils (2h)

**Task 3.1: Create src/parsers/utils.rs**
```rust
//! Shared utilities for YAML parsing.

use std::path::Path;
use serde::de::DeserializeOwned;
use crate::{NovaNetError, Result};

/// Load and parse a YAML file with proper error context.
pub fn load_yaml<T: DeserializeOwned>(path: &Path) -> Result<T> {
    if !path.exists() {
        return Err(NovaNetError::FileNotFound {
            path: path.display().to_string(),
        });
    }
    let content = std::fs::read_to_string(path)
        .map_err(NovaNetError::Io)?;
    serde_yaml::from_str(&content).map_err(|e| NovaNetError::Schema {
        path: path.display().to_string(),
        detail: e.to_string(),
    })
}

/// Validate that a key matches expected format (kebab-case).
pub fn validate_key(key: &str, context: &str) -> Result<()> {
    if key.is_empty() {
        return Err(NovaNetError::Validation(
            format!("{context}: key cannot be empty")
        ));
    }
    if !key.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return Err(NovaNetError::Validation(
            format!("{context}: key '{key}' must be kebab-case")
        ));
    }
    Ok(())
}
```

**Task 3.2: Update parsers to use utils**
```rust
// yaml_node.rs, relations.rs, organizing.rs, views.rs
use super::utils::load_yaml;

// Replace inline fs::read_to_string + serde_yaml::from_str patterns
```

---

### Batch 4: Performance (2h)

**Task 4.1: Parallel YAML loading**
```rust
// src/parsers/yaml_node.rs - parse_all_nodes()
use rayon::prelude::*;

pub fn parse_all_nodes(root: &Path) -> Result<Vec<YamlNode>> {
    let paths = discover_yaml_files(root)?;

    // Parallel parsing with rayon
    let results: Vec<Result<YamlNode>> = paths
        .par_iter()
        .map(|path| parse_node(path))
        .collect();

    // Collect results, fail on first error
    results.into_iter().collect()
}
```

**Task 4.2: Add timing instrumentation**
```rust
// src/commands/schema.rs
use std::time::Instant;

let start = Instant::now();
let nodes = parse_all_nodes(root)?;
tracing::info!(count = nodes.len(), elapsed_ms = start.elapsed().as_millis(), "Parsed nodes");
```

---

### Batch 5: Testing Enhancement (3h)

**Task 5.1: Add insta snapshot tests for generators**
```rust
// tests/generators_snapshot.rs
use insta::assert_snapshot;
use novanet::generators::*;

#[test]
fn snapshot_kind_generator() {
    let nodes = vec![/* minimal test data */];
    let output = generate_kind(&nodes).unwrap();
    assert_snapshot!(output);
}

#[test]
fn snapshot_edge_schema_generator() {
    let relations = /* test relations */;
    let output = generate_edge_schema(&relations).unwrap();
    assert_snapshot!(output);
}
```

**Task 5.2: Add proptest for cypher_utils**
```rust
// src/generators/cypher_utils.rs
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn cypher_str_escapes_all_quotes(s in ".*") {
            let result = cypher_str(&s);
            // No unescaped single quotes
            assert!(!result.contains("'") || result.contains("\\'"));
        }

        #[test]
        fn cypher_list_produces_valid_syntax(items in prop::collection::vec("\\w+", 0..10)) {
            let refs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
            let result = cypher_list_ref(&refs);
            assert!(result.starts_with('['));
            assert!(result.ends_with(']'));
        }
    }
}
```

---

### Batch 6: Structured Logging (2h)

**Task 6.1: Add tracing instrumentation**
```rust
// src/commands/db.rs
use tracing::{info, debug, warn, instrument};

#[instrument(skip(db), fields(root = %root.display()))]
pub async fn run_seed(db: &Db, root: &Path) -> crate::Result<()> {
    info!("Starting database seed");
    // ... existing code
    debug!(file = %path.display(), "Executing Cypher file");
}
```

**Task 6.2: Configure tracing-subscriber in main**
```rust
// src/main.rs
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Only init tracing if not in TUI mode
    if !is_tui_command() {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }

    // ... existing code
}
```

---

### Batch 7: CI Pipeline (GitHub Actions)

**Task 7.1: Create .github/workflows/rust.yml**
```yaml
name: Rust CI

on:
  push:
    branches: [main]
    paths:
      - 'tools/novanet/**'
  pull_request:
    paths:
      - 'tools/novanet/**'

jobs:
  check:
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: tools/novanet
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
        with:
          workspaces: tools/novanet

      - name: Format check
        run: cargo fmt --check

      - name: Clippy
        run: cargo clippy -- -D warnings

      - name: Test
        run: cargo test

      - name: Audit
        run: cargo audit

      - name: Build release
        run: cargo build --release
```

---

## Execution Order

| Batch | Tasks | Est. Time | Dependencies |
|-------|-------|-----------|--------------|
| 1 | serde_yaml migration | 2h | None |
| 2 | Quick wins (error, limits, password) | 30min | None |
| 3 | Parser utils.rs | 2h | Batch 1 |
| 4 | Parallel YAML loading | 2h | Batch 3 |
| 5 | Testing enhancement | 3h | Batch 1-4 |
| 6 | Structured logging | 2h | None |
| 7 | CI pipeline | 1h | Batch 1-6 |

**Total estimated**: ~12h

---

## Verification After Each Batch

```bash
# After every batch:
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo run -- schema validate
```

---

---

## Phase 9: Advanced Optimizations (from rust-perf analysis)

### 9.1 Parallel TUI Data Loading
```rust
// src/tui/data.rs - load independent queries in parallel
use tokio::join;

pub async fn load(db: &Db, root: &Path) -> Result<Self> {
    // Load independent data in parallel
    let (stats_result, realms_result, edge_families_result) = join!(
        Self::load_stats(db),
        Self::load_realms(db, root),
        Self::load_edge_families(db)
    );

    let stats = stats_result?;
    let realms = realms_result?;
    let edge_families = edge_families_result?;
    // ...
}
```

### 9.2 TUI Caching Layer
```rust
// src/tui/app.rs - cache YAML content and tree lines
pub struct App {
    // Existing fields...

    // Caching
    yaml_cache: HashMap<String, String>,      // path -> content
    yaml_highlighted: Vec<Line<'static>>,     // Pre-computed highlighting
    tree_lines_cache: Vec<CachedTreeLine>,    // Cached tree rendering
    tree_dirty: bool,                         // Rebuild flag
}

impl App {
    pub fn load_yaml_for_current(&mut self) {
        // Check cache first
        if let Some(cached) = self.yaml_cache.get(&self.yaml_path) {
            if self.yaml_content != *cached {
                self.yaml_content = cached.clone();
                self.rebuild_yaml_highlighting();
            }
            return;
        }
        // Load from disk and cache
        let content = fs::read_to_string(&full_path).unwrap_or_default();
        self.yaml_cache.insert(self.yaml_path.clone(), content.clone());
        self.yaml_content = content;
        self.rebuild_yaml_highlighting();
    }
}
```

### 9.3 FxHashSet for Collapsed State
```toml
# Cargo.toml
rustc-hash = "2.0"  # Faster hashing for string keys
```

```rust
// src/tui/data.rs
use rustc_hash::FxHashSet;

pub struct TaxonomyTree {
    pub collapsed: FxHashSet<String>,  // 30% faster lookups
}
```

### 9.4 SmallVec for Small Collections
```toml
# Cargo.toml
smallvec = "1.13"
```

```rust
// src/parsers/yaml_node.rs
use smallvec::SmallVec;

pub struct KindDef {
    pub properties: SmallVec<[Property; 16]>,           // Usually < 16
    pub required_properties: SmallVec<[String; 8]>,     // Usually < 8
    pub edges_out: SmallVec<[EdgeDef; 8]>,              // Usually < 8
}
```

### 9.5 Clippy Pedantic Fixes
```bash
# Run pedantic clippy
cargo clippy -- -W clippy::pedantic -A clippy::must_use_candidate

# Fixes needed:
# - Unnecessary raw string hashes (r##"..."## → r#"..."#)
# - Unnested or-patterns (Some(A) | Some(B) → Some(A | B))
# - Manual map entry usage
```

### 9.6 Zero-Copy Serde Patterns
```rust
// Use Cow for strings that may or may not need allocation
use std::borrow::Cow;

#[derive(Deserialize)]
pub struct NodeDocument<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub description: Cow<'a, str>,
}
```

---

## Phase 10: Performance Metrics & Profiling

### 10.1 Add Timing Instrumentation
```rust
// src/commands/schema.rs
use std::time::Instant;
use tracing::info;

pub fn run_generate(root: &Path, dry_run: bool) -> Result<()> {
    let total_start = Instant::now();

    let parse_start = Instant::now();
    let nodes = load_all_nodes(root)?;
    info!(elapsed_ms = parse_start.elapsed().as_millis(), count = nodes.len(), "Parsed nodes");

    let gen_start = Instant::now();
    // ... generate artifacts
    info!(elapsed_ms = gen_start.elapsed().as_millis(), "Generated artifacts");

    info!(total_ms = total_start.elapsed().as_millis(), "Schema generation complete");
    Ok(())
}
```

### 10.2 Benchmark Suite
```rust
// benches/generators.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_yaml_parsing(c: &mut Criterion) {
    let root = Path::new("../../");
    c.bench_function("parse_all_nodes", |b| {
        b.iter(|| load_all_nodes(root).unwrap())
    });
}

fn bench_cypher_generation(c: &mut Criterion) {
    let nodes = load_all_nodes(root).unwrap();
    c.bench_function("generate_kind", |b| {
        b.iter(|| generate_kind(&nodes).unwrap())
    });
}

criterion_group!(benches, bench_yaml_parsing, bench_cypher_generation);
criterion_main!(benches);
```

---

## Execution Priority (Updated)

| Phase | Priority | Impact | Effort |
|-------|----------|--------|--------|
| Phase 5.1 (serde_yaml) | P0 | Security | 2h |
| Phase 9.1 (parallel TUI load) | P1 | Startup time | 1h |
| Phase 4.1 (parallel YAML) | P1 | Startup time | 2h |
| Phase 9.2 (TUI caching) | P2 | Responsiveness | 2h |
| Phase 9.3-9.4 (FxHash/SmallVec) | P2 | Memory/Speed | 1h |
| Phase 9.5 (clippy pedantic) | P3 | Code quality | 30min |
| Phase 10 (benchmarks) | P3 | Observability | 2h |
| Phase 9.6 (zero-copy serde) | P4 | Memory | 3h |

---

**Next Action**: Execute Batch 1 (serde_yaml migration)
