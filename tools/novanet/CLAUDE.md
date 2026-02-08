# CLAUDE.md

This file provides guidance to Claude Code when working in the `tools/novanet/` Rust project.

## Overview

`novanet` is a unified Rust CLI + TUI binary for managing the NovaNet context graph.
It replaces the TypeScript `@novanet/schema-tools` and `@novanet/cli` packages.

**Version**: v11.0.0 (SEO tenant migration + v10.9 naming refactor)

## Current Status

**v11.0.0 SEO tenant migration** — Moved 9 SEO/GEO nodes from `global/seo` to `tenant/seo` (ADR-012 fix). Fixed 22 arc scopes (cross_realm → intra_realm). Architecture: GLOBAL (2 layers: config, locale-knowledge), TENANT (7 layers: +seo). Galaxy-themed mission control TUI with search, detail, arc explorer, CRUD dialogs, dashboard stats, ASCII logo, breadcrumb navigation, command palette, help overlay, boot animation, effects engine, and onboarding.

| Area | Commands | Status |
|------|----------|--------|
| Read | `data`, `meta`, `overlay`, `query` | Implemented (faceted Cypher) |
| Write | `node create/edit/delete`, `arc create/delete` | Implemented (label validation) |
| Schema | `schema generate`, `schema validate` | Implemented (12 artifacts) |
| Docs | `doc generate`, `doc generate --list` | Implemented (11 views) |
| Search | `search --query=... [--kind=...]` | Implemented (fulltext + property) |
| Locale | `locale list`, `locale import` | Implemented |
| DB | `db seed`, `db migrate`, `db reset` | Implemented |
| Filter | `filter build` | Implemented (JSON stdin, Studio subprocess) |
| TUI | `tui` | Galaxy theme, mission control, search, detail, arc explorer, CRUD dialogs, dashboard, logo, command palette, help overlay, boot animation, effects engine, onboarding |

**490 tests pass** (`cargo test`). Zero clippy warnings.

**Testing stack:**
- `insta` — Snapshot testing (5 generator outputs)
- `proptest` — Property-based testing (cypher_utils edge cases)
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
cargo run -- meta                                 # Mode 1: Meta-graph only
cargo run -- data                                 # Mode 2: Data nodes only
cargo run -- overlay                              # Mode 3: Data + Meta overlay
cargo run -- query --realm=tenant --format=json   # Mode 4: Faceted query

# Write operations (Neo4j)
cargo run -- node create --kind=Page --key=my-page --props='{"display_name":"My Page"}'
cargo run -- node edit --key=my-page --set='{"description":"Updated"}'
cargo run -- node delete --key=my-page --confirm
cargo run -- arc create --from=page1 --to=entity1 --kind=USES_ENTITY

# Search (Neo4j)
cargo run -- search --query="page" --kind=Page --limit=20

# Locale (Neo4j)
cargo run -- locale list --format=table
cargo run -- locale import --file=path/to/locale.cypher

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

# TUI (Neo4j)
cargo run -- tui                                  # Interactive terminal UI

# Quality
cargo clippy -- -D warnings    # Zero warnings policy
cargo fmt --check              # Formatting check
cargo nextest run              # 482 tests (fast, parallel)
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
  facets.rs       FacetFilter (Realm/Layer/Trait/ArcFamily/Kind) + JSON serde
  output.rs       OutputFormat (Table/Json/Cypher) + rendering helpers
  commands/
    mod.rs        Module registry
    read.rs       data/meta/overlay/query (CypherStatement → Neo4j → format)
    node.rs       node create/edit/delete (label validation + Cypher)
    arc.rs        arc create/delete (ArcKind validation + Cypher)
    search.rs     search --query (fulltext + property match)
    locale.rs     locale list/import
    db.rs         db seed/migrate/reset (Cypher file execution)
    schema.rs     schema generate/validate (YAML → artifacts)
    doc.rs        doc generate/list (YAML views → Mermaid)
    filter.rs     filter build (JSON stdin → Cypher stdout)
  parsers/        YAML parsers (yaml_node, relations, taxonomy, organizing, views)
  generators/     Code generators (organizing, kind, arc_schema, layer, mermaid, view_mermaid, autowire, hierarchy, colors, icons, visual_encoding, views, tui_icons)
  tui/            Terminal UI v2 — rebuilt for stability (feature-gated)
    mod.rs        Entry point (terminal setup + event loop)
    app.rs        State machine (NavMode, Focus, tree/yaml scroll, collapse state)
    data.rs       TaxonomyTree (Realm > Layer > Kind + ArcFamily > ArcKind)
    theme.rs      Visual encoding + Icons (colors from taxonomy.yaml, icons from visual-encoding.yaml)
    ui.rs         3-panel layout (Tree | Info | YAML) + search/help overlays
```

## Key Patterns

- **Error handling**: `thiserror` for `NovaNetError` enum, `color-eyre` in main.rs
- **Neo4j**: `neo4rs::Graph` wrapped in `Arc` (clone freely across tasks)
- **Root discovery**: `--root` flag > `NOVANET_ROOT` env > walk up to `pnpm-workspace.yaml`
- **YAML models**: Live in `packages/core/models/` (relative to monorepo root)
- **Feature gate**: `cargo build --no-default-features` for CLI-only (no TUI deps)
- **YAML-first architecture**: Each Kind YAML has explicit `realm:` and `layer:` fields (source of truth)
  - Path validation: file must be at `models/node-kinds/{realm}/{layer}/{name}.yaml`
  - Generators read realm/layer from YAML content, validate against path
  - v11.0: 2 realms (global, tenant), 9 layers (2 global + 7 tenant), 64 node types total
- **Icons source of truth (v11.0)**: `visual-encoding.yaml` → `icons:` section
  - Dual format: `web` (Lucide for Studio) + `terminal` (Unicode for TUI)
  - Categories: realms, layers, traits, arc_families, states, navigation, quality, modes
  - TypeScript generated: `packages/core/src/graph/visual-encoding.ts` (ICONS export)
  - Rust compile-time: `tools/novanet/src/tui/icons.rs` (generated constants)
  - Runtime fallback: `Theme::with_root()` loads from YAML with graceful defaults

## TUI Keybindings

See **[KEYBINDINGS.md](./KEYBINDINGS.md)** for complete keyboard shortcuts reference.

Quick summary:
```
Navigation:  j/k (up/down)  h/l (toggle)  d/u (page)  1-4 (modes)
Scroll:      ENC1 (tree)    ENC2 (yaml)
Overlays:    / (help)       f (search)
Exit:        q or Esc
```

> **Hook**: `.claude/hooks/keybindings-reminder.sh` triggers when `tui/*.rs` is edited.

## Dependencies on Monorepo

This binary reads YAML from `packages/core/models/` (node-kinds, arc-kinds, taxonomy, views)
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
