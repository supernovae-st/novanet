# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NovaNet is a **native content generation system** (NOT translation) using Neo4j graph database. It uses an orchestrator-subagent architecture where an orchestrator dispatches content generation tasks to specialized sub-agents, each responsible for generating a specific block of a page.

**Target Application**: QR Code AI (https://qrcode-ai.com) - a multilingual SaaS for QR code generation.
**Supported Locales**: 200+ locales (fr-FR, en-US, es-MX, ja-JP, etc.)
**Current Version**: v9.0.0

## CRITICAL: Generation, NOT Translation

```
NOVANET = NATIVE GENERATION

Source -> Translate -> Target        <-- WRONG
Concept (invariant) -> Generate natively -> ConceptL10n (local)  <-- RIGHT
```

Each locale content is **generated natively** from the invariant Concept, NOT translated. The LLM receives context **entirely in the target locale** and generates natively.

For complete graph schema, node categories, and relations, see: **`models/_index.yaml`**

## v9 Migration Context

v9 refactors the meta-graph to a **self-describing context graph** with faceted classification:

| v8 Term | v9 Term |
|---------|---------|
| Scope | **Realm** (global / project / shared) |
| Subcategory | **Layer** (9 functional layers) |
| NodeTypeMeta | **Kind** (35 node types, 1:1 with Neo4j labels) |
| _(new)_ | **Trait** (invariant / localized / knowledge / derived / job) |
| _(new)_ | **EdgeFamily** (ownership / localization / semantic / generation / mining) |
| _(new)_ | **EdgeKind** (47 relationship types) |

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
MATCH (b)-[:USES_CONCEPT]->(c:Concept)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale {key: "fr-FR"})
MATCH (b)-[:OF_TYPE]->(bt:BlockType)
MATCH (l)-[:HAS_VOICE]->(v:LocaleVoice)
MATCH (l)-[:HAS_LEXICON]->(lex:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
WHERE e.semantic_field IN ['urgency', 'value']
RETURN b.instructions, c.key, cl.title, bt.rules, v.formality_score, collect(e.text) AS expressions;

-- v9: Navigate meta-graph (Realm -> Layer -> Kind)
MATCH (r:Realm {key: "project"})-[:HAS_LAYER]->(l:Layer)-[:HAS_KIND]->(k:Kind)
RETURN r.key, l.key, collect(k.label) AS kinds;

-- v9: Find all Kinds with a specific Trait
MATCH (k:Kind)-[:HAS_TRAIT]->(t:Trait {key: "localized"})
RETURN k.label, t.key;

-- v9: Edge schema for a Kind
MATCH (ek:EdgeKind)-[:FROM_KIND]->(k:Kind {label: "Block"})
MATCH (ek)-[:TO_KIND]->(target:Kind)
MATCH (ek)-[:IN_FAMILY]->(ef:EdgeFamily)
RETURN ek.key, ef.key AS family, target.label AS target_kind;
```

## File Structure

```
core/
├── models/                    # YAML schema definitions (SOURCE OF TRUTH)
│   ├── _index.yaml            # MODEL INDEX (graph structure, node categories, changes)
│   ├── relations.yaml         # All 47 Neo4j relationships (with family field in v9)
│   ├── organizing-principles.yaml  # v9: Realm/Layer/Trait/EdgeFamily definitions
│   ├── nodes/                 # ONE FILE PER NODE TYPE (35 files)
│   │   ├── global/            # Realm: global (15 nodes)
│   │   │   ├── config/        #   Layer: config (Locale)
│   │   │   └── knowledge/     #   Layer: knowledge (14 nodes)
│   │   ├── project/           # Realm: project (14 nodes)
│   │   │   ├── foundation/    #   Layer: foundation (Project, BrandIdentity, ProjectL10n)
│   │   │   ├── structure/     #   Layer: structure (Page, Block, PageType, BlockType)
│   │   │   ├── semantic/      #   Layer: semantic (Concept, ConceptL10n)
│   │   │   ├── instruction/   #   Layer: instruction (PagePrompt, BlockPrompt, BlockRules)
│   │   │   └── output/        #   Layer: output (PageL10n, BlockL10n)
│   │   └── shared/            # Realm: shared (6 nodes)
│   │       ├── seo/           #   Layer: seo (SEOKeywordL10n, SEOKeywordMetrics, SEOMiningRun)
│   │       └── geo/           #   Layer: geo (GEOSeedL10n, GEOSeedMetrics, GEOMiningRun)
│   └── views/                 # YAML view definitions
├── src/                       # TypeScript source
│   ├── db/                    # Neo4j connection (client.ts)
│   ├── filters/               # NovaNetFilter, CypherGenerator, ViewLoader
│   ├── parsers/               # Markdown parsers for Locale Knowledge
│   ├── schemas/               # Zod validation schemas
│   └── types/                 # TypeScript type definitions
├── scripts/                   # Build scripts
├── neo4j/                     # Seed files (Docker in /infra)
├── config/                    # Locale codes
└── docs/                      # Additional documentation
```

## Nomenclature

```
*L10n suffix    = ALL localized content (human OR LLM generated)
:HAS_L10N       = human-curated content (ConceptL10n, ProjectL10n)
:HAS_OUTPUT     = LLM-generated content (PageL10n, BlockL10n)
Locale*         = Locale Knowledge nodes (LocaleVoice, LocaleCulture, etc.)
*Metrics        = Time-series observations (SEOKeywordMetrics, GEOSeedMetrics)
*MiningRun      = Batch operations (SEOMiningRun, GEOMiningRun)
```

**v9 meta-graph terminology:**
```
Realm           = WHERE? (global / project / shared) — replaces "Scope"
Layer           = WHAT? (9 functional layers) — replaces "Subcategory"
Kind            = Neo4j label as meta-node — replaces "NodeTypeMeta"
Trait           = HOW? (invariant / localized / knowledge / derived / job) — NEW
EdgeFamily      = Relationship classification (ownership / localization / ...) — NEW
EdgeKind        = Individual relationship type as meta-node — NEW
:Meta           = Double-label on all meta-nodes — NEW
OF_KIND         = Instance -> Kind bridge — replaces "IN_SUBCATEGORY"
NavigationMode  = 4 modes (data / meta / overlay / query) — replaces "DataMode"
```

## Language Convention

| English (invariant) | Localized (generated natively) |
|---------------------|--------------------------------|
| `*.key`, `*.llm_context`, `*.instructions`, `*.rules` | `*L10n.*` fields |
| Project.key, Concept.key, Page.key, Block.key | ConceptL10n, PageL10n, BlockL10n |

**Remember**: Localized content is **generated natively**, not translated.

## Content Generation Instructions

Block instructions use these directives:
- `[FIXED]` - Use exact value (brand names, URLs, etc.)
- `[GENERATE]` - Create content **natively** in target locale (NOT translate)

Concept references use `@concept-key` syntax in instructions.

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
