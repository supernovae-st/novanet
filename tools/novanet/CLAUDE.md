# CLAUDE.md

This file provides guidance to Claude Code when working in the `tools/novanet/` Rust project.

## Overview

`novanet` is a unified Rust CLI + TUI binary for managing the NovaNet context graph.
It replaces the TypeScript `@novanet/schema-tools` and `@novanet/cli` packages.

**Design plan**: `docs/plans/2026-02-01-ontology-v9-design.md` (source of truth)

## Current Status

**Phase 2 complete** — All parsers and generators implemented. `novanet schema generate`
produces 7 artifacts (4 Cypher seeds, 2 TypeScript, 1 Mermaid). `novanet schema validate`
checks YAML coherence. 68 tests pass. Remaining commands (data, meta, query, node, db, tui)
are Phase 7 stubs.

## Commands

```bash
# Build + run
cargo build                    # Debug build
cargo run -- data              # Mode 1: Data nodes only
cargo run -- meta              # Mode 2: Meta-graph only
cargo run -- overlay           # Mode 3: Data + Meta overlay
cargo run -- query --realm=project --format=json  # Mode 4: Faceted query
cargo run -- tui               # Interactive TUI

# Quality
cargo clippy -- -D warnings    # Zero warnings policy
cargo fmt --check              # Formatting check
cargo test                     # Unit + CLI integration tests
cargo test -- --ignored        # Neo4j integration tests (requires running Neo4j)

# Pre-commit
cargo fmt && cargo clippy -- -D warnings && cargo test
```

## Architecture

Module structure:

```
src/
  main.rs         Thin entry point (clap parse + dispatch)
  config.rs       Root discovery (resolve_root) + connection config
  db.rs           Neo4j connection pool (neo4rs + Arc)
  error.rs        NovaNetError enum (thiserror) + Result type alias
  commands/       Subcommand implementations (data, meta, query, node, schema, db)
  parsers/        YAML parser trait + implementations (yaml_node, relations, locale_md)
  generators/     Code generators (organizing, kind, edge_schema, layer, mermaid, autowire, hierarchy)
  tui/            Terminal UI (feature-gated behind `tui` feature)
```

## Key Patterns

- **Error handling**: `thiserror` for `NovaNetError` enum, `color-eyre` in main.rs
- **Neo4j**: `neo4rs::Graph` wrapped in `Arc` (clone freely across tasks)
- **Root discovery**: `--root` flag > `NOVANET_ROOT` env > walk up to `pnpm-workspace.yaml`
- **YAML models**: Live in `packages/core/models/` (relative to monorepo root)
- **Feature gate**: `cargo build --no-default-features` for CLI-only (no TUI deps)

## Dependencies on Monorepo

This binary reads YAML from `packages/core/models/` and writes to `packages/db/seed/`.
It does NOT depend on any npm packages at build time.

## Neo4j

Same credentials as the monorepo:
- **Bolt**: bolt://localhost:7687
- **User**: neo4j
- **Password**: novanetpassword
