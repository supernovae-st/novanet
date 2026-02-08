# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NovaNet is a **native content generation system** (NOT translation) using Neo4j graph database. It uses an orchestrator-subagent architecture where an orchestrator dispatches content generation tasks to specialized sub-agents, each responsible for generating a specific block of a page.

**Target Application**: QR Code AI (https://qrcode-ai.com) - a multilingual SaaS for QR code generation.
**Supported Locales**: 200+ locales (fr-FR, en-US, es-MX, ja-JP, etc.)
**Current Version**: v10.9.0

## CRITICAL: Generation, NOT Translation

```
NOVANET = NATIVE GENERATION

Source -> Translate -> Target        <-- WRONG
Entity (invariant) -> Generate natively -> EntityContent (local)  <-- RIGHT
```

Each locale content is **generated natively** from the invariant Entity, NOT translated. The LLM receives context **entirely in the target locale** and generates natively.

For complete graph schema, node categories, and relations, see: **`models/_index.yaml`**

## v10.9 Architecture

v10.9 establishes 2-Realm Architecture with naming convention refactor:

| Axis | Values |
|------|--------|
| **Realm** | global / tenant |
| **Layer** | 9 functional layers (3 global, 6 tenant) |
| **Trait** | invariant / localized / knowledge / derived / job |
| **ArcFamily** | ownership / localization / semantic / generation / mining |

**Key v10.9 changes:**
- Renamed nodes: EntityL10n → EntityContent, PageL10n → PageGenerated, BlockL10n → BlockGenerated
- Renamed arcs: HAS_L10N → HAS_CONTENT, HAS_OUTPUT → HAS_GENERATED
- GLOBAL (3 layers): config, locale-knowledge, seo — universal, READ-ONLY
- TENANT (6 layers): config, foundation, structure, semantic, instruction, output
- 64 node types, 116 arc types

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
pnpm schema:validate
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

-- v10.9: Navigate meta-graph (Realm -> Layer -> Kind)
MATCH (r:Realm {key: "tenant"})-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
RETURN r.key, l.key, collect(k.label) AS kinds;

-- v9: Find all Kinds with a specific Trait
MATCH (k:Kind)-[:HAS_TRAIT]->(t:Trait {key: "localized"})
RETURN k.label, t.key;

-- v9: Arc schema for a Kind
MATCH (ak:ArcKind)-[:FROM_KIND]->(k:Kind {label: "Block"})
MATCH (ak)-[:TO_KIND]->(target:Kind)
MATCH (ak)-[:IN_FAMILY]->(af:ArcFamily)
RETURN ak.key, af.key AS family, target.label AS target_kind;
```

## File Structure

```
core/
├── models/                    # YAML schema definitions (SOURCE OF TRUTH)
│   ├── _index.yaml            # MODEL INDEX (graph structure, node categories)
│   ├── taxonomy.yaml          # v10.6: 2 Realms (global/tenant), 9 Layers
│   ├── relations.yaml         # Legacy format (kept for parser compatibility)
│   ├── node-kinds/            # ONE FILE PER NODE TYPE
│   │   ├── global/            # Realm: global
│   │   │   ├── config/        #   Layer: config (Locale + utility nodes)
│   │   │   ├── locale-knowledge/  #   Layer: locale-knowledge (Knowledge Atoms)
│   │   │   └── seo/           #   Layer: seo (SEOKeyword, Metrics, MiningRun)
│   │   └── tenant/            # Realm: tenant (v10.9)
│   │       ├── config/        #   Layer: config (Organization, Tenant)
│   │       ├── foundation/    #   Layer: foundation (Project, Brand, ProjectL10n)
│   │       ├── structure/     #   Layer: structure (Page, Block, ContentSlot)
│   │       ├── semantic/      #   Layer: semantic (Entity, EntityContent, Persona)
│   │       ├── instruction/   #   Layer: instruction (Prompts, BlockRules)
│   │       └── output/        #   Layer: output (PageGenerated, BlockGenerated)
│   ├── arc-kinds/             # ONE FILE PER ARC TYPE (64 files)
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

## Nomenclature (v10.9.0)

```
*Content suffix = Human-curated localized content (EntityContent)
*Generated      = LLM-generated output content (PageGenerated, BlockGenerated)
:HAS_CONTENT    = Human-curated content arc (Entity→EntityContent)
:HAS_GENERATED  = LLM-generated content arc (Page→PageGenerated, Block→BlockGenerated)
*L10n suffix    = Other localized content (ProjectL10n - still uses L10n)
Locale*         = Locale Knowledge nodes (LocaleVoice, LocaleCulture, etc.)
*Metrics        = Time-series observations (SEOKeywordMetrics)
*MiningRun      = Batch operations (SEOMiningRun)
```

**v10.9 meta-graph terminology:**
```
Realm           = WHERE? (global / tenant)
Layer           = WHAT? (9 functional layers)
Kind            = Neo4j label as meta-node
Trait           = HOW? (invariant / localized / knowledge / derived / job)
ArcFamily       = Relationship classification (ownership / localization / semantic / generation / mining)
ArcKind         = Individual relationship type as meta-node
:Meta           = Double-label on all meta-nodes
OF_KIND         = Instance -> Kind bridge
NavigationMode  = 4 modes (data / meta / overlay / query)
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
