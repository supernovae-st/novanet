# Schema Completion v0.12.5 Plan

**Date**: 2026-02-14
**Status**: In Progress
**Predecessor**: semantic-coherence-v0121, yaml-panel-taxonomy-migration

## Overview

Complete all remaining schema coherence work to achieve 100% implementation of ADR-023 through ADR-030.

**New ADRs in this release:**
- **ADR-029**: *Native Pattern (EntityContent→EntityNative, PageGenerated→PageNative, etc.)
- **ADR-030**: Slug Ownership (Page owns URL, Entity owns semantics)

## Tasks

### Phase 1: Taxonomy Explosion (26 files)

Split `taxonomy.yaml` into individual YAML files for better maintainability.

**Directory structure**:
```
packages/core/models/taxonomy/
├── _index.yaml           # Registry + imports
├── realms/
│   ├── shared.yaml
│   └── org.yaml
├── layers/
│   ├── config.yaml
│   ├── locale.yaml
│   ├── geography.yaml
│   ├── knowledge.yaml
│   ├── foundation.yaml
│   ├── structure.yaml
│   ├── semantic.yaml
│   ├── instruction.yaml
│   └── output.yaml
├── traits/
│   ├── defined.yaml
│   ├── authored.yaml
│   ├── imported.yaml
│   ├── generated.yaml
│   └── retrieved.yaml
└── arc-families/
    ├── ownership.yaml
    ├── localization.yaml
    ├── semantic.yaml
    ├── generation.yaml
    └── mining.yaml
```

**Files**: 26 total (1 index + 2 realms + 9 layers + 5 traits + 5 arc-families + 4 directories)

### Phase 2: Icon Dual Format (57 conversions)

Convert all emoji icons to dual format `{ web: "lucide-name", terminal: "◆" }`.

**Files to update**:
- `packages/core/models/visual-encoding.yaml`
- Any node YAML files with emoji in `icon:` field

**Pattern**:
```yaml
# Before
icon: "🔷"

# After
icon:
  web: "square"
  terminal: "◆"
```

### Phase 3: llm_context Standardization (20+ nodes)

Apply USE/TRIGGERS/NOT/RELATES pattern to high-priority nodes.

**Target**: 34% → 80% coverage

**Pattern**:
```yaml
llm_context: |
  USE: when [primary use case].
  TRIGGERS: "keyword1", "keyword2", "keyword3".
  NOT: for [disambiguation] (use [alternative] instead).
  RELATES: [Source] (role), [Target] (role).
```

**Priority nodes**:
1. Page, Block, Entity (core structure)
2. PageGenerated, BlockGenerated (output)
3. EntityContent, ProjectContent (authored)
4. PageInstruction, BlockInstruction (instruction)
5. All nodes in org/foundation layer

### Phase 4: Block.key Format Decision

**Current format**: `blk-{page}-{slug}` (e.g., `blk-pricing-hero`)
**ADR-028 spec**: `{page_key}:{block_type}:{index}` (e.g., `pricing:hero:1`)

**Decision needed**:
- Option A: Update to ADR-028 spec (breaking change)
- Option B: Document current format as valid variant
- Option C: Support both formats during transition

### Phase 5: TUI YAML Panel Redesign

Contextual YAML display based on TreeItem selection.

**Changes**:
- Show relevant YAML section based on selected node type
- Tab-based navigation within YAML panel
- Path display showing file location

### Phase 6: *Native Pattern (ADR-029)

Rename all locale-specific nodes to use `*Native` suffix, unifying naming while using traits to distinguish authorship.

**Renames**:

| Old Name | New Name | Trait | Who Creates |
|----------|----------|-------|-------------|
| `EntityContent` | `EntityNative` | authored | Human writes natively |
| `ProjectContent` | `ProjectNative` | authored | Human writes natively |
| `PageGenerated` | `PageNative` | generated | LLM generates natively |
| `BlockGenerated` | `BlockNative` | generated | LLM generates natively |

**Arc Unification**:

| Old Arc | New Arc | Properties |
|---------|---------|------------|
| `HAS_CONTENT` | `HAS_NATIVE` | `{locale: "fr-FR"}` |
| `HAS_GENERATED` | `HAS_NATIVE` | `{locale: "fr-FR"}` |
| `CONTENT_OF` | `NATIVE_OF` | — |
| `GENERATED_FOR` | `NATIVE_OF` | — |

**Key Pattern**:
```
{type}:{invariant_key}@{locale}

EntityNative.key  = "entity:qr-code@fr-FR"
ProjectNative.key = "project:qrcode-ai@fr-FR"
PageNative.key    = "page:homepage@fr-FR"
BlockNative.key   = "block:homepage:hero:1@fr-FR"
```

**Files to update** (4 node renames + 4 arc renames):
```
packages/core/models/node-kinds/org/semantic/entity-content.yaml → entity-native.yaml
packages/core/models/node-kinds/org/foundation/project-content.yaml → project-native.yaml
packages/core/models/node-kinds/org/output/page-generated.yaml → page-native.yaml
packages/core/models/node-kinds/org/output/block-generated.yaml → block-native.yaml

packages/core/models/arc-kinds/ownership/has-content.yaml → has-native.yaml
packages/core/models/arc-kinds/ownership/has-generated.yaml → DELETE (merged)
packages/core/models/arc-kinds/ownership/content-of.yaml → native-of.yaml
```

### Phase 7: Slug Ownership (ADR-030)

Clear separation of concerns: Entity owns semantics, Page owns URL.

**Principle**:
```
Entity  = QUOI (semantic concept, invariant)
Page    = OÙ   (URL structure, navigation)

Entity.key     = Semantic identifier (english, invariant)
Page.slug      = URL segment (english, invariant)
PageNative.slug = Localized URL segment (per locale)
```

**Property Migration**:

| Property | Remove From | Add To |
|----------|-------------|--------|
| `slug` | EntityNative | PageNative |
| `full_path` | EntityNative | PageNative |
| `parent_slug` | EntityNative | (not needed) |
| `depth` | EntityNative | (not needed) |
| `slug_history` | EntityNative | (not needed) |

**Key Design Decision**:
```
Entity.key:  "qr-code-instagram"  (full semantic identity)
Page.slug:   "instagram"          (just the URL segment)
```

This avoids `/qr-code-generator/qr-code-instagram` → `/qr-code-generator/instagram` ✅

**Concrete Example - 4 Entities**:

```
Entity: instagram (BRAND, no Page)
  └── Referenced by SEMANTIC_LINK from other entities

Entity: qr-code-generator (PILLAR)
  └── Page.slug: "qr-code-generator"
  └── PageNative(fr).slug: "générateur-qr-code"
  └── PageNative(fr).full_path: "/fr/générateur-qr-code"

Entity: qr-code-instagram (SUBTOPIC of qr-code-generator)
  └── Page.slug: "instagram" (NOT "qr-code-instagram")
  └── PageNative(fr).slug: "instagram" (brand unchanged)
  └── PageNative(fr).full_path: "/fr/générateur-qr-code/instagram"

Entity: template-instagram (SUBTOPIC of templates)
  └── Page.slug: "instagram" (same segment, different parent!)
  └── PageNative(fr).full_path: "/fr/modeles/instagram"
```

**Files to update**:
```
# Remove slug properties
packages/core/models/node-kinds/org/semantic/entity-native.yaml

# Add slug properties
packages/core/models/node-kinds/org/output/page-native.yaml

# Fix misleading comment
packages/core/models/node-kinds/org/semantic/entity.yaml (line ~312)
```

## Execution Order

1. ✅ Phase 2 (Icon dual format) - Independent, can run in parallel
2. ✅ Phase 3 (llm_context) - Independent, can run in parallel
3. ✅ Phase 1 (Taxonomy explosion) - Independent, can run in parallel
4. ⏸ Phase 4 (Block.key) - Needs decision
5. ⏸ Phase 5 (TUI YAML Panel) - Depends on Phase 1
6. 🆕 Phase 6 (*Native Pattern) - ADR-029
7. 🆕 Phase 7 (Slug Ownership) - ADR-030, depends on Phase 6

**Recommended order**: Phase 6 → Phase 7 (rename first, then migrate properties)

## Success Criteria

- [ ] All 26 taxonomy files created and validated
- [ ] 0 emoji icons remaining (all dual format)
- [ ] 80%+ nodes have standardized llm_context
- [ ] Block.key format documented or migrated
- [ ] TUI YAML panel shows contextual content
- [ ] *Native pattern implemented (4 nodes, 4 arcs renamed)
- [ ] Slug properties moved from EntityNative to PageNative
- [ ] Entity.yaml HAS_CHILD comment corrected
- [ ] All 1053+ tests pass
- [ ] Schema validates with 0 errors

---

## Appendix A: SEMANTIC_LINK Architecture

Spreading activation pattern for related entity traversal.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SEMANTIC_LINK: Spreading Activation Pattern                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Entity: qr-code-instagram                                                  │
│  │                                                                          │
│  ├──[:SEMANTIC_LINK {link_type: "used_for", temperature: 0.9}]──▶ instagram │
│  │   └── "QR codes FOR Instagram" (high relevance)                         │
│  │                                                                          │
│  ├──[:SEMANTIC_LINK {link_type: "component_of", temperature: 0.7}]──▶ qr-code │
│  │   └── "Instagram QR is a TYPE OF QR code" (medium relevance)            │
│  │                                                                          │
│  └──[:SEMANTIC_LINK {link_type: "compared_to", temperature: 0.3}]──▶ linktree │
│      └── "Sometimes compared to Linktree" (low relevance)                  │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  TEMPERATURE CALIBRATION                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Temperature = Propagation strength for spreading activation                │
│                                                                             │
│  0.9-1.0  HOT:   Always load (core dependencies)                           │
│  0.6-0.8  WARM:  Load for detailed context                                 │
│  0.3-0.5  COOL:  Load only if explicitly relevant                          │
│  0.0-0.2  COLD:  Reference only, rarely propagate                          │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  LINK_TYPE ENUM (11 types)                                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  STRUCTURAL:                                                                │
│  - component_of    "X is part of Y"                                        │
│  - variant_of      "X is a variant of Y"                                   │
│  - instance_of     "X is an instance of Y"                                 │
│                                                                             │
│  USAGE:                                                                     │
│  - used_for        "X is used for Y"                                       │
│  - used_with       "X is used with Y"                                      │
│  - enables         "X enables Y"                                           │
│  - requires        "X requires Y"                                          │
│                                                                             │
│  COMPARISON:                                                                │
│  - compared_to     "X is compared to Y"                                    │
│  - alternative_to  "X is an alternative to Y"                              │
│  - competes_with   "X competes with Y"                                     │
│                                                                             │
│  ASSOCIATION:                                                               │
│  - associated_with "X is associated with Y" (catch-all)                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Query Example**:
```cypher
// Load related entities with spreading activation
MATCH (e:Entity {key: $entity_key})
OPTIONAL MATCH (e)-[sl:SEMANTIC_LINK]->(related:Entity)
WHERE sl.temperature >= $min_temperature
OPTIONAL MATCH (related)-[:HAS_NATIVE {locale: $locale}]->(rn)
RETURN e, collect({entity: related, temp: sl.temperature, link: sl.link_type, native: rn}) AS related
ORDER BY sl.temperature DESC
```

---

## Appendix B: Dual SEO Pattern

Two-level targeting: Entity owns keywords (strategy), EntityNative targets keywords (execution).

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  DUAL SEO PATTERN: HAS_KEYWORD vs TARGETS                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  LEVEL 1: STRATEGIC (Entity → SEOKeyword)                                   │
│  ─────────────────────────────────────────                                  │
│                                                                             │
│  Entity: qr-code-generator                                                  │
│  │                                                                          │
│  ├──[:HAS_KEYWORD {rank: "primary"}]──▶ SEOKeyword: "qr code generator"    │
│  │   └── Volume: 165,000 │ KD: 75 │ Intent: transactional                  │
│  │                                                                          │
│  └──[:HAS_KEYWORD {rank: "secondary"}]──▶ SEOKeyword: "create qr code"     │
│      └── Volume: 110,000 │ KD: 65 │ Intent: transactional                  │
│                                                                             │
│  WHY? Strategy is locale-independent. An Entity "owns" its keywords        │
│  regardless of language. This is the WHAT we want to rank for.             │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  LEVEL 2: TACTICAL (EntityNative → SEOKeyword)                              │
│  ─────────────────────────────────────────────                              │
│                                                                             │
│  EntityNative: entity:qr-code-generator@fr-FR                               │
│  │                                                                          │
│  ├──[:TARGETS]──▶ SEOKeyword: "générateur qr code"                         │
│  │   └── Volume: 12,100 │ KD: 45 │ Serp: featured_snippet                  │
│  │                                                                          │
│  └──[:TARGETS]──▶ SEOKeyword: "créer qr code gratuit"                      │
│      └── Volume: 8,100 │ KD: 35 │ Serp: people_also_ask                    │
│                                                                             │
│  WHY? French content targets FRENCH keywords. This is the HOW we           │
│  implement the strategy in each locale.                                     │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  RELATIONSHIP                                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Entity.HAS_KEYWORD    = "We want to rank for this topic"                  │
│  EntityNative.TARGETS  = "This content targets this keyword"               │
│                                                                             │
│  Usually: Entity.HAS_KEYWORD[primary] ≈ EntityNative.TARGETS[main]         │
│  But not always! Content might target long-tail not in HAS_KEYWORD.        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Query Example**:
```cypher
// Get SEO strategy + tactical targeting for a locale
MATCH (e:Entity {key: $entity_key})
OPTIONAL MATCH (e)-[hk:HAS_KEYWORD]->(strategic:SEOKeyword)
OPTIONAL MATCH (e)-[:HAS_NATIVE {locale: $locale}]->(en:EntityNative)-[t:TARGETS]->(tactical:SEOKeyword)
RETURN e.key,
       collect(DISTINCT {kw: strategic.value, rank: hk.rank}) AS strategy,
       collect(DISTINCT {kw: tactical.value, vol: tactical.volume}) AS targeting
```

---

## Appendix C: GEO Monitoring Pattern

Track AI visibility across LLM platforms (Claude, GPT, Perplexity, Gemini).

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  GEO MONITORING: AI Visibility Tracking                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  EntityNative: entity:qr-code-generator@en-US                               │
│  │                                                                          │
│  └──[:MONITORS_GEO]──▶ GEOQuery                                            │
│                        │ value: "how to create a QR code"                  │
│                        │ query_type: "how_to"                              │
│                        │ platforms: [gemini, gpt, perplexity, claude]      │
│                        │                                                    │
│                        └──[:HAS_GEO_ANSWERS]──▶ GEOAnswer (time-series)    │
│                                                │                            │
│                                                ├── captured_at: 2026-02-14 │
│                                                ├── engine: "perplexity"    │
│                                                ├── answer_text: "..."      │
│                                                ├── cited_domains: [...]    │
│                                                ├── brand_mentions: 2       │
│                                                ├── competitor_mentions: 1  │
│                                                ├── ai_visibility_score: 78 │
│                                                └── share_of_voice: 0.23    │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  KEY DISTINCTION                                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  GEOQuery:  IMPORTED (static query definition)                              │
│             - We define which queries to monitor                            │
│             - Does NOT change over time                                     │
│                                                                             │
│  GEOAnswer: RETRIEVED (time-series snapshots)                               │
│             - What the LLM actually returned                                │
│             - Changes every time we poll                                    │
│             - Immutable once captured                                       │
│                                                                             │
│  NOTE: GEOAnswer is RETRIEVED, not GENERATED.                               │
│  We retrieved it from Claude/GPT/Perplexity API, we didn't generate it.    │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  GEO vs SEO                                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SEO = Search Engine Optimization                                           │
│        EntityNative ──[:TARGETS]──▶ SEOKeyword                             │
│        "How do we rank in Google?"                                          │
│                                                                             │
│  GEO = Generative Engine Optimization                                       │
│        EntityNative ──[:MONITORS_GEO]──▶ GEOQuery                          │
│        "How visible are we in AI answers?"                                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Query Example**:
```cypher
// Get GEO visibility trends for an entity
MATCH (e:Entity {key: $entity_key})-[:HAS_NATIVE {locale: $locale}]->(en:EntityNative)
MATCH (en)-[:MONITORS_GEO]->(q:GEOQuery)-[:HAS_GEO_ANSWERS]->(a:GEOAnswer)
WHERE a.captured_at >= date() - duration('P30D')
RETURN q.value AS query,
       a.engine,
       a.ai_visibility_score,
       a.share_of_voice,
       a.captured_at
ORDER BY a.captured_at DESC
```

---

## Appendix D: Page/Block Ecosystem

Complete structure from Page through Block to generated output.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  PAGE/BLOCK ECOSYSTEM                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Page (defined)                                                              │
│  │ key: "pricing"                                                           │
│  │ slug: "pricing"                                                          │
│  │                                                                          │
│  ├──[:REPRESENTS]──▶ Entity: pricing                                       │
│  │   └── 1:1 mandatory (ADR-028)                                           │
│  │                                                                          │
│  ├──[:SUBTOPIC_OF]──▶ Page (URL hierarchy)                                 │
│  │   └── For nested URLs: /products/pricing                                │
│  │                                                                          │
│  ├──[:HAS_BLOCK {order: 1}]──▶ Block: pricing:hero:1                       │
│  │   │                                                                      │
│  │   ├──[:OF_TYPE]──▶ BlockType: hero                                      │
│  │   │   └── JSON schema defining allowed fields                           │
│  │   │                                                                      │
│  │   ├──[:HAS_INSTRUCTION]──▶ BlockInstruction                             │
│  │   │   └── Markdown with @ refs: "Compare @entity:tier-pro..."           │
│  │   │                                                                      │
│  │   ├──[:REFERENCES]──▶ Entity (aggregated from @ refs)                   │
│  │   │   └── purpose: inject | link                                        │
│  │   │                                                                      │
│  │   └──[:HAS_NATIVE {locale}]──▶ BlockNative                              │
│  │       │ key: "block:pricing:hero:1@fr-FR"                               │
│  │       │ anchor_slug: "section-hero"                                     │
│  │       └── Generated content for this block                              │
│  │                                                                          │
│  ├──[:HAS_BLOCK {order: 2}]──▶ Block: pricing:comparison:1                 │
│  │                                                                          │
│  └──[:HAS_NATIVE {locale}]──▶ PageNative                                   │
│      │ key: "page:pricing@fr-FR"                                           │
│      │ slug: "tarifs"                                                       │
│      │ full_path: "/fr/tarifs"                                              │
│      │ meta_title: "Tarifs - QR Code AI"                                    │
│      └── Assembled from BlockNative nodes                                  │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  BLOCK.KEY FORMAT (ADR-028)                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Block.key = "{page_key}:{block_type}:{index}"                              │
│                                                                             │
│  Examples:                                                                  │
│  - pricing:hero:1          (first hero on pricing)                         │
│  - pricing:comparison:1    (first comparison on pricing)                   │
│  - homepage:testimonials:1 (first testimonials on homepage)                │
│  - homepage:testimonials:2 (second testimonials on homepage)               │
│                                                                             │
│  Benefits:                                                                  │
│  - Globally unique (no collision between pages)                            │
│  - Parseable (extract page, type, index)                                   │
│  - Allows multiple blocks of same type per page                            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Related ADRs

| ADR | Title | Summary |
|-----|-------|---------|
| ADR-023 | Class/Instance Terminology | Kind→Class, Meta→Schema |
| ADR-024 | Trait = Data Origin | defined/authored/imported/generated/retrieved |
| ADR-025 | Instruction Rename | PagePrompt→PageInstruction, etc. |
| ADR-026 | Inverse Arc Policy | TIER 1/2/3 classification |
| ADR-027 | Generation Family Semantics | Pipeline documentation |
| ADR-028 | Page-Entity Architecture | 1:1 mandatory, @ refs, Block.key format |
| ADR-029 | *Native Pattern | EntityContent→EntityNative (NEW) |
| ADR-030 | Slug Ownership | Page owns URL, Entity owns semantics (NEW) |
