# NovaNet

Turborepo monorepo for NovaNet - knowledge graph localization orchestrator.

## Auto-Imported Context

@README.md @ROADMAP.md @CHANGELOG.md

---

## Overview

NovaNet uses Neo4j to orchestrate **native content generation** (NOT translation) across 200+ locales.

**Target Application**: QR Code AI (https://qrcode-ai.com)
**Current Version**: v11.0.0
**Roadmap**: `ROADMAP.md` | **Changelog**: `CHANGELOG.md`

```
CRITICAL: Generation, NOT Translation

Source -> Translate -> Target        <-- WRONG
Entity (invariant) -> Generate natively -> EntityContent (local)  <-- RIGHT
```

---

## v11.0 Nomenclature

v11.0 establishes 2-Realm Architecture with SEO in tenant realm:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  VOCABULARY                                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│  Level           │ Vertex    │ Edge     │                                   │
│  ─────────────────────────────────────────                                  │
│  General         │ Node      │ Arc      │                                   │
│  Instance (data) │ NodeData  │ ArcData  │                                   │
│  Definition      │ NodeKind  │ ArcKind  │                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│  CLASSIFICATION AXES                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│  NodeKind:                                                                  │
│    WHERE?  NodeRealm  (global / tenant)                                     │
│    WHAT?   NodeLayer  (9 layers: 2 global + 7 tenant)                       │
│    HOW?    NodeTrait  (invariant / localized / knowledge / derived / job)   │
│                                                                             │
│  ArcKind:                                                                   │
│    SCOPE   ArcScope       (intra_realm / cross_realm)                       │
│    FUNC    ArcFamily      (ownership / localization / semantic / gen / min) │
│    MULT    ArcCardinality (1:1 / 1:N / N:M)                                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Key changes in v11.0:**
- **SEO → Tenant**: Moved 9 SEO/GEO nodes from `global/seo` to `tenant/seo` (ADR-012 fix)
- **Arc coherence**: Fixed 22 arc scopes (cross_realm → intra_realm)
- **Node naming** (v10.9): `*L10n` → semantic suffixes (`EntityContent`, `PageGenerated`, `BlockGenerated`)
- **Arc naming** (v10.9): `HAS_L10N` → `HAS_CONTENT`, `HAS_OUTPUT` → `HAS_GENERATED`
- **Composite keys**: `entity:{entity_key}@{locale_key}` pattern for locale-specific nodes

**Architecture (v11.0):**
- 2 realms: GLOBAL + TENANT
- GLOBAL (2 layers): config, locale-knowledge — universal, READ-ONLY
- TENANT (7 layers): config, foundation, structure, semantic, instruction, seo, output

**Rust binary:** `tools/novanet/` — single crate for CLI + TUI (neo4rs, ratatui, clap).
All commands implemented: data/meta/overlay/query, node/arc CRUD, search, locale, db,
schema generate/validate, doc generate, filter build, Galaxy-themed TUI with boot animation, effects engine, and onboarding. 246 tests pass.

**YAML-first architecture:** Each Kind YAML has explicit `realm:` and `layer:` fields (source of truth).
Path validation ensures `models/node-kinds/{realm}/{layer}/{name}.yaml` matches YAML content.
v11.0: 2 realms (global, tenant), 9 layers total (2 global + 7 tenant).

**Icons source of truth (v11.0):** `visual-encoding.yaml` → `icons:` section provides dual-format icons:
- `web`: Lucide icon name for Studio
- `terminal`: Unicode symbol for TUI
Categories: realms, layers, traits, arc_families, states, navigation, quality, modes.

**Boundary rule (v9 target):** TypeScript generates code artifacts. Rust executes at runtime.

---

## Knowledge Atoms Architecture

**Knowledge Atoms** provide granular knowledge nodes for selective LLM context loading.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  KNOWLEDGE ARCHITECTURE                                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Locale ──[:HAS_TERMS]──> TermSet ──[:CONTAINS]──> Term (atom)              │
│          ──[:HAS_EXPRESSIONS]──> ExpressionSet ──[:CONTAINS]──> Expression  │
│          ──[:HAS_PATTERNS]──> PatternSet ──[:CONTAINS]──> Pattern           │
│          ──[:HAS_CULTURE]──> CultureSet ──[:CONTAINS]──> CultureRef         │
│          ──[:HAS_TABOOS]──> TabooSet ──[:CONTAINS]──> Taboo                 │
│          ──[:HAS_AUDIENCE]──> AudienceSet ──[:CONTAINS]──> AudienceTrait    │
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
│     └─ Unlike Entities (invariant + Content for ALL locales)                │
│     └─ Atoms exist only where needed: fr-FR may have 20K Terms              │
│     └─ sw-KE may have 500 Terms - no translation, native generation         │
│                                                                             │
│  3. SELECTIVE LLM LOADING                                                   │
│     └─ Load 50 relevant Terms, not 20K JSON blob                            │
│     └─ Graph queries: "Terms used by this Block"                            │
│     └─ [:USES_TERM], [:USES_EXPRESSION] on Block nodes                      │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  STATISTICS                                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Containers (6): TermSet, ExpressionSet, PatternSet,                        │
│                  CultureSet, TabooSet, AudienceSet                          │
│  Atoms (6):      Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait│
│  Total:          76 nodes, 123 arcs                                         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**No backward compatibility needed** - this is v0, design for clean architecture.

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
cargo run -- meta --format=json            # Mode 1: Meta-graph
cargo run -- data --format=table           # Mode 2: Data nodes
cargo run -- overlay                       # Mode 3: Data + Meta
cargo run -- query --realm=tenant          # Mode 4: Faceted query
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
| tools/novanet | Rust CLI + TUI — all runtime commands (245 tests) |

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
2. YAML created                        # packages/core/models/node-kinds/{realm}/{layer}/my-node.yaml
   ↓                                   # with explicit realm: and layer: fields
3. cargo run -- schema generate        # Regenerate all artifacts from YAML
   ↓
4. cargo run -- schema validate        # Validate YAML coherence + path/content match
   ↓
5. cargo run -- db seed                # Update Neo4j
```

### YAML Kind Structure

```yaml
# packages/core/models/node-kinds/global/locale-knowledge/locale-voice.yaml
node:
  name: LocaleVoice
  realm: global               # Source of truth (must match path)
  layer: locale-knowledge     # v11.0: 2 realms (global, tenant)
  trait: knowledge
  description: "..."
  properties:
    # ...
```
