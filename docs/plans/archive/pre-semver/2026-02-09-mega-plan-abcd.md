# Mega Plan: SEOQuestion + TUI 80% + Context Engine + Content Pipeline

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to execute phases.

**Goal:** Execute 4 parallel workstreams: A) Import SEOQuestion data, B) Reach 80% TUI coverage, C) Add Phase 9.9 retrieval properties, D) Create content generation pipeline.

**Architecture:** Multi-phase execution with specialized agents per domain.

**Tech Stack:** Python (import scripts), Rust (TUI/CLI), YAML (schema), Cypher (Neo4j)

---

## Skills & Agents Reference

| Skill/Agent | When to Use |
|-------------|-------------|
| `spn-rust:rust-pro` | Rust implementation, TDD |
| `general-purpose` | Multi-step tasks, scripts |
| `Explore` | Fast codebase exploration |
| `feature-dev:code-reviewer` | Review after major work |

---

## Phase A: SEOQuestion Import (~27 questions)

### Task A.1: Parse ATP CSV structure

**Agent:** `general-purpose`

**Files:**
- Read: `docs/assets/keywods/fr-fr_qr/seo/paas_export_qr code.csv`
- Create: `scripts/seo-import/parse_atp_questions.py`

**Steps:**
1. Analyze CSV structure (indentation = hierarchy)
2. Extract parent keyword → child questions mapping
3. Detect question_word (comment, pourquoi, quel, où, etc.)
4. Output: JSON with questions array

---

### Task A.2: Generate SEOQuestion Cypher

**Agent:** `general-purpose`

**Files:**
- Create: `packages/db/seed/13-seoquestion-fr-fr.cypher`

**Steps:**
1. Read parsed JSON from A.1
2. Generate CREATE statements for SEOQuestion nodes
3. Generate MERGE for [:HAS_QUESTIONS] arcs to parent SEOKeyword
4. Validate Cypher syntax
5. Commit

---

### Task A.3: Seed to Neo4j

**Agent:** `general-purpose`

**Steps:**
1. Run `cargo run -- db seed`
2. Verify: `MATCH (q:SEOQuestion) RETURN count(q)` = 27
3. Verify: `MATCH (:SEOKeyword)-[:HAS_QUESTIONS]->(:SEOQuestion) RETURN count(*)` = 27
4. Commit if needed

---

## Phase B: TUI 80% Coverage

### Task B.1: Add ui/tree.rs tests

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `src/tui/ui/tree.rs` (currently 13% coverage)

**Steps:**
1. Add tests for tree item rendering
2. Add tests for selection highlighting
3. Add tests for collapse/expand icons
4. Run `cargo test`, verify pass
5. Commit

---

### Task B.2: Add ui/atlas.rs rendering tests

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `src/tui/ui/atlas.rs` (currently 0% coverage)

**Steps:**
1. Add tests for `render_atlas_realm_map()` output
2. Add tests for `render_atlas_spreading_activation()` formula display
3. Add snapshot tests for static content
4. Run `cargo test`, verify pass
5. Commit

---

### Task B.3: Add ui/status.rs tests

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `src/tui/ui/status.rs` (currently 0% coverage)

**Steps:**
1. Add tests for status bar content
2. Add tests for mode display
3. Add tests for stats rendering
4. Run `cargo test`, verify pass
5. Commit

---

### Task B.4: Coverage verification

**Agent:** `general-purpose`

**Steps:**
1. Run `cargo llvm-cov --lib`
2. Calculate TUI module average
3. Report: target 80%, actual %
4. Identify remaining gaps

---

## Phase C: Context Engine (Phase 9.9)

### Task C.1: Add retrieval defaults to taxonomy.yaml

**Agent:** `general-purpose`

**Files:**
- Modify: `packages/core/models/taxonomy.yaml`

**Steps:**
1. Add `kind_retrieval_defaults` section after `node_traits`
2. Add per-trait settings: `traversal_depth`, `context_budget`, `token_estimate`
3. Run `cargo run -- schema validate`
4. Commit

**Content to add:**
```yaml
kind_retrieval_defaults:
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

---

### Task C.2: Add default_traversal to arc_families

**Agent:** `general-purpose`

**Files:**
- Modify: `packages/core/models/taxonomy.yaml`

**Steps:**
1. Add `default_traversal` to each arc_family entry
2. Values: `eager` (always follow), `lazy` (if budget), `skip` (never)
3. Run `cargo run -- schema validate`
4. Commit

**Mapping:**
- ownership: eager
- localization: eager
- semantic: lazy
- generation: lazy
- mining: skip

---

### Task C.3: Update Rust parser for retrieval properties

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `src/parsers/taxonomy.rs`

**Steps:**
1. Add `RetrievalDefaults` struct
2. Add `default_traversal` to `ArcFamilyDef`
3. Parse new YAML fields
4. Add tests for new parsing
5. Run `cargo test`, verify pass
6. Commit

---

## Phase D: Content Pipeline

### Task D.1: Create generation command structure

**Agent:** `spn-rust:rust-pro`

**Files:**
- Create: `src/commands/generate.rs`
- Modify: `src/commands/mod.rs`
- Modify: `src/main.rs`

**Steps:**
1. Add `generate` subcommand with clap
2. Add `--entity`, `--locale`, `--dry-run` flags
3. Add basic command dispatch
4. Run `cargo build`, verify compiles
5. Commit

---

### Task D.2: Implement context assembly

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `src/commands/generate.rs`

**Steps:**
1. Query Entity + EntityContent for locale
2. Traverse [:USES_ENTITY], [:TARGETS] to gather context
3. Assemble context window with token estimation
4. Return assembled context as JSON
5. Add tests
6. Commit

---

### Task D.3: Add LLM integration stub

**Agent:** `spn-rust:rust-pro`

**Files:**
- Create: `src/llm.rs`
- Modify: `src/lib.rs`

**Steps:**
1. Create `LlmClient` trait
2. Create `MockLlmClient` for testing
3. Create `AnthropicClient` stub (API key from env)
4. Add to generate command
5. Run `cargo test`, verify pass
6. Commit

---

## Execution Order

```
Parallel Batch 1:
├── A.1: Parse ATP CSV
├── B.1: tree.rs tests
├── C.1: retrieval defaults
└── D.1: generate command structure

Parallel Batch 2:
├── A.2: Generate Cypher
├── B.2: atlas.rs tests
├── C.2: default_traversal
└── D.2: context assembly

Parallel Batch 3:
├── A.3: Seed to Neo4j
├── B.3: status.rs tests
├── C.3: Rust parser update
└── D.3: LLM integration stub

Final:
├── B.4: Coverage verification
└── Code review all phases
```

---

## Success Criteria

| Phase | Metric | Target |
|-------|--------|--------|
| A | SEOQuestion nodes | 27 imported |
| B | TUI coverage | >= 80% |
| C | Retrieval properties | Schema validates |
| D | Generate command | `cargo run -- generate --help` works |

