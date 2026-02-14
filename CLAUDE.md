# NovaNet

Turborepo monorepo for NovaNet - knowledge graph localization orchestrator.

## Auto-Imported Context

@README.md @ROADMAP.md @CHANGELOG.md

---

## Why NovaNet Exists

**Problem**: Scaling content across 200+ locales is prohibitively expensive with traditional translation.
- Translation loses cultural nuance (idioms, humor, formality levels)
- Cost grows linearly: 200 locales = 200× translation cost
- Maintaining translation databases creates synchronization nightmares

**Solution**: NovaNet generates content **natively** per locale from defined semantic entities.
- LLM generates in target locale with cultural context (LocaleVoice, LocaleCulture)
- Entity definitions are written once; content is generated 200×
- Knowledge atoms (Terms, Expressions, Patterns) provide locale-specific vocabulary

**Result**: Native-quality content at a fraction of translation cost, with consistent brand voice across all locales.

---

## Overview

NovaNet uses Neo4j to orchestrate **native content generation** (NOT translation) across 200+ locales.

**Target Application**: QR Code AI (https://qrcode-ai.com)
**Current Version**: v0.12.0 "Class Act" (ADR-023 + ADR-024)
**Roadmap**: `ROADMAP.md` | **Changelog**: `CHANGELOG.md`

**Related docs**:
- `.claude/rules/novanet-decisions.md` — Architecture decisions (ADR-001 through ADR-025)
- `.claude/rules/novanet-terminology.md` — Canonical terminology reference

```
CRITICAL: Generation, NOT Translation

Source -> Translate -> Target        <-- WRONG
Entity (defined) -> Generate natively -> EntityContent (authored)  <-- RIGHT
```

---

## v11.5 Nomenclature

v11.5 refines the layer structure with Locale moved to shared/config:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  VOCABULARY                                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│  Level           │ Vertex    │ Edge     │                                   │
│  ─────────────────────────────────────────                                  │
│  General         │ Node      │ Arc      │                                   │
│  Instance (data) │ NodeData  │ ArcData  │                                   │
│  Definition      │ NodeClass │ ArcClass │                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│  CLASSIFICATION AXES                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│  NodeClass:                                                                 │
│    WHERE?  NodeRealm  (shared / org)                                        │
│    WHAT?   NodeLayer  (10 layers: 4 shared + 6 org)                         │
│    HOW?    NodeTrait  (defined / authored / imported / generated / retrieved) │
│                                                                             │
│  ArcClass:                                                                  │
│    SCOPE   ArcScope       (intra_realm / cross_realm)                       │
│    FUNC    ArcFamily      (ownership / localization / semantic / gen / min) │
│    MULT    ArcCardinality (1:1 / 1:N / N:M)                                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Key changes in v0.12.4:**
- **Brand Architecture**: 4 new nodes (Brand, BrandDesign, BrandPrinciples, PromptStyle)
- **Country added**: shared/geography now has 7 nodes (Country added)
- **ADR-028**: PageStructure/PageInstruction removed, REFERENCES/HAS_KEYWORD arcs added
- **61 nodes** total: 40 shared + 21 org, **156 arcs** (5 families)

**Architecture (v0.12.4):**
- 2 realms: SHARED + ORG
- SHARED (4 layers): config, locale, geography, knowledge — universal, READ-ONLY (40 nodes)
- ORG (6 layers): config, foundation, structure, semantic, instruction, output (21 nodes)

**Rust binary:** `tools/novanet/` — single crate for CLI + TUI (neo4rs, ratatui, clap).
All commands implemented: blueprint/data/overlay/query, node/arc CRUD, search, locale, db,
schema generate/validate, doc generate, filter build. Galaxy-themed TUI with unified tree mode (v11.7), boot animation, effects engine, Nexus hub, and onboarding. 980 tests pass.

**YAML-first architecture:** Each Class YAML has explicit `realm:` and `layer:` fields (source of truth).
Path validation ensures `models/node-classes/{realm}/{layer}/{name}.yaml` matches YAML content.
v0.12.4: 2 realms (shared, org), 10 layers total (4 shared + 6 org), 61 nodes, 156 arcs.

**Icons source of truth (v11.5):** `visual-encoding.yaml` → `icons:` section provides dual-format icons:
- `web`: Lucide icon name for Studio
- `terminal`: Unicode symbol for TUI
Categories: realms, layers, traits, arc_families, states, navigation, quality, modes.

**Boundary rule:** TypeScript generates code artifacts. Rust executes at runtime.

---

## v11.7 Unified Tree Architecture

v11.7 introduces the Unified Tree where Realm, Layer, ArcFamily, ArcClass are all clickable nodes.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  UNIFIED TREE PRINCIPLE                                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  "If it's a node in Neo4j, it's a node everywhere"                          │
│                                                                             │
│  Before v11.7:                                                              │
│    Tree: Realm (label) > Layer (label) > Class (clickable)                  │
│    5 modes: Meta, Data, Overlay, Query, Atlas                               │
│                                                                             │
│  After v11.7:                                                               │
│    Tree: Realm (node) > Layer (node) > Class (node) > Instance (lazy)       │
│    2 modes: [1] Graph + [2] Nexus                                           │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  KEY CHANGES                                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. EVERYTHING IS A NODE                                                    │
│     └─ Realm, Layer, ArcFamily, ArcClass are clickable with detail panels   │
│     └─ Consistent UX: click any node → see properties, arcs, actions        │
│                                                                             │
│  2. 5 MODES → 2 MODES                                                       │
│     └─ [1] Graph: Unified tree with lazy instance loading                   │
│     └─ [2] Nexus: Hub for Quiz, Audit, Stats, Help                          │
│                                                                             │
│  3. LAZY INSTANCE LOADING                                                   │
│     └─ Class nodes expand to show first 10 instances + "load more"          │
│     └─ No upfront loading of all instances                                  │
│                                                                             │
│  4. DUAL ICONS (NO EMOJI)                                                   │
│     └─ Web: Lucide icons                                                    │
│     └─ Terminal: Unicode symbols                                            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Architecture:**
- **Graph mode** (`[1]`): Unified tree with Realm > Layer > Class > Instance hierarchy
- **Nexus mode** (`[2]`): Hub for Nexus Quiz, codebase audit, stats dashboard, help

**TUI implementation:** `tools/novanet/src/tui/` — unified tree replaces multi-mode navigation

---

## Knowledge Atoms Architecture

**Knowledge Atoms** provide granular knowledge nodes for selective LLM context loading.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  KNOWLEDGE ARCHITECTURE                                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Locale ──[:HAS_TERMS]──> TermSet ──[:CONTAINS_TERM]──> Term (atom)        │
│          ──[:HAS_EXPRESSIONS]──> ExpressionSet ──[:CONTAINS_EXPRESSION]──> Expression  │
│          ──[:HAS_PATTERNS]──> PatternSet ──[:CONTAINS_PATTERN]──> Pattern           │
│          ──[:HAS_CULTURE]──> CultureSet ──[:CONTAINS_CULTURE_REF]──> CultureRef         │
│          ──[:HAS_TABOOS]──> TabooSet ──[:CONTAINS_TABOO]──> Taboo                 │
│          ──[:HAS_AUDIENCE]──> AudienceSet ──[:CONTAINS_AUDIENCE_TRAIT]──> AudienceTrait    │
│          ──[:HAS_CATEGORIES]──> CategorySet ──[:CONTAINS_CATEGORY]──> EntityCategory     │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  KEY PRINCIPLES                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. CONTAINERS ARE EMPTY                                                    │
│     └─ Pure grouping nodes (no JSON blobs)                                  │
│     └─ Only property: grouping identifier (domain, register, etc.)          │
│     └─ All data lives in atoms                                              │
│                                                                             │
│  2. ATOMS ARE LOCALE-NATIVE                                                 │
│     └─ Unlike Entities (defined + Content for ALL locales)                  │
│     └─ Atoms exist only where needed: fr-FR may have 20K Terms              │
│     └─ sw-KE may have 500 Terms - no translation, native generation         │
│                                                                             │
│  3. SELECTIVE LLM LOADING                                                   │
│     └─ Load 50 relevant Terms, not 20K JSON blob                            │
│     └─ Graph queries: "Terms used by this Block"                            │
│     └─ [:USES_TERM], [:USES_EXPRESSION] on Block nodes                      │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  STATISTICS (v0.12.4)                                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Containers (6): TermSet, ExpressionSet, PatternSet,                        │
│                  CultureSet, TabooSet, AudienceSet                          │
│  Atoms (6):      Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait│
│  Total:          61 nodes (40 shared + 21 org)                              │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**No backward compatibility needed** - this is v0, design for clean architecture.

---

## Query-First Architecture (v11.6)

NovaNet Studio uses **Query-First Architecture** where Cypher is the single source of truth for graph visualization.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  QUERY-FIRST FLOW                                                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ViewPicker ──▶ viewStore.executeView() ──▶ /api/views/:id/query ──▶ Neo4j │
│       │                    │                        │                  │    │
│       ▼                    ▼                        ▼                  ▼    │
│  QueryPill ◀── queryStore.setQuery() ◀── YAML cypher ◀── Results (nodes)   │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  CORE PRINCIPLES                                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. Cypher Query = Source of Truth (graph displays query results only)      │
│  2. YAML Views = Single Definition Source (no hardcoded queries in TS)      │
│  3. Auto-Execute with Edit Option (click = run, Ctrl+click = edit first)   │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  SCHEMA VIEW QUERIES                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  CLASSES_QUERY: MATCH (c:Class) RETURN c.name, c.realm, c.layer, c.trait   │
│  ARCS_QUERY:    MATCH (a:ArcClass) RETURN a.name, a.family, a.scope, ...   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**View categories**: `global` (full graph), `contextual` (node-specific), `generation` (AI agent), `mining` (SEO/GEO)

**Reference**: ADR-021 in `.claude/rules/novanet-decisions.md`

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
cargo run -- schema generate               # Regenerate all artifacts (12 generators)
cargo run -- schema validate               # Validate YAML coherence
cargo run -- doc generate                  # Generate 12 view Mermaid diagrams
cargo run -- doc generate --list           # List available views
cargo run -- blueprint --format=json       # Schema-graph visualization
cargo run -- data --format=table           # Mode 2: Data nodes
cargo run -- overlay                       # Mode 3: Data + Schema
cargo run -- query --realm=org             # Mode 4: Faceted query
cargo run -- search --query="page"         # Fulltext + property search
cargo run -- node create --class=Page --key=my-page  # CRUD
cargo run -- db seed                       # Execute seed Cypher files
cargo run -- locale list                   # Locale operations
cargo run -- tui                           # Interactive TUI (unified tree + Nexus hub)

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
| tools/novanet | Rust CLI + TUI — all runtime commands (985 tests) |

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

## Learning Path (New Developers)

1. **Read this file** — Understand the generation philosophy (not translation)
2. **Explore TUI** — `cargo run -- tui` in `tools/novanet/` for unified tree exploration (v11.7)
3. **Read `models/_index.yaml`** — Complete schema overview with all 61 nodes
4. **Study `taxonomy.yaml`** — Realm/Layer/Trait definitions with visual encoding
5. **Check ADRs** — `.claude/rules/novanet-decisions.md` explains WHY decisions were made
6. **Run Studio** — `pnpm dev` and explore the graph visually at http://localhost:3000

**Key concepts progression**: Realm → Layer → Trait → Class → Arc → ArcFamily

---

## Conventions

| Aspect | Convention |
|--------|------------|
| **Package Manager** | pnpm (required) |
| **Build Tool** | Turborepo |
| **Naming** | `novanet` (packages), `NovaNet` (classes/types) |
| **Formatting** | 2 spaces, 100 chars, single quotes, semicolons |
| **Commits** | Conventional Commits: `type(scope): description` |
| **Versioning** | SemVer: `MAJOR.MINOR.PATCH[-prerelease]` |
| **Rust (v9)** | `cargo fmt`, `cargo clippy`, edition 2024 |

---

## Security Compliance

**Multi-layer security toolchain:**

| Layer | Tool | Command | CI Integration |
|-------|------|---------|----------------|
| **Rust deps** | cargo-deny | `cargo deny check` | Yes (required) |
| **Rust vulns** | cargo-audit | `cargo audit` | Yes (weekly) |
| **Rust unused** | cargo-machete | `cargo machete` | Manual |
| **TS deps** | pnpm audit | `pnpm audit --audit-level=moderate` | Yes |
| **Secrets** | TruffleHog | CI only | Yes (on PR) |

**Pre-commit checklist** (see `.claude/rules/security.md`):
- Rust: `cargo deny check && cargo clippy -- -D warnings`
- TypeScript: `pnpm audit` passes
- No credentials in source code

**Run full audit:** `/security-audit all`

**Exception policy:** Document in `deny.toml` with RUSTSEC ID, reason, and quarterly review date.

---

## Versioning & Releases

**Strategy**: Semantic Versioning (SemVer) with Conventional Commits.

```
MAJOR  = Breaking changes (ontology restructure, API changes)
MINOR  = New features (commands, UI components, generators)
PATCH  = Bug fixes, documentation, refactoring
```

**Release process**:
1. Ensure `pnpm test`, `pnpm lint`, `pnpm type-check`, `cargo test` all pass
2. Run `./tools/scripts/release-notes.sh <version>` to generate notes
3. Update `CHANGELOG.md`
4. Create annotated tag: `git tag -a v<version> -m "v<version>"`
5. Push tag: `git push origin v<version>`
6. Create GitHub Release with generated notes

**Files**: `CHANGELOG.md`, `ROADMAP.md`, `.github/RELEASE_TEMPLATE.md`

**GitHub Milestones**: v9.0.0, v10.0.0, v10.5.0, v10.6.0, v10.9.0, v11.0.0

---

## Claude Code DX

See `.claude/README.md` for full documentation.

### Key Commands

| Command | Description |
|---------|-------------|
| `/novanet-arch` | Architecture diagrams (ASCII) |
| `/novanet-sync` | Schema validation/regeneration |
| `/security-audit` | Comprehensive security audit |
| `/schema:add-node <name>` | Add new node type |
| `/schema:edit-node <name>` | Modify existing node |
| `/schema:add-arc <ARC>` | Add new arc type |

### Schema Management Workflow

```
1. /schema:add-node MyNode             # Socratic discovery
   ↓
2. YAML created                        # packages/core/models/node-classes/{realm}/{layer}/my-node.yaml
   ↓                                   # with explicit realm: and layer: fields
3. cargo run -- schema generate        # Regenerate all artifacts from YAML
   ↓
4. cargo run -- schema validate        # Validate YAML coherence + path/content match
   ↓
5. cargo run -- db seed                # Update Neo4j
```

### YAML Class Structure

```yaml
# packages/core/models/node-classes/shared/knowledge/term.yaml
node:
  name: Term
  realm: shared               # Source of truth (must match path)
  layer: knowledge            # v11.5: 10 layers (4 shared + 6 org)
  trait: imported             # v11.8: defined/authored/imported/generated/retrieved
  description: "..."
  properties:
    # ...
```

> **v11.8 Changes**: "Kind" → "Class", "Meta" eliminated. Trait redefined as "Data Origin" (ADR-024).
