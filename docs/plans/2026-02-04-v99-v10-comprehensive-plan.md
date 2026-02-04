# NovaNet v9.9-v10.0 Comprehensive Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Finalize v9.x schema, update all DX documentation, and prepare detailed v10 Dynamic Retrieval implementation.

**Architecture:** Three-phase approach: (1) Pre-v10 cleanup & verification, (2) DX documentation overhaul, (3) v10 Context Assembly Engine foundation.

**Tech Stack:** Rust (neo4rs, tracing, ratatui), TypeScript (Next.js 16, React 19), Neo4j 5.26

---

## Executive Summary

| Phase | Tasks | Priority | Status |
|-------|-------|----------|--------|
| **A** | Pre-v10 Verification | P0 | Planning |
| **B** | DX Documentation Update | P1 | Planning |
| **C** | v10 Context Assembly Engine | P2 | Planning |

**Current State (v9.8.0):**
- 46 NodeKinds (includes v9.6-9.8 additions)
- 75 ArcKinds (individual YAML files)
- Tracing instrumentation: DONE
- TUI YAML caching: DONE
- Parallel YAML loading: DONE (rayon)
- FxHashSet/SmallVec optimizations: DONE

---

## Phase A: Pre-v10 Verification

### Task A.1: Schema Count Verification

**Step 1: Count current schema**

```bash
# NodeKinds
find packages/core/models/node-kinds -name "*.yaml" | wc -l
# Expected: 46

# ArcKinds
find packages/core/models/arc-kinds -name "*.yaml" | grep -v "_index" | wc -l
# Expected: 77 (per v9.7 plan)
```

**Step 2: Verify v9.6-9.8 additions exist**

| NodeKind | Layer | Status |
|----------|-------|--------|
| GenerationJob | project/output | ✅ EXISTS |
| PromptArtifact | project/instruction | ✅ EXISTS |
| OutputArtifact | project/output | ✅ EXISTS |
| EvaluationSignal | project/output | ✅ EXISTS |
| ContentSlot | project/structure | ✅ EXISTS |
| TopicCluster | project/semantic | ✅ EXISTS |
| SearchIntent | project/semantic | ✅ EXISTS |
| Thing | shared/geo | ✅ EXISTS |
| ThingL10n | shared/geo | ✅ EXISTS |

**Step 3: Check for missing ArcKinds**

Compare arc-kinds/ count (75) vs relations.yaml expected (77).

```bash
cargo run -- schema validate
```

---

### Task A.2: Run Full Test Suite

**Step 1: Rust tests**

```bash
cd tools/novanet && cargo test --quiet
```

Expected: 220+ tests pass.

**Step 2: TypeScript tests**

```bash
pnpm test
```

Expected: 500+ tests pass.

**Step 3: Clippy + type-check**

```bash
cd tools/novanet && cargo clippy -- -D warnings
pnpm type-check
```

Expected: Zero warnings/errors.

---

## Phase B: DX Documentation Update

### Task B.1: Update .claude/README.md

**File:** `.claude/README.md`

**Step 1: Update version banner**

```markdown
# Before (line 5):
**Version**: v9.0.1 | **Docs**: [Claude Code Official](https://docs.anthropic.com/en/docs/claude-code)

# After:
**Version**: v9.8.0 | **Docs**: [Claude Code Official](https://docs.anthropic.com/en/docs/claude-code)
```

**Step 2: Update quick reference box (line 13)**

```
║                              NOVANET DX - v9.8.0                                                  ║
```

**Step 3: Update hook output (line 118)**

```
NovaNet v9.8.0 | Branch: main | Uncommitted: 3 files
```

---

### Task B.2: Update Root CLAUDE.md

**File:** `CLAUDE.md`

**Step 1: Update version**

Find: `**Current Version**: v9.0.1`
Replace: `**Current Version**: v9.8.0`

**Step 2: Update Kind counts**

Find: `35 node types`
Replace: `46 node types`

Find any `76 ArcKinds` references
Replace: `77 ArcKinds`

---

### Task B.3: Update tools/novanet/CLAUDE.md

**File:** `tools/novanet/CLAUDE.md`

**Step 1: Update test count**

Find: `223 tests pass`
Replace with actual count from `cargo test`

**Step 2: Verify command table is accurate**

All commands should match actual CLI:
- `data`, `meta`, `overlay`, `query` ✅
- `node create/edit/delete` ✅
- `arc create/delete` ✅
- `schema generate/validate` ✅
- `db seed/migrate/reset` ✅
- `tui` ✅

---

### Task B.4: Update packages/core/CLAUDE.md

**File:** `packages/core/CLAUDE.md`

**Step 1: Update v9 migration context**

The table showing v8 → v9 terminology should reflect final counts:
- `**Kind** | 46 node types` (was 35)
- `**ArcKind** | 77 relationship types` (was ~50)

---

### Task B.5: Update apps/studio/CLAUDE.md

**File:** `apps/studio/CLAUDE.md`

**Step 1: Update Kind counts in schema section**

Find: `35 node types`
Replace: `46 node types`

**Step 2: Update API routes count if needed**

Current: 12 routes - verify this is still accurate.

---

### Task B.6: Update README.md

**File:** `README.md`

**Step 1: Update graph schema section**

```markdown
# Before:
NovaNet models content as a knowledge graph with **35 node types** across **3 Realms** and **9 Layers** (v9.0.0):

# After:
NovaNet models content as a knowledge graph with **46 node types** across **3 Realms** and **9 Layers** (v9.8.0):
```

---

### Task B.7: Update Session Hook

**File:** `.claude/hooks/session-start.sh`

**Step 1: Update version in hook output**

```bash
# Find the version string and update
sed -i '' 's/v9.0.1/v9.8.0/' .claude/hooks/session-start.sh
```

Or manually verify the hook reads version from a source of truth.

---

### Task B.8: Commit DX Updates

```bash
git add -A && git commit -m "$(cat <<'EOF'
docs(dx): update version to v9.8.0, correct schema counts

- Update .claude/README.md version banner
- Update CLAUDE.md files with 46 NodeKinds, 77 ArcKinds
- Update README.md graph schema section
- Sync session-start hook version

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>
EOF
)"
```

---

## Phase C: v10 Dynamic Retrieval

### Overview

v10 activates the context assembly features that v9 already carries in the meta-graph.
No new schema migration needed - v10 uses existing properties:
- `traversal_depth` (Kind)
- `default_traversal` (ArcFamily)
- `temperature_threshold` (ArcKind)

### Task C.1: Context Assembly Engine Design

**File:** `tools/novanet/src/retrieval/mod.rs` (NEW)

**Architecture:**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CONTEXT ASSEMBLY ENGINE (v10)                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Input: Block + Locale + TokenBudget                                        │
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌───────────┐   │
│  │  Read Meta  │ →  │ Plan Graph  │ →  │  Traverse   │ →  │ Assemble  │   │
│  │   Graph     │    │  Traversal  │    │   & Fetch   │    │  Context  │   │
│  └─────────────┘    └─────────────┘    └─────────────┘    └───────────┘   │
│                                                                             │
│  Output: ContextWindow { nodes: Vec<Node>, tokens_used: usize }             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Step 1: Define context types**

```rust
// src/retrieval/types.rs
pub struct ContextRequest {
    pub block_key: String,
    pub locale_key: String,
    pub token_budget: usize,
}

pub struct ContextWindow {
    pub nodes: Vec<ContextNode>,
    pub edges: Vec<ContextEdge>,
    pub tokens_used: usize,
    pub traversal_log: Vec<TraversalStep>,
}

pub struct ContextNode {
    pub kind: String,
    pub key: String,
    pub properties: serde_json::Value,
    pub token_estimate: usize,
}

pub struct TraversalStep {
    pub arc_kind: String,
    pub from_key: String,
    pub to_key: String,
    pub depth: u8,
    pub reason: String,
}
```

**Step 2: Create retrieval module structure**

```rust
// src/retrieval/mod.rs
mod types;
mod engine;
mod traversal;
mod budget;

pub use types::*;
pub use engine::ContextEngine;
```

---

### Task C.2: Meta-Graph Reader

**File:** `tools/novanet/src/retrieval/meta.rs` (NEW)

**Purpose:** Read traversal rules from Neo4j meta-graph.

```rust
pub struct MetaGraphReader {
    db: Arc<neo4rs::Graph>,
}

impl MetaGraphReader {
    /// Get traversal rules for a Kind
    pub async fn get_kind_rules(&self, kind: &str) -> Result<KindRules> {
        // Query Kind node for traversal_depth, context_budget
        let query = r#"
            MATCH (k:Kind {label: $kind})
            RETURN k.traversal_depth AS depth,
                   k.context_budget AS budget,
                   k.token_estimate AS tokens
        "#;
        // ...
    }

    /// Get arc traversal rules from ArcFamily/ArcKind
    pub async fn get_arc_rules(&self, family: &str) -> Result<Vec<ArcRule>> {
        // Query ArcKinds in family for default_traversal, temperature_threshold
        let query = r#"
            MATCH (af:ArcFamily {key: $family})<-[:IN_FAMILY]-(ak:ArcKind)
            RETURN ak.key AS arc,
                   af.default_traversal AS traversal,
                   ak.temperature_threshold AS temp
        "#;
        // ...
    }
}
```

---

### Task C.3: Traversal Planner

**File:** `tools/novanet/src/retrieval/traversal.rs` (NEW)

**Purpose:** Build traversal plan from meta-graph rules.

```rust
pub struct TraversalPlanner {
    meta: MetaGraphReader,
}

impl TraversalPlanner {
    /// Create traversal plan for a context request
    pub async fn plan(&self, request: &ContextRequest) -> Result<TraversalPlan> {
        // 1. Get starting node kind
        let start_kind = self.get_block_kind().await?;

        // 2. Get outgoing arcs with traversal rules
        let arcs = self.meta.get_arc_rules_for_kind(&start_kind).await?;

        // 3. Sort by priority (ownership > localization > semantic > generation)
        let prioritized = self.prioritize_arcs(arcs);

        // 4. Build depth-first traversal plan
        let plan = self.build_plan(prioritized, request.token_budget);

        Ok(plan)
    }
}

pub struct TraversalPlan {
    pub steps: Vec<PlannedStep>,
    pub estimated_tokens: usize,
}

pub struct PlannedStep {
    pub arc_kind: String,
    pub direction: Direction,
    pub max_depth: u8,
    pub filter: Option<String>,
}
```

---

### Task C.4: Context Executor

**File:** `tools/novanet/src/retrieval/engine.rs` (NEW)

**Purpose:** Execute traversal plan against Neo4j.

```rust
pub struct ContextEngine {
    db: Arc<neo4rs::Graph>,
    planner: TraversalPlanner,
}

impl ContextEngine {
    /// Assemble context window for a block generation
    pub async fn assemble(&self, request: ContextRequest) -> Result<ContextWindow> {
        // 1. Plan traversal
        let plan = self.planner.plan(&request).await?;

        // 2. Execute traversal with budget tracking
        let mut window = ContextWindow::new();
        let mut budget = TokenBudget::new(request.token_budget);

        for step in plan.steps {
            if budget.exhausted() {
                break;
            }

            let nodes = self.execute_step(&step, &mut budget).await?;
            window.add_nodes(nodes);
        }

        // 3. Return assembled context
        Ok(window)
    }
}
```

---

### Task C.5: CLI Command

**File:** `tools/novanet/src/commands/context.rs` (NEW)

**Purpose:** Expose context assembly via CLI.

```rust
// Add to main.rs:
// context --block=<key> --locale=<key> [--budget=<tokens>] [--format=json|table]

pub async fn run_context(
    db: &Db,
    block: &str,
    locale: &str,
    budget: usize,
    format: OutputFormat,
) -> crate::Result<()> {
    let engine = ContextEngine::new(db.clone());

    let request = ContextRequest {
        block_key: block.to_string(),
        locale_key: locale.to_string(),
        token_budget: budget,
    };

    let window = engine.assemble(request).await?;

    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&window)?),
        OutputFormat::Table => render_context_table(&window),
    }

    Ok(())
}
```

---

### Task C.6: Integration Tests

**File:** `tools/novanet/tests/context_integration.rs` (NEW)

```rust
#[tokio::test]
#[ignore] // Requires Neo4j
async fn context_assembly_respects_budget() {
    let db = test_db().await;
    let engine = ContextEngine::new(db);

    let request = ContextRequest {
        block_key: "hero-pricing".to_string(),
        locale_key: "fr-FR".to_string(),
        token_budget: 1000,
    };

    let window = engine.assemble(request).await.unwrap();

    assert!(window.tokens_used <= 1000);
    assert!(!window.nodes.is_empty());
}

#[tokio::test]
#[ignore]
async fn context_includes_locale_knowledge() {
    let db = test_db().await;
    let engine = ContextEngine::new(db);

    let request = ContextRequest {
        block_key: "hero-pricing".to_string(),
        locale_key: "fr-FR".to_string(),
        token_budget: 5000,
    };

    let window = engine.assemble(request).await.unwrap();

    // Should include LocaleVoice, LocaleCulture, etc.
    let kinds: Vec<_> = window.nodes.iter().map(|n| n.kind.as_str()).collect();
    assert!(kinds.contains(&"LocaleVoice"));
    assert!(kinds.contains(&"LocaleCulture"));
}
```

---

## Verification Checklist

After each phase:

### Phase A
- [ ] `cargo test` — All Rust tests pass
- [ ] `pnpm test` — All TypeScript tests pass
- [ ] `cargo clippy -- -D warnings` — Zero warnings
- [ ] Schema counts verified (46 NodeKinds, 77 ArcKinds)

### Phase B
- [ ] All CLAUDE.md files updated
- [ ] Version shows v9.8.0 everywhere
- [ ] Counts are accurate
- [ ] Session hook updated

### Phase C
- [ ] `cargo test --features=retrieval` — Context tests pass
- [ ] `cargo run -- context --help` — Command available
- [ ] Integration test with Neo4j passes
- [ ] Context respects token budget

---

## Skills & Agents to Load

**Recommended skills for implementation:**

| Skill | Purpose |
|-------|---------|
| `spn-rust:rust-async` | Tokio patterns for Neo4j queries |
| `spn-rust:rust-core` | Error handling, type-state patterns |
| `neo4j-architect` | Cypher query optimization |
| `test-driven-development` | TDD for context engine |
| `systematic-debugging` | Debug traversal issues |

**Recommended agents:**

| Agent | Purpose |
|-------|---------|
| `neo4j-architect` | Design efficient Cypher for context retrieval |
| `code-reviewer` | Review context engine implementation |

---

## Execution Order

```
Phase A ──▶ Phase B ──▶ Phase C.1-C.2 ──▶ Phase C.3-C.4 ──▶ Phase C.5-C.6
(verify)   (docs)      (types+meta)      (planner+engine)  (cli+tests)
```

**Estimated time:**
- Phase A: 30min
- Phase B: 1h
- Phase C: 4-6h (can be parallelized with subagents)

---

## Notes

- v10 does NOT require schema changes — it activates existing meta-graph properties
- Context assembly is autonomous — traversal decisions come from meta-graph, not code
- Token estimation uses heuristics (chars/4 or model-specific tokenizer)
- Budget is soft limit — engine may slightly exceed for essential context
