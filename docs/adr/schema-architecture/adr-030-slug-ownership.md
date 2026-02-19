---
id: ADR-030
title: "Slug Ownership"
version: v0.13.1
status: active
domain: schema-architecture
---

# ADR-030: Slug Ownership

**Status**: Approved (v0.12.5, **Updated v0.13.1**)

**Problem**: Current architecture mixes concerns:
1. Entity has semantic identity (key)
2. EntityNative has slug, full_path, parent_slug, depth
3. Page has slug
4. Entity.HAS_CHILD comment says "URL path = parent.slug" but Entity has NO slug
5. Page.REPRESENTS Entity (1:1 mandatory per ADR-028)

Which is source of truth for URLs?

**Decision**: Clear separation of concerns — Entity owns semantics, Page owns URLs.

## v0.13.1 Update: BlockNative:head-seo-meta Owns Slug

**Previous** (v0.12.5): PageNative had slug, full_path properties directly.

**New** (v0.13.1): Slug lives in the **head-seo-meta** BlockNative (first block of every page).

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SLUG ARCHITECTURE (v0.13.1)                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  PageNative ──[:ASSEMBLES {order: 0}]──> BlockNative:head-seo-meta              │
│                                              │                                  │
│                                              ├── slug: "código-qr"              │
│                                              ├── meta_title: "..."              │
│                                              └── meta_description: "..."        │
│                                                                                 │
│  TWO COMPLEMENTARY ARCS:                                                        │
│  ├── [:DERIVED_SLUG_FROM] = PROVENANCE (BlockNative → EntityNative)             │
│  └── [:SLUGIFIED_BY]      = VALIDATION (BlockNative → Slugification)            │
│                                                                                 │
│  SLUG SOURCE TRACKING (on TARGETS arc):                                         │
│  EntityNative ──[:TARGETS {rank, is_slug_source: true}]──> SEOKeyword           │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  CRITICAL RULES                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. SEOKeyword.slug_form = INPUT REFERENCE, not final output.                   │
│     The LLM derives the slug using one of these modes:                          │
│       copy      → direct use of slug_form (e.g. "qr-code-erstellen")           │
│       extract   → subset of slug_form terms (no-repetition rule, ADR-032)      │
│       merge     → combine terms from multiple keywords                          │
│       modify    → adapt with brand/tech terms (e.g. add "ai", "gratuit")       │
│       derive    → strategic slug not directly in keywords                       │
│     Diacritics and locale rules (ADR-032) apply to the final derived value.    │
│                                                                                 │
│  2. DERIVED_SLUG_FROM points to EntityNative (not SEOKeyword directly).         │
│     All keyword knowledge stays on EntityNative via TARGETS arcs.               │
│     The slug source keyword is identified by TARGETS {is_slug_source: true}.    │
│                                                                                 │
│  3. is_slug_source ≠ rank='primary'. They can differ (es-MX: primary=código-qr  │
│     but slug source = secondary crear-código-qr, no competitor positioned).     │
│                                                                                 │
│  4. After slug derivation, the url form is written back to                      │
│     EntityNative.denomination_forms — enabling cross-page URL consistency.      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Principle

```
Entity  = QUOI (semantic concept, invariant)
Page    = OU   (URL structure, navigation)
Block   = CONTENU (content blocks)

Entity.key     = Semantic identifier (english, invariant)
Page.slug      = URL segment (english, invariant)
BlockNative:head-seo-meta.slug = Localized URL segment (per locale)
```

## Who Has What (v0.13.1)

| Node | slug? | full_path? | Why |
|------|-------|------------|-----|
| Entity | No | No | Semantic concept, not URL-related |
| EntityNative | No | No | Content for concept, URL lives on Block |
| Page | Yes EN | No | URL segment (invariant, english) |
| PageNative | No | No | **v0.13.1**: Assembled output, slug moved to head-seo-meta |
| BlockNative:head-seo-meta | Yes L10n | Yes | **v0.13.1**: Owns slug + meta_title + meta_description |

## head-seo-meta BlockType

First block of every page (order=0), contains SEO metadata:

```yaml
BlockType:
  key: head-seo-meta
  block_type: head-seo-meta
  schema:
    slug:
      type: string
      required: true
      description: "Localized URL segment (COPIED from SEOKeyword.slug_form)"
    full_path:
      type: string
      required: true
      description: "Full localized path (computed from parent + slug)"
    meta_title:
      type: string
      required: true
      max_length: 60
    meta_description:
      type: string
      required: true
      max_length: 160
```

## Slug Derivation Flow

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  SLUG DERIVATION (Option B — keyword knowledge stays on EntityNative)           │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. EntityNative has TARGETS arcs with keyword knowledge:                       │
│     (EntityNative@es-MX)-[:TARGETS {rank:'primary', is_slug_source:false}]->    │
│       (SEOKeyword "código-qr")                                                  │
│     (EntityNative@es-MX)-[:TARGETS {rank:'secondary', is_slug_source:true}]->  │
│       (SEOKeyword "crear-código-qr")    ← THIS is the slug source               │
│                                                                                 │
│  2. The slug source keyword provides the INPUT reference:                       │
│     SEOKeyword {key: "crear-codigo-qr@es-MX", slug_form: "crear-código-qr"}    │
│                                                                                 │
│  3. LLM DERIVES slug from slug_form using one of:                               │
│     copy     → "crear-código-qr"     (direct use)                              │
│     extract  → "crear-qr"            (no-repetition rule removes parent terms) │
│     merge    → "crear-código-qr-ai"  (brand/tech modifier)                     │
│     Locale rule (ADR-032) applies to final value:                               │
│     latin_preserve: "crear-código-qr" ✓ (ó retained)                          │
│     BlockNative.content.slug = derived value                                    │
│                                                                                 │
│  4. DERIVED_SLUG_FROM arc points to EntityNative (provenance via entity):       │
│     (BlockNative)-[:DERIVED_SLUG_FROM {derivation_score, derivation_rationale}] │
│       ->(EntityNative)                                                          │
│                                                                                 │
│  5. SLUGIFIED_BY arc validates against locale rules:                            │
│     (BlockNative)-[:SLUGIFIED_BY {validated: true, applied_rule: "latin_pre"}]->│
│       (Slugification)                                                           │
│                                                                                 │
│  6. Slug written back to EntityNative.denomination_forms[url]:                  │
│     EntityNative.denomination_forms += {type: "url", value: "crear-código-qr"} │
│                                                                                 │
│  QUERY: Find slug source keyword for a BlockNative                              │
│  MATCH (bn)-[:DERIVED_SLUG_FROM]->(en)-[:TARGETS {is_slug_source:true}]->(kw)  │
│  RETURN kw.slug_form AS reference_slug                                          │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Slugification Rules per Locale (ADR-032)

| Rule | Locales | Effect | Example |
|------|---------|--------|---------|
| latin_strip | en-US, en-GB | Remove diacritics | café → cafe |
| latin_preserve | fr-FR, es-MX, es-ES, pt-BR | Keep diacritics | código-qr ✓ |
| native_script | ja-JP, ko-KR, zh-CN, ar-SA | Non-Latin allowed | qrコード作成 ✓ |
| latin_transform | de-DE, de-AT | ü→ue, ö→oe, ä→ae, ß→ss | für → fuer |
| transliterate | ru-RU, uk-UA, el-GR | Cyrillic→Latin | код → kod |

## Key Design Decision: Entity.key != Page.slug

```
Entity.key:  "qr-code-instagram"  (full semantic identity)
Page.slug:   "instagram"          (just the URL segment)
```

This avoids: `/qr-code-generator/qr-code-instagram` (wrong)
We get: `/qr-code-generator/instagram` (correct)

## Concrete Example - 4 Entities

**Entity: instagram (BRAND)**
- No Page — external brand, not a page on our site
- Referenced via SEMANTIC_LINK from other entities

**Entity: qr-code-generator (PILLAR)**
```
Page.slug: "qr-code-generator"
BlockNative:head-seo-meta@fr-FR.slug: "generateur-qr-code"
BlockNative:head-seo-meta@fr-FR.full_path: "/fr/generateur-qr-code"
```

**Entity: qr-code-instagram (SUBTOPIC of qr-code-generator)**
```
Page.slug: "instagram"              # NOT "qr-code-instagram"
Page.SUBTOPIC_OF: page:qr-code-generator
BlockNative:head-seo-meta@fr-FR.slug: "instagram"    # Brand unchanged
BlockNative:head-seo-meta@fr-FR.full_path: "/fr/generateur-qr-code/instagram"
```

**Entity: template-instagram (SUBTOPIC of templates)**
```
Page.slug: "instagram"              # Same segment, different parent!
Page.SUBTOPIC_OF: page:templates
BlockNative:head-seo-meta@fr-FR.full_path: "/fr/modeles/instagram"
```

## Hierarchy Separation

```
SEMANTIC (Entity)              URL (Page)
─────────────────              ──────────
Entity:qr-code                 Page:qr-code
    │ SUBTOPIC_OF                  │ SUBTOPIC_OF
    ▼                              ▼
Entity:qr-code-instagram       Page:qr-code-instagram

SAME STRUCTURE but DIFFERENT PURPOSE:
- Entity hierarchy = topic/cluster (for content strategy)
- Page hierarchy = URL/navigation (for routing)
```

## Migration Required (v0.13.1)

**Removed from PageNative**:
- `slug`, `full_path`, `parent_slug`, `depth`, `slug_history` (routing_properties section)

**Added to BlockType**:
- `head-seo-meta` BlockType with slug, full_path, meta_title, meta_description

**Added arcs**:
- `[:DERIVED_SLUG_FROM]` (BlockNative → EntityNative) - provenance (Option B: all keyword knowledge stays on EntityNative)
- `[:SLUGIFIED_BY]` (BlockNative → Slugification) - validation

**Updated arcs**:
- `[:TARGETS {is_slug_source: true}]` added to EntityNative→SEOKeyword to mark slug source keyword

## Rationale

1. **Single Source of Truth**: head-seo-meta block owns URL for each locale
2. **No Duplication**: slug/full_path only in head-seo-meta BlockNative
3. **Flexibility**: Page.slug can differ from Entity.key
4. **Localization**: head-seo-meta has localized slug per locale
5. **Brands Protected**: "instagram" stays "instagram" everywhere
6. **Validation**: SLUGIFIED_BY arc validates diacritics per locale rules
7. **No Bypass**: DERIVED_SLUG_FROM → EntityNative keeps all keyword knowledge on EntityNative

## Reference Files

- `packages/core/models/node-classes/org/output/page-native.yaml` - routing_properties removed
- `packages/core/models/arc-classes/semantic/slugified-by.yaml` - validation arc
- `packages/core/models/arc-classes/generation/derived-slug-from.yaml` - provenance arc (BlockNative → EntityNative)
- `packages/db/seed/49-blocknative-head-seo-meta.cypher` - BlockNative:head-seo-meta instances (slug, meta_title, meta_description + DERIVED_SLUG_FROM arcs)
- `packages/db/seed/51-seokeywords-ahrefs.cypher` - SEOKeyword nodes (11 keywords, 5 locales, real Ahrefs data)
- `packages/db/seed/52-targets-arcs.cypher` - TARGETS arcs (EntityNative → SEOKeyword with is_slug_source)
- `packages/db/seed/53-blocktype-head-seo-meta.cypher` - BlockType:head-seo-meta definition
- `docs/plans/2026-02-14-entity-page-slug-brainstorm.md`
