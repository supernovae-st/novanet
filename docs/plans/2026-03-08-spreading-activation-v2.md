# Spreading Activation v2 — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix spreading activation to use exponential decay, load configuration from YAML, and respect arc temperatures.

**Architecture:** Replace hardcoded linear `1/(d+1)` relevance with exponential `0.85^d` decay, load `spreading-activation.yaml` config at runtime, implement arc family filtering, and add block-type weighted context assembly.

**Tech Stack:** Rust (novanet-mcp), Cypher migrations, Neo4j

---

## Critical Findings (Agent Audit)

| Issue | Severity | Location |
|-------|----------|----------|
| 0/21 NATIVE_OF arcs exist | 🔴 Critical | Neo4j DB |
| spreading-activation.yaml NOT LOADED | 🔴 Critical | Runtime |
| Hardcoded `1/(d+1)` relevance | 🔴 Critical | `assemble.rs:320` |
| `arc_families` param UNUSED | 🟡 High | `assemble.rs:51` |
| No temperature weighting | 🟡 High | `assemble.rs` |
| No activation threshold | 🟡 High | `assemble.rs` |

---

## Phase 1: DB Integrity — NATIVE_OF Arcs

### Task 1.1: Write failing test for NATIVE_OF traversal

**Files:**
- Test: `tools/novanet-mcp/tests/integration/native_of_arcs.rs`

**Step 1: Write the failing test**

```rust
// tools/novanet-mcp/tests/integration/native_of_arcs.rs
use novanet_mcp::test_utils::setup_test_pool;

#[tokio::test]
async fn test_native_of_arcs_exist() {
    let pool = setup_test_pool().await;

    // Query for EntityNatives that should have NATIVE_OF arc back to Entity
    let query = r#"
        MATCH (en:EntityNative)
        WHERE NOT (en)-[:NATIVE_OF]->(:Entity)
        RETURN count(en) as orphan_count
    "#;

    let result = pool.execute(query, vec![]).await.unwrap();
    let orphan_count: i64 = result.rows[0]["orphan_count"].as_i64().unwrap();

    assert_eq!(orphan_count, 0, "Found {} EntityNatives without NATIVE_OF arc", orphan_count);
}

#[tokio::test]
async fn test_traverse_entity_to_native_bidirectional() {
    let pool = setup_test_pool().await;

    // Test that we can traverse Entity → EntityNative via HAS_NATIVE
    // AND back via NATIVE_OF
    let query = r#"
        MATCH (e:Entity {key: 'qr-code'})-[:HAS_NATIVE]->(en:EntityNative)-[:NATIVE_OF]->(e2:Entity)
        WHERE e = e2
        RETURN count(*) as count
    "#;

    let result = pool.execute(query, vec![]).await.unwrap();
    let count: i64 = result.rows[0]["count"].as_i64().unwrap();

    assert!(count > 0, "Bidirectional traversal via HAS_NATIVE/NATIVE_OF failed");
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p novanet-mcp test_native_of_arcs_exist -- --nocapture`
Expected: FAIL with `Found N EntityNatives without NATIVE_OF arc`

**Step 3: Commit failing test**

```bash
git add tools/novanet-mcp/tests/integration/native_of_arcs.rs
git commit -m "test(mcp): add failing test for NATIVE_OF arcs

NATIVE_OF is the inverse of HAS_NATIVE, required for:
- EntityNative → Entity traversal
- Bidirectional spreading activation

Currently 0/21 NATIVE_OF arcs exist.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

### Task 1.2: Create NATIVE_OF migration

**Files:**
- Create: `packages/db/migrations/061-create-native-of-arcs.cypher`

**Step 1: Write the migration**

```cypher
// packages/db/migrations/061-create-native-of-arcs.cypher
// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 061: Create NATIVE_OF inverse arcs
// ═══════════════════════════════════════════════════════════════════════════════
// Creates inverse NATIVE_OF arcs for all HAS_NATIVE relationships.
// NATIVE_OF points from *Native node back to its parent.
//
// Pattern: (Entity)-[:HAS_NATIVE]->(EntityNative)-[:NATIVE_OF]->(Entity)
// ═══════════════════════════════════════════════════════════════════════════════

// Create NATIVE_OF for EntityNative nodes
MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
WHERE NOT (en)-[:NATIVE_OF]->(e)
MERGE (en)-[:NATIVE_OF]->(e);

// Create NATIVE_OF for PageNative nodes
MATCH (p:Page)-[:HAS_NATIVE]->(pn:PageNative)
WHERE NOT (pn)-[:NATIVE_OF]->(p)
MERGE (pn)-[:NATIVE_OF]->(p);

// Create NATIVE_OF for BlockNative nodes
MATCH (b:Block)-[:HAS_NATIVE]->(bn:BlockNative)
WHERE NOT (bn)-[:NATIVE_OF]->(b)
MERGE (bn)-[:NATIVE_OF]->(b);

// Create NATIVE_OF for ProjectNative nodes
MATCH (proj:Project)-[:HAS_NATIVE]->(projn:ProjectNative)
WHERE NOT (projn)-[:NATIVE_OF]->(proj)
MERGE (projn)-[:NATIVE_OF]->(proj);

// Verify counts
MATCH ()-[r:NATIVE_OF]->()
RETURN count(r) as native_of_count;
```

**Step 2: Run the migration**

Run: `pnpm infra:migrate` (or `cypher-shell < packages/db/migrations/061-create-native-of-arcs.cypher`)
Expected: `native_of_count` matches count of HAS_NATIVE arcs

**Step 3: Verify test passes**

Run: `cargo test -p novanet-mcp test_native_of_arcs_exist -- --nocapture`
Expected: PASS

**Step 4: Commit migration**

```bash
git add packages/db/migrations/061-create-native-of-arcs.cypher
git commit -m "fix(db): create NATIVE_OF inverse arcs for all *Native nodes

Migration 061 creates NATIVE_OF arcs from:
- EntityNative → Entity
- PageNative → Page
- BlockNative → Block
- ProjectNative → Project

Enables bidirectional traversal for spreading activation.
Fixes CSR coverage issue detected by novanet_audit.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Phase 2: Configuration Loading

### Task 2.1: Create SpreadingConfig struct

**Files:**
- Create: `tools/novanet-mcp/src/config/mod.rs`
- Create: `tools/novanet-mcp/src/config/spreading.rs`
- Modify: `tools/novanet-mcp/src/lib.rs`

**Step 1: Write failing test**

```rust
// tools/novanet-mcp/src/config/spreading.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_spreading_config() {
        let config = SpreadingConfig::load().expect("Failed to load spreading config");

        // From spreading-activation.yaml
        assert_eq!(config.default.decay_factor, 0.01);
        assert_eq!(config.default.retention_factor, 0.5);
        assert_eq!(config.default.propagation_steps, 2);
        assert_eq!(config.default.activation_threshold, 0.3);
        assert_eq!(config.default.output_threshold, 0.1);
    }

    #[test]
    fn test_get_task_modifier() {
        let config = SpreadingConfig::load().expect("Failed to load");

        let cta = config.get_task_modifier("CTA");
        assert_eq!(cta.activation_threshold, 0.25);
        assert!(cta.semantic_boosts.get("urgency").is_some());
    }

    #[test]
    fn test_calculate_decay() {
        let config = SpreadingConfig::load().expect("Failed to load");

        // At depth 0: should be 1.0
        let decay_0 = config.calculate_decay(0);
        assert!((decay_0 - 1.0).abs() < 0.001);

        // At depth 2: e^(-0.01 * 2) ≈ 0.98
        let decay_2 = config.calculate_decay(2);
        assert!(decay_2 > 0.9 && decay_2 < 1.0);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p novanet-mcp test_load_spreading_config`
Expected: FAIL (module doesn't exist)

**Step 3: Implement SpreadingConfig**

```rust
// tools/novanet-mcp/src/config/mod.rs
mod spreading;

pub use spreading::SpreadingConfig;
```

```rust
// tools/novanet-mcp/src/config/spreading.rs
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// Spreading activation configuration loaded from spreading-activation.yaml
#[derive(Debug, Clone, Deserialize)]
pub struct SpreadingConfig {
    pub default: DefaultConfig,
    pub task_modifiers: HashMap<String, TaskModifier>,
    pub semantic_link_defaults: HashMap<String, f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DefaultConfig {
    pub decay_factor: f64,           // ρ - exponential decay
    pub retention_factor: f64,       // δ - activation retained
    pub propagation_steps: u32,      // T - max hops
    pub initial_activation: f64,     // A₀ - starting activation
    pub activation_threshold: f64,   // Min to continue spreading
    pub output_threshold: f64,       // Min to include in results
    pub max_fan_out: u32,            // Limit outgoing edges
    pub fan_penalty: f64,            // Reduce for high-degree nodes
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskModifier {
    pub activation_threshold: f64,
    pub propagation_steps: u32,
    pub semantic_boosts: HashMap<String, f64>,
    pub priority_filter: Vec<String>,
}

impl SpreadingConfig {
    /// Load configuration from spreading-activation.yaml
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::find_config_path()?;
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::Io(e, config_path.clone()))?;

        serde_yaml::from_str(&content)
            .map_err(|e| ConfigError::Parse(e, config_path))
    }

    /// Find spreading-activation.yaml in known locations
    fn find_config_path() -> Result<PathBuf, ConfigError> {
        let candidates = [
            // Relative to novanet-mcp binary
            PathBuf::from("../../schema/models/config/spreading-activation.yaml"),
            // Absolute path from NOVANET_SCHEMA_PATH env
            std::env::var("NOVANET_SCHEMA_PATH")
                .ok()
                .map(|p| PathBuf::from(p).join("models/config/spreading-activation.yaml"))
                .unwrap_or_default(),
            // Development fallback
            PathBuf::from("/Users/thibaut/dev/supernovae/schema/models/config/spreading-activation.yaml"),
        ];

        for path in &candidates {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        Err(ConfigError::NotFound(candidates.to_vec()))
    }

    /// Get task modifier or fallback to DEFAULT
    pub fn get_task_modifier(&self, task_type: &str) -> &TaskModifier {
        self.task_modifiers
            .get(task_type)
            .or_else(|| self.task_modifiers.get("DEFAULT"))
            .expect("DEFAULT task modifier must exist")
    }

    /// Calculate exponential decay: e^(-ρ × depth)
    pub fn calculate_decay(&self, depth: u32) -> f64 {
        (-self.default.decay_factor * depth as f64).exp()
    }

    /// Calculate relevance with temperature weighting
    pub fn calculate_relevance(&self, depth: u32, temperature: f64, semantic_type: Option<&str>, task_type: &str) -> f64 {
        let base_decay = self.calculate_decay(depth);
        let temp_weight = temperature.clamp(0.0, 1.0);

        let semantic_boost = semantic_type
            .and_then(|st| self.get_task_modifier(task_type).semantic_boosts.get(st))
            .copied()
            .unwrap_or(1.0);

        base_decay * temp_weight * semantic_boost
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Config file not found. Searched: {0:?}")]
    NotFound(Vec<PathBuf>),
    #[error("Failed to read {1}: {0}")]
    Io(std::io::Error, PathBuf),
    #[error("Failed to parse {1}: {0}")]
    Parse(serde_yaml::Error, PathBuf),
}

#[cfg(test)]
mod tests {
    // ... tests from Step 1
}
```

**Step 4: Update lib.rs**

```rust
// tools/novanet-mcp/src/lib.rs
// Add after other module declarations:
pub mod config;
```

**Step 5: Run tests**

Run: `cargo test -p novanet-mcp test_load_spreading_config`
Expected: PASS

**Step 6: Commit**

```bash
git add tools/novanet-mcp/src/config/
git add tools/novanet-mcp/src/lib.rs
git commit -m "feat(mcp): add SpreadingConfig loader for spreading-activation.yaml

SpreadingConfig loads and parses:
- Default parameters (decay_factor, retention_factor, thresholds)
- Task modifiers (CTA, FAQ, HERO, PRICING, TESTIMONIAL)
- Semantic link type defaults

Methods:
- calculate_decay(depth) → exponential decay e^(-ρ×d)
- calculate_relevance(depth, temp, type, task) → full relevance

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

### Task 2.2: Load config in State

**Files:**
- Modify: `tools/novanet-mcp/src/server/state.rs`

**Step 1: Write failing test**

```rust
#[tokio::test]
async fn test_state_has_spreading_config() {
    let state = State::new().await.expect("Failed to create state");
    let config = state.spreading_config();

    assert_eq!(config.default.decay_factor, 0.01);
}
```

**Step 2: Read current state.rs**

Read `tools/novanet-mcp/src/server/state.rs` to understand the structure.

**Step 3: Add SpreadingConfig to StateInner**

Add to `StateInner`:
```rust
spreading_config: SpreadingConfig,
```

Add loader in `State::new()`:
```rust
let spreading_config = SpreadingConfig::load()
    .map_err(|e| Error::config(format!("Failed to load spreading config: {}", e)))?;
```

Add accessor:
```rust
pub fn spreading_config(&self) -> &SpreadingConfig {
    &self.inner.spreading_config
}
```

**Step 4: Run tests**

Run: `cargo test -p novanet-mcp test_state_has_spreading_config`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet-mcp/src/server/state.rs
git commit -m "feat(mcp): load SpreadingConfig in server State

SpreadingConfig is loaded once at server startup and shared
across all tool invocations via State.spreading_config().

Fails fast if spreading-activation.yaml is not found.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

## Phase 3: Core Algorithm — Exponential Decay

### Task 3.1: Replace hardcoded relevance in assemble.rs

**Files:**
- Modify: `tools/novanet-mcp/src/tools/assemble.rs`

**Step 1: Write failing test**

```rust
#[tokio::test]
async fn test_evidence_uses_exponential_decay() {
    let state = test_state().await;
    let params = AssembleParams {
        focus_key: "qr-code".to_string(),
        locale: "en-US".to_string(),
        token_budget: Some(10000),
        strategy: None,
        ..Default::default()
    };

    let result = execute_assemble(&state, params).await.unwrap();

    // Find evidence at different depths
    let depth_1: Vec<_> = result.evidence.iter().filter(|e| e.distance == 1).collect();
    let depth_2: Vec<_> = result.evidence.iter().filter(|e| e.distance == 2).collect();

    if !depth_1.is_empty() && !depth_2.is_empty() {
        // Exponential decay: depth 2 relevance should be ~0.85 of depth 1
        let avg_rel_1: f64 = depth_1.iter().map(|e| e.relevance).sum::<f64>() / depth_1.len() as f64;
        let avg_rel_2: f64 = depth_2.iter().map(|e| e.relevance).sum::<f64>() / depth_2.len() as f64;

        // Check ratio is approximately 0.85 (exponential decay factor)
        let ratio = avg_rel_2 / avg_rel_1;
        assert!(ratio > 0.8 && ratio < 0.95, "Expected ~0.85 decay ratio, got {}", ratio);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p novanet-mcp test_evidence_uses_exponential_decay`
Expected: FAIL (ratio will be ~0.5 for linear decay)

**Step 3: Replace hardcoded calculation**

In `assemble.rs:320`, replace:
```rust
// OLD: Linear decay
relevance: 1.0 / (row["distance"].as_f64().unwrap_or(1.0) + 1.0),
```

With:
```rust
// NEW: Exponential decay from config
relevance: state.spreading_config().calculate_relevance(
    row["distance"].as_u64().unwrap_or(1) as u32,
    row["temperature"].as_f64().unwrap_or(1.0),
    row["semantic_type"].as_str(),
    block_type.unwrap_or("DEFAULT"),
),
```

Also update the Cypher query to return `temperature` and `semantic_type`:
```cypher
MATCH (focus {key: $key})-[r*1..3]-(related)
WHERE labels(related)[0] IN ['Entity', 'EntityNative', 'Term']
WITH related, length(r) AS distance,
     CASE WHEN r[-1].temperature IS NOT NULL THEN r[-1].temperature ELSE 1.0 END AS temperature,
     r[-1].semantic_type AS semantic_type
RETURN DISTINCT related.key AS key, ...
```

**Step 4: Run tests**

Run: `cargo test -p novanet-mcp test_evidence_uses_exponential_decay`
Expected: PASS

**Step 5: Also update structure relevance at line 436**

Same pattern for structure evidence.

**Step 6: Commit**

```bash
git add tools/novanet-mcp/src/tools/assemble.rs
git commit -m "feat(mcp): replace linear decay with exponential from config

BEFORE: relevance = 1/(d+1) → 1, 0.5, 0.33, 0.25
AFTER:  relevance = e^(-ρ×d) × temp × boost → 1, 0.99, 0.98...

Cypher queries now return temperature and semantic_type
for calculate_relevance() to use.

Fixes hardcoded relevance issue from agent audit.

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika 🦋 <nika@supernovae.studio>"
```

---

### Task 3.2: Add activation threshold cutoff

**Files:**
- Modify: `tools/novanet-mcp/src/tools/assemble.rs`

**Step 1: Write failing test**

```rust
#[tokio::test]
async fn test_low_relevance_evidence_filtered() {
    let state = test_state().await;
    let params = AssembleParams {
        focus_key: "qr-code".to_string(),
        locale: "en-US".to_string(),
        token_budget: Some(10000),
        ..Default::default()
    };

    let result = execute_assemble(&state, params).await.unwrap();

    // All evidence should have relevance >= output_threshold (0.1)
    for evidence in &result.evidence {
        assert!(
            evidence.relevance >= 0.1,
            "Found evidence with relevance {} below threshold 0.1",
            evidence.relevance
        );
    }
}
```

**Step 2: Add filtering after relevance calculation**

```rust
// After calculating evidence, filter by output_threshold
let output_threshold = state.spreading_config().default.output_threshold;
evidence.retain(|e| e.relevance >= output_threshold);
```

**Step 3: Run tests and commit**

---

## Phase 4: Arc Family Filtering

### Task 4.1: Implement arc_families parameter

**Files:**
- Modify: `tools/novanet-mcp/src/tools/assemble.rs`

**Step 1: Write failing test**

```rust
#[tokio::test]
async fn test_arc_families_filter() {
    let state = test_state().await;
    let params = AssembleParams {
        focus_key: "qr-code".to_string(),
        locale: "en-US".to_string(),
        arc_families: Some(vec!["semantic".to_string()]),  // Only semantic arcs
        ..Default::default()
    };

    let result = execute_assemble(&state, params).await.unwrap();

    // Should only include evidence from SEMANTIC_LINK, USES_ENTITY, etc.
    // Not from ownership arcs like HAS_NATIVE
}
```

**Step 2: Modify Cypher query to filter by arc family**

Update the Cypher query to include arc family filter:

```cypher
// Build arc type filter based on families
WITH $arc_families AS families
MATCH (focus {key: $key})
CALL {
    WITH focus, families
    // If families is null or empty, match all arcs
    // Otherwise filter by arc type family
    MATCH (focus)-[r]->(related)
    WHERE families IS NULL
       OR size(families) = 0
       OR type(r) IN CASE
           WHEN 'ownership' IN families THEN ['HAS_NATIVE', 'HAS_BLOCK', 'HAS_PAGE']
           WHEN 'semantic' IN families THEN ['SEMANTIC_LINK', 'USES_ENTITY', 'REFERENCES']
           WHEN 'localization' IN families THEN ['FOR_LOCALE', 'NATIVE_OF']
           ELSE []
       END
    RETURN related, r
}
...
```

**Step 3: Run tests and commit**

---

## Phase 5: Block-Type Weights

### Task 5.1: Pass block_type to context assembly

**Files:**
- Modify: `tools/novanet-mcp/src/tools/generate.rs`

**Step 1: Detect block_type from focus_key**

```rust
/// Extract block type from key pattern: "block:hero@en-US" → "HERO"
fn extract_block_type(key: &str) -> Option<String> {
    // Pattern: block:{block_type}@{locale} or block:{page}:{block_type}@{locale}
    if key.starts_with("block:") {
        let parts: Vec<_> = key.split('@').next()?.split(':').collect();
        if parts.len() >= 2 {
            return Some(parts.last()?.to_uppercase());
        }
    }
    None
}
```

**Step 2: Pass block_type to assemble**

In `execute_generate()`, pass the detected block_type:

```rust
let block_type = extract_block_type(&params.focus_key);

// When calling assemble for block content
let assemble_params = AssembleParams {
    focus_key: params.focus_key.clone(),
    locale: params.locale.clone(),
    block_type,  // NEW: pass block type for task-specific modifiers
    ..Default::default()
};
```

**Step 3: Use task modifier in assemble**

In `assemble.rs`, use the block_type to get task-specific parameters:

```rust
let task_modifier = block_type
    .as_deref()
    .map(|bt| state.spreading_config().get_task_modifier(bt))
    .unwrap_or_else(|| state.spreading_config().get_task_modifier("DEFAULT"));

let threshold = task_modifier.activation_threshold;
let propagation_steps = task_modifier.propagation_steps;
```

**Step 4: Run tests and commit**

---

## Phase 6: Tests & Benchmarks

### Task 6.1: Add comprehensive integration tests

**Files:**
- Create: `tools/novanet-mcp/tests/integration/spreading_activation.rs`

**Tests to add:**

1. `test_spreading_config_load` — Config loads from YAML
2. `test_exponential_decay_at_depths` — Verify decay curve
3. `test_temperature_weighting` — High-temp links ranked higher
4. `test_semantic_boost_cta` — CTA blocks boost urgency concepts
5. `test_semantic_boost_faq` — FAQ blocks boost definitions
6. `test_arc_families_ownership_only` — Filter to ownership arcs
7. `test_arc_families_semantic_only` — Filter to semantic arcs
8. `test_output_threshold_filtering` — Low relevance filtered out
9. `test_native_of_bidirectional` — Can traverse both directions
10. `test_fan_penalty_high_degree` — High-degree nodes penalized

---

### Task 6.2: Add performance benchmark

**Files:**
- Create: `tools/novanet-mcp/benches/spreading_activation.rs`

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use novanet_mcp::config::SpreadingConfig;

fn bench_decay_calculation(c: &mut Criterion) {
    let config = SpreadingConfig::load().unwrap();

    c.bench_function("calculate_decay_depth_3", |b| {
        b.iter(|| config.calculate_decay(3))
    });
}

fn bench_relevance_calculation(c: &mut Criterion) {
    let config = SpreadingConfig::load().unwrap();

    c.bench_function("calculate_relevance_full", |b| {
        b.iter(|| config.calculate_relevance(2, 0.8, Some("urgency"), "CTA"))
    });
}

fn bench_context_assembly(c: &mut Criterion) {
    // Requires running Neo4j
    // Benchmark full context assembly for a typical block
}

criterion_group!(benches, bench_decay_calculation, bench_relevance_calculation);
criterion_main!(benches);
```

---

### Task 6.3: Verify with novanet_audit

After all changes, run:

```bash
# Check NATIVE_OF coverage
novanet_audit target=coverage

# Expected: CSR ≥ 0.95 for NATIVE_OF arcs
```

---

## Verification Checklist

After completing all phases:

- [ ] `cargo test -p novanet-mcp` — All tests pass
- [ ] `cargo clippy -- -D warnings` — Zero warnings
- [ ] `novanet_audit target=coverage` — CSR ≥ 0.95
- [ ] `novanet_generate` benchmark — Context assembly < 100ms
- [ ] spreading-activation.yaml changes propagate to runtime
- [ ] Block-type modifiers (CTA, FAQ, HERO) affect relevance scores

---

## Summary

| Phase | Tasks | Key Changes |
|-------|-------|-------------|
| 1. DB Integrity | 2 | Create NATIVE_OF migration |
| 2. Config Loading | 2 | SpreadingConfig struct + State integration |
| 3. Core Algorithm | 2 | Exponential decay + threshold filtering |
| 4. Arc Family | 1 | Implement unused arc_families param |
| 5. Block Weights | 1 | Task-specific modifiers for block types |
| 6. Tests | 3 | Integration tests + benchmarks + audit |

**Total: 11 tasks across 6 phases**

---

## Next Steps

After this plan is executed:
1. Run `novanet_audit target=all` to verify CSR
2. Update CLAUDE.md with new spreading activation features
3. Tag release v0.18.0 "Spreading Activation v2"
