# NovaNet

Turborepo monorepo for NovaNet - knowledge graph localization orchestrator.

---

## Overview

NovaNet uses Neo4j to orchestrate **native content generation** (NOT translation) across 200+ locales.

**Target Application**: QR Code AI (https://qrcode-ai.com)
**Current Version**: v9.0.0
**Design Plan**: `docs/plans/2026-02-01-ontology-v9-design.md`

```
CRITICAL: Generation, NOT Translation

Source -> Translate -> Target        <-- WRONG
Concept (invariant) -> Generate natively -> ConceptL10n (local)  <-- RIGHT
```

---

## v9 Migration Context

v9 refactors the meta-graph from a flat tree (Scope > Subcategory > NodeTypeMeta) to a **self-describing context graph** with faceted classification:

```
Axis 1 — WHERE?  :Realm     (global / project / shared)
Axis 2 — WHAT?   :Layer     (9 functional layers)
Axis 3 — HOW?    :Trait     (invariant / localized / knowledge / derived / job)
Axis 4 — LINKS?  :EdgeKind  (50 relationship types in 5 families)
```

**Key renames:** Scope -> Realm, Subcategory -> Layer, NodeTypeMeta -> Kind, DataMode -> NavigationMode

**New concepts:** Trait, EdgeFamily, EdgeKind, OF_KIND instance bridge, :Meta double-label

**Rust binary:** `tools/novanet/` — single crate for CLI + TUI (neo4rs, ratatui, clap).
All commands implemented: data/meta/overlay/query, node/relation CRUD, search, locale, db,
schema generate/validate, doc generate, filter build, TUI scaffold. 195 tests pass.

**Boundary rule (v9 target):** TypeScript generates code artifacts. Rust executes at runtime.

---

## Monorepo Structure

```
novanet/
├── turbo.json              # Turborepo pipeline config
├── pnpm-workspace.yaml     # Workspace definitions
├── packages/
│   ├── core/               # @novanet/core - types, schemas, filters
│   └── db/                 # @novanet/db - Neo4j infrastructure
├── tools/
│   └── novanet/            # Rust CLI + TUI — schema generation, validation, queries
└── apps/
    └── studio/             # @novanet/studio - web visualization
```

---

## Commands

```bash
# Development
pnpm dev                   # Start studio dev server
pnpm build                 # Build all packages
pnpm lint                  # Lint all packages
pnpm type-check            # Type check all packages
pnpm test                  # Test all packages

# Infrastructure (Neo4j)
pnpm infra:up              # Start Neo4j
pnpm infra:down            # Stop Neo4j
pnpm infra:seed            # Seed database
pnpm infra:reset           # Reset database

# Rust binary (tools/novanet) — all commands
cargo run -- schema generate               # Regenerate all artifacts (7 generators)
cargo run -- schema validate               # Validate YAML coherence
cargo run -- doc generate                  # Generate 12 view Mermaid diagrams
cargo run -- doc generate --list           # List available views
cargo run -- data --format=table           # Mode 1: Data nodes
cargo run -- meta --format=json            # Mode 2: Meta-graph
cargo run -- overlay                       # Mode 3: Data + Meta
cargo run -- query --realm=project         # Mode 4: Faceted query
cargo run -- search --query="page"         # Fulltext + property search
cargo run -- node create --kind=Page --key=my-page  # CRUD
cargo run -- db seed                       # Execute seed Cypher files
cargo run -- locale list                   # Locale operations
cargo run -- tui                           # Interactive terminal UI

# Turbo filters
pnpm build --filter=@novanet/core       # Build only core
pnpm test --filter=@novanet/studio      # Test only studio
```

---

## Packages

| Package | Description |
|---------|-------------|
| @novanet/core | Types, schemas, filters, generators |
| @novanet/db | Neo4j Docker, seeds, migrations |
| @novanet/studio | Web-based graph visualization |
| tools/novanet | Rust CLI + TUI — all runtime commands (195 tests) |

---

## Dependencies

```
                    @novanet/core
                          ↑
                          │
                   @novanet/studio

@novanet/db (standalone)
tools/novanet (Rust, standalone — reads YAML, writes Cypher/TS/Mermaid)
```

---

## Neo4j

- **Browser**: http://localhost:7474
- **Bolt**: bolt://localhost:7687
- **Credentials**: `neo4j` / `novanetpassword`

---

## Quick Start

```bash
# 1. Clone
git clone git@github.com:supernovae-st/novanet-hq.git
cd novanet-hq

# 2. Install (requires pnpm)
pnpm install

# 3. Start Neo4j + seed
pnpm infra:up
pnpm infra:seed

# 4. Start development
pnpm dev    # → http://localhost:3000
```

---

## Conventions

| Aspect | Convention |
|--------|------------|
| **Package Manager** | pnpm (required) |
| **Build Tool** | Turborepo |
| **Naming** | `novanet` (packages), `NovaNet` (classes/types) |
| **Formatting** | 2 spaces, 100 chars, single quotes, semicolons |
| **Commits** | Conventional Commits |
| **Rust (v9)** | `cargo fmt`, `cargo clippy`, edition 2024 |

---

## Claude Code DX

See `.claude/README.md` for full documentation.

### Key Commands

| Command | Description |
|---------|-------------|
| `/novanet-arch` | Architecture diagrams (ASCII) |
| `/novanet-sync` | Schema validation/regeneration |
| `/schema:add-node <name>` | Add new node type |
| `/schema:edit-node <name>` | Modify existing node |
| `/schema:add-relation <REL>` | Add new relationship |

### Schema Management Workflow

```
1. /schema:add-node MyNode             # Socratic discovery
   ↓
2. YAML created                        # packages/core/models/nodes/.../my-node.yaml
   ↓
3. cargo run -- schema generate        # Regenerate all artifacts from YAML
   ↓
4. cargo run -- schema validate        # Validate YAML coherence
   ↓
5. cargo run -- db seed                # Update Neo4j
```
