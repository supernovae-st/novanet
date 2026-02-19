---
name: novanet-yaml
description: Comprehensive guide for writing perfect NovaNet node-class and arc-class YAML files. Use when creating, editing, or debugging schema YAML. Covers 6 BLOCs structure, llm_context patterns, icon dual format, relation patterns, traits, and validation rules.
user-invocable: true
---

# NovaNet YAML Schema Authoring

> Perfect node/arc YAML definitions every time. Follows schema-standard.md exactly.

## File Location Rules

All YAML files MUST follow this path structure:

```
packages/core/models/
├── node-classes/
│   ├── shared/           # Realm: shared (40 nodes)
│   │   ├── config/       # Layer: config
│   │   ├── locale/       # Layer: locale
│   │   ├── geography/    # Layer: geography
│   │   └── knowledge/    # Layer: knowledge
│   └── org/              # Realm: org (21 nodes)
│       ├── config/       # Layer: config
│       ├── foundation/   # Layer: foundation
│       ├── structure/    # Layer: structure
│       ├── semantic/     # Layer: semantic
│       ├── instruction/  # Layer: instruction
│       └── output/       # Layer: output
└── arc-classes/
    └── {family}/         # ownership, localization, semantic, generation, mining, schema
```

**Path MUST match content:** `models/node-classes/shared/knowledge/term.yaml` MUST have `realm: shared` and `layer: knowledge`.

---

## The 6 BLOCs (Required Order)

Every node-class YAML MUST follow this exact structure:

```yaml
node:
  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 1: IDENTITY (required, exact order: name → realm → layer → trait)
  # ═══════════════════════════════════════════════════════════════════════════
  name: NodeName
  realm: shared | org
  layer: config | locale | geography | knowledge | foundation | structure | semantic | instruction | output
  trait: defined | authored | imported | generated | retrieved

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 2: SEMANTIC (required)
  # ═══════════════════════════════════════════════════════════════════════════
  description: "One-line description ending with period."

  llm_context: |
    USE: when [primary use case].
    TRIGGERS: "keyword1", "keyword2", "keyword3".
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
    # EXACT ORDER: key → *_key → display_name → description → created_at → updated_at

  properties:
    # Node-specific properties in logical groupings

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 5: GRAPH (optional but recommended)
  # ═══════════════════════════════════════════════════════════════════════════
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
    cypher: |
      // Example query
```

---

## Traits (ADR-024: Data Origin)

Trait answers: "WHO creates this data and HOW OFTEN?"

| Trait | Who Creates | Frequency | Examples |
|-------|-------------|-----------|----------|
| `defined` | Human | ONCE (invariant) | Page, Block, Entity, Locale |
| `authored` | Human | PER locale | EntityNative, ProjectNative |
| `imported` | External data | Brought in | Term, SEOKeyword, GEOQuery |
| `generated` | Our LLM | PER locale | PageNative, BlockNative |
| `retrieved` | External APIs | Fetched | GEOAnswer, SEOKeywordMetrics |

---

## Realms and Layers (Architecture)

```
SHARED REALM (40 nodes, READ-ONLY)     ORG REALM (21 nodes)
├── config     (3)  OrgConfig          ├── config      (1)
├── locale     (6)  Locale settings    ├── foundation  (6)  Project, Brand
├── geography  (7)  Geographic data    ├── structure   (3)  Page, Block
└── knowledge (24)  Knowledge atoms    ├── semantic    (4)  Entity, EntityNative
                                       ├── instruction (4)  PageStructure
                                       └── output      (3)  PageNative
```

**Realm determines editability:**
- `shared`: Universal, READ-ONLY (can't be edited per-org)
- `org`: Organization-specific, editable

---

## llm_context Pattern (CRITICAL)

The `llm_context` field tells Claude HOW to use this node class. Use this exact pattern:

```yaml
llm_context: |
  USE: when [primary scenario for using this node].
  TRIGGERS: "keyword1", "keyword2", "keyword3".
  NOT: for [what NOT to use this for] (use [alternative] instead).
  RELATES: [Source] (role), [Target] (role), [Related] (relationship).
```

### Good Example (Entity)

```yaml
llm_context: |
  USE: when defining semantic concepts that exist across all locales.
  TRIGGERS: "entity", "concept", "definition", "semantic".
  NOT: for locale-specific content (use EntityNative instead).
  RELATES: Project (owner), EntityNative (locale content), EntityCategory (classification).
```

### Good Example (HAS_NATIVE arc)

```yaml
llm_context: |
  USE: when loading locale-specific content for a defined node.
  TRIGGERS: "content", "native", "locale", "localized", "l10n".
  NOT: for structure (use HAS_BLOCK), for definitions (read the invariant).
  RELATES: Entity (parent), EntityNative (locale content), FOR_LOCALE (locale link).
```

---

## Icon Dual Format (NO EMOJI)

Icons MUST use the dual web/terminal format. Never use emoji.

```yaml
# ✅ CORRECT
icon:
  web: file-text      # Lucide icon name
  terminal: "◆"       # Unicode symbol

# ❌ WRONG
icon: "📄"            # No emoji
icon: file-text       # Missing terminal variant
```

### Common Icon Mappings

| Node Type | web (Lucide) | terminal |
|-----------|--------------|----------|
| Page | `file-text` | `◆` |
| Block | `box` | `■` |
| Entity | `gem` | `◇` |
| Locale | `globe` | `🌍` |
| Term | `tag` | `#` |
| Project | `folder` | `📁` |

---

## Standard Properties (EXACT Order)

For nodes WITH identity key:

```yaml
standard_properties:
  key:
    type: string
    required: true
    pattern: "^[a-z][a-z0-9-]*$"
    description: "Unique identifier (kebab-case)"
    examples: ["my-entity", "home-page"]

  display_name:
    type: string
    required: true
    description: "Human-readable name"

  description:
    type: string
    required: true
    description: "Brief description of this instance"

  created_at:
    type: datetime
    required: true
    description: "Creation timestamp (ISO 8601)"

  updated_at:
    type: datetime
    required: true
    description: "Last modification timestamp (ISO 8601)"
```

For COMPOSITE KEY nodes (EntityNative, PageNative, BlockNative):

```yaml
standard_properties:
  key:
    type: string
    required: true
    pattern: "^entity:[a-z][a-z0-9-]*@[a-z]{2}-[A-Z]{2}$"
    description: "Composite key: entity:{key}@{locale}"
    examples: ["entity:qr-code@fr-FR"]

  entity_key:              # Denormalized parent key
    type: string
    required: true

  locale_key:              # Denormalized locale key
    type: string
    required: true

  display_name:
    type: string
    required: true

  # ... rest of standard properties
```

---

## Relation Patterns

### Pattern A (Canonical) — Use for new files

Arc name as YAML key, direction by section:

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

### Pattern B (Legacy) — Keep for complex files

Nested lists with `type:` field (entity.yaml, page-native.yaml):

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

---

## Cardinality Rules

| Pattern | Meaning | Example |
|---------|---------|---------|
| `1:1` | Exactly one | Page REPRESENTS Entity |
| `1:N` | Parent → many children | Project HAS_PAGE Pages |
| `N:1` | Child → one parent | Page HAS_PROJECT Project |
| `N:M` | Many-to-many | Entity SEMANTIC_LINK Entity |

---

## Nodes Without Key (Satellites)

These 8 nodes are identified by relation chain, not key:

| Node | Identification |
|------|----------------|
| ProjectNative | `Project→HAS_NATIVE→ProjectNative→FOR_LOCALE→Locale` |
| BlockRules | `BlockType→HAS_RULES→BlockRules` |
| TermSet | `Locale→HAS_TERMS→TermSet` + domain property |
| ExpressionSet | `Locale→HAS_EXPRESSIONS→ExpressionSet` + domain |
| PatternSet | `Locale→HAS_PATTERNS→PatternSet` + domain |
| CultureSet | `Locale→HAS_CULTURE→CultureSet` + domain |
| TabooSet | `Locale→HAS_TABOOS→TabooSet` + domain |
| AudienceSet | `Locale→HAS_AUDIENCE→AudienceSet` + domain |

---

## Common Mistakes and Fixes

### 1. Path doesn't match content

```yaml
# File: models/node-classes/org/semantic/entity.yaml
# ❌ WRONG
realm: shared
layer: knowledge

# ✅ CORRECT (must match path)
realm: org
layer: semantic
```

### 2. Wrong property order

```yaml
# ❌ WRONG
standard_properties:
  display_name:    # Should come after key
    type: string
  key:
    type: string

# ✅ CORRECT
standard_properties:
  key:
    type: string
  display_name:
    type: string
```

### 3. Missing llm_context

```yaml
# ❌ WRONG - llm_context required
node:
  name: MyNode
  realm: org
  layer: foundation
  trait: defined
  description: "My node description"
  icon: ...

# ✅ CORRECT
node:
  name: MyNode
  realm: org
  layer: foundation
  trait: defined
  description: "My node description"
  llm_context: |
    USE: when [use case].
    TRIGGERS: "keyword1", "keyword2".
    NOT: for [alternative use] (use [other] instead).
    RELATES: [Related] (role).
  icon: ...
```

### 4. Emoji in icon

```yaml
# ❌ WRONG
icon: "📄"
icon:
  web: "📄"
  terminal: "📄"

# ✅ CORRECT
icon:
  web: file-text
  terminal: "◆"
```

### 5. Missing timestamps

```yaml
# ❌ WRONG - missing created_at/updated_at
standard_properties:
  key: ...
  display_name: ...
  description: ...

# ✅ CORRECT
standard_properties:
  key: ...
  display_name: ...
  description: ...
  created_at:
    type: datetime
    required: true
  updated_at:
    type: datetime
    required: true
```

---

## Complete Example (Entity Node)

```yaml
node:
  # BLOC 1: IDENTITY
  name: Entity
  realm: org
  layer: semantic
  trait: defined

  # BLOC 2: SEMANTIC
  description: "Semantic concept that exists invariantly across all locales."

  llm_context: |
    USE: when defining concepts that transcend locale (the "what" not the "how to say it").
    TRIGGERS: "entity", "concept", "definition", "semantic", "invariant".
    NOT: for locale-specific content (use EntityNative), for page content (use Page).
    RELATES: Project (owner), EntityNative (locale content), EntityCategory (classification).

  # BLOC 3: VISUAL
  icon:
    web: gem
    terminal: "◇"

  # BLOC 4: DATA
  standard_properties:
    key:
      type: string
      required: true
      pattern: "^[a-z][a-z0-9-]*$"
      description: "Unique semantic identifier"
      examples: ["qr-code", "data-matrix", "barcode"]

    display_name:
      type: string
      required: true
      description: "Human-readable entity name"

    description:
      type: string
      required: true
      description: "Semantic definition of this entity"

    created_at:
      type: datetime
      required: true

    updated_at:
      type: datetime
      required: true

  properties:
    primary_locale:
      type: string
      required: true
      description: "Default locale for this entity (BCP-47)"
      examples: ["en-US", "fr-FR"]

    entity_type:
      type: string
      enum: [concept, product, service, organization, person, place]
      required: true
      description: "Classification of entity type"

  # BLOC 5: GRAPH
  relations:
    outgoing:
      - type: HAS_NATIVE
        to: EntityNative
        cardinality: "1:N"
        description: "Locale-specific content for this entity"

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

      - type: REPRESENTS
        from: Page
        cardinality: "1:1"
        description: "Page that represents this entity"

  # BLOC 6: REFERENCE
  example:
    data:
      key: "qr-code"
      display_name: "QR Code"
      description: "Two-dimensional barcode that stores data in a pattern of black and white squares."
      primary_locale: "en-US"
      entity_type: "concept"
    cypher: |
      MATCH (e:Entity {key: 'qr-code'})-[:HAS_NATIVE]->(en:EntityNative)-[:FOR_LOCALE]->(l:Locale)
      RETURN e.key, en.title, l.key as locale
```

---

## Validation Commands

```bash
# Validate all YAML
cargo run -- schema validate

# Validate with strict mode (path/content matching)
cargo run -- schema validate --strict

# Regenerate all artifacts from YAML
cargo run -- schema generate

# Seed database with validated schema
cargo run -- db seed
```

---

## Quick Reference Card

| Element | Rule |
|---------|------|
| File path | `models/node-classes/{realm}/{layer}/{name}.yaml` |
| BLOC order | Identity → Semantic → Visual → Data → Graph → Reference |
| Property order | key → *_key → display_name → description → created_at → updated_at |
| llm_context | USE: → TRIGGERS: → NOT: → RELATES: |
| Icon format | `{ web: "lucide-name", terminal: "◆" }` |
| Trait | defined / authored / imported / generated / retrieved |
| Realm | shared (40, READ-ONLY) / org (21) |
| Layers | 4 shared + 6 org = 10 total |
