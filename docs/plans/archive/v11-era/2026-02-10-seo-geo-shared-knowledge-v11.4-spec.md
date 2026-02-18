# SEO/GEO Shared Knowledge Schema — v11.4 Spec

**Date**: 2026-02-10
**Status**: Ready for Implementation
**Implementation Plan**: `2026-02-10-v11.4-implementation-plan.md`
**Breaking Change**: Yes

---

## Executive Summary

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  v11.4 CHANGES                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  REALM MIGRATION:                                                           │
│  ├─ org/seo (5 nodes) ───────────────────► shared (4 SEO nodes)            │
│  ├─ org/geo (3 nodes) ───────────────────► shared (3 GEO nodes)            │
│  └─ org/config/EntityCategory ───────────► shared/config (universal)       │
│                                                                             │
│  NODE CHANGES:                                                              │
│  ├─ NEW: SEOKeywordSet, GEOQuerySet (containers, trait=invariant)          │
│  ├─ NEW: SEOKeywordFormat (classification, trait=invariant)                │
│  ├─ MOVE: EntityCategory → shared/config (universal categories)            │
│  ├─ STATIC: SEOKeyword, GEOQuery (atoms, trait=knowledge)                  │
│  ├─ TIME-SERIES: SEOKeywordMetrics, GEOAnswer (trait=aggregated)           │
│  ├─ REMOVED: SEOQuestion, SEOComparison, SEOPreposition (consolidated)     │
│  └─ REMOVED: GEOMetrics (GEOAnswer IS the time-series)                     │
│                                                                             │
│  TRAIT DECISIONS:                                                           │
│  ├─ Containers (SEOKeywordSet, GEOQuerySet): invariant                     │
│  ├─ Atoms (SEOKeyword, GEOQuery): knowledge (discovered, not generated)    │
│  ├─ Time-series (SEOKeywordMetrics, GEOAnswer): aggregated                 │
│  └─ Classifications (SEOKeywordFormat, EntityCategory): invariant          │
│                                                                             │
│  ARC CHANGES:                                                               │
│  ├─ Type-specific: HAS_SEO_KEYWORD_METRICS, HAS_GEO_ANSWER                 │
│  ├─ NEW: HAS_FORMAT (SEOKeyword → SEOKeywordFormat)                        │
│  ├─ NEW: COMPARES_A, COMPARES_B, USE_CASE_FOR, MENTIONS_BRAND              │
│  ├─ Cross-realm: TARGETS, MONITORS_GEO (org → shared)                      │
│  └─ Intra-realm: HAS_SEO_KEYWORDS, HAS_GEO_QUERIES (shared → shared)       │
│                                                                             │
│  LAYER CHANGES:                                                             │
│  ├─ ADD: shared/config layer (SEOKeywordFormat, EntityCategory)            │
│  ├─ REMOVE: org/seo layer                                                  │
│  ├─ REMOVE: org/geo layer                                                  │
│  ├─ SHARED: 4 layers (config, locale, geography, knowledge)                │
│  └─ ORG: 6 layers (config, semantic, foundation, structure, instr, output) │
│                                                                             │
│  TOTAL: 9 nodes in shared (8 SEO/GEO + EntityCategory)                     │
│                                                                             │
│  PATTERNS:                                                                  │
│  ├─ SEOKeywordFormat follows EntityCategory (ADR-017)                      │
│  └─ SEOKeyword/GEOQuery follow Knowledge Atoms (Term, Expression)          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Context

Moving SEO and GEO from `org` realm to `shared/knowledge` realm following the Knowledge Atoms pattern (like Term, Expression, Pattern).

**Rationale**:
- SEO keywords are universal market data (same "créer qr code" for everyone)
- GEO queries are universal AI search patterns
- Only the *targeting* and *performance* is org-specific (via arcs)

## Current State (v11.3)

```
org/seo/     → SEOKeyword, SEOKeywordMetrics, SEOQuestion, SEOComparison, SEOPreposition
org/geo/     → GEOQuery, GEOAnswer, GEOMetrics
```

## Proposed State (v11.4)

```
shared/knowledge/ → All SEO/GEO nodes (Knowledge Atoms pattern)
org/             → 6 layers (remove seo, geo layers)
```

---

## 1. Container Classification

### SEOKeywordSet (Container, Invariant)

Groups keywords by **search intent** (industry standard):

| Intent | Description | Example |
|--------|-------------|---------|
| `informational` | Learn/research | "comment créer qr code" |
| `navigational` | Find specific site | "qrcode-ai login" |
| `commercial` | Compare options | "meilleur générateur qr code" |
| `transactional` | Ready to buy/act | "acheter qr code premium" |

### GEOQuerySet (Container, Invariant)

Groups AI queries by **query type**:

| Type | Description | Example |
|------|-------------|---------|
| `definition` | Explain concept | "what is a QR code" |
| `how_to` | Step-by-step | "how to create QR code" |
| `comparison` | Evaluate options | "best QR code generators" |
| `recommendation` | Suggest solution | "QR code for business" |
| `troubleshooting` | Solve problem | "QR code not scanning" |
| `opinion` | Perspective | "is QR code still relevant" |

---

## 2. SEOKeyword Properties (STATIC - shared/knowledge)

> **Principle**: SEOKeyword = STATIC identity. All changing metrics go to SEOKeywordMetrics.

### Core Identity (Immutable)

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `value` | string | ✓ | Keyword string ("créer qr code gratuit") |
| `intent` | enum | ✓ | informational, navigational, commercial, transactional |
| `platform` | string | | google, bing, youtube |
| `language_hint` | string | | ISO 639-1 code (fr, en, de) |

### Stable Characteristics (Rarely Change)

| Property | Type | Description |
|----------|------|-------------|
| `competition_level` | enum | LOW, MEDIUM, HIGH (stable category) |
| `categories` | int[] | Google Ads category IDs |
| `seasonality_pattern` | int[] | 12-month index [80,85,90,100...] for quick trend access |

### ⚠️ MOVED TO SEOKeywordMetrics (Time-Series)

These properties change over time and belong in snapshots:
- ~~search_volume~~ → `SEOKeywordMetrics.monthly_search_volume`
- ~~cpc~~ → `SEOKeywordMetrics.cost_per_click`
- ~~keyword_difficulty~~ → `SEOKeywordMetrics.ranking_difficulty`
- ~~competition~~ → `SEOKeywordMetrics.organic_competition`
- ~~serp_features~~ → `SEOKeywordMetrics.serp_features` (they appear/disappear!)
- ~~clicks~~ → `SEOKeywordMetrics.estimated_clicks`
- ~~traffic_potential~~ → `SEOKeywordMetrics.traffic_potential`

---

## 2b. SEOKeywordMetrics Properties (TIME-SERIES - shared/knowledge)

> **Principle**: Immutable snapshots. Each observation creates a new node.

### Observation Metadata

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `observed_at` | datetime | ✓ | When metrics were captured (ISO 8601) |
| `source` | string | ✓ | dataforseo, semrush, ahrefs |
| `observation_period` | string | | ISO 8601 duration (P1M = monthly) |

### Volume & Traffic Metrics

| Property | Type | Description |
|----------|------|-------------|
| `monthly_search_volume` | int | Monthly average searches |
| `estimated_clicks` | int | Monthly clicks (actual, not impressions) |
| `clicks_per_search` | float | Click satisfaction ratio |
| `traffic_potential` | int | Est. traffic if ranking #1 |

### Competition & Difficulty

| Property | Type | Description |
|----------|------|-------------|
| `ranking_difficulty` | int | 0-100 organic ranking difficulty |
| `organic_competition` | float | 0.0-1.0 competition level |
| `cost_per_click` | float | CPC in USD |
| `low_top_of_page_bid` | float | Min bid for top SERP |
| `high_top_of_page_bid` | float | Max bid for top SERP |

### SERP Features (Change Over Time!)

| Property | Type | Description |
|----------|------|-------------|
| `serp_features` | string[] | featured_snippet, ai_overview, people_also_ask, etc. |
| `serp_result_count` | int | Total search results |

### Trend Deltas

| Property | Type | Description |
|----------|------|-------------|
| `volume_change_7d` | float | % change vs 7 days ago |
| `volume_change_30d` | float | % change vs 30 days ago |

---

## 3. SEOKeywordFormat (Invariant Classification - Pattern EntityCategory)

> **Pattern**: Follows EntityCategory (ADR-017) — invariant classification nodes in shared/config.

### SEOKeywordFormat Node (shared/config, invariant)

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `format_key` | string | ✓ | Unique format identifier |
| `display_name` | string | ✓ | Human-readable name |
| `description` | string | | LLM context for this format |
| `has_entity_links` | boolean | | Whether this format can link to entities |

### Format Instances (7 invariant nodes)

| format_key | display_name | has_entity_links | Description |
|------------|--------------|------------------|-------------|
| `standard` | Standard | false | Regular keyword (no special format) |
| `question` | Question | false | Question-form keyword (how, what, why...) |
| `comparison` | Comparison | **true** | X vs Y (links to 2 entities via COMPARES_A/B) |
| `preposition` | Preposition | **true** | X for Y (links to use-case entity) |
| `long_tail` | Long-tail | false | 4+ words, specific intent |
| `brand` | Brand | **true** | Contains brand name (links to brand entity) |
| `local` | Local | false | Contains location ("near me", city name) |

### Arc: HAS_FORMAT

```
SEOKeyword ──[:HAS_FORMAT]──► SEOKeywordFormat
```

- **Scope**: intra_realm (shared → shared)
- **Family**: ownership
- **Cardinality**: many_to_one (many keywords can have same format)

### Format-Specific Properties on SEOKeyword

When a keyword has a specific format, these properties are populated:

| Property | Type | When Used | Description |
|----------|------|-----------|-------------|
| `question_word` | string | format=question | what, how, why, when, where, who, can, is, does |
| `answer_type` | string | format=question | explanation, list, yes_no, steps, comparison, definition |
| `comparison_type` | string | format=comparison | vs, or, versus, compared_to, better_than |
| `entity_a_key` | string | format=comparison | Key of left-side Entity (X in "X vs Y") |
| `entity_b_key` | string | format=comparison | Key of right-side Entity (Y in "X vs Y") |
| `preposition` | string | format=preposition | for, with, without, near, to, like, in |
| `use_case` | string | format=preposition | The use case or context |

### Format-Specific Arcs (on SEOKeyword)

These arcs exist only when the format requires entity links:

```
# When format = comparison
SEOKeyword ──[:COMPARES_A]──► Entity  (left side: "QR Code" in "QR Code vs Barcode")
SEOKeyword ──[:COMPARES_B]──► Entity  (right side: "Barcode" in "QR Code vs Barcode")

# When format = preposition
SEOKeyword ──[:USE_CASE_FOR]──► Entity  ("restaurants" in "QR code for restaurants")

# When format = brand
SEOKeyword ──[:MENTIONS_BRAND]──► Entity  (brand entity mentioned)
```

### Why This Pattern (vs Separate Nodes)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  BEFORE (v11.3): Separate nodes                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  SEOKeyword (base) + SEOQuestion + SEOComparison + SEOPreposition           │
│  └─ 4 NodeKinds, duplicated properties, inconsistent arcs                   │
├─────────────────────────────────────────────────────────────────────────────┤
│  AFTER (v11.4): EntityCategory pattern                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│  SEOKeyword ──[:HAS_FORMAT]──► SEOKeywordFormat                             │
│  └─ 1 NodeKind + 1 Classification, extensible, consistent with ADR-017     │
│                                                                             │
│  ✅ Extensible: Add "featured_snippet" format without schema changes        │
│  ✅ Consistent: Same pattern as Entity → EntityCategory                     │
│  ✅ Queryable: MATCH (k)-[:HAS_FORMAT]->(f {format_key: "question"})        │
│  ✅ Simpler: 1 keyword node with conditional properties                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. GEOQuery Properties (STATIC - shared/knowledge)

> **Principle**: GEOQuery = STATIC query identity. All visibility data lives in GEOAnswer snapshots.

### Core Identity (Immutable)

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `value` | string | ✓ | Query text ("best QR code generators 2026") |
| `query_type` | enum | ✓ | definition, how_to, comparison, recommendation, troubleshooting, opinion |
| `language_hint` | string | | ISO 639-1 code (fr, en, de) |

### Target Platforms (Stable)

| Property | Type | Description |
|----------|------|-------------|
| `platforms` | string[] | Target AI engines: chatgpt, perplexity, gemini, copilot, claude |

### ⚠️ MOVED TO GEOAnswer (Time-Series Snapshots)

These properties change per answer and belong in GEOAnswer:
- ~~ai_search_volume~~ → Aggregated from GEOAnswer count per period
- ~~citation_rate~~ → `GEOAnswer.is_cited` (boolean per answer)
- ~~answer_inclusion_rate~~ → `GEOAnswer.mentions_entity` (boolean per answer)
- ~~source_rank~~ → `GEOAnswer.citation_position` (per answer)
- ~~mention_sentiment~~ → `GEOAnswer.sentiment` (per answer)
- ~~platform_visibility~~ → `GEOAnswer.engine` + `GEOAnswer.visibility_score`

---

## 4b. GEOAnswer Properties (TIME-SERIES - shared/knowledge)

> **Principle**: GEOAnswer IS the time-series. Each answer IS a snapshot — no separate GEOMetrics.

### Observation Metadata

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `observed_at` | datetime | ✓ | When this answer was captured (ISO 8601) |
| `engine` | string | ✓ | chatgpt, perplexity, gemini, copilot, claude |
| `engine_version` | string | | gpt-4o, claude-3.5-sonnet, gemini-2.0-flash |
| `query_variant` | string | | Exact query string used (may differ from GEOQuery.value) |

### Answer Content

| Property | Type | Description |
|----------|------|-------------|
| `response_text` | string | Full answer text (for analysis) |
| `response_length` | int | Character count |
| `cited_urls` | string[] | URLs cited in the answer |
| `cited_sources` | string[] | Source names mentioned |

### Visibility Metrics (Per Answer)

| Property | Type | Description |
|----------|------|-------------|
| `mentions_entity` | boolean | Does answer mention our entity? |
| `is_cited` | boolean | Are we in cited_urls? |
| `citation_position` | int | Position in citations (1-N, null if not cited) |
| `sentiment` | enum | positive, neutral, negative |
| `visibility_score` | float | 0.0-1.0 composite score |

### Content Analysis

| Property | Type | Description |
|----------|------|-------------|
| `competitor_mentions` | string[] | Competitor names mentioned |
| `recommendation_rank` | int | Position in recommendations (1-5) |
| `is_primary_recommendation` | boolean | Are we the #1 recommendation? |

---

## 5. ~~GEOMetrics~~ (REMOVED)

> **Decision**: GEOAnswer IS the time-series. No separate GEOMetrics node needed.
>
> Unlike SEO (where keyword metrics like volume/difficulty are universal and change over time),
> GEO data is inherently per-answer. Each AI response is a unique snapshot.
>
> **Aggregated metrics** (citation_rate over 30 days, average visibility_score) are COMPUTED
> from GEOAnswer nodes via Cypher queries, not stored as separate nodes.

---

## 6. Graph Structure

### SHARED Realm (Universal Knowledge)

```
SEOKeywordFormat (shared/config, invariant)
├── standard, question, comparison, preposition, long_tail, brand, local

Locale (shared/locale)
   │
   ├──[:HAS_SEO_KEYWORDS]──► SEOKeywordSet (shared/knowledge, 1:1)
   │                              │  ├─ last_sync: datetime
   │                              │  ├─ data_source: dataforseo
   │                              │  └─ total_keywords: int
   │                              │
   │                              └──[:CONTAINS_SEO_KEYWORD]──► SEOKeyword
   │                                       │  ├─ value, intent
   │                                       │  ├─ question_word (nullable)
   │                                       │  ├─ comparison_type, entity_a_key, entity_b_key (nullable)
   │                                       │  └─ preposition, use_case (nullable)
   │                                       │
   │                                       ├──[:HAS_FORMAT]──► SEOKeywordFormat (invariant)
   │                                       │
   │                                       ├──[:HAS_SEO_KEYWORD_METRICS]──► SEOKeywordMetrics (time-series)
   │                                       │        └─ observed_at, monthly_search_volume, ranking_difficulty...
   │                                       │
   │                                       ├──[:COMPARES_A]──► Entity (when format=comparison)
   │                                       ├──[:COMPARES_B]──► Entity (when format=comparison)
   │                                       ├──[:USE_CASE_FOR]──► Entity (when format=preposition)
   │                                       └──[:MENTIONS_BRAND]──► Entity (when format=brand)
   │
   └──[:HAS_GEO_QUERIES]──► GEOQuerySet (shared/knowledge, 1:1)
                                │  ├─ last_sync: datetime
                                │  └─ total_queries: int
                                │
                                └──[:CONTAINS_GEO_QUERY]──► GEOQuery
                                         │  ├─ value, query_type
                                         │  └─ platforms, language_hint
                                         │
                                         └──[:HAS_GEO_ANSWER]──► GEOAnswer (time-series)
                                                  └─ observed_at, engine, visibility_score...
```

> **Arc Naming Convention** (ADR-016 compliant):
> - Type-specific: `HAS_SEO_KEYWORD_METRICS` not `HAS_METRICS`
> - LLM-readable: Arc name reveals source AND target type
> - No ambiguity: Graph queries are self-documenting

### Cross-Realm Arcs (ORG → SHARED)

```
EntityContent (org/semantic)
     │
     ├──[:TARGETS {priority, target_position}]──► SEOKeyword (shared/knowledge)
     │
     └──[:MONITORS_GEO {priority, alert_threshold}]──► GEOQuery (shared/knowledge)
```

> **Removed arcs** (consolidated into SEOKeyword):
> - ~~ANSWERS~~ → SEOKeyword with format=question now has all question properties
> - ~~ADDRESSES~~ → SEOKeyword with format=comparison/preposition links to entities directly

**Key insight**: Org-specific data lives on ARC PROPERTIES, not in shared nodes.

---

## 6b. Node Schema Reference (Complete)

> **This section is the DEFINITIVE reference for all v11.4 SEO/GEO nodes.**

### Trait Assignment Rationale

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  TRAIT SEMANTICS (v11.4 clarification)                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  invariant   = Same everywhere, no locale variation                            │
│                Examples: Entity, Page, SEOKeywordFormat, EntityCategory         │
│                                                                                 │
│  localized   = Content GENERATED per locale from invariant parent              │
│                Examples: EntityContent (from Entity), ProjectContent            │
│                Pattern: Invariant ──[:HAS_CONTENT]──► Localized                │
│                                                                                 │
│  knowledge   = Locale-native DISCOVERED data (not generated)                   │
│                Examples: Term, Expression, SEOKeyword, GEOQuery                 │
│                Pattern: Locale ──[:HAS_*]──► Container ──[:CONTAINS]──► Atom   │
│                Key: No invariant parent, data is external/curated              │
│                                                                                 │
│  generated   = LLM OUTPUT (pages, blocks, artifacts)                           │
│                Examples: PageGenerated, BlockGenerated                          │
│                                                                                 │
│  aggregated  = TIME-SERIES snapshots, computed metrics                         │
│                Examples: SEOKeywordMetrics, GEOAnswer                           │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│  WHY SEOKeyword = knowledge (not localized)?                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. DISCOVERED, not created                                                     │
│     └── Keywords come from DataForSEO/Semrush, not LLM generation              │
│     └── Like Term/Expression: we observe what exists, don't create it          │
│                                                                                 │
│  2. NO INVARIANT PARENT                                                         │
│     └── EntityContent has Entity as parent → localized                         │
│     └── SEOKeyword has NO parent → knowledge atom                              │
│                                                                                 │
│  3. CONTAINER PATTERN                                                           │
│     └── Locale → SEOKeywordSet → SEOKeyword (matches Knowledge Atoms)          │
│     └── Same as: Locale → TermSet → Term                                       │
│                                                                                 │
│  4. MARKET INTELLIGENCE = LOCALE KNOWLEDGE                                      │
│     └── "What French users search" = knowledge about French market             │
│     └── Like "How French people speak" = knowledge about French language       │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Complete Node Definitions

#### SEOKeywordFormat (Classification)

```yaml
# packages/core/models/node-classes/shared/config/seo-keyword-format.yaml
node:
  name: SEOKeywordFormat
  realm: shared
  layer: config
  trait: invariant  # Universal classification, same everywhere

  description: |
    Keyword format classification following EntityCategory pattern (ADR-017).
    7 invariant instances: standard, question, comparison, preposition,
    long_tail, brand, local.

  properties:
    format_key:
      type: string
      required: true
      description: "Unique format identifier"
    display_name:
      type: string
      required: true
      description: "Human-readable name"
    description:
      type: string
      description: "LLM context for this format"
    has_entity_links:
      type: boolean
      default: false
      description: "Whether this format can link to entities"

  llm_context: |
    SEOKeywordFormat classifies keywords by their structural format.
    Use HAS_FORMAT arc from SEOKeyword to determine format-specific behavior.
    Formats with has_entity_links=true have COMPARES_A/B or USE_CASE_FOR arcs.
```

#### SEOKeywordSet (Container)

```yaml
# packages/core/models/node-classes/shared/knowledge/seo-keyword-set.yaml
node:
  name: SEOKeywordSet
  realm: shared
  layer: knowledge
  trait: invariant  # Container is universal category, not locale-native

  description: |
    Container for SEO keywords belonging to a locale.
    One SEOKeywordSet per Locale (1:1 cardinality).
    Follows Knowledge Atoms container pattern.

  properties:
    last_sync:
      type: datetime
      description: "Last DataForSEO sync timestamp"
    data_source:
      type: string
      default: "dataforseo"
      description: "Source API: dataforseo, semrush, ahrefs"
    total_keywords:
      type: int
      description: "Count of keywords in this set"

  llm_context: |
    SEOKeywordSet groups all SEO keywords for a locale.
    Access via: (Locale)-[:HAS_SEO_KEYWORDS]->(SEOKeywordSet)
    Then: (SEOKeywordSet)-[:CONTAINS_SEO_KEYWORD]->(SEOKeyword)
```

#### SEOKeyword (Knowledge Atom)

```yaml
# packages/core/models/node-classes/shared/knowledge/seo-keyword.yaml
node:
  name: SEOKeyword
  realm: shared
  layer: knowledge
  trait: knowledge  # Discovered market data, locale-native

  description: |
    SEO keyword with static identity and format-specific properties.
    Time-series metrics live in SEOKeywordMetrics (aggregated trait).
    Format classification via HAS_FORMAT arc to SEOKeywordFormat.

  properties:
    # Core Identity (Immutable)
    value:
      type: string
      required: true
      description: "Keyword string (e.g., 'créer qr code gratuit')"
    intent:
      type: string
      required: true
      enum: [informational, navigational, commercial, transactional]
      description: "Search intent classification"
    platform:
      type: string
      description: "google, bing, youtube"
    language_hint:
      type: string
      description: "ISO 639-1 code (fr, en, de)"

    # Stable Characteristics
    competition_level:
      type: string
      enum: [low, medium, high]
      description: "Stable competition category"
    categories:
      type: int[]
      description: "Google Ads category IDs"
    seasonality_pattern:
      type: int[]
      description: "12-month index [80,85,90,100...]"

    # Format-Specific (nullable, depends on HAS_FORMAT)
    question_word:
      type: string
      description: "what, how, why... (when format=question)"
    answer_type:
      type: string
      description: "explanation, list, yes_no... (when format=question)"
    comparison_type:
      type: string
      description: "vs, or, versus... (when format=comparison)"
    entity_a_key:
      type: string
      description: "Left entity key (when format=comparison)"
    entity_b_key:
      type: string
      description: "Right entity key (when format=comparison)"
    preposition:
      type: string
      description: "for, with, near... (when format=preposition)"
    use_case:
      type: string
      description: "Use case context (when format=preposition)"

  llm_context: |
    SEOKeyword represents market search behavior for a locale.
    It is DISCOVERED data from DataForSEO, not generated content.
    Use to understand what users search for, then TARGET from EntityContent.
    Format determines which optional properties are populated.
    Time-series metrics: follow HAS_SEO_KEYWORD_METRICS arc.
```

#### SEOKeywordMetrics (Time-Series)

```yaml
# packages/core/models/node-classes/shared/knowledge/seo-keyword-metrics.yaml
node:
  name: SEOKeywordMetrics
  realm: shared
  layer: knowledge
  trait: aggregated  # Time-series snapshots

  description: |
    Immutable snapshot of SEO keyword metrics at a point in time.
    Each observation creates a new node (append-only pattern).

  properties:
    # Observation Metadata
    observed_at:
      type: datetime
      required: true
      description: "When metrics were captured (ISO 8601)"
    source:
      type: string
      required: true
      description: "dataforseo, semrush, ahrefs"
    observation_period:
      type: string
      description: "ISO 8601 duration (P1M = monthly)"

    # Volume & Traffic
    monthly_search_volume:
      type: int
      description: "Monthly average searches"
    estimated_clicks:
      type: int
      description: "Monthly clicks (actual)"
    clicks_per_search:
      type: float
      description: "Click satisfaction ratio"
    traffic_potential:
      type: int
      description: "Est. traffic if ranking #1"

    # Competition & Difficulty
    ranking_difficulty:
      type: int
      description: "0-100 organic ranking difficulty"
    organic_competition:
      type: float
      description: "0.0-1.0 competition level"
    cost_per_click:
      type: float
      description: "CPC in USD"
    low_top_of_page_bid:
      type: float
      description: "Min bid for top SERP"
    high_top_of_page_bid:
      type: float
      description: "Max bid for top SERP"

    # SERP Features
    serp_features:
      type: string[]
      description: "featured_snippet, ai_overview, people_also_ask..."
    serp_result_count:
      type: int
      description: "Total search results"

    # Trend Deltas
    volume_change_7d:
      type: float
      description: "% change vs 7 days ago"
    volume_change_30d:
      type: float
      description: "% change vs 30 days ago"

  llm_context: |
    SEOKeywordMetrics is a time-series snapshot of keyword performance.
    Query latest: ORDER BY observed_at DESC LIMIT 1
    Query trend: multiple nodes over time for the same keyword.
    Immutable: never update, always append new observations.
```

#### GEOQuerySet (Container)

```yaml
# packages/core/models/node-classes/shared/knowledge/geo-query-set.yaml
node:
  name: GEOQuerySet
  realm: shared
  layer: knowledge
  trait: invariant  # Container is universal category

  description: |
    Container for GEO queries belonging to a locale.
    One GEOQuerySet per Locale (1:1 cardinality).

  properties:
    last_sync:
      type: datetime
      description: "Last AI visibility check timestamp"
    total_queries:
      type: int
      description: "Count of queries in this set"

  llm_context: |
    GEOQuerySet groups all GEO queries for a locale.
    Access: (Locale)-[:HAS_GEO_QUERIES]->(GEOQuerySet)-[:CONTAINS_GEO_QUERY]->(GEOQuery)
```

#### GEOQuery (Knowledge Atom)

```yaml
# packages/core/models/node-classes/shared/knowledge/geo-query.yaml
node:
  name: GEOQuery
  realm: shared
  layer: knowledge
  trait: knowledge  # Discovered AI query patterns, locale-native

  description: |
    GEO query for AI visibility tracking. STATIC identity.
    Time-series visibility data lives in GEOAnswer (aggregated trait).

  properties:
    # Core Identity
    value:
      type: string
      required: true
      description: "Query text (e.g., 'best QR code generators 2026')"
    query_type:
      type: string
      required: true
      enum: [definition, how_to, comparison, recommendation, troubleshooting, opinion]
      description: "Query intent classification"
    language_hint:
      type: string
      description: "ISO 639-1 code"

    # Target Platforms
    platforms:
      type: string[]
      description: "chatgpt, perplexity, gemini, copilot, claude"

  llm_context: |
    GEOQuery represents what users ask AI engines in a locale.
    STATIC identity: query text and type don't change.
    Visibility metrics: follow HAS_GEO_ANSWER arc to GEOAnswer snapshots.
    Monitor from EntityContent via MONITORS_GEO arc.
```

#### GEOAnswer (Time-Series)

```yaml
# packages/core/models/node-classes/shared/knowledge/geo-answer.yaml
node:
  name: GEOAnswer
  realm: shared
  layer: knowledge
  trait: aggregated  # Time-series snapshots

  description: |
    Immutable snapshot of an AI engine's answer to a GEOQuery.
    Each observation creates a new node. GEOAnswer IS the time-series.

  properties:
    # Observation Metadata
    observed_at:
      type: datetime
      required: true
      description: "When this answer was captured"
    engine:
      type: string
      required: true
      description: "chatgpt, perplexity, gemini, copilot, claude"
    engine_version:
      type: string
      description: "gpt-4o, claude-3.5-sonnet, gemini-2.0-flash"
    query_variant:
      type: string
      description: "Exact query string used (may differ from GEOQuery.value)"

    # Answer Content
    response_text:
      type: string
      description: "Full answer text (for analysis)"
    response_length:
      type: int
      description: "Character count"
    cited_urls:
      type: string[]
      description: "URLs cited in the answer"
    cited_sources:
      type: string[]
      description: "Source names mentioned"

    # Visibility Metrics
    mentions_entity:
      type: boolean
      description: "Does answer mention our entity?"
    is_cited:
      type: boolean
      description: "Are we in cited_urls?"
    citation_position:
      type: int
      description: "Position in citations (1-N, null if not cited)"
    sentiment:
      type: string
      enum: [positive, neutral, negative]
      description: "Sentiment toward our entity"
    visibility_score:
      type: float
      description: "0.0-1.0 composite score"

    # Content Analysis
    competitor_mentions:
      type: string[]
      description: "Competitor names mentioned"
    recommendation_rank:
      type: int
      description: "Position in recommendations (1-5)"
    is_primary_recommendation:
      type: boolean
      description: "Are we the #1 recommendation?"

  llm_context: |
    GEOAnswer is a snapshot of an AI engine's response.
    Each answer IS a data point (no separate GEOMetrics needed).
    Query trend: multiple GEOAnswer nodes over time.
    Aggregate visibility: COMPUTE via Cypher, not stored.
```

#### EntityCategory (MOVED to shared/config)

```yaml
# packages/core/models/node-classes/shared/config/entity-category.yaml
# MOVED from org/config in v11.4
node:
  name: EntityCategory
  realm: shared  # v11.4: moved from org (universal categories)
  layer: config
  trait: invariant  # Universal classification

  description: |
    Entity category classification (ADR-017).
    13 invariant instances: thing, content_type, place, person,
    organization, event, concept, product, service, resource,
    media, document, abstract.

  properties:
    category_key:
      type: string
      required: true
      description: "Unique category identifier"
    display_name:
      type: string
      required: true
      description: "Human-readable name"
    description:
      type: string
      description: "LLM context for this category"

  llm_context: |
    EntityCategory provides universal classification for Entity nodes.
    Categories are Schema.org/Wikidata aligned (universal, not org-specific).
    Entity links via: (Entity)-[:BELONGS_TO]->(EntityCategory)
```

### Trait Summary Table

| Node | Realm | Layer | Trait | Rationale |
|------|-------|-------|-------|-----------|
| **SEOKeywordFormat** | shared | config | `invariant` | Universal classification (7 instances) |
| **EntityCategory** | shared | config | `invariant` | Universal classification (13 instances) |
| **SEOKeywordSet** | shared | knowledge | `invariant` | Container (universal category) |
| **GEOQuerySet** | shared | knowledge | `invariant` | Container (universal category) |
| **SEOKeyword** | shared | knowledge | `knowledge` | Discovered market data, locale-native |
| **GEOQuery** | shared | knowledge | `knowledge` | Discovered AI patterns, locale-native |
| **SEOKeywordMetrics** | shared | knowledge | `aggregated` | Time-series snapshots |
| **GEOAnswer** | shared | knowledge | `aggregated` | Time-series snapshots |

### Arc Summary Table

| Arc | Source | Target | Family | Scope | Cardinality |
|-----|--------|--------|--------|-------|-------------|
| `HAS_SEO_KEYWORDS` | Locale | SEOKeywordSet | ownership | intra_realm | 1:1 |
| `CONTAINS_SEO_KEYWORD` | SEOKeywordSet | SEOKeyword | ownership | intra_realm | 1:N |
| `HAS_FORMAT` | SEOKeyword | SEOKeywordFormat | ownership | intra_realm | N:1 |
| `HAS_SEO_KEYWORD_METRICS` | SEOKeyword | SEOKeywordMetrics | ownership | intra_realm | 1:N |
| `COMPARES_A` | SEOKeyword | Entity | semantic | cross_realm | N:1 |
| `COMPARES_B` | SEOKeyword | Entity | semantic | cross_realm | N:1 |
| `USE_CASE_FOR` | SEOKeyword | Entity | semantic | cross_realm | N:1 |
| `MENTIONS_BRAND` | SEOKeyword | Entity | semantic | cross_realm | N:1 |
| `TARGETS` | EntityContent | SEOKeyword | semantic | cross_realm | N:M |
| `HAS_GEO_QUERIES` | Locale | GEOQuerySet | ownership | intra_realm | 1:1 |
| `CONTAINS_GEO_QUERY` | GEOQuerySet | GEOQuery | ownership | intra_realm | 1:N |
| `HAS_GEO_ANSWER` | GEOQuery | GEOAnswer | ownership | intra_realm | 1:N |
| `MONITORS_GEO` | EntityContent | GEOQuery | semantic | cross_realm | N:M |
| `BELONGS_TO` | Entity | EntityCategory | ownership | cross_realm | N:1 |

---

## 7. Migration Plan

> **Detailed Implementation**: See `2026-02-10-v11.4-implementation-plan.md` for step-by-step execution guide.

### Phase 1: Create shared/config Classifications

**Create directory:** `packages/core/models/node-classes/shared/config/`

1. Create `SEOKeywordFormat` node YAML (`shared/config/seo-keyword-format.yaml`)
   - realm: shared, layer: config, trait: invariant
   - 7 instances: standard, question, comparison, preposition, long_tail, brand, local

2. **MOVE** `EntityCategory` from org/config → shared/config
   - `org/config/entity-category.yaml` → `shared/config/entity-category.yaml`
   - Update realm: org → shared (categories are universal, not org-specific)
   - Update BELONGS_TO arc scope: intra_realm → cross_realm

### Phase 2: Create Container Nodes in shared/knowledge

1. Create `SEOKeywordSet` node YAML (`shared/knowledge/seo-keyword-set.yaml`)
2. Create `GEOQuerySet` node YAML (`shared/knowledge/geo-query-set.yaml`)

### Phase 3: Move & Consolidate SEO Nodes

**Move and update:**
1. `seo-keyword.yaml` → `shared/knowledge/seo-keyword.yaml`
   - Add format-specific properties (question_word, comparison_type, etc.)
   - Add HAS_FORMAT arc to SEOKeywordFormat
2. `seo-keyword-metrics.yaml` → `shared/knowledge/seo-keyword-metrics.yaml`

**DELETE (consolidated into SEOKeyword):**
3. **DELETE** `seo-question.yaml` → properties merged into SEOKeyword
4. **DELETE** `seo-comparison.yaml` → properties merged into SEOKeyword
5. **DELETE** `seo-preposition.yaml` → properties merged into SEOKeyword

### Phase 4: Move GEO Nodes (org/geo → shared/knowledge)

1. `geo-query.yaml` → `shared/knowledge/geo-query.yaml`
2. `geo-answer.yaml` → `shared/knowledge/geo-answer.yaml`
3. **DELETE** `geo-metrics.yaml` — NOT needed, GEOAnswer IS the time-series

### Phase 5: Create/Update Arcs

**New arcs (intra_realm, ownership):**
1. `HAS_SEO_KEYWORDS`: Locale → SEOKeywordSet
2. `CONTAINS_SEO_KEYWORD`: SEOKeywordSet → SEOKeyword
3. `HAS_GEO_QUERIES`: Locale → GEOQuerySet
4. `CONTAINS_GEO_QUERY`: GEOQuerySet → GEOQuery
5. `HAS_FORMAT`: SEOKeyword → SEOKeywordFormat

**New arcs (semantic, conditional on format):**
6. `COMPARES_A`: SEOKeyword → Entity (when format=comparison)
7. `COMPARES_B`: SEOKeyword → Entity (when format=comparison)
8. `USE_CASE_FOR`: SEOKeyword → Entity (when format=preposition)
9. `MENTIONS_BRAND`: SEOKeyword → Entity (when format=brand)

**Update scope (intra_realm → cross_realm):**
10. `TARGETS`: EntityContent (org) → SEOKeyword (shared)
11. `MONITORS_GEO`: EntityContent (org) → GEOQuery (shared)

**DELETE (consolidated):**
12. **DELETE** `ANSWERS` arc → replaced by TARGETS to question-format keywords
13. **DELETE** `ADDRESSES` arc → replaced by direct entity links on keyword
14. **DELETE** `HAS_SEO_QUESTION`, `HAS_SEO_COMPARISON`, `HAS_SEO_PREPOSITION` arcs

### Phase 6: Update Taxonomy

1. Remove `seo` layer from org realm
2. Remove `geo` layer from org realm
3. Update layer count: org now has 6 layers (was 8)

### Phase 7: Regenerate & Validate

```bash
cargo run -- schema generate
cargo run -- schema validate --strict
cargo nextest run
pnpm type-check
```

### Phase 8: Documentation Cleanup

> **CRITICAL**: Update ALL documentation to reflect v11.4 changes.

**Source of Truth:**
1. `packages/core/models/node-classes/shared/knowledge/` — SEO/GEO YAML definitions
2. `packages/core/models/arc-classes/` — New arc definitions
3. `packages/core/models/taxonomy.yaml` — Layer updates (remove org/seo, org/geo)

**CLAUDE.md Updates:**
1. Root `CLAUDE.md` — Update layer count, realm architecture
2. `tools/novanet/CLAUDE.md` — Update node counts, TUI filtering
3. `.claude/rules/novanet-decisions.md` — Add ADR-018 (SEO/GEO to Shared)
4. `.claude/rules/novanet-terminology.md` — Update layer list, add SEO/GEO terms

**Skills & DX:**
1. `/schema:add-node` — Verify works for shared/knowledge realm
2. `/novanet-arch` — Update architecture diagram (no org/seo, org/geo layers)
3. `/novanet-sync` — Test regeneration with new structure

**Diagram Updates:**
1. `packages/core/models/docs/views/*.mermaid` — Regenerate all 11 views
2. `tools/novanet/src/tui/guide/` — Update TUI help text

**Verification Checklist:**
- [ ] SEOKeywordFormat in `shared/config/` (7 instances)
- [ ] SEOKeyword, SEOKeywordSet, SEOKeywordMetrics in `shared/knowledge/`
- [ ] GEOQuery, GEOQuerySet, GEOAnswer in `shared/knowledge/`
- [ ] SEOQuestion, SEOComparison, SEOPreposition DELETED
- [ ] GEOMetrics DELETED
- [ ] Type-specific arc names (`HAS_SEO_KEYWORD_METRICS`, `HAS_FORMAT`, etc.)
- [ ] No references to `org/seo` or `org/geo` anywhere
- [ ] `cargo run -- blueprint` shows correct layer structure
- [ ] TUI filters work with new realm/layer
- [ ] EntityCategory pattern for SEOKeywordFormat validated

---

## 8. Open Questions (RESOLVED)

### ✅ Metrics Location (RESOLVED)
**Decision**: ALL SEO/GEO nodes in `shared` realm

**SEO Nodes (4 total):**
- SEOKeywordFormat → **shared/config** (invariant, 7 instances)
- SEOKeywordSet, SEOKeyword, SEOKeywordMetrics → **shared/knowledge**
- ~~SEOQuestion, SEOComparison, SEOPreposition~~ → **REMOVED** (consolidated into SEOKeyword)

**GEO Nodes (3 total):**
- GEOQuerySet, GEOQuery, GEOAnswer → **shared/knowledge**
- ~~GEOMetrics~~ → **REMOVED** (GEOAnswer IS the time-series)

**Total: 7 NodeKinds** (4 SEO + 3 GEO), down from 11

Org-specific data → on TARGETS arc properties or external tool (GSC)

### ✅ Container Grouping (RESOLVED)
**Decision**: ONE SEOKeywordSet per Locale (not grouped by intent)
- Intent is a **property** on SEOKeyword (flexible compound queries)
- Container provides sync metadata: `last_sync`, `data_source`, `total_keywords`
- Follows ExpressionSet pattern (1:1 Locale:Set cardinality)

### ✅ Variant Nodes (RESOLVED - UPDATED)
**Decision**: Consolidate into SEOKeyword + SEOKeywordFormat (EntityCategory pattern)
- SEOQuestion, SEOComparison, SEOPreposition → **REMOVED**
- SEOKeywordFormat (invariant) with 7 format types: standard, question, comparison, preposition, long_tail, brand, local
- SEOKeyword has format-specific properties (nullable): question_word, comparison_type, preposition, etc.
- Format-specific arcs: COMPARES_A/B (when comparison), USE_CASE_FOR (when preposition), MENTIONS_BRAND (when brand)

**Rationale**:
- Follows EntityCategory pattern (ADR-017)
- Extensible: add formats without schema changes
- Simpler: 1 keyword node with conditional properties
- Industry standard: DataForSEO/Ahrefs/Semrush don't have separate "question keyword" tables

### ✅ Cross-Realm Arcs (RESOLVED)
**Decision**: TARGETS and MONITORS_GEO become `cross_realm`
- EntityContent (org/semantic) → SEOKeyword (shared/knowledge) via `:TARGETS`
- EntityContent (org/semantic) → GEOQuery (shared/knowledge) via `:MONITORS_GEO`
- Arc properties for org-specific data: `priority`, `target_position`, `alert_threshold`

### ✅ Org-Specific Ranking Data (RESOLVED)
**Decision**: Store on TARGETS arc, NOT in SEOKeywordMetrics
- SEOKeywordMetrics = universal time-series (volume, difficulty from DataForSEO)
- TARGETS arc properties = org-specific targeting (`priority`, `target_position`)
- GSC data (impressions, clicks, CTR) → external tool or future PageMetrics

### ✅ Property Deduplication (RESOLVED)
**Decision**: SEOKeyword = STATIC, SEOKeywordMetrics = TIME-SERIES
- SEOKeyword has: `value`, `intent`, `platform`, `language_hint`, `competition_level`, `categories`, `seasonality_pattern`
- SEOKeywordMetrics has: ALL changing numbers (`monthly_search_volume`, `ranking_difficulty`, `cost_per_click`, `serp_features`)
- serp_features MOVED to Metrics (they appear/disappear over time!)

### ✅ GSC Data Location (RESOLVED)
**Decision**: Option C — NOT in NovaNet (external tool)
- SEOKeywordMetrics = universal DataForSEO data (same for everyone)
- GSC data (position, CTR, impressions) = org-specific, but NOT stored in NovaNet
- Rationale: GSC requires Google OAuth, complex auth, frequent polling — better as external service
- Future: If needed, create PageMetrics node in org realm for website performance

### ✅ GEOAnswer vs GEOMetrics (RESOLVED)
**Decision**: GEOAnswer IS the time-series. NO separate GEOMetrics.
- Each AI response is a unique snapshot with timestamp
- Aggregated metrics (citation_rate, avg_visibility) are COMPUTED via Cypher, not stored
- Simpler model: GEOQuery → GEOAnswer (many, time-series)

### ✅ Platform Tracking (RESOLVED)
**Decision**: Separate properties per answer, NOT separate nodes
- `GEOAnswer.engine`: chatgpt, perplexity, gemini, copilot, claude
- `GEOAnswer.engine_version`: gpt-4o, claude-3.5-sonnet, etc.
- Rationale: One answer = one engine. Multiple engines = multiple GEOAnswer nodes.

### ✅ Arc Naming (RESOLVED)
**Decision**: Type-specific names (ADR-016 compliant)
- `HAS_SEO_KEYWORD_METRICS` (not HAS_METRICS)
- `HAS_GEO_ANSWER` (not HAS_ANSWERS)
- `CONTAINS_SEO_KEYWORD` (not CONTAINS_KEYWORD)
- `CONTAINS_GEO_QUERY` (not CONTAINS_QUERY)
- Rationale: LLM-friendly, self-documenting graph queries

---

## 9. Deferred to v11.5+ (Backlog)

### ⏸️ GEOQueryFormat Classification

**Status**: DEFERRED (validated for future implementation)

**Decision**: Add GEOQueryFormat as separate classification from SEOKeywordFormat.

**Rationale** (from Socratic brainstorm):
- Query types encode different information than keyword formats
- SEOKeywordFormat = structural format (question, comparison, preposition)
- GEOQueryFormat = query intent type (definition, how_to, comparison, recommendation, troubleshooting, opinion)
- Separate classifications allow independent evolution

**Proposed Structure** (v11.5):
```yaml
GEOQueryFormat (shared/config, invariant)
├── definition      # "What is X?"
├── how_to          # "How to X?"
├── comparison      # "X vs Y" or "Best X"
├── recommendation  # "Suggest X for Y"
├── troubleshooting # "X not working"
└── opinion         # "Is X worth it?"
```

**Arc**: `GEOQuery ──[:HAS_QUERY_TYPE]──► GEOQueryFormat`

**Why deferred**:
- v11.4 scope is already significant (realm migration + consolidation)
- GEO is newer than SEO, less mature patterns
- Can validate SEOKeywordFormat pattern first, then apply to GEO

---

### ⏸️ SEO↔GEO Intent Linking

**Status**: DEFERRED (validated for future implementation)

**Decision**: Add semantic arcs to link keywords and queries with same/related intent.

**Proposed Arcs** (v11.5):
```
SEOKeyword ──[:INTENT_EQUIVALENT]──► GEOQuery
  └── Same intent, different channel (search vs AI)

GEOQuery ──[:INTENT_SUPERSET]──► SEOKeyword
  └── AI query encompasses multiple keywords

SEOKeyword ──[:INTENT_EVOLVED]──► GEOQuery
  └── Traditional keyword evolved to AI query
```

**Use Cases**:
- Track keyword→query evolution over time
- Build unified intent graph across channels
- Optimize content for both SEO and GEO simultaneously

**Why deferred**:
- Requires understanding of real-world SEO↔GEO relationships first
- Better to observe patterns from data before encoding in schema
- v11.4 focuses on structural migration, v11.5 can add semantic richness

---

## 10. Exploration Findings (10-Agent Audit)

> **Date**: 2026-02-10
> **Status**: Analyzed, prioritized for v11.4 and v11.5+

### Overview

10 parallel agents explored connected nodes and arcs to find incoherences and improvements for the LLM/AI knowledge ontology.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  AGENT AUDIT SUMMARY                                                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Agent 1  │ Locale→SEO/GEO Path    │ ⚠️  Realm mismatch in YAML (to fix)       │
│  Agent 2  │ Entity→SEO Connection  │ ⚠️  Arc scope issues (to fix)              │
│  Agent 3  │ Knowledge Atoms        │ ⚠️  Arc scope bugs (to fix)                │
│  Agent 4  │ Cross-Realm Arcs       │ ✅  All 8 arcs CORRECT                      │
│  Agent 5  │ LLM Context Quality    │ ⚠️  3-9/10 scores, rewrites needed         │
│  Agent 6  │ Arc Naming Audit       │ ✅  92% LLM-friendly, 8 incomplete          │
│  Agent 7  │ Trait Coherence        │ ⚠️  GEOAnswer trait should be `generated`? │
│  Agent 8  │ Page→SEO Traversal     │ 📋  3 hops too long (v11.5 optimization)   │
│  Agent 9  │ Cypher Query Library   │ ✅  8 patterns documented                   │
│  Agent 10 │ Ontology Coherence     │ ⚠️  7/10 overall, SEO/GEO conflation        │
│                                                                                 │
│  Legend: ✅ = Validated, ⚠️ = Fix Required, 📋 = Backlog                        │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Prioritized Action List

#### P0: Must Fix in v11.4 (Before Migration)

| Finding | Agent | Action | Files |
|---------|-------|--------|-------|
| YAML realm mismatch | 1 | Update `realm: org` → `realm: shared` | All SEO/GEO YAMLs |
| TARGETS scope | 2 | Update `scope: intra_realm` → `cross_realm` | `targets.yaml` |
| MONITORS_GEO scope | 2 | Update `scope: intra_realm` → `cross_realm` | `monitors-geo.yaml` |
| HAS_SEO_KEYWORDS scope | 3 | Update `scope: cross_realm` → `intra_realm` | After migration |
| HAS_GEO_QUERIES scope | 3 | Update `scope: cross_realm` → `intra_realm` | After migration |

#### P1: Fix in v11.4 (During Migration)

| Finding | Agent | Action | Effort |
|---------|-------|--------|--------|
| LLM context rewrites | 5 | Improve llm_context for 6 SEO/GEO nodes | 2h |
| Arc llm_context | 6 | Add missing llm_context to 8 arcs | 1h |
| SEO semantic layer llm_context | 2 | Update to mention cross-realm TARGETS | 15min |

#### P2: Consider for v11.4 (Trait Debate)

| Finding | Agent | Decision Needed |
|---------|-------|-----------------|
| GEOAnswer trait | 7 | Is it `aggregated` (time-series) or `generated` (LLM output)? |

**Current decision**: `aggregated` is correct.
- GEOAnswer IS a time-series snapshot of observed AI responses
- The content is "aggregated" (captured from external AI engines), not "generated" by NovaNet
- `generated` trait is for content WE create (PageGenerated, BlockGenerated)
- GEOAnswer is data WE collect about what OTHER LLMs generate

#### P3: Backlog for v11.5+ (Optimizations)

| Finding | Agent | Description |
|---------|-------|-------------|
| Page→SEO traversal | 8 | 3 hops is long (Page→Block→Entity→SEOKeyword). Consider denormalization. |
| SEO↔GEO conflation | 10 | Consider splitting SEO and GEO into separate subsystems |
| Feedback loops | 8 | Add PageGenerated→SEOKeyword performance feedback |
| Missing indexes | 9 | Add Neo4j indexes for common Cypher patterns |
| SEMANTIC_LINK split | 10 | Split into 5 specific arc families (future) |

### Detailed Agent Findings

#### Agent 1: Locale→SEO/GEO Path Analysis

**Status**: YAML files still say `realm: org` (will be fixed in migration)

**Current path**: `Locale → (no direct path to SEO/GEO)`
**v11.4 path**: `Locale → HAS_SEO_KEYWORDS → SEOKeywordSet → CONTAINS_SEO_KEYWORD → SEOKeyword`

**Issues found**:
- SEOKeyword YAML says `realm: org` → must update to `realm: shared`
- Missing container pattern → SEOKeywordSet being added
- No direct Locale→SEOKeyword arc → adding HAS_SEO_KEYWORDS

**All addressed by v11.4 migration.**

#### Agent 2: Entity→EntityContent→SEO Connection

**Issues found**:
1. TARGETS arc has `scope: intra_realm` but will be cross-realm after migration
2. MONITORS_GEO arc has same issue
3. org/semantic layer llm_context doesn't mention TARGETS/MONITORS_GEO arcs

**Fixes**:
```yaml
# arc-classes/semantic/targets.yaml
arc:
  scope: cross_realm  # Changed from intra_realm

# arc-classes/semantic/monitors-geo.yaml
arc:
  scope: cross_realm  # Changed from intra_realm
```

#### Agent 3: Knowledge Atoms Pattern Coherence

**Validation**: Pattern is consistent with Term, Expression, Pattern atoms.

**Arc scope bugs after migration**:
- `HAS_SEO_KEYWORDS` will be intra_realm (Locale [shared] → SEOKeywordSet [shared])
- `HAS_GEO_QUERIES` will be intra_realm (same reason)

**Note**: Current YAMLs may say cross_realm because they haven't been migrated yet.

#### Agent 4: Cross-Realm Arc Validation

**Result**: All 8 cross-realm arcs are correctly identified and documented.

| Arc | Source (Realm) | Target (Realm) | Status |
|-----|----------------|----------------|--------|
| TARGETS | EntityContent (org) | SEOKeyword (shared) | ✅ Correct |
| MONITORS_GEO | EntityContent (org) | GEOQuery (shared) | ✅ Correct |
| COMPARES_A | SEOKeyword (shared) | Entity (org) | ✅ Correct |
| COMPARES_B | SEOKeyword (shared) | Entity (org) | ✅ Correct |
| USE_CASE_FOR | SEOKeyword (shared) | Entity (org) | ✅ Correct |
| MENTIONS_BRAND | SEOKeyword (shared) | Entity (org) | ✅ Correct |
| BELONGS_TO | Entity (org) | EntityCategory (shared) | ✅ Correct |
| FOR_LOCALE | EntityContent (org) | Locale (shared) | ✅ Correct |

#### Agent 5: LLM Context Quality Audit

**Scores** (10-point scale):

| Node | Score | Issue |
|------|-------|-------|
| SEOKeyword | 5/10 | Too technical, missing usage examples |
| SEOKeywordMetrics | 4/10 | No query patterns, no "when to use" |
| GEOQuery | 3/10 | Minimal context, no platform examples |
| GEOAnswer | 4/10 | Missing aggregation guidance |
| SEOKeywordFormat | 6/10 | Good structure, needs examples |
| EntityCategory | 7/10 | Clear but could be richer |
| Term | 9/10 | Good reference example |
| Expression | 8/10 | Good reference example |

**Recommended rewrites** (already in Section 6b with improved llm_context).

#### Agent 6: Arc Naming LLM-Friendliness

**Overall**: 92% of arc names are LLM-friendly (self-documenting).

**Missing llm_context** (8 arcs):
- HAS_SEO_KEYWORDS (new)
- CONTAINS_SEO_KEYWORD (new)
- HAS_FORMAT (new)
- HAS_SEO_KEYWORD_METRICS (new)
- COMPARES_A (new)
- COMPARES_B (new)
- USE_CASE_FOR (new)
- MENTIONS_BRAND (new)

**Note**: All new arcs need llm_context added during creation.

#### Agent 7: Trait Coherence Analysis

**Overall**: 98.4% trait assignments are correct.

**Debate**: GEOAnswer trait

| Option | Trait | Rationale |
|--------|-------|-----------|
| A | `aggregated` | Time-series snapshots of external data |
| B | `generated` | Content from LLM engines |

**Decision**: Keep `aggregated`.
- GEOAnswer captures what OTHER LLMs produce, not what WE generate
- Pattern matches SEOKeywordMetrics (external API data snapshots)
- `generated` is reserved for NovaNet's own LLM output

#### Agent 8: Page→SEO Traversal Analysis

**Current traversal** (3 hops):
```
Page → HAS_BLOCK → Block → USES_ENTITY → Entity → (EntityContent) → TARGETS → SEOKeyword
```

**Issues**:
1. Long traversal path for common query
2. No direct Page→SEOKeyword relationship
3. Missing feedback loop (SEO performance → Page optimization)

**Recommendations for v11.5**:
- Consider TARGETS_PAGE arc (denormalization)
- Add FEEDBACK arc for performance tracking
- Create materialized view for Page→SEOKeyword

#### Agent 9: Cypher Query Library

**8 documented patterns**:

1. **Keywords for Entity**: `MATCH (e:Entity {key: $key})-[:HAS_CONTENT]->(ec)-[:TARGETS]->(k:SEOKeyword)`
2. **Latest Metrics**: `MATCH (k:SEOKeyword)-[:HAS_SEO_KEYWORD_METRICS]->(m) ORDER BY m.observed_at DESC LIMIT 1`
3. **GEO Visibility**: `MATCH (q:GEOQuery)-[:HAS_GEO_ANSWER]->(a) WHERE a.visibility_score > 0.5`
4. **Format Distribution**: `MATCH (k:SEOKeyword)-[:HAS_FORMAT]->(f) RETURN f.format_key, count(k)`
5. **Cross-Realm Targeting**: `MATCH (ec:EntityContent)-[:TARGETS]->(k:SEOKeyword) WHERE ec.locale_key = k.language_hint`
6. **Question Keywords**: `MATCH (k:SEOKeyword)-[:HAS_FORMAT]->(f {format_key: 'question'}) RETURN k.value, k.question_word`
7. **Trend Analysis**: `MATCH (k:SEOKeyword)-[:HAS_SEO_KEYWORD_METRICS]->(m) WITH k, collect(m) as metrics`
8. **Category SEO Performance**: `MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory), (e)-[:HAS_CONTENT]->(ec)-[:TARGETS]->(k)`

**Index recommendations**:
- `CREATE INDEX seo_keyword_value FOR (k:SEOKeyword) ON (k.value)`
- `CREATE INDEX metrics_observed_at FOR (m:SEOKeywordMetrics) ON (m.observed_at)`
- `CREATE INDEX geo_answer_engine FOR (a:GEOAnswer) ON (a.engine)`

#### Agent 10: Ontology Coherence Assessment

**Overall score**: 7/10

**Strengths**:
- Knowledge Atoms pattern is well-applied
- EntityCategory pattern provides good extensibility
- Cross-realm architecture is clean

**Weaknesses**:
1. SEO/GEO conflation in some property names
2. Missing explicit intent taxonomy
3. No feedback loops from performance to optimization

**Recommendations for v11.5+**:
1. Consider separate SEO and GEO subsystems
2. Add Intent as first-class node (unifies SEO+GEO)
3. Build performance feedback graph

### Summary: v11.4 Migration Checklist (from Agent Audit)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  v11.4 MIGRATION CHECKLIST (Agent-Informed)                                     │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Before Migration:                                                              │
│  [ ] Update TARGETS arc scope → cross_realm                                     │
│  [ ] Update MONITORS_GEO arc scope → cross_realm                                │
│  [ ] Update org/semantic layer llm_context                                      │
│                                                                                 │
│  During Migration:                                                              │
│  [ ] Create SEOKeywordSet, GEOQuerySet with llm_context                         │
│  [ ] Create all 8 new arcs with llm_context                                     │
│  [ ] Improve llm_context for SEOKeyword, GEOQuery (Agent 5 rewrites)            │
│  [ ] Update HAS_SEO_KEYWORDS scope → intra_realm (after realm change)           │
│  [ ] Update HAS_GEO_QUERIES scope → intra_realm (after realm change)            │
│                                                                                 │
│  After Migration:                                                               │
│  [ ] Validate all 8 cross-realm arcs (Agent 4 checklist)                        │
│  [ ] Run schema validate --strict                                               │
│  [ ] Test all 8 Cypher patterns (Agent 9)                                       │
│  [ ] Add recommended indexes                                                    │
│                                                                                 │
│  Backlog (v11.5):                                                               │
│  [ ] Consider Page→SEO denormalization                                          │
│  [ ] Add GEOQueryFormat classification                                          │
│  [ ] Build SEO↔GEO intent linking                                               │
│  [ ] Add performance feedback loops                                             │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Sources

- DataForSEO API v3: https://docs.dataforseo.com/v3/
- Semrush API: https://developer.semrush.com/
- Ahrefs API: https://ahrefs.com/api
- Answer The Public: https://answerthepublic.com
- GEO Research: arxiv.org/html/2602.02961v1, arxiv.org/html/2601.16858v1
