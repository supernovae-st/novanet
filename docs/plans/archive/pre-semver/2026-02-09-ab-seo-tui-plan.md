# A+B Implementation Plan: SEOQuestion Import + TUI 80% Coverage

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to execute tasks.

**Goal:** Import 27 French SEO questions from ATP CSV + achieve 80% TUI test coverage

**Architecture:** Python script for CSV parsing, Cypher for Neo4j import, Rust tests for TUI

**Tech Stack:** Python (CSV parsing), Cypher (Neo4j), Rust (TUI tests), proptest

---

## Part A: SEOQuestion Import from ATP CSV

### Task A.1: Parse ATP CSV and generate Cypher

**Agent:** `general-purpose`

**Files:**
- Read: `docs/assets/keywods/fr-fr_qr/seo/paas_export_qr code.csv`
- Create: `packages/db/seed/13-seoquestion-fr-fr.cypher`

**Steps:**
1. Parse the hierarchical ATP CSV format:
   - Level 0: Root keyword ("qr code")
   - Level 1: Parent questions (2-space indent)
   - Level 2: Child questions (4-space indent)
2. Generate SEOQuestion nodes with properties:
   - key: slugified question
   - value: original question text
   - question_word: extracted (comment/pourquoi/où/quel/est-ce)
   - parent_keyword: "qr-code-fr"
3. Generate [:HAS_QUESTIONS] arcs from SEOKeyword
4. Write Cypher to seed file
5. Run: `cargo run -- db seed`

---

### Task A.2: Verify SEOQuestion import

**Agent:** `Explore`

**Steps:**
1. Query Neo4j: `MATCH (q:SEOQuestion) RETURN count(q)`
2. Verify 27 questions imported
3. Check [:HAS_QUESTIONS] arcs exist
4. Report findings

---

## Part B: TUI 80% Coverage

### Task B.1: Add ui/atlas.rs render tests

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `src/tui/ui/atlas.rs`

**Steps:**
1. Add snapshot tests for `render_atlas_realm_map()`
2. Add tests for `render_atlas_spreading_activation()` formula display
3. Add tests for `render_atlas_knowledge_atoms()` content
4. Target: 50%+ coverage for atlas.rs
5. Commit

---

### Task B.2: Add ui/tree.rs rendering tests

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `src/tui/ui/tree.rs`

**Steps:**
1. Add tests for tree node rendering
2. Add tests for expand/collapse icons
3. Add tests for selection highlighting
4. Target: 50%+ coverage for tree.rs
5. Commit

---

### Task B.3: Add ui/status.rs tests

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `src/tui/ui/status.rs`

**Steps:**
1. Add tests for status bar content
2. Add tests for mode display
3. Add tests for stats formatting
4. Target: 80%+ coverage for status.rs
5. Commit

---

### Task B.4: Add ui/yaml_panel.rs tests

**Agent:** `spn-rust:rust-pro`

**Files:**
- Modify: `src/tui/ui/yaml_panel.rs`

**Steps:**
1. Add tests for YAML content rendering
2. Add tests for syntax highlighting
3. Add tests for scroll position
4. Target: 50%+ coverage for yaml_panel.rs
5. Commit

---

### Task B.5: Coverage verification

**Agent:** `general-purpose`

**Steps:**
1. Run `cargo test` - verify all pass
2. Run `cargo llvm-cov report --summary-only`
3. Check TUI module coverage
4. Report: which modules hit 80%

---

## Execution Order

```
Part A (SEOQuestion Import)
├── A.1: Parse CSV → Cypher
└── A.2: Verify import

Part B (TUI Coverage) - parallel where possible
├── B.1: ui/atlas.rs tests
├── B.2: ui/tree.rs tests
├── B.3: ui/status.rs tests
├── B.4: ui/yaml_panel.rs tests
└── B.5: Coverage verification
```

---

## Success Criteria

| Task | Metric | Target |
|------|--------|--------|
| A.1 | SEOQuestion nodes | 27 |
| A.2 | HAS_QUESTIONS arcs | Present |
| B.1-B.4 | TUI coverage | ≥80% |
| B.5 | All tests pass | Yes |
