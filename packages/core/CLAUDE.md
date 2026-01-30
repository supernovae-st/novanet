# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NovaNet is a **native content generation system** (NOT translation) using Neo4j graph database. It uses an orchestrator-subagent architecture where an orchestrator dispatches content generation tasks to specialized sub-agents, each responsible for generating a specific block of a page.

**Target Application**: QR Code AI (https://qrcode-ai.com) - a multilingual SaaS for QR code generation.
**Supported Locales**: 200+ locales (fr-FR, en-US, es-MX, ja-JP, etc.)
**Current Version**: v8.2.0

## CRITICAL: Generation, NOT Translation

```
NOVANET = NATIVE GENERATION

Source -> Translate -> Target        <-- WRONG
Concept (invariant) -> Generate natively -> ConceptL10n (local)  <-- RIGHT
```

Each locale content is **generated natively** from the invariant Concept, NOT translated. The LLM receives context **entirely in the target locale** and generates natively.

For complete graph schema, node categories, and relations, see: **`models/_index.yaml`**

## Commands

### Neo4j Setup

```bash
# From monorepo root:
pnpm infra:up              # Start Neo4j
pnpm infra:seed    # Run seed (constraints + data)

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
npm run build

# Run tests
npm test

# Lint
npm run lint

# Validate schemas
npm run validate
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

-- Spreading activation from a concept
MATCH (c:Concept {key: "tier-pro"})-[r:SEMANTIC_LINK*1..2]->(c2:Concept)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH c2, reduce(a = 1.0, rel IN r | a * rel.temperature) AS activation
WHERE activation >= 0.3
RETURN c2.key, activation ORDER BY activation DESC;
```

## File Structure (v8.2.0)

```
core/
├── models/                    # YAML schema definitions
│   ├── _index.yaml            # MODEL INDEX (graph structure, node categories, changes)
│   ├── relations.yaml         # All 50 Neo4j relationships
│   ├── nodes/                 # ONE FILE PER NODE TYPE (35 files)
│   │   ├── global/            # 🌍 GLOBAL scope (15 nodes)
│   │   │   ├── config/        #    Locale
│   │   │   └── knowledge/     #    14 LocaleKnowledge nodes
│   │   ├── project/           # 📦 PROJECT scope (14 nodes)
│   │   │   ├── foundation/    #    Project, BrandIdentity, ProjectL10n
│   │   │   ├── structure/     #    Page, Block, BlockType, PageType
│   │   │   ├── semantic/      #    Concept, ConceptL10n
│   │   │   ├── instruction/   #    PagePrompt, BlockPrompt, BlockRules
│   │   │   └── output/        #    PageL10n, BlockL10n
│   │   └── shared/            # 🎯 SHARED scope (6 nodes)
│   │       ├── seo/           #    SEOKeywordL10n, SEOKeywordMetrics, SEOMiningRun
│   │       └── geo/           #    GEOSeedL10n, GEOSeedMetrics, GEOMiningRun
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

## Nomenclature v8.2.0

```
*L10n suffix    = ALL localized content (human OR LLM generated)
:HAS_L10N       = human-curated content (ConceptL10n, ProjectL10n)
:HAS_OUTPUT     = LLM-generated content (PageL10n, BlockL10n)
Locale*         = Locale Knowledge nodes (LocaleVoice, LocaleCulture, etc.)
*Metrics        = Time-series observations (SEOKeywordMetrics, GEOSeedMetrics)
```

**v8.2.0 Breaking Changes:**
- Removed: icon, priority, freshness properties from all nodes (YAGNI)
- Standard properties now: key, display_name, description, llm_context, created_at, updated_at
- Added: Binary schema sync tests (YAML ↔ TypeScript ↔ Neo4j)
- Validated: All 35 node types across 3 scopes

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

### Enable MCP Servers

Add to `~/.claude/settings.json`:
```json
{
  "enableAllProjectMcpServers": true
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
