# QR Code AI — Entity Design for NovaNet

**Date**: 2026-02-07
**Status**: Complete (v10.7)
**Project**: QR Code AI (qrcode-ai.com)
**Version**: v10.7 Entity Schema

---

## Executive Summary

QR Code AI is a platform with **2 main products** (QR Code, Smart Link) that share common infrastructure (Short Link, Analytics). This document defines the complete Entity structure for NovaNet knowledge graph.

**Total Entities**: ~279 across 13 types
**Semantic Arcs**: ~825 (VARIANT_OF, REQUIRES, ENABLES, etc.)

---

## Research-Based Design Decisions (v10.7)

Based on deep research (Perplexity + Context7) on knowledge graph ontology best practices,
GraphRAG patterns, and Neo4j relationship modeling.

### Decision 1: Typed Relationships (not SEMANTIC_LINK)

**Choice**: Use specific relationship types instead of generic SEMANTIC_LINK container.

```cypher
# ❌ OLD: Generic container
(a)-[:SEMANTIC_LINK {type: "variant_of", strength: 0.85}]->(b)

# ✅ NEW: Typed relationships
(a)-[:VARIANT_OF {strength: 0.85}]->(b)
```

**Relationship Types** (by category):

| Category | Types |
|----------|-------|
| Hierarchical | `TYPE_OF`, `VARIANT_OF`, `SUBTOPIC_OF`, `INCLUDES` |
| Dependency | `REQUIRES`, `ENABLES`, `DEPENDS_ON` |
| Associative | `RELATED_TO`, `SIMILAR_TO`, `CONTRASTS_WITH` |
| Functional | `USED_FOR`, `PART_OF`, `IS_ACTION_ON` |
| Identity | `SAME_AS`, `ALIAS_OF` |
| Cross-realm | `POPULAR_IN` (Entity → GeoRegion) |

**Rationale**: GraphRAG traversal benefits from typed edges; meta-graph visibility; clearer semantics.

### Decision 2: Property Name = `strength`

**Choice**: Use `strength` (not `weight`, `relevance`, or `confidence`).

```cypher
(a)-[:VARIANT_OF {strength: 0.85}]->(b)
```

**Rationale**: Aligns with GraphRAG convention (`relationship_strength`); more expressive than technical "weight".

### Decision 3: Add `entity_summary` to Entity Schema

**Choice**: Add `entity_summary` property for LLM context optimization.

```yaml
# Entity properties
entity_summary:
  type: string
  required: false
  description: "2-3 sentence summary for LLM context loading"
```

**Rationale**: GraphRAG requires `entity_description` for effective context. Our existing
`description` (short) + `llm_context` (structured) + new `entity_summary` (prose) covers all needs.

### Decision 4: No Description on Individual Arcs

**Choice**: The relationship TYPE carries the semantics; no `description` property on each arc.

```cypher
# Sufficient - type is self-documenting
(a)-[:VARIANT_OF {strength: 0.85}]->(b)

# Not needed - 825 descriptions would be overkill
(a)-[:VARIANT_OF {strength: 0.85, description: "Custom QR codes are..."}]->(b)
```

**Rationale**: With typed relationships, the semantics are clear. Adding descriptions to 825 arcs
would be maintenance burden with little value.

### Decision 5: Hybrid Key Format (YAML vs Neo4j)

**Choice**: Type-prefixed keys in YAML for readability, flat keys in Neo4j for stability.

```yaml
# YAML arc files (human-readable, self-documenting)
arcs:
  - from: thing:custom-qr-code
    to: thing:qr-code
    type: VARIANT_OF
    strength: 0.85
```

```cypher
// Neo4j (stable flat keys)
MATCH (a:Entity {key: 'custom-qr-code', entity_type: 'THING'})
MATCH (b:Entity {key: 'qr-code', entity_type: 'THING'})
MERGE (a)-[:VARIANT_OF {strength: 0.85}]->(b)
```

**Generator Validation**:
1. Parse `thing:custom-qr-code` → `(type: "thing", key: "custom-qr-code")`
2. Lookup Entity with `key = "custom-qr-code"`
3. Verify `entity.entity_type == "THING"`
4. If mismatch → ERROR

**Rationale**:
- YAML is self-documenting (you see the relationship nature immediately)
- Neo4j keys are stable (type changes don't break keys)
- URLs can use flat keys: `/topics/qr-code`
- Cross-validation ensures consistency

---

## Graph Hierarchy

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  OWNERSHIP HIERARCHY                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Organization (SuperNovae Studio)                                           │
│       │                                                                     │
│       └──[:HAS_PROJECT]──> Project (QR Code AI)                            │
│                                  │                                          │
│                                  └──[:HAS_ENTITY]──> Entity (invariant)     │
│                                                          │                  │
│                                                          └──[:HAS_L10N]──>  │
│                                                              EntityL10n     │
│                                                              (per locale)   │
│                                                                  │          │
│                                           ┌──────────────────────┴──────────┐
│                                           │                                 │
│                                           ▼                                 ▼
│                              [:FOR_LOCALE]──> Locale     [:TARGETS]──> SEOKeyword
│                                           (fr-FR)              (separate node!)
│                                                                     │
│                                                          ┌──────────┴──────────┐
│                                                          │ value: "Code QR"    │
│                                                          │ volume: 40500       │
│                                                          │ difficulty: 35      │
│                                                          │ intent: transact    │
│                                                          └─────────────────────┘
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  KEY PRINCIPLE                                                              │
│  • Entity = invariant (EN) - what it IS, universal key                     │
│  • EntityL10n = locale-native content (per locale)                         │
│  • SEOKeyword = SEPARATE NODE with metrics (volume, difficulty, CPC)       │
│  • EntityL10n --[:TARGETS]--> SEOKeyword (many-to-many)                    │
│                                                                             │
│  CRITICAL: "Code QR" (FR) ≠ "QR Code" (EN) ≠ "QRコード" (JA)               │
│  Research real search terms with Ahrefs/SEMrush, not translations!         │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## SEO Schema Architecture (v10.6)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SEO KEYWORD GRAPH                                                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Locale (fr-FR) ──[:HAS_SEO_KEYWORDS]──> SEOKeyword                        │
│                                               │                             │
│                                               ├─ value: "Code QR"           │
│                                               ├─ volume: 40500              │
│                                               ├─ difficulty: 35             │
│                                               ├─ intent: transactional      │
│                                               ├─ traffic_potential: 35000   │
│                                               ├─ serp_features: [...]       │
│                                               └─ trend: rising              │
│                                                   │                         │
│       ┌───────────────────┬───────────────────┬───┴───────────┐            │
│       │                   │                   │               │            │
│       ▼                   ▼                   ▼               ▼            │
│  [:HAS_METRICS]     [:HAS_QUESTIONS]    [:HAS_COMPARISONS] [:HAS_PREPOS.]  │
│       │                   │                   │               │            │
│       ▼                   ▼                   ▼               ▼            │
│  SEOKeywordMetrics   SEOQuestion        SEOComparison    SEOPreposition   │
│  (time-series)       (from ATP)         (X vs Y)         (X for Y)        │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐  ┌─────────────┐  │
│  │observed_at   │   │value:        │   │value:        │  │value:       │  │
│  │volume        │   │"comment      │   │"Code QR vs   │  │"Code QR     │  │
│  │difficulty    │   │ créer..."    │   │ barcode"     │  │ pour wifi"  │  │
│  │position      │   │question_word │   │entity_a_key  │  │preposition  │  │
│  │impressions   │   │answer_type   │   │entity_b_key  │  │use_case     │  │
│  │clicks_gsc    │   │featured_snip │   │winner        │  │use_case_type│  │
│  │ctr           │   │paa_position  │   │              │  │             │  │
│  └──────────────┘   └──────────────┘   └──────────────┘  └─────────────┘  │
│                           │                   │               │            │
│                           ▼                   ▼               ▼            │
│                      COMPARES_A/B         USE_CASE_ENTITY                  │
│                           │                   │               │            │
│                           └───────────────────┴───────────────┘            │
│                                       │                                     │
│                                       ▼                                     │
│                                    Entity (invariant)                       │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  ENTITYL10N → SEO RELATIONS                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  EntityL10n ──[:TARGETS]──> SEOKeyword (N:N, targeting for ranking)        │
│  EntityL10n ──[:ANSWERS]──> SEOQuestion (N:N, content coverage)            │
│  PageL10n ──[:ADDRESSES]──> SEOComparison (coverage tracking)              │
│  BlockL10n ──[:ADDRESSES]──> SEOPreposition (coverage tracking)            │
│                                                                             │
│  SEOKeyword ──[:EXPRESSES]──> Entity (semantic link back)                  │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  MINING JOB                                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  SEOMiningRun ──[:SEO_MINES]──> SEOKeyword (discovery tracking)            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Key Points:**
- **SEOKeyword**: Root keyword with rich metrics (volume, difficulty, CPC, intent, traffic_potential)
- **SEOKeywordMetrics**: Time-series snapshots for historical tracking (GSC + Ahrefs/Semrush)
- **SEOQuestion**: Question-form keywords from Answer The Public ("comment créer...")
- **SEOComparison**: Comparison keywords ("X vs Y") with links to 2 Entities
- **SEOPreposition**: Preposition keywords ("X for Y") with use case Entity
- **All derived nodes** (Question/Comparison/Preposition) have their own volume/difficulty

---

## Product Architecture

```
                       ┌─────────────────┐
                       │   Short Link    │
                       │  (infra layer)  │
                       └────────┬────────┘
                                │
                 ┌──────────────┴──────────────┐
                 │                             │
                 ▼                             ▼
┌─────────────────────────────┐  ┌─────────────────────────────┐
│         QR Code             │  │        Smart Link           │
│        (pillar)             │  │         (pillar)            │
├─────────────────────────────┤  ├─────────────────────────────┤
│ Focus: VISUAL encoding      │  │ Focus: LINK intelligence    │
│ Metric: "Scans"             │  │ Metric: "Clicks"            │
│ Domain: qrc-ai.com          │  │ Domain: qrc.sh              │
│                             │  │                             │
│ CAN BE:                     │  │ ALWAYS:                     │
│ • Static (WiFi, vCard...)   │  │ • Dynamic (has Short Link)  │
│ • Dynamic (URL-based)       │  │ • Trackable                 │
│                             │  │ • Editable                  │
└─────────────────────────────┘  └─────────────────────────────┘
```

---

## Entity Type System (v10.7) — 13 Types

```
┌─────────────────────────────────────────────────────────────────────────┐
│  QUESTION     │  TYPE         │  DESCRIPTION                           │
├───────────────┼───────────────┼────────────────────────────────────────┤
│  WHAT?        │  THING        │  Core products/objects (QR Code)       │
│               │  CONTENT_TYPE │  What QR encodes (URL, WiFi, vCard)    │
│               │  FEATURE      │  Software capabilities (Tracking)      │
│               │  TOOL         │  Generators, scanners, builders        │
├───────────────┼───────────────┼────────────────────────────────────────┤
│  WHERE?       │  MEDIUM       │  Surfaces/placements (posters, cards)  │
├───────────────┼───────────────┼────────────────────────────────────────┤
│  WHY?         │  USE_CASE     │  Application scenarios (marketing)     │
├───────────────┼───────────────┼────────────────────────────────────────┤
│  WHO?         │  INDUSTRY     │  Vertical markets (restaurants)        │
├───────────────┼───────────────┼────────────────────────────────────────┤
│  HOW?         │  ACTION       │  User verbs (create, scan, track)      │
│               │  GUIDE        │  How-to instructional content          │
│               │  COMPARISON   │  Versus content (static vs dynamic)    │
├───────────────┼───────────────┼────────────────────────────────────────┤
│  ABSTRACT     │  CONCEPT      │  Educational ideas (dynamic, static)   │
├───────────────┼───────────────┼────────────────────────────────────────┤
│  EXTERNAL     │  BRAND        │  Third-party brands (Google, PayPal)   │
│               │  INTEGRATION  │  Third-party integrations (Zapier)     │
└───────────────┴───────────────┴────────────────────────────────────────┘

NOTE: Geographic targeting uses cross-realm links to global nodes
      (GeoRegion, GeoSubRegion, Continent) instead of an entity type.
      See: [:POPULAR_IN] arc from Entity to global geographic nodes.
```

| Type | Purpose | Example |
|------|---------|---------|
| **THING** | Core products/objects | QR Code, Smart Link, Barcode, Landing Page |
| **CONTENT_TYPE** | What QR encodes | QR Code URL, QR Code WiFi, QR Code Instagram |
| **FEATURE** | Software capability | Analytics, Contextual Routing, Bulk QR Codes |
| **TOOL** | Generators, scanners | QR Code Generator, Barcode Scanner |
| **MEDIUM** | Surfaces/placements | Business Cards, Posters, Flyers, Packaging |
| **USE_CASE** | Application scenarios | Digital Marketing, Contactless, Event Management |
| **INDUSTRY** | Vertical markets | Restaurants, Retail, Healthcare, Marketing Agencies |
| **ACTION** | What user DOES | Create QR Code, Scan, Track, Design |
| **GUIDE** | How-to content | Print Guide, Size Calculator |
| **COMPARISON** | Versus content | Static vs Dynamic QR Codes |
| **CONCEPT** | Educational abstraction | Dynamic QR Code, Static QR Code, Quiet Zone |
| **BRAND** | Third-party brands | Google, Instagram, PayPal, Spotify |
| **INTEGRATION** | Third-party integrations | Zapier, Google Sheets, HubSpot |

---

## Phase 1: Core Products (THING) — 39 Entities

### Hierarchy Overview

```
qr-code (PILLAR)
    ├──[includes]──> qr-code-style (category)
    │                   ├──> custom-qr-code ──> design specs
    │                   ├──> qr-code-image
    │                   │       └── qr-code-photo (alias, same_as)
    │                   └──> qr-code-art
    │
    ├──[includes]──> qr-code-content (category) → Phase 2
    │
    └──[includes]──> qr-code-frame (category)
                        └──> business-card, poster, flyer, etc.

smart-link (PILLAR) ──[type_of]──> short-link
barcode (PILLAR) ──[includes]──> barcode-format → Phase 3
landing-page (PILLAR) ──[includes]──> landing-page-type
```

### 1.1 Pillars (4) — is_pillar: true

| Key | Type | Description |
|-----|------|-------------|
| `qr-code` | THING | 2D matrix barcode, core product pillar |
| `smart-link` | THING | Intelligent shortened URL with routing |
| `barcode` | THING | 1D linear barcode (EAN, UPC, Code 128) |
| `landing-page` | THING | Destination page created via builder |

### 1.2 Infrastructure (1)

| Key | Type | Description |
|-----|------|-------------|
| `short-link` | THING | Shortened URL, shared tech layer |

### 1.3 Categories (5)

| Key | Type | Parent | Contains |
|-----|------|--------|----------|
| `qr-code-style` | THING | qr-code | custom, image, art |
| `qr-code-content` | THING | qr-code | url, wifi, social, etc. (Phase 2) |
| `qr-code-frame` | THING | qr-code | business-card, poster, etc. |
| `barcode-format` | THING | barcode | ean-13, upc-a, etc. (Phase 3) |
| `landing-page-type` | THING | landing-page | link-in-bio, menu, etc. |

### 1.4 QR Code Styles (3 + 1 alias)

| Key | Type | Semantic | Light/Dark |
|-----|------|----------|------------|
| `custom-qr-code` | THING | PARAMETRIC: user configures each element | ✓ both |
| `qr-code-image` | THING | SUPERPOSITION: image bg + QR fg (2 layers) | ✓ both |
| `qr-code-art` | THING | FUSION: AI generates unified artwork | ✓ both |
| `qr-code-photo` | THING | **Alias** (same_as qr-code-image) | - |

### 1.5 Design Specs (9) — Apply to styles

```
┌─────────────────────────────┬────────┬───────┬─────┐
│  DESIGN SPEC                │ custom │ image │ art │
├─────────────────────────────┼────────┼───────┼─────┤
│  qr-code-with-logo          │   ✓    │   ✓   │  ✗  │
│  qr-code-with-text          │   ✓    │   ✓   │  ✗  │
│  qr-code-color              │   ✓    │   ✓   │  ✓  │
│  qr-code-shapes             │   ✓    │   ✗   │  ✗  │
│  qr-code-transparent-bg     │   ✓    │   ✗   │  ✗  │
│  qr-code-background         │   ✓    │   ✗   │  ✗  │
│  qr-code-background-color   │   ✓    │   ✗   │  ✗  │
│  qr-code-background-gradient│   ✓    │   ✗   │  ✗  │
│  qr-code-background-image   │   ✓    │   ✗   │  ✗  │
└─────────────────────────────┴────────┴───────┴─────┘
```

| Key | Type | Parent |
|-----|------|--------|
| `qr-code-with-logo` | THING | custom-qr-code, qr-code-image |
| `qr-code-with-text` | THING | custom-qr-code, qr-code-image |
| `qr-code-color` | THING | custom-qr-code, qr-code-image, qr-code-art |
| `qr-code-shapes` | THING | custom-qr-code |
| `qr-code-transparent-background` | THING | custom-qr-code |
| `qr-code-background` | THING | custom-qr-code (category) |
| `qr-code-background-color` | THING | qr-code-background |
| `qr-code-background-gradient` | THING | qr-code-background |
| `qr-code-background-image` | THING | qr-code-background |

### 1.6 Concepts (4) — Behaviors

| Key | Type | Description |
|-----|------|-------------|
| `dynamic-qr-code` | CONCEPT | Uses Short Link, trackable, editable after print |
| `static-qr-code` | CONCEPT | Data encoded directly, not trackable or editable |
| `qr-code-light-mode` | CONCEPT | Light version (bg clair, fg foncé) |
| `qr-code-dark-mode` | CONCEPT | Dark version (bg foncé, fg clair) |

### 1.7 QR Code Frames (6)

| Key | Type | MEDIUM connection |
|-----|------|-------------------|
| `qr-code-business-card` | THING | business-cards |
| `qr-code-email-signature` | THING | emails |
| `qr-code-flyer` | THING | flyers |
| `qr-code-poster` | THING | posters-billboards |
| `qr-code-table-tent` | THING | table-tents |
| `qr-code-packaging-label` | THING | product-packaging |

### 1.8 Landing Page Types (6)

| Key | Type | Description |
|-----|------|-------------|
| `link-in-bio` | THING | Social media bio page |
| `menu-restaurant` | THING | Digital restaurant menu |
| `forms` | THING | Contact/lead forms |
| `announcement` | THING | Message/announcement page |
| `event-rsvp` | THING | Event RSVP page |
| `booking-appointment` | THING | Booking/scheduling page |

### 1.9 Use Cases (from design) (2)

| Key | Type | Applies to |
|-----|------|------------|
| `funny-qr-codes` | USE_CASE | All 3 styles |
| `qr-code-tattoo` | USE_CASE | All 3 styles |

---

## Phase 1 SEMANTIC_LINK Connections

```yaml
# ═══════════════════════════════════════════════════════════════
# PILLARS
# ═══════════════════════════════════════════════════════════════

qr-code:
  outgoing:
    - [includes, 0.70] -> qr-code-style
    - [includes, 0.70] -> qr-code-content
    - [includes, 0.70] -> qr-code-frame
    - [includes, 0.70] -> dynamic-qr-code
    - [includes, 0.70] -> static-qr-code
    - [related_to, 0.40] -> barcode
    - [related_to, 0.50] -> smart-link

smart-link:
  outgoing:
    - [type_of, 0.95] -> short-link
    - [requires, 0.90] -> short-link
    - [enables, 0.90] -> analytics
    - [enables, 0.90] -> tracking
    - [enables, 0.85] -> contextual-routing
    - [related_to, 0.50] -> qr-code
    - [related_to, 0.50] -> landing-page

barcode:
  outgoing:
    - [includes, 0.70] -> barcode-format
    - [exhibits, 0.90] -> static-qr-code
    - [related_to, 0.40] -> qr-code
    - [contrasts, 0.25] -> qr-code

landing-page:
  outgoing:
    - [includes, 0.70] -> landing-page-type
    - [related_to, 0.50] -> smart-link
    - [related_to, 0.40] -> qr-code

short-link:
  outgoing:
    - [enables, 0.90] -> analytics
    - [enables, 0.90] -> tracking
    - [enables, 0.85] -> contextual-routing
    - [enables, 0.80] -> custom-domain-name

# ═══════════════════════════════════════════════════════════════
# CATEGORIES
# ═══════════════════════════════════════════════════════════════

qr-code-style:
  outgoing:
    - [part_of, 0.80] -> qr-code
    - [includes, 0.70] -> custom-qr-code
    - [includes, 0.70] -> qr-code-image
    - [includes, 0.70] -> qr-code-art

qr-code-frame:
  outgoing:
    - [part_of, 0.80] -> qr-code
    - [includes, 0.70] -> qr-code-business-card
    - [includes, 0.70] -> qr-code-email-signature
    - [includes, 0.70] -> qr-code-flyer
    - [includes, 0.70] -> qr-code-poster
    - [includes, 0.70] -> qr-code-table-tent
    - [includes, 0.70] -> qr-code-packaging-label

landing-page-type:
  outgoing:
    - [part_of, 0.80] -> landing-page
    - [includes, 0.70] -> link-in-bio
    - [includes, 0.70] -> menu-restaurant
    - [includes, 0.70] -> forms
    - [includes, 0.70] -> announcement
    - [includes, 0.70] -> event-rsvp
    - [includes, 0.70] -> booking-appointment

# ═══════════════════════════════════════════════════════════════
# STYLES (with skip connections + light/dark)
# ═══════════════════════════════════════════════════════════════

custom-qr-code:
  outgoing:
    - [type_of, 0.95] -> qr-code-style
    - [variant_of, 0.85] -> qr-code           # skip
    - [exhibits, 0.70] -> qr-code-light-mode
    - [exhibits, 0.70] -> qr-code-dark-mode
    - [contrasts, 0.25] -> qr-code-image
    - [contrasts, 0.25] -> qr-code-art
    - [includes, 0.70] -> qr-code-with-logo
    - [includes, 0.70] -> qr-code-with-text
    - [includes, 0.70] -> qr-code-color
    - [includes, 0.70] -> qr-code-shapes
    - [includes, 0.70] -> qr-code-transparent-background
    - [includes, 0.70] -> qr-code-background

qr-code-image:
  outgoing:
    - [type_of, 0.95] -> qr-code-style
    - [variant_of, 0.85] -> qr-code           # skip
    - [exhibits, 0.70] -> qr-code-light-mode
    - [exhibits, 0.70] -> qr-code-dark-mode
    - [contrasts, 0.25] -> custom-qr-code
    - [contrasts, 0.25] -> qr-code-art
    - [includes, 0.70] -> qr-code-with-logo
    - [includes, 0.70] -> qr-code-with-text
    - [includes, 0.70] -> qr-code-color

qr-code-photo:
  outgoing:
    - [same_as, 1.00] -> qr-code-image        # alias

qr-code-art:
  outgoing:
    - [type_of, 0.95] -> qr-code-style
    - [variant_of, 0.85] -> qr-code           # skip
    - [exhibits, 0.70] -> qr-code-light-mode
    - [exhibits, 0.70] -> qr-code-dark-mode
    - [contrasts, 0.25] -> custom-qr-code
    - [contrasts, 0.25] -> qr-code-image
    - [includes, 0.70] -> qr-code-color

# ═══════════════════════════════════════════════════════════════
# CONCEPTS (light/dark + static/dynamic)
# ═══════════════════════════════════════════════════════════════

qr-code-light-mode:
  outgoing:
    - [contrasts, 0.25] -> qr-code-dark-mode

qr-code-dark-mode:
  outgoing:
    - [contrasts, 0.25] -> qr-code-light-mode

dynamic-qr-code:
  outgoing:
    - [requires, 0.90] -> short-link
    - [contrasts, 0.25] -> static-qr-code

static-qr-code:
  outgoing:
    - [contrasts, 0.25] -> dynamic-qr-code

# ═══════════════════════════════════════════════════════════════
# FRAMES (with MEDIUM connections)
# ═══════════════════════════════════════════════════════════════

qr-code-business-card:
  outgoing:
    - [type_of, 0.95] -> qr-code-frame
    - [part_of, 0.80] -> qr-code              # skip
    - [used_for, 0.85] -> business-cards      # MEDIUM
    - [related_to, 0.70] -> qr-code-vcard

qr-code-poster:
  outgoing:
    - [type_of, 0.95] -> qr-code-frame
    - [part_of, 0.80] -> qr-code              # skip
    - [used_for, 0.85] -> posters-billboards  # MEDIUM

qr-code-flyer:
  outgoing:
    - [type_of, 0.95] -> qr-code-frame
    - [part_of, 0.80] -> qr-code              # skip
    - [used_for, 0.85] -> flyers              # MEDIUM

qr-code-email-signature:
  outgoing:
    - [type_of, 0.95] -> qr-code-frame
    - [part_of, 0.80] -> qr-code              # skip
    - [used_for, 0.85] -> emails              # MEDIUM

qr-code-table-tent:
  outgoing:
    - [type_of, 0.95] -> qr-code-frame
    - [part_of, 0.80] -> qr-code              # skip
    - [used_for, 0.85] -> table-tents         # MEDIUM

qr-code-packaging-label:
  outgoing:
    - [type_of, 0.95] -> qr-code-frame
    - [part_of, 0.80] -> qr-code              # skip
    - [used_for, 0.85] -> product-packaging   # MEDIUM

# ═══════════════════════════════════════════════════════════════
# USE CASES (funny, tattoo)
# ═══════════════════════════════════════════════════════════════

funny-qr-codes:
  outgoing:
    - [used_for, 0.85] -> custom-qr-code
    - [used_for, 0.85] -> qr-code-image
    - [used_for, 0.85] -> qr-code-art
    - [part_of, 0.70] -> qr-code

qr-code-tattoo:
  outgoing:
    - [used_for, 0.85] -> custom-qr-code
    - [used_for, 0.85] -> qr-code-image
    - [used_for, 0.85] -> qr-code-art
    - [part_of, 0.70] -> qr-code
```

---

## link_type Enum (13 types)

```yaml
link_type:
  enum:
    # Hierarchical
    - type_of       # IS-A relationship
    - variant_of    # Style/mode variant
    - includes      # Parent contains child
    # Dependency
    - requires      # Cannot function without
    - enables       # Makes possible
    # Behavioral
    - exhibits      # Shows behavior/characteristic
    - contrasts     # Opposite/alternative
    # Functional
    - is_action_on  # ACTION targets THING
    - used_for      # Tool/feature used for object
    - part_of       # Component of whole
    # Associative
    - related_to    # Generic weak link
    # Identity
    - same_as       # Alias (NEW)
```

---

## SEMANTIC_LINK System (v10.7)

### link_type Enum (11 types)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  LINK_TYPE CATEGORIES                                                        │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  HIERARCHICAL:                                                               │
│  • type_of      → Child IS A type of parent                                 │
│  • variant_of   → Style/mode variant (orthogonal dimension)                 │
│  • includes     → Parent includes child concept                             │
│                                                                              │
│  DEPENDENCY:                                                                 │
│  • requires     → Cannot function without                                   │
│  • enables      → Makes possible / unlocks capability                       │
│                                                                              │
│  BEHAVIORAL:                                                                 │
│  • exhibits     → Shows/demonstrates behavior pattern                       │
│  • contrasts    → Opposite/alternative (for disambiguation)                 │
│                                                                              │
│  FUNCTIONAL:                                                                 │
│  • is_action_on → ACTION targets THING                                      │
│  • used_for     → Tool/feature used for object                              │
│  • part_of      → Component of larger whole                                 │
│                                                                              │
│  ASSOCIATIVE:                                                                │
│  • related_to   → Generic weak association                                  │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Temperature Calibration

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  TEMPERATURE CALIBRATION                                                     │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Range      │ Strength │ Link Types                                          │
│  ──────────────────────────────────────────────────────────────────────────  │
│  0.90-1.00  │ STRONG   │ type_of, is_action_on, used_for                    │
│  0.80-0.90  │ MEDIUM+  │ variant_of, requires, enables, exhibits            │
│  0.50-0.70  │ MEDIUM   │ includes, part_of                                  │
│  0.30-0.50  │ WEAK     │ related_to                                         │
│  0.10-0.30  │ MINIMAL  │ contrasts                                          │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Complete SEMANTIC_LINK Map

### 1. Style Variants (variant_of)

```
custom-qr-code ──[variant_of, 0.85]──> qr-code
qr-code-image ──[variant_of, 0.85]──> qr-code
qr-code-art ──[variant_of, 0.85]──> qr-code
```

### 2. Behavior Concepts (type_of + contrasts)

```
dynamic-qr-code ──[type_of, 0.90]──> qr-code
static-qr-code ──[type_of, 0.90]──> qr-code
dynamic-qr-code ──[contrasts, 0.25]──> static-qr-code
```

### 3. Infrastructure (requires + enables + type_of)

```
smart-link ──[type_of, 0.95]──> short-link
dynamic-qr-code ──[requires, 0.90]──> short-link

short-link ──[enables, 0.90]──> analytics
short-link ──[enables, 0.90]──> tracking
short-link ──[enables, 0.85]──> contextual-routing
short-link ──[enables, 0.80]──> custom-link-preview
```

### 4. Content Types (type_of + exhibits)

```
# All CONTENT_TYPE type_of qr-code
qr-code-url ──[type_of, 0.95]──> qr-code
qr-code-wifi ──[type_of, 0.95]──> qr-code
qr-code-vcard ──[type_of, 0.95]──> qr-code
qr-code-pdf ──[type_of, 0.95]──> qr-code
qr-code-menu ──[type_of, 0.95]──> qr-code
qr-code-social ──[type_of, 0.95]──> qr-code
qr-code-event ──[type_of, 0.95]──> qr-code
qr-code-payment ──[type_of, 0.95]──> qr-code
qr-code-location ──[type_of, 0.95]──> qr-code
qr-code-mecard ──[type_of, 0.95]──> qr-code
qr-code-text ──[type_of, 0.95]──> qr-code
qr-code-email ──[type_of, 0.95]──> qr-code
qr-code-phone ──[type_of, 0.95]──> qr-code

# Dynamic content exhibits dynamic behavior
qr-code-url ──[exhibits, 0.85]──> dynamic-qr-code
qr-code-pdf ──[exhibits, 0.85]──> dynamic-qr-code
qr-code-menu ──[exhibits, 0.85]──> dynamic-qr-code
qr-code-social ──[exhibits, 0.85]──> dynamic-qr-code
qr-code-event ──[exhibits, 0.85]──> dynamic-qr-code
qr-code-payment ──[exhibits, 0.85]──> dynamic-qr-code
qr-code-location ──[exhibits, 0.85]──> dynamic-qr-code

# Static content exhibits static behavior
qr-code-wifi ──[exhibits, 0.85]──> static-qr-code
qr-code-vcard ──[exhibits, 0.85]──> static-qr-code
qr-code-mecard ──[exhibits, 0.85]──> static-qr-code
qr-code-text ──[exhibits, 0.85]──> static-qr-code
qr-code-email ──[exhibits, 0.85]──> static-qr-code
qr-code-phone ──[exhibits, 0.85]──> static-qr-code
```

### 5. Actions (is_action_on)

```
# QR Code actions
create-qr-code ──[is_action_on, 0.95]──> qr-code
scan-qr-code ──[is_action_on, 0.95]──> qr-code
customize-qr-code ──[is_action_on, 0.95]──> qr-code
download-qr-code ──[is_action_on, 0.95]──> qr-code
print-qr-code ──[is_action_on, 0.95]──> qr-code

# Customization sub-actions
add-logo ──[is_action_on, 0.90]──> custom-qr-code
change-colors ──[is_action_on, 0.90]──> custom-qr-code

# Link actions
shorten-url ──[is_action_on, 0.95]──> short-link
create-smart-link ──[is_action_on, 0.95]──> smart-link

# Other actions
create-barcode ──[is_action_on, 0.95]──> barcode
create-landing-page ──[is_action_on, 0.95]──> landing-page
track-scans ──[is_action_on, 0.90]──> analytics
```

### 6. Tools (used_for)

```
qr-code-generator ──[used_for, 0.95]──> qr-code
qr-code-scanner ──[used_for, 0.95]──> qr-code
barcode-generator ──[used_for, 0.95]──> barcode
barcode-scanner ──[used_for, 0.95]──> barcode
landing-page-builder ──[used_for, 0.95]──> landing-page
```

### 7. Components (part_of)

```
frame ──[part_of, 0.80]──> qr-code
template ──[part_of, 0.80]──> qr-code
logo ──[part_of, 0.85]──> custom-qr-code
```

### 8. Features (part_of Smart Link)

```
analytics ──[part_of, 0.85]──> smart-link
tracking ──[part_of, 0.85]──> smart-link
contextual-routing ──[part_of, 0.85]──> smart-link
custom-domain-name ──[part_of, 0.80]──> smart-link
custom-link-preview ──[part_of, 0.80]──> smart-link
utm-builder ──[part_of, 0.80]──> smart-link
```

### 9. Platform Features (related_to)

```
bulk-creation ──[related_to, 0.50]──> qr-code
bulk-creation ──[related_to, 0.50]──> smart-link
team-workspaces ──[related_to, 0.40]──> qr-code
team-workspaces ──[related_to, 0.40]──> smart-link
api ──[related_to, 0.40]──> qr-code
api ──[related_to, 0.40]──> smart-link
```

### 10. Cross-Domain (related_to)

```
qr-code ──[related_to, 0.40]──> barcode
smart-link ──[related_to, 0.50]──> landing-page
```

---

## Orthogonal Dimensions (QR Code)

### Dimension 1: Visual Style (3 options)

| Style | Semantic | Description |
|-------|----------|-------------|
| Custom QR Code | PARAMETRIC | User configures each element manually |
| QR Code Image | SUPERPOSITION | Image background + QR foreground (2 layers) |
| QR Code Art | FUSION | AI generates unified artwork with QR fused in |

### Dimension 2: Content Type (determines static/dynamic)

**Static (no Short Link, no tracking):**
- WiFi, vCard, meCard, Text, Email, Phone, SMS

**Dynamic (uses Short Link, trackable):**
- URL, PDF, Menu, Social, Event, Payment, Location

### Valid Combinations

Any style + Any content type is valid:
- QR Code Art + WiFi = Static, AI-styled, no tracking
- Custom QR Code + URL = Dynamic, branded, trackable
- QR Code Image + vCard = Static, photo-styled, no tracking

---

## Entity Property Templates

### THING (Product/Object)

```yaml
key: "qr-code"
display_name: "QR Code"
type: THING
is_pillar: true
description: "2D matrix barcode that encodes data scannable by smartphones"
llm_context: |
  USE: When discussing QR code creation, scanning, or customization.
  TRIGGERS: generate, create, scan, design, customize, dynamic, static.
  NOT: Barcode (1D), Smart Link (different product).
wikidata_id: "Q1052379"
```

### ACTION

```yaml
key: "create-qr-code"
display_name: "Create QR Code"
type: ACTION
description: "Generate a QR code from URL, text, WiFi credentials, or other data"
llm_context: |
  USE: When user wants to generate a new QR code.
  TRIGGERS: create, generate, make, new, build.
  NOT: Scan (reading), customize (modifying existing).
```

### CONCEPT

```yaml
key: "dynamic-qr-code"
display_name: "Dynamic QR Code"
type: CONCEPT
description: "QR code whose destination can be modified after creation without reprinting"
llm_context: |
  USE: When explaining editable QR codes with tracking.
  TRIGGERS: editable, modifiable, trackable, analytics, change destination.
  NOT: Static QR (opposite concept).
```

---

## Implementation Checklist

- [x] Entity schema v10.7 documented
- [x] EntityL10n schema updated with new properties
- [x] EntityL10n ANSWERS relation to SEOQuestion added
- [x] Type enum expanded (10 types)
- [x] SEMANTIC_LINK link_type enum expanded (11 types)
- [x] SEMANTIC_LINK temperatures calibrated
- [x] 52 core entities defined (was ~35 MVP)
- [x] SEO schema architecture documented
- [x] Complete SEMANTIC_LINK map documented (all 52 entities)
- [ ] Create Project node for QR Code AI
- [ ] Implement entities in Neo4j
- [ ] Create all SEMANTIC_LINK arcs in Neo4j
- [ ] Generate EntityL10n for priority locales
- [ ] Mine SEO keywords for fr-FR locale
- [ ] Create SEOQuestion/SEOComparison/SEOPreposition nodes from ATP

---

## Phase 2: Content Types (CONTENT_TYPE) — ~58 Entities

### Hierarchy Overview

```
qr-code-content (category)
    │
    ├── BASIC TYPES (13)
    │   ├── qr-code-url (dynamic)
    │   ├── qr-code-wifi (static)
    │   ├── qr-code-vcard (static)
    │   ├── qr-code-mecard (static)
    │   ├── qr-code-pdf (dynamic)
    │   ├── qr-code-text (static)
    │   ├── qr-code-email (dynamic)
    │   ├── qr-code-sms (dynamic)
    │   ├── qr-code-phone (static)
    │   ├── qr-code-video (dynamic)
    │   ├── qr-code-audio (dynamic)
    │   ├── qr-code-image-gallery (dynamic)
    │   └── qr-code-coupon (dynamic)
    │
    ├── SOCIAL (category) (15)
    │   ├── qr-code-social (category)
    │   ├── qr-code-instagram
    │   ├── qr-code-linkedin
    │   ├── qr-code-facebook
    │   ├── qr-code-twitter / qr-code-x
    │   ├── qr-code-youtube
    │   ├── qr-code-tiktok
    │   ├── qr-code-snapchat
    │   ├── qr-code-whatsapp
    │   ├── qr-code-telegram
    │   ├── qr-code-pinterest
    │   ├── qr-code-spotify
    │   ├── qr-code-apple-music
    │   └── qr-code-soundcloud
    │
    ├── PAYMENT (category) (9)
    │   ├── qr-code-payment (category)
    │   ├── qr-code-pix (Brazil)
    │   ├── qr-code-upi (India)
    │   ├── qr-code-paypal
    │   ├── qr-code-venmo
    │   ├── qr-code-bitcoin
    │   ├── qr-code-ethereum
    │   ├── qr-code-crypto
    │   └── qr-code-bank-transfer
    │
    ├── LOCATION (category) (6)
    │   ├── qr-code-location (category)
    │   ├── qr-code-google-maps
    │   ├── qr-code-apple-maps
    │   ├── qr-code-waze
    │   └── qr-code-coordinates
    │
    ├── APP (category) (5)
    │   ├── qr-code-app (category)
    │   ├── qr-code-app-store
    │   ├── qr-code-play-store
    │   └── qr-code-app-download
    │
    └── SPECIALIZED (10)
        ├── qr-code-review (Google, TripAdvisor)
        ├── qr-code-survey
        ├── qr-code-feedback
        ├── qr-code-menu (restaurant)
        ├── qr-code-resume / qr-code-cv
        ├── qr-code-certificate
        ├── qr-code-ticket
        ├── qr-code-attendance
        ├── qr-code-pet-tag
        └── qr-code-medical-id
```

### 2.1 Category Node

| Key | Type | Parent | Description |
|-----|------|--------|-------------|
| `qr-code-content` | THING | qr-code | Category for all content types |

### 2.2 Basic Content Types (13)

| Key | Type | Behavior | Description |
|-----|------|----------|-------------|
| `qr-code-url` | CONTENT_TYPE | dynamic | Redirects to any URL |
| `qr-code-wifi` | CONTENT_TYPE | static | WiFi credentials (SSID, password, encryption) |
| `qr-code-vcard` | CONTENT_TYPE | static | Digital business card (vCard 3.0/4.0) |
| `qr-code-mecard` | CONTENT_TYPE | static | Japanese contact format (MeCard) |
| `qr-code-pdf` | CONTENT_TYPE | dynamic | Links to PDF document |
| `qr-code-text` | CONTENT_TYPE | static | Plain text content |
| `qr-code-email` | CONTENT_TYPE | dynamic | Opens email composer with prefilled fields |
| `qr-code-sms` | CONTENT_TYPE | dynamic | Opens SMS with prefilled message |
| `qr-code-phone` | CONTENT_TYPE | static | Phone number for direct call |
| `qr-code-video` | CONTENT_TYPE | dynamic | Links to video content |
| `qr-code-audio` | CONTENT_TYPE | dynamic | Links to audio/podcast |
| `qr-code-image-gallery` | CONTENT_TYPE | dynamic | Image gallery/album |
| `qr-code-coupon` | CONTENT_TYPE | dynamic | Digital coupon/discount code |

### 2.3 Social Category (15)

| Key | Type | Parent | Platform |
|-----|------|--------|----------|
| `qr-code-social` | THING | qr-code-content | Category for social media |
| `qr-code-instagram` | CONTENT_TYPE | qr-code-social | Instagram profile/post |
| `qr-code-linkedin` | CONTENT_TYPE | qr-code-social | LinkedIn profile/company |
| `qr-code-facebook` | CONTENT_TYPE | qr-code-social | Facebook page/profile |
| `qr-code-twitter` | CONTENT_TYPE | qr-code-social | Twitter/X profile |
| `qr-code-x` | CONTENT_TYPE | - | Alias (same_as qr-code-twitter) |
| `qr-code-youtube` | CONTENT_TYPE | qr-code-social | YouTube channel/video |
| `qr-code-tiktok` | CONTENT_TYPE | qr-code-social | TikTok profile |
| `qr-code-snapchat` | CONTENT_TYPE | qr-code-social | Snapchat profile |
| `qr-code-whatsapp` | CONTENT_TYPE | qr-code-social | WhatsApp chat link |
| `qr-code-telegram` | CONTENT_TYPE | qr-code-social | Telegram channel/chat |
| `qr-code-pinterest` | CONTENT_TYPE | qr-code-social | Pinterest profile/board |
| `qr-code-spotify` | CONTENT_TYPE | qr-code-social | Spotify artist/playlist |
| `qr-code-apple-music` | CONTENT_TYPE | qr-code-social | Apple Music artist/playlist |
| `qr-code-soundcloud` | CONTENT_TYPE | qr-code-social | SoundCloud artist/track |

### 2.4 Payment Category (9)

| Key | Type | Parent | Region/Type |
|-----|------|--------|-------------|
| `qr-code-payment` | THING | qr-code-content | Category for payment types |
| `qr-code-pix` | CONTENT_TYPE | qr-code-payment | Brazil instant payment |
| `qr-code-upi` | CONTENT_TYPE | qr-code-payment | India unified payment |
| `qr-code-paypal` | CONTENT_TYPE | qr-code-payment | PayPal payment link |
| `qr-code-venmo` | CONTENT_TYPE | qr-code-payment | Venmo payment (US) |
| `qr-code-bitcoin` | CONTENT_TYPE | qr-code-payment | Bitcoin wallet address |
| `qr-code-ethereum` | CONTENT_TYPE | qr-code-payment | Ethereum wallet address |
| `qr-code-crypto` | CONTENT_TYPE | qr-code-payment | Generic crypto payment |
| `qr-code-bank-transfer` | CONTENT_TYPE | qr-code-payment | Bank/SEPA transfer |

### 2.5 Location Category (5)

| Key | Type | Parent | Service |
|-----|------|--------|---------|
| `qr-code-location` | THING | qr-code-content | Category for location types |
| `qr-code-google-maps` | CONTENT_TYPE | qr-code-location | Google Maps link |
| `qr-code-apple-maps` | CONTENT_TYPE | qr-code-location | Apple Maps link |
| `qr-code-waze` | CONTENT_TYPE | qr-code-location | Waze navigation |
| `qr-code-coordinates` | CONTENT_TYPE | qr-code-location | Raw GPS coordinates |

### 2.6 App Category (4)

| Key | Type | Parent | Platform |
|-----|------|--------|----------|
| `qr-code-app` | THING | qr-code-content | Category for app downloads |
| `qr-code-app-store` | CONTENT_TYPE | qr-code-app | iOS App Store |
| `qr-code-play-store` | CONTENT_TYPE | qr-code-app | Google Play Store |
| `qr-code-app-download` | CONTENT_TYPE | qr-code-app | Smart redirect (auto-detect OS) |

### 2.7 Specialized Content Types (12)

| Key | Type | Behavior | Description |
|-----|------|----------|-------------|
| `qr-code-review` | CONTENT_TYPE | dynamic | Google/TripAdvisor review link |
| `qr-code-survey` | CONTENT_TYPE | dynamic | Survey/form link |
| `qr-code-feedback` | CONTENT_TYPE | dynamic | Feedback collection |
| `qr-code-menu` | CONTENT_TYPE | dynamic | Restaurant digital menu |
| `qr-code-resume` | CONTENT_TYPE | dynamic | Digital resume/CV |
| `qr-code-cv` | CONTENT_TYPE | - | Alias (same_as qr-code-resume) |
| `qr-code-certificate` | CONTENT_TYPE | dynamic | Digital certificate verification |
| `qr-code-ticket` | CONTENT_TYPE | dynamic | Event ticket |
| `qr-code-attendance` | CONTENT_TYPE | dynamic | Attendance tracking |
| `qr-code-pet-tag` | CONTENT_TYPE | dynamic | Pet identification |
| `qr-code-medical-id` | CONTENT_TYPE | dynamic | Medical info emergency access |
| `qr-code-file` | CONTENT_TYPE | dynamic | Generic file download |

---

## Phase 2 SEMANTIC_LINK Connections

```yaml
# ═══════════════════════════════════════════════════════════════
# CONTENT CATEGORY
# ═══════════════════════════════════════════════════════════════

qr-code-content:
  outgoing:
    - [part_of, 0.80] -> qr-code
    - [includes, 0.70] -> qr-code-url
    - [includes, 0.70] -> qr-code-wifi
    - [includes, 0.70] -> qr-code-vcard
    - [includes, 0.70] -> qr-code-mecard
    - [includes, 0.70] -> qr-code-pdf
    - [includes, 0.70] -> qr-code-text
    - [includes, 0.70] -> qr-code-email
    - [includes, 0.70] -> qr-code-sms
    - [includes, 0.70] -> qr-code-phone
    - [includes, 0.70] -> qr-code-video
    - [includes, 0.70] -> qr-code-audio
    - [includes, 0.70] -> qr-code-image-gallery
    - [includes, 0.70] -> qr-code-coupon
    - [includes, 0.70] -> qr-code-social
    - [includes, 0.70] -> qr-code-payment
    - [includes, 0.70] -> qr-code-location
    - [includes, 0.70] -> qr-code-app
    - [includes, 0.70] -> qr-code-review
    - [includes, 0.70] -> qr-code-survey
    - [includes, 0.70] -> qr-code-feedback
    - [includes, 0.70] -> qr-code-menu
    - [includes, 0.70] -> qr-code-resume
    - [includes, 0.70] -> qr-code-certificate
    - [includes, 0.70] -> qr-code-ticket
    - [includes, 0.70] -> qr-code-attendance
    - [includes, 0.70] -> qr-code-pet-tag
    - [includes, 0.70] -> qr-code-medical-id
    - [includes, 0.70] -> qr-code-file

# ═══════════════════════════════════════════════════════════════
# BASIC TYPES + BEHAVIOR
# ═══════════════════════════════════════════════════════════════

qr-code-url:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code           # skip
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link

qr-code-wifi:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code           # skip
    - [exhibits, 0.85] -> static-qr-code

qr-code-vcard:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code           # skip
    - [exhibits, 0.85] -> static-qr-code
    - [related_to, 0.70] -> qr-code-business-card
    - [contrasts, 0.25] -> qr-code-mecard

qr-code-mecard:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code           # skip
    - [exhibits, 0.85] -> static-qr-code
    - [contrasts, 0.25] -> qr-code-vcard

qr-code-pdf:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link

qr-code-text:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> static-qr-code

qr-code-email:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link

qr-code-sms:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link

qr-code-phone:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> static-qr-code

qr-code-video:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.60] -> qr-code-youtube

qr-code-audio:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.60] -> qr-code-spotify

qr-code-image-gallery:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link

qr-code-coupon:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [used_for, 0.85] -> retail

# ═══════════════════════════════════════════════════════════════
# SOCIAL CATEGORY
# ═══════════════════════════════════════════════════════════════

qr-code-social:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [includes, 0.70] -> qr-code-instagram
    - [includes, 0.70] -> qr-code-linkedin
    - [includes, 0.70] -> qr-code-facebook
    - [includes, 0.70] -> qr-code-twitter
    - [includes, 0.70] -> qr-code-youtube
    - [includes, 0.70] -> qr-code-tiktok
    - [includes, 0.70] -> qr-code-snapchat
    - [includes, 0.70] -> qr-code-whatsapp
    - [includes, 0.70] -> qr-code-telegram
    - [includes, 0.70] -> qr-code-pinterest
    - [includes, 0.70] -> qr-code-spotify
    - [includes, 0.70] -> qr-code-apple-music
    - [includes, 0.70] -> qr-code-soundcloud

qr-code-instagram:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code           # skip
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> instagram      # BRAND

qr-code-linkedin:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> linkedin       # BRAND

qr-code-facebook:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> facebook       # BRAND

qr-code-twitter:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> twitter        # BRAND

qr-code-x:
  outgoing:
    - [same_as, 1.00] -> qr-code-twitter   # alias

qr-code-youtube:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> youtube        # BRAND
    - [related_to, 0.60] -> qr-code-video

qr-code-tiktok:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> tiktok         # BRAND

qr-code-snapchat:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> snapchat       # BRAND

qr-code-whatsapp:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> whatsapp       # BRAND

qr-code-telegram:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> telegram       # BRAND

qr-code-pinterest:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> pinterest      # BRAND

qr-code-spotify:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> spotify        # BRAND
    - [related_to, 0.60] -> qr-code-audio

qr-code-apple-music:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> apple          # BRAND
    - [related_to, 0.60] -> qr-code-audio

qr-code-soundcloud:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> soundcloud     # BRAND

# ═══════════════════════════════════════════════════════════════
# PAYMENT CATEGORY
# ═══════════════════════════════════════════════════════════════

qr-code-payment:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [includes, 0.70] -> qr-code-pix
    - [includes, 0.70] -> qr-code-upi
    - [includes, 0.70] -> qr-code-paypal
    - [includes, 0.70] -> qr-code-venmo
    - [includes, 0.70] -> qr-code-bitcoin
    - [includes, 0.70] -> qr-code-ethereum
    - [includes, 0.70] -> qr-code-crypto
    - [includes, 0.70] -> qr-code-bank-transfer

qr-code-pix:
  outgoing:
    - [type_of, 0.95] -> qr-code-payment
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [used_for, 0.85] -> brazil           # region

qr-code-upi:
  outgoing:
    - [type_of, 0.95] -> qr-code-payment
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [used_for, 0.85] -> india            # region

qr-code-paypal:
  outgoing:
    - [type_of, 0.95] -> qr-code-payment
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [related_to, 0.80] -> paypal         # BRAND

qr-code-venmo:
  outgoing:
    - [type_of, 0.95] -> qr-code-payment
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [related_to, 0.80] -> venmo          # BRAND

qr-code-bitcoin:
  outgoing:
    - [type_of, 0.95] -> qr-code-payment
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> static-qr-code   # wallet address = static
    - [type_of, 0.85] -> qr-code-crypto

qr-code-ethereum:
  outgoing:
    - [type_of, 0.95] -> qr-code-payment
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> static-qr-code   # wallet address = static
    - [type_of, 0.85] -> qr-code-crypto

qr-code-crypto:
  outgoing:
    - [type_of, 0.95] -> qr-code-payment
    - [part_of, 0.80] -> qr-code
    - [includes, 0.70] -> qr-code-bitcoin
    - [includes, 0.70] -> qr-code-ethereum

qr-code-bank-transfer:
  outgoing:
    - [type_of, 0.95] -> qr-code-payment
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> static-qr-code   # IBAN = static

# ═══════════════════════════════════════════════════════════════
# LOCATION CATEGORY
# ═══════════════════════════════════════════════════════════════

qr-code-location:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [includes, 0.70] -> qr-code-google-maps
    - [includes, 0.70] -> qr-code-apple-maps
    - [includes, 0.70] -> qr-code-waze
    - [includes, 0.70] -> qr-code-coordinates

qr-code-google-maps:
  outgoing:
    - [type_of, 0.95] -> qr-code-location
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> google         # BRAND

qr-code-apple-maps:
  outgoing:
    - [type_of, 0.95] -> qr-code-location
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> apple          # BRAND

qr-code-waze:
  outgoing:
    - [type_of, 0.95] -> qr-code-location
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> waze           # BRAND

qr-code-coordinates:
  outgoing:
    - [type_of, 0.95] -> qr-code-location
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> static-qr-code   # raw GPS = static

# ═══════════════════════════════════════════════════════════════
# APP CATEGORY
# ═══════════════════════════════════════════════════════════════

qr-code-app:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [includes, 0.70] -> qr-code-app-store
    - [includes, 0.70] -> qr-code-play-store
    - [includes, 0.70] -> qr-code-app-download

qr-code-app-store:
  outgoing:
    - [type_of, 0.95] -> qr-code-app
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> apple          # BRAND

qr-code-play-store:
  outgoing:
    - [type_of, 0.95] -> qr-code-app
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.80] -> google         # BRAND

qr-code-app-download:
  outgoing:
    - [type_of, 0.95] -> qr-code-app
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [enables, 0.85] -> contextual-routing  # auto-detect iOS/Android

# ═══════════════════════════════════════════════════════════════
# SPECIALIZED TYPES
# ═══════════════════════════════════════════════════════════════

qr-code-review:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [used_for, 0.85] -> restaurants
    - [used_for, 0.85] -> retail
    - [related_to, 0.70] -> google         # Google Reviews

qr-code-survey:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.60] -> qr-code-feedback

qr-code-feedback:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.60] -> qr-code-survey

qr-code-menu:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [used_for, 0.90] -> restaurants
    - [related_to, 0.70] -> menu-restaurant  # landing page type

qr-code-resume:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.60] -> qr-code-vcard
    - [related_to, 0.60] -> qr-code-linkedin

qr-code-cv:
  outgoing:
    - [same_as, 1.00] -> qr-code-resume    # alias

qr-code-certificate:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [used_for, 0.85] -> education

qr-code-ticket:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [used_for, 0.85] -> event-management

qr-code-attendance:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [used_for, 0.85] -> education
    - [used_for, 0.85] -> event-management

qr-code-pet-tag:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [related_to, 0.60] -> qr-code-vcard

qr-code-medical-id:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [used_for, 0.85] -> healthcare

qr-code-file:
  outgoing:
    - [type_of, 0.95] -> qr-code-content
    - [part_of, 0.80] -> qr-code
    - [exhibits, 0.85] -> dynamic-qr-code
    - [requires, 0.90] -> short-link
    - [includes, 0.70] -> qr-code-pdf
```

---

## Phase 2 Summary

| Category | Count | Static | Dynamic |
|----------|-------|--------|---------|
| Basic | 13 | 5 | 8 |
| Social | 15 | 0 | 14 (+1 alias) |
| Payment | 9 | 4 | 4 (+1 category) |
| Location | 5 | 1 | 3 (+1 category) |
| App | 4 | 0 | 3 (+1 category) |
| Specialized | 12 | 0 | 10 (+2 alias) |
| **Total** | **58** | **10** | **42** (+6 cat/alias) |

---

## Phase 3: Barcode Types (THING) — 18 Entities

### Hierarchy Overview

```
barcode (PILLAR)
    │
    ├──[includes]──> barcode-format (category)
    │                   │
    │                   ├── 1D LINEAR (10)
    │                   │   ├── ean-13
    │                   │   ├── ean-8
    │                   │   ├── upc-a
    │                   │   ├── upc-e
    │                   │   ├── code-128
    │                   │   ├── code-39
    │                   │   ├── itf-14
    │                   │   ├── codabar
    │                   │   ├── msi-plessey
    │                   │   └── gs1-128
    │                   │
    │                   └── 2D MATRIX (5)
    │                       ├── data-matrix
    │                       ├── pdf417
    │                       ├── aztec-code
    │                       ├── maxicode
    │                       └── gs1-datamatrix
    │
    └──[exhibits]──> static-qr-code (all barcodes are static)
```

### 3.1 Category Node

| Key | Type | Parent | Description |
|-----|------|--------|-------------|
| `barcode-format` | THING | barcode | Category for all barcode formats |

### 3.2 1D Linear Barcodes (10)

| Key | Type | Use Case | Description |
|-----|------|----------|-------------|
| `ean-13` | THING | Retail (global) | European Article Number, 13 digits |
| `ean-8` | THING | Retail (small items) | Compact EAN, 8 digits |
| `upc-a` | THING | Retail (US/Canada) | Universal Product Code, 12 digits |
| `upc-e` | THING | Retail (small items) | Compressed UPC, 6 digits |
| `code-128` | THING | Logistics | High-density alphanumeric |
| `code-39` | THING | Industrial | Full ASCII alphanumeric |
| `itf-14` | THING | Packaging | Interleaved 2 of 5, shipping cartons |
| `codabar` | THING | Libraries, blood banks | Numeric with special chars |
| `msi-plessey` | THING | Inventory | Check digit for inventory |
| `gs1-128` | THING | Supply chain | GS1 Application Identifiers |

### 3.3 2D Matrix Barcodes (5)

| Key | Type | Use Case | Description |
|-----|------|----------|-------------|
| `data-matrix` | THING | Electronics, pharma | Small items, high density |
| `pdf417` | THING | IDs, tickets | Stacked linear, large data |
| `aztec-code` | THING | Tickets, transport | No quiet zone needed |
| `maxicode` | THING | Shipping (UPS) | Fixed-size, high speed |
| `gs1-datamatrix` | THING | Healthcare, retail | GS1 standard with AIs |

### 3.4 TOOLS (2)

| Key | Type | Description |
|-----|------|-------------|
| `barcode-generator` | TOOL | Creates barcode images |
| `barcode-scanner` | TOOL | Reads barcode data |

---

## Phase 3 SEMANTIC_LINK Connections

```yaml
# ═══════════════════════════════════════════════════════════════
# BARCODE CATEGORY
# ═══════════════════════════════════════════════════════════════

barcode-format:
  outgoing:
    - [part_of, 0.80] -> barcode
    - [includes, 0.70] -> ean-13
    - [includes, 0.70] -> ean-8
    - [includes, 0.70] -> upc-a
    - [includes, 0.70] -> upc-e
    - [includes, 0.70] -> code-128
    - [includes, 0.70] -> code-39
    - [includes, 0.70] -> itf-14
    - [includes, 0.70] -> codabar
    - [includes, 0.70] -> msi-plessey
    - [includes, 0.70] -> gs1-128
    - [includes, 0.70] -> data-matrix
    - [includes, 0.70] -> pdf417
    - [includes, 0.70] -> aztec-code
    - [includes, 0.70] -> maxicode
    - [includes, 0.70] -> gs1-datamatrix

# ═══════════════════════════════════════════════════════════════
# 1D LINEAR BARCODES
# ═══════════════════════════════════════════════════════════════

ean-13:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode              # skip
    - [exhibits, 0.85] -> static-qr-code      # all barcodes static
    - [used_for, 0.85] -> retail
    - [contrasts, 0.25] -> upc-a              # regional alternatives

ean-8:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> retail
    - [variant_of, 0.85] -> ean-13            # compact version

upc-a:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> retail
    - [contrasts, 0.25] -> ean-13             # regional alternatives

upc-e:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> retail
    - [variant_of, 0.85] -> upc-a             # compact version

code-128:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> logistics
    - [related_to, 0.60] -> gs1-128

code-39:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> manufacturing

itf-14:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> logistics
    - [used_for, 0.85] -> product-packaging   # MEDIUM

codabar:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> healthcare          # blood banks

msi-plessey:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> retail              # inventory

gs1-128:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> logistics
    - [type_of, 0.85] -> code-128             # based on Code 128

# ═══════════════════════════════════════════════════════════════
# 2D MATRIX BARCODES
# ═══════════════════════════════════════════════════════════════

data-matrix:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> manufacturing
    - [used_for, 0.85] -> healthcare
    - [contrasts, 0.25] -> qr-code            # 2D alternatives

pdf417:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> government          # IDs, licenses
    - [used_for, 0.85] -> event-management    # tickets

aztec-code:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> transportation      # boarding passes
    - [contrasts, 0.25] -> qr-code

maxicode:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> logistics           # UPS

gs1-datamatrix:
  outgoing:
    - [type_of, 0.95] -> barcode-format
    - [part_of, 0.80] -> barcode
    - [exhibits, 0.85] -> static-qr-code
    - [used_for, 0.85] -> healthcare
    - [used_for, 0.85] -> retail
    - [type_of, 0.85] -> data-matrix          # GS1 version

# ═══════════════════════════════════════════════════════════════
# BARCODE TOOLS
# ═══════════════════════════════════════════════════════════════

barcode-generator:
  outgoing:
    - [used_for, 0.95] -> barcode
    - [used_for, 0.85] -> ean-13
    - [used_for, 0.85] -> upc-a
    - [used_for, 0.85] -> code-128
    - [used_for, 0.85] -> data-matrix
    - [is_action_on, 0.90] -> create-barcode

barcode-scanner:
  outgoing:
    - [used_for, 0.95] -> barcode
    - [is_action_on, 0.90] -> scan-barcode
```

---

## Phase 3 Summary

| Category | Count | Description |
|----------|-------|-------------|
| 1D Linear | 10 | Traditional retail/logistics barcodes |
| 2D Matrix | 5 | High-density 2D codes |
| Tools | 2 | Generator + Scanner |
| **Total** | **17** (+1 category) |

---

## Phase 4: Features & Tools (FEATURE/TOOL) — 25 Entities

### 4.1 Platform Features (FEATURE) — 15

| Key | Type | Parent | Description |
|-----|------|--------|-------------|
| `analytics` | FEATURE | smart-link | Scan/click tracking, stats |
| `tracking` | FEATURE | smart-link | Visitor tracking, geolocation |
| `contextual-routing` | FEATURE | smart-link | OS/device/location redirect |
| `custom-domain-name` | FEATURE | smart-link | Branded short domains |
| `custom-link-preview` | FEATURE | smart-link | OG meta customization |
| `utm-builder` | FEATURE | smart-link | Campaign parameter builder |
| `bulk-creation` | FEATURE | qr-code | Mass QR code generation |
| `team-workspaces` | FEATURE | - | Multi-user collaboration |
| `api` | FEATURE | - | Developer API access |
| `webhooks` | FEATURE | - | Event notifications |
| `white-label` | FEATURE | - | Remove branding |
| `password-protection` | FEATURE | qr-code | Require password to access |
| `expiration` | FEATURE | qr-code | Time-limited QR codes |
| `scan-limit` | FEATURE | qr-code | Max scan count |
| `retargeting-pixel` | FEATURE | smart-link | Facebook/Google pixel |

### 4.2 Tools (TOOL) — 10

| Key | Type | Description |
|-----|------|-------------|
| `qr-code-generator` | TOOL | Primary QR code creation tool |
| `qr-code-scanner` | TOOL | QR code reader app |
| `qr-code-api` | TOOL | API for QR generation |
| `landing-page-builder` | TOOL | No-code page builder |
| `url-shortener` | TOOL | Link shortening tool |
| `link-in-bio-builder` | TOOL | Social bio page builder |
| `menu-builder` | TOOL | Restaurant menu builder |
| `vcard-generator` | TOOL | Digital business card creator |
| `wifi-qr-generator` | TOOL | WiFi credential QR creator |
| `batch-qr-generator` | TOOL | Bulk QR creation tool |

---

## Phase 4 SEMANTIC_LINK Connections

```yaml
# ═══════════════════════════════════════════════════════════════
# FEATURES (enabled by short-link or platform)
# ═══════════════════════════════════════════════════════════════

analytics:
  outgoing:
    - [part_of, 0.85] -> smart-link
    - [requires, 0.90] -> short-link

tracking:
  outgoing:
    - [part_of, 0.85] -> smart-link
    - [requires, 0.90] -> short-link
    - [related_to, 0.70] -> analytics

contextual-routing:
  outgoing:
    - [part_of, 0.85] -> smart-link
    - [requires, 0.90] -> short-link
    - [enables, 0.85] -> qr-code-app-download

custom-domain-name:
  outgoing:
    - [part_of, 0.80] -> smart-link
    - [part_of, 0.80] -> short-link

custom-link-preview:
  outgoing:
    - [part_of, 0.80] -> smart-link
    - [requires, 0.90] -> short-link

utm-builder:
  outgoing:
    - [part_of, 0.80] -> smart-link
    - [used_for, 0.85] -> analytics

bulk-creation:
  outgoing:
    - [related_to, 0.50] -> qr-code
    - [related_to, 0.50] -> smart-link
    - [used_for, 0.85] -> enterprise

team-workspaces:
  outgoing:
    - [related_to, 0.40] -> qr-code
    - [related_to, 0.40] -> smart-link
    - [used_for, 0.85] -> enterprise

api:
  outgoing:
    - [related_to, 0.40] -> qr-code
    - [related_to, 0.40] -> smart-link
    - [used_for, 0.85] -> developers

webhooks:
  outgoing:
    - [related_to, 0.50] -> api
    - [used_for, 0.85] -> developers

white-label:
  outgoing:
    - [used_for, 0.85] -> enterprise
    - [used_for, 0.85] -> agencies

password-protection:
  outgoing:
    - [part_of, 0.70] -> dynamic-qr-code
    - [requires, 0.90] -> short-link

expiration:
  outgoing:
    - [part_of, 0.70] -> dynamic-qr-code
    - [requires, 0.90] -> short-link

scan-limit:
  outgoing:
    - [part_of, 0.70] -> dynamic-qr-code
    - [requires, 0.90] -> short-link

retargeting-pixel:
  outgoing:
    - [part_of, 0.80] -> smart-link
    - [used_for, 0.85] -> marketing-agencies

# ═══════════════════════════════════════════════════════════════
# TOOLS
# ═══════════════════════════════════════════════════════════════

qr-code-generator:
  outgoing:
    - [used_for, 0.95] -> qr-code
    - [is_action_on, 0.90] -> create-qr-code

qr-code-scanner:
  outgoing:
    - [used_for, 0.95] -> qr-code
    - [is_action_on, 0.90] -> scan-qr-code

qr-code-api:
  outgoing:
    - [used_for, 0.90] -> qr-code
    - [type_of, 0.85] -> api
    - [used_for, 0.85] -> developers

landing-page-builder:
  outgoing:
    - [used_for, 0.95] -> landing-page
    - [is_action_on, 0.90] -> create-landing-page

url-shortener:
  outgoing:
    - [used_for, 0.95] -> short-link
    - [is_action_on, 0.90] -> shorten-url

link-in-bio-builder:
  outgoing:
    - [used_for, 0.95] -> link-in-bio
    - [type_of, 0.85] -> landing-page-builder

menu-builder:
  outgoing:
    - [used_for, 0.95] -> menu-restaurant
    - [type_of, 0.85] -> landing-page-builder
    - [used_for, 0.85] -> restaurants

vcard-generator:
  outgoing:
    - [used_for, 0.95] -> qr-code-vcard
    - [type_of, 0.85] -> qr-code-generator

wifi-qr-generator:
  outgoing:
    - [used_for, 0.95] -> qr-code-wifi
    - [type_of, 0.85] -> qr-code-generator

batch-qr-generator:
  outgoing:
    - [used_for, 0.95] -> bulk-creation
    - [type_of, 0.85] -> qr-code-generator
```

---

## Phase 5: Mediums (MEDIUM) — 20 Entities

### 5.1 Print Mediums

| Key | Type | Description |
|-----|------|-------------|
| `business-cards` | MEDIUM | Professional contact cards |
| `flyers` | MEDIUM | Promotional flyers |
| `posters-billboards` | MEDIUM | Large format displays |
| `brochures` | MEDIUM | Folded informational |
| `catalogs` | MEDIUM | Product catalogs |
| `magazines` | MEDIUM | Print publications |
| `newspapers` | MEDIUM | News publications |
| `direct-mail` | MEDIUM | Mailed marketing |
| `stickers-labels` | MEDIUM | Adhesive prints |
| `banners` | MEDIUM | Event/trade show banners |

### 5.2 Product Mediums

| Key | Type | Description |
|-----|------|-------------|
| `product-packaging` | MEDIUM | Product boxes/containers |
| `product-labels` | MEDIUM | Applied product labels |
| `receipts` | MEDIUM | Transaction receipts |
| `table-tents` | MEDIUM | Restaurant table displays |
| `menus-printed` | MEDIUM | Physical menus |
| `tickets-physical` | MEDIUM | Event tickets |

### 5.3 Digital Mediums

| Key | Type | Description |
|-----|------|-------------|
| `emails` | MEDIUM | Email signatures/campaigns |
| `presentations` | MEDIUM | Slides/decks |
| `documents` | MEDIUM | PDFs, reports |
| `websites` | MEDIUM | Web pages |

---

## Phase 5 SEMANTIC_LINK Connections

```yaml
# ═══════════════════════════════════════════════════════════════
# MEDIUMS → FRAMES (used_for connection)
# ═══════════════════════════════════════════════════════════════

business-cards:
  outgoing:
    - [related_to, 0.85] -> qr-code-business-card
    - [related_to, 0.70] -> qr-code-vcard

flyers:
  outgoing:
    - [related_to, 0.85] -> qr-code-flyer
    - [used_for, 0.80] -> marketing

posters-billboards:
  outgoing:
    - [related_to, 0.85] -> qr-code-poster
    - [used_for, 0.80] -> advertising

brochures:
  outgoing:
    - [used_for, 0.80] -> marketing
    - [related_to, 0.60] -> flyers

product-packaging:
  outgoing:
    - [related_to, 0.85] -> qr-code-packaging-label
    - [used_for, 0.80] -> retail
    - [used_for, 0.80] -> manufacturing

table-tents:
  outgoing:
    - [related_to, 0.85] -> qr-code-table-tent
    - [used_for, 0.90] -> restaurants

emails:
  outgoing:
    - [related_to, 0.85] -> qr-code-email-signature
    - [used_for, 0.80] -> marketing
```

---

## Phase 6: Actions (ACTION) — 15 Entities

| Key | Type | Target | Description |
|-----|------|--------|-------------|
| `create-qr-code` | ACTION | qr-code | Generate a new QR code |
| `scan-qr-code` | ACTION | qr-code | Read QR code data |
| `customize-qr-code` | ACTION | qr-code | Design/style a QR code |
| `download-qr-code` | ACTION | qr-code | Export QR as image |
| `print-qr-code` | ACTION | qr-code | Print for physical use |
| `track-scans` | ACTION | analytics | Monitor scan analytics |
| `shorten-url` | ACTION | short-link | Create short URL |
| `create-smart-link` | ACTION | smart-link | Create intelligent link |
| `add-logo` | ACTION | custom-qr-code | Add logo to QR center |
| `change-colors` | ACTION | custom-qr-code | Customize QR colors |
| `create-barcode` | ACTION | barcode | Generate barcode |
| `scan-barcode` | ACTION | barcode | Read barcode data |
| `create-landing-page` | ACTION | landing-page | Build destination page |
| `edit-destination` | ACTION | dynamic-qr-code | Change QR target URL |
| `share-qr-code` | ACTION | qr-code | Share via link/email |

---

## Phase 6 SEMANTIC_LINK Connections

```yaml
create-qr-code:
  outgoing:
    - [is_action_on, 0.95] -> qr-code
    - [enables, 0.85] -> customize-qr-code

scan-qr-code:
  outgoing:
    - [is_action_on, 0.95] -> qr-code
    - [contrasts, 0.25] -> create-qr-code

customize-qr-code:
  outgoing:
    - [is_action_on, 0.95] -> qr-code
    - [requires, 0.85] -> create-qr-code
    - [includes, 0.70] -> add-logo
    - [includes, 0.70] -> change-colors

download-qr-code:
  outgoing:
    - [is_action_on, 0.95] -> qr-code
    - [requires, 0.85] -> create-qr-code
    - [enables, 0.85] -> print-qr-code

print-qr-code:
  outgoing:
    - [is_action_on, 0.95] -> qr-code
    - [requires, 0.85] -> download-qr-code

track-scans:
  outgoing:
    - [is_action_on, 0.90] -> analytics
    - [requires, 0.90] -> dynamic-qr-code

shorten-url:
  outgoing:
    - [is_action_on, 0.95] -> short-link

create-smart-link:
  outgoing:
    - [is_action_on, 0.95] -> smart-link
    - [requires, 0.90] -> shorten-url

add-logo:
  outgoing:
    - [is_action_on, 0.90] -> custom-qr-code
    - [part_of, 0.80] -> customize-qr-code

change-colors:
  outgoing:
    - [is_action_on, 0.90] -> custom-qr-code
    - [part_of, 0.80] -> customize-qr-code

create-barcode:
  outgoing:
    - [is_action_on, 0.95] -> barcode

scan-barcode:
  outgoing:
    - [is_action_on, 0.95] -> barcode
    - [contrasts, 0.25] -> create-barcode

create-landing-page:
  outgoing:
    - [is_action_on, 0.95] -> landing-page

edit-destination:
  outgoing:
    - [is_action_on, 0.90] -> dynamic-qr-code
    - [requires, 0.90] -> dynamic-qr-code

share-qr-code:
  outgoing:
    - [is_action_on, 0.90] -> qr-code
    - [requires, 0.85] -> create-qr-code
```

---

## Phase 7: Industries (INDUSTRY) — 25 Entities

### 7.1 B2C Industries

| Key | Type | Description |
|-----|------|-------------|
| `restaurants` | INDUSTRY | Food service, cafes, bars |
| `retail` | INDUSTRY | Shops, stores, e-commerce |
| `hospitality` | INDUSTRY | Hotels, resorts, tourism |
| `healthcare` | INDUSTRY | Hospitals, clinics, pharma |
| `education` | INDUSTRY | Schools, universities, training |
| `real-estate` | INDUSTRY | Property sales, rentals |
| `fitness` | INDUSTRY | Gyms, sports, wellness |
| `beauty` | INDUSTRY | Salons, spas, cosmetics |
| `entertainment` | INDUSTRY | Movies, games, events |
| `transportation` | INDUSTRY | Airlines, transit, logistics |

### 7.2 B2B Industries

| Key | Type | Description |
|-----|------|-------------|
| `manufacturing` | INDUSTRY | Production, assembly |
| `logistics` | INDUSTRY | Shipping, warehousing |
| `construction` | INDUSTRY | Building, infrastructure |
| `finance` | INDUSTRY | Banking, insurance |
| `government` | INDUSTRY | Public sector, admin |

### 7.3 Service Industries

| Key | Type | Description |
|-----|------|-------------|
| `marketing-agencies` | INDUSTRY | Digital marketing firms |
| `creative-agencies` | INDUSTRY | Design, branding |
| `event-management` | INDUSTRY | Conferences, weddings |
| `nonprofits` | INDUSTRY | Charities, NGOs |
| `consulting` | INDUSTRY | Business consulting |

### 7.4 Roles (WHO)

| Key | Type | Description |
|-----|------|-------------|
| `developers` | INDUSTRY | API users, integrators |
| `enterprise` | INDUSTRY | Large organizations |
| `agencies` | INDUSTRY | Marketing/creative agencies |
| `small-business` | INDUSTRY | SMBs, local business |
| `freelancers` | INDUSTRY | Independent professionals |

---

## Phase 8: Brands (BRAND) — 20 Entities

### 8.1 Social Platforms

| Key | Type |
|-----|------|
| `instagram` | BRAND |
| `linkedin` | BRAND |
| `facebook` | BRAND |
| `twitter` | BRAND |
| `youtube` | BRAND |
| `tiktok` | BRAND |
| `snapchat` | BRAND |
| `whatsapp` | BRAND |
| `telegram` | BRAND |
| `pinterest` | BRAND |

### 8.2 Music Platforms

| Key | Type |
|-----|------|
| `spotify` | BRAND |
| `apple` | BRAND |
| `soundcloud` | BRAND |

### 8.3 Payment Brands

| Key | Type |
|-----|------|
| `paypal` | BRAND |
| `venmo` | BRAND |

### 8.4 Tech Brands

| Key | Type |
|-----|------|
| `google` | BRAND |
| `waze` | BRAND |

---

## Cross-Realm Geographic Targeting (No Entity Type)

**DECISION**: Geographic markets use EXISTING global nodes instead of Entity type.

NovaNet already has comprehensive geographic taxonomy in the global realm:
- **Continent** (6): africa, americas, asia, europe, oceania, antarctica
- **GeoRegion** (22): northern-africa, western-europe, eastern-asia, etc.
- **GeoSubRegion** (50+): maghreb, gulf-states, indian-subcontinent, brazil, etc.
- **CulturalRealm** (6): occidental, islamic, indic, east-asian, southeast-asian, meso-african

### Pattern: Entity → [:POPULAR_IN] → Global Geographic

```cypher
// Regional payment method linked to geographic subregion
(qrCodePix:Entity {key: 'qr-code-pix'})
  -[:POPULAR_IN {weight: 0.95}]->
(brazil:GeoSubRegion {key: 'brazil'})

// UPI payment linked to India
(qrCodeUpi:Entity {key: 'qr-code-upi'})
  -[:POPULAR_IN {weight: 0.95}]->
(india:GeoSubRegion {key: 'indian-subcontinent'})

// Twint linked to Western Europe
(qrCodeTwint:Entity {key: 'qr-code-twint'})
  -[:POPULAR_IN {weight: 0.85}]->
(westEurope:GeoRegion {key: 'western-europe'})
```

### Why Cross-Realm?

1. **No duplication** — Global geographic nodes exist, READ-ONLY
2. **Richer metadata** — GeoRegion/GeoSubRegion have population, income level, cultural realm
3. **Locale integration** — Geographic nodes already link to Locale nodes
4. **Query efficiency** — Single hop from Entity to geography to locale knowledge

### Available Global Geographic Nodes

| NodeKind | Count | Example Keys |
|----------|-------|--------------|
| Continent | 6 | africa, americas, asia, europe |
| GeoRegion | 22 | northern-africa, western-europe, south-eastern-asia |
| GeoSubRegion | 50+ | maghreb, gulf-states, brazil, indian-subcontinent |
| IncomeGroup | 4 | high-income, upper-middle, lower-middle, low-income |
| EconomicRegion | 7 | eurozone, nafta-usmca, mercosur, asean |
| CulturalRealm | 6 | occidental, islamic, indic, east-asian |

---

## Phase 9: Integrations (INTEGRATION) — 12 Entities

Third-party integrations that connect QR Code AI with external services.

**Link Pattern**: INTEGRATION → `[:related_to]` → BRAND (parent platform)

### 9.1 Automation Integrations

| Key | Type | Parent Brand | Description |
|-----|------|--------------|-------------|
| `zapier-integration` | INTEGRATION | zapier | Workflow automation |
| `make-integration` | INTEGRATION | make | Visual automation |
| `n8n-integration` | INTEGRATION | n8n | Open-source automation |

### 9.2 CRM Integrations

| Key | Type | Parent Brand | Description |
|-----|------|--------------|-------------|
| `hubspot-integration` | INTEGRATION | hubspot | CRM integration |
| `salesforce-integration` | INTEGRATION | salesforce | Enterprise CRM |
| `mailchimp-integration` | INTEGRATION | mailchimp | Email marketing |

### 9.3 Productivity Integrations

| Key | Type | Parent Brand | Description |
|-----|------|--------------|-------------|
| `google-sheets-integration` | INTEGRATION | google | Spreadsheet sync |
| `notion-integration` | INTEGRATION | notion | Workspace sync |
| `slack-integration` | INTEGRATION | slack | Team notifications |

### 9.4 E-commerce Integrations

| Key | Type | Parent Brand | Description |
|-----|------|--------------|-------------|
| `shopify-integration` | INTEGRATION | shopify | E-commerce QR codes |
| `woocommerce-integration` | INTEGRATION | woocommerce | WordPress e-commerce |
| `wordpress-integration` | INTEGRATION | wordpress | CMS integration |

### 9.5 SEMANTIC_LINK Connections

```yaml
zapier-integration:
  outgoing:
    - [related_to, 0.90] -> zapier          # BRAND
    - [related_to, 0.80] -> api             # FEATURE
    - [used_for, 0.85] -> bulk-creation

hubspot-integration:
  outgoing:
    - [related_to, 0.90] -> hubspot         # BRAND
    - [used_for, 0.85] -> marketing-agencies

google-sheets-integration:
  outgoing:
    - [related_to, 0.90] -> google          # BRAND
    - [used_for, 0.85] -> bulk-creation
    - [related_to, 0.70] -> batch-qr-generator
```

---

## Phase 10: Technical Concepts (CONCEPT) — 8 Entities

Educational concepts for technical QR code understanding.

### 10.1 QR Code Structure

| Key | Type | Description |
|-----|------|-------------|
| `quiet-zone` | CONCEPT | White margin around QR (minimum 4 modules) |
| `error-correction` | CONCEPT | Reed-Solomon encoding (L/M/Q/H levels) |
| `data-capacity` | CONCEPT | Max characters based on version + EC |
| `qr-code-version` | CONCEPT | Size grid (21x21 v1 to 177x177 v40) |

### 10.2 QR Code Encoding

| Key | Type | Description |
|-----|------|-------------|
| `encoding-mode` | CONCEPT | Numeric/Alphanumeric/Byte/Kanji |
| `module` | CONCEPT | Single black/white square unit |
| `finder-pattern` | CONCEPT | 3 corner squares for orientation |
| `timing-pattern` | CONCEPT | Alternating modules for alignment |

### 10.3 SEMANTIC_LINK Connections

```yaml
quiet-zone:
  outgoing:
    - [part_of, 0.80] -> qr-code
    - [related_to, 0.70] -> qr-code-print-guide

error-correction:
  outgoing:
    - [part_of, 0.80] -> qr-code
    - [enables, 0.85] -> qr-code-with-logo    # Higher EC allows logo
    - [related_to, 0.70] -> data-capacity

data-capacity:
  outgoing:
    - [part_of, 0.80] -> qr-code
    - [related_to, 0.70] -> qr-code-version
    - [related_to, 0.60] -> error-correction

qr-code-version:
  outgoing:
    - [part_of, 0.80] -> qr-code
    - [enables, 0.85] -> data-capacity
```

---

## Phase 11: Social Subcategories (THING) — 4 Entities

Semantic groupings within qr-code-social for better organization.

### 11.1 Social Subcategories

| Key | Type | Parent | Contains |
|-----|------|--------|----------|
| `qr-code-messaging` | THING | qr-code-social | whatsapp, telegram, sms |
| `qr-code-video` | THING | qr-code-social | youtube, tiktok, snapchat |
| `qr-code-professional` | THING | qr-code-social | linkedin |
| `qr-code-music` | THING | qr-code-social | spotify, apple-music, soundcloud |

### 11.2 SEMANTIC_LINK Connections

```yaml
qr-code-messaging:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [includes, 0.70] -> qr-code-whatsapp
    - [includes, 0.70] -> qr-code-telegram

qr-code-video:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [includes, 0.70] -> qr-code-youtube
    - [includes, 0.70] -> qr-code-tiktok
    - [includes, 0.70] -> qr-code-snapchat

qr-code-professional:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [includes, 0.70] -> qr-code-linkedin

qr-code-music:
  outgoing:
    - [type_of, 0.95] -> qr-code-social
    - [includes, 0.70] -> qr-code-spotify
    - [includes, 0.70] -> qr-code-apple-music
    - [includes, 0.70] -> qr-code-soundcloud
    - [related_to, 0.80] -> qr-code-audio
```

---

## Entity Count Summary

| Phase | Category | Count |
|-------|----------|-------|
| 1 | Core Products (THING) | 39 |
| 2 | Content Types (CONTENT_TYPE) | 58 |
| 3 | Barcode Types (THING) | 18 |
| 4 | Features & Tools (FEATURE/TOOL) | 25 |
| 5 | Mediums (MEDIUM) | 20 |
| 6 | Actions (ACTION) | 15 |
| 7 | Industries (INDUSTRY) | 25 |
| 8 | Brands (BRAND) | 25 |
| 9 | Integrations (INTEGRATION) | 12 |
| 10 | Technical Concepts (CONCEPT) | 8 |
| 11 | Social Subcategories (THING) | 4 |
| 12 | Use Cases (USE_CASE) | 12 |
| 13 | Guides (GUIDE) | 10 |
| 14 | Comparisons (COMPARISON) | 8 |
| **TOTAL** | | **~279** |

**Note**: Geographic targeting (regions, markets) uses cross-realm links to global GeoRegion/GeoSubRegion nodes instead of Entity type.

---

## Visual Graph Overview (ASCII)

```
╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
║  QR CODE AI — ENTITY GRAPH (~279 entities, 13 types, 13 link_types)                               ║
╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

                                    ┌─────────────────┐
                                    │   SHORT-LINK    │ ◄─────────────────────────────────────────┐
                                    │  (INFRA LAYER)  │                                           │
                                    └────────┬────────┘                                           │
                                             │                                                    │
              ┌──────────────────────────────┼──────────────────────────────┐                    │
              │ [enables]                    │ [type_of]                    │ [enables]          │
              ▼                              ▼                              ▼                    │
   ┌─────────────────────┐      ┌─────────────────────┐      ┌─────────────────────┐            │
   │      ANALYTICS      │      │     SMART-LINK      │      │  CONTEXTUAL-ROUTING │            │
   │      (FEATURE)      │      │      (PILLAR)       │      │      (FEATURE)      │            │
   └─────────────────────┘      └──────────┬──────────┘      └─────────────────────┘            │
                                           │                                                     │
                                    [related_to 0.50]                                            │
                                           │                                                     │
╔══════════════════════════════════════════╪════════════════════════════════════════════════════╗
║  4 PILLARS                               │                                                    ║
╠══════════════════════════════════════════╪════════════════════════════════════════════════════╣
║                                          │                                                    ║
║  ┌───────────────────┐          ┌────────┴────────┐          ┌───────────────────┐           ║
║  │      BARCODE      │◄─────────│     QR-CODE     │─────────►│   LANDING-PAGE    │           ║
║  │     (PILLAR)      │[rel 0.40]│    (PILLAR)     │[rel 0.40]│     (PILLAR)      │           ║
║  └─────────┬─────────┘          └────────┬────────┘          └─────────┬─────────┘           ║
║            │                             │                             │                      ║
║    [includes]                    [includes]                    [includes]                     ║
║            │                             │                             │                      ║
║            ▼                             ▼                             ▼                      ║
║  ┌─────────────────┐   ┌─────────────────────────────────┐   ┌─────────────────────┐         ║
║  │ BARCODE-FORMAT  │   │          QR-CODE-STYLE          │   │ LANDING-PAGE-TYPE   │         ║
║  │   (category)    │   │          (category)             │   │    (category)       │         ║
║  └────────┬────────┘   └───────────────┬─────────────────┘   └──────────┬──────────┘         ║
║           │                            │                                │                     ║
╚═══════════╪════════════════════════════╪════════════════════════════════╪═════════════════════╝
            │                            │                                │
            ▼                            ▼                                ▼
┌───────────────────────┐  ┌─────────────────────────────┐  ┌─────────────────────────┐
│  1D: ean-13, upc-a    │  │  custom-qr-code             │  │  link-in-bio            │
│      code-128, etc.   │  │  qr-code-image              │  │  menu-restaurant        │
│  2D: data-matrix      │  │  qr-code-art                │  │  forms, event-rsvp      │
│      pdf417, aztec    │  └──────────┬──────────────────┘  └─────────────────────────┘
└───────────────────────┘             │
                                      ▼
                          ┌───────────────────────┐
                          │    DESIGN SPECS       │
                          │  with-logo, color,    │
                          │  shapes, background   │
                          └───────────────────────┘


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  CONTENT TYPES (58) — QR-CODE-CONTENT category                                                 ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║       ┌──────────────┬──────────────┬───────────────┬──────────────┬──────────────┐           ║
║       │              │              │               │              │              │           ║
║       ▼              ▼              ▼               ▼              ▼              ▼           ║
║  ┌─────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐      ║
║  │  BASIC  │   │  SOCIAL  │   │ PAYMENT  │   │ LOCATION │   │   APP    │   │SPECIALIZED│     ║
║  │  (13)   │   │  (15)    │   │   (9)    │   │   (5)    │   │   (4)    │   │   (12)   │      ║
║  └────┬────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘   └────┬─────┘      ║
║       │             │              │              │              │              │             ║
║  url, wifi     instagram      pix (🇧🇷)     google-maps    app-store     review            ║
║  vcard, pdf    linkedin       upi (🇮🇳)     apple-maps     play-store    menu              ║
║  email, sms    facebook       paypal        waze           app-download  resume            ║
║  video, audio  youtube        bitcoin       coordinates                  ticket            ║
║                tiktok         ethereum                                   pet-tag           ║
║                spotify        crypto                                     medical-id        ║
║                               └──► MUSIC (subcategory)                                      ║
║                                    spotify, apple-music, soundcloud                         ║
║                                                                                                ║
║  STATIC:  wifi, vcard, mecard, text, phone, coordinates, bitcoin, ethereum, bank-transfer     ║
║  DYNAMIC: url, pdf, email, sms, video, audio, all social, pix, upi, paypal, maps, apps, etc.  ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  USE CASES (12) — WHY people use QR codes                                                      ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║  CREATIVE           EVENTS              PERSONAL            BUSINESS                          ║
║  ────────           ──────              ────────            ────────                          ║
║  funny-qr-codes     qr-code-wedding     qr-code-tattoo      qr-code-marketing                 ║
║  qr-code-art-piece  qr-code-event       qr-code-memorial    qr-code-inventory                 ║
║                     qr-code-conference  qr-code-gift        qr-code-authentication            ║
║                                                             qr-code-contactless               ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  GUIDES (10) — HOW-TO content                                                                  ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║  how-to-create-qr-code           qr-code-size-guide           qr-code-print-guide             ║
║  how-to-scan-qr-code             qr-code-best-practices       qr-code-design-tips             ║
║  how-to-track-qr-code            qr-code-error-correction     qr-code-placement-guide         ║
║                                  qr-code-testing-guide                                         ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  COMPARISONS (8) — VS content for SEO                                                          ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║  static-vs-dynamic-qr-code           qr-code-vs-barcode                                        ║
║  qr-code-vs-nfc                      custom-vs-standard-qr-code                                ║
║  free-vs-paid-qr-generator           qr-code-vs-short-link                                     ║
║  1d-vs-2d-barcode                    png-vs-svg-qr-code                                        ║
║                                                                                                ║
║  SEMANTIC_LINK:                                                                                ║
║  static-vs-dynamic-qr-code ──[contrasts]──► static-qr-code                                     ║
║                            ──[contrasts]──► dynamic-qr-code                                    ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  BRANDS (25) — External platforms                                                              ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║  SOCIAL (10)           MUSIC (3)            PAYMENT (5)           TECH (4)        MAPS (3)    ║
║  ───────────           ─────────            ───────────           ────────        ────────    ║
║  instagram             spotify              paypal                google          google-maps ║
║  linkedin              apple-music          venmo                 apple           apple-maps  ║
║  facebook              soundcloud           pix-brazil            microsoft       waze        ║
║  twitter/x                                  upi-india             amazon                      ║
║  youtube                                    stripe                                            ║
║  tiktok                                                                                        ║
║  snapchat                                                                                      ║
║  whatsapp                                                                                      ║
║  telegram                                                                                      ║
║  pinterest                                                                                     ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  CROSS-REALM GEOGRAPHIC TARGETING — Uses global nodes, no Entity type needed                  ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║  GLOBAL REALM (READ-ONLY)                         CROSS-REALM ARC                             ║
║  ────────────────────────                         ───────────────                             ║
║  Continent (6): africa, americas, asia...         Entity ──[:POPULAR_IN {weight}]──►          ║
║  GeoRegion (22): northern-africa, western-europe      GeoRegion | GeoSubRegion | Continent   ║
║  GeoSubRegion (50+): brazil, indian-subcontinent                                              ║
║  CulturalRealm (6): occidental, islamic, indic...                                             ║
║  IncomeGroup (4): high-income, upper-middle...                                                ║
║  EconomicRegion (7): eurozone, asean, mercosur...                                             ║
║                                                                                                ║
║  EXAMPLES:                                                                                    ║
║  qr-code-pix ──[:POPULAR_IN 0.95]──► brazil (GeoSubRegion)                                   ║
║  qr-code-upi ──[:POPULAR_IN 0.95]──► indian-subcontinent (GeoSubRegion)                      ║
║  qr-code-twint ──[:POPULAR_IN 0.85]──► western-europe (GeoRegion)                            ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  INTEGRATIONS (12) — Third-party integrations → linked to BRAND                                ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║  AUTOMATION (3)        CRM (3)              PRODUCTIVITY (3)      E-COMMERCE (3)              ║
║  ──────────────        ───────              ────────────────      ──────────────              ║
║  zapier-integration    hubspot-integration  google-sheets-int.    shopify-integration         ║
║  make-integration      salesforce-integ.    notion-integration    woocommerce-integ.          ║
║  n8n-integration       mailchimp-integ.     slack-integration     wordpress-integ.            ║
║                                                                                                ║
║  SEMANTIC: zapier-integration ──[related_to 0.90]──► zapier (BRAND)                           ║
║            zapier-integration ──[used_for 0.85]──► bulk-creation                              ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  TECHNICAL CONCEPTS (8) — QR Code internals for education                                      ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║  STRUCTURE (4)                              ENCODING (4)                                       ║
║  ─────────────                              ────────────                                       ║
║  quiet-zone        (white margin ≥4 mod)    encoding-mode    (Num/Alpha/Byte/Kanji)           ║
║  error-correction  (L/M/Q/H levels)         module           (single black/white unit)        ║
║  data-capacity     (chars by version+EC)    finder-pattern   (3 corner squares)               ║
║  qr-code-version   (v1=21x21 to v40=177)    timing-pattern   (alternating alignment)          ║
║                                                                                                ║
║  SEMANTIC: error-correction ──[enables 0.85]──► qr-code-with-logo                             ║
║            quiet-zone ──[part_of 0.80]──► qr-code                                             ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  SOCIAL SUBCATEGORIES (4) — Groupings within qr-code-social                                    ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║  qr-code-messaging     ──[includes]──► whatsapp, telegram                                     ║
║  qr-code-video         ──[includes]──► youtube, tiktok, snapchat                              ║
║  qr-code-professional  ──[includes]──► linkedin                                               ║
║  qr-code-music         ──[includes]──► spotify, apple-music, soundcloud                       ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║  LINK_TYPE LEGEND (13 types)                                                                   ║
╠════════════════════════════════════════════════════════════════════════════════════════════════╣
║                                                                                                ║
║  HIERARCHICAL          DEPENDENCY           BEHAVIORAL           FUNCTIONAL                   ║
║  type_of (0.95)        requires (0.90)      exhibits (0.85)      is_action_on (0.95)          ║
║  variant_of (0.85)     enables (0.85)       contrasts (0.25)     used_for (0.85-0.95)         ║
║  includes (0.70)                                                 part_of (0.80)               ║
║                                                                                                ║
║  ASSOCIATIVE           IDENTITY                                                                ║
║  related_to (0.40-0.70) same_as (1.00)  ← aliases                                             ║
║                                                                                                ║
╚════════════════════════════════════════════════════════════════════════════════════════════════╝
```

---

## Research References
- **Source concepts**: `/Users/thibaut/Projects/traduction_ai/ath-know-qrcai/_docs/concepts-detailed.md`
- **Sitemap**: `sitemap-structure/sitemap-qrcai.ini`

### SEO Schema References

- **SEOKeyword**: `packages/core/models/node-kinds/global/seo/seo-keyword.yaml`
- **SEOKeywordMetrics**: `packages/core/models/node-kinds/global/seo/seo-keyword-metrics.yaml`
- **SEOQuestion**: `packages/core/models/node-kinds/global/seo/seo-question.yaml`
- **SEOComparison**: `packages/core/models/node-kinds/global/seo/seo-comparison.yaml`
- **SEOPreposition**: `packages/core/models/node-kinds/global/seo/seo-preposition.yaml`
- **SEOMiningRun**: `packages/core/models/node-kinds/global/seo/seo-mining-run.yaml`

---

## Source of Truth Architecture

This section defines WHERE data lives and HOW it flows from source to Neo4j.

### YAML = Schema + Semantic Arcs (Source of Truth)

```
packages/core/models/
├── node-kinds/                          ← NODE SCHEMA (what nodes ARE)
│   ├── tenant/semantic/
│   │   ├── entity.yaml                  ✅ 13 types, POPULAR_IN, SEMANTIC_LINK
│   │   └── entity-l10n.yaml             ✅ TARGETS, ANSWERS, properties
│   └── global/seo/
│       ├── seo-keyword.yaml
│       ├── seo-question.yaml
│       └── ...
│
├── entity-arcs/                         ← SEMANTIC ARCS (N:N, validated)
│   ├── _schema.yaml                     Schema for arc YAML files
│   ├── semantic-links.yaml              ~500 SEMANTIC_LINK arcs
│   ├── subtopic-of.yaml                 ~275 SUBTOPIC_OF arcs
│   └── popular-in.yaml                  ~50 POPULAR_IN cross-realm arcs
│
└── taxonomy.yaml                        ← Realms, Layers, Traits
```

### Cypher = Node Data + Structural Arcs

```
packages/db/seed/
├── 20-locales.cypher                    200 Locale nodes
├── 27-geographic-taxonomy.cypher        Continents, GeoRegions, etc.
├── 28-locale-taxonomy-links.cypher      Locale → GeoRegion arcs
│
├── 30-entities-pillars.cypher           4 pillars + infra (5 nodes)
├── 31-entities-styles.cypher            QR styles + design specs (13)
├── 32-entities-content-types.cypher     URL, WiFi, social... (58)
├── 33-entities-features-tools.cypher    Analytics, Generator... (25)
├── 34-entities-mediums.cypher           Business cards, posters (20)
├── 35-entities-actions.cypher           Create, Scan, Track (15)
├── 36-entities-industries.cypher        Restaurants, Retail (25)
├── 37-entities-brands.cypher            Google, Instagram (25)
├── 38-entities-concepts.cypher          Dynamic, Static, Quiet Zone (12)
├── 39-entities-misc.cypher              Integrations, Guides, etc. (81)
│
├── 50-entity-arcs.cypher                ← GENERATED from entity-arcs/*.yaml
│
├── 60-entity-l10n-fr-FR.cypher          French localizations (future)
├── 61-entity-l10n-en-US.cypher          US English (future)
└── ...
```

### Data Flow

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  DATA FLOW: Source → Neo4j                                                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. SCHEMA (YAML)                                                               │
│     node-kinds/*.yaml ──► cargo run -- schema generate ──► TypeScript + Cypher │
│                                                                                 │
│  2. ENTITY NODES (Cypher direct)                                                │
│     Hand-written 30-39-entities-*.cypher ──► pnpm infra:seed ──► Neo4j         │
│                                                                                 │
│  3. SEMANTIC ARCS (YAML → Cypher)                                              │
│     entity-arcs/*.yaml ──► cargo run -- entity-arcs generate                   │
│                        ──► 50-entity-arcs.cypher                               │
│                        ──► pnpm infra:seed ──► Neo4j                           │
│                                                                                 │
│  4. LOCALE DATA (Cypher direct)                                                │
│     20-locales.cypher, 27-*.cypher, 28-*.cypher ──► pnpm infra:seed ──► Neo4j  │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Why This Split?

| Data Type | Format | Reason |
|-----------|--------|--------|
| **Entity nodes** | Cypher | Simple structure, created once, inline ownership arcs |
| **SEMANTIC_LINK** | YAML | N:N, properties (temperature, link_type), needs validation |
| **SUBTOPIC_OF** | YAML | Hierarchy important, depth property, reviewable |
| **POPULAR_IN** | YAML | Cross-realm, weight property, needs GeoRegion validation |
| **HAS_ENTITY, HAS_L10N** | Cypher inline | Created with node, 1:N ownership, atomic |

---

## Implementation Plan

### Phase 0: Preparation (DONE ✅)

| Task | Status | File |
|------|--------|------|
| Entity schema with 13 types | ✅ | `entity.yaml` |
| EntityL10n with SEO relations | ✅ | `entity-l10n.yaml` |
| POPULAR_IN cross-realm arc | ✅ | `entity.yaml` |
| Design document | ✅ | This file |

### Phase 1: Structure (Create Folders + YAML Schema)

| Task | File | Description |
|------|------|-------------|
| Create entity-arcs folder | `packages/core/models/entity-arcs/` | New directory |
| Arc schema definition | `_schema.yaml` | Structure for arc YAML files |
| Semantic links template | `semantic-links.yaml` | Empty, structure defined |
| Subtopic template | `subtopic-of.yaml` | Empty, structure defined |
| Popular-in template | `popular-in.yaml` | Empty, structure defined |

### Phase 2: Rust Generator

| Task | File | Description |
|------|------|-------------|
| YAML parser | `tools/novanet/src/parsers/entity_arcs.rs` | Parse entity-arcs/*.yaml |
| Cypher generator | `tools/novanet/src/generators/entity_arcs.rs` | Generate 50-entity-arcs.cypher |
| CLI command | `tools/novanet/src/commands/entity_arcs.rs` | `entity-arcs generate/validate` |
| Register command | `tools/novanet/src/commands/mod.rs` | Add to CLI |
| Tests | `tools/novanet/src/generators/entity_arcs_test.rs` | Snapshot + validation tests |

**Validations:**
- `from`/`to` entity keys exist in seed Cypher files
- `temperature` ∈ [0.0, 1.0]
- `link_type` ∈ enum (type_of, requires, enables, ...)
- `GeoRegion`/`GeoSubRegion` keys exist for POPULAR_IN

### Phase 3: Entity Nodes (Cypher Seeds)

| Seed File | Content | Count |
|-----------|---------|-------|
| `30-entities-pillars.cypher` | 4 pillars + short-link | 5 |
| `31-entities-styles.cypher` | QR styles + design specs | 13 |
| `32-entities-content-types.cypher` | URL, WiFi, social, payment... | 58 |
| `33-entities-features-tools.cypher` | Analytics, Generator... | 25 |
| `34-entities-mediums.cypher` | Business cards, posters... | 20 |
| `35-entities-actions.cypher` | Create, Scan, Track... | 15 |
| `36-entities-industries.cypher` | Restaurants, Retail... | 25 |
| `37-entities-brands.cypher` | Google, Instagram... | 25 |
| `38-entities-concepts.cypher` | Dynamic, Static, Quiet Zone... | 12 |
| `39-entities-misc.cypher` | Integrations, Guides, Comparisons | 81 |
| **TOTAL** | | **~279** |

### Phase 4: Entity Arcs (YAML → Cypher)

| YAML File | Arc Type | Count |
|-----------|----------|-------|
| `semantic-links.yaml` | SEMANTIC_LINK | ~500 |
| `subtopic-of.yaml` | SUBTOPIC_OF | ~275 |
| `popular-in.yaml` | POPULAR_IN | ~50 |
| **TOTAL** | | **~825** |

**Generated output:** `packages/db/seed/50-entity-arcs.cypher`

### Phase 5: EntityL10n (Future - Post-MVP)

| Seed File | Content | Locales |
|-----------|---------|---------|
| `60-entity-l10n-fr-FR.cypher` | French localizations | fr-FR |
| `61-entity-l10n-en-US.cypher` | US English | en-US |
| `62-entity-l10n-es-ES.cypher` | Spanish | es-ES |
| `63-entity-l10n-de-DE.cypher` | German | de-DE |
| `64-entity-l10n-ja-JP.cypher` | Japanese | ja-JP |

**Note:** EntityL10n content generated by LLM, validated by human.

### Phase 6: SEO Keywords (Future - Post-L10n)

| Seed File | Content |
|-----------|---------|
| `70-seo-keywords-fr-FR.cypher` | "Code QR", "Code QR dynamique"... |
| `71-seo-questions-fr-FR.cypher` | "comment créer un code qr"... |
| `72-seo-targeting-fr-FR.cypher` | EntityL10n --[:TARGETS]--> SEOKeyword |

**Note:** SEO data from Ahrefs/SEMrush research, not invented.

---

## Commands Reference

```bash
# Schema (existing)
cargo run -- schema generate           # Regenerate all artifacts
cargo run -- schema validate           # Validate YAML coherence

# Entity Arcs (new)
cargo run -- entity-arcs generate      # YAML → 50-entity-arcs.cypher
cargo run -- entity-arcs validate      # Validate arc YAML files

# Database
pnpm infra:seed                        # Execute all seed files
pnpm infra:reset                       # Drop + reseed
```

---

## Development Workflow & Tools

### Skills to Use

| Skill | When to Use |
|-------|-------------|
| `rust-core` | Rust fundamentals: ownership, error handling (thiserror), type-state patterns |
| `rust-async` | If async needed: Tokio patterns, channels, concurrency |
| `test-driven-development` | Write tests FIRST for parser and generator |
| `brainstorming` | Refine design decisions before coding |
| `verification-before-completion` | Run `cargo test` before claiming done |

### Agents to Dispatch

| Agent | Task |
|-------|------|
| `rust-pro` | Complex Rust implementation (parser, generator, CLI) |
| `rust-architect` | Design decisions for module structure |
| `code-reviewer` | Review implementation against this plan |
| `feature-dev:code-explorer` | Understand existing parser/generator patterns |

### Mandatory Workflows

```
1. Research existing patterns
   └── Read: parsers/yaml_node.rs, generators/node_kind.rs
   └── Understand: how existing YAML→Cypher works

2. TDD for Rust code
   └── Write test first (insta snapshots)
   └── Run test (RED)
   └── Implement minimal code
   └── Run test (GREEN)
   └── Refactor

3. Verification before commit
   └── cargo fmt && cargo clippy -- -D warnings
   └── cargo nextest run
   └── cargo deny check
```

### Claude Code Documentation

| Topic | Command/Skill |
|-------|---------------|
| Rust patterns | `/spn-rust:rust-core` |
| Async Tokio | `/spn-rust:rust-async` |
| NovaNet architecture | `/novanet-arch` |
| Schema management | `/novanet-sync` |
| Claude Code features | `spn-powers:claude-code-docs` |

### Quality Gates

| Phase | Verification |
|-------|--------------|
| Schema YAML | `cargo run -- schema validate` |
| Entity Arcs YAML | `cargo run -- entity-arcs validate` |
| Rust code | `cargo nextest run && cargo clippy -- -D warnings` |
| Generated Cypher | `pnpm infra:seed` (no errors) |
| Neo4j data | Query in browser: `MATCH (e:Entity) RETURN count(e)` |

---

## File Summary

### Files to CREATE

| Path | Type | Description |
|------|------|-------------|
| `packages/core/models/entity-arcs/_schema.yaml` | YAML | Arc file structure |
| `packages/core/models/entity-arcs/semantic-links.yaml` | YAML | ~500 SEMANTIC_LINK |
| `packages/core/models/entity-arcs/subtopic-of.yaml` | YAML | ~275 SUBTOPIC_OF |
| `packages/core/models/entity-arcs/popular-in.yaml` | YAML | ~50 POPULAR_IN |
| `packages/db/seed/30-entities-pillars.cypher` | Cypher | 5 pillar entities |
| `packages/db/seed/31-entities-styles.cypher` | Cypher | 13 style entities |
| `packages/db/seed/32-entities-content-types.cypher` | Cypher | 58 content types |
| `packages/db/seed/33-entities-features-tools.cypher` | Cypher | 25 features/tools |
| `packages/db/seed/34-entities-mediums.cypher` | Cypher | 20 mediums |
| `packages/db/seed/35-entities-actions.cypher` | Cypher | 15 actions |
| `packages/db/seed/36-entities-industries.cypher` | Cypher | 25 industries |
| `packages/db/seed/37-entities-brands.cypher` | Cypher | 25 brands |
| `packages/db/seed/38-entities-concepts.cypher` | Cypher | 12 concepts |
| `packages/db/seed/39-entities-misc.cypher` | Cypher | 81 misc entities |
| `tools/novanet/src/parsers/entity_arcs.rs` | Rust | YAML parser |
| `tools/novanet/src/generators/entity_arcs.rs` | Rust | Cypher generator |
| `tools/novanet/src/commands/entity_arcs.rs` | Rust | CLI command |

### Files ALREADY UPDATED

| Path | Status |
|------|--------|
| `packages/core/models/node-kinds/tenant/semantic/entity.yaml` | ✅ 13 types, POPULAR_IN |
| `packages/core/models/node-kinds/tenant/semantic/entity-l10n.yaml` | ✅ TARGETS, ANSWERS |

### Files to GENERATE

| Path | Generated By |
|------|--------------|
| `packages/db/seed/50-entity-arcs.cypher` | `cargo run -- entity-arcs generate` |

---

## Complete Architecture Overview (ASCII)

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════════════════╗
║  QR CODE AI — COMPLETE ENTITY ARCHITECTURE                                                                    ║
║  ~279 Entities │ 13 Types │ ~825 Arcs │ 200 Locales │ Cross-Realm Geographic Targeting                       ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════════════════╝


┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  GLOBAL REALM (READ-ONLY)                                                                                     │
├──────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                               │
│  ┌─────────────────────────────────────────┐    ┌─────────────────────────────────────────────────────────┐ │
│  │  GEOGRAPHIC TAXONOMY                     │    │  LOCALE KNOWLEDGE                                      │ │
│  │  ─────────────────────                   │    │  ────────────────                                      │ │
│  │                                          │    │                                                        │ │
│  │  Continent (6)                           │    │  Locale (200)                                          │ │
│  │    └── GeoRegion (22)                    │    │    ├──[:IN_SUBREGION]────► GeoRegion                   │ │
│  │          └── GeoSubRegion (50+)          │    │    ├──[:SPEAKS_BRANCH]───► LanguageBranch              │ │
│  │                                          │    │    ├──[:HAS_INCOME_LEVEL]─► IncomeGroup                │ │
│  │  CulturalRealm (6)                       │    │    ├──[:IN_CULTURAL_SUBREALM]─► CulturalSubRealm       │ │
│  │    └── CulturalSubRealm (30+)            │    │    └──[:HAS_PRIMARY_POPULATION]─► PopulationSubCluster │ │
│  │                                          │    │                                                        │ │
│  │  LanguageFamily (12)                     │    │  IncomeGroup (4): hic, umic, lmic, lic                 │ │
│  │    └── LanguageBranch (23)               │    │  EconomicRegion (7): eurozone, asean, mercosur...     │ │
│  │                                          │    │                                                        │ │
│  └─────────────────────────────────────────┘    └─────────────────────────────────────────────────────────┘ │
│                         ▲                                                                                     │
│                         │ [:POPULAR_IN]                                                                       │
│                         │ (cross-realm)                                                                       │
│                         │                                                                                     │
└─────────────────────────┼────────────────────────────────────────────────────────────────────────────────────┘
                          │
┌─────────────────────────┼────────────────────────────────────────────────────────────────────────────────────┐
│  TENANT REALM           │                                                                                     │
├─────────────────────────┼────────────────────────────────────────────────────────────────────────────────────┤
│                         │                                                                                     │
│  ┌──────────────────────┴───────────────────────────────────────────────────────────────────────────────────┐│
│  │  SEMANTIC LAYER — Entity Graph (~279 nodes, ~825 arcs)                                                    ││
│  │  ════════════════════════════════════════════════════════════════════════════════════════════════════════││
│  │                                                                                                           ││
│  │  ═══════════════════════════════════════════════════════════════════════════════════════════════════════ ││
│  │  CORE DEPENDENCY CHAIN                                                                                    ││
│  │  ═══════════════════════════════════════════════════════════════════════════════════════════════════════ ││
│  │                                                                                                           ││
│  │  ┌─────────────────┐         ┌─────────────────┐         ┌─────────────────────────────────────────────┐ ││
│  │  │   SHORT-LINK    │         │  DYNAMIC-QR-CODE│         │         ANALYTICS (FEATURE)                 │ ││
│  │  │    (THING)      │◄────────│    (CONCEPT)    │────────►│  ┌──────────────────────────────────────┐   │ ││
│  │  │                 │[requires│                 │[enables]│  │  INCLUDES (sub-entities):            │   │ ││
│  │  │ Core redirect   │  0.90]  │ Editable after  │  [0.85] │  │  • click-tracking (FEATURE)          │   │ ││
│  │  │ mechanism       │         │ print           │         │  │  • scan-counting (FEATURE)           │   │ ││
│  │  │                 │         │                 │         │  │  • geo-tracking (FEATURE)            │   │ ││
│  │  │ [enables]       │         │ [variant_of]    │         │  │  • device-detection (FEATURE)        │   │ ││
│  │  │  ├─► analytics  │         │  ▲              │         │  │  • time-series (FEATURE)             │   │ ││
│  │  │  ├─► smart-link │         │  │ [0.85]       │         │  └──────────────────────────────────────┘   │ ││
│  │  │  └─► context-rt │         │  │              │         │                                             │ ││
│  │  └─────────────────┘         └──┼──────────────┘         └─────────────────────────────────────────────┘ ││
│  │                                 │                                                                         ││
│  │  ═══════════════════════════════╪═══════════════════════════════════════════════════════════════════════ ││
│  │  4 PILLARS                      │                                                                         ││
│  │  ═══════════════════════════════╪═══════════════════════════════════════════════════════════════════════ ││
│  │                                 │                                                                         ││
│  │  ┌───────────────┐    ┌─────────┴───────────────────┐    ┌───────────────┐    ┌───────────────┐          ││
│  │  │    BARCODE    │◄───│         QR-CODE             │───►│  SMART-LINK   │───►│ LANDING-PAGE  │          ││
│  │  │   (PILLAR)    │rel │        (PILLAR)             │rel │   (PILLAR)    │rel │   (PILLAR)    │          ││
│  │  │   18 nodes    │0.40│       is_pillar: true       │0.50│               │0.40│   7 nodes     │          ││
│  │  └───────┬───────┘    └─────────────┬───────────────┘    └───────────────┘    └───────┬───────┘          ││
│  │          │                          │                                                  │                  ││
│  │          │ [includes]               │ [includes]                                       │ [includes]       ││
│  │          ▼                          ▼                                                  ▼                  ││
│  │  ┌───────────────┐    ┌──────────────────────────────────────────────┐        ┌───────────────┐          ││
│  │  │BARCODE-FORMAT │    │              QR-CODE VARIANTS                 │        │LANDING-TYPE   │          ││
│  │  │   (THING)     │    │ ┌───────────────────┐ ┌───────────────────┐  │        │   (THING)     │          ││
│  │  │               │    │ │  STATIC-QR-CODE   │ │  DYNAMIC-QR-CODE  │  │        │               │          ││
│  │  │ • ean-13      │    │ │    (CONCEPT)      │ │    (CONCEPT)      │  │        │ • link-in-bio │          ││
│  │  │ • upc-a       │    │ │                   │ │                   │  │        │ • menu        │          ││
│  │  │ • code-128    │    │ │ Fixed URL         │ │ Editable URL      │  │        │ • forms       │          ││
│  │  │ • data-matrix │    │ │ No tracking       │ │ + Analytics       │  │        │ • event-rsvp  │          ││
│  │  │ • pdf417      │    │ │                   │ │                   │  │        │ • booking     │          ││
│  │  └───────────────┘    │ │ [contrasts 0.25]◄─┼─┤ [requires 0.90]───┼──┼────►short-link        │          ││
│  │                       │ │ [variant_of 0.85] │ │ [variant_of 0.85] │  │        └───────────────┘          ││
│  │                       │ │        │          │ │        │          │  │                                   ││
│  │                       │ └────────┼──────────┘ └────────┼──────────┘  │                                   ││
│  │                       │          │                     │             │                                   ││
│  │                       │          └─────────┬───────────┘             │                                   ││
│  │                       │                    ▼                         │                                   ││
│  │                       │           ┌─────────────────┐                │                                   ││
│  │                       │           │  QR-CODE-STYLE  │                │                                   ││
│  │                       │           │   (THING)       │                │                                   ││
│  │                       │           │ ┌─────┐┌─────┐┌─────┐            │                                   ││
│  │                       │           │ │CUST.││IMAGE││ ART │            │                                   ││
│  │                       │           │ │[0.85││[0.85││[0.85│            │                                   ││
│  │                       │           │ └─────┘└─────┘└─────┘            │                                   ││
│  │                       │           │ • with-logo • with-text          │                                   ││
│  │                       │           │ • color • shapes • bg            │                                   ││
│  │                       └───────────┴─────────────────┴────────────────┘                                   ││
│  │                                                                                                           ││
│  │  ═══════════════════════════════════════════════════════════════════════════════════════════════════════ ││
│  │  CONTENT TYPES (58)                  CONCEPTS (12)                 ACTIONS (15)                           ││
│  │  ═══════════════════════════════════════════════════════════════════════════════════════════════════════ ││
│  │                                                                                                           ││
│  │  ┌─────────────┐ ┌─────────────┐    ┌──────────────────────────────────────┐    ┌─────────────┐          ││
│  │  │    BASIC    │ │   SOCIAL    │    │  CONCEPTS                            │    │   ACTIONS   │          ││
│  │  │  url, wifi  │ │  instagram  │    │  • dynamic-qr-code [variant_of qr]   │    │  • create   │          ││
│  │  │  vcard, pdf │ │  linkedin   │    │  • static-qr-code  [variant_of qr]   │    │  • scan     │          ││
│  │  │  email, sms │ │  facebook   │    │  • error-correction                  │    │  • generate │          ││
│  │  │             │ │  youtube    │    │  • encoding                          │    │  • download │          ││
│  │  └─────────────┘ └─────────────┘    │  • redirect                          │    │  • customize│          ││
│  │                                      └──────────────────────────────────────┘    │  • track    │          ││
│  │  ┌─────────────┐ ┌─────────────┐                                                 │  [is_action_│          ││
│  │  │   PAYMENT   │ │  LOCATION   │    ┌─────────────────────────────────────────┐  │  on 0.95]  │          ││
│  │  │  pix (🇧🇷)   │ │ google-maps │    │  FEATURES & TOOLS (25+)                 │  └─────────────┘          ││
│  │  │  upi (🇮🇳)   │ │ apple-maps  │    │  ───────────────────────                │                          ││
│  │  │  paypal     │ │ waze        │    │                                         │                          ││
│  │  │  bitcoin    │ │ coordinates │    │  ┌─────────────────────────────────┐    │                          ││
│  │  │             │ │             │    │  │ ANALYTICS [includes]:           │    │                          ││
│  │  │ [:POPULAR_IN│ │             │    │  │  • click-tracking               │    │                          ││
│  │  │  brazil 0.9]│ │             │    │  │  • scan-counting                │    │                          ││
│  │  └─────────────┘ └─────────────┘    │  │  • geo-tracking                 │    │                          ││
│  │                                      │  │  • device-detection             │    │                          ││
│  │                                      │  │  • time-series                  │    │                          ││
│  │                                      │  └─────────────────────────────────┘    │                          ││
│  │                                      │                                         │                          ││
│  │                                      │  Other features:                        │                          ││
│  │                                      │  contextual-routing, custom-domain,     │                          ││
│  │                                      │  bulk-creation, team-workspaces, api,   │                          ││
│  │                                      │  qr-code-generator, qr-code-scanner,    │                          ││
│  │                                      │  barcode-generator, landing-page-builder│                          ││
│  │                                      │                                         │                          ││
│  │                                      │  [used_for 0.95] ──► qr-code            │                          ││
│  │                                      │  [part_of 0.85] ──► smart-link          │                          ││
│  │                                      └─────────────────────────────────────────┘                          ││
│  │                                                                                                           ││
│  │  ═══════════════════════════════════════════════════════════════════════════════════════════════════════ ││
│  │  MEDIUMS (20)        INDUSTRIES (25)       BRANDS (25)        INTEGRATIONS (12)                          ││
│  │  ═══════════════════════════════════════════════════════════════════════════════════════════════════════ ││
│  │                                                                                                           ││
│  │  • business-cards    • restaurants          • google           • zapier-integration                      ││
│  │  • posters           • retail               • instagram        • hubspot-integration                     ││
│  │  • flyers            • healthcare           • facebook         • google-sheets-integration               ││
│  │  • packaging         • marketing-agencies   • spotify          • shopify-integration                     ││
│  │  • emails            • real-estate          • paypal           • salesforce-integration                  ││
│  │  • table-tents       • education            • apple            • make-integration                        ││
│  │                                                                                                           ││
│  └───────────────────────────────────────────────────────────────────────────────────────────────────────────┘│
│                                                                                                               │
│  ┌───────────────────────────────────────────────────────────────────────────────────────────────────────────┐│
│  │  Entity ──[:HAS_L10N]──► EntityL10n ──[:FOR_LOCALE]──► Locale                                              ││
│  │                              │                                                                             ││
│  │                              ├──[:TARGETS]──► SEOKeyword (global/seo)                                      ││
│  │                              └──[:ANSWERS]──► SEOQuestion (global/seo)                                     ││
│  └───────────────────────────────────────────────────────────────────────────────────────────────────────────┘│
│                                                                                                               │
└───────────────────────────────────────────────────────────────────────────────────────────────────────────────┘


┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  KEY RELATIONSHIPS SUMMARY                                                                                     │
├───────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                               │
│  qr-code (PILLAR)                                                                                             │
│    ├──[includes]──► static-qr-code (CONCEPT)                                                                  │
│    │                  └──[contrasts 0.25]──► dynamic-qr-code                                                  │
│    │                  └──[variant_of 0.85]──► qr-code                                                         │
│    └──[includes]──► dynamic-qr-code (CONCEPT)                                                                 │
│                       └──[requires 0.90]──► short-link                                                        │
│                       └──[variant_of 0.85]──► qr-code                                                         │
│                       └──[enables 0.85]──► analytics                                                          │
│                                                                                                               │
│  short-link (THING)                                                                                           │
│    ├──[enables 0.85]──► analytics                                                                             │
│    ├──[enables 0.85]──► smart-link                                                                            │
│    └──[enables 0.85]──► contextual-routing                                                                    │
│                                                                                                               │
│  analytics (FEATURE)                                                                                          │
│    ├──[includes]──► click-tracking (FEATURE)                                                                  │
│    ├──[includes]──► scan-counting (FEATURE)                                                                   │
│    ├──[includes]──► geo-tracking (FEATURE)                                                                    │
│    ├──[includes]──► device-detection (FEATURE)                                                                │
│    └──[includes]──► time-series (FEATURE)                                                                     │
│                                                                                                               │
├───────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│  ARC TYPES LEGEND                                                                                              │
├───────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                               │
│  HIERARCHICAL              DEPENDENCY              BEHAVIORAL              FUNCTIONAL                         │
│  ─────────────             ──────────              ──────────              ──────────                         │
│  type_of (0.95)            requires (0.90)         exhibits (0.85)         is_action_on (0.95)                │
│  variant_of (0.85)         enables (0.85)          contrasts (0.25)        used_for (0.85-0.95)               │
│  includes (0.70)                                                           part_of (0.80)                     │
│                                                                                                               │
│  ASSOCIATIVE               IDENTITY                CROSS-REALM                                                │
│  ───────────               ────────                ───────────                                                │
│  related_to (0.40-0.70)    same_as (1.00)          POPULAR_IN (0.8-0.95)                                      │
│                                                                                                               │
│  STRUCTURAL (Cypher inline)                                                                                   │
│  ──────────────────────────                                                                                   │
│  HAS_ENTITY, HAS_L10N, FOR_LOCALE, SUBTOPIC_OF                                                                │
│                                                                                                               │
└───────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

