# Comprehensive Next Steps Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to execute phases.

**Goal:** Complete 4 phases: 80% TUI coverage, SEO/GEO architecture, code review, documentation

**Architecture:** Multi-phase execution with specialized agents per domain

**Tech Stack:** Rust (TUI), YAML (schema), Cypher (Neo4j), TypeScript (generators)

---

## Skills & Agents Reference

### Skills (spn-powers/superpowers)

| Skill | When to Use |
|-------|-------------|
| `spn-powers:brainstorming` | Before coding, refine ideas into designs |
| `spn-powers:writing-plans` | Create detailed implementation plans |
| `spn-powers:subagent-driven-development` | Execute plans with fresh subagent per task |
| `spn-powers:test-driven-development` | TDD: test first, implement, verify |
| `spn-powers:verification-before-completion` | Run tests before claiming complete |
| `spn-powers:requesting-code-review` | Review after each major phase |
| `spn-powers:systematic-debugging` | When bugs/failures occur |
| `spn-powers:finishing-a-development-branch` | Complete work, decide merge/PR |

### Agents (Task tool)

| Agent | Purpose |
|-------|---------|
| `spn-rust:rust-pro` | Rust implementation, TDD, best practices |
| `spn-rust:rust-architect` | Rust architecture design |
| `spn-rust:rust-security` | Security review for Rust code |
| `feature-dev:code-explorer` | Deep codebase analysis |
| `feature-dev:code-architect` | Design feature architectures |
| `feature-dev:code-reviewer` | Code review against plan |
| `Explore` | Fast codebase exploration |
| `general-purpose` | Multi-step complex tasks |

---

## Phase 1: 80% TUI Coverage (~4,282 LOC)

**Target Modules:**
- `ui/graph.rs` (779 lines) - Graph visualization
- `ui/atlas.rs` (1,634 lines) - Atlas mode views
- `ui/info.rs` (1,388 lines) - Info panel rendering
- `ui/overlays.rs` (481 lines) - Modal dialogs

### Task 1.1: Analyze graph.rs for testable logic

**Agent:** `Explore`
**Skill:** Analysis only

**Steps:**
1. Read `src/tui/ui/graph.rs` completely
2. Identify pure functions (no Frame dependency)
3. List functions that can be unit tested
4. Report: function names, signatures, test strategy

---

### Task 1.2: Add graph.rs rendering helper tests

**Agent:** `spn-rust:rust-pro`
**Skill:** `test-driven-development`

**Files:**
- Modify: `src/tui/ui/graph.rs`

**Steps:**
1. Write failing tests for helper functions identified in 1.1
2. If helpers don't exist, extract testable logic
3. Run tests, verify pass
4. Commit

---

### Task 1.3: Analyze atlas.rs for testable logic

**Agent:** `Explore`

**Steps:**
1. Read `src/tui/ui/atlas.rs` completely
2. Identify: state management, view selection, data transformation
3. List testable functions
4. Report findings

---

### Task 1.4: Add atlas.rs state and view tests

**Agent:** `spn-rust:rust-pro`
**Skill:** `test-driven-development`

**Files:**
- Modify: `src/tui/ui/atlas.rs`
- Modify: `src/tui/atlas/state.rs`

**Steps:**
1. Write tests for AtlasState navigation
2. Write tests for view selection logic
3. Write tests for data transformation
4. Run and verify
5. Commit

---

### Task 1.5: Analyze info.rs for testable logic

**Agent:** `Explore`

**Steps:**
1. Read `src/tui/ui/info.rs`
2. Identify: property formatting, arc display, content building
3. List testable functions

---

### Task 1.6: Add info.rs panel content tests

**Agent:** `spn-rust:rust-pro`
**Skill:** `test-driven-development`

**Files:**
- Modify: `src/tui/ui/info.rs`

**Steps:**
1. Write tests for property formatting
2. Write tests for arc display helpers
3. Run and verify
4. Commit

---

### Task 1.7: Analyze overlays.rs for testable logic

**Agent:** `Explore`

**Steps:**
1. Read `src/tui/ui/overlays.rs`
2. Identify: command palette filtering, help content building
3. List testable functions

---

### Task 1.8: Add overlays.rs dialog tests

**Agent:** `spn-rust:rust-pro`
**Skill:** `test-driven-development`

**Files:**
- Modify: `src/tui/ui/overlays.rs`

**Steps:**
1. Write tests for command filtering
2. Write tests for help content generation
3. Run and verify
4. Commit

---

### Task 1.9: Coverage verification

**Agent:** `general-purpose`
**Skill:** `verification-before-completion`

**Steps:**
1. Run `cargo llvm-cov --html`
2. Check TUI module coverage
3. Report: before/after comparison
4. Identify remaining gaps

---

## Phase 2: SEO/GEO Architecture

### Task 2.1: Create SEOMetrics schema

**Agent:** `feature-dev:code-architect`
**Skill:** `schema:add-node`

**Files:**
- Create: `packages/core/models/node-classes/global/seo/seo-metrics.yaml`

**Steps:**
1. Design SEOMetrics node with time-series properties
2. Create YAML following existing patterns
3. Run `cargo run -- schema generate`
4. Run `cargo run -- schema validate`
5. Commit

---

### Task 2.2: Update SEOKeyword schema

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `packages/core/models/node-classes/global/seo/seo-keyword.yaml`

**Steps:**
1. Remove volatile properties (volume, difficulty, cpc, etc.)
2. Keep stable properties (key, value, intent, platform, source)
3. Regenerate schema
4. Commit

---

### Task 2.3: Create HAS_METRICS arc

**Agent:** `feature-dev:code-architect`
**Skill:** `schema:add-arc`

**Files:**
- Create: `packages/core/models/arc-classes/ownership/has-metrics.yaml`

**Steps:**
1. Design arc linking SEO/GEO nodes to Metrics
2. Set cardinality: 1:N (one node can have many metric snapshots)
3. Regenerate schema
4. Commit

---

### Task 2.4: Import SEOQuestion from ATP

**Agent:** `general-purpose`

**Files:**
- Create: `scripts/seo-import/import_seo_questions.py`
- Create: `packages/db/seed/13-seoquestion-fr-fr.cypher`

**Steps:**
1. Parse ATP CSV: `docs/assets/keywods/fr-fr_qr/seo/paas_export_qr code.csv`
2. Extract question data (27 questions)
3. Generate Cypher for SEOQuestion nodes
4. Link to parent SEOKeyword via [:HAS_QUESTIONS]
5. Commit

---

### Task 2.5: Create GEO node kinds

**Agent:** `feature-dev:code-architect`

**Files:**
- Create: `packages/core/models/node-classes/global/seo/geo-prompt.yaml`
- Create: `packages/core/models/node-classes/global/seo/geo-response.yaml`
- Create: `packages/core/models/node-classes/global/seo/geo-citation.yaml`
- Create: `packages/core/models/node-classes/global/seo/geo-mention.yaml`
- Create: `packages/core/models/node-classes/global/seo/geo-metrics.yaml`

**Steps:**
1. Create each GEO node following plan spec
2. Add appropriate properties per type
3. Regenerate all artifacts
4. Validate schema
5. Commit

---

### Task 2.6: Create GEO arc kinds

**Agent:** `feature-dev:code-architect`

**Files:**
- Create: `packages/core/models/arc-classes/semantic/has-response.yaml`
- Create: `packages/core/models/arc-classes/semantic/has-citation.yaml`
- Create: `packages/core/models/arc-classes/semantic/has-mention.yaml`
- Create: `packages/core/models/arc-classes/semantic/answers.yaml`

**Steps:**
1. Create arcs for GEO relationships
2. Define cardinalities and scopes
3. Regenerate all artifacts
4. Validate
5. Commit

---

## Phase 3: Code Review

### Task 3.1: TUI test infrastructure review

**Agent:** `feature-dev:code-reviewer`
**Skill:** `requesting-code-review`

**Review scope:**
- All commits from 2026-02-09
- `src/tui/testing.rs` module
- Theme, data, schema test additions

**Checklist:**
- [ ] TDD followed (test first)
- [ ] No code duplication
- [ ] Proper test isolation
- [ ] Edge cases covered
- [ ] Security tests present

---

### Task 3.2: SEO/GEO schema review

**Agent:** `feature-dev:code-reviewer`

**Review scope:**
- All new YAML schema files
- Arc cardinalities and scopes
- Naming conventions (ADR compliance)

**Checklist:**
- [ ] Realm/layer correct
- [ ] Properties properly typed
- [ ] Arc scopes valid
- [ ] No circular dependencies

---

### Task 3.3: Rust security review

**Agent:** `spn-rust:rust-security`

**Review scope:**
- Cypher injection prevention
- Input validation
- Error handling

**Checklist:**
- [ ] validate_cypher_label covers all cases
- [ ] No string interpolation in Cypher
- [ ] Proper error propagation

---

## Phase 4: Documentation & Cleanup

### Task 4.1: Update CLAUDE.md counts

**Agent:** `general-purpose`

**Files:**
- Modify: `CLAUDE.md`
- Modify: `tools/novanet/CLAUDE.md`
- Modify: `.claude/README.md`

**Steps:**
1. Count current nodes: `ls packages/core/models/node-classes/**/*.yaml | wc -l`
2. Count current arcs: `ls packages/core/models/arc-classes/**/*.yaml | wc -l`
3. Update all documentation with correct counts
4. Commit

---

### Task 4.2: Update ROADMAP.md

**Agent:** `general-purpose`

**Files:**
- Modify: `ROADMAP.md`

**Steps:**
1. Mark completed items from v11.0.0
2. Add v11.1.0 section with GEO features
3. Update timeline
4. Commit

---

### Task 4.3: Update CHANGELOG.md

**Agent:** `general-purpose`

**Files:**
- Modify: `CHANGELOG.md`

**Steps:**
1. Add v11.0.0 section
2. Document: TUI test infrastructure, SEO/GEO nodes, coverage improvements
3. Follow keep-a-changelog format
4. Commit

---

### Task 4.4: Run full audit

**Agent:** `general-purpose`
**Skill:** `/security-audit`

**Steps:**
1. `cargo deny check`
2. `cargo audit`
3. `cargo clippy -- -D warnings`
4. `pnpm audit`
5. Report any issues
6. Fix if needed

---

### Task 4.5: Final verification

**Agent:** `Explore`
**Skill:** `verification-before-completion`

**Steps:**
1. `cargo test` - all pass?
2. `cargo run -- schema validate` - clean?
3. `git status` - no uncommitted changes?
4. Coverage report - improved?
5. Summary report

---

## Execution Order

```
Phase 1 (TUI Coverage)
├── 1.1-1.2: graph.rs (parallel: analyze → implement)
├── 1.3-1.4: atlas.rs (parallel: analyze → implement)
├── 1.5-1.6: info.rs (parallel: analyze → implement)
├── 1.7-1.8: overlays.rs (parallel: analyze → implement)
└── 1.9: Verify coverage

Phase 2 (SEO/GEO)
├── 2.1-2.2: SEOMetrics + SEOKeyword update
├── 2.3: HAS_METRICS arc
├── 2.4: SEOQuestion import
└── 2.5-2.6: GEO nodes + arcs

Phase 3 (Review)
├── 3.1: TUI review
├── 3.2: Schema review
└── 3.3: Security review

Phase 4 (Docs)
├── 4.1: Update counts
├── 4.2: ROADMAP
├── 4.3: CHANGELOG
├── 4.4: Security audit
└── 4.5: Final verification
```

---

## Success Criteria

| Phase | Metric | Target |
|-------|--------|--------|
| 1 | TUI coverage | ≥80% |
| 2 | GEO nodes created | 5 nodes |
| 2 | GEO arcs created | 4 arcs |
| 3 | Review issues | 0 critical |
| 4 | All tests pass | 100% |
| 4 | All docs updated | Yes |
