# CLAUDE.md

This file provides guidance to Claude Code when working in the `tools/novanet/` Rust project.

## Overview

`novanet` is a unified Rust CLI + TUI binary for managing the NovaNet context graph.
It replaces the TypeScript `@novanet/schema-tools` and `@novanet/cli` packages.

**Design plan**: `docs/plans/2026-02-01-ontology-v9-design.md` (source of truth)

## Current Status

**Phase 7B Batch 7 complete** — Galaxy-themed mission control TUI with search, detail, edge explorer, CRUD dialogs, dashboard stats, ASCII logo, breadcrumb navigation, command palette, help overlay, boot animation (matrix rain + logo reveal), effects engine (CRT scanlines, glitch transitions, nebula pulse, screen shake), typewriter effect, and first-run onboarding (welcome screen + guided tour).

| Area | Commands | Status |
|------|----------|--------|
| Read | `data`, `meta`, `overlay`, `query` | Implemented (faceted Cypher) |
| Write | `node create/edit/delete`, `relation create/delete` | Implemented (label validation) |
| Schema | `schema generate`, `schema validate` | Implemented (7 artifacts) |
| Docs | `doc generate`, `doc generate --list` | Implemented (12 views) |
| Search | `search --query=... [--kind=...]` | Implemented (fulltext + property) |
| Locale | `locale list`, `locale import` | Implemented |
| DB | `db seed`, `db migrate`, `db reset` | Implemented |
| Filter | `filter build` | Implemented (JSON stdin, Studio subprocess) |
| TUI | `tui` | Galaxy theme, mission control, search, detail, edge explorer, CRUD dialogs, dashboard, logo, command palette, help overlay, boot animation, effects engine, onboarding |

**396 tests pass** (`cargo test`). Zero clippy warnings.

## Commands

```bash
# Build
cargo build                                       # Debug build
cargo build --features tui                        # Build with TUI (default)
cargo build --no-default-features                 # CLI-only (no TUI deps)

# Read modes (Neo4j)
cargo run -- data                                 # Mode 1: Data nodes only
cargo run -- meta                                 # Mode 2: Meta-graph only
cargo run -- overlay                              # Mode 3: Data + Meta overlay
cargo run -- query --realm=project --format=json  # Mode 4: Faceted query

# Write operations (Neo4j)
cargo run -- node create --kind=Page --key=my-page --props='{"display_name":"My Page"}'
cargo run -- node edit --key=my-page --set='{"description":"Updated"}'
cargo run -- node delete --key=my-page --confirm
cargo run -- relation create --from=page1 --to=concept1 --type=USES_CONCEPT

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
cargo run -- schema generate                      # All 7 artifacts from YAML
cargo run -- schema generate --dry-run            # Preview without writing
cargo run -- schema validate                      # Validate YAML coherence
cargo run -- schema validate --strict             # Fail on warnings

# Documentation (YAML, no Neo4j)
cargo run -- doc generate                         # All 12 view Mermaid diagrams
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
cargo test                     # 396 unit tests
cargo test -- --ignored        # Neo4j integration tests (requires running Neo4j)

# Pre-commit
cargo fmt && cargo clippy -- -D warnings && cargo test
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
  facets.rs       FacetFilter (Realm/Layer/Trait/EdgeFamily/Kind) + JSON serde
  output.rs       OutputFormat (Table/Json/Cypher) + rendering helpers
  commands/
    mod.rs        Module registry
    read.rs       data/meta/overlay/query (CypherStatement → Neo4j → format)
    node.rs       node create/edit/delete (label validation + Cypher)
    relation.rs   relation create/delete (rel type validation + Cypher)
    search.rs     search --query (fulltext + property match)
    locale.rs     locale list/import
    db.rs         db seed/migrate/reset (Cypher file execution)
    schema.rs     schema generate/validate (YAML → artifacts)
    doc.rs        doc generate/list (YAML views → Mermaid)
    filter.rs     filter build (JSON stdin → Cypher stdout)
  parsers/        YAML parsers (yaml_node, relations, organizing, views)
  generators/     Code generators (organizing, kind, edge_schema, layer, mermaid, view_mermaid, autowire, hierarchy)
  tui/            Terminal UI (feature-gated behind `tui` feature)
    app.rs        State machine (NavMode, AppState, ActivePanel, onboarding)
    tree.rs       TaxonomyTree (Realm > Layer > Kind hierarchy + cursor + parent_of)
    events.rs     Keyboard handling (Action dispatch + overlays: dialog > onboarding > palette > help > search)
    ui.rs         Galaxy-themed mission control layout + Cypher syntax highlighting + overlays
    runtime.rs    Async event loop (crossterm + mpsc channel bridge + adaptive tick + boot transitions)
    theme.rs      SuperNovae Galaxy palette + style helpers (realm/layer/family colors)
    detail.rs     KindDetail struct + Neo4j fetch + styled/explorer rendering
    search.rs     Fuzzy search overlay (nucleo-matcher + SearchState)
    dialogs.rs    CRUD dialog forms (create/edit/delete node + create/delete relation)
    dashboard.rs  Dashboard stats (Neo4j metrics, realm/family bar charts)
    logo.rs       ASCII logo (Saturn-graph full/compact/inline + Galaxy-themed colors)
    palette.rs    Command palette (fuzzy search, 11 commands, 5 categories)
    boot.rs       Boot sequence (6-stage animation: matrix rain → logo reveal → fade)
    effects.rs    Effects engine (CRT scanlines, glitch, screen shake, nebula pulse, typewriter)
    onboarding.rs First-run detection, welcome screen, guided tour (5 steps)
```

## Key Patterns

- **Error handling**: `thiserror` for `NovaNetError` enum, `color-eyre` in main.rs
- **Neo4j**: `neo4rs::Graph` wrapped in `Arc` (clone freely across tasks)
- **Root discovery**: `--root` flag > `NOVANET_ROOT` env > walk up to `pnpm-workspace.yaml`
- **YAML models**: Live in `packages/core/models/` (relative to monorepo root)
- **Feature gate**: `cargo build --no-default-features` for CLI-only (no TUI deps)

## Dependencies on Monorepo

This binary reads YAML from `packages/core/models/` (nodes, relations, organizing-principles, views)
and writes to `packages/db/seed/` (Cypher), `packages/core/src/` (TypeScript), and
`packages/core/models/docs/` (Mermaid). It does NOT depend on any npm packages at build time.

## Neo4j

Same credentials as the monorepo:
- **Bolt**: bolt://localhost:7687
- **User**: neo4j
- **Password**: novanetpassword
