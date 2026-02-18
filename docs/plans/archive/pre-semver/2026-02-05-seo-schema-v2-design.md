# SEO Schema v2 Design

**Date**: 2026-02-05
**Status**: Approved
**Author**: Thibaut + Claude

## Overview

This design extends the NovaNet SEO schema to capture richer keyword data from Ahrefs, Semrush, and Answer The Public (ATP). It introduces 3 new NodeKinds for ATP-derived question/comparison/preposition keywords, and establishes bidirectional relationships between SEO data and Entity nodes.

## Goals

1. **Enrich SEOKeyword** with traffic potential, clicks, SERP features, and trends
2. **Capture ATP data** as first-class nodes with their own volume/difficulty
3. **Link SEO to Entities** bidirectionally (EXPRESSES + TARGETS)
4. **Track coverage** - which questions are answered, which comparisons addressed

## Non-Goals

- Personal KD (too site-specific, not portable)
- Parent topic clustering (handled by Entity → SEMANTIC_LINK → Entity)
- URL/position on SEOKeyword (lives in SEOKeywordMetrics or PageL10n)

## Data Sources

### Ahrefs API

| Field | Description | Maps To |
|-------|-------------|---------|
| volume | Monthly searches (12-month average) | `volume` |
| keyword_difficulty | KD 0-100 | `difficulty` |
| traffic_potential | Estimated traffic if ranking #1 | `traffic_potential` |
| clicks | Actual clicks (not just impressions) | `clicks` |
| clicks_per_search | Clicks/volume ratio | `clicks_per_search` |
| cpc | Cost per click | `cpc` |
| serp_features | AI Overview, Featured Snippet, etc. | `serp_features[]` |
| trend | Historical direction | `trend` |

### Semrush API

| Field | Description | Maps To |
|-------|-------------|---------|
| volume (Nq/Ph) | Monthly searches | `volume` |
| keyword_difficulty (KD%) | 1-100 | `difficulty` |
| cpc | Cost per click | `cpc` |
| intent | informational/transactional/etc. | `intent` |
| competition | Organic competition 0-1 | `competition` |
| serp_features (Sfc) | Count + breakdown | `serp_features[]` |
| trends | 12-month interest curve | `seasonality[]` |

### Answer The Public API

| Category | Example | NodeKind |
|----------|---------|----------|
| Questions | "comment créer un qr code" | SEOQuestion |
| Comparisons | "qr code vs barcode" | SEOComparison |
| Prepositions | "qr code pour wifi" | SEOPreposition |
| Alphabeticals | "a qr code", "b qr code" | (skip - low value) |

## Schema Changes

### NodeKinds

#### SEOKeyword (enriched)

```yaml
node:
  name: SEOKeyword
  realm: global
  layer: seo
  trait: knowledge

properties:
  # Existing
  value: string (required)
  volume: int
  difficulty: int (0-100)
  cpc: float
  intent: string (transactional|informational|navigational|commercial)
  platform: string (google|bing|youtube)
  source: string (ahrefs|semrush)

  # New - Ahrefs/Semrush enrichment
  traffic_potential: int        # Estimated traffic if #1
  clicks: int                   # Actual clicks (not volume)
  clicks_per_search: float      # clicks/volume ratio
  serp_features: string[]       # ["featured_snippet", "ai_overview", "images"]
  competition: float            # Organic competition 0-1
  trend: string                 # "rising"|"stable"|"declining"
  seasonality: int[]            # 12-month index [80,85,90,100,...]
```

#### SEOKeywordMetrics (enriched)

```yaml
node:
  name: SEOKeywordMetrics
  realm: global
  layer: seo
  trait: derived

properties:
  # Existing
  observed_at: datetime (required)
  source: string (required)
  volume: int
  difficulty: float
  cpc: float

  # New - snapshot data
  clicks: int
  traffic_potential: int

  # New - position tracking
  position: float               # Our average ranking
  best_position: int            # Best position that day
  url: string                   # Page that ranks
  impressions: int              # GSC impressions
  clicks_gsc: int               # GSC clicks (our clicks)
  ctr: float                    # clicks_gsc/impressions
```

#### SEOQuestion (new)

```yaml
node:
  name: SEOQuestion
  realm: global
  layer: seo
  trait: knowledge
  icon: "❓"
  description: "Question-form keyword from Answer The Public"

properties:
  value: string (required)      # "comment créer un qr code"
  volume: int
  difficulty: int
  cpc: float
  question_word: string (required)  # what|how|why|when|where|who|can|is|does
  answer_type: string           # explanation|list|yes_no|steps
  featured_snippet: bool        # Has FS opportunity
  paa_position: int             # Position in People Also Ask
```

#### SEOComparison (new)

```yaml
node:
  name: SEOComparison
  realm: global
  layer: seo
  trait: knowledge
  icon: "⚖️"
  description: "Comparison keyword linking two entities"

properties:
  value: string (required)      # "qr code vs barcode"
  volume: int
  difficulty: int
  cpc: float
  comparison_type: string (required)  # vs|or|versus|compared_to|better_than|like
  entity_a_key: string (required)     # Left side entity key
  entity_b_key: string (required)     # Right side entity key
  winner: string                # a|b|tie|depends (if known)
```

#### SEOPreposition (new)

```yaml
node:
  name: SEOPreposition
  realm: global
  layer: seo
  trait: knowledge
  icon: "🔗"
  description: "Preposition keyword indicating use case"

properties:
  value: string (required)      # "qr code pour wifi"
  volume: int
  difficulty: int
  cpc: float
  preposition: string (required)  # for|with|without|near|to|like
  use_case: string (required)   # "wifi" (what it's for/with)
  use_case_type: string         # feature|industry|audience|integration
```

### ArcKinds

#### Ownership (SEOKeyword → ATP nodes)

| Arc | From | To | Family | Description |
|-----|------|----|--------|-------------|
| HAS_QUESTIONS | SEOKeyword | SEOQuestion | ownership | 1:N |
| HAS_COMPARISONS | SEOKeyword | SEOComparison | ownership | 1:N |
| HAS_PREPOSITIONS | SEOKeyword | SEOPreposition | ownership | 1:N |

#### Semantic (SEO ↔ Entity)

| Arc | From | To | Family | Description |
|-----|------|----|--------|-------------|
| EXPRESSES | SEOKeyword | Entity | semantic | Keyword expresses this concept |
| TARGETS | EntityL10n | SEOKeyword | semantic | Content targets this keyword |
| COMPARES_A | SEOComparison | Entity | semantic | Left side of comparison |
| COMPARES_B | SEOComparison | Entity | semantic | Right side of comparison |
| USE_CASE_ENTITY | SEOPreposition | Entity | semantic | Optional link if use_case is an Entity |

#### Coverage Tracking

| Arc | From | To | Family | Description |
|-----|------|----|--------|-------------|
| ANSWERS | EntityL10n | SEOQuestion | semantic | This content answers this question |
| ADDRESSES | PageL10n/BlockL10n | SEOComparison/SEOPreposition | semantic | This page addresses this topic |

## Graph Structure

```
                            GLOBAL/config
                           ┌──────────────┐
                           │   Locale     │
                           │   (fr-FR)    │
                           └──────┬───────┘
                                  │
                     [:HAS_SEO_KEYWORDS]
                                  │
                                  ▼
                            GLOBAL/seo
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                                                                 │
│                         ┌─────────────────┐                                     │
│                         │   SEOKeyword    │──[:EXPRESSES]──────────┐            │
│                         │ "créer qr code" │                        │            │
│                         └────────┬────────┘                        │            │
│                ┌─────────────────┼─────────────────┐               │            │
│                ▼                 ▼                 ▼               │            │
│         ┌──────────┐      ┌──────────┐      ┌──────────┐          │            │
│         │SEOQuestion│     │SEOCompare│      │SEOPrepos │          │            │
│         └─────┬────┘      └────┬─┬───┘      └────┬─────┘          │            │
│               │           [:COMPARES_A/B]        │                │            │
└───────────────┼────────────────┼─┼───────────────┼────────────────┼────────────┘
                │                │ │               │                │
                │ [:ANSWERS]     ▼ ▼    [:USE_CASE_ENTITY]          │
                │           ┌─────────────────────────────┐         │
                │           │      TENANT/semantic        │         │
                │           │  ┌────────┐  ┌────────┐     │         │
                └───────────┼─►│ Entity │  │ Entity │     │         │
                            │  │qr-code │  │barcode │◄────┼─────────┘
                            │  └───┬────┘  └────────┘     │
                            │      │                      │
                            │ ┌────▼─────┐                │
                            │ │EntityL10n│──[:TARGETS]───►│ SEOKeyword
                            │ │  fr-FR   │                │
                            │ └──────────┘                │
                            └─────────────────────────────┘
```

## Examples

### SEOKeyword

```json
{
  "key": "seo-creer-qr-code-gratuit-fr",
  "display_name": "créer qr code gratuit",
  "value": "créer qr code gratuit",
  "volume": 12100,
  "difficulty": 35,
  "cpc": 0.85,
  "intent": "transactional",
  "platform": "google",
  "source": "ahrefs",
  "traffic_potential": 8500,
  "clicks": 9800,
  "clicks_per_search": 0.81,
  "serp_features": ["featured_snippet", "people_also_ask", "images"],
  "competition": 0.65,
  "trend": "rising",
  "seasonality": [80, 85, 90, 100, 95, 88, 82, 78, 85, 92, 98, 105]
}
```

### SEOQuestion

```json
{
  "key": "seoq-comment-creer-qr-code-fr",
  "display_name": "comment créer un qr code",
  "value": "comment créer un qr code",
  "volume": 8000,
  "difficulty": 28,
  "cpc": 0.45,
  "question_word": "how",
  "answer_type": "steps",
  "featured_snippet": true,
  "paa_position": 2
}
```

### SEOComparison

```json
{
  "key": "seoc-qr-code-vs-barcode-fr",
  "display_name": "qr code vs barcode",
  "value": "qr code vs barcode",
  "volume": 2000,
  "difficulty": 22,
  "cpc": 0.30,
  "comparison_type": "vs",
  "entity_a_key": "qr-code",
  "entity_b_key": "barcode",
  "winner": null
}
```

### SEOPreposition

```json
{
  "key": "seop-qr-code-pour-wifi-fr",
  "display_name": "qr code pour wifi",
  "value": "qr code pour wifi",
  "volume": 3000,
  "difficulty": 18,
  "cpc": 0.55,
  "preposition": "for",
  "use_case": "wifi",
  "use_case_type": "feature"
}
```

## Migration

No migration needed - these are additive changes:
- New properties on existing nodes are optional
- New NodeKinds are independent
- New ArcKinds can be created incrementally

## Implementation Plan

1. Update `seo-keyword.yaml` with new properties
2. Update `seo-keyword-metrics.yaml` with position tracking
3. Create `seo-question.yaml`
4. Create `seo-comparison.yaml`
5. Create `seo-preposition.yaml`
6. Create ArcKind YAMLs:
   - `has-questions.yaml`
   - `has-comparisons.yaml`
   - `has-prepositions.yaml`
   - `expresses.yaml`
   - `targets.yaml`
   - `compares-a.yaml`
   - `compares-b.yaml`
   - `use-case-entity.yaml`
   - `answers.yaml`
   - `addresses.yaml`
7. Run `cargo run -- schema generate` to regenerate artifacts
8. Run `cargo run -- schema validate` to verify

## Statistics

| Category | Before | After | Delta |
|----------|--------|-------|-------|
| NodeKinds (SEO layer) | 3 | 6 | +3 |
| ArcKinds | ~63 | ~73 | +10 |
| Properties (SEOKeyword) | 11 | 18 | +7 |
| Properties (SEOKeywordMetrics) | 10 | 16 | +6 |

---

*Design approved: 2026-02-05*
