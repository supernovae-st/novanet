---
description: Display the complete NovaNet architecture diagram in ASCII
argument-hint: [section]
allowed-tools: Bash
---

## Pre-flight: Sync Check

Before displaying architecture, validate the source of truth is synchronized:

```bash
pnpm schema:validate
```

If validation fails, run `pnpm schema:generate` to fix, then continue.

---

Display the NovaNet architecture diagram showing the complete system overview.

## Available Sections

If `$ARGUMENTS` is provided, focus on that specific section:

| Argument | Description |
|----------|-------------|
| `source`, `yaml` | YAML source of truth structure |
| `pipeline`, `sync` | Source of Truth sync pipeline (YAML → TypeScript/Mermaid → Neo4j) |
| `locale`, `knowledge` | Locale Knowledge node structure (14 types) |
| `infra`, `neo4j` | Infrastructure (Docker, seeds, migrations) |
| `studio` | Studio web app (API routes, Zustand stores) |
| `packages`, `deps` | Packages dependency graph |
| `flow`, `generation` | Data flow / LLM generation pipeline |
| `all` or empty | Complete architecture (default) |

## Examples

```
/novanet-arch              # Full architecture
/novanet-arch pipeline     # How YAML propagates to Neo4j
/novanet-arch locale       # Locale Knowledge structure
/novanet-arch infra        # Seeds and migrations
```

Use the novanet-architecture skill to display the diagrams.
