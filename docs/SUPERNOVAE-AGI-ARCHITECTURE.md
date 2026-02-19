# SuperNovae-AGI Architecture Documentation

**Version**: v0.13.1 | **Last Updated**: 2026-02-19

> This document provides a comprehensive overview of the SuperNovae-AGI system architecture for use with NotebookLM and other AI-assisted learning tools.

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [The Brain & Body Architecture](#the-brain--body-architecture)
3. [NovaNet: The Knowledge Graph System](#novanet-the-knowledge-graph-system)
4. [Nika: The Workflow Engine](#nika-the-workflow-engine)
5. [MCP Integration](#mcp-integration)
6. [Neo4j Database Structure](#neo4j-database-structure)
7. [Key Architecture Decision Records (ADRs)](#key-architecture-decision-records-adrs)
8. [Technical Specifications](#technical-specifications)

---

## Executive Summary

SuperNovae-AGI is a monorepo containing two complementary AI systems:

- **NovaNet** ("The Brain"): A Neo4j-powered knowledge graph that orchestrates native content generation across 200+ locales
- **Nika** ("The Body"): A semantic YAML workflow engine that executes multi-step AI workflows consuming NovaNet's knowledge

### Core Philosophy

> **"Generation, NOT Translation"**
>
> Content is generated natively per locale from defined semantic entities, NOT translated from a source language. Each locale gets culturally-native content, preserving local nuances that translation would lose.

### Key Statistics

| Component | Metric |
|-----------|--------|
| NovaNet Node Classes | 61 |
| NovaNet Arc Classes | 182 |
| Supported Locales | 200+ |
| Rust Tests | 1,139 passing |
| Neo4j Nodes | 22,189+ |
| Cypher Seed Lines | 317,073 |
| TUI Lines of Code | 49,460 |

---

## The Brain & Body Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  supernovae-agi ARCHITECTURE                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────┐         MCP Protocol        ┌─────────────────┐   │
│  │     NOVANET         │◄──────────────────────────►│      NIKA       │   │
│  │     (Brain)         │                             │     (Body)      │   │
│  ├─────────────────────┤                             ├─────────────────┤   │
│  │ • Knowledge Graph   │    novanet_generate         │ • YAML Workflows│   │
│  │ • Entity Memory     │    novanet_describe         │ • LLM Providers │   │
│  │ • Locale Context    │    novanet_traverse         │ • DAG Execution │   │
│  │ • SEO/GEO Intel     │◄────────────────────────────│ • Tool Calling  │   │
│  │ • ADR Decisions     │                             │ • State Machine │   │
│  └─────────────────────┘                             └─────────────────┘   │
│         │                                                     │            │
│         ▼                                                     ▼            │
│  novanet-dev/                                          nika-dev/           │
│  └── tools/novanet/                                    └── tools/nika/     │
│      └── src/                                              └── src/        │
│          ├── core/     ← shared patterns                       ├── core/   │
│          └── tui/      ← terminal UI                           └── tui/    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Communication Protocol

NovaNet and Nika communicate exclusively via the **Model Context Protocol (MCP)**:
- **Transport**: stdio (standard input/output)
- **SDK**: rmcp 0.15 (Anthropic's Rust MCP SDK)
- **Direction**: Nika → NovaNet (Nika is the MCP client, NovaNet is the server)

---

## NovaNet: The Knowledge Graph System

### Overview

NovaNet is a Neo4j-powered knowledge graph localization orchestrator. It generates native content across 200+ locales without translation.

### Directory Structure

```
novanet-dev/
├── packages/
│   ├── core/                  # @novanet/core — types, schemas, filters
│   │   ├── models/            # YAML schema definitions (SOURCE OF TRUTH)
│   │   │   ├── taxonomy.yaml  # 2 realms, 10 layers, 5 traits
│   │   │   ├── node-classes/  # 61 node definitions
│   │   │   └── arc-classes/   # 182 arc definitions
│   │   └── src/               # TypeScript implementation
│   └── db/                    # @novanet/db — Neo4j infrastructure
│       ├── docker-compose.yml # Neo4j 5.26 + APOC
│       └── seed/              # 45 Cypher seed scripts
├── tools/
│   ├── novanet/               # Rust CLI + TUI (1,139 tests)
│   └── novanet-mcp/           # MCP Server (7 tools)
└── apps/
    └── studio/                # Next.js 16 web visualization
```

### Schema Architecture

#### Two Realms

| Realm | Purpose | Node Count |
|-------|---------|------------|
| **SHARED** | Universal locale knowledge (READ-ONLY) | 40 nodes |
| **ORG** | Organization-specific business content | 21 nodes |

#### Ten Layers

**SHARED Realm (4 layers):**
| Layer | Nodes | Examples |
|-------|-------|----------|
| config | 3 | EntityCategory, Locale, SEOKeywordFormat |
| locale | 6 | Culture, Style, Formatting, Slugification |
| geography | 7 | Continent, GeoRegion, Country |
| knowledge | 24 | Term, Expression, Pattern, SEOKeyword |

**ORG Realm (6 layers):**
| Layer | Nodes | Examples |
|-------|-------|----------|
| config | 1 | OrgConfig |
| foundation | 6 | Project, Brand, BrandDesign |
| structure | 3 | Page, Block, ContentSlot |
| semantic | 4 | Entity, EntityNative |
| instruction | 4 | PageStructure, BlockInstruction |
| output | 3 | PageNative, BlockNative, OutputArtifact |

#### Five Traits (Data Origin)

Traits answer: **"WHERE does the data come from?"**

| Trait | Count | Who Creates | Examples |
|-------|-------|-------------|----------|
| **defined** | 31 | Human, ONCE | Page, Block, Entity, Locale |
| **authored** | 2 | Human, PER locale | EntityNative, ProjectNative |
| **imported** | 20 | External data | Term, SEOKeyword, GEOQuery |
| **generated** | 4 | Our LLM | PageNative, BlockNative |
| **retrieved** | 2 | External APIs | GEOAnswer, SEOKeywordMetrics |

#### Six Arc Families

| Family | Purpose | Example Arcs |
|--------|---------|--------------|
| ownership | Parent→Child hierarchy | HAS_PAGE, HAS_BLOCK, HAS_ENTITY |
| localization | Locale links | FOR_LOCALE, HAS_NATIVE |
| semantic | Meaning links | USES_ENTITY, SEMANTIC_LINK |
| generation | LLM pipeline | GENERATED, COMPILED_FROM |
| mining | SEO/GEO intelligence | TARGETS_KEYWORD, MONITORS_GEO |
| schema | Meta-schema relationships | OF_CLASS, FROM_CLASS, TO_CLASS |

### The *Native Pattern (ADR-029)

All locale-specific content uses the `*Native` suffix:

| Invariant Node | Native Node | Trait |
|----------------|-------------|-------|
| Entity | EntityNative | authored |
| Project | ProjectNative | authored |
| Page | PageNative | generated |
| Block | BlockNative | generated |

```cypher
// Example: Entity with locale-specific content
MATCH (e:Entity {key: "qr-code"})
MATCH (e)-[:HAS_NATIVE {locale: "fr-FR"}]->(en:EntityNative)
RETURN e.key, en.display_name, en.description
```

### Rust CLI Architecture

**Location**: `tools/novanet/`

```
tools/novanet/
├── src/
│   ├── main.rs          # Entry point, clap dispatcher
│   ├── config.rs        # Monorepo root discovery
│   ├── error.rs         # NovaNetError enum
│   ├── db.rs            # Neo4j connection pool
│   ├── cypher.rs        # CypherStatement builder
│   ├── commands/        # 13 CLI commands
│   ├── parsers/         # 20+ YAML parsers
│   ├── generators/      # 12 code generators
│   ├── validation/      # 14 rules + autofix system
│   └── tui/             # 50+ files, 49,460 lines
```

#### CLI Commands

| Command | Description |
|---------|-------------|
| `schema generate` | Regenerate all artifacts from YAML |
| `schema validate` | Validate YAML coherence |
| `blueprint` | Schema-graph visualization |
| `data` | Data nodes visualization |
| `overlay` | Combined schema + data view |
| `query` | Faceted queries |
| `search` | Fulltext + property search |
| `node create/edit/delete` | CRUD operations |
| `db seed/reset` | Database management |
| `locale list` | Locale operations |
| `doc generate` | Generate Mermaid diagrams |
| `tui` | Interactive terminal UI |

#### 12 Code Generators

The YAML-first architecture generates:

1. **Cypher**: Schema seed scripts for Neo4j
2. **TypeScript**: Type definitions for Studio
3. **Rust**: Struct definitions for CLI
4. **Mermaid**: Documentation diagrams
5. **Hierarchy**: Layer/Realm mappings
6. **Filters**: NovaNetFilter API
7. **Icons**: Dual format (web + terminal)
8. **Views**: Query definitions
9. **Constraints**: Neo4j indexes
10. **Arc definitions**: Relationship schemas
11. **Node definitions**: Label schemas
12. **Taxonomy**: Classification metadata

### TUI (Terminal User Interface)

**Two Navigation Modes:**

1. **Graph Mode** (`[1]`): Unified tree with lazy instance loading
   - Realm → Layer → Class → Instance hierarchy
   - Everything is a clickable node

2. **Nexus Mode** (`[2]`): Hub for learning and validation
   - Quiz: Test NovaNet knowledge
   - Audit: Validate schema consistency
   - Stats: Dashboard with graph metrics
   - Help: Keybindings and documentation

---

## Nika: The Workflow Engine

### Overview

Nika is a semantic YAML workflow engine that executes multi-step AI workflows. It serves as the "body" that consumes NovaNet's knowledge through MCP.

### Directory Structure

```
nika-dev/
└── tools/nika/
    └── src/
        ├── main.rs              # CLI entry point
        ├── lib.rs               # Public API
        ├── error.rs             # NikaError (40+ variants)
        ├── ast/                 # YAML → Rust structs
        │   ├── workflow.rs      # Workflow + MCP config
        │   ├── action.rs        # TaskAction (5 variants)
        │   ├── invoke.rs        # InvokeParams
        │   └── agent.rs         # AgentParams
        ├── mcp/                 # MCP Client
        │   ├── types.rs         # McpConfig, ToolResult
        │   └── client.rs        # McpClient
        ├── runtime/             # Execution engine
        │   ├── executor.rs      # Task dispatch
        │   ├── runner.rs        # Workflow orchestration
        │   └── agent_loop.rs    # Agentic execution
        ├── provider/            # LLM providers
        │   ├── types.rs         # Message, ToolCall, Usage
        │   ├── claude.rs        # Anthropic API
        │   └── openai.rs        # OpenAI API
        ├── event/               # Observability
        │   ├── log.rs           # EventLog (16 variants)
        │   └── trace.rs         # NDJSON writer
        ├── resilience/          # Production patterns
        │   ├── retry.rs         # Exponential backoff (21 tests)
        │   ├── circuit_breaker.rs # Closed→Open→HalfOpen (12 tests)
        │   ├── rate_limiter.rs  # Token bucket (11 tests)
        │   └── metrics.rs       # Performance tracking
        └── tui/                 # Terminal UI (planned)
```

### Workflow Schema Versions

| Version | Capabilities |
|---------|--------------|
| v0.1 | infer, exec, fetch |
| v0.2 | +invoke (MCP), +agent (agentic loop) |
| v0.3 | +for_each (parallel execution) |

### Five Task Actions (Verbs)

| Verb | Purpose | Example |
|------|---------|---------|
| **infer** | One-shot LLM call | Generate, summarize, analyze |
| **exec** | Shell command | Build, test, deploy |
| **fetch** | HTTP request | APIs, webhooks |
| **invoke** | MCP tool call | novanet_generate, novanet_traverse |
| **agent** | Multi-turn agentic loop | Complex reasoning with tool calling |

### Workflow YAML Example

```yaml
schema: "nika/workflow@0.2"
provider: claude
model: claude-sonnet-4-20250514

mcp:
  novanet:
    command: cargo
    args: [run, --manifest-path, ../novanet-mcp/Cargo.toml]
    env:
      NEO4J_URI: bolt://localhost:7687
      NEO4J_PASSWORD: novanetpassword

tasks:
  - id: load_entity
    invoke:
      mcp: novanet
      tool: novanet_describe
      params:
        key: "qr-code"
        depth: 2
    use.ctx: entity_context

  - id: load_locale
    invoke:
      mcp: novanet
      tool: novanet_atoms
      params:
        locale: "fr-FR"
        categories: [terms, expressions, patterns]
    use.ctx: locale_context

  - id: generate_content
    needs: [load_entity, load_locale]
    infer: |
      Generate native French content for the QR Code landing page.
      Use the entity context and locale knowledge atoms.
    context:
      - $entity_context
      - $locale_context
```

### DAG Execution

Nika resolves task dependencies as a Directed Acyclic Graph (DAG):

```
┌─────────┐     ┌─────────┐
│ load_   │     │ load_   │
│ entity  │     │ locale  │
└────┬────┘     └────┬────┘
     │               │
     └───────┬───────┘
             │
       ┌─────▼─────┐
       │ generate_ │
       │ content   │
       └───────────┘
```

Tasks without dependencies run in parallel. Tasks with `needs:` wait for dependencies.

### Resilience Patterns (66 tests)

| Pattern | Purpose | Tests |
|---------|---------|-------|
| **Retry** | Exponential backoff with jitter | 21 |
| **CircuitBreaker** | Prevent cascade failures | 12 |
| **RateLimiter** | Token bucket algorithm | 11 |
| **Metrics** | Performance tracking | - |

**Circuit Breaker States:**
```
Closed ──(failures >= threshold)──► Open
                                      │
                                      │ (timeout)
                                      ▼
                               ┌─────────────┐
                               │  HalfOpen   │
                               └──────┬──────┘
                                      │
               ┌──────────────────────┼──────────────────────┐
               │                      │                      │
         (success)              (failure)                    │
               │                      │                      │
               ▼                      ▼                      │
           Closed                   Open ◄───────────────────┘
```

---

## MCP Integration

### MCP Server (NovaNet)

**Location**: `tools/novanet-mcp/`

The NovaNet MCP server exposes the knowledge graph to AI agents.

#### 7 Tools

| Tool | Purpose | Parameters |
|------|---------|------------|
| `novanet_query` | Execute raw Cypher | cypher, params, limit |
| `novanet_describe` | Describe a node | key, depth, include_arcs |
| `novanet_search` | Fulltext search | query, classes, limit |
| `novanet_traverse` | Graph traversal | start, arc, direction, depth |
| `novanet_assemble` | Assemble page context | page_key, locale, include_blocks |
| `novanet_atoms` | Load knowledge atoms | locale, categories, limit |
| `novanet_generate` | Full generation context | focus_key, locale, mode, token_budget |

#### 4 Resources

| Resource URI | Purpose |
|--------------|---------|
| `entity://{key}` | Entity definition + natives |
| `class://{name}` | NodeClass definition |
| `locale://{code}` | Locale configuration |
| `view://{id}` | Predefined Cypher view |

#### 6 Prompts

| Prompt | Purpose |
|--------|---------|
| `cypher_query` | Generate Cypher from natural language |
| `cypher_explain` | Explain Cypher query |
| `block_generation` | Generate block content |
| `page_generation` | Generate page content |
| `entity_analysis` | Analyze entity relationships |
| `locale_briefing` | Locale cultural briefing |

### MCP Client (Nika)

Nika connects to MCP servers via the `mcp:` configuration block:

```yaml
mcp:
  novanet:
    command: cargo
    args: [run, --manifest-path, tools/novanet-mcp/Cargo.toml]
    env:
      NEO4J_URI: bolt://localhost:7687
```

The `invoke:` verb calls MCP tools:

```yaml
- id: get_context
  invoke:
    mcp: novanet
    tool: novanet_generate
    params:
      focus_key: "homepage"
      locale: "fr-FR"
      mode: "page"
      token_budget: 50000
```

---

## Neo4j Database Structure

### Overview

| Aspect | Value |
|--------|-------|
| Version | Neo4j 5.26.0 Community |
| Plugins | APOC |
| Nodes | 22,189+ |
| Seed Files | 45 |
| Seed Lines | 317,073 |

### Seed Execution Phases

```
Phase 1: Schema Foundation (00-02)
├── 00-constraints.cypher     → 195 indexes/constraints
├── 00.5-taxonomy.cypher      → Realm, Layer, Trait, ArcFamily nodes
├── 01-classes.cypher         → 61 NodeClass schema nodes
└── 02-arc-classes.cypher     → 182 ArcClass schema nodes

Phase 2: Core Content (10-20)
├── 10-entity-categories.cypher
├── 11-entities.cypher
├── 12-entity-natives.cypher
└── 15-locales.cypher         → 200+ BCP-47 locales

Phase 3: Geography (21-24)
├── 21-continents.cypher
├── 22-regions.cypher
├── 23-subregions.cypher
└── 24-countries.cypher       → 195 countries

Phase 4: Locale Knowledge (25-30)
├── 25-slugification.cypher
├── 26-formatting.cypher
├── 27-culture.cypher
├── 28-term-sets.cypher
└── 29-terms.cypher

Phase 5: Business Content (40-50)
├── 40-org-config.cypher
├── 41-project.cypher
├── 42-pages.cypher
├── 43-blocks.cypher
└── 44-block-natives.cypher

Phase 6: SEO/GEO (51-60)
├── 51-seo-keywords.cypher
├── 52-targets-arcs.cypher
└── 53-geo-queries.cypher

Phase 7: Output (70-80)
├── 70-page-natives.cypher
├── 71-block-natives.cypher
└── 72-output-artifacts.cypher

Phase 8: Post-processing (98-99)
├── 98-autowire.cypher        → Automatic arc creation
└── 99-indexes.cypher         → Performance indexes
```

### Key Cypher Patterns

**Load Entity with Native Content:**
```cypher
MATCH (e:Entity {key: $key})
OPTIONAL MATCH (e)-[:HAS_NATIVE]->(en:EntityNative)
WHERE en.locale_key = $locale
OPTIONAL MATCH (e)-[:BELONGS_TO]->(cat:EntityCategory)
RETURN e, en, cat
```

**Spreading Activation (Knowledge Loading):**
```cypher
MATCH (focus:Entity {key: $focus_key})
CALL apoc.path.subgraphNodes(focus, {
  relationshipFilter: "HAS_NATIVE|USES_ENTITY|SEMANTIC_LINK",
  minLevel: 0,
  maxLevel: $depth
}) YIELD node
RETURN node
```

**Page Assembly:**
```cypher
MATCH (p:Page {key: $page_key})
MATCH (p)-[r:HAS_BLOCK]->(b:Block)
OPTIONAL MATCH (b)-[:HAS_NATIVE]->(bn:BlockNative)
WHERE bn.locale_key = $locale
RETURN p, b, bn
ORDER BY r.order
```

---

## Key Architecture Decision Records (ADRs)

### ADR-007: Generation, NOT Translation

> Content is generated natively per locale, NOT translated from a source.

```
WRONG:  Source → Translate → Target
RIGHT:  Entity (invariant) → Generate natively → EntityNative (locale)
```

### ADR-021: Query-First Architecture

> Cypher query = source of truth for graph visualization.

The graph displays exactly what the Cypher query returns. No hidden state or mode-specific filtering.

### ADR-022: Unified Tree Architecture

> "If it's a node in Neo4j, it's a node everywhere."

Two modes replace five:
- `[1]` Graph: Unified tree with lazy instance loading
- `[2]` Nexus: Hub for learning and validation

### ADR-024: Trait = Data Origin

> Trait answers "WHERE does data come from?" not "what it IS" (that's Layer).

| Trait | Origin |
|-------|--------|
| defined | Human creates ONCE |
| authored | Human writes PER locale |
| imported | External data brought in |
| generated | Our LLM produces |
| retrieved | External API snapshots |

### ADR-028: Page-Entity 1:1 Mandatory

> Every Page MUST have exactly one Entity via `[:REPRESENTS]`.

- Page owns URL structure (slug)
- Entity owns semantic identity (key)
- @ reference system for content injection

### ADR-029: *Native Pattern

> All locale-specific nodes use `*Native` suffix.

| Before | After |
|--------|-------|
| EntityContent | EntityNative |
| PageGenerated | PageNative |
| HAS_CONTENT | HAS_NATIVE |

### ADR-030: Slug Ownership

> Page owns URL, Entity owns semantics.

Slugs live in `BlockNative:head-seo-meta` (first block of every page):
- `slug`: Localized URL segment
- `meta_title`: SEO title
- `meta_description`: SEO description

### ADR-033: Denomination Forms

> Prescriptive canonical forms for LLM entity references.

```yaml
denomination_forms:
  - { type: "text", value: "código qr" }
  - { type: "title", value: "Código QR" }
  - { type: "abbrev", value: "qr" }
  - { type: "url", value: "crear-código-qr" }  # Post-SEO pipeline
```

**ABSOLUTE RULE**: LLM MUST use ONLY denomination_forms values. NO invention, NO paraphrase.

---

## Technical Specifications

### Technology Stack

| Layer | Technology |
|-------|------------|
| Knowledge Graph | Neo4j 5.26.0 + APOC |
| Schema Definition | YAML (source of truth) |
| Code Generation | Rust (12 generators) |
| CLI/TUI | Rust (clap, ratatui) |
| MCP Protocol | rmcp 0.15 |
| Web Studio | Next.js 16, React 19, TypeScript 5.9 |
| Workflow Engine | Rust (Nika) |
| LLM Providers | Claude API, OpenAI API |

### Performance Metrics

| Metric | Value |
|--------|-------|
| Rust Tests | 1,139 passing |
| Resilience Tests | 66 passing |
| TUI Lines | 49,460 |
| Cypher Seed Lines | 317,073 |
| Neo4j Nodes | 22,189+ |
| Supported Locales | 200+ |

### Repository Structure

```
supernovae-agi/
├── ROADMAP.md              # Master roadmap
├── CHANGELOG.md            # Version history
├── docs/                   # Cross-project documentation
├── novanet-dev/            # NovaNet submodule
│   ├── packages/core/      # Types, schemas, filters
│   ├── packages/db/        # Neo4j infrastructure
│   ├── tools/novanet/      # Rust CLI + TUI
│   ├── tools/novanet-mcp/  # MCP Server
│   └── apps/studio/        # Web visualization
└── nika-dev/               # Nika submodule
    └── tools/nika/         # Workflow engine
```

---

## Quick Reference

### Essential Commands

```bash
# NovaNet
cargo run -- schema generate      # Regenerate all artifacts
cargo run -- schema validate      # Validate YAML
cargo run -- tui                  # Interactive TUI

# Nika
cargo run -- run workflow.yaml    # Execute workflow
cargo run -- validate workflow.yaml  # Validate workflow

# Neo4j
pnpm infra:up                     # Start Neo4j
pnpm infra:seed                   # Seed database
```

### Key Files

| File | Purpose |
|------|---------|
| `packages/core/models/taxonomy.yaml` | Realm/Layer/Trait definitions |
| `packages/core/models/node-classes/` | 61 node YAML definitions |
| `packages/core/models/arc-classes/` | 182 arc YAML definitions |
| `.claude/rules/adr/` | 32 Architecture Decision Records |
| `packages/db/seed/` | 45 Cypher seed scripts |

### Claude Code Commands

| Command | Purpose |
|---------|---------|
| `/novanet-arch` | Architecture diagrams |
| `/adr <N>` | ADR lookup |
| `/schema:add-node` | Add node type |
| `/schema:edit-node` | Edit node type |
| `/schema:add-arc` | Add arc type |

---

## Gap Analysis (vs ROADMAP)

### MVP Status Summary

| MVP | Name | Status | Completion |
|-----|------|--------|------------|
| 0 | DX Setup Core | ✅ Done | 100% |
| 1 | Invoke Verb | ✅ Done | 100% |
| 2 | Agent + Observability | ✅ Done | 100% |
| 3 | TUI + CLI Trace | ✅ Done | 100% |
| 4 | Real Integration | ⏳ In Progress | ~90% |
| 5 | Production Hardening | ✅ Done | 100% |
| 6 | v0.3 Features | ⏳ In Progress | ~92% |

### Identified Gaps

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  🔴 CRITICAL (Blocking v0.3 Release)                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. ADR-033 denomination_forms NOT RETURNED by MCP                          │
│     ├── Location: novanet-dev/tools/novanet-mcp/src/tools/generate.rs       │
│     ├── Issue: novanet_generate response missing denomination_forms field   │
│     └── Fix: Add denomination_forms to NovanetGenerateResponse struct       │
│                                                                             │
│  2. context_build_log NOT IMPLEMENTED                                       │
│     ├── Location: novanet-dev/tools/novanet-mcp/                            │
│     ├── Issue: No trace of how context was assembled                        │
│     └── Fix: Add context_build_log field with assembly trace                │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  🟡 IMPORTANT (Should fix for v0.3)                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  3. Integration Test CI Pipeline Missing                                    │
│     ├── Location: .github/workflows/                                        │
│     └── Fix: Add workflow with Neo4j service container                      │
│                                                                             │
│  4. v0.3 Showcase Examples Missing                                          │
│     ├── Location: nika-dev/tools/nika/examples/                             │
│     └── Fix: Add for_each + invoke + agent showcase workflows               │
│                                                                             │
│  5. Mock Mode Still Default in McpClient                                    │
│     ├── Location: nika-dev/tools/nika/src/mcp/client.rs                     │
│     └── Fix: Invert default, use mock only for unit tests                   │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  🟢 NICE TO HAVE                                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  6. Documentation sync across CLAUDE.md files                               │
│  7. ADR quick-reference needs refresh                                       │
│  8. Workspace split (deferred - single crate works fine)                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Recommended Next Steps

**Immediate (Before v0.3):**
1. Add `denomination_forms` to MCP response
2. Add `context_build_log` field
3. Create v0.3 example workflows
4. Fix McpClient mock default

**Short Term:**
1. CI integration tests with Neo4j container
2. Documentation refresh
3. ADR maintenance

---

## Version History

| Date | Version | Changes |
|------|---------|---------|
| 2026-02-19 | 1.1.0 | Added gap analysis from 14-agent exploration |
| 2026-02-19 | 1.0.0 | Initial architecture spec |

---

*This document is maintained from exploration of the supernovae-agi codebase.*
