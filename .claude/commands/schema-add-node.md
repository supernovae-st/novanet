---
description: Add a new node type to the NovaNet knowledge graph
argument-hint: <node-name>
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, AskUserQuestion
---

Add a new node type to the NovaNet ontology using Socratic discovery.

## Workflow

1. **Discovery Phase**: Ask clarifying questions about the node:
   - What realm? (shared, org)
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

## Nomenclature Rules (v11.3)

| Pattern | Trait | Use For | Examples |
|---------|-------|---------|----------|
| `*Content` | localized | Human-curated localized content | EntityContent, ProjectContent |
| `*Generated` | generated | LLM-generated output content | PageGenerated, BlockGenerated |
| `*Metrics` | aggregated | Computed/aggregated data | SEOKeywordMetrics, GEOMetrics |
| `*Category` | invariant | Categorical groupings | EntityCategory |
| `Locale*` | knowledge | Locale knowledge atoms | LocaleVoice, LocaleCulture |
| `*Set` | invariant | Container nodes for atoms | TermSet, ExpressionSet |

**Deprecated patterns (do not use):**
- `*L10n` - Use `*Content` or `*Generated` instead

## Realm/Layer (v11.3)

| Realm | Layers |
|-------|--------|
| `shared` | locale, geography, knowledge |
| `org` | config, foundation, structure, semantic, instruction, seo, geo, output |

## Trait Selection (v11.3)

| Trait | Use For | Border Style |
|-------|---------|--------------|
| `invariant` | Universal, locale-independent | solid |
| `localized` | Has locale-specific content | dashed |
| `knowledge` | Semantic knowledge atoms | dotted |
| `generated` | LLM-generated output | double |
| `aggregated` | Computed metrics | thin-dotted |

## Example

```bash
/schema:add-node LocaleHumor
```

This will start a Socratic dialog to understand the node's purpose, properties, and relationships before creating it.
