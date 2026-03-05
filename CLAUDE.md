# NovaNet

Turborepo monorepo for NovaNet - knowledge graph localization orchestrator.

## Auto-Imported Context

@README.md @ROADMAP.md @CHANGELOG-LATEST.md

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
**Current Version**: v0.17.0 "Neuro-Symbolic Validation" (novanet_check + novanet_audit)
**Roadmap**: `ROADMAP.md` | **Changelog**: `CHANGELOG.md`

**Related docs**:
- `.claude/rules/novanet-decisions.md` — ADR index (quick reference)
- `.claude/rules/novanet-terminology.md` — Canonical terminology reference
- `../.claude/rules/adr/` — Full ADR content by domain (32 ADRs, in parent workspace)
- Use `/adr <number>` command for quick lookup

```
CRITICAL: Generation, NOT Translation

Source -> Translate -> Target        <-- WRONG
Entity (defined) -> Generate natively -> EntityNative (authored)  <-- RIGHT
```

---

## v0.13.0 Nomenclature

v0.13.0 introduces the *Native pattern with unified arcs:

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

**Key changes in v0.13.0:**
- ***Native Pattern** (ADR-029): EntityContent→EntityNative, ProjectContent→ProjectNative, PageGenerated→PageNative, BlockGenerated→BlockNative
- **Unified Arcs** (ADR-029): HAS_CONTENT/HAS_GENERATED→HAS_NATIVE, CONTENT_OF/GENERATED_FOR→NATIVE_OF
- **Slug Ownership** (ADR-030): URL properties moved from EntityNative to PageNative
- **59 nodes** total: 40 shared + 19 org, **178 arcs** (6 families)

**Architecture (v0.16.0):**
- 2 realms: SHARED + ORG
- SHARED (4 layers): config, locale, geography, knowledge — universal, READ-ONLY (40 nodes)
- ORG (6 layers): config, foundation, structure, semantic, instruction, output (19 nodes)

**Rust binary:** `tools/novanet/` — single crate for CLI + TUI (neo4rs, ratatui, clap).
All commands implemented: blueprint/data/overlay/query, node/arc CRUD, search, locale, db,
schema generate/validate, doc generate, filter build. Galaxy-themed TUI with unified tree mode (v11.7), boot animation, effects engine, Nexus hub, and onboarding. 1279 tests pass.

**YAML-first architecture:** Each Class YAML has explicit `realm:` and `layer:` fields (source of truth).
Path validation ensures `models/node-classes/{realm}/{layer}/{name}.yaml` matches YAML content.
v0.16: 2 realms (shared, org), 10 layers total (4 shared + 6 org), 59 nodes, 178 arcs.

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
│  STATISTICS (v0.13.0)                                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Containers (6): TermSet, ExpressionSet, PatternSet,                        │
│                  CultureSet, TabooSet, AudienceSet                          │
│  Atoms (6):      Term, Expression, Pattern, CultureRef, Taboo, AudienceTrait│
│  Total:          59 nodes (40 shared + 19 org)                              │
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

### novanet CLI (tools/novanet)

```bash
# TUI (default when no command)
novanet                              # Launch interactive TUI
novanet tui --fresh                  # Regenerate + reset, then TUI

# Schema (no Neo4j)
novanet schema generate              # YAML → Cypher/TS/Mermaid
novanet schema validate              # Validate coherence
novanet blueprint                    # Schema visualization

# Database (Neo4j)
novanet db seed                      # Seed database
novanet db reset                     # Reset database
novanet search --query="page"        # Fulltext search

# System
novanet doctor                       # Health check
```

> **Dev mode**: Use `cargo run --` instead of `novanet` when developing.

### Monorepo (pnpm)

```bash
pnpm dev                   # Start Studio at localhost:3000
pnpm build                 # Build all packages
pnpm test                  # Test all packages
pnpm infra:up              # Start Neo4j
pnpm infra:seed            # Seed database
```

---

## Packages

| Package | Description |
|---------|-------------|
| @novanet/core | Types, schemas, filters, generators |
| @novanet/db | Neo4j Docker, seeds, migrations |
| @novanet/studio | Web-based graph visualization |
| tools/novanet | Rust CLI + TUI — all runtime commands (1279 tests) |

---

## MCP Server (v0.17.0)

NovaNet exposes an MCP (Model Context Protocol) server for workflow automation and AI agent integration.

**Location:** `tools/novanet-mcp/`

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  MCP TOOLS (14 tools)                                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  novanet_query       Execute read-only Cypher against Neo4j                 │
│                      params: cypher, params, limit, timeout_ms              │
│                      returns: rows, row_count, token_estimate, cached       │
│                                                                             │
│  novanet_describe    Bootstrap agent understanding of the knowledge graph   │
│                      params: describe (schema|entity|category|relations|    │
│                              locales|stats), entity_key, category_key       │
│                      returns: target, data, token_estimate                  │
│                                                                             │
│  novanet_search      Fulltext + property search with hybrid mode            │
│                      params: query, mode (fulltext|property|hybrid),        │
│                              kinds, realm, limit                            │
│                      returns: hits, total_hits, mode, token_estimate        │
│                                                                             │
│  novanet_traverse    Graph traversal with configurable depth and filters    │
│                      params: start_key, max_depth, direction (outgoing|     │
│                              incoming|both), arc_families, target_kinds     │
│                      returns: start, nodes, arcs, max_depth_reached         │
│                                                                             │
│  novanet_assemble    Assemble context for LLM generation (token-aware)      │
│                      params: focus_key, locale, token_budget, strategy      │
│                              (breadth|depth|relevance|custom)               │
│                      returns: focus, evidence[], locale_context, truncated  │
│                                                                             │
│  novanet_atoms       Retrieve knowledge atoms for a specific locale         │
│                      params: locale, atom_type (term|expression|pattern|    │
│                              cultureref|taboo|audiencetrait|all), domain    │
│                      returns: locale, atoms[], containers[], total_count    │
│                                                                             │
│  novanet_generate    Full RLM-on-KG context assembly for generation         │
│                      params: focus_key, locale, mode (block|page),          │
│                              token_budget, spreading_depth                  │
│                      returns: prompt, evidence_summary, locale_context,     │
│                               context_anchors, denomination_forms (ADR-033),│
│                               context_build_log (DX-11)                     │
│                                                                             │
│  novanet_introspect  Schema introspection for agents to query metadata      │
│                      params: target (classes|class|arcs|arc), name,         │
│                              realm, layer, family, include_arcs             │
│                      returns: target, data, token_estimate                  │
│                                                                             │
│  novanet_batch       Bulk operations with parallel execution                │
│                      params: operations[], parallel, stop_on_error          │
│                      returns: results[], total_execution_time_ms            │
│                                                                             │
│  novanet_cache_stats Get cache statistics (entries, hit rate, memory)       │
│                      params: (none)                                         │
│                      returns: entries, hit_rate, hits, misses, ttl_secs     │
│                                                                             │
│  novanet_cache_invalidate  Invalidate cache by pattern or clear all         │
│                      params: pattern, clear_all                             │
│                      returns: invalidated_count, pattern_used               │
│                                                                             │
│  novanet_write       Write data to Neo4j with schema validation             │
│                      params: operation, class/arc_class, key/from_key/to_key│
│                              properties, locale                             │
│                      operations: upsert_node, create_arc, update_props      │
│                      returns: success, created, updated_properties,         │
│                               auto_arcs_created, cache_invalidated          │
│                                                                             │
│  novanet_check       Pre-write validation with intelligent feedback         │
│                      params: operation, class, key, properties, locale      │
│                      returns: valid, errors[], warnings[], suggestions[]    │
│                      (v0.17.0: neuro-symbolic validation with llm_context)  │
│                                                                             │
│  novanet_audit       Post-write quality audit with CSR metrics              │
│                      params: target (coverage|orphans|integrity|freshness|  │
│                              all), scope, limit                             │
│                      returns: issues[], summary, csr, recommendations[]     │
│                      (v0.17.0: MMKG-RDS research-based quality scoring)     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Integration with Nika:**

```yaml
# Nika workflow using NovaNet MCP
workflow: generate-page
mcp:
  servers:
    novanet:
      command: "cargo run --manifest-path ../novanet-dev/tools/novanet-mcp/Cargo.toml"

tasks:
  - id: load_context
    invoke: novanet_generate
    params:
      entity: "qr-code"
      locale: "fr-FR"
      forms: ["text", "title", "abbrev"]
    use.ctx: entity_context

  - id: generate_page
    infer: "Generate landing page content"
    context: $entity_context
```

**v0.17.0 MCP Server:**

| Feature | Description | Reference |
|---------|-------------|-----------|
| `novanet_check` | Pre-write validation with llm_context suggestions | v0.17.0 |
| `novanet_audit` | Post-write quality audit with CSR metrics | v0.17.0 |
| `denomination_forms` | Prescriptive canonical forms for LLM entity references | ADR-033 |
| `context_build_log` | Step-by-step context assembly debugging | DX-11 |

**ADR-033 Denomination Forms:**

The `novanet_generate` tool returns `denomination_forms` — prescriptive canonical forms for LLM entity references:

| Form | Usage | Example (es-MX) |
|------|-------|-----------------|
| `text` | Prose, body content | "código qr" |
| `title` | H1, H2, meta_title | "Código QR" |
| `abbrev` | After first mention | "qr" |
| `url` | URL-safe slug | "crear-código-qr" |

**Running the MCP Server:**

```bash
cd tools/novanet-mcp
cargo run                    # Start MCP server (stdio transport)
cargo test                   # Run MCP tests
```

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
5. **Check ADRs** — Use `/adr <number>` command for quick lookup (full ADRs in parent `supernovae-agi` workspace)
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
| `/adr [query]` | Quick ADR lookup by number or keyword |

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
