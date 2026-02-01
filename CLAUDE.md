# NovaNet

Turborepo monorepo for NovaNet - knowledge graph localization orchestrator.

---

## Overview

NovaNet uses Neo4j to orchestrate **native content generation** (NOT translation) across 200+ locales.

**Target Application**: QR Code AI (https://qrcode-ai.com)
**Current Version**: v8.2.0 (migrating to v9.0.0)
**Migration Plan**: `docs/plans/2026-02-01-ontology-v9-design.md`

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
Axis 4 — LINKS?  :EdgeKind  (47 relationship types in 5 families)
```

**Key renames:** Scope -> Realm, Subcategory -> Layer, NodeTypeMeta -> Kind, DataMode -> NavigationMode

**New concepts:** Trait, EdgeFamily, EdgeKind, OF_KIND instance bridge, :Meta double-label

**Rust binary:** `tools/novanet/` — single crate for CLI + TUI (neo4rs, ratatui, clap)

**Boundary rule:** TypeScript generates code artifacts. Rust executes at runtime.

---

## Monorepo Structure

```
novanet/
├── turbo.json              # Turborepo pipeline config
├── pnpm-workspace.yaml     # Workspace definitions
├── packages/
│   ├── core/               # @novanet/core - types, schemas, filters
│   ├── db/                 # @novanet/db - Neo4j infrastructure
│   └── schema-tools/       # @novanet/schema-tools - TS code generation (eliminated in v9)
├── tools/
│   └── novanet/            # Rust binary (CLI + TUI) — v9+
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

# Schema (TypeScript generators)
pnpm schema:generate       # Regenerate TS + Mermaid from YAML
pnpm schema:validate       # Validate sync (CI check)

# Rust binary (v9+)
cargo run -- data kinds     # List all Kinds
cargo run -- meta realms    # Show Realm hierarchy
cargo run -- schema validate --strict  # Authoritative validation
cargo run -- tui            # Interactive TUI explorer

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
| @novanet/schema-tools | TS code generation (eliminated in v9 by Rust binary) |
| @novanet/studio | Web-based graph visualization |

---

## Dependencies

```
                    @novanet/core
                    ↑           ↑
    @novanet/schema-tools       │
                                │
                         @novanet/studio

@novanet/db (standalone)
tools/novanet (Rust, standalone — reads YAML + Neo4j directly)
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
| **Rust** | `cargo fmt`, `cargo clippy`, edition 2024 |

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
1. /schema:add-node MyNode     # Socratic discovery
   ↓
2. YAML created                # packages/core/models/nodes/.../my-node.yaml
   ↓
3. pnpm schema:generate        # Regenerate TypeScript + Mermaid
   ↓
4. pnpm schema:validate        # Validate sync
   ↓
5. pnpm infra:seed             # Update Neo4j
```
