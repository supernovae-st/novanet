# Phase 9.9 + v10 Context Assembly Engine Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add traversal properties to schema (Phase 9.9) and implement the v10 Context Assembly Engine that reads meta-graph rules to autonomously assemble context windows.

**Architecture:** Data-driven traversal where the engine reads `traversal_depth`, `context_budget`, `default_traversal`, and `temperature_threshold` from meta-graph nodes to make autonomous decisions (no hardcoded traversal logic).

**Tech Stack:** Rust (tools/novanet), YAML schemas, Neo4j Cypher, serde for serialization.

---

## Phase 9.9: Schema Properties for Dynamic Retrieval

### Task 1: Add Retrieval Properties to taxonomy.yaml

**Files:**
- Modify: `packages/core/models/taxonomy.yaml:142-200`

**Step 1: Add kind_retrieval_defaults section after node_traits**

Add the following YAML after line 200 (after `node_traits`):

```yaml
# =============================================================================
# RETRIEVAL PROPERTIES — v9.9 Context Assembly
# =============================================================================
# Default values per trait for context assembly engine

kind_retrieval_defaults:
  # Token budget allocation per trait type
  invariant:
    traversal_depth: 2
    context_budget: 500
    token_estimate: 100
  localized:
    traversal_depth: 2
    context_budget: 800
    token_estimate: 150
  knowledge:
    traversal_depth: 1
    context_budget: 200
    token_estimate: 50
  derived:
    traversal_depth: 1
    context_budget: 100
    token_estimate: 30
  job:
    traversal_depth: 0
    context_budget: 0
    token_estimate: 20
```

**Step 2: Add default_traversal to arc_families**

Modify each arc_family entry to add `default_traversal`:

```yaml
arc_families:
  - key: ownership
    # ... existing fields ...
    default_traversal: eager  # Always follow in traversal

  - key: localization
    # ... existing fields ...
    default_traversal: eager  # Always follow for content

  - key: semantic
    # ... existing fields ...
    default_traversal: lazy   # Follow if budget allows

  - key: generation
    # ... existing fields ...
    default_traversal: lazy   # Follow for provenance

  - key: mining
    # ... existing fields ...
    default_traversal: skip   # Skip unless explicitly requested
```

**Step 3: Run schema validate**

```bash
cd tools/novanet && cargo run -- schema validate
```

Expected: PASS with 0 errors

**Step 4: Commit**

```bash
git add packages/core/models/taxonomy.yaml
git commit -m "feat(schema): add v9.9 retrieval properties (traversal_depth, context_budget, default_traversal)

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 2: Add temperature_threshold to Semantic ArcKinds

**Files:**
- Modify: `packages/core/models/arc-classes/semantic/semantic-link.yaml`
- Modify: `packages/core/models/arc-classes/semantic/uses-concept.yaml`
- Modify: `packages/core/models/arc-classes/semantic/links-to.yaml`
- Modify: `packages/core/models/arc-classes/semantic/subtopic-of.yaml`
- Modify: `packages/core/models/arc-classes/semantic/fills-slot.yaml`
- Modify: `packages/core/models/arc-classes/semantic/clusters-topic.yaml`
- Modify: `packages/core/models/arc-classes/semantic/satisfies-intent.yaml`
- Modify: `packages/core/models/arc-classes/semantic/for-intent.yaml`
- Modify: `packages/core/models/arc-classes/semantic/maps-to-concept.yaml`
- Modify: `packages/core/models/arc-classes/semantic/mentions.yaml`
- Modify: `packages/core/models/arc-classes/semantic/covers.yaml`
- Modify: `packages/core/models/arc-classes/semantic/specializes.yaml`
- Modify: `packages/core/models/arc-classes/semantic/related-thing.yaml`

**Step 1: Add temperature_threshold property to each semantic arc**

For each file, add this property at the arc level:

```yaml
arc:
  name: SEMANTIC_LINK  # (varies per file)
  family: semantic
  # ... existing fields ...
  temperature_threshold: 0.3  # Min activation temperature for traversal
```

Thresholds by semantic strength:
- `semantic-link.yaml`: 0.3 (high-confidence)
- `uses-concept.yaml`: 0.0 (always follow)
- `links-to.yaml`: 0.4
- `subtopic-of.yaml`: 0.2 (strong hierarchy)
- `fills-slot.yaml`: 0.0 (always follow)
- `clusters-topic.yaml`: 0.3
- `satisfies-intent.yaml`: 0.2
- `for-intent.yaml`: 0.0 (always follow)
- `maps-to-concept.yaml`: 0.0 (always follow)
- `mentions.yaml`: 0.5 (weak)
- `covers.yaml`: 0.4
- `specializes.yaml`: 0.2 (strong hierarchy)
- `related-thing.yaml`: 0.5 (weak)

**Step 2: Validate schema**

```bash
cd tools/novanet && cargo run -- schema validate
```

Expected: PASS

**Step 3: Commit**

```bash
git add packages/core/models/arc-classes/semantic/
git commit -m "feat(schema): add temperature_threshold to semantic arcs (v9.9)

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 3: Update Rust Parser for Retrieval Properties

**Files:**
- Modify: `tools/novanet/src/parsers/taxonomy.rs`

**Step 1: Add retrieval types to taxonomy parser**

Find the `ArcFamily` struct and add:

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArcFamily {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub stroke_style: String,
    pub stroke_width: u8,
    pub arrow_style: String,
    pub llm_context: String,
    #[serde(default)]
    pub default_traversal: TraversalMode,  // NEW
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraversalMode {
    Eager,
    #[default]
    Lazy,
    Skip,
}
```

**Step 2: Add KindRetrievalDefaults struct**

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KindRetrievalDefaults {
    pub invariant: RetrievalParams,
    pub localized: RetrievalParams,
    pub knowledge: RetrievalParams,
    pub derived: RetrievalParams,
    pub job: RetrievalParams,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetrievalParams {
    pub traversal_depth: u8,
    pub context_budget: u32,
    pub token_estimate: u32,
}
```

**Step 3: Add to Taxonomy struct**

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Taxonomy {
    pub version: String,
    pub node_realms: Vec<NodeRealm>,
    pub node_traits: Vec<NodeTrait>,
    pub arc_families: Vec<ArcFamily>,
    pub arc_scopes: Vec<ArcScope>,
    pub arc_cardinalities: Vec<ArcCardinality>,
    pub terminal: TerminalPalette,
    #[serde(default)]
    pub kind_retrieval_defaults: Option<KindRetrievalDefaults>,  // NEW
}
```

**Step 4: Run tests**

```bash
cd tools/novanet && cargo test parsers::taxonomy
```

Expected: All tests pass

**Step 5: Commit**

```bash
git add tools/novanet/src/parsers/taxonomy.rs
git commit -m "feat(rust): parse v9.9 retrieval properties in taxonomy

- Add TraversalMode enum (eager/lazy/skip)
- Add KindRetrievalDefaults struct
- Add default_traversal to ArcFamily

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 4: Update Arc Schema Parser for temperature_threshold

**Files:**
- Modify: `tools/novanet/src/parsers/relations.rs` (or `arc_schema.rs`)

**Step 1: Find ArcKind struct and add field**

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArcKind {
    pub name: String,
    pub family: String,
    pub scope: String,
    pub source: String,
    pub target: String,
    pub cardinality: String,
    #[serde(default)]
    pub is_self_referential: bool,
    #[serde(default)]
    pub properties: Vec<ArcProperty>,
    pub llm_context: String,
    pub cypher_pattern: String,
    #[serde(default)]
    pub temperature_threshold: Option<f32>,  // NEW: v9.9
}
```

**Step 2: Run tests**

```bash
cd tools/novanet && cargo test parsers::relations
```

**Step 3: Commit**

```bash
git add tools/novanet/src/parsers/
git commit -m "feat(rust): parse temperature_threshold in arc-kind YAML

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 5: Update Organizing Generator for Neo4j Properties

**Files:**
- Modify: `tools/novanet/src/generators/organizing.rs`

**Step 1: Update ArcFamily Cypher generation**

Find the ArcFamily Cypher template and add:

```rust
// In the ArcFamily creation section:
fn generate_arc_family_cypher(family: &ArcFamily) -> String {
    format!(
        r#"MERGE (af:ArcFamily:Meta {{key: '{key}'}})
SET af.display_name = '{display_name}',
    af.color = '{color}',
    af.stroke_style = '{stroke_style}',
    af.stroke_width = {stroke_width},
    af.arrow_style = '{arrow_style}',
    af.default_traversal = '{default_traversal}',
    af.llm_context = $af_{key}_context;"#,
        key = family.key,
        display_name = family.display_name,
        color = family.color,
        stroke_style = family.stroke_style,
        stroke_width = family.stroke_width,
        arrow_style = family.arrow_style,
        default_traversal = family.default_traversal.as_str(),
    )
}
```

**Step 2: Run schema generate**

```bash
cd tools/novanet && cargo run -- schema generate --dry-run
```

Verify `00.5-taxonomy.cypher` includes `default_traversal` property.

**Step 3: Run tests**

```bash
cd tools/novanet && cargo test generators::organizing
```

**Step 4: Commit**

```bash
git add tools/novanet/src/generators/organizing.rs
git commit -m "feat(rust): generate default_traversal property in Neo4j

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 6: Update Edge Schema Generator for temperature_threshold

**Files:**
- Modify: `tools/novanet/src/generators/edge_schema.rs` (or `arc_schema.rs`)

**Step 1: Add temperature_threshold to ArcKind Cypher**

```rust
fn generate_arc_kind_cypher(arc: &ArcKind) -> String {
    let temp_line = arc.temperature_threshold
        .map(|t| format!("    ak.temperature_threshold = {},\n", t))
        .unwrap_or_default();

    format!(
        r#"MERGE (ak:ArcKind:Meta {{key: '{key}'}})
SET ak.family = '{family}',
    ak.scope = '{scope}',
    ak.source = '{source}',
    ak.target = '{target}',
    ak.cardinality = '{cardinality}',
{temp_line}    ak.llm_context = $ak_{key}_context;"#,
        // ... fields
    )
}
```

**Step 2: Run schema generate**

```bash
cd tools/novanet && cargo run -- schema generate
```

**Step 3: Run full test suite**

```bash
cd tools/novanet && cargo test
```

Expected: All 223+ tests pass

**Step 4: Commit**

```bash
git add tools/novanet/src/generators/
git commit -m "feat(rust): generate temperature_threshold in ArcKind Neo4j nodes

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 7: Regenerate and Seed

**Step 1: Regenerate all artifacts**

```bash
cd tools/novanet && cargo run -- schema generate
```

Expected output: 7 files generated (or 8 with new retrieval properties)

**Step 2: Validate coherence**

```bash
cd tools/novanet && cargo run -- schema validate
```

Expected: 0 errors, 0 warnings

**Step 3: Seed database**

```bash
cd /Users/thibaut/supernovae-st/novanet-hq
pnpm infra:seed
```

**Step 4: Verify properties in Neo4j**

```bash
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (af:ArcFamily) RETURN af.key, af.default_traversal LIMIT 5"
```

Expected: `ownership | eager`, `localization | eager`, etc.

```bash
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (ak:ArcKind) WHERE ak.temperature_threshold IS NOT NULL RETURN ak.key, ak.temperature_threshold LIMIT 5"
```

Expected: Semantic arcs with threshold values

**Step 5: Commit seed files**

```bash
git add packages/db/seed/
git commit -m "chore(seed): regenerate Neo4j seeds with v9.9 retrieval properties

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Phase C: v10 Context Assembly Engine

### Task 8: Create Retrieval Module Structure

**Files:**
- Create: `tools/novanet/src/retrieval/mod.rs`
- Create: `tools/novanet/src/retrieval/types.rs`
- Modify: `tools/novanet/src/lib.rs`

**Step 1: Create retrieval directory**

```bash
mkdir -p tools/novanet/src/retrieval
```

**Step 2: Create mod.rs**

```rust
// tools/novanet/src/retrieval/mod.rs
//! v10 Context Assembly Engine
//!
//! Data-driven context window assembly using meta-graph traversal rules.

pub mod types;
pub mod meta;
pub mod planner;
pub mod engine;

pub use types::*;
pub use meta::MetaGraphReader;
pub use planner::TraversalPlanner;
pub use engine::ContextEngine;
```

**Step 3: Create types.rs with core types**

```rust
// tools/novanet/src/retrieval/types.rs
//! Core types for context assembly.

use serde::{Deserialize, Serialize};

/// Request for context assembly
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContextRequest {
    /// Target block key
    pub block_key: String,
    /// Target locale key
    pub locale_key: String,
    /// Maximum token budget
    pub token_budget: u32,
    /// Optional temperature for semantic arcs (default: 0.3)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    /// Optional max traversal depth override
    pub max_depth: Option<u8>,
}

fn default_temperature() -> f32 { 0.3 }

/// Assembled context window
#[derive(Debug, Clone, Serialize)]
pub struct ContextWindow {
    /// Nodes included in context
    pub nodes: Vec<ContextNode>,
    /// Edges included in context (for provenance)
    pub edges: Vec<ContextEdge>,
    /// Total tokens used
    pub tokens_used: u32,
    /// Token budget
    pub token_budget: u32,
    /// Traversal log for debugging
    pub traversal_log: Vec<TraversalStep>,
}

/// Node in context window
#[derive(Debug, Clone, Serialize)]
pub struct ContextNode {
    pub key: String,
    pub kind: String,
    pub realm: String,
    pub layer: String,
    pub trait_type: String,
    pub properties: serde_json::Value,
    pub token_estimate: u32,
    pub depth: u8,
}

/// Edge in context window
#[derive(Debug, Clone, Serialize)]
pub struct ContextEdge {
    pub from_key: String,
    pub to_key: String,
    pub arc_kind: String,
    pub family: String,
    pub properties: serde_json::Value,
}

/// Traversal step for debugging
#[derive(Debug, Clone, Serialize)]
pub struct TraversalStep {
    pub depth: u8,
    pub from_key: String,
    pub arc_kind: String,
    pub to_key: String,
    pub decision: TraversalDecision,
    pub reason: String,
}

/// Decision made during traversal
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraversalDecision {
    Include,
    Skip,
    BudgetExceeded,
    DepthExceeded,
    ThresholdNotMet,
}

/// Traversal mode from ArcFamily
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraversalMode {
    Eager,
    #[default]
    Lazy,
    Skip,
}

impl TraversalMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eager => "eager",
            Self::Lazy => "lazy",
            Self::Skip => "skip",
        }
    }
}
```

**Step 4: Update lib.rs to export retrieval module**

Add to `tools/novanet/src/lib.rs`:

```rust
pub mod retrieval;
```

**Step 5: Run cargo check**

```bash
cd tools/novanet && cargo check
```

Expected: No errors (warnings OK at this stage)

**Step 6: Commit**

```bash
git add tools/novanet/src/retrieval/ tools/novanet/src/lib.rs
git commit -m "feat(rust): add retrieval module structure with core types (v10)

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 9: Implement MetaGraphReader

**Files:**
- Create: `tools/novanet/src/retrieval/meta.rs`

**Step 1: Create MetaGraphReader**

```rust
// tools/novanet/src/retrieval/meta.rs
//! Meta-graph reader for traversal rules.

use crate::error::Result;
use neo4rs::Graph;
use std::collections::HashMap;
use std::sync::Arc;

use super::types::TraversalMode;

/// Rules for a Kind (node type)
#[derive(Debug, Clone)]
pub struct KindRules {
    pub key: String,
    pub trait_type: String,
    pub traversal_depth: u8,
    pub context_budget: u32,
    pub token_estimate: u32,
}

/// Rules for an ArcFamily
#[derive(Debug, Clone)]
pub struct ArcFamilyRules {
    pub key: String,
    pub default_traversal: TraversalMode,
}

/// Rules for an ArcKind
#[derive(Debug, Clone)]
pub struct ArcKindRules {
    pub key: String,
    pub family: String,
    pub source: String,
    pub target: String,
    pub temperature_threshold: Option<f32>,
}

/// Reads traversal rules from Neo4j meta-graph
pub struct MetaGraphReader {
    graph: Arc<Graph>,
    kind_rules: HashMap<String, KindRules>,
    family_rules: HashMap<String, ArcFamilyRules>,
    arc_rules: HashMap<String, ArcKindRules>,
}

impl MetaGraphReader {
    /// Create a new reader and load rules from Neo4j
    pub async fn new(graph: Arc<Graph>) -> Result<Self> {
        let mut reader = Self {
            graph,
            kind_rules: HashMap::new(),
            family_rules: HashMap::new(),
            arc_rules: HashMap::new(),
        };
        reader.load_rules().await?;
        Ok(reader)
    }

    /// Load all rules from meta-graph
    async fn load_rules(&mut self) -> Result<()> {
        self.load_kind_rules().await?;
        self.load_family_rules().await?;
        self.load_arc_rules().await?;
        Ok(())
    }

    async fn load_kind_rules(&mut self) -> Result<()> {
        let query = r#"
            MATCH (k:Kind:Meta)-[:HAS_TRAIT]->(t:Trait:Meta)
            RETURN k.key AS key, t.key AS trait_type,
                   coalesce(k.traversal_depth, 2) AS traversal_depth,
                   coalesce(k.context_budget, 500) AS context_budget,
                   coalesce(k.token_estimate, 100) AS token_estimate
        "#;

        let mut result = self.graph.execute(neo4rs::query(query)).await?;
        while let Some(row) = result.next().await? {
            let rules = KindRules {
                key: row.get("key")?,
                trait_type: row.get("trait_type")?,
                traversal_depth: row.get::<i64>("traversal_depth")? as u8,
                context_budget: row.get::<i64>("context_budget")? as u32,
                token_estimate: row.get::<i64>("token_estimate")? as u32,
            };
            self.kind_rules.insert(rules.key.clone(), rules);
        }
        Ok(())
    }

    async fn load_family_rules(&mut self) -> Result<()> {
        let query = r#"
            MATCH (af:ArcFamily:Meta)
            RETURN af.key AS key,
                   coalesce(af.default_traversal, 'lazy') AS default_traversal
        "#;

        let mut result = self.graph.execute(neo4rs::query(query)).await?;
        while let Some(row) = result.next().await? {
            let traversal_str: String = row.get("default_traversal")?;
            let rules = ArcFamilyRules {
                key: row.get("key")?,
                default_traversal: match traversal_str.as_str() {
                    "eager" => TraversalMode::Eager,
                    "skip" => TraversalMode::Skip,
                    _ => TraversalMode::Lazy,
                },
            };
            self.family_rules.insert(rules.key.clone(), rules);
        }
        Ok(())
    }

    async fn load_arc_rules(&mut self) -> Result<()> {
        let query = r#"
            MATCH (ak:ArcKind:Meta)-[:IN_FAMILY]->(af:ArcFamily:Meta)
            MATCH (ak)-[:FROM_KIND]->(source:Kind:Meta)
            MATCH (ak)-[:TO_KIND]->(target:Kind:Meta)
            RETURN ak.key AS key, af.key AS family,
                   source.key AS source, target.key AS target,
                   ak.temperature_threshold AS temperature_threshold
        "#;

        let mut result = self.graph.execute(neo4rs::query(query)).await?;
        while let Some(row) = result.next().await? {
            let rules = ArcKindRules {
                key: row.get("key")?,
                family: row.get("family")?,
                source: row.get("source")?,
                target: row.get("target")?,
                temperature_threshold: row.get::<Option<f64>>("temperature_threshold")?
                    .map(|v| v as f32),
            };
            self.arc_rules.insert(rules.key.clone(), rules);
        }
        Ok(())
    }

    /// Get rules for a Kind
    pub fn get_kind_rules(&self, key: &str) -> Option<&KindRules> {
        self.kind_rules.get(key)
    }

    /// Get rules for an ArcFamily
    pub fn get_family_rules(&self, key: &str) -> Option<&ArcFamilyRules> {
        self.family_rules.get(key)
    }

    /// Get rules for an ArcKind
    pub fn get_arc_rules(&self, key: &str) -> Option<&ArcKindRules> {
        self.arc_rules.get(key)
    }

    /// Check if an arc should be traversed given current temperature
    pub fn should_traverse_arc(&self, arc_key: &str, temperature: f32) -> bool {
        if let Some(rules) = self.arc_rules.get(arc_key) {
            // Check family default
            if let Some(family_rules) = self.family_rules.get(&rules.family) {
                if family_rules.default_traversal == TraversalMode::Skip {
                    return false;
                }
            }
            // Check temperature threshold
            if let Some(threshold) = rules.temperature_threshold {
                return temperature >= threshold;
            }
        }
        true // Default: traverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traversal_mode_as_str() {
        assert_eq!(TraversalMode::Eager.as_str(), "eager");
        assert_eq!(TraversalMode::Lazy.as_str(), "lazy");
        assert_eq!(TraversalMode::Skip.as_str(), "skip");
    }
}
```

**Step 2: Run cargo check**

```bash
cd tools/novanet && cargo check
```

**Step 3: Commit**

```bash
git add tools/novanet/src/retrieval/meta.rs
git commit -m "feat(rust): implement MetaGraphReader for traversal rules (v10)

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 10: Implement TraversalPlanner

**Files:**
- Create: `tools/novanet/src/retrieval/planner.rs`

**Step 1: Create TraversalPlanner**

```rust
// tools/novanet/src/retrieval/planner.rs
//! Traversal planner builds execution plan from meta-graph rules.

use crate::error::Result;
use super::meta::MetaGraphReader;
use super::types::{ContextRequest, TraversalMode};

/// A step in the traversal plan
#[derive(Debug, Clone)]
pub struct PlanStep {
    pub arc_kind: String,
    pub family: String,
    pub target_kind: String,
    pub priority: TraversalPriority,
    pub max_depth: u8,
    pub temperature_threshold: Option<f32>,
}

/// Priority determines traversal order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraversalPriority {
    High = 0,   // Eager arcs (ownership, localization)
    Medium = 1, // Lazy arcs with high threshold
    Low = 2,    // Lazy arcs with low threshold
}

/// Builds traversal plans from meta-graph rules
pub struct TraversalPlanner<'a> {
    meta: &'a MetaGraphReader,
}

impl<'a> TraversalPlanner<'a> {
    pub fn new(meta: &'a MetaGraphReader) -> Self {
        Self { meta }
    }

    /// Build traversal plan for a given Kind
    pub fn plan_for_kind(&self, kind: &str, request: &ContextRequest) -> Vec<PlanStep> {
        let mut steps = Vec::new();

        // Get Kind rules for max depth
        let kind_rules = self.meta.get_kind_rules(kind);
        let max_depth = request.max_depth
            .unwrap_or_else(|| kind_rules.map(|r| r.traversal_depth).unwrap_or(2));

        // Find all arcs FROM this kind
        // (In real implementation, query meta-graph for FROM_KIND relationships)
        // For now, use common patterns
        let arc_patterns = self.get_outgoing_arcs_for_kind(kind);

        for (arc_kind, family, target) in arc_patterns {
            let family_rules = self.meta.get_family_rules(&family);
            let arc_rules = self.meta.get_arc_rules(&arc_kind);

            let priority = match family_rules.map(|r| r.default_traversal) {
                Some(TraversalMode::Eager) => TraversalPriority::High,
                Some(TraversalMode::Skip) => continue, // Skip this arc
                _ => {
                    // Lazy: priority based on temperature threshold
                    match arc_rules.and_then(|r| r.temperature_threshold) {
                        Some(t) if t <= 0.3 => TraversalPriority::Medium,
                        _ => TraversalPriority::Low,
                    }
                }
            };

            steps.push(PlanStep {
                arc_kind: arc_kind.clone(),
                family,
                target_kind: target,
                priority,
                max_depth,
                temperature_threshold: arc_rules.and_then(|r| r.temperature_threshold),
            });
        }

        // Sort by priority (eager first)
        steps.sort_by_key(|s| s.priority);
        steps
    }

    /// Get outgoing arcs for a kind (hardcoded patterns for common kinds)
    fn get_outgoing_arcs_for_kind(&self, kind: &str) -> Vec<(String, String, String)> {
        // This would ideally query the meta-graph, but for initial implementation
        // we use known patterns
        match kind {
            "Block" => vec![
                ("USES_CONCEPT".into(), "semantic".into(), "Concept".into()),
                ("OF_TYPE".into(), "ownership".into(), "BlockType".into()),
                ("HAS_PROMPT".into(), "ownership".into(), "BlockPrompt".into()),
                ("HAS_RULES".into(), "ownership".into(), "BlockRules".into()),
            ],
            "Concept" => vec![
                ("HAS_L10N".into(), "localization".into(), "ConceptL10n".into()),
                ("SEMANTIC_LINK".into(), "semantic".into(), "Concept".into()),
            ],
            "ConceptL10n" => vec![
                ("FOR_LOCALE".into(), "localization".into(), "Locale".into()),
            ],
            "Locale" => vec![
                ("HAS_VOICE".into(), "localization".into(), "LocaleVoice".into()),
                ("HAS_CULTURE".into(), "localization".into(), "LocaleCulture".into()),
                ("HAS_LEXICON".into(), "localization".into(), "LocaleLexicon".into()),
            ],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(TraversalPriority::High < TraversalPriority::Medium);
        assert!(TraversalPriority::Medium < TraversalPriority::Low);
    }
}
```

**Step 2: Commit**

```bash
git add tools/novanet/src/retrieval/planner.rs
git commit -m "feat(rust): implement TraversalPlanner for v10 context assembly

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 11: Implement ContextEngine

**Files:**
- Create: `tools/novanet/src/retrieval/engine.rs`

**Step 1: Create ContextEngine**

```rust
// tools/novanet/src/retrieval/engine.rs
//! Context assembly engine executes traversal plans.

use crate::error::Result;
use neo4rs::{Graph, query};
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

use super::meta::MetaGraphReader;
use super::planner::{TraversalPlanner, PlanStep};
use super::types::*;

/// Context assembly engine
pub struct ContextEngine {
    graph: Arc<Graph>,
    meta: MetaGraphReader,
}

impl ContextEngine {
    /// Create a new engine
    pub async fn new(graph: Arc<Graph>) -> Result<Self> {
        let meta = MetaGraphReader::new(graph.clone()).await?;
        Ok(Self { graph, meta })
    }

    /// Assemble context window for a block + locale
    pub async fn assemble(&self, request: &ContextRequest) -> Result<ContextWindow> {
        let mut window = ContextWindow {
            nodes: Vec::new(),
            edges: Vec::new(),
            tokens_used: 0,
            token_budget: request.token_budget,
            traversal_log: Vec::new(),
        };

        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(String, String, u8)> = VecDeque::new(); // (key, kind, depth)

        // Start with the block
        let block = self.fetch_node(&request.block_key).await?;
        if let Some(node) = block {
            window.tokens_used += node.token_estimate;
            visited.insert(node.key.clone());
            queue.push_back((node.key.clone(), node.kind.clone(), 0));
            window.nodes.push(node);
        }

        // BFS traversal
        while let Some((current_key, current_kind, depth)) = queue.pop_front() {
            // Get traversal plan for this kind
            let planner = TraversalPlanner::new(&self.meta);
            let plan = planner.plan_for_kind(&current_kind, request);

            for step in plan {
                // Check depth
                if depth >= step.max_depth {
                    window.traversal_log.push(TraversalStep {
                        depth,
                        from_key: current_key.clone(),
                        arc_kind: step.arc_kind.clone(),
                        to_key: "?".into(),
                        decision: TraversalDecision::DepthExceeded,
                        reason: format!("depth {} >= max {}", depth, step.max_depth),
                    });
                    continue;
                }

                // Check temperature threshold
                if let Some(threshold) = step.temperature_threshold {
                    if request.temperature < threshold {
                        window.traversal_log.push(TraversalStep {
                            depth,
                            from_key: current_key.clone(),
                            arc_kind: step.arc_kind.clone(),
                            to_key: "?".into(),
                            decision: TraversalDecision::ThresholdNotMet,
                            reason: format!("temp {} < threshold {}", request.temperature, threshold),
                        });
                        continue;
                    }
                }

                // Fetch connected nodes
                let neighbors = self.fetch_neighbors(
                    &current_key,
                    &step.arc_kind,
                    &request.locale_key,
                ).await?;

                for (neighbor, edge) in neighbors {
                    if visited.contains(&neighbor.key) {
                        continue;
                    }

                    // Check budget
                    if window.tokens_used + neighbor.token_estimate > request.token_budget {
                        window.traversal_log.push(TraversalStep {
                            depth: depth + 1,
                            from_key: current_key.clone(),
                            arc_kind: step.arc_kind.clone(),
                            to_key: neighbor.key.clone(),
                            decision: TraversalDecision::BudgetExceeded,
                            reason: format!(
                                "tokens {} + {} > budget {}",
                                window.tokens_used,
                                neighbor.token_estimate,
                                request.token_budget
                            ),
                        });
                        continue;
                    }

                    // Include node
                    window.traversal_log.push(TraversalStep {
                        depth: depth + 1,
                        from_key: current_key.clone(),
                        arc_kind: step.arc_kind.clone(),
                        to_key: neighbor.key.clone(),
                        decision: TraversalDecision::Include,
                        reason: "within budget and depth".into(),
                    });

                    window.tokens_used += neighbor.token_estimate;
                    visited.insert(neighbor.key.clone());
                    queue.push_back((neighbor.key.clone(), neighbor.kind.clone(), depth + 1));
                    window.nodes.push(neighbor);
                    window.edges.push(edge);
                }
            }
        }

        Ok(window)
    }

    /// Fetch a single node by key
    async fn fetch_node(&self, key: &str) -> Result<Option<ContextNode>> {
        let q = query(r#"
            MATCH (n {key: $key})
            OPTIONAL MATCH (n)-[:OF_KIND]->(k:Kind:Meta)-[:HAS_TRAIT]->(t:Trait:Meta)
            OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer:Meta)-[:IN_REALM]->(r:Realm:Meta)
            RETURN n, labels(n)[0] AS kind,
                   r.key AS realm, l.key AS layer, t.key AS trait_type,
                   coalesce(k.token_estimate, 100) AS token_estimate
        "#).param("key", key);

        let mut result = self.graph.execute(q).await?;
        if let Some(row) = result.next().await? {
            let node: neo4rs::Node = row.get("n")?;
            Ok(Some(ContextNode {
                key: key.to_string(),
                kind: row.get("kind").unwrap_or_default(),
                realm: row.get("realm").unwrap_or_default(),
                layer: row.get("layer").unwrap_or_default(),
                trait_type: row.get("trait_type").unwrap_or_default(),
                properties: serde_json::to_value(node.properties()).unwrap_or_default(),
                token_estimate: row.get::<i64>("token_estimate").unwrap_or(100) as u32,
                depth: 0,
            }))
        } else {
            Ok(None)
        }
    }

    /// Fetch neighbors via a specific arc type
    async fn fetch_neighbors(
        &self,
        from_key: &str,
        arc_kind: &str,
        locale_key: &str,
    ) -> Result<Vec<(ContextNode, ContextEdge)>> {
        let q = query(&format!(r#"
            MATCH (from {{key: $from_key}})-[r:{arc_kind}]->(to)
            WHERE NOT to:Meta
            OPTIONAL MATCH (to)-[:OF_KIND]->(k:Kind:Meta)-[:HAS_TRAIT]->(t:Trait:Meta)
            OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer:Meta)-[:IN_REALM]->(realm:Realm:Meta)
            OPTIONAL MATCH (r)-[:OF_ARC_KIND]->(ak:ArcKind:Meta)-[:IN_FAMILY]->(af:ArcFamily:Meta)
            RETURN to, labels(to)[0] AS kind,
                   realm.key AS realm, l.key AS layer, t.key AS trait_type,
                   coalesce(k.token_estimate, 100) AS token_estimate,
                   r, type(r) AS rel_type, af.key AS family
        "#, arc_kind = arc_kind))
        .param("from_key", from_key)
        .param("locale_key", locale_key);

        let mut result = self.graph.execute(q).await?;
        let mut neighbors = Vec::new();

        while let Some(row) = result.next().await? {
            let to_node: neo4rs::Node = row.get("to")?;
            let rel: neo4rs::Relation = row.get("r")?;

            let node = ContextNode {
                key: to_node.get::<String>("key").unwrap_or_default(),
                kind: row.get("kind").unwrap_or_default(),
                realm: row.get("realm").unwrap_or_default(),
                layer: row.get("layer").unwrap_or_default(),
                trait_type: row.get("trait_type").unwrap_or_default(),
                properties: serde_json::to_value(to_node.properties()).unwrap_or_default(),
                token_estimate: row.get::<i64>("token_estimate").unwrap_or(100) as u32,
                depth: 0, // Will be set by caller
            };

            let edge = ContextEdge {
                from_key: from_key.to_string(),
                to_key: node.key.clone(),
                arc_kind: row.get::<String>("rel_type").unwrap_or_default(),
                family: row.get::<String>("family").unwrap_or_default(),
                properties: serde_json::to_value(rel.properties()).unwrap_or_default(),
            };

            neighbors.push((node, edge));
        }

        Ok(neighbors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_window_default() {
        let window = ContextWindow {
            nodes: vec![],
            edges: vec![],
            tokens_used: 0,
            token_budget: 1000,
            traversal_log: vec![],
        };
        assert_eq!(window.token_budget, 1000);
    }
}
```

**Step 2: Commit**

```bash
git add tools/novanet/src/retrieval/engine.rs
git commit -m "feat(rust): implement ContextEngine for v10 context assembly

- BFS traversal with budget tracking
- Temperature-based semantic arc filtering
- Depth limiting per Kind rules
- Full provenance in traversal_log

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 12: Add context CLI Command

**Files:**
- Create: `tools/novanet/src/commands/context.rs`
- Modify: `tools/novanet/src/commands/mod.rs`
- Modify: `tools/novanet/src/main.rs`

**Step 1: Create context command**

```rust
// tools/novanet/src/commands/context.rs
//! `novanet context` command — assemble context window for a block.

use crate::db::connect;
use crate::error::Result;
use crate::output::OutputFormat;
use crate::retrieval::{ContextEngine, ContextRequest};
use clap::Args;
use std::sync::Arc;

#[derive(Args, Debug)]
pub struct ContextArgs {
    /// Block key to assemble context for
    #[arg(long)]
    pub block: String,

    /// Locale key
    #[arg(long)]
    pub locale: String,

    /// Token budget (default: 4000)
    #[arg(long, default_value = "4000")]
    pub budget: u32,

    /// Temperature for semantic arcs (default: 0.3)
    #[arg(long, default_value = "0.3")]
    pub temperature: f32,

    /// Max traversal depth override
    #[arg(long)]
    pub max_depth: Option<u8>,

    /// Output format
    #[arg(long, default_value = "json")]
    pub format: OutputFormat,

    /// Show traversal log
    #[arg(long)]
    pub debug: bool,
}

pub async fn run(args: ContextArgs) -> Result<()> {
    let graph = Arc::new(connect().await?);
    let engine = ContextEngine::new(graph).await?;

    let request = ContextRequest {
        block_key: args.block,
        locale_key: args.locale,
        token_budget: args.budget,
        temperature: args.temperature,
        max_depth: args.max_depth,
    };

    let window = engine.assemble(&request).await?;

    match args.format {
        OutputFormat::Json => {
            if args.debug {
                println!("{}", serde_json::to_string_pretty(&window)?);
            } else {
                // Without debug, omit traversal_log
                let output = serde_json::json!({
                    "nodes": window.nodes,
                    "edges": window.edges,
                    "tokens_used": window.tokens_used,
                    "token_budget": window.token_budget,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
        }
        OutputFormat::Table => {
            println!("Context Window for block '{}' @ locale '{}'",
                     request.block_key, request.locale_key);
            println!("─".repeat(60));
            println!("Tokens: {}/{}", window.tokens_used, window.token_budget);
            println!("Nodes:  {}", window.nodes.len());
            println!("Edges:  {}", window.edges.len());
            println!();

            println!("Included Nodes:");
            for node in &window.nodes {
                println!("  [{}] {} ({})", node.kind, node.key, node.token_estimate);
            }

            if args.debug {
                println!();
                println!("Traversal Log:");
                for step in &window.traversal_log {
                    println!("  d{}: {} -[{}]-> {} ({:?})",
                             step.depth, step.from_key, step.arc_kind,
                             step.to_key, step.decision);
                }
            }
        }
        OutputFormat::Cypher => {
            // Output Cypher to recreate context
            println!("// Context window for {} @ {}", request.block_key, request.locale_key);
            let keys: Vec<_> = window.nodes.iter().map(|n| format!("'{}'", n.key)).collect();
            println!("MATCH (n) WHERE n.key IN [{}] RETURN n", keys.join(", "));
        }
    }

    Ok(())
}
```

**Step 2: Register in commands/mod.rs**

Add to `tools/novanet/src/commands/mod.rs`:

```rust
pub mod context;
```

**Step 3: Register in main.rs**

Add to the `Commands` enum in `main.rs`:

```rust
/// Assemble context window for a block
Context(commands::context::ContextArgs),
```

And in the match:

```rust
Commands::Context(args) => commands::context::run(args).await,
```

**Step 4: Run cargo check**

```bash
cd tools/novanet && cargo check
```

**Step 5: Commit**

```bash
git add tools/novanet/src/commands/context.rs tools/novanet/src/commands/mod.rs tools/novanet/src/main.rs
git commit -m "feat(rust): add 'novanet context' command for v10 context assembly

Usage: novanet context --block=hero-pricing --locale=fr-FR --budget=4000

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 13: Add Integration Tests

**Files:**
- Create: `tools/novanet/tests/retrieval_tests.rs`

**Step 1: Create test file**

```rust
// tools/novanet/tests/retrieval_tests.rs
//! Integration tests for context retrieval (requires running Neo4j)

use novanet::retrieval::types::*;

#[test]
fn test_context_request_defaults() {
    let json = r#"{"block_key": "hero", "locale_key": "fr-FR", "token_budget": 4000}"#;
    let request: ContextRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.block_key, "hero");
    assert_eq!(request.locale_key, "fr-FR");
    assert_eq!(request.token_budget, 4000);
    assert_eq!(request.temperature, 0.3); // default
    assert!(request.max_depth.is_none());
}

#[test]
fn test_traversal_decision_serialization() {
    assert_eq!(
        serde_json::to_string(&TraversalDecision::Include).unwrap(),
        "\"include\""
    );
    assert_eq!(
        serde_json::to_string(&TraversalDecision::BudgetExceeded).unwrap(),
        "\"budgetexceeded\""
    );
}

#[test]
fn test_context_window_serialization() {
    let window = ContextWindow {
        nodes: vec![ContextNode {
            key: "test".into(),
            kind: "Block".into(),
            realm: "project".into(),
            layer: "structure".into(),
            trait_type: "invariant".into(),
            properties: serde_json::json!({}),
            token_estimate: 100,
            depth: 0,
        }],
        edges: vec![],
        tokens_used: 100,
        token_budget: 4000,
        traversal_log: vec![],
    };

    let json = serde_json::to_string(&window).unwrap();
    assert!(json.contains("\"tokens_used\":100"));
    assert!(json.contains("\"kind\":\"Block\""));
}

// Integration tests (require Neo4j)
#[tokio::test]
#[ignore] // Run with: cargo test -- --ignored
async fn test_meta_graph_reader() {
    use novanet::db::connect;
    use novanet::retrieval::MetaGraphReader;
    use std::sync::Arc;

    let graph = Arc::new(connect().await.expect("Neo4j connection"));
    let reader = MetaGraphReader::new(graph).await.expect("MetaGraphReader");

    // Should have loaded Kind rules
    assert!(reader.get_kind_rules("Block").is_some());
    assert!(reader.get_kind_rules("Concept").is_some());

    // Should have loaded ArcFamily rules
    let ownership = reader.get_family_rules("ownership");
    assert!(ownership.is_some());
    assert_eq!(ownership.unwrap().default_traversal, novanet::retrieval::TraversalMode::Eager);
}

#[tokio::test]
#[ignore]
async fn test_context_assembly() {
    use novanet::db::connect;
    use novanet::retrieval::{ContextEngine, ContextRequest};
    use std::sync::Arc;

    let graph = Arc::new(connect().await.expect("Neo4j connection"));
    let engine = ContextEngine::new(graph).await.expect("ContextEngine");

    let request = ContextRequest {
        block_key: "hero-pricing".into(), // Assumes this exists in seed
        locale_key: "fr-FR".into(),
        token_budget: 4000,
        temperature: 0.3,
        max_depth: Some(2),
    };

    let window = engine.assemble(&request).await.expect("assembly");

    // Should have at least the block itself
    assert!(!window.nodes.is_empty());
    assert!(window.tokens_used <= window.token_budget);
}
```

**Step 2: Run unit tests**

```bash
cd tools/novanet && cargo test retrieval
```

**Step 3: Run integration tests (with Neo4j)**

```bash
cd tools/novanet && cargo test -- --ignored
```

**Step 4: Commit**

```bash
git add tools/novanet/tests/
git commit -m "test(rust): add retrieval integration tests for v10 context engine

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 14: Final Verification

**Step 1: Run full test suite**

```bash
cd tools/novanet && cargo test
```

Expected: All tests pass (230+ tests)

**Step 2: Run clippy**

```bash
cd tools/novanet && cargo clippy -- -D warnings
```

Expected: 0 warnings

**Step 3: Test context command**

```bash
cd tools/novanet && cargo run -- context --block=hero-pricing --locale=fr-FR --budget=4000 --debug
```

Expected: JSON output with nodes, edges, traversal_log

**Step 4: Update CLAUDE.md**

Add to the Commands table in `tools/novanet/CLAUDE.md`:

```markdown
| Context | `context --block=... --locale=... [--budget=4000]` | v10: Assemble context window |
```

**Step 5: Final commit**

```bash
git add -A
git commit -m "feat(v10): complete Phase 9.9 + Phase C context assembly engine

Phase 9.9:
- Add retrieval properties to taxonomy.yaml (traversal_depth, context_budget, default_traversal)
- Add temperature_threshold to semantic ArcKinds
- Update Rust parsers and generators

Phase C (v10 Context Assembly):
- Create retrieval module (types, meta, planner, engine)
- Implement MetaGraphReader for meta-graph rules
- Implement TraversalPlanner with priority ordering
- Implement ContextEngine with BFS traversal and budget tracking
- Add 'novanet context' CLI command
- Add integration tests

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Success Criteria

1. **Phase 9.9 Complete:**
   - [ ] taxonomy.yaml has `kind_retrieval_defaults` section
   - [ ] ArcFamilies have `default_traversal` property
   - [ ] Semantic ArcKinds have `temperature_threshold`
   - [ ] `cargo run -- schema validate` passes
   - [ ] Neo4j seeds regenerated with new properties

2. **Phase C Complete:**
   - [ ] `retrieval/` module with 4 files (types, meta, planner, engine)
   - [ ] `novanet context` command works
   - [ ] 230+ tests pass
   - [ ] 0 clippy warnings
   - [ ] Integration tests pass with Neo4j

3. **Verification:**
   - [ ] `novanet context --block=hero-pricing --locale=fr-FR` returns valid JSON
   - [ ] Traversal log shows budget and depth decisions
   - [ ] Token counts match estimates

---

## Estimated Time

| Task | Time |
|------|------|
| Task 1-7 (Phase 9.9) | 2-3 hours |
| Task 8-12 (Phase C) | 3-4 hours |
| Task 13-14 (Tests + Verification) | 1 hour |
| **Total** | **6-8 hours** |
