---
id: "028"
title: "Page-Entity Architecture"
version: "v0.12.3"
status: "active"
domain: "schema-architecture"
---

# ADR-028: Page-Entity Architecture

**Status**: Approved (v0.12.3)

**v0.12.3 Additions** (Research-backed AI prompt refinements):
- **Refined visual_prompt schema** based on Midjourney, DALL-E 3, Sora, Stable Diffusion best practices
- Added `weighted_modifiers` with Stable Diffusion weight syntax `{ term: "X", weight: 1.4 }`
- Added structured `negative_prompts` (content, technical, style, cultural categories)
- Added `cinematography` section for Sora video generation (camera_movement, shot_type, direction)
- Added `platform_hints` for cross-platform compatibility (Midjourney, DALL-E, SD, Sora parameters)
- Added `quality` section with DALL-E 3 API parameters (`dalle_style`, `dalle_quality`)
- Added prompt compilation pipeline showing how visual_prompt converts to platform-specific formats

**v0.12.2 Additions**:
- Brand Architecture (Atlas Pattern): Brand (Soul + Pitch + Voice) + BrandDesign + BrandPrinciples
- PromptStyle system for AI image/video generation
- Geographic `cultural_style` properties on Continent/GeoRegion/GeoSubRegion
- Geographic `visual_prompt` for AI generation (image, video, illustration, product_3d)
- New @ references: `@brand.design`, `@brand.principles`, `@prompt:X`, `@geo:X.visual_prompt`
- Merge algorithm: Brand.PromptStyle + Geographic visual_prompt hierarchy

**Problem**: Page and Entity relationships lacked clear architecture:
1. No enforced Page↔Entity relationship (some pages had Entity, some didn't)
2. Order stored redundantly (PageStructure JSON AND [:HAS_BLOCK].order)
3. No formal @ reference system for content injection vs links
4. Unclear separation between technical constraints (BlockType) and creative instructions (BlockInstruction)

**Decision**: Establish 1:1 mandatory Page↔Entity architecture with @ reference system and calculated structure.

## Core Principles

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  PAGE-ENTITY ARCHITECTURE PRINCIPLES                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. Page ↔ Entity = 1:1 OBLIGATOIRE                                             │
│     └─ Every Page MUST have exactly one Entity via [:REPRESENTS]                │
│     └─ Even utility pages (contact, legal) have their own Entity                │
│                                                                                 │
│  2. Slug = Entity.key (SOURCE OF TRUTH)                                         │
│     └─ Entity.key = "qr-generator" → URL = /qr-generator                        │
│     └─ Page.key is DERIVED from Entity.key, not independent                     │
│                                                                                 │
│  3. Order on Arc (SINGLE SOURCE)                                                │
│     └─ [:HAS_BLOCK {order: N}] is the ONLY place order is stored                │
│     └─ PageStructure = CALCULATED from Block order                              │
│     └─ PageInstruction = CALCULATED from BlockInstruction concatenation         │
│                                                                                 │
│  4. @ References: Injection vs Links                                            │
│     └─ @type:key = injection (LLM context, no HTML link)                        │
│     └─ [@type:key] = link (creates <a href>)                                    │
│                                                                                 │
│  5. Separation of Concerns                                                      │
│     └─ BlockType = constraints (schema, behaviors, lengths)                     │
│     └─ BlockInstruction = creativity (@ refs, what to say)                      │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Page ↔ Entity Relationship

```
Page (structure, defined) ──[:REPRESENTS]──▶ Entity (semantic, defined)
                           1:1 OBLIGATOIRE
```

**EntityCategory examples for "all pages have Entity":**

| Category | Pages | Why Entity? |
|----------|-------|-------------|
| product | /qr-code-generator | Product entity |
| feature | /api-documentation | Feature entity |
| pricing | /pricing | Business concept entity |
| legal | /terms-of-service | Legal document entity |
| support | /contact | Contact concept entity |
| index | /blog | Collection entity |

**Sub-pages**: `/pricing/enterprise` → Entity "pricing-enterprise" (NOT child of pricing)
- No Page parent/child hierarchy
- Flat Entity structure, composite keys if needed

## Block.key Composite Format

```
Block.key = "{page_key}:{block_type}:{index}"
```

Examples:
- `homepage:hero:1` — first hero on homepage
- `pricing:hero:1` — hero on pricing (different from homepage)
- `homepage:testimonials:1` — first testimonials
- `homepage:testimonials:2` — second testimonials (if repeated)

**Benefits:**
- Globally unique (no collision between pages)
- Parseable (extract page, type, index)
- Allows multiple blocks of same type per page

## @ Reference System

### Injection (LLM Context)

```
@type:key              → Inject content (NO HTML link)
```

| Syntaxe | Effet | Exemple |
|---------|-------|---------|
| `@entity:X` | Inject EntityNative(X@locale) | `@entity:tier-pro` |
| `@entity:X.field` | Inject specific field | `@entity:tier-pro.tagline` |
| `@project` | Inject ProjectNative | Global project context |
| `@brand` | Inject Brand (soul, pitch, voice) | `@brand.elevator_pitch` |
| `@brand.design` | Inject BrandDesign | `@brand.design.style_mood` |
| `@brand.principles` | Inject BrandPrinciples | `@brand.principles.heuristics` |
| `@prompt:X` | Inject PromptStyle preset | `@prompt:hero-illustration` |
| `@design.tokens.X` | Inject design token | `@design.tokens.semantic.colors.primary` |
| `@geo:X` | Inject cultural_style from geography | `@geo:EA` (Eastern Asia) |
| `@geo:X.visual_prompt` | Inject AI visual prompt preset | `@geo:JP.visual_prompt` |
| `@geo:X.visual_prompt.image` | Inject image generation preset | `@geo:JP.visual_prompt.image` |
| `@geo:X.visual_prompt.video` | Inject video generation preset | `@geo:JP.visual_prompt.video` |
| `@audience:X` | Inject AudiencePersona | `@audience:developers` |
| `@block:X` | Inject BlockNative/Instruction | `@block:shared-footer` |
| `@term:X` | Inject Term(X@locale) | `@term:subscription` |
| `@expr:X` | Inject Expression(X@locale) | `@expr:call-to-action` |
| `@seo:X` | Inject SEOKeyword | `@seo:qr-generator` |
| `@competitor:X` | Inject competitor context | `@competitor:qr-monkey` |

### Links (HTML Output)

```
[@type:key]            → Creates <a href>
[@type:key|anchor]     → Custom anchor text
```

| Syntaxe | Resultat HTML |
|---------|---------------|
| `[@page:X]` | `<a href="/X">{page.title}</a>` |
| `[@page:X\|@entity:Y]` | `<a href="/X">{entity.name}</a>` |
| `[@page:X\|@term:Y]` | `<a href="/X">{term.value}</a>` |
| `[@page:X\|"text"]` | `<a href="/X">text</a>` |
| `[@page:X#section]` | `<a href="/X#section">...</a>` |
| `[@external:X]` | `<a href="{url}">...</a>` |

## Architecture Layers

```
Brand (1 per Project, Atlas Pattern)
│ Soul: purpose, mission, vision (who we are)
│ Pitch: what, for_whom, how, elevator_pitch (positioning)
│ Voice: voice, tone, humor, formality, values (communication)
│
├──[:HAS_DESIGN]──────────▶ BrandDesign (1:1)
│   │ design_philosophy, style_keywords, style_mood
│   │ tokens (primitives → semantic → component)
│   └── typography, ui patterns
│
├──[:HAS_PRINCIPLES]──────▶ BrandPrinciples (1:1)
│   │ heuristics (trigger, rule, rationale)
│   └── do/dont rules for LLM decision-making
│
├──[:HAS_PROMPT_STYLE]────▶ PromptStyle* (1:N presets)
│   │ style, subject, environment, lighting
│   │ color_palette, composition, mood, quality
│   │
│   ├──[:INSPIRED_BY_REGION]▶ GeoRegion (cultural inspiration)
│   └──[:FOR_LOCALE]────────▶ Locale (locale-specific)
│
└──[:TARGETS_PERSONA]─────▶ AudiencePersona* (semantic link)

Geographic Cultural Styles (on Continent, GeoRegion, GeoSubRegion):
│ color_preferences, visual_style, typography, cultural_codes
│
└── At generation: Brand.PromptStyle + Geo.cultural_style merged
```

**Inheritance cascade**: Brand → BlockType → BlockInstruction (each can override)

## Field Behaviors

| Behavior | Description | Exemple |
|----------|-------------|---------|
| `translate` | LLM generates natively for locale | title, description |
| `fixed` | Copied as-is (no processing) | urls, image paths, ids |
| `derive` | LLM derives/paraphrases from source | meta_description from title |
| `copy` | Copy from another Block | shared footer |
| `computed` | Calculated (not LLM) | reading_time from body.length |
| `conditional` | Behavior depends on context | legal_text: fixed if US |

## Calculated Concepts (NOT Stored)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  CALCULATED AT GENERATION TIME (not stored as nodes)                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  PageStructure (calculated)                                                     │
│  └── Query: MATCH (p:Page)-[r:HAS_BLOCK]->(b:Block)-[:OF_TYPE]->(bt:BlockType)  │
│             RETURN bt ORDER BY r.order                                          │
│  └── Result: [BlockType schemas in order]                                       │
│                                                                                 │
│  PageInstruction (calculated)                                                   │
│  └── Query: MATCH (p:Page)-[r:HAS_BLOCK]->(b:Block)-[:HAS_INSTRUCTION]->(bi)    │
│             RETURN bi.content ORDER BY r.order                                  │
│  └── Result: Concatenated BlockInstructions                                     │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Link Model (3 Levels)

```
Level 1: MENTIONS (granular)
BlockInstruction ──[:MENTIONS]──▶ Entity|Page|Term|...
                   { position: N, ref_type: "entity|page|term|...", purpose: "inject|link" }

Level 2: REFERENCES (per block)
Block ──[:REFERENCES]──▶ Entity
        { purpose: "inject|link", count: N }

Level 3: LINKS_TO (per page)
Page ──[:LINKS_TO]──▶ Page
       { via_blocks: ["hero", "pricing"], strength: N }
```

## New Arcs

| Arc | Source | Target | Family | Properties |
|-----|--------|--------|--------|------------|
| `REPRESENTS` | Page | Entity | semantic | — (1:1 mandatory) |
| `LINKS_TO` | Page | Page | semantic | via_blocks[], strength |
| `REFERENCES` | Block | Entity | semantic | purpose, count |
| `MENTIONS` | BlockInstruction | * | semantic | position, ref_type, purpose |
| `HAS_BRAND` | Project | Brand | ownership | — (1:1) |
| `HAS_KEYWORD` | Entity | SEOKeyword | ownership | rank (primary/secondary) |
| `HAS_PAGE` | Project | Page | ownership | — |
| `HAS_ENTITY` | Project | Entity | ownership | — |
| `HAS_DESIGN` | Brand | BrandDesign | ownership | — (1:1) |
| `HAS_PRINCIPLES` | Brand | BrandPrinciples | ownership | — (1:1) |
| `HAS_PROMPT_STYLE` | Brand | PromptStyle | ownership | — (1:N) |
| `TARGETS_PERSONA` | Brand | AudiencePersona | semantic | priority |
| `FOR_MARKET` | Brand | Market | semantic | — |
| `INSPIRED_BY_REGION` | PromptStyle | GeoRegion | semantic | — |
| `FOR_LOCALE` | PromptStyle | Locale | localization | — |

## Supersedes ADR-025 (Partial)

This ADR supersedes the **Pipeline** section of ADR-025:
- `PageStructure` node → CALCULATED (not stored)
- `PageInstruction` node → CALCULATED (not stored)
- `[:HAS_STRUCTURE]` (Page→PageStructure) → REMOVED
- `[:HAS_INSTRUCTION]` (Page→PageInstruction) → REMOVED
- `[:HAS_BLOCK {order}]` → SINGLE source of truth for block order

**BlockType and BlockInstruction remain as nodes** (ADR-025 is still valid for those).

## Validation Rules

1. Every Page MUST have exactly one `[:REPRESENTS]` to Entity
2. Page.key MUST equal Entity.key
3. `[:HAS_BLOCK].order` must be unique per Page (no duplicates)
4. `[:LINKS_TO]` arcs are calculated from @ refs with `purpose: link`
5. Invalid @ refs generate validation errors

## Migration Impact

**Nodes removed:**
- PageStructure (calculated instead)
- PageInstruction (calculated instead)

**Arcs removed:**
- [:HAS_STRUCTURE] (Page→PageStructure)
- [:HAS_INSTRUCTION] (Page→PageInstruction)

**Arcs added:**
- [:REPRESENTS] (Page→Entity)
- [:LINKS_TO] (Page→Page)
- [:REFERENCES] (Block→Entity)
- [:MENTIONS] (BlockInstruction→*)
- [:HAS_BRAND] (Project→Brand)
- [:HAS_KEYWORD] (Entity→SEOKeyword)
- [:HAS_PAGE] (Project→Page)
- [:HAS_ENTITY] (Project→Entity)
- [:HAS_DESIGN] (Brand→BrandDesign)
- [:HAS_PRINCIPLES] (Brand→BrandPrinciples)
- [:HAS_PROMPT_STYLE] (Brand→PromptStyle)
- [:TARGETS_PERSONA] (Brand→AudiencePersona)
- [:FOR_MARKET] (Brand→Market)
- [:INSPIRED_BY_REGION] (PromptStyle→GeoRegion)
- [:FOR_LOCALE] (PromptStyle→Locale)

**New nodes (v0.12.2):**
- Brand (replaces BrandIdentity, org/foundation)
- BrandDesign (org/foundation)
- BrandPrinciples (org/foundation)
- PromptStyle (org/foundation)

**Modified nodes (v0.12.2):**
- Continent, GeoRegion, GeoSubRegion, Country: added `cultural_style` property
- Continent, GeoRegion, GeoSubRegion, Country: added `visual_prompt` property (AI generation presets)

**Rationale:**

1. **1:1 Mandatory**: Eliminates "some pages have Entity" ambiguity
2. **Single Order Source**: `[:HAS_BLOCK {order}]` prevents redundancy
3. **@ Reference System**: Clear syntax for injection vs links
4. **Calculated Structure**: Avoids sync issues between stored and derived data
5. **Separation of Concerns**: BlockType (constraints) vs BlockInstruction (creativity)

**Reference**: `docs/plans/2026-02-13-page-entity-refs-design.md`
