---
description: Add a new node type to the NovaNet knowledge graph
argument-hint: <node-name>
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, AskUserQuestion
---

Add a new node type to the NovaNet ontology using Socratic discovery.

## Workflow

1. **Discovery Phase**: Ask clarifying questions about the node:
   - What realm? (global, tenant)
   - What layer? (existing or new)
   - What is its purpose in the graph?
   - What properties does it need?
   - What relationships does it have?

2. **Validation Phase**: Check for conflicts:
   - Does this node already exist?
   - Does it overlap with existing nodes?
   - Does it follow NovaNet nomenclature?

3. **Creation Phase**:
   - Create YAML file in `packages/core/models/node-kinds/{realm}/{layer}/{node-name}.yaml`
   - Add arc YAML in `arc-kinds/` if new arc types needed
   - Run `novanet schema generate`
   - Run `novanet schema validate`

4. **Seed Phase** (optional):
   - Create migration file if needed
   - Update seed files

## Naming Requirements

- **Node names**: PascalCase (e.g., `LocaleHumor`, `EntityContent`)
- **YAML filename**: kebab-case (e.g., `locale-humor.yaml`, `entity-content.yaml`)
- **Property names**: snake_case (e.g., `display_name`, `llm_context`)

## Nomenclature Rules (v10.9.0)

| Pattern | Use For | Examples |
|---------|---------|----------|
| `*Content` | Human-curated localized content | EntityContent |
| `*Generated` | LLM-generated output content | PageGenerated, BlockGenerated |
| `*L10n` | Other localized content | ProjectL10n |
| `Locale*` | Locale knowledge | LocaleVoice, LocaleCulture, LocaleLexicon |
| `*Metrics` | Time-series data | SEOKeywordMetrics |
| `*MiningRun` | Batch operations | SEOMiningRun |

## Example

```bash
/schema:add-node LocaleHumor
```

This will start a Socratic dialog to understand the node's purpose, properties, and relationships before creating it.
