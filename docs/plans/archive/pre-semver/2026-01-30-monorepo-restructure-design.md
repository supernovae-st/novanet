# NovaNet Monorepo Restructure Design

> **Partially superseded by v9**: The monorepo structure is correct, but `@novanet/cli` and
> `@novanet/schema-tools` are eliminated in v9. Replaced by single `novanet` Rust binary at
> `tools/novanet/`. See [`2026-02-01-ontology-v9-design.md`](2026-02-01-ontology-v9-design.md).

> Generated: 2026-01-30
> Status: Partially superseded (v9 Rust-first architecture)

## Problem Statement

Current structure has unclear responsibilities:
- `novanet-core/` mixes runtime library + build tools + neo4j seeds
- `novanet-infra/` is a separate git repo for just docker-compose
- `novanet-studio/` is isolated but depends on core
- No clear path for future CLI/TUI tools

## Goals

1. **Clear separation**: runtime library vs dev tools vs database vs apps
2. **Unified git**: single monorepo, no nested .git
3. **Shared design system**: colors/tokens used by studio AND future TUI
4. **Rust-ready**: architecture supports future Rust CLI/TUI migration

## Final Structure

```
novanet-hq/
├── .git/                         # SINGLE git repo
├── .claude/
├── CLAUDE.md                     # Monorepo-level instructions
├── package.json                  # workspaces: ["packages/*", "apps/*"]
├── Cargo.toml                    # (future) Rust workspace
│
├── packages/                     # Libraries & dev tools
│   │
│   ├── core/                     # @novanet/core - SHARED RUNTIME
│   │   ├── .claude/skills/       # neo4j-expert, context-graph-architect
│   │   ├── CLAUDE.md
│   │   ├── package.json          # name: "@novanet/core"
│   │   ├── src/
│   │   │   ├── types/            # NodeType, FilterCriteria, ViewDefinition
│   │   │   ├── filters/          # NovaNetFilter, CypherGenerator
│   │   │   ├── schemas/          # Zod validation
│   │   │   └── design/           # colors.ts, tokens.ts (shared)
│   │   ├── models/               # YAML SOURCE OF TRUTH
│   │   │   ├── nodes/
│   │   │   ├── views/
│   │   │   ├── docs/
│   │   │   ├── _index.yaml
│   │   │   └── relations.yaml
│   │   └── __tests__/
│   │
│   ├── cli/                      # @novanet/cli - DEV TOOLS
│   │   ├── CLAUDE.md
│   │   ├── package.json          # name: "@novanet/cli", bin: "novanet"
│   │   ├── src/
│   │   │   ├── commands/         # validate, generate, seed, inspect
│   │   │   └── generators/       # Mermaid, Markdown, Cypher exporters
│   │   └── __tests__/
│   │
│   └── db/                       # @novanet/db - DATABASE
│       ├── CLAUDE.md
│       ├── package.json          # name: "@novanet/db"
│       ├── docker-compose.yml    # ← from novanet-infra
│       ├── seed/                 # ← from novanet-core/neo4j/seed
│       ├── queries/              # ← from novanet-core/neo4j/queries
│       ├── migrations/           # ← from novanet-core/neo4j/migrations
│       └── scripts/              # seed.sh, reset.sh
│
├── apps/                         # End-user applications
│   │
│   ├── studio/                   # @novanet/studio - WEB UI
│   │   ├── .claude/skills/
│   │   ├── CLAUDE.md
│   │   ├── package.json          # name: "@novanet/studio"
│   │   └── src/                  # Next.js app (unchanged)
│   │
│   └── tui/                      # @novanet/tui - TERMINAL UI (future)
│       ├── CLAUDE.md
│       ├── package.json          # name: "@novanet/tui"
│       └── src/                  # Ink/React or Rust ratatui
│
├── crates/                       # (future) Rust packages
│   ├── novanet-types/            # Generated from YAML
│   ├── novanet-cli/              # CLI commands in Rust
│   └── novanet-tui/              # Terminal UI with ratatui
│
└── docs/
    ├── architecture/
    └── plans/
```

## Package Dependencies

```
                ┌─────────────────┐
                │  @novanet/core  │
                │  types, colors  │
                │  filters, models│
                └────────┬────────┘
       ┌─────────────────┼─────────────────┐
       ▼                 ▼                 ▼
┌─────────────┐   ┌─────────────┐   ┌─────────────┐
│@novanet/cli │   │ @novanet/db │   │@novanet/    │
│ generators  │──▶│   seeds     │   │  studio     │
│ commands    │   │   queries   │   │  Next.js    │
└─────────────┘   └─────────────┘   └─────────────┘
                                           │
                                    ┌──────┴──────┐
                                    │@novanet/tui │
                                    │  (future)   │
                                    └─────────────┘
```

## Git Strategy

### Current State
- `novanet-hq/.git` - monorepo (KEEP)
- `novanet-core/.git` - nested (REMOVE)
- `novanet-studio/.git` - nested (REMOVE)
- `novanet-infra/.git` - separate repo (REMOVE, merge content)
- `novanet-public-repo/.git` - public GitHub (KEEP SEPARATE)

### Migration
1. Merge histories with `git subtree` or simple move
2. Remove nested `.git` directories
3. Single remote: `origin → github.com/supernovae-st/novanet-hq`

## Claude Setup

### Hierarchy
```
novanet-hq/CLAUDE.md              # Global monorepo conventions
    ↓ inherits
packages/core/CLAUDE.md           # Models, types, Neo4j specifics
packages/cli/CLAUDE.md            # Commands, generators
packages/db/CLAUDE.md             # Docker, seeds, Cypher
apps/studio/CLAUDE.md             # React, Next.js, Zustand
apps/tui/CLAUDE.md                # Terminal UI specifics
```

### Skills Location
- `packages/core/.claude/skills/` - neo4j-expert, context-graph-architect, spreading-activation
- `apps/studio/.claude/skills/` - React/visualization skills

## Rust Migration Path

### Phase 1 (Now)
- Restructure to packages/ + apps/
- CLI/TUI in TypeScript for fast iteration

### Phase 2 (When TUI becomes priority)
- Add `crates/` directory
- Create YAML → Rust type generator
- Rewrite CLI/TUI in Rust with ratatui
- Keep TypeScript for studio (web)

### Why Rust for CLI/TUI
- **CLI**: Instant startup, single binary, no npm install
- **TUI**: ratatui >> Ink, native terminal rendering, smooth animations
- **Both**: Low memory, cross-platform binaries

### Architecture Support
YAML models are the source of truth (not TypeScript), enabling:
```
models/*.yaml
     │
     ├──────────────────┐
     ▼                  ▼
generate TS        generate Rust
(for studio)       (for tui/cli)
```

## Migration Tasks

### Phase 1: Restructure (Now)
1. Create `packages/` and `apps/` directories
2. Move `novanet-core/` → `packages/core/`
3. Move `novanet-studio/` → `apps/studio/`
4. Create `packages/cli/` from core's scripts/generators
5. Create `packages/db/` from core's neo4j/ + infra's docker-compose
6. Remove nested `.git` directories
7. Update package.json workspaces
8. Update all import paths
9. Update CLAUDE.md files
10. Test everything works

### Phase 2: TUI (Future)
1. Create `apps/tui/` with basic Ink setup
2. Share design tokens from core
3. Build graph visualization in terminal

### Phase 3: Rust (Future)
1. Add `Cargo.toml` workspace
2. Create `crates/novanet-types/` from YAML
3. Migrate CLI to Rust
4. Migrate TUI to Rust with ratatui

---

*Design validated on 2026-01-30*
