# Plan: SEO/GEO Architecture Next Steps

**Date**: 2026-02-08
**Status**: Planifié
**Depends on**: SEO Keywords fr-FR Import (completed)

---

## Completed Today

- [x] Schema updates: EntityL10n + SEOKeyword simplified
- [x] 281 Entity nodes imported to Neo4j
- [x] 281 EntityL10n fr-FR imported to Neo4j
- [x] 1,519 SEOKeyword fr-FR imported to Neo4j
- [x] Architecture documented: SEO + GEO layer design

---

## Next Steps

### 1. Correct SEOKeyword Structure

**Problem**: Volatile metrics (volume, difficulty, cpc) are currently on SEOKeyword but should be in SEOMetrics (time-series).

**Tasks**:
- [ ] Update `seo-keyword.yaml` schema - remove volatile properties
- [ ] Create `seo-metrics.yaml` schema with time-series properties
- [ ] Update seed generation script to create SEOMetrics nodes
- [ ] Regenerate `12-seokeyword-fr-fr.cypher` with correct structure
- [ ] Re-import to Neo4j

**SEOKeyword (stable only)**:
```yaml
properties:
  key: string
  value: string          # "créer qr code"
  intent: string         # transactional|informational|navigational|commercial
  platform: string       # google|bing|youtube
  source: string         # ahrefs|semrush
```

**SEOMetrics (volatile, time-series)**:
```yaml
properties:
  snapshot_date: datetime
  volume: int
  difficulty: int
  cpc: float
  traffic_potential: int
  clicks: int
  clicks_per_search: float
  competition: float
  position: int
  impressions: int
  ctr: float
  serp_features: string[]
  trend: string
  seasonality: int[]
```

---

### 2. Import SEOQuestion from ATP

**Source**: `docs/assets/keywods/fr-fr_qr/seo/paas_export_qr code.csv` (27 questions)

**Tasks**:
- [ ] Parse ATP CSV file
- [ ] Create SEOQuestion nodes
- [ ] Link to parent SEOKeyword via [:HAS_QUESTIONS]
- [ ] Link EntityL10n [:ANSWERS] for content coverage tracking

**SEOQuestion schema**:
```yaml
properties:
  key: string
  value: string              # "comment créer un qr code gratuit?"
  question_word: string      # comment|pourquoi|quand|où|quel|est-ce
  parent_keyword: string     # reference to SEOKeyword
```

---

### 3. Create GEO Node Kinds

**New nodes to create** in `packages/core/models/node-kinds/global/seo/`:

| Node | Description | Key Properties |
|------|-------------|----------------|
| `geo-prompt.yaml` | Question asked to AI | value, intent, sentiment, prompt_type, question_word |
| `geo-response.yaml` | AI response (1 per platform) | platform, model_version, response_text, response_date |
| `geo-citation.yaml` | Source cited in response | url, domain, position, anchor_text, is_our_domain |
| `geo-mention.yaml` | Brand/entity mentioned | entity_key, mention_type, sentiment, context, position |
| `geo-metrics.yaml` | Time-series tracking | response_count, citation_rate, mention_rate, sentiment_score |

**Relations**:
```
EntityL10n ──[:ANSWERS]──────► GEOPrompt
EntityL10n ◄─[:MENTIONS]───── GEOMention
GEOPrompt ──[:HAS_RESPONSE]─► GEOResponse
GEOResponse ─[:HAS_CITATION]► GEOCitation
GEOResponse ─[:HAS_MENTION]─► GEOMention
GEO* ───────[:HAS_METRICS]──► GEOMetrics
```

**Platforms to track**:
- chatgpt
- perplexity
- gemini
- copilot
- claude
- google-ai-overview

---

## Architecture Reference

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SEO LAYER (keyword-based)          │  GEO LAYER (prompt→response based)   │
├─────────────────────────────────────┼─────────────────────────────────────────┤
│                                     │                                       │
│  SEOKeyword (stable)                │  GEOPrompt (the question)             │
│       │                             │       │                               │
│       ├── SEOQuestion               │       └── GEOResponse (1/platform)    │
│       ├── SEOComparison             │              ├── GEOCitation          │
│       └── SEOPreposition            │              └── GEOMention           │
│                                     │                                       │
│  ALL ──[:HAS_METRICS]──► SEOMetrics │  ALL ──[:HAS_METRICS]──► GEOMetrics   │
│                                     │                                       │
└─────────────────────────────────────┴─────────────────────────────────────────┘

EntityL10n connections:
  - [:TARGETS] ──► SEOKeyword
  - [:ANSWERS] ──► SEOQuestion, GEOPrompt
  - [:MENTIONED_IN] ◄── GEOMention
```

---

## Open Question: Naming Convention

**Issue**: Both EntityL10n and SEOKeyword are locale-specific, but only EntityL10n has the `L10n` suffix.

**Current convention**: `*L10n` = "localization OF a parent invariant node"
- EntityL10n has parent Entity (invariant) → L10n suffix
- SEOKeyword has no parent invariant → no L10n suffix

**Options to consider**:
- A: Keep current (technically correct but potentially confusing)
- B: Add L10n to all locale-specific nodes (SEOKeywordL10n, etc.)
- C: Remove L10n, use different naming (EntityLocale, LocalizedEntity)
- D: Use composite keys with locale (create-qr-code:fr-FR)

**Decision**: TBD

---

## Files Reference

**Scripts**: `scripts/seo-import/`
**Seeds**: `packages/db/seed/10-*, 11-*, 12-*`
**Schemas**: `packages/core/models/node-kinds/global/seo/`
