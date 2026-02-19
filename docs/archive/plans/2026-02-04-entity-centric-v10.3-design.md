# Entity-Centric Architecture v10.3 Design

> **For Claude:** This is a design document from brainstorming sessions. Use superpowers:writing-plans to create implementation plan when ready.

**Goal:** Refactor NovaNet's semantic layer from Concept-centric to Entity-centric, aligned with schema.org/Wikidata patterns, with proper SEO structuring and instruction compilation.

**Version:** v10.3 (builds on v10.2 SHARED→GLOBAL merge)

**Status:** Implemented (2026-02-04)

---

## 1. Problem Statement

### Current Issues (v10.2)

1. **SEO Path Too Indirect**: Page → Concept → ConceptL10n → SEOKeyword = 3 hops
2. **Concept is Too Generic**: Doesn't capture entity types (action, thing, feature, brand, etc.)
3. **SearchIntent Redundant**: An ACTION-type entity IS a search intent
4. **TopicCluster Redundant**: Can be modeled with Entity.is_pillar + SUBTOPIC_OF
5. **No Instruction Model**: How pages are built from block instructions isn't captured
6. **Internal Links Unclear**: Mixed conceptual, strategic, and rendered link levels

### User's Actual Workflow

```
1. Create Entities from keywords (Entity comes FIRST)
2. Attach SEOKeywords to Entities (keywords EXPRESS entities)
3. Build Pages that MATERIALIZE entities
4. Define BlockInstructions with @entity: and @link: references
5. Compile BlockInstructions → PageInstruction (at runtime)
6. Generate PageL10n from compiled instructions
```

---

## 2. Research: Schema.org & Knowledge Graph Patterns

### Schema.org Hierarchy

```
Thing (root)
├── Action         → User intents, conversions
├── CreativeWork   → Content, media
├── Event          → Time-bound occurrences
├── Intangible     → Abstract concepts, services
├── Organization   → Companies, brands
├── Person         → People, personas
├── Place          → Locations, venues
└── Product        → Physical/digital products
```

### Google Knowledge Graph

- 500 billion facts, 5 billion entities
- Entity = canonical representation linked to keywords
- Entities have types and relationships

### Wikidata Pattern

- Q-items (entities) with P31 (instance of) and P279 (subclass of)
- Hierarchical taxonomy via class membership

---

## 3. Proposed Architecture

### 3.1 Entity Model (replaces Concept)

```yaml
# packages/core/models/nodes/global/knowledge/entity.yaml
node:
  name: Entity
  realm: global
  layer: knowledge
  trait: invariant
  description: |
    Universal semantic entity aligned with schema.org Thing.
    Represents any concept, action, feature, brand, place, or audience
    that can be expressed by keywords and materialized as pages.
  properties:
    - name: key
      type: string
      required: true
      description: Unique identifier (kebab-case)
    - name: type
      type: EntityType
      required: true
      description: Semantic type (ACTION, THING, FEATURE, BRAND, PLACE, AUDIENCE, GUIDE, COMPARISON)
    - name: is_pillar
      type: boolean
      default: false
      description: Whether this is a pillar entity (has cluster children)
    - name: schema_org_type
      type: string
      required: false
      description: Optional schema.org type (e.g., "Product", "HowTo")
    - name: wikidata_id
      type: string
      required: false
      description: Optional Wikidata Q-identifier (e.g., "Q12345")
```

### 3.2 EntityType Enum

```typescript
export type EntityType =
  | 'ACTION'      // User intents, conversions (ex-SearchIntent)
  | 'THING'       // Objects, products
  | 'FEATURE'     // Product features, capabilities
  | 'BRAND'       // Companies, brands
  | 'PLACE'       // Locations, venues
  | 'AUDIENCE'    // Target personas, segments
  | 'GUIDE'       // How-to, tutorials
  | 'COMPARISON'; // Comparisons, alternatives
```

### 3.3 EntityL10n Node

```yaml
# packages/core/models/nodes/global/knowledge/entity-l10n.yaml
node:
  name: EntityL10n
  realm: global
  layer: knowledge
  trait: localized
  description: |
    Localized content for an Entity.
    The display_name changes per locale (e.g., "QR Code Generator" vs "Générateur de QR Code").
  properties:
    - name: display_name
      type: string
      required: true
      description: Localized name of the entity
    - name: description
      type: string
      required: true
      description: Localized description of the entity
```

### 3.4 SEO Arc Inversion

**Before (v10.2):**
```
ConceptL10n ──HAS_SEO_KEYWORDS──> SEOKeyword
```

**After (v10.3):**
```
SEOKeyword ──EXPRESSES──> Entity
```

**Rationale:** Keywords EXPRESS entities, not the other way around. This matches the user's workflow where entities are created first from keyword research.

### 3.5 MATERIALIZES_AS Cardinality

**Cardinality:** N:M (many-to-many)
**Optional:** Yes (an Entity may not have a dedicated Page)

| Case | Example |
|------|---------|
| 1 Entity → 1 Page | `qr-generator` → `/qr-generator` |
| N Entities → 1 Page | `dynamic-qr` + `static-qr` → `/dynamic-vs-static` (comparison) |
| 1 Entity → 0 Pages | `analytics-feature` (support entity, no dedicated page) |
| 1 Entity → N Pages | `qr-generator` → `/qr-generator` + `/free-qr-generator` |

### 3.6 Pillar/Cluster via SUBTOPIC_OF

**Remove:** TopicCluster node (redundant)

**Use instead:**
```
Entity (is_pillar=true)
    ↑
    │ SUBTOPIC_OF
    │
Entity (is_pillar=false)  ← cluster child
```

### 3.7 Two-Level Internal Linking (Simplified)

| Level | Arc | From → To | Purpose |
|-------|-----|-----------|---------|
| 1. Conceptual | SEMANTIC_LINK | Entity → Entity | Knowledge graph connections |
| 2. Instruction | REFERENCES_PAGE | BlockInstruction → Page | Links via @link: syntax (auto-parsed) |
| 3. Rendered | HAS_INTERNAL_LINK | BlockL10n → PageL10n | Actual HTML links in output |

**Note:** No manual LINKS_TO arc. Page linking strategy is defined in BlockInstructions via @link: syntax.

---

## 4. Instruction Model

### 4.1 Simplified Model (No Block Intermediate)

```
BEFORE (too complex):
Page ──HAS_BLOCK──> Block ──OF_TYPE──> BlockType
                     │
                     └── HAS_INSTRUCTION ──> BlockInstruction

AFTER (simplified):
Page ──HAS_INSTRUCTION──> BlockInstruction ──OF_TYPE──> BlockType
```

### 4.2 BlockInstruction Node

```yaml
# packages/core/models/nodes/project/instruction/block-instruction.yaml
node:
  name: BlockInstruction
  realm: project
  layer: instruction
  trait: invariant
  description: |
    Instructions for generating a specific block within a page.
    Contains markdown with @entity: and @link: references.
    Multiple BlockInstructions are concatenated to form PageInstruction.
  properties:
    - name: key
      type: string
      required: true
      description: Unique identifier (e.g., "page-qr-generator_hero")
    - name: content
      type: string
      required: true
      description: Markdown content with @entity:key, @link:key, [FIXED], [GENERATE], [TRANSLATE] tags
    - name: order
      type: number
      required: true
      description: Order in page compilation (1, 2, 3...)
```

### 4.3 Instruction Syntax

```markdown
## Hero Section [GENERATE]

Create a compelling hero for @entity:qr-code-generator.
Link to @link:pricing-page for conversion.

## Features [TRANSLATE]

List the 5 main features of @entity:qr-code-generator:
- @entity:dynamic-qr
- @entity:analytics
- @entity:customization
- @entity:bulk-generation
- @entity:api-access
```

**Tags:**
- `[FIXED]` = Invariant content, identical across locales
- `[GENERATE]` = LLM generates with creative freedom
- `[TRANSLATE]` = Close to original, less creative freedom

### 4.3 Reference Arcs (Auto-Parsed)

```
BlockInstruction ──REFERENCES_ENTITY──> Entity
BlockInstruction ──REFERENCES_PAGE──> Page
```

These arcs are created automatically by parsing @entity: and @link: references.

### 4.4 PageInstruction = Computed Artifact

PageInstruction is **NOT** a Neo4j node. It's computed at runtime:

```
PageInstruction = CONCAT(
  BlockInstruction[order=1].content,
  BlockInstruction[order=2].content,
  BlockInstruction[order=3].content,
  ...
)
```

---

## 5. Complete Arc Inventory (v10.3 Changes)

### New Arcs

| Arc | Family | From → To | Cardinality | Description |
|-----|--------|-----------|-------------|-------------|
| EXPRESSES | semantic | SEOKeyword → Entity | N:1 | Keyword expresses entity |
| MATERIALIZES_AS | semantic | Entity → Page | N:M | Entity materializes as page(s) |
| HAS_INSTRUCTION | ownership | Page → BlockInstruction | 1:N | Page has ordered instructions |
| OF_TYPE | ownership | BlockInstruction → BlockType | N:1 | Instruction uses block type |
| REFERENCES_ENTITY | semantic | BlockInstruction → Entity | N:M | Instruction references entity (@entity:) |
| REFERENCES_PAGE | semantic | BlockInstruction → Page | N:M | Instruction references page (@link:) |
| HAS_INTERNAL_LINK | localization | BlockL10n → PageL10n | N:M | Rendered internal link |

### Modified Arcs

| Arc | Change |
|-----|--------|
| HAS_SEO_KEYWORDS | REMOVED (replaced by EXPRESSES) |

### Renamed Arcs

| Old | New |
|-----|-----|
| USES_CONCEPT | USES_ENTITY |
| HAS_RELATED_CONCEPTS | SEMANTIC_LINK |

---

## 6. Node Inventory Changes

### Renamed

| Old (v10.2) | New (v10.3) |
|-------------|-------------|
| Concept | Entity |
| ConceptL10n | EntityL10n |

### Removed

| Node | Reason |
|------|--------|
| SearchIntent | Absorbed into Entity.type=ACTION |
| TopicCluster | Absorbed into Entity.is_pillar + SUBTOPIC_OF |

### New

| Node | Layer | Description |
|------|-------|-------------|
| BlockInstruction | instruction | Instructions for block generation |

---

## 7. Layer Structure (v10.3)

```
GLOBAL REALM
├── config (3)      │ Project, Locale, LocaleVoice
├── knowledge (2)   │ Entity, EntityL10n  ← renamed from Concept
├── foundation (3)  │ BlockType, Currency, Timezone
└── seo (2)         │ SEOKeyword, SearchQuery

PROJECT REALM
├── structure (3)   │ Page, Block, Section
├── instruction (1) │ BlockInstruction  ← NEW
├── output (3)      │ PageL10n, BlockL10n, SectionL10n
└── job (3)         │ GenerationJob, GenerationTask, GenerationResult
```

---

## 8. Visual Summary

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  v10.3 ENTITY-CENTRIC ARCHITECTURE                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  LAYER: SEO + KNOWLEDGE                                                     │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │  SEOKeyword ──EXPRESSES──> Entity ──SUBTOPIC_OF──> Entity (pillar)    │  │
│  │                               │                                       │  │
│  │                               │ MATERIALIZES_AS (N:M, optional)       │  │
│  │                               ↓                                       │  │
│  └───────────────────────────────┼───────────────────────────────────────┘  │
│                                  │                                          │
│  LAYER: STRUCTURE + INSTRUCTION  │                                          │
│  ┌───────────────────────────────┼───────────────────────────────────────┐  │
│  │                               ↓                                       │  │
│  │                             Page                                      │  │
│  │                               │                                       │  │
│  │                               │ HAS_INSTRUCTION (1:N ordered)         │  │
│  │                               ↓                                       │  │
│  │                        BlockInstruction ──OF_TYPE──> BlockType        │  │
│  │                        ├── REFERENCES_ENTITY (@entity:) ──> Entity    │  │
│  │                        └── REFERENCES_PAGE (@link:) ──────> Page      │  │
│  └───────────────────────────────┼───────────────────────────────────────┘  │
│                                  │                                          │
│                                  │ CONCAT (runtime)                         │
│                                  ↓                                          │
│                           [PageInstruction]  ← computed, not stored         │
│                                  │                                          │
│                                  │ LLM                                      │
│                                  ↓                                          │
│  LAYER: OUTPUT                                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                             PageL10n                                  │  │
│  │                               │                                       │  │
│  │                               │ HAS_BLOCK                             │  │
│  │                               ↓                                       │  │
│  │                            BlockL10n ──HAS_INTERNAL_LINK──> PageL10n  │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 9. Migration Path

### Phase 1: Schema Updates
1. Rename Concept → Entity in YAML
2. Add EntityType enum
3. Add Entity.is_pillar, schema_org_type, wikidata_id
4. Create BlockInstruction node YAML
5. Create new arcs (EXPRESSES, MATERIALIZES_AS, etc.)

### Phase 2: Arc Changes
1. Remove HAS_SEO_KEYWORDS arc
2. Add EXPRESSES arc (SEOKeyword → Entity)
3. Rename USES_CONCEPT → USES_ENTITY
4. Rename HAS_RELATED_CONCEPTS → SEMANTIC_LINK

### Phase 3: Node Cleanup
1. Remove SearchIntent node
2. Remove TopicCluster node
3. Migrate any existing data

### Phase 4: Generator Updates
1. Update Rust generators for new arcs
2. Update TypeScript types
3. Update Cypher seeds
4. Update Mermaid diagrams

---

## 10. Open Questions (All Resolved)

1. ~~**EntityL10n scope**: Does EntityL10n need description only, or also display_name variations?~~
   → **RESOLVED**: EntityL10n has both `display_name` AND `description`. The name changes per locale.

2. ~~**BlockInstruction per instance**: Is BlockInstruction per BlockType or per Page+BlockType combo?~~
   → **RESOLVED**: BlockInstruction is per Page (Page ──HAS_INSTRUCTION──> BlockInstruction ──OF_TYPE──> BlockType)

3. ~~**LINKS_TO cardinality**: Should Page ──LINKS_TO──> Page be explicit or derived from BlockInstruction's @link: references?~~
   → **RESOLVED**: No manual LINKS_TO arc. Use REFERENCES_PAGE only (auto-parsed from @link: in BlockInstruction).

4. ~~**Block node**: Do we keep the Block node for structure (Page → Block → BlockType) or is BlockInstruction enough?~~
   → **RESOLVED**: Simplified model without Block intermediate. BlockInstruction carries the block_type reference.

---

## 11. References

- schema.org: https://schema.org/docs/full.html
- Google Knowledge Graph: https://developers.google.com/knowledge-graph
- Wikidata: https://www.wikidata.org/wiki/Wikidata:Introduction
- Previous design: `docs/plans/2026-02-01-ontology-v9-design.md`
- v10 decisions: `docs/plans/2026-02-03-v10-brainstorm-decisions.md`

---

## Changelog

| Date | Change |
|------|--------|
| 2026-02-04 | Initial design from brainstorming sessions |
| 2026-02-04 | Implemented: Entity/EntityL10n/BlockInstruction nodes, EXPRESSES/MATERIALIZES_AS/HAS_INSTRUCTION/REFERENCES_ENTITY/REFERENCES_PAGE/HAS_INTERNAL_LINK arcs, USES_CONCEPT→USES_ENTITY rename |
