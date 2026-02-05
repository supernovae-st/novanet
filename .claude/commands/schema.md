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
├── node-kinds/                 ├── index.ts
│   ├── global/                 └── locale-knowledge.ts
│   └── tenant/
├── arc-kinds/
└── taxonomy.yaml
```

## Workflow

Based on `$ARGUMENTS`:

### `status` (default)

Show current schema statistics:

```bash
# Count YAML node definitions
find packages/core/models/node-kinds -name "*.yaml" | wc -l

# Validate sync
novanet schema validate

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

## Current Schema (v10.6.0)

**43 Kind Types** across 2 Realms:
- **Global (20)**: Locale, Knowledge Atoms (Sets + Atoms), SEO nodes (READ-ONLY)
- **Tenant (23)**: Organization, Project structure, Entity, prompts, outputs, generation

**63 Arcs** in 5 ArcFamilies:
- **Ownership**: HAS_PAGE, HAS_BLOCK, OF_TYPE
- **Localization**: HAS_L10N, FOR_LOCALE
- **Semantic**: SEMANTIC_LINK, USES_ENTITY
- **Generation**: HAS_OUTPUT, HAS_PROMPT
- **Mining**: HAS_SEO_TARGET

## Validation Commands

```bash
# Validate YAML ↔ generated files sync
novanet schema validate

# Regenerate from YAML
novanet schema generate

# Full database reset
novanet db reset
```
