# CLAUDE.md

This file provides guidance to Claude Code when working in the `tools/novanet/` Rust project.

## Overview

`novanet` is a unified Rust CLI + TUI binary for managing the NovaNet context graph.
It replaces the TypeScript `@novanet/schema-tools` and `@novanet/cli` packages.

**Version**: v0.13.1 (*Native Pattern + LLM-First BLOC Schema + ADR-029/ADR-030 + ADR-024 Data Origin + ADR-025 Instruction Layer + ADR-028 Brand Architecture)

## Current Status

**v0.13.1 LLM-First BLOC Schema** — All 61 node-class YAML files standardized with canonical 6-BLOC property ordering for optimal AI comprehension: BLOC 1 (Identity), BLOC 2 (Semantic), BLOC 3 (Visual), BLOC 4 (Data), BLOC 5 (Graph), BLOC 6 (Reference). Includes v0.13.0 changes: ADR-029 *Native Pattern, ADR-030 Slug Ownership, ADR-024 Data Origin traits (defined/authored/imported/generated/retrieved), ADR-025 Instruction Layer, ADR-028 Brand Architecture. SHARED (4 layers: config, locale, geography, knowledge, 40 nodes), ORG (6 layers: config, foundation, structure, semantic, instruction, output, 21 nodes). 61 total nodes, 182 arcs, 10 layers, 6 arc families.

| Area | Commands | Status |
|------|----------|--------|
| Read | `blueprint`, `data`, `overlay`, `query` | Implemented (faceted Cypher) |
| Write | `node create/edit/delete`, `arc create/delete` | Implemented (label validation) |
| Schema | `schema generate`, `schema validate` | Implemented (12 artifacts) |
| Auto-Fix | 6 fixers via FixEngine | Implemented (TDD + property-based tests) |
| Docs | `doc generate`, `doc generate --list` | Implemented (40 views) |
| Search | `search --query=... [--class=...]` | Implemented (fulltext + property) |
| Locale | `locale list`, `locale import`, `locale generate` | Implemented |
| Knowledge | `knowledge generate`, `knowledge list` | Implemented (ATH integration) |
| Entity | `entity seed`, `entity list`, `entity validate` | Implemented (phase-based) |
| DB | `db seed`, `db migrate`, `db reset` | Implemented |
| Filter | `filter build` | Implemented (JSON stdin, Studio subprocess) |
| Blueprint | `blueprint [--view=X]` | Implemented (11 views) |
| TUI | `tui` | Unified tree (Graph/Nexus modes), lazy loading, async channels |
| System | `completions`, `doctor` | Implemented |

**1139 tests pass** (`cargo test`). Zero clippy warnings.

**Testing stack:**
- `insta` — Snapshot testing (5 generator outputs)
- `proptest` — Property-based testing (auto-fix invariants + cypher_utils)
- `pretty_assertions` — Colorful diffs
- `cargo-nextest` — Fast parallel test runner (CI)

**Quality tools:**
- `cargo-deny` — License/security policy (`deny.toml`)
- `cargo-audit` — Vulnerability scanning
- `cargo-machete` — Unused dependency detection
- `cargo-llvm-cov` — Coverage reporting
- `cargo-mutants` — Mutation testing
- `cargo-bloat` — Binary size analysis
- `bacon` — Live reload dev experience

**Performance optimizations:**
- `rayon` — Parallel YAML loading (~4x speedup for node types)
- `FxHashSet` — 30% faster string key lookups (TUI collapsed state)
- `SmallVec` — Stack-allocated vectors for properties/labels (avoid heap)

**Security toolchain:**
- `cargo-deny` — License/security policy enforcement (`deny.toml`)
- `cargo-audit` — RustSec vulnerability database scanning
- `cargo-machete` — Unused dependency detection (reduce attack surface)

## Security Workflow

### Pre-Commit

```bash
# Required before every commit touching Cargo.toml or dependencies
cargo deny check

# Expected output: advisories ok, bans ok, licenses ok, sources ok
```

### Dependency Policy

**Allowed licenses:** MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, MPL-2.0, CDLA-Permissive-2.0

**Advisory handling:**
- Direct deps: Fix immediately or document exception
- Transitive deps: Document in `[advisories.ignore]` with reason

### Exception Management

Exceptions are documented in `deny.toml`:

```toml
[advisories]
ignore = [
    "RUSTSEC-2025-0012",  # backoff (neo4rs transitive) - waiting on neo4rs 0.9.0
]
```

**Review schedule:** Quarterly (check if exceptions can be removed)

### Full Security Audit

```bash
# Complete security check
cargo deny check && cargo audit && cargo machete
```

See `/.claude/rules/security.md` for full security guidelines.

## Commands

```bash
# Build
cargo build                                       # Debug build
cargo build --features tui                        # Build with TUI (default)
cargo build --no-default-features                 # CLI-only (no TUI deps)

# Read modes (Neo4j)
cargo run -- blueprint                            # Mode 1: Schema-graph visualization
cargo run -- data                                 # Mode 2: Data nodes only
cargo run -- overlay                              # Mode 3: Data + Schema overlay
cargo run -- query --realm=org --format=json      # Mode 4: Faceted query

# Write operations (Neo4j)
cargo run -- node create --class=Page --key=my-page --props='{"display_name":"My Page"}'
cargo run -- node edit --key=my-page --set='{"description":"Updated"}'
cargo run -- node delete --key=my-page --confirm
cargo run -- arc create --from=page1 --to=entity1 --class=USES_ENTITY

# Search (Neo4j)
cargo run -- search --query="page" --class=Page --limit=20

# Locale (Neo4j)
cargo run -- locale list --format=table
cargo run -- locale import --file=path/to/locale.cypher
cargo run -- locale generate --csv=... --output=...  # Generate 20-locales.cypher

# Knowledge (ATH integration)
cargo run -- knowledge generate --tier=all           # Generate from ATH data
cargo run -- knowledge list                          # List knowledge tiers

# Entity (Phase-based seeding)
cargo run -- entity seed --project=qrcode-ai         # Seed all phases
cargo run -- entity seed --project=qrcode-ai --phase=1  # Seed specific phase
cargo run -- entity list --project=qrcode-ai         # List available phases
cargo run -- entity validate --project=qrcode-ai     # Validate phase data

# Database (Neo4j)
cargo run -- db seed                              # Execute seed Cypher files
cargo run -- db migrate                           # Run migrations
cargo run -- db reset                             # Drop + seed

# Schema (YAML, no Neo4j)
cargo run -- schema generate                      # All 12 artifacts from YAML
cargo run -- schema generate --dry-run            # Preview without writing
cargo run -- schema validate                      # Validate YAML coherence
cargo run -- schema validate --strict             # Fail on warnings

# Documentation (YAML, no Neo4j)
cargo run -- doc generate                         # All 11 view Mermaid diagrams
cargo run -- doc generate --view=block-generation # Single view
cargo run -- doc generate --dry-run               # Preview without writing
cargo run -- doc generate --list                  # List available views

# Filter (Studio subprocess, no Neo4j)
echo '{"realms":["project"]}' | cargo run -- filter build

# Blueprint (YAML, no Neo4j — rich ASCII visualization)
cargo run -- blueprint                            # Default overview with all sections
cargo run -- blueprint --view=tree                # Realm > Layer > Class hierarchy
cargo run -- blueprint --view=flow                # 6 flow diagrams
cargo run -- blueprint --view=arcs                # Arc families with relationships
cargo run -- blueprint --view=stats               # Raw counts (supports --format=json)
cargo run -- blueprint --view=glossary            # Term definitions
cargo run -- blueprint --view=cardinality         # 1:1, 1:N, N:M constraints
cargo run -- blueprint --no-validate              # Skip validation for faster output

# TUI (Neo4j)
cargo run -- tui                                  # Interactive terminal UI

# System utilities
cargo run -- completions bash                     # Generate shell completions
cargo run -- completions zsh                      # (also: fish, powershell, elvish)
cargo run -- doctor                               # System health check
cargo run -- doctor --skip-db                     # Skip Neo4j connectivity check

# Quality
cargo clippy -- -D warnings    # Zero warnings policy
cargo fmt --check              # Formatting check
cargo nextest run              # 1082 tests (fast, parallel)
cargo test -- --ignored        # Neo4j integration tests (requires running Neo4j)

# Security & auditing
cargo deny check               # License/security policy (deny.toml)
cargo audit                    # Vulnerability scanning (RustSec)
cargo machete                  # Unused dependencies

# Code quality analysis
cargo llvm-cov                 # Coverage report
cargo mutants                  # Mutation testing (long)
cargo bloat --release          # Binary size analysis
bacon clippy                   # Live reload clippy

# Pre-commit
cargo fmt && cargo clippy -- -D warnings && cargo nextest run && cargo deny check
```

## Architecture

Module structure:

```
src/
  main.rs         Thin entry point (clap parse + dispatch)
  lib.rs          Public API (re-exports all modules)
  config.rs       Root discovery (resolve_root) + path helpers
  db.rs           Neo4j connection pool (neo4rs::Graph + Arc)
  error.rs        NovaNetError enum (thiserror) + Result type alias
  cypher.rs       CypherStatement builder (data/meta/overlay/query/search)
  facets.rs       FacetFilter (Realm/Layer/Trait/ArcFamily/Class) + JSON serde
  output.rs       OutputFormat (Table/Json/Cypher) + rendering helpers
  commands/
    mod.rs        Module registry
    read.rs       data/meta/overlay/query (CypherStatement → Neo4j → format)
    node.rs       node create/edit/delete (label validation + Cypher)
    arc.rs        arc create/delete (ArcClass validation + Cypher)
    search.rs     search --query (fulltext + property match)
    locale.rs     locale list/import
    db.rs         db seed/migrate/reset (Cypher file execution)
    schema.rs     schema generate/validate (YAML → artifacts)
    doc.rs        doc generate/list (YAML views → Mermaid)
    filter.rs     filter build (JSON stdin → Cypher stdout)
  parsers/        YAML parsers (yaml_node, relations, taxonomy, organizing, views)
  generators/     Code generators (organizing, kind, arc_schema, layer, mermaid, view_mermaid, autowire, hierarchy, colors, icons, visual_encoding, views, tui_icons)
  validation/     Schema validation + auto-fix system
    mod.rs        ValidationEngine + rule registry
    autofix/      AutoFix trait + 6 fixers (FixEngine registry)
      composite_key.rs      Adds pattern to composite key fields
      denormalized_key.rs   Adds denormalized keys for composite nodes
      description.rs        Generates default descriptions
      example_data.rs       Generates type-aware example data
      property_order.rs     Reorders properties to canonical order
      timestamps.rs         Adds created_at/updated_at properties
  tui/            Terminal UI v3 — Unified Tree Architecture (feature-gated)
    mod.rs        Entry point (terminal setup + event loop)
    app.rs        State machine (NavMode: Graph/Nexus, async channels for lazy loading)
    data.rs       UnifiedTree (Realm > Layer > Class > Instance, ArcFamily > ArcClass)
    theme.rs      Visual encoding + Icons (colors from taxonomy.yaml, icons from visual-encoding.yaml)
    ui.rs         3-panel layout (Tree | Info | YAML) + search/help overlays
```

## Key Patterns

- **Error handling**: `thiserror` for `NovaNetError` enum, `color-eyre` in main.rs
- **Neo4j**: `neo4rs::Graph` wrapped in `Arc` (clone freely across tasks)
- **Root discovery**: `--root` flag > `NOVANET_ROOT` env > walk up to `pnpm-workspace.yaml`
- **YAML models**: Live in `packages/core/models/` (relative to monorepo root)
- **Feature gate**: `cargo build --no-default-features` for CLI-only (no TUI deps)
- **YAML-first architecture**: Each Class YAML has explicit `realm:` and `layer:` fields (source of truth)
  - Path validation: file must be at `models/node-classes/{realm}/{layer}/{name}.yaml`
  - Generators read realm/layer from YAML content, validate against path
  - v0.13.1: 2 realms (shared, org), 10 layers (4 shared + 6 org), 61 node types, all with standardized BLOC ordering
- **Query-First architecture (v11.6)**: Cypher is the single source of truth for graph display
  - Schema mode uses CLASSES_QUERY + ARCS_QUERY to build the schema graph
  - `cargo run -- blueprint` executes these foundational queries
  - Views defined in `packages/core/models/views/*.yaml` (no hardcoded TypeScript)
  - See ADR-021 in `.claude/rules/novanet-decisions.md`
- **Icons source of truth (v11.5)**: `visual-encoding.yaml` → `icons:` section
  - Dual format: `web` (Lucide for Studio) + `terminal` (Unicode for TUI)
  - Categories: realms, layers, traits, arc_families, states, navigation, quality, modes
  - TypeScript generated: `packages/core/src/graph/visual-encoding.ts` (ICONS export)
  - Rust compile-time: `tools/novanet/src/tui/icons.rs` (generated constants)
  - Runtime fallback: `Theme::with_root()` loads from YAML with graceful defaults

## Auto-Fix System

**Architecture**: Trait-based auto-fix system with registry pattern for automatically correcting schema validation violations.

### AutoFix Trait

```rust
pub trait AutoFix: Send + Sync {
    fn can_fix(&self, issue: &SchemaIssue) -> bool;
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction>;
    fn description(&self) -> &str;
}
```

### FixEngine Registry

Central registry that applies the first matching fixer for validation issues:

```rust
let engine = FixEngine::default();  // 6 fixers registered
let result = engine.apply_fix(&mut node, &issue)?;

match result {
    FixAction::Modified { changes } => { /* applied */ },
    FixAction::Skipped { reason } => { /* not applicable */ },
}
```

### Registered Fixers

| Fixer | Rule | Action | Example |
|-------|------|--------|---------|
| **CompositeKeyFixer** | COMPOSITE_KEY_FORMAT | Adds `pattern` property to composite key fields | `pattern: "^entity:[a-z0-9-]+@[a-z]{2}-[A-Z]{2}$"` |
| **DenormalizedKeyFixer** | DENORM_REQUIRED | Adds denormalized keys (entity_key, page_key, locale_key) | For EntityNative composite keys |
| **DescriptionFixer** | DESCRIPTION_REQUIRED | Generates contextual description | `"{name} node in the {layer} layer (realm: {realm})"` |
| **ExampleDataFixer** | EXAMPLE_DATA | Generates type-aware example data for required properties | string → "example-{prop}", integer → 1 |
| **PropertyOrderFixer** | PROP_ORDER | Reorders standard_properties to canonical order | key, display_name, description, created_at, updated_at |
| **TimestampFixer** | TIMESTAMP_REQUIRED | Adds created_at/updated_at datetime properties | Required for all nodes |

### Testing Methodology

**TDD RED-GREEN-REFACTOR** cycle:

1. **RED**: Write failing tests (expected behavior, not yet implemented)
2. **GREEN**: Implement minimal code to make tests pass
3. **REFACTOR**: Add property-based tests with `proptest` to verify invariants

**Property-based tests** verify:
- **Correctness**: Required elements always added
- **Idempotence**: Applying fix twice = applying once
- **Identity preservation**: Node metadata (name, realm, layer, trait) unchanged
- **Property preservation**: Existing properties intact

**Coverage**: 52 tests (24 unit + 24 property-based + 4 integration)

### Usage Pattern

```rust
use novanet::validation::autofix::{FixEngine, FixAction};

let mut engine = FixEngine::default();
let result = engine.apply_fix(&mut node, &issue)?;

match result {
    FixAction::Modified { changes } => {
        for change in changes {
            println!("✓ Fixed {}: {:?} → {:?}",
                change.field, change.old_value, change.new_value);
        }
    }
    FixAction::Skipped { reason } => {
        println!("⊘ Skipped: {}", reason);
    }
}
```

### Future Integration

The auto-fix system is ready for integration into `schema validate`:

```bash
cargo run -- schema validate --fix  # Apply automatic fixes (future feature)
```

## TUI Keybindings

See **[KEYBINDINGS.md](./KEYBINDINGS.md)** for complete keyboard shortcuts reference.

Quick summary:
```
Navigation:  j/k (up/down)  h/l (toggle)  d/u (page)
Modes:       [1] Graph (unified tree)  [2] Nexus (Quiz, Audit, Stats, Help)
Scroll:      ENC1 (tree)    ENC2 (yaml)
Overlays:    / (help)       f (search)
Exit:        q or Esc
```

> **Hook**: `.claude/hooks/keybindings-reminder.sh` triggers when `tui/*.rs` is edited.

## Dependencies on Monorepo

This binary reads YAML from `packages/core/models/` (node-classes, arc-classes, taxonomy, views)
and writes to `packages/db/seed/` (Cypher), `packages/core/src/` (TypeScript),
`packages/core/models/docs/` (Mermaid), and `tools/novanet/src/tui/icons.rs` (Rust).
It does NOT depend on any npm packages at build time.

**v11.0 visual encoding**: The `taxonomy.yaml` file is the source of truth for:
- Colors (realms, layers, traits, arc families)
- Border styles (traits: solid/dashed/dotted/double/none)
- Stroke styles (arc families: solid/dashed)
- Unicode characters (box drawing for TUI)

The `organizing` generator reads `taxonomy.yaml` and writes visual encoding properties
to Neo4j via `00.5-taxonomy.cypher`.

## Neo4j

Same credentials as the monorepo:
- **Bolt**: bolt://localhost:7687
- **User**: neo4j
- **Password**: novanetpassword
