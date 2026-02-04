# Knowledge Atoms v10.1 Design

**Date**: 2026-02-04
**Status**: Draft
**Authors**: Thibaut, Claude

## Summary

Transform the Global realm knowledge nodes from JSON blob containers (`TermSet.terms: json`)
into granular atomic nodes (`Term`, `Expression`, `Taboo`, etc.) to enable:

1. **Selective LLM Loading** — Load only relevant atoms instead of entire JSON blobs
2. **Locale Uniqueness** — Each locale has its own vocabulary without forced parity
3. **Graph Traversal** — Query across locales ("find all locales using this term")

## Key Distinction: Concepts vs Knowledge Atoms

```
CONCEPTS (Page, Block, SEOKeyword...)
├── Invariant parent + L10n children
├── ALL locales MUST have localizations
└── 1 Concept → 200 ConceptL10n (required)

KNOWLEDGE ATOMS (Term, Expression, Taboo...)
├── Locale-native, NO invariant parent
├── Each locale has UNIQUE vocabulary
└── fr-FR: 20,000 terms | sw-KE: 500 terms (variable)
```

## Architecture

```
                      ┌─────────────┐
                      │   Locale    │
                      │  (fr-FR)    │
                      └──────┬──────┘
                             │
         ┌───────────────────┼───────────────────┐
         │ HAS_TERMS         │ HAS_EXPRESSIONS   │ HAS_TABOOS ...
         ▼                   ▼                   ▼
  ┌────────────┐      ┌────────────┐      ┌────────────┐
  │  TermSet   │      │ Expression │      │  TabooSet  │
  │domain=pric │      │    Set     │      │            │
  └─────┬──────┘      └─────┬──────┘      └─────┬──────┘
        │ CONTAINS          │ CONTAINS          │ CONTAINS
  ┌─────┼─────┐       ┌─────┼─────┐       ┌─────┼─────┐
  ▼     ▼     ▼       ▼     ▼     ▼       ▼     ▼     ▼
Term  Term  Term   Expr  Expr  Expr    Taboo Taboo Taboo
```

## Node Types

### Container Nodes (existing, modified)

Keep existing Set nodes as containers/categories. Remove `knowledge_tier` property.

| Set Node       | Purpose                          | Arc from Locale |
|----------------|----------------------------------|-----------------|
| TermSet        | Domain-specific vocabulary       | HAS_TERMS       |
| ExpressionSet  | Idioms, phrases, templates       | HAS_EXPRESSIONS |
| PatternSet     | Content structure patterns       | HAS_PATTERNS    |
| TabooSet       | Forbidden terms/concepts         | HAS_TABOOS      |
| CultureSet     | Cultural references              | HAS_CULTURE     |
| AudienceSet    | Audience segment behaviors       | HAS_AUDIENCE    |

### Atomic Nodes (new)

| Atom Node      | Parent Set     | Key Properties                              |
|----------------|----------------|---------------------------------------------|
| Term           | TermSet        | key, value, domain, register, avoid_context |
| Expression     | ExpressionSet  | key, phrase, tone, formality, use_case      |
| Pattern        | PatternSet     | key, template, context, variables           |
| Taboo          | TabooSet       | key, term, severity, alternatives, reason   |
| CultureRef     | CultureSet     | key, reference, type, context, safe_for     |
| AudienceTrait  | AudienceSet    | key, segment, behavior, preferences         |

### Common Properties (all atoms)

```yaml
standard_properties:
  key:
    type: string
    required: true
    description: "Unique identifier within the Set"

  display_name:
    type: string
    required: true

  description:
    type: string
    required: false

  llm_context:
    type: string
    required: false
    description: "When to use this atom"

  created_at:
    type: datetime
    required: true

  updated_at:
    type: datetime
    required: true
```

## TUI Badge Changes

### Removed

- `[T]` technical tier badge
- `[M]` semantic tier badge
- `[S]` style tier badge

The node type itself indicates the category. No need for redundant badges.

### Kept

- `(n)` instance count — shows data exists for this kind

### Before/After

```
BEFORE:
  Locale Knowledge
    Adaptation [T]
    TermSet [M] (6)
    Style [S]

AFTER:
  Locale Knowledge
    Adaptation
    TermSet (6)
      └── Term (1847)    ← atoms shown with count
    Style
```

## Cypher Examples

### Create Term atom

```cypher
MATCH (l:Locale {key: "fr-FR"})-[:HAS_TERMS]->(ts:TermSet {domain: "pricing"})
CREATE (t:Term {
  key: "subscription_monthly",
  value: "abonnement mensuel",
  domain: "pricing",
  register: "formal",
  created_at: datetime(),
  updated_at: datetime()
})
CREATE (ts)-[:CONTAINS]->(t)
RETURN t
```

### Load terms for LLM context (selective)

```cypher
// Load only pricing terms for fr-FR
MATCH (l:Locale {key: $locale})-[:HAS_TERMS]->(ts:TermSet {domain: "pricing"})
MATCH (ts)-[:CONTAINS]->(t:Term)
RETURN t.key, t.value, t.register
```

### Find term across locales

```cypher
// Which locales have a term for "subscription"?
MATCH (t:Term {key: "subscription_monthly"})<-[:CONTAINS]-(ts:TermSet)<-[:HAS_TERMS]-(l:Locale)
RETURN l.key AS locale, t.value AS translation
```

### Count atoms per locale

```cypher
MATCH (l:Locale)-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS]->(t:Term)
RETURN l.key AS locale, count(t) AS term_count
ORDER BY term_count DESC
```

## Migration Path

1. **Phase 1**: Create atomic node YAMLs in `models/node-kinds/global/knowledge/atoms/`
2. **Phase 2**: Update generators to produce atom Cypher
3. **Phase 3**: Migrate existing JSON blob data to atoms
4. **Phase 4**: Remove `knowledge_tier` from Set YAMLs
5. **Phase 5**: Update TUI to show atoms under Sets

## Files to Create

```
models/node-kinds/global/knowledge/atoms/
├── term.yaml
├── expression.yaml
├── pattern.yaml
├── taboo.yaml
├── culture-ref.yaml
└── audience-trait.yaml
```

## Files to Modify

```
models/node-kinds/global/knowledge/
├── term-set.yaml         # Remove knowledge_tier, add CONTAINS relation
├── expression-set.yaml   # Remove knowledge_tier, add CONTAINS relation
├── pattern-set.yaml      # Remove knowledge_tier, add CONTAINS relation
├── taboo-set.yaml        # Remove knowledge_tier, add CONTAINS relation
├── culture-set.yaml      # Remove knowledge_tier, add CONTAINS relation
└── audience-set.yaml     # Remove knowledge_tier, add CONTAINS relation

tools/novanet/src/tui/ui.rs  # Remove tier_badge display
```

## Open Questions

1. Should atoms have a `locale` property for direct filtering, or rely on graph traversal?
2. Should we add `SIMILAR_TO` arcs between equivalent terms across locales?
3. Maximum atoms per Set before performance concerns?

## References

- ADR-007: Generation, Not Translation
- GraphRAG patterns for selective retrieval
- Neo4j best practices: labels for types, properties for attributes
