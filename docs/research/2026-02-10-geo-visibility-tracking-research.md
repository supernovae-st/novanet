# GEO (Generative Engine Optimization) Visibility Tracking Research

**Date**: 2026-02-10
**Purpose**: Inform GEOQuery and GEOAnswer node design in NovaNet v11.x
**Researcher**: Claude (automated research agent)

---

## Executive Summary

GEO (Generative Engine Optimization) is the practice of optimizing brand visibility in AI-generated responses from platforms like ChatGPT, Perplexity, Gemini, Claude, and Google AI Overviews. Unlike traditional SEO (ranking in search results), GEO focuses on becoming the **cited source** or **recommended brand** in conversational AI answers.

Key findings:
1. **Market leaders**: ChatGPT (60.7% market share, 815M users), Gemini (18.2%), Copilot (13.2%), Perplexity (6.6%)
2. **Core metrics**: Visibility score, Share of Voice (SOV), Citation frequency, Sentiment, Position in response
3. **Data model**: Query -> Answer snapshots -> Time-series metrics (immutable)
4. **Tracking frequency**: Daily updates, weekly analysis, monthly trends
5. **Conversion**: AI referral traffic converts 4.4x higher than traditional SEO

---

## 1. GEO Tracking Tools (2024-2025)

### Tier 1: Enterprise Solutions

| Tool | Engines Covered | Key Features | Pricing |
|------|-----------------|--------------|---------|
| **Profound** | ChatGPT, Perplexity, Google AIO, Copilot, Gemini, Claude | Real-time tracking, 400M+ prompt insights, CDN integration, Profound Index | $399/mo (100 prompts/3 engines) |
| **Semrush AI Toolkit** | ChatGPT, Gemini, Claude, Perplexity, Grok, DeepSeek | Share of voice, sentiment, citation analysis, SEO integration | ~$99/mo per domain |
| **ZipTie** | ChatGPT, Perplexity, Google AIO, Gemini | AI Success Scores, content optimization gaps, screenshot verification | Enterprise pricing |

### Tier 2: Specialized Tools

| Tool | Focus | Strengths |
|------|-------|-----------|
| **Peec AI** | Real-time monitoring | Dual validation (API + native), prompt-level tracking |
| **Otterly.AI** | Prompt discovery | Keyword expansion, automated monitoring |
| **Gauge** | Competitive intelligence | 600+ predefined prompts, 7 AI platforms |
| **AIClicks** | Agency use | Content creation suggestions, crawler visibility |
| **Knowatoa AI** | Sentiment analysis | Accuracy tracking, competitor rankings |

### Key Differentiators

- **Profound**: Deepest enterprise depth (crawler logs + snapshots), real-time front-end visibility
- **Semrush**: Best SEO integration, unified marketing platform
- **Peec AI**: Best for prompt-level citation tracking with API + interface validation
- **ZipTie**: Best for screenshot verification and visual proof

---

## 2. Data Model Structure

### 2.1 Query Layer (GEOQuery)

The query is the fundamental unit of GEO tracking. Each query represents a prompt that users ask AI engines.

**Core Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `query_text` | string | Yes | The exact prompt/question submitted |
| `query_type` | enum | No | Classification: informational, how_to, comparison, recommendation, transactional, navigational |
| `topic` | string | No | Topic cluster (e.g., "QR codes", "restaurant marketing") |
| `intent` | string | No | User intent category |
| `locale` | string | Yes | BCP-47 locale code (responses vary by region) |
| `search_volume` | int | No | Estimated monthly query volume |
| `difficulty` | float | No | Competitive difficulty score (0-100) |

**Query Type Classification:**

| Type | Description | GEO Relevance |
|------|-------------|---------------|
| `informational` | Seeking knowledge | High - AI synthesizes answers |
| `how_to` | Step-by-step guidance | Very high - structured responses |
| `comparison` | Evaluating options ("X vs Y") | Top performer (32.5% citations) |
| `recommendation` | Seeking suggestions ("best X for Y") | Core GEO use case |
| `transactional` | Ready to purchase | High for commercial queries |
| `navigational` | Finding specific sites | Lower - less AI-native |

### 2.2 Answer Layer (GEOAnswer)

Each answer is an **immutable snapshot** of an AI response at a specific moment. Never updated after creation.

**Core Fields:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `engine` | enum | Yes | AI platform: chatgpt, gemini, perplexity, claude, copilot, grok, deepseek, google_aio |
| `engine_version` | string | No | Model identifier (e.g., "gpt-4o-2024-08-01", "claude-3-5-sonnet") |
| `observed_at` | datetime | Yes | UTC timestamp of capture |
| `answer_text` | string | Yes | Full response text |
| `answer_summary` | string | No | Condensed summary for quick analysis |

**Citation Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `cited_urls` | string[] | Full URLs cited in response |
| `cited_domains` | string[] | Domains extracted from URLs |
| `citation_count` | int | Number of citations |
| `citation_positions` | int[] | Position of each citation in response |

**Brand Analysis Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `brand_mentions` | string[] | Brands mentioned in response |
| `brand_position` | int | Position of tracked brand (1 = first, null = not mentioned) |
| `brand_sentiment` | enum | positive, neutral, negative |
| `brand_context` | string | Excerpt showing brand mention in context |
| `recommendation_strength` | enum | strong, moderate, neutral, weak, negative |

**Quality Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `relevance_score` | float | How relevant to original query (0-100) |
| `accuracy_score` | float | Factual accuracy if verifiable (0-100) |
| `has_screenshot` | boolean | Whether visual proof was captured |

### 2.3 Metrics Layer (GEOMetrics)

Time-series aggregations for trend analysis. Created periodically (daily/weekly).

**Visibility Metrics:**

| Metric | Type | Calculation |
|--------|------|-------------|
| `visibility_score` | float (0-100) | % of responses where brand appears |
| `visibility_by_engine` | map<engine, float> | Per-engine visibility scores |
| `visibility_change_7d` | float | Change in visibility over 7 days (percentage points) |
| `visibility_change_30d` | float | Change over 30 days |

**Share of Voice Metrics:**

| Metric | Type | Calculation |
|--------|------|-------------|
| `share_of_voice` | float (0-100) | Brand mentions / Total brand mentions x 100 |
| `sov_by_engine` | map<engine, float> | Per-engine SOV |
| `sov_rank` | int | Rank among competitors |
| `competitor_sov` | map<brand, float> | Competitor SOV values |

**Citation Metrics:**

| Metric | Type | Calculation |
|--------|------|-------------|
| `citation_frequency` | float | Avg citations per response |
| `citation_rate` | float (0-100) | % of responses citing brand domain |
| `avg_citation_position` | float | Average position of citations |
| `top_cited_pages` | string[] | Most frequently cited URLs |

**Sentiment Metrics:**

| Metric | Type | Calculation |
|--------|------|-------------|
| `sentiment_score` | float (-100 to 100) | Net sentiment (positive - negative) |
| `sentiment_distribution` | map<sentiment, float> | % positive/neutral/negative |
| `sentiment_trend` | enum | rising, stable, declining |

**Position Metrics:**

| Metric | Type | Calculation |
|--------|------|-------------|
| `avg_brand_position` | float | Average position when mentioned (lower = better) |
| `first_mention_rate` | float (0-100) | % of times brand is mentioned first |
| `recommendation_rate` | float (0-100) | % of times brand is actively recommended |

---

## 3. Engine Coverage

### Priority Engines (2025)

| Engine | Market Share | Monthly Users | Notes |
|--------|--------------|---------------|-------|
| **ChatGPT** | 60.7% | 815M | Dominant leader, 5.7B visits |
| **Gemini** | 18.2% | 346M | Fastest growing (647% YoY) |
| **Copilot** | 13.2% | 103M | Microsoft integration |
| **Perplexity** | 6.6% | ~100M | 780M monthly queries |
| **Google AI Overviews** | N/A | 2B | Integrated into search |
| **Claude** | ~2% | ~50M | Enterprise focus |
| **Grok** | <1% | ~10M | X/Twitter integration |
| **DeepSeek** | <1% | Variable | Chinese market |
| **Meta AI** | <1% | ~30M | Facebook/Instagram |

### Engine-Specific Considerations

- **ChatGPT**: Has web browsing, plugins, GPT Store - track which GPTs mention brand
- **Perplexity**: Always shows citations - best for citation tracking
- **Gemini**: Strong RAG integration with Google Search
- **Claude**: Less web-connected, more training-based responses
- **Google AI Overviews**: Appears in search results, massive reach

---

## 4. Best Practices

### 4.1 Snapshot Storage

**Do Store:**
- Full response text (primary record)
- Vector embeddings for similarity search (secondary)
- Metadata: timestamp, model version, temperature, token counts
- Screenshots for visual proof
- Prompt hash (SHA-256) for deduplication

**Storage Pattern:**
- Append-only / immutable snapshots
- Time-series database or versioned storage
- TTL-based cleanup for old snapshots (retain summary metrics)

### 4.2 Cross-Engine Normalization

| Challenge | Solution |
|-----------|----------|
| Different response formats | Standardize schema, strip model-specific artifacts |
| Variable response lengths | Normalize to fixed length or extract key entities |
| Different citation formats | Parse and normalize URLs, domains |
| Model version drift | Track version explicitly, enable A/B comparison |

### 4.3 Tracking Frequency

| Cadence | Use Case |
|---------|----------|
| **Daily** | Real-time alerts, fast-moving industries |
| **Weekly** | Trend spotting, strategy refinement |
| **Monthly** | Baseline monitoring, long-term performance |

**Key Insight**: AI responses are probabilistic - single checks are unreliable. Track over time to identify stable patterns.

### 4.4 Response Variability

AI responses vary even for identical prompts. Best practices:
- Run multiple samples per query (3-5 minimum)
- Track consistency/variance as a metric
- Use median or mode for position metrics
- Flag high-variance queries for review

---

## 5. Metrics Formulas

### Visibility Score
```
visibility_score = (responses_with_brand / total_responses) * 100
```

### Share of Voice
```
share_of_voice = (brand_mentions / sum(all_brand_mentions)) * 100
```

### Citation Rate
```
citation_rate = (responses_citing_brand_domain / total_responses) * 100
```

### Net Sentiment Score
```
net_sentiment = ((positive_mentions - negative_mentions) / total_mentions) * 100
```

### AI Success Score (composite)
```
ai_success_score =
  (visibility_score * 0.3) +
  (share_of_voice * 0.3) +
  (citation_rate * 0.2) +
  (sentiment_normalized * 0.1) +
  (position_score * 0.1)
```

---

## 6. Recommendations for NovaNet Schema

### 6.1 GEOQuery Enhancements

**Add Fields:**
- `query_type`: enum (informational, how_to, comparison, recommendation, transactional, navigational)
- `topic`: string (topic cluster)
- `difficulty`: float (competitive difficulty 0-100)
- `search_volume`: int (estimated monthly volume)
- `target_engines`: string[] (which engines to track)

**Remove/Deprecate:**
- Per-engine visibility scores (move to GEOMetrics)

### 6.2 GEOAnswer Enhancements

**Add Fields:**
- `cited_urls`: string[] (full URLs, not just domains)
- `citation_count`: int
- `brand_position`: int (1-based, null if not mentioned)
- `brand_sentiment`: enum (positive, neutral, negative)
- `brand_context`: string (excerpt showing mention)
- `recommendation_strength`: enum
- `answer_summary`: string (condensed version)
- `has_screenshot`: boolean
- `sample_number`: int (for multi-sample tracking)

**Rename/Clarify:**
- `cited_domains` stays (extracted from URLs)

### 6.3 GEOMetrics Enhancements

**Add Fields:**
- `share_of_voice`: float (0-100)
- `sov_by_engine`: map<engine, float>
- `citation_rate`: float (0-100)
- `avg_citation_position`: float
- `sentiment_score`: float (-100 to 100)
- `sentiment_distribution`: map<sentiment, float>
- `avg_brand_position`: float
- `first_mention_rate`: float
- `sample_count`: int (number of samples in this period)
- `variance_score`: float (response consistency)

### 6.4 New Node: GEOCompetitor

Track competitor visibility alongside brand:

```yaml
node:
  name: GEOCompetitor
  realm: org
  layer: geo
  trait: knowledge
  properties:
    competitor_name: string
    competitor_domain: string
    visibility_score: float
    share_of_voice: float
    sentiment_score: float
```

**Arc:** `GEOQuery -[:TRACKS_COMPETITOR]-> GEOCompetitor`

### 6.5 New Node: GEOCitation

Granular citation tracking:

```yaml
node:
  name: GEOCitation
  realm: org
  layer: geo
  trait: derived
  properties:
    url: string
    domain: string
    position: int
    context: string
```

**Arc:** `GEOAnswer -[:HAS_CITATION]-> GEOCitation`

---

## 7. Sources

### Primary Research
1. Perplexity AI searches (2026-02-10)
2. Tool documentation: Profound, Semrush, ZipTie, Peec AI, Otterly.AI

### Market Data
- ChatGPT: 815M users, 60.7% market share
- Gemini: 346M users, 647% growth YoY
- AI referral traffic converts 4.4x higher than traditional SEO
- AI search handles increasing share of zero-click queries

### Methodology
- Web research via Perplexity API
- Analysis of existing NovaNet GEO schema (v10.7)
- Cross-referencing multiple tool approaches

---

## 8. Confidence Level

**High confidence:**
- Core metrics (visibility, SOV, citations)
- Data model structure (query -> answer -> metrics)
- Engine coverage list

**Medium confidence:**
- Specific calculation formulas (tools are proprietary)
- Optimal tracking frequency (varies by industry)
- Response variability handling

**Requires validation:**
- Competitor tracking node design
- Citation granularity (node vs. array)
- Sample size requirements

---

## Appendix: Engine Enum Values

Recommended standardized engine identifiers:

```
chatgpt      # OpenAI ChatGPT (all versions)
gemini       # Google Gemini
perplexity   # Perplexity AI
claude       # Anthropic Claude
copilot      # Microsoft Copilot
google_aio   # Google AI Overviews
grok         # xAI Grok
deepseek     # DeepSeek
meta_ai      # Meta AI
```
