---
description: Add a new arc type to the NovaNet knowledge graph
argument-hint: <ARC_NAME>
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, AskUserQuestion
---

Add a new arc type (directed relationship) between nodes in the NovaNet ontology.

## Terminology

> **Arc** = directed link between nodes (graph theory term for directed edges)
> **ArcKind** = arc type definition (e.g., HAS_BLOCK, USES_ENTITY)
> **ArcFamily** = classification of arc types (ownership, localization, semantic, generation, mining)

## Workflow

1. **Discovery Phase**: Ask about the arc:
   - From which node type(s)? (source Kind)
   - To which node type(s)? (target Kind)
   - What ArcFamily? (ownership, localization, semantic, generation, mining)
   - What is the cardinality? (one_to_one, one_to_many, many_to_many)
   - Does it need properties? (temperature, position, etc.)

2. **Classification**: Determine ArcFamily:
   - **ownership**: Structural containment (HAS_PAGE, HAS_BLOCK, OF_TYPE)
   - **localization**: Locale-specific variants (HAS_L10N, FOR_LOCALE)
   - **semantic**: Content relationships (USES_ENTITY, SEMANTIC_LINK)
   - **generation**: AI-generated content (HAS_OUTPUT, HAS_PROMPT)
   - **mining**: SEO/GEO data (HAS_SEO_TARGET, HAS_GEO_TARGET)

3. **Bidirectionality Check**: Does it need an inverse?
   - HAS_L10N needs L10N_OF
   - HAS_BLOCK needs BLOCK_OF
   - Not all arcs need inverses

4. **Creation Phase**:
   - Add to `packages/core/models/arc-kinds/{family}/{arc-name}.yaml`
   - Update source node YAML with arc reference
   - Run `novanet schema generate`
   - Run `novanet schema validate`

5. **Seed Phase** (optional):
   - Create migration to add arc to existing data
   - Update seed files with arc creation

## ArcFamily Reference

| Family | Purpose | Arrow Style | Examples |
|--------|---------|-------------|----------|
| `ownership` | Structural containment | `-->` | HAS_PAGE, HAS_BLOCK, OF_TYPE |
| `localization` | Locale variants | `-.->` | HAS_L10N, FOR_LOCALE |
| `semantic` | Content relationships | `-.->` | USES_ENTITY, SEMANTIC_LINK |
| `generation` | AI-generated | `==>` | HAS_OUTPUT, HAS_PROMPT |
| `mining` | SEO/GEO data | `--o` | HAS_SEO_TARGET, HAS_GEO_TARGET |

## Naming Conventions

| Pattern | Use For | Examples |
|---------|---------|----------|
| `HAS_*` | Ownership/containment | HAS_PAGE, HAS_BLOCK |
| `HAS_L10N` | Localized content (human-curated) | Entity→EntityL10n |
| `HAS_OUTPUT` | Localized content (LLM-generated) | Page→PageL10n |
| `*_OF` | Inverse of HAS_* | L10N_OF, BLOCK_OF, OUTPUT_OF |
| `FOR_*` | Target association | FOR_LOCALE |
| `USES_*` | Reference/usage | USES_ENTITY |
| `EXPRESSES` | SEO targeting | EntityL10n→SEOKeyword |

## Naming Requirements

- **Arc names**: UPPER_SNAKE_CASE (e.g., `HAS_HUMOR`, `FOR_LOCALE`)
- **Node names in source/target**: PascalCase (e.g., `LocaleLexicon`, `EntityL10n`)

## Example

```bash
/schema:add-arc HAS_HUMOR
```

This will start a dialog to understand the arc between LocaleLexicon and LocaleHumor (or whatever nodes are involved).

## YAML Structure (v10.3)

```yaml
# packages/core/models/arc-kinds/semantic/uses-entity.yaml
arc:
  name: USES_ENTITY
  family: semantic
  scope: cross_realm
  cardinality: many_to_many
  source: [Page, Block]
  target: Entity
  description: "Content references a semantic entity"
  properties:
    temperature:
      type: float
      required: true
      description: "Relevance temperature for spreading activation (0.0-1.0)"
```
