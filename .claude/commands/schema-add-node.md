---
description: Add a new node type to the NovaNet knowledge graph
argument-hint: <node-name>
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, AskUserQuestion
---

Add a new node type to the NovaNet ontology using Socratic discovery.

## Workflow

1. **Discovery Phase**: Ask clarifying questions about the node:
   - What scope? (global, project, shared)
   - What subcategory? (existing or new)
   - What is its purpose in the graph?
   - What properties does it need?
   - What relationships does it have?

2. **Validation Phase**: Check for conflicts:
   - Does this node already exist?
   - Does it overlap with existing nodes?
   - Does it follow NovaNet nomenclature?

3. **Creation Phase**:
   - Create YAML file in `packages/core/models/nodes/{scope}/{subcategory}/{node-name}.yaml`
   - Update `relations.yaml` if new relationships
   - Run `pnpm schema:generate`
   - Run `pnpm schema:validate`

4. **Seed Phase** (optional):
   - Create migration file if needed
   - Update seed files

## Naming Requirements

- **Node names**: PascalCase (e.g., `LocaleHumor`, `ConceptL10n`)
- **YAML filename**: kebab-case (e.g., `locale-humor.yaml`, `concept-l10n.yaml`)
- **Property names**: snake_case (e.g., `display_name`, `llm_context`)

## Nomenclature Rules

| Pattern | Use For | Examples |
|---------|---------|----------|
| `*L10n` | Localized content | ConceptL10n, PageL10n, BlockL10n, ProjectL10n |
| `Locale*` | Locale knowledge | LocaleVoice, LocaleCulture, LocaleLexicon |
| `*Metrics` | Time-series data | SEOKeywordMetrics, GEOSeedMetrics |
| `*MiningRun` | Batch operations | SEOMiningRun, GEOMiningRun |

## Example

```bash
/schema:add-node LocaleHumor
```

This will start a Socratic dialog to understand the node's purpose, properties, and relationships before creating it.
