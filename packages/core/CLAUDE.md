# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NovaNet is a **native content generation system** (NOT translation) using Neo4j graph database. It uses an orchestrator-subagent architecture where an orchestrator dispatches content generation tasks to specialized sub-agents, each responsible for generating a specific block of a page.

**Target Application**: QR Code AI (https://qrcode-ai.com) - a multilingual SaaS for QR code generation.
**Supported Locales**: 200+ locales (fr-FR, en-US, es-MX, ja-JP, etc.)
**Current Version**: v0.17.1

## CRITICAL: Generation, NOT Translation

```
NOVANET = NATIVE GENERATION

Source -> Translate -> Target        <-- WRONG
Entity (defined) -> Generate natively -> EntityNative (locale)  <-- RIGHT
```

Each locale content is **generated natively** from the invariant Entity, NOT translated. The LLM receives context **entirely in the target locale** and generates natively.

For complete graph schema, node categories, and relations, see: **`models/_index.yaml`**

## v0.13.0 Architecture

v0.13.0 introduces the *Native pattern (ADR-029) and Slug Ownership (ADR-030):

| Axis | Values |
|------|--------|
| **Realm** | shared / org |
| **Layer** | 10 functional layers (4 shared + 6 org) |
| **Trait** | defined / authored / imported / generated / retrieved |
| **ArcFamily** | ownership / localization / semantic / generation / mining |

**Key v0.13.0 changes:**
- ADR-029: *Native pattern (EntityContent→EntityNative, ProjectContent→ProjectNative, PageGenerated→PageNative, BlockGenerated→BlockNative)
- ADR-029: Unified arcs (HAS_CONTENT/HAS_GENERATED→HAS_NATIVE, CONTENT_OF/GENERATED_FOR→NATIVE_OF)
- ADR-030: Slug Ownership (URL properties moved from EntityNative to PageNative)
- SHARED (4 layers): config, locale, geography, knowledge — universal, READ-ONLY (36 nodes)
- ORG (6 layers): config, foundation, structure, semantic, instruction, output (21 nodes)
- 57 node types, 131 arc types

**v0.17.1 Schema Cleanup:**
- Removed: Term, TermSet, SEOKeywordMetrics (YAGNI)
- Added: ProjectGEOScope, ProjectSEOScope (project-level scope config)

**Boundary rule:** TypeScript (this package) generates code artifacts. Rust (`tools/novanet/`) executes at runtime.

## Commands

### Neo4j Setup

```bash
# From monorepo root:
pnpm infra:up              # Start Neo4j
pnpm infra:seed            # Run seed (constraints + data)

# Reset database
pnpm infra:down
docker volume rm novanet_neo4j_data
pnpm infra:up && pnpm infra:seed

# Cypher shell
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword
```

### Development

```bash
# Build TypeScript
pnpm build

# Run tests
pnpm test

# Lint
pnpm lint

# Validate schemas
cargo run -- schema validate
```

### Useful Cypher Queries

```cypher
-- Count all nodes by type
MATCH (n) RETURN labels(n)[0] AS label, count(*) AS count ORDER BY count DESC;

-- Get project with its pages
MATCH (p:Project {key: "qrcode-ai"})-[:HAS_PAGE]->(page:Page)
RETURN p.key, collect(page.key) AS pages;

-- Load block context for generation
MATCH (b:Block {key: "hero-pricing"})
MATCH (b)-[:USES_ENTITY]->(e:Entity)-[:HAS_NATIVE]->(el:EntityNative)-[:FOR_LOCALE]->(l:Locale {key: "fr-FR"})
MATCH (b)-[:OF_TYPE]->(bt:BlockType)
MATCH (l)-[:HAS_VOICE]->(v:LocaleVoice)
MATCH (l)-[:HAS_LEXICON]->(lex:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
WHERE e.semantic_field IN ['urgency', 'value']
RETURN b.instructions, e.key, el.title, bt.rules, v.formality_score, collect(ex.text) AS expressions;

-- v0.12.0: Navigate schema-graph (Realm -> Layer -> Class) - ADR-023
MATCH (r:Realm {key: "org"})-[:HAS_LAYER]->(l:Layer)-[:HAS_CLASS]->(c:Schema:Class)
RETURN r.key, l.key, collect(c.label) AS classes;

-- v0.12.0: Find all Classes with a specific Trait - ADR-024
MATCH (c:Schema:Class)-[:HAS_TRAIT]->(t:Trait {key: "generated"})
RETURN c.label, t.key;

-- v0.12.0: Arc schema for a Class
MATCH (ac:Schema:ArcClass)-[:FROM_CLASS]->(c:Schema:Class {label: "Block"})
MATCH (ac)-[:TO_CLASS]->(target:Schema:Class)
MATCH (ac)-[:IN_FAMILY]->(af:Schema:ArcFamily)
RETURN ac.key, af.key AS family, target.label AS target_class;
```

## File Structure

```
core/
├── models/                    # YAML schema definitions (SOURCE OF TRUTH)
│   ├── _index.yaml            # MODEL INDEX (graph structure, node categories)
│   ├── taxonomy.yaml          # v0.17.1: 2 Realms (shared/org), 10 Layers
│   ├── node-classes/            # ONE FILE PER NODE TYPE
│   │   ├── shared/            # Realm: shared (36 nodes)
│   │   │   ├── config/        #   Layer: config (3 nodes: Locale, EntityCategory, SEOKeywordFormat)
│   │   │   ├── locale/        #   Layer: locale (5 nodes)
│   │   │   ├── geography/     #   Layer: geography (7 nodes)
│   │   │   └── knowledge/     #   Layer: knowledge (21 nodes incl. SEO/GEO)
│   │   └── org/               # Realm: org (21 nodes)
│   │       ├── config/        #   Layer: config (1 node: OrgConfig)
│   │       ├── foundation/    #   Layer: foundation (8 nodes: Project, Brand, ProjectGEOScope, etc.)
│   │       ├── structure/     #   Layer: structure (Page, Block, ContentSlot)
│   │       ├── semantic/      #   Layer: semantic (Entity, EntityNative)
│   │       ├── instruction/   #   Layer: instruction (BlockType, BlockRules, BlockInstruction, PromptArtifact)
│   │       └── output/        #   Layer: output (PageNative, BlockNative, OutputArtifact)
│   ├── arc-classes/             # ONE FILE PER ARC TYPE (131 arcs)
│   └── views/                 # YAML view definitions
├── src/                       # TypeScript source
│   ├── config/                # Locale codes configuration
│   ├── filters/               # NovaNetFilter, CypherGenerator, ViewLoader
│   ├── graph/                 # Graph utilities
│   ├── schemas/               # Zod validation schemas
│   └── types/                 # TypeScript type definitions
└── __tests__/                 # Jest test suites
```

> **Note:** `parsers/`, `services/`, `db/`, and `scripts/` were absorbed into the Rust binary (`tools/novanet/`) in v9.0.0.

## Nomenclature (v0.13.0 — ADR-029)

```
*Native suffix  = Locale-specific content (EntityNative, ProjectNative, PageNative, BlockNative)
:HAS_NATIVE     = Unified arc for all *Native nodes (replaces HAS_CONTENT + HAS_GENERATED)
:NATIVE_OF      = Inverse arc (replaces CONTENT_OF + GENERATED_FOR)
:HAS_INSTRUCTION= Instruction arc (Page→PageInstruction, Block→BlockInstruction)
Locale*         = Locale Knowledge nodes (LocaleVoice, LocaleCulture, etc.)
*Scope          = Project-level scope config (ProjectGEOScope, ProjectSEOScope) — v0.17.1
```

**v0.12.0 schema-graph terminology (ADR-023):**
```
Realm           = WHERE? (shared / org)
Layer           = WHAT? (10 functional layers: 4 shared + 6 org)
Class           = Neo4j label as schema-node (was "Kind")
Trait           = HOW? Data Origin (defined / authored / imported / generated / retrieved)
ArcFamily       = Relationship classification (ownership / localization / semantic / generation / mining)
ArcClass        = Individual relationship type as schema-node (was "ArcKind")
:Schema         = Double-label on all schema-nodes (was ":Meta")
OF_CLASS        = Instance -> Class bridge (was "OF_KIND")
NavigationMode  = 2 modes (graph / nexus)
```

**Trait definitions (ADR-024 Data Origin):**
```
defined         = Human-created once (templates, configs) — was "invariant"
authored        = Human-written per locale (editorial content) — was "localized"
imported        = External data brought in (corpora, SEO keywords) — was "knowledge"
generated       = Produced by NovaNet LLM
retrieved       = Fetched from external APIs (GEO snapshots) — was "aggregated"
```

## Language Convention

| English (defined) | Localized (generated natively) |
|-------------------|--------------------------------|
| `*.key`, `*.llm_context`, `*.instructions`, `*.rules` | `*Native.*` fields |
| Project.key, Entity.key, Page.key, Block.key | ProjectNative, EntityNative, PageNative, BlockNative |

**Remember**: Localized content is **generated natively**, not translated.

## Content Generation Instructions

Block instructions use these directives:
- `[FIXED]` - Use exact value (brand names, URLs, etc.)
- `[GENERATE]` - Create content **natively** in target locale (NOT translate)

Entity references use `@entity-key` syntax in instructions.

> There is no `[TRANSLATE]` directive. All content is generated natively per locale.

## Neo4j Credentials

- **URL**: http://localhost:7474
- **Bolt**: bolt://localhost:7687
- **User**: `neo4j`
- **Password**: `novanetpassword`

## MCP Integration

Project-level MCP servers are defined in `.mcp.json`:

```json
{
  "mcpServers": {
    "neo4j-cypher": { ... },
    "neo4j-memory": { ... }
  }
}
```

### Available MCP Tools

| Tool | Description |
|------|-------------|
| `mcp__neo4j__get_neo4j_schema` | Get graph schema |
| `mcp__neo4j__read_neo4j_cypher` | Execute read queries |
| `mcp__neo4j__write_neo4j_cypher` | Execute write queries |

## Skills

Project-specific skills in `.claude/skills/`:

| Skill | Use When |
|-------|----------|
| `neo4j-expert` | Writing Cypher, optimizing queries, debugging |
| `context-graph-architect` | Designing graph schemas for AI context |
| `spreading-activation` | Implementing semantic traversal |
| `dev-environment` | Setting up, troubleshooting environment |

## Quick Reference

```bash
# Start dev environment (from monorepo root)
pnpm infra:up && pnpm infra:seed

# Test MCP connection
# Use: mcp__neo4j__get_neo4j_schema(sample_size=100)

# Interactive Neo4j
open http://localhost:7474
```
