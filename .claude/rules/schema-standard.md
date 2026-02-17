# NovaNet Schema Standard v0.13.1

## Canonical YAML Structure

Every node-class YAML file MUST follow this exact structure:

### BLOC Order (6 BLOCs)

```yaml
node:
  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 1: IDENTITY (required, order: name → realm → layer → trait)
  # ═══════════════════════════════════════════════════════════════════════════
  name: NodeName
  realm: shared | org
  layer: config | locale | geography | knowledge | foundation | structure | semantic | instruction | output
  trait: defined | authored | imported | generated | retrieved

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 2: SEMANTIC (required)
  # ═══════════════════════════════════════════════════════════════════════════
  description: "One-line description"

  llm_context: |
    USE: when [primary use case].
    TRIGGERS: "keyword1", "keyword2".
    NOT: for [disambiguation] (use [alternative] instead).
    RELATES: [Source] (role), [Target] (role).

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 3: VISUAL (required)
  # ═══════════════════════════════════════════════════════════════════════════
  icon:
    web: lucide-icon-name
    terminal: "◆"

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 4: DATA (required)
  # ═══════════════════════════════════════════════════════════════════════════
  standard_properties:
    # Order: key → *_key (denormalized) → display_name → description → created_at → updated_at
    key:
      type: string
      required: true
      # ... pattern, examples, etc.

    display_name:
      type: string
      required: true

    description:
      type: string
      required: true

    created_at:
      type: datetime
      required: true

    updated_at:
      type: datetime
      required: true

  properties:
    # Node-specific properties in logical groupings

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 5: GRAPH (optional but recommended)
  # ═══════════════════════════════════════════════════════════════════════════
  # Pattern A (canonical): Arc name as flat key, direction by section
  relations:
    ARC_NAME:
      to: TargetNode
      cardinality: "1:N"
      description: "..."

  incoming_relations:
    ARC_NAME:
      from: SourceNode
      cardinality: "N:1"
      description: "..."

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 6: REFERENCE (optional but recommended)
  # ═══════════════════════════════════════════════════════════════════════════
  example:
    data:
      key: "example-key"
      # ...
    cypher: |
      // Example query
```

## BLOC 5 Relation Patterns

Two patterns exist in the codebase. **Pattern A is canonical** for new files.

### Pattern A (Canonical) — Flat keys, separate sections

Arc name is the YAML key. Direction determined by section (`relations:` = outgoing, `incoming_relations:` = incoming).

```yaml
relations:
  HAS_BLOCK:
    to: Block
    cardinality: "1:N"
    description: "Blocks composing this page"
    props:
      order: { type: integer, required: true }

  REPRESENTS:
    to: Entity
    cardinality: "1:1"
    required: true
    description: "Entity this page represents"

incoming_relations:
  HAS_PAGE:
    from: Project
    cardinality: "1:N"
    description: "Project owning this page"
```

**Files using Pattern A**: block.yaml, page.yaml, project.yaml, locale.yaml (majority)

### Pattern B (Legacy) — Nested lists with `type:` field

Arc name is inside a `type:` field. Direction determined by nesting under `outgoing:` or `incoming:`.

```yaml
relations:
  outgoing:
    - type: HAS_NATIVE
      to: EntityNative
      cardinality: "1:N"
      description: "Locale-specific content"

    - type: BELONGS_TO
      to: EntityCategory
      cardinality: "N:1"
      scope: cross_realm
      description: "Classification category"

  incoming:
    - type: HAS_ENTITY
      from: Project
      cardinality: "1:N"
      description: "Project owning this entity"
```

**Files using Pattern B**: entity.yaml, page-native.yaml

### When to Use Which

| Situation | Pattern |
|-----------|---------|
| New node files | **A** (flat keys) |
| Existing Pattern A files | **A** (keep consistent) |
| entity.yaml / page-native.yaml | **B** (keep as-is, complex arc properties) |
| Migration target | Eventually migrate B → A |

**Key difference**: Pattern B allows list-style arc definitions with richer `props:` blocks inline (e.g., entity.yaml's SEMANTIC_LINK with full `link_type` enum). Pattern A uses a `props:` sub-key for arc properties.

## Standard Properties Order

For ALL nodes:
1. `key` (if node has identity)
2. `*_key` denormalized properties (for composite keys: entity_key, page_key, block_key, locale_key)
3. `display_name`
4. `description`
5. `created_at`
6. `updated_at`

## llm_context: Dual Pattern

`llm_context` exists at TWO distinct levels — same name, different purposes:

### BLOC 2: Schema-level (CLASS directive, required)
```yaml
node:
  llm_context: |
    USE: when [primary use case].
    TRIGGERS: "keyword1", "keyword2".
    NOT: for [disambiguation] (use [alternative] instead).
    RELATES: [Source] (role), [Target] (role).
```
→ Describes how Claude should USE this node CLASS.
→ Generic, applies to ALL instances of this class.
→ Required for all nodes.

### BLOC 4: Instance-level (data property, optional)
```yaml
  properties:
    llm_context:
      type: string
      required: false
      description: "Instance-specific context for LLM generation."
```
→ Contains specific context for THIS particular instance.
→ Optional — not all instances need it, but it can be set in seed files.
→ Best practices: focused (2-4 key points), include data provenance if relevant.
→ Example seed: `s.llm_context = 'URL slugification rules for fr-FR. latin_preserve rule.'`

**Note on standard_properties**: `llm_context` is NOT in standard_properties (it's in `properties:`).
Standard properties order remains: `key → *_key → display_name → description → created_at → updated_at`

## Composite Key Nodes

Nodes with composite keys (`entity:{key}@{locale}`, `page:{key}@{locale}`, `block:{key}@{locale}`) MUST have:
- `key`: Full composite key
- `{parent}_key`: Denormalized parent key (entity_key, page_key, block_key)
- `locale_key`: Denormalized locale key

| Node | Composite Key Pattern | Required Denormalized Props |
|------|----------------------|----------------------------|
| EntityNative | `entity:{key}@{locale}` | entity_key, locale_key |
| PageNative | `page:{key}@{locale}` | page_key, locale_key |
| BlockNative | `block:{key}@{locale}` | block_key, locale_key |

## Nodes Without Key (Satellites)

These 8 nodes are intentionally without `key` — identified by relation chain:

| Node | Identification Pattern |
|------|----------------------|
| ProjectNative | Project→HAS_NATIVE→ProjectNative→FOR_LOCALE→Locale |
| BlockRules | BlockType→HAS_RULES→BlockRules |
| TermSet | Locale→HAS_TERMS→TermSet + domain property |
| ExpressionSet | Locale→HAS_EXPRESSIONS→ExpressionSet + domain property |
| PatternSet | Locale→HAS_PATTERNS→PatternSet + domain property |
| CultureSet | Locale→HAS_CULTURE→CultureSet + domain property |
| TabooSet | Locale→HAS_TABOOS→TabooSet + domain property |
| AudienceSet | Locale→HAS_AUDIENCE→AudienceSet + domain property |

## Trait Definitions (ADR-024)

| Trait | Who Creates | Examples |
|-------|-------------|----------|
| defined | Human, ONCE | Page, Block, Entity, Locale |
| authored | Human, PER locale | EntityNative, ProjectNative |
| imported | External data | Term, SEOKeyword, GEOQuery |
| generated | Our LLM | PageNative, BlockNative |
| retrieved | External APIs | GEOAnswer, SEOKeywordMetrics |

## Validation Rules

The Rust `schema_rules` module enforces these rules in CI:

1. **KEY_REQUIRED**: Non-satellite nodes must have `key` in standard_properties
2. **DENORM_REQUIRED**: Composite key nodes must have denormalized properties
3. **TIMESTAMP_REQUIRED**: All nodes must have `created_at` and `updated_at`
4. **PROP_ORDER**: Standard properties must be in canonical order

Run validation: `cargo run -- schema validate --strict`
