---
description: Add a new relationship type to the NovaNet knowledge graph
argument-hint: <RELATION_NAME>
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, AskUserQuestion
---

Add a new relationship type between nodes in the NovaNet ontology.

## Workflow

1. **Discovery Phase**: Ask about the relationship:
   - From which node type(s)?
   - To which node type(s)?
   - What is the cardinality? (1:1, 1:N, N:N)
   - Does it need properties? (temperature, position, etc.)
   - Is it semantic or auxiliary?

2. **Classification**: Determine relationship type:
   - **Semantic** (used in spreading activation): SEMANTIC_LINK, USES_CONCEPT, INFLUENCED_BY
   - **Auxiliary** (structural/provenance): HAS_BLOCK, HAS_PROMPT, HAS_METRICS

3. **Bidirectionality Check**: Does it need an inverse?
   - HAS_L10N needs L10N_OF
   - HAS_BLOCK needs BLOCK_OF
   - Not all relations need inverses

4. **Creation Phase**:
   - Add to `packages/core/models/relations.yaml`
   - Update source node YAML with relation reference
   - Run `pnpm schema:generate`
   - Run `pnpm schema:validate`

5. **Seed Phase** (optional):
   - Create migration to add relationship to existing data
   - Update seed files with relationship creation

## Naming Conventions

| Pattern | Use For | Examples |
|---------|---------|----------|
| `HAS_*` | Ownership/containment | HAS_PAGE, HAS_BLOCK, HAS_L10N |
| `*_OF` | Inverse of HAS_* | L10N_OF, BLOCK_OF |
| `FOR_*` | Target association | FOR_LOCALE |
| `USES_*` | Reference/usage | USES_CONCEPT |
| `TARGETS_*` | Cross-scope targeting | TARGETS_SEO, TARGETS_GEO |

## Example

```bash
/schema:add-relation HAS_HUMOR
```

This will start a dialog to understand the relationship between LocaleLexicon and LocaleHumor (or whatever nodes are involved).
