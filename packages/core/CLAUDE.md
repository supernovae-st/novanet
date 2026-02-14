# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NovaNet is a **native content generation system** (NOT translation) using Neo4j graph database. It uses an orchestrator-subagent architecture where an orchestrator dispatches content generation tasks to specialized sub-agents, each responsible for generating a specific block of a page.

**Target Application**: QR Code AI (https://qrcode-ai.com) - a multilingual SaaS for QR code generation.
**Supported Locales**: 200+ locales (fr-FR, en-US, es-MX, ja-JP, etc.)
**Current Version**: v0.12.0

## CRITICAL: Generation, NOT Translation

```
NOVANET = NATIVE GENERATION

Source -> Translate -> Target        <-- WRONG
Entity (invariant) -> Generate natively -> EntityContent (local)  <-- RIGHT
```

Each locale content is **generated natively** from the invariant Entity, NOT translated. The LLM receives context **entirely in the target locale** and generates natively.

For complete graph schema, node categories, and relations, see: **`models/_index.yaml`**

## v0.12.0 Architecture

v0.12.0 refines the 2-Realm Architecture with ADR-023 (Class/Instance terminology) and ADR-024 (Data Origin traits):

| Axis | Values |
|------|--------|
| **Realm** | shared / org |
| **Layer** | 10 functional layers (4 shared + 6 org) |
| **Trait** | defined / authored / imported / generated / retrieved |
| **ArcFamily** | ownership / localization / semantic / generation / mining |

**Key v0.12.0 changes:**
- ADR-023: "Kind" → "Class" terminology (NodeKind→NodeClass, ArcKind→ArcClass)
- ADR-023: ":Meta:" → ":Schema:" in Neo4j labels
- ADR-024: Trait redefinition as "Data Origin" (invariant→defined, localized→authored, knowledge→imported, aggregated→retrieved)
- ADR-025: PagePrompt→PageInstruction, BlockPrompt→BlockInstruction
- ADR-028: Brand Architecture (Brand, BrandDesign, BrandPrinciples, PromptStyle, Country)
- SHARED (4 layers): config, locale, geography, knowledge — universal, READ-ONLY (40 nodes)
- ORG (6 layers): config, foundation, structure, semantic, instruction, output (21 nodes)
- 61 node types, 156 arc types

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
MATCH (b)-[:USES_ENTITY]->(e:Entity)-[:HAS_CONTENT]->(el:EntityContent)-[:FOR_LOCALE]->(l:Locale {key: "fr-FR"})
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
│   ├── taxonomy.yaml          # v11.7: 2 Realms (shared/org), 10 Layers
│   ├── node-classes/            # ONE FILE PER NODE TYPE
│   │   ├── shared/            # Realm: shared (39 nodes)
│   │   │   ├── config/        #   Layer: config (3 nodes: Locale, EntityCategory, SEOKeywordFormat)
│   │   │   ├── locale/        #   Layer: locale (6 nodes)
│   │   │   ├── geography/     #   Layer: geography (6 nodes)
│   │   │   └── knowledge/     #   Layer: knowledge (24 nodes incl. SEO/GEO)
│   │   └── org/               # Realm: org (20 nodes)
│   │       ├── config/        #   Layer: config (1 node: OrgConfig)
│   │       ├── foundation/    #   Layer: foundation (Project, BrandIdentity, ProjectContent)
│   │       ├── structure/     #   Layer: structure (Page, Block, ContentSlot)
│   │       ├── semantic/      #   Layer: semantic (Entity, EntityContent, etc.)
│   │       ├── instruction/   #   Layer: instruction (PageInstruction, BlockInstruction, etc.)
│   │       └── output/        #   Layer: output (PageGenerated, BlockGenerated)
│   ├── arc-classes/             # ONE FILE PER ARC TYPE (116 files)
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

## Nomenclature (v0.12.0)

```
*Content suffix = Human-authored localized content (EntityContent)
*Generated      = LLM-generated output content (PageGenerated, BlockGenerated)
:HAS_CONTENT    = Human-authored content arc (Entity→EntityContent)
:HAS_GENERATED  = LLM-generated content arc (Page→PageGenerated, Block→BlockGenerated)
:HAS_INSTRUCTION= Instruction arc (Page→PageInstruction, Block→BlockInstruction)
ProjectContent  = Localized project content
Locale*         = Locale Knowledge nodes (LocaleVoice, LocaleCulture, etc.)
*Metrics        = Time-series observations (SEOKeywordMetrics, GEOMetrics)
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

| English (invariant) | Localized (generated natively) |
|---------------------|--------------------------------|
| `*.key`, `*.llm_context`, `*.instructions`, `*.rules` | `*Content.*`, `*Generated.*` fields |
| Project.key, Entity.key, Page.key, Block.key | EntityContent, PageGenerated, BlockGenerated |

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
