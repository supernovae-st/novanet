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

## Nomenclature Rules

- `*L10n` suffix = localized content (ConceptL10n, PageL10n)
- `Locale*` prefix = locale knowledge (LocaleVoice, LocaleCulture)
- `*Metrics` suffix = time-series data (SEOKeywordMetrics)
- `*MiningRun` suffix = batch operations (SEOMiningRun)

## Example

```bash
/schema:add-node LocaleHumor
```

This will start a Socratic dialog to understand the node's purpose, properties, and relationships before creating it.
