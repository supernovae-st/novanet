# NovaNet Arc Design Best Practices

This guide documents best practices for designing, naming, and documenting arcs in NovaNet.

## Arc Families

NovaNet organizes arcs into 5 families based on their semantic purpose:

| Family | Purpose | Key Arcs |
|--------|---------|----------|
| **ownership** | Parent→Child hierarchy | HAS_PAGE, HAS_ENTITY, HAS_BLOCK |
| **localization** | Locale relationships | FOR_LOCALE, HAS_VOICE, HAS_CULTURE |
| **semantic** | Meaning relationships | USES_ENTITY, REFERENCES, MENTIONS |
| **generation** | LLM pipeline | GENERATED, COMPILED_FROM, ASSEMBLES |
| **mining** | SEO/GEO intelligence | TARGETS_KEYWORD, MONITORS_GEO |

## Naming Conventions

### Forward Arcs (HAS_*)

Use `HAS_*` prefix for ownership arcs (parent→child):

```yaml
arc:
  name: HAS_PAGE       # Project → Page
  name: HAS_ENTITY     # Project → Entity
  name: HAS_BLOCK      # Page → Block
```

### Inverse Arcs (*_OF)

Use `*_OF` suffix for inverse ownership:

```yaml
arc:
  name: PAGE_OF        # Page → Project (inverse of HAS_PAGE)
  name: ENTITY_OF      # Entity → Project (inverse of HAS_ENTITY)
  name: BLOCK_OF       # Block → Page (inverse of HAS_BLOCK)
```

### Container Arcs (CONTAINS_*)

Use `CONTAINS_*` prefix for container→atom relationships:

```yaml
arc:
  name: CONTAINS_TERM        # TermSet → Term
  name: CONTAINS_EXPRESSION  # ExpressionSet → Expression
```

**Note**: Container arcs are TIER 3 (no inverse needed) per ADR-026.

### Action Arcs (VERB or VERB_NOUN)

Use verb form for semantic relationships:

```yaml
arc:
  name: USES_ENTITY    # Page → Entity (semantic reference)
  name: TARGETS_KEYWORD # Page → SEOKeyword (SEO targeting)
  name: MONITORS_GEO   # EntityNative → GEOQuery (GEO monitoring)
```

## llm_context Pattern

Every arc MUST have an llm_context field following this pattern:

```yaml
llm_context: |
  USE: when [primary use case for traversing this arc].
  TRIGGERS: "keyword1", "keyword2", "keyword3".
  NOT: for [disambiguation] (use [alternative] instead).
  RELATES: [Source] (role), [Target] (role), [Related Arc] (relationship).
```

### Example: HAS_ENTITY

```yaml
llm_context: |
  USE: when loading entities for a project.
  TRIGGERS: project entities, semantic concepts, project vocabulary.
  NOT: entity content (use HAS_NATIVE), entity hierarchy (use HAS_CHILD).
  RELATES: Project (source), Entity (target), ENTITY_OF (inverse).
```

### Guidelines

1. **USE**: Describe the primary traversal scenario
2. **TRIGGERS**: List 3-5 keywords that should activate this arc in RAG
3. **NOT**: Disambiguate from similar arcs with clear alternatives
4. **RELATES**: Document source, target, and related arcs

## Inverse Arc Policy (ADR-026)

### TIER 1: Required Inverses

Core ownership arcs with frequent bidirectional traversal:

| Forward | Inverse | When to Create |
|---------|---------|----------------|
| HAS_ENTITY | ENTITY_OF | Always |
| HAS_PAGE | PAGE_OF | Always |
| HAS_PROJECT | PROJECT_OF | Always |
| HAS_BLOCK | BLOCK_OF | Always |
| HAS_NATIVE | NATIVE_OF | Always |
| HAS_NATIVE | NATIVE_OF | Always |
| HAS_CHILD | CHILD_OF | Always |
| HAS_INSTRUCTION | INSTRUCTION_OF | Always |

### TIER 2: Recommended Inverses

Knowledge/locale traversal arcs (high LLM context value):

| Forward | Inverse | When to Create |
|---------|---------|----------------|
| HAS_TERMS | TERMS_OF | When needed for spreading activation |
| USES_ENTITY | USED_BY | When "what uses this?" is common |
| FOR_LOCALE | LOCALE_OF | When locale-based traversal is needed |

### TIER 3: No Inverse Needed

- Configuration arcs: `BELONGS_TO_ORG`, `SUPPORTS_LOCALE`
- Type arcs: `OF_TYPE`, `HAS_STRUCTURE`
- Container arcs: `CONTAINS_*`

## Arc YAML Structure

### Forward Arc (declares inverse)

```yaml
# packages/core/models/arc-classes/ownership/has-entity.yaml
arc:
  name: HAS_ENTITY
  family: ownership
  scope: intra_realm
  source: Project
  target: Entity
  cardinality: one_to_many
  inverse: ENTITY_OF              # <-- Declares inverse

  llm_context: |
    USE: when loading entities for a project.
    TRIGGERS: project entities, semantic concepts, project vocabulary.
    NOT: entity content (use HAS_NATIVE), entity hierarchy (use HAS_CHILD).
    RELATES: Project (source), Entity (target), ENTITY_OF (inverse).

  cypher_pattern: (Project)-[:HAS_ENTITY]->(Entity)
```

### Inverse Arc (references forward)

```yaml
# packages/core/models/arc-classes/ownership/entity-of.yaml
arc:
  name: ENTITY_OF
  family: ownership
  scope: intra_realm
  source: Entity
  target: Project
  cardinality: many_to_one
  inverse_of: HAS_ENTITY          # <-- References forward arc

  llm_context: |
    USE: when finding which Project owns an Entity.
    TRIGGERS: entity owner, entity project, which project.
    NOT: loading entities for a project (use HAS_ENTITY).
    RELATES: Entity (source), Project (target), HAS_ENTITY (inverse).

  cypher_pattern: (Entity)-[:ENTITY_OF]->(Project)
```

## Cardinality Rules

| Cardinality | Symbol | Example |
|-------------|--------|---------|
| `one_to_one` | 1:1 | Locale → Style (one style per locale) |
| `one_to_many` | 1:N | Project → Page (project has many pages) |
| `many_to_one` | N:1 | Entity → EntityCategory (many entities per category) |
| `many_to_many` | N:M | Page → Entity (USES_ENTITY - many-to-many) |

### Inverse Cardinality

When creating an inverse, flip the cardinality:

| Forward | Inverse |
|---------|---------|
| one_to_many | many_to_one |
| many_to_one | one_to_many |
| one_to_one | one_to_one |
| many_to_many | many_to_many |

## Scope Rules

| Scope | When to Use |
|-------|-------------|
| `intra_realm` | Both source and target in same realm (org→org or shared→shared) |
| `cross_realm` | Source and target in different realms (org→shared) |

### Examples

```yaml
# Intra-realm (both in org)
HAS_PAGE: Project (org) → Page (org)

# Cross-realm (org references shared)
FOR_LOCALE: EntityNative (org) → Locale (shared)
INCLUDES_STYLE: PageInstruction (org) → Style (shared)
```

## Validation

Run schema validation after creating/modifying arcs:

```bash
cargo run -- schema validate
```

Validation checks:
- Arc references valid source/target node types
- `inverse:` field points to existing arc
- `inverse_of:` matches the forward arc's `inverse:` declaration
- No circular inverse references

## Common Mistakes

### 1. Missing inverse_of

```yaml
# WRONG: Inverse doesn't reference forward arc
arc:
  name: ENTITY_OF
  # Missing inverse_of: HAS_ENTITY

# CORRECT:
arc:
  name: ENTITY_OF
  inverse_of: HAS_ENTITY
```

### 2. Wrong cardinality on inverse

```yaml
# Forward: one_to_many
HAS_PAGE: Project (1) → Page (N)

# WRONG: Inverse should be many_to_one
PAGE_OF: Page (1) → Project (N)  # Incorrect!

# CORRECT:
PAGE_OF: Page (N) → Project (1)  # many_to_one
```

### 3. Unclear llm_context

```yaml
# WRONG: Generic, doesn't help disambiguation
llm_context: "Links page to project"

# CORRECT: Clear USE/TRIGGERS/NOT/RELATES
llm_context: |
  USE: when finding which Project owns a Page.
  TRIGGERS: page owner, page project, which project.
  NOT: loading pages for a project (use HAS_PAGE).
  RELATES: Page (source), Project (target), HAS_PAGE (inverse).
```

## References

- ADR-026: Inverse Arc Policy
- ADR-027: Generation Family Arc Semantics
- `packages/core/models/arc-classes/` - All arc definitions
