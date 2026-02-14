# Entity-Centric v10.3 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Migrate NovaNet from Concept-centric to Entity-centric architecture with proper SEO structuring and instruction model.

**Architecture:** Rename Concept→Entity, add EntityType enum, create BlockInstruction node, invert SEO arc direction (SEOKeyword EXPRESSES Entity), simplify internal linking.

**Tech Stack:** YAML schemas, Rust generators, TypeScript types, Cypher seeds

**Design Doc:** `docs/plans/2026-02-04-entity-centric-v10.3-design.md`

---

## Phase 1: Node Schema Updates

### Task 1: Create Entity node (rename from Concept)

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/entity.yaml`
- Delete: `packages/core/models/node-classes/project/semantic/concept.yaml` (after migration)

**Step 1: Create the Entity node YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/entity.yaml
# Entity - Universal semantic entity (replaces Concept in v10.3)

node:
  name: Entity
  realm: global
  layer: knowledge
  trait: invariant
  icon: "🔷"
  description: |
    Universal semantic entity aligned with schema.org Thing.
    Represents any concept, action, feature, brand, place, or audience
    that can be expressed by keywords and materialized as pages.

  standard_properties:
    key:
      type: string
      required: true
      description: "Unique identifier (kebab-case)"
      pattern: "^[a-z0-9-]+$"
      examples:
        - "qr-code-generator"
        - "dynamic-qr"
        - "analytics-feature"

    display_name:
      type: string
      required: true
      description: "Human-readable UI name (invariant, for admin)"
      example: "QR Code Generator"

    description:
      type: string
      required: true
      description: "One-line description of the entity"
      example: "Tool for generating QR codes from URLs, text, or contacts"

    llm_context:
      type: string
      required: true
      description: "LLM generation hints"
      example: "USE: when user wants to create QR codes. TRIGGERS: qr, code, generate."

    created_at:
      type: datetime
      required: true

    updated_at:
      type: datetime
      required: true

  properties:
    type:
      type: string
      required: true
      enum: [ACTION, THING, FEATURE, BRAND, PLACE, AUDIENCE, GUIDE, COMPARISON]
      description: "Semantic entity type aligned with schema.org"
      example: "THING"

    is_pillar:
      type: boolean
      required: false
      default: false
      description: "Whether this is a pillar entity (has cluster children via SUBTOPIC_OF)"
      example: true

    schema_org_type:
      type: string
      required: false
      description: "Optional schema.org type (e.g., 'Product', 'HowTo', 'SoftwareApplication')"
      example: "SoftwareApplication"

    wikidata_id:
      type: string
      required: false
      description: "Optional Wikidata Q-identifier for knowledge graph linking"
      example: "Q12345"

  embedding_properties:
    embedding:
      type: vector
      dimensions: 1536
      required: false
      description: "OpenAI text-embedding-3-small vector for semantic search"

    embedding_source:
      type: string
      required: false
      description: "Text used to generate embedding"

    embedding_updated_at:
      type: datetime
      required: false

  relations:
    HAS_L10N:
      to: EntityL10n
      cardinality: "1:N"
      description: "Localized content for this entity (one per locale)"

    SUBTOPIC_OF:
      to: Entity
      cardinality: "N:1"
      description: "This entity is a subtopic of a pillar entity"

    SEMANTIC_LINK:
      to: Entity
      cardinality: "N:N"
      props:
        temperature:
          type: float
          description: "Activation strength 0.0-1.0"
        link_type:
          type: string
          enum: [is_action_on, includes, type_of, used_for, contrasts_with]
      description: "Spreading activation links to related entities"

  example:
    data:
      key: "qr-code-generator"
      display_name: "QR Code Generator"
      description: "Tool for generating QR codes from URLs, text, or contacts"
      llm_context: "USE: when user wants to create QR codes. TRIGGERS: qr, code, generate, create."
      type: "THING"
      is_pillar: true
      schema_org_type: "SoftwareApplication"
```

**Step 2: Verify YAML syntax**

Run: `cat packages/core/models/node-classes/global/knowledge/entity.yaml | head -20`
Expected: Valid YAML header displayed

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/entity.yaml
git commit -m "feat(schema): add Entity node (replaces Concept in v10.3)

- New global/knowledge/entity.yaml
- EntityType enum: ACTION, THING, FEATURE, BRAND, PLACE, AUDIENCE, GUIDE, COMPARISON
- is_pillar for pillar/cluster structure
- schema_org_type and wikidata_id for knowledge graph linking

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 2: Create EntityL10n node (rename from ConceptL10n)

**Files:**
- Create: `packages/core/models/node-classes/global/knowledge/entity-l10n.yaml`
- Delete: `packages/core/models/node-classes/project/semantic/concept-l10n.yaml` (after migration)

**Step 1: Create the EntityL10n node YAML**

```yaml
# packages/core/models/node-classes/global/knowledge/entity-l10n.yaml
# EntityL10n - Localized content for Entity (replaces ConceptL10n in v10.3)

node:
  name: EntityL10n
  realm: global
  layer: knowledge
  trait: localized
  icon: "🟢"
  description: |
    Localized content for an Entity in a specific locale.
    The display_name changes per locale (e.g., "QR Code Generator" vs "Générateur de QR Code").

  standard_properties:
    display_name:
      type: string
      required: true
      description: "Localized display name for the entity"
      example: "Générateur de QR Code"

    description:
      type: string
      required: true
      description: "Localized description of the entity"
      example: "Outil pour générer des QR codes à partir d'URLs, texte ou contacts"

    llm_context:
      type: string
      required: true
      description: "Localized LLM generation hints"
      example: "USE: contenu natif français pour génération. TRIGGERS: fr-FR content needed."

    created_at:
      type: datetime
      required: true

    updated_at:
      type: datetime
      required: true

  properties:
    definition:
      type: string
      required: false
      description: "What it is - localized definition"
      example: "Générer un QR code à partir d'une URL, texte ou contact."

    purpose:
      type: string
      required: false
      description: "Why it exists - localized purpose"
      example: "Convertir une information en format scannable par smartphone."

    benefits:
      type: string[]
      required: false
      description: "Value props in target locale"
      example: ["Accès rapide à l'information", "Sans contact, hygiénique"]

    cultural_notes:
      type: string
      required: false
      description: "Locale-specific cultural considerations"
      example: "En France, préférer 'QR Code' à 'code QR'."

    version:
      type: int
      required: true
      description: "Content version for staleness detection"
      example: 1

  embedding_properties:
    embedding:
      type: vector
      dimensions: 1536
      required: false
      description: "OpenAI text-embedding-3-small vector"

    embedding_source:
      type: string
      required: false

    embedding_updated_at:
      type: datetime
      required: false

  relations:
    FOR_LOCALE:
      to: Locale
      cardinality: "N:1"
      description: "Links to the locale this content is for"

  incoming_relations:
    - relation: HAS_L10N
      from: Entity
      cardinality: "1:N"
      description: "Parent entity that owns this localization"

  example:
    display_name: "Générateur de QR Code"
    description: "Outil pour générer des QR codes à partir d'URLs, texte ou contacts"
    llm_context: "USE: contenu natif français pour génération. TRIGGERS: fr-FR content needed."
    definition: "Générer un QR code à partir d'une URL, texte ou contact."
    purpose: "Convertir une information en format scannable par smartphone."
    benefits:
      - "Accès rapide à l'information"
      - "Sans contact, hygiénique"
    cultural_notes: "En France, préférer 'QR Code' à 'code QR'."
    version: 1
```

**Step 2: Verify YAML syntax**

Run: `cat packages/core/models/node-classes/global/knowledge/entity-l10n.yaml | head -20`
Expected: Valid YAML header displayed

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/knowledge/entity-l10n.yaml
git commit -m "feat(schema): add EntityL10n node (replaces ConceptL10n in v10.3)

- New global/knowledge/entity-l10n.yaml
- display_name changes per locale
- definition, purpose, benefits, cultural_notes fields

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 3: Create BlockInstruction node

**Files:**
- Create: `packages/core/models/node-classes/project/instruction/block-instruction.yaml`

**Step 1: Create the BlockInstruction node YAML**

```yaml
# packages/core/models/node-classes/project/instruction/block-instruction.yaml
# BlockInstruction - Instructions for generating a block within a page (v10.3)

node:
  name: BlockInstruction
  realm: project
  layer: instruction
  trait: invariant
  icon: "📝"
  description: |
    Instructions for generating a specific block within a page.
    Contains markdown with @entity: and @link: references.
    Multiple BlockInstructions are concatenated at runtime to form PageInstruction.

    Syntax:
    - @entity:key references an Entity
    - @link:key references a Page
    - [FIXED] = invariant content, identical across locales
    - [GENERATE] = LLM generates with creative freedom
    - [TRANSLATE] = close to original, less creative freedom

  standard_properties:
    key:
      type: string
      required: true
      description: "Unique identifier (page-key_block-type format)"
      pattern: "^[a-z0-9-]+_[a-z0-9-]+$"
      example: "qr-generator_hero"

    display_name:
      type: string
      required: true
      description: "Human-readable name"
      example: "QR Generator Hero Instructions"

    description:
      type: string
      required: true
      description: "Brief description of these instructions"
      example: "Hero section instructions for the QR Generator page"

    llm_context:
      type: string
      required: true
      description: "Context for LLM when processing these instructions"

    created_at:
      type: datetime
      required: true

    updated_at:
      type: datetime
      required: true

  properties:
    content:
      type: string
      required: true
      description: "Markdown content with @entity:key, @link:key, [FIXED], [GENERATE], [TRANSLATE] tags"
      example: |
        ## Hero Section [GENERATE]

        Create a compelling hero for @entity:qr-code-generator.
        Link to @link:pricing for conversion.

    order:
      type: int
      required: true
      description: "Order in page compilation (1, 2, 3...)"
      example: 1

  relations:
    OF_TYPE:
      to: BlockType
      cardinality: "N:1"
      description: "The block type (hero, features, faq, etc.) this instruction is for"

    REFERENCES_ENTITY:
      to: Entity
      cardinality: "N:M"
      description: "Entities referenced via @entity:key syntax (auto-parsed)"

    REFERENCES_PAGE:
      to: Page
      cardinality: "N:M"
      description: "Pages referenced via @link:key syntax (auto-parsed)"

  incoming_relations:
    - relation: HAS_INSTRUCTION
      from: Page
      cardinality: "1:N"
      description: "Page that owns this instruction"

  example:
    data:
      key: "qr-generator_hero"
      display_name: "QR Generator Hero Instructions"
      description: "Hero section instructions for the QR Generator page"
      llm_context: "Generate compelling hero content for QR code generator tool"
      content: |
        ## Hero Section [GENERATE]

        Create a compelling hero for @entity:qr-code-generator.
        Highlight the main value proposition.
        Link to @link:pricing for conversion.
      order: 1
```

**Step 2: Verify YAML syntax**

Run: `cat packages/core/models/node-classes/project/instruction/block-instruction.yaml | head -20`
Expected: Valid YAML header displayed

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/project/instruction/block-instruction.yaml
git commit -m "feat(schema): add BlockInstruction node for v10.3 instruction model

- Page HAS_INSTRUCTION BlockInstruction OF_TYPE BlockType
- @entity:key and @link:key reference syntax
- [FIXED], [GENERATE], [TRANSLATE] tags
- REFERENCES_ENTITY and REFERENCES_PAGE auto-parsed arcs

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 4: Delete SearchIntent node (absorbed into Entity.type=ACTION)

**Files:**
- Delete: `packages/core/models/node-classes/project/semantic/search-intent.yaml`

**Step 1: Remove the file**

```bash
rm packages/core/models/node-classes/project/semantic/search-intent.yaml
```

**Step 2: Verify deletion**

Run: `ls packages/core/models/node-classes/project/semantic/`
Expected: search-intent.yaml NOT in list

**Step 3: Commit**

```bash
git add -u packages/core/models/node-classes/project/semantic/search-intent.yaml
git commit -m "feat(schema): remove SearchIntent node (absorbed into Entity.type=ACTION)

v10.3: SearchIntent is redundant - use Entity with type=ACTION instead

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 5: Delete TopicCluster node (absorbed into Entity.is_pillar)

**Files:**
- Delete: `packages/core/models/node-classes/project/semantic/topic-cluster.yaml`

**Step 1: Remove the file**

```bash
rm packages/core/models/node-classes/project/semantic/topic-cluster.yaml
```

**Step 2: Verify deletion**

Run: `ls packages/core/models/node-classes/project/semantic/`
Expected: topic-cluster.yaml NOT in list

**Step 3: Commit**

```bash
git add -u packages/core/models/node-classes/project/semantic/topic-cluster.yaml
git commit -m "feat(schema): remove TopicCluster node (absorbed into Entity.is_pillar + SUBTOPIC_OF)

v10.3: Use Entity.is_pillar=true + SUBTOPIC_OF arc instead

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 6: Delete old Concept and ConceptL10n nodes

**Files:**
- Delete: `packages/core/models/node-classes/project/semantic/concept.yaml`
- Delete: `packages/core/models/node-classes/project/semantic/concept-l10n.yaml`

**Step 1: Remove the files**

```bash
rm packages/core/models/node-classes/project/semantic/concept.yaml
rm packages/core/models/node-classes/project/semantic/concept-l10n.yaml
```

**Step 2: Verify deletion**

Run: `ls packages/core/models/node-classes/project/semantic/`
Expected: concept.yaml and concept-l10n.yaml NOT in list

**Step 3: Commit**

```bash
git add -u packages/core/models/node-classes/project/semantic/
git commit -m "feat(schema): remove Concept/ConceptL10n (replaced by Entity/EntityL10n)

v10.3: Entity is now in global/knowledge/ realm

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 2: Arc Schema Updates

### Task 7: Create EXPRESSES arc (SEOKeyword → Entity)

**Files:**
- Create: `packages/core/models/arc-classes/semantic/expresses.yaml`

**Step 1: Create the EXPRESSES arc YAML**

```yaml
# packages/core/models/arc-classes/semantic/expresses.yaml
# EXPRESSES - SEOKeyword expresses an Entity (v10.3)
#
# v10.3: Inverted SEO relationship
#        - Keywords EXPRESS entities (not the other way around)
#        - Matches user workflow: create Entity first, then attach keywords

arc:
  name: EXPRESSES
  family: semantic
  scope: cross_realm
  source: SEOKeyword
  target: Entity
  cardinality: many_to_one
  temperature_threshold: 0.5
  llm_context: |
    SEO keyword expresses a semantic entity.
    Multiple keywords can express the same entity.
    Use to find all keywords targeting an entity, or to find
    which entity a keyword represents.
  cypher_pattern: (SEOKeyword)-[:EXPRESSES]->(Entity)

  props:
    role:
      type: string
      required: false
      enum: [primary, secondary, long_tail]
      description: "Keyword role for this entity"

    match_score:
      type: float
      required: false
      description: "Semantic match score 0.0-1.0"

  example:
    description: "Find all keywords expressing an entity"
    cypher: |
      MATCH (kw:SEOKeyword)-[:EXPRESSES]->(e:Entity {key: $entityKey})
      RETURN kw.value, kw.volume, kw.difficulty
      ORDER BY kw.volume DESC
```

**Step 2: Commit**

```bash
git add packages/core/models/arc-classes/semantic/expresses.yaml
git commit -m "feat(schema): add EXPRESSES arc (SEOKeyword → Entity)

v10.3: Inverted SEO relationship - keywords express entities
Replaces HAS_SEO_KEYWORDS pattern

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 8: Create MATERIALIZES_AS arc (Entity → Page)

**Files:**
- Create: `packages/core/models/arc-classes/semantic/materializes-as.yaml`

**Step 1: Create the MATERIALIZES_AS arc YAML**

```yaml
# packages/core/models/arc-classes/semantic/materializes-as.yaml
# MATERIALIZES_AS - Entity materializes as Page(s) (v10.3)
#
# v10.3: N:M relationship (many entities can materialize as one page, one entity can have multiple pages)
#        - Optional: an Entity may not have a dedicated Page

arc:
  name: MATERIALIZES_AS
  family: semantic
  scope: cross_realm
  source: Entity
  target: Page
  cardinality: many_to_many
  temperature_threshold: 0.7
  llm_context: |
    Entity materializes as one or more pages.
    N:M relationship:
    - 1 Entity → 1 Page (common: dedicated page)
    - N Entities → 1 Page (comparison pages)
    - 1 Entity → N Pages (multiple page variants)
    - 1 Entity → 0 Pages (support entity, no dedicated page)
  cypher_pattern: (Entity)-[:MATERIALIZES_AS]->(Page)

  props:
    role:
      type: string
      required: false
      enum: [primary, secondary, comparison]
      description: "Role of this entity on the page"

  example:
    description: "Find all pages for an entity"
    cypher: |
      MATCH (e:Entity {key: $entityKey})-[:MATERIALIZES_AS]->(p:Page)
      RETURN p.slug, p.display_name
```

**Step 2: Commit**

```bash
git add packages/core/models/arc-classes/semantic/materializes-as.yaml
git commit -m "feat(schema): add MATERIALIZES_AS arc (Entity → Page)

v10.3: N:M relationship, optional (entity may not have page)

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 9: Create HAS_INSTRUCTION arc (Page → BlockInstruction)

**Files:**
- Create: `packages/core/models/arc-classes/ownership/has-instruction.yaml`

**Step 1: Create the HAS_INSTRUCTION arc YAML**

```yaml
# packages/core/models/arc-classes/ownership/has-instruction.yaml
# HAS_INSTRUCTION - Page owns BlockInstructions (v10.3)

arc:
  name: HAS_INSTRUCTION
  family: ownership
  scope: intra_realm
  source: Page
  target: BlockInstruction
  cardinality: one_to_many
  llm_context: |
    Page owns ordered BlockInstructions.
    Instructions are concatenated by order to form PageInstruction at runtime.
  cypher_pattern: (Page)-[:HAS_INSTRUCTION]->(BlockInstruction)

  example:
    description: "Get all instructions for a page in order"
    cypher: |
      MATCH (p:Page {slug: $slug})-[:HAS_INSTRUCTION]->(bi:BlockInstruction)
      RETURN bi.content, bi.order
      ORDER BY bi.order
```

**Step 2: Commit**

```bash
git add packages/core/models/arc-classes/ownership/has-instruction.yaml
git commit -m "feat(schema): add HAS_INSTRUCTION arc (Page → BlockInstruction)

v10.3: Page owns ordered block instructions

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 10: Create REFERENCES_ENTITY arc (BlockInstruction → Entity)

**Files:**
- Create: `packages/core/models/arc-classes/semantic/references-entity.yaml`

**Step 1: Create the REFERENCES_ENTITY arc YAML**

```yaml
# packages/core/models/arc-classes/semantic/references-entity.yaml
# REFERENCES_ENTITY - BlockInstruction references Entity (v10.3)
#
# Auto-parsed from @entity:key syntax in BlockInstruction.content

arc:
  name: REFERENCES_ENTITY
  family: semantic
  scope: cross_realm
  source: BlockInstruction
  target: Entity
  cardinality: many_to_many
  temperature_threshold: 0.6
  llm_context: |
    BlockInstruction references an Entity via @entity:key syntax.
    Auto-parsed when BlockInstruction is saved.
    Use to find all entities mentioned in instructions.
  cypher_pattern: (BlockInstruction)-[:REFERENCES_ENTITY]->(Entity)

  example:
    description: "Find all entities referenced in a page's instructions"
    cypher: |
      MATCH (p:Page {slug: $slug})-[:HAS_INSTRUCTION]->(bi:BlockInstruction)
            -[:REFERENCES_ENTITY]->(e:Entity)
      RETURN DISTINCT e.key, e.display_name
```

**Step 2: Commit**

```bash
git add packages/core/models/arc-classes/semantic/references-entity.yaml
git commit -m "feat(schema): add REFERENCES_ENTITY arc (BlockInstruction → Entity)

v10.3: Auto-parsed from @entity:key syntax

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 11: Create REFERENCES_PAGE arc (BlockInstruction → Page)

**Files:**
- Create: `packages/core/models/arc-classes/semantic/references-page.yaml`

**Step 1: Create the REFERENCES_PAGE arc YAML**

```yaml
# packages/core/models/arc-classes/semantic/references-page.yaml
# REFERENCES_PAGE - BlockInstruction references Page (v10.3)
#
# Auto-parsed from @link:key syntax in BlockInstruction.content
# Replaces manual LINKS_TO arc - internal linking defined in instructions

arc:
  name: REFERENCES_PAGE
  family: semantic
  scope: intra_realm
  source: BlockInstruction
  target: Page
  cardinality: many_to_many
  temperature_threshold: 0.6
  llm_context: |
    BlockInstruction references a Page via @link:key syntax.
    Auto-parsed when BlockInstruction is saved.
    Use to find internal linking strategy from instructions.
  cypher_pattern: (BlockInstruction)-[:REFERENCES_PAGE]->(Page)

  example:
    description: "Find all pages linked from a page's instructions"
    cypher: |
      MATCH (p:Page {slug: $slug})-[:HAS_INSTRUCTION]->(bi:BlockInstruction)
            -[:REFERENCES_PAGE]->(target:Page)
      RETURN DISTINCT target.slug, target.display_name
```

**Step 2: Commit**

```bash
git add packages/core/models/arc-classes/semantic/references-page.yaml
git commit -m "feat(schema): add REFERENCES_PAGE arc (BlockInstruction → Page)

v10.3: Auto-parsed from @link:key syntax
Replaces manual LINKS_TO arc

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 12: Create HAS_INTERNAL_LINK arc (BlockL10n → PageL10n)

**Files:**
- Create: `packages/core/models/arc-classes/localization/has-internal-link.yaml`

**Step 1: Create the HAS_INTERNAL_LINK arc YAML**

```yaml
# packages/core/models/arc-classes/localization/has-internal-link.yaml
# HAS_INTERNAL_LINK - BlockL10n contains link to PageL10n (v10.3)
#
# Tracks actual rendered internal links in generated content

arc:
  name: HAS_INTERNAL_LINK
  family: localization
  scope: intra_realm
  source: BlockL10n
  target: PageL10n
  cardinality: many_to_many
  llm_context: |
    BlockL10n contains an actual HTML internal link to PageL10n.
    Created when content is generated.
    Use to audit internal linking and find orphan pages.
  cypher_pattern: (BlockL10n)-[:HAS_INTERNAL_LINK]->(PageL10n)

  props:
    anchor_text:
      type: string
      required: false
      description: "The link anchor text"

    position:
      type: int
      required: false
      description: "Position in content (character offset)"

  example:
    description: "Find all internal links in a locale"
    cypher: |
      MATCH (bl:BlockL10n)-[:HAS_INTERNAL_LINK]->(pl:PageL10n)
            -[:FOR_LOCALE]->(l:Locale {key: $locale})
      RETURN bl, pl.slug, count(*) as link_count
```

**Step 2: Commit**

```bash
git add packages/core/models/arc-classes/localization/has-internal-link.yaml
git commit -m "feat(schema): add HAS_INTERNAL_LINK arc (BlockL10n → PageL10n)

v10.3: Tracks rendered internal links in generated content

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

### Task 13: Update HAS_SEO_KEYWORDS or remove (replaced by EXPRESSES)

**Files:**
- Modify or delete: `packages/core/models/arc-classes/ownership/has-seo-keywords.yaml`

**Step 1: Check if HAS_SEO_KEYWORDS is still needed**

The current `has-seo-keywords.yaml` is Locale → SEOKeyword (ownership).
With EXPRESSES (SEOKeyword → Entity), we still need Locale to own keywords.

Decision: Keep HAS_SEO_KEYWORDS as Locale → SEOKeyword (ownership).
EXPRESSES is separate (semantic link from keyword to entity).

**Step 2: Verify no changes needed**

Run: `cat packages/core/models/arc-classes/ownership/has-seo-keywords.yaml`
Expected: Locale → SEOKeyword (this is still valid)

**Step 3: No commit needed** (no changes)

---

### Task 14: Rename USES_CONCEPT → USES_ENTITY

**Files:**
- Find and update any arc referencing USES_CONCEPT

**Step 1: Search for USES_CONCEPT**

```bash
grep -r "USES_CONCEPT" packages/core/models/
```

**Step 2: If found, rename to USES_ENTITY**

Update the arc file and all references.

**Step 3: Commit if changes made**

```bash
git add -A packages/core/models/
git commit -m "feat(schema): rename USES_CONCEPT → USES_ENTITY

v10.3: Concept renamed to Entity

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 3: Update _index.yaml

### Task 15: Update _index.yaml with new nodes and arcs

**Files:**
- Modify: `packages/core/models/_index.yaml`

**Step 1: Read current _index.yaml**

**Step 2: Update with v10.3 changes**

- Add Entity, EntityL10n to global/knowledge
- Add BlockInstruction to project/instruction
- Remove SearchIntent, TopicCluster from project/semantic
- Remove Concept, ConceptL10n from project/semantic
- Add new arcs to arc list

**Step 3: Commit**

```bash
git add packages/core/models/_index.yaml
git commit -m "feat(schema): update _index.yaml for v10.3 Entity-centric model

- Add Entity, EntityL10n (global/knowledge)
- Add BlockInstruction (project/instruction)
- Remove SearchIntent, TopicCluster, Concept, ConceptL10n

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 4: Regenerate Artifacts

### Task 16: Run schema generate and fix any errors

**Step 1: Run schema generate**

```bash
cd tools/novanet && cargo run -- schema generate --dry-run
```

**Step 2: Fix any parser errors**

If errors occur, update Rust parsers as needed.

**Step 3: Run actual generation**

```bash
cargo run -- schema generate
```

**Step 4: Run tests**

```bash
cargo test
```

**Step 5: Commit generated files**

```bash
git add packages/db/seed/ packages/core/src/
git commit -m "chore(codegen): regenerate artifacts for v10.3

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 5: Update Design Doc Status

### Task 17: Mark design as implemented

**Files:**
- Modify: `docs/plans/2026-02-04-entity-centric-v10.3-design.md`

**Step 1: Update status**

Change `Status: Design validated, pending implementation plan` to `Status: Implemented`

**Step 2: Commit**

```bash
git add docs/plans/2026-02-04-entity-centric-v10.3-design.md
git commit -m "docs: mark v10.3 Entity-centric design as implemented

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| 1 | 1-6 | Node schema updates (Entity, EntityL10n, BlockInstruction, deletions) |
| 2 | 7-14 | Arc schema updates (EXPRESSES, MATERIALIZES_AS, etc.) |
| 3 | 15 | Update _index.yaml |
| 4 | 16 | Regenerate artifacts |
| 5 | 17 | Update documentation |

**Total: 17 tasks**

---

## Execution Handoff

**Plan complete and saved to `docs/plans/2026-02-04-entity-centric-v10.3-impl.md`.**

**Two execution options:**

1. **Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

2. **Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?**
