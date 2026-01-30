---
description: NovaNet schema management - add/edit nodes and relations
argument-hint: [add-node|edit-node|add-relation|status] [name]
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, AskUserQuestion, mcp__neo4j__read_neo4j_cypher, mcp__neo4j__get_neo4j_schema
---

# NovaNet Schema Management

Master command for managing the NovaNet knowledge graph schema.

## Quick Commands

| Command | Description |
|---------|-------------|
| `/schema status` | Show current schema stats |
| `/schema add-node <name>` | Add new node type |
| `/schema edit-node <name>` | Modify existing node |
| `/schema add-relation <NAME>` | Add new relationship |

## Schema Architecture

```
YAML (Source of Truth)          TypeScript (Generated)         Neo4j (Runtime)
packages/core/models/     -->   packages/core/src/types/  -->  bolt://localhost:7687
├── nodes/                      ├── index.ts
│   ├── global/                 └── locale-knowledge.ts
│   ├── project/
│   └── shared/
└── relations.yaml
```

## Workflow

Based on `$ARGUMENTS`:

### `status` (default)

Show current schema statistics:

```bash
# Count YAML node definitions
find packages/core/models/nodes -name "*.yaml" | wc -l

# Validate sync
pnpm schema:validate

# Show Neo4j node counts
```

Use MCP to query Neo4j:
```cypher
MATCH (n)
RETURN labels(n)[0] AS type, count(*) AS count
ORDER BY count DESC LIMIT 20
```

### `add-node <name>`

Redirect to `/schema:add-node <name>`

### `edit-node <name>`

Redirect to `/schema:edit-node <name>`

### `add-relation <NAME>`

Redirect to `/schema:add-relation <NAME>`

## Current Schema (v8.2.0)

**35 Node Types** across 3 scopes:
- **Global (15)**: Locale + 14 LocaleKnowledge nodes
- **Project (14)**: Project structure, concepts, prompts, outputs
- **Shared (6)**: SEO/GEO targeting and metrics

**47 Relationships** in 2 categories:
- **Semantic**: Used in spreading activation (SEMANTIC_LINK, USES_CONCEPT, INFLUENCED_BY)
- **Auxiliary**: Structural/provenance (HAS_BLOCK, HAS_PROMPT, HAS_METRICS)

## Validation Commands

```bash
# Validate YAML ↔ TypeScript sync
pnpm schema:validate

# Regenerate from YAML
pnpm schema:generate

# Full database reset
pnpm infra:reset
```
