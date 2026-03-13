# Entity Architecture Preview (v0.19.0)

**Brouillon** presentant le format CORRECT pour les proprietes standard.

---

## Format Specification (ADR-044)

| Property | Type | Format | Purpose |
|----------|------|--------|---------|
| `content` | string | **MARKDOWN** | DECLARATIVE - WHAT the node IS |
| `llm_context` | string | **MARKDOWN** | IMPERATIVE - HOW to USE it |
| `provenance` | string | **JSON** | Data origin (Neo4j limitation) |

---

## The Two Patterns

### Pattern 1: `content` (DECLARATIVE) - WHAT IT IS

```
DEFINITION: [semantic definition in one clear sentence].
ROLE: [function in NovaNet architecture, why this node exists].
SPECS: [technical characteristics, constraints, formats].
```

**Rules:**
- `DEFINITION:` - Required. One sentence maximum. Factual, not promotional.
- `ROLE:` - Required. Explains WHY this node exists in the architecture.
- `SPECS:` - Optional but recommended. Technical details, constraints.

### Pattern 2: `llm_context` (IMPERATIVE) - HOW TO USE IT

```
USE: when [primary use case for this element].
TRIGGERS: "keyword1", "keyword2", "keyword3".
NOT: for [what NOT to use this for] (use [alternative] instead).
RELATES: [Source] (role), [Target] (role), [Related] (relationship).
```

**Rules:**
- `USE:` - Required. Active voice, imperative mood.
- `TRIGGERS:` - Required. Comma-separated keywords in quotes.
- `NOT:` - Required. Always include `(use [alternative])` parenthetical.
- `RELATES:` - Required. Format: `[Node] (arc)`.

---

## Key Distinction

| Property | Purpose | Voice | Format |
|----------|---------|-------|--------|
| `content` | WHAT IT IS | Declarative | DEFINITION/ROLE/SPECS |
| `llm_context` | HOW TO USE IT | Imperative | USE/TRIGGERS/NOT/RELATES |

---

## Before vs After

### BEFORE (JSON strings - WRONG)

```yaml
content:
  example: '{"definition": "Two-dimensional barcode...", "context": "Pillar entity...", "technical": [...]}'

llm_context:
  example: '{"use": "When discussing 2D barcodes...", "triggers": ["qr", "qr code"], "not_for": ["1D barcodes"]}'
```

### AFTER (Structured Markdown - CORRECT)

```yaml
content:
  example: |
    DEFINITION: Two-dimensional barcode encoding data in a scannable visual pattern.
    ROLE: Pillar semantic concept anchoring all locale-specific content, pages, and SEO targeting for QR Code AI.
    SPECS: Matrix barcode with position detection patterns; error correction L/M/Q/H; capacity up to 4,296 alphanumeric characters; ISO/IEC 18004 standard.

llm_context:
  example: |
    USE: when discussing 2D barcodes, mobile scanning, contactless data transfer.
    TRIGGERS: "qr", "qr code", "scan", "mobile", "barcode", "matrix".
    NOT: for 1D barcodes (use Barcode), for NFC (use NFC Tag).
    RELATES: EntityNative (HAS_NATIVE), Page (REPRESENTS), Project (HAS_ENTITY).
```

---

## Examples by Node Class

### Entity

```yaml
content: |
  DEFINITION: Two-dimensional barcode encoding data in a scannable visual pattern.
  ROLE: Pillar semantic concept anchoring all locale-specific content, pages, and SEO targeting for QR Code AI.
  SPECS: Matrix barcode with position detection patterns; error correction L/M/Q/H; capacity up to 4,296 alphanumeric characters; ISO/IEC 18004 standard.

llm_context: |
  USE: when referencing the core semantic concept, creating new localized content, linking pages to topics.
  TRIGGERS: "entity", "concept", "topic", "pillar", "semantic", "definition".
  NOT: for locale-specific content (use EntityNative), for URL slugs (use Page.slug).
  RELATES: EntityNative (localized content via HAS_NATIVE), Page (structure via REPRESENTS), Project (ownership via HAS_ENTITY).
```

### EntityNative

```yaml
content: |
  DEFINITION: Code-barres bidimensionnel encodant des donnees dans un motif visuel scannable.
  ROLE: Contenu natif localise en francais pour l'entite QR Code, fournissant les formes canoniques.
  SPECS: Jusqu'a 4 296 caracteres alphanumeriques; correction d'erreurs integree; ISO/IEC 18004.

llm_context: |
  USE: when generating French content about QR codes, mobile scanning.
  TRIGGERS: "code qr", "scanner", "mobile", "code-barres matriciel".
  NOT: for English content (use entity:qr-code@en-US), for 1D barcodes (use Barcode).
  RELATES: Entity (NATIVE_OF), Locale (FOR_LOCALE), Page (slug derivation).
```

### Page

```yaml
content: |
  DEFINITION: Landing page for QR code creation in French.
  ROLE: Structural container owning URL slug, organizes content blocks.
  SPECS: slug from EntityNative.url, 4-6 blocks, SEO metadata.

llm_context: |
  USE: when orchestrating page-level generation.
  TRIGGERS: "page", "landing", "structure".
  NOT: for block content (use Block).
  RELATES: Block (HAS_BLOCK), Entity (REPRESENTS).
```

### Block

```yaml
content: |
  DEFINITION: Hero section for the French QR code creation page.
  ROLE: First visual block, captures attention, primary CTA.
  SPECS: block_type=hero, position=1, h1 + subtitle + CTA button.

llm_context: |
  USE: when generating hero section content.
  TRIGGERS: "hero", "header", "above fold", "cta".
  NOT: for FAQ content (use block_type=faq), for footer (use block_type=footer).
  RELATES: Page (BELONGS_TO), BlockNative (HAS_NATIVE), Entity (MENTIONS).
```

---

## Files in This Draft

| File | Description |
|------|-------------|
| `entity.yaml` | Entity with correct DEFINITION/ROLE/SPECS format |
| `entity-native.yaml` | EntityNative with correct format + denomination_forms |

---

## Changes Made

1. **content** - Changed from JSON string to structured markdown (DEFINITION/ROLE/SPECS)
2. **llm_context** - Changed from JSON to ADR-027 markdown pattern (USE/TRIGGERS/NOT/RELATES)
3. **Arc comments** - Moved to BLOC 6 at end of file
4. **Structure** - Follows BLOC 1-6 organization

---

## Next Steps

1. ✅ Review this draft
2. ⬜ Copy to live `packages/core/models/node-classes/org/semantic/`
3. ⬜ Run `cargo run -- schema validate`
4. ⬜ Update seeds in `packages/db/seed/`

---

## Reference

- ADR-044: Eight Standard Properties (DEFINITION/ROLE/SPECS + USE/TRIGGERS/NOT/RELATES)
- ADR-033: Denomination Forms
- v0.19.0 Migration Plan: `/docs/plans/2026-03-11-v019-migration-master-plan.md`

---

## Architecture Diagram

```
+===============================================================================+
|  ENTITY ARCHITECTURE (v0.19.0) - Page-Entity-SEO Flow                         |
+===============================================================================+

                           ┌──────────────────┐
                           │     Project      │
                           │   (foundation)   │
                           └────────┬─────────┘
                                    │ [:HAS_ENTITY]
                                    ▼
+-------------------------------------------------------------------------------+
|  SEMANTIC LAYER (org/semantic)                                                 |
+-------------------------------------------------------------------------------+
|                                                                               |
|   ┌─────────────────────────────────────────────────────────────────────┐     |
|   │  Entity (defined)                                                   │     |
|   │  key: "entity:qr-code"                                              │     |
|   │  content: "DEFINITION: ... ROLE: ... SPECS: ..."                    │     |
|   │  llm_context: "USE: ... TRIGGERS: ... NOT: ... RELATES: ..."        │     |
|   └───────────────────────────────┬─────────────────────────────────────┘     |
|                                   │                                           |
|                                   │ [:HAS_NATIVE]                             |
|                                   ▼                                           |
|   ┌─────────────────────────────────────────────────────────────────────┐     |
|   │  EntityNative (authored, per locale)                                │     |
|   │  key: "entity:qr-code@fr-FR"                                        │     |
|   │  locale: "fr-FR"                                                    │     |
|   │  content: "DEFINITION: Code-barres... ROLE: ... SPECS: ..."         │     |
|   │  denomination_forms: [                                               │     |
|   │    {type: "text",   value: "code QR"},                              │     |
|   │    {type: "title",  value: "Code QR"},                              │     |
|   │    {type: "abbrev", value: "QR"},                                   │     |
|   │    {type: "url",    value: "code-qr"}  <-- SEO pipeline write-back  │     |
|   │  ]                                                                  │     |
|   └───────────────────────────────┬─────────────────────────────────────┘     |
|                                   │ [:FOR_LOCALE]                             |
|                                   ▼                                           |
|   ┌─────────────────────────────────────────────────────────────────────┐     |
|   │  Locale (shared/config)                                             │     |
|   │  key: "fr-FR"                                                       │     |
|   │  script: "latin"                                                    │     |
|   └─────────────────────────────────────────────────────────────────────┘     |
|                                                                               |
+-------------------------------------------------------------------------------+

+-------------------------------------------------------------------------------+
|  STRUCTURE LAYER (org/structure)                                              |
+-------------------------------------------------------------------------------+
|                                                                               |
|   ┌─────────────────────────────────────────────────────────────────────┐     |
|   │  Page (defined)                          [:ABOUT {role:"focus"}]    │     |
|   │  key: "page:qr-code-generator"     ───────────────────────────► Entity    |
|   │  content: "DEFINITION: ... ROLE: ... SPECS: ..."                    │     |
|   └───────────────────────────────┬─────────────────────────────────────┘     |
|                                   │                                           |
|              ┌────────────────────┼────────────────────┐                      |
|              │ [:HAS_BLOCK]       │ [:HAS_NATIVE]      │                      |
|              ▼                    ▼                    ▼                      |
|   ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐           |
|   │  Block (hero)    │  │  PageNative      │  │  Block (faq)     │           |
|   │  position: 1     │  │  @fr-FR          │  │  position: 3     │           |
|   └────────┬─────────┘  │  slug: "code-qr" │  └────────┬─────────┘           |
|            │            │  ↑ from          │           │                      |
|            │            │    EntityNative  │           │                      |
|            │            │    .url form     │           │                      |
|            │            └──────────────────┘           │                      |
|            │ [:HAS_NATIVE]                 [:HAS_NATIVE]│                      |
|            ▼                                           ▼                      |
|   ┌──────────────────┐                       ┌──────────────────┐            |
|   │  BlockNative     │                       │  BlockNative     │            |
|   │  (generated)     │                       │  (generated)     │            |
|   │  @fr-FR          │                       │  @fr-FR          │            |
|   └──────────────────┘                       └──────────────────┘            |
|                                                                               |
+-------------------------------------------------------------------------------+

+-------------------------------------------------------------------------------+
|  SLUGIFICATION PIPELINE (ADR-030 + ADR-033)                                   |
+-------------------------------------------------------------------------------+
|                                                                               |
|   Entity.key = "entity:qr-code"                                               |
|         │                                                                     |
|         │ [:HAS_NATIVE]                                                       |
|         ▼                                                                     |
|   EntityNative (per locale)                                                   |
|         │                                                                     |
|         ├── en-US: denomination_forms.url = "qr-code-generator"               |
|         │          (latin_strip: remove diacritics)                           |
|         │                                                                     |
|         ├── fr-FR: denomination_forms.url = "code-qr"                         |
|         │          (latin_preserve: keep diacritics)                          |
|         │                                                                     |
|         ├── de-DE: denomination_forms.url = "qr-code-erstellen"               |
|         │          (latin_transform: u->ue, o->oe)                            |
|         │                                                                     |
|         ├── ja-JP: denomination_forms.url = "qr-code-sakusei"                 |
|         │          (native_script: ALWAYS romanized ASCII for URL)            |
|         │                                                                     |
|         └── ru-RU: denomination_forms.url = "qr-kod"                          |
|                    (transliterate: Cyrillic -> Latin)                         |
|         │                                                                     |
|         │ SEO Pipeline Write-Back                                             |
|         ▼                                                                     |
|   PageNative.slug = EntityNative.denomination_forms.url                       |
|         │                                                                     |
|         ▼                                                                     |
|   URL: /{locale}/{slug} -> /fr-FR/code-qr                                     |
|                                                                               |
+-------------------------------------------------------------------------------+

+===============================================================================+
|  LEGEND                                                                       |
+===============================================================================+
|                                                                               |
|   (defined)   = Structurally fixed, version-controlled                        |
|   (authored)  = Human-authored locale-specific content                        |
|   (generated) = LLM-generated output                                          |
|                                                                               |
|   [:HAS_NATIVE]  = Ownership arc (parent -> child)                            |
|   [:FOR_LOCALE]  = Localization arc (content -> locale)                       |
|   [:ABOUT]       = Semantic arc (page -> entity, with role+weight)            |
|   [:HAS_BLOCK]   = Ownership arc (page -> block, with order)                  |
|                                                                               |
+===============================================================================+
```
