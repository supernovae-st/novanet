# NovaNet Blueprint - Implementation Plan

**Date**: 2026-02-09
**Design Doc**: `2026-02-09-novanet-blueprint-design.md`

## Overview

Replace `novanet meta` with `novanet blueprint` - a comprehensive meta-graph visualization and validation command.

## Shell Aliases

Added to `~/.spn/dotfiles/zshrc`:

```bash
nv        # → novanet tui
nv bp     # → novanet blueprint
nv bp --view=flow  # → novanet blueprint --view=flow
nv meta   # → novanet meta (legacy)
```

## Implementation Tasks

### Phase 1: Foundation (Core Infrastructure)

#### Task 1.1: Create blueprint module structure
**Files**: `src/blueprint/mod.rs`, `src/blueprint/sources.rs`

```rust
// src/blueprint/mod.rs
pub mod sources;
pub mod validation;
pub mod ascii;
pub mod views;

pub use sources::BlueprintSources;
pub use validation::ValidationResult;
```

```rust
// src/blueprint/sources.rs
use crate::parsers::{load_node_kinds, load_arc_kinds, load_taxonomy};
use crate::db::Db;

pub struct BlueprintSources {
    pub node_kinds: Vec<NodeKindYaml>,
    pub arc_kinds: Vec<ArcKindYaml>,
    pub taxonomy: Taxonomy,
    pub neo4j_meta: Option<Neo4jMeta>,
}

impl BlueprintSources {
    /// Load from YAML only (no Neo4j)
    pub fn from_yaml(root: &Path) -> Result<Self> { ... }

    /// Load from YAML + Neo4j
    pub async fn from_all(root: &Path, db: &Db) -> Result<Self> { ... }
}
```

#### Task 1.2: Create ASCII box drawing utilities
**File**: `src/blueprint/ascii.rs`

```rust
pub struct BoxBuilder {
    title: String,
    width: usize,
    sections: Vec<Section>,
}

impl BoxBuilder {
    pub fn new(title: &str) -> Self { ... }
    pub fn width(mut self, w: usize) -> Self { ... }
    pub fn section(mut self, name: &str, content: String) -> Self { ... }
    pub fn render(&self) -> String { ... }
}

// Progress bar helper
pub fn progress_bar(value: usize, max: usize, width: usize) -> String { ... }

// Example: ████████░░░░ 67%
```

#### Task 1.3: Create validation system
**File**: `src/blueprint/validation.rs`

```rust
pub struct ValidationResult {
    pub checks: Vec<ValidationCheck>,
    pub issues: Vec<ValidationIssue>,
}

pub struct ValidationCheck {
    pub name: String,
    pub passed: bool,
    pub details: Option<String>,
}

pub struct ValidationIssue {
    pub severity: Severity,
    pub category: String,
    pub message: String,
    pub fix_hint: Option<String>,
}

pub enum Severity { Error, Warning, Info }

impl ValidationResult {
    pub fn validate(sources: &BlueprintSources) -> Self { ... }

    fn check_yaml_neo4j_sync(&mut self, sources: &BlueprintSources) { ... }
    fn check_arc_coherence(&mut self, sources: &BlueprintSources) { ... }
    fn check_path_content_match(&mut self, sources: &BlueprintSources) { ... }
    fn check_orphan_nodes(&mut self, sources: &BlueprintSources) { ... }
}
```

---

### Phase 2: Command & Default View

#### Task 2.1: Create blueprint command
**Files**: `src/commands/blueprint.rs`, update `src/commands/mod.rs`, update `src/main.rs`

```rust
// src/commands/blueprint.rs
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum BlueprintView {
    Tree,
    Flow,
    Content,
    Arcs,
    Cardinality,
    Glossary,
    Audit,
    Deps,
    Coverage,
    Stats,
}

pub async fn run_blueprint(
    db: Option<&Db>,
    root: &Path,
    view: Option<BlueprintView>,
    format: OutputFormat,
    no_validate: bool,
) -> Result<()> {
    let sources = if let Some(db) = db {
        BlueprintSources::from_all(root, db).await?
    } else {
        BlueprintSources::from_yaml(root)?
    };

    match view {
        None => render_default(&sources, !no_validate),
        Some(v) => render_view(&sources, v, format),
    }
}
```

#### Task 2.2: Implement default overview
**File**: `src/blueprint/views/default.rs`

Render the full overview:
- Header with version
- Stats section
- Realms with progress bars
- Layers with counts and icons
- Traits with symbols and examples
- Core flow diagram
- Arc families
- Validation status
- Views hint

---

### Phase 3: Understanding Views (6 views)

#### Task 3.1: Tree view
**File**: `src/blueprint/views/tree.rs`

```
GLOBAL (read-only, universal)
├── config ⚙
│   └── SystemConfig [invariant] — settings globaux
└── locale-knowledge 📚
    ├── Locale [invariant] — fr-FR, en-US...
    └── LocaleVoice [knowledge] — ton, style par locale
...
```

#### Task 3.2: Flow view
**File**: `src/blueprint/views/flow.rs`

6 sub-diagrams:
1. Ownership flow
2. Localization flow
3. Knowledge flow
4. Generation flow
5. SEO flow
6. Cross-realm flow

#### Task 3.3: Content view
**File**: `src/blueprint/views/content.rs`

Detailed Content Model diagram showing:
- Structural hierarchy
- Semantic content layer
- Output generation layer
- SEO integration
- Legend

#### Task 3.4: Arcs view
**File**: `src/blueprint/views/arcs.rs`

All arcs grouped by family:
```
OWNERSHIP (43 arcs)
├── HAS_PAGE        Project → Page
├── HAS_BLOCK       Page → Block
...

LOCALIZATION (12 arcs)
├── HAS_CONTENT     Entity → EntityContent
...
```

#### Task 3.5: Cardinality view
**File**: `src/blueprint/views/cardinality.rs`

Show 1:1, 1:N, N:M relationships with visual symbols.

#### Task 3.6: Glossary view
**File**: `src/blueprint/views/glossary.rs`

```
GLOSSARY

Realm
  WHERE a node lives. 2 realms: global (universal) and tenant (business).

Layer
  WHAT category a node belongs to. 9 layers total...

Trait
  HOW a node behaves with locales...
```

---

### Phase 4: Analysis Views (4 views)

#### Task 4.1: Audit view
**File**: `src/blueprint/views/audit.rs`

Detailed health check:
- YAML ↔ Neo4j drift
- Orphan nodes
- Invalid arc references
- Path/content mismatches

#### Task 4.2: Deps view
**File**: `src/blueprint/views/deps.rs`

```bash
novanet blueprint --view=deps --kind=Entity
```

Shows:
- What depends on this kind
- What this kind depends on
- Impact score

#### Task 4.3: Coverage view
**File**: `src/blueprint/views/coverage.rs`

Requires Neo4j connection. Shows locale completion:
- Content coverage per locale
- Generated output coverage
- Knowledge atoms loaded

#### Task 4.4: Stats view
**File**: `src/blueprint/views/stats.rs`

Raw numbers for CI/scripts:
```json
{
  "node_kinds": 76,
  "arc_kinds": 123,
  "realms": 2,
  "layers": 9,
  "validation": { "passed": true, "issues": 0 }
}
```

---

### Phase 5: Cleanup & Polish

#### Task 5.1: Deprecate `novanet meta`
- Keep `meta` working but show deprecation warning
- Suggest using `blueprint` or `blueprint --view=stats`

#### Task 5.2: Update CLAUDE.md
- Document new `blueprint` command
- Update command table
- Add examples

#### Task 5.3: Add tests
- Unit tests for ASCII rendering
- Unit tests for validation logic
- Integration tests for each view
- Snapshot tests for output format

#### Task 5.4: Update help text
- `novanet blueprint --help` with examples
- `novanet blueprint --view=X --help` per-view help

---

## File Checklist

```
src/
  commands/
    mod.rs                    # [EDIT] Add blueprint
    blueprint.rs              # [NEW] Command entry
  blueprint/
    mod.rs                    # [NEW] Module root
    sources.rs                # [NEW] YAML + Neo4j loader
    validation.rs             # [NEW] Coherence checker
    ascii.rs                  # [NEW] Box drawing
    views/
      mod.rs                  # [NEW] View dispatcher
      default.rs              # [NEW] Overview
      tree.rs                 # [NEW]
      flow.rs                 # [NEW]
      content.rs              # [NEW]
      arcs.rs                 # [NEW]
      cardinality.rs          # [NEW]
      glossary.rs             # [NEW]
      audit.rs                # [NEW]
      deps.rs                 # [NEW]
      coverage.rs             # [NEW]
      stats.rs                # [NEW]
  main.rs                     # [EDIT] Add blueprint subcommand
  lib.rs                      # [EDIT] Export blueprint module

tools/novanet/CLAUDE.md       # [EDIT] Document blueprint
```

## Estimated Effort

| Phase | Tasks | Complexity |
|-------|-------|------------|
| Phase 1: Foundation | 3 | Medium |
| Phase 2: Command & Default | 2 | Medium |
| Phase 3: Understanding Views | 6 | High |
| Phase 4: Analysis Views | 4 | Medium |
| Phase 5: Cleanup | 4 | Low |

**Total**: 19 tasks

## Dependencies

- No new crates required
- Uses existing: `tabled`, `neo4rs`, `serde_yaml`, `clap`

## Success Metrics

- [ ] `novanet blueprint` renders in < 2s
- [ ] All 10 views working
- [ ] Validation catches YAML/Neo4j drift
- [ ] `nv bp` alias works from any directory
- [ ] JSON output valid for `jq` parsing
- [ ] Zero clippy warnings
- [ ] 100% validation logic test coverage
