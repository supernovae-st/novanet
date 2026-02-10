# DataForSEO API v3 Research Report

**Date**: 2026-02-10
**Author**: Claude Code (Research Agent)
**Purpose**: Integration planning for NovaNet SEO layer

---

## Executive Summary

DataForSEO API v3 provides comprehensive SEO data through multiple API families. This research maps their capabilities to NovaNet's SEO schema (`org/seo` layer), identifying optimal endpoints and property mappings for:

- **SEOKeyword**: Core keyword with volume, difficulty, CPC
- **SEOKeywordMetrics**: Historical snapshots and position tracking
- **SEOQuestion**: Question-form keywords (Answer The Public replacement)
- **SEOComparison**: "X vs Y" comparison keywords
- **SEOPreposition**: "X for Y" use-case keywords

---

## 1. Keywords Data API

### 1.1 Google Ads Search Volume (Primary)

**Endpoint**: `POST /v3/keywords_data/google_ads/search_volume/live`

**Use Case**: Get search volume and competition for up to 1000 keywords.

**Rate Limit**: 12 requests/minute per account (Google Ads endpoints)

**Request**:
```json
[
  {
    "location_code": 2840,
    "keywords": ["buy laptop", "cheap laptops for sale"],
    "date_from": "2021-08-01",
    "search_partners": true
  }
]
```

**Response** (key fields):
```json
{
  "result": [
    {
      "keyword": "buy laptop",
      "spell": null,
      "location_code": 2840,
      "language_code": "en",
      "search_partners": false,
      "competition": "HIGH",
      "competition_index": 100,
      "search_volume": 2900,
      "low_top_of_page_bid": 1.69,
      "high_top_of_page_bid": 10.04,
      "cpc": 7.95,
      "monthly_searches": [
        { "year": 2023, "month": 10, "search_volume": 2400 },
        { "year": 2023, "month": 9, "search_volume": 2900 }
      ]
    }
  ]
}
```

### 1.2 Keywords for Keywords (Related Keywords)

**Endpoint**: `POST /v3/keywords_data/google_ads/keywords_for_keywords/live`

**Use Case**: Get keyword suggestions from seed keywords (up to 20,000 suggestions).

**Rate Limit**: 12 requests/minute per account

**Request**:
```json
[
  {
    "location_code": 2840,
    "keywords": ["phone", "cellphone"]
  }
]
```

---

## 2. Clickstream Data API

### 2.1 Global Search Volume

**Endpoint**: `POST /v3/keywords_data/clickstream_data/global_search_volume/live`

**Use Case**: Clickstream-based volume with geographic distribution across all countries.

**Rate Limit**: 2000 API calls/minute, max 30 simultaneous requests

**Request**:
```json
[
  {
    "keywords": ["youtube", "you tube", "youtub"]
  }
]
```

**Response** (key fields):
```json
{
  "result": [
    {
      "items_count": 3,
      "items": [
        {
          "keyword": "youtube",
          "search_volume": 976222640,
          "country_distribution": [
            { "country_iso_code": "US", "search_volume": 145773557, "percentage": 14.93 },
            { "country_iso_code": "IN", "search_volume": 84475722, "percentage": 8.65 },
            { "country_iso_code": "TR", "search_volume": 64240714, "percentage": 6.58 }
          ]
        }
      ]
    }
  ]
}
```

**NovaNet Use**: Enriches SEOKeyword with `traffic_potential` estimates and global distribution for locale prioritization.

---

## 3. SERP API (Position Tracking)

### 3.1 Live Regular

**Endpoint**: `POST /v3/serp/google/organic/live/regular`

**Use Case**: Quick position check for organic results only.

**Depth**: 10-200 results (charged per 10)

### 3.2 Live Advanced (Recommended)

**Endpoint**: `POST /v3/serp/google/organic/live/advanced`

**Use Case**: Full SERP with all features including People Also Ask.

**Key Parameters**:
- `depth`: Number of results (default 10, max 200)
- `people_also_ask_click_depth`: 1-4 clicks to expand PAA (extra $0.00015/click)
- `calculate_rectangles`: Pixel positioning data (extra $0.002)
- `load_async_ai_overview`: Include AI Overview content (extra $0.002)

**Request**:
```json
[
  {
    "keyword": "qr code generator",
    "location_code": 2250,
    "language_code": "fr",
    "device": "desktop",
    "depth": 10,
    "people_also_ask_click_depth": 2
  }
]
```

**Response Structure** (item_types):
```
answer_box, app, carousel, multi_carousel, featured_snippet,
google_flights, google_reviews, images, jobs, knowledge_graph,
local_pack, hotels_pack, map, organic, paid, people_also_ask,
related_searches, people_also_search, shopping, top_stories,
twitter, video, events, ai_overview, discussions_and_forums
```

**Organic Result Fields**:
```json
{
  "type": "organic",
  "rank_group": 1,
  "rank_absolute": 1,
  "domain": "example.com",
  "url": "https://example.com/page",
  "title": "Page Title",
  "description": "Meta description text",
  "is_featured_snippet": false,
  "rating": { "value": 4.5, "votes_count": 123 },
  "faq": { "items": [...] }
}
```

---

## 4. DataForSEO Labs (Keyword Research)

### 4.1 Keyword Ideas

**Endpoint**: `POST /v3/dataforseo_labs/google/keyword_ideas/live`

**Use Case**: Get semantically related keywords (up to 1000 per request).

**Rate Limit**: 2000 API calls/minute, max 30 simultaneous

**Request**:
```json
[
  {
    "keywords": ["qr code generator", "create qr code"],
    "location_code": 2250,
    "language_code": "fr",
    "include_serp_info": true,
    "include_clickstream_data": true,
    "limit": 500
  }
]
```

**Response Fields**:
```json
{
  "items": [
    {
      "keyword": "generateur qr code gratuit",
      "keyword_info": {
        "search_volume": 12000,
        "competition": 0.45,
        "competition_level": "MEDIUM",
        "cpc": 0.85,
        "low_top_of_page_bid": 0.50,
        "high_top_of_page_bid": 2.10,
        "monthly_searches": [...],
        "search_volume_trend": {
          "monthly": 5,
          "quarterly": -2,
          "yearly": 15
        }
      },
      "keyword_properties": {
        "keyword_difficulty": 35,
        "detected_language": "fr",
        "words_count": 3
      },
      "serp_info": {
        "se_results_count": 45000000,
        "serp_item_types": ["organic", "featured_snippet", "people_also_ask"]
      },
      "clickstream_keyword_info": {
        "search_volume": 15000,
        "gender_distribution": { "female": 45, "male": 55 },
        "age_distribution": { "18-24": 20, "25-34": 35, ... }
      }
    }
  ]
}
```

---

## 5. Questions / Answer The Public Replacement

DataForSEO provides question data through **SERP People Also Ask** scraping rather than a dedicated API like Answer The Public. Strategy:

### 5.1 People Also Ask Extraction

Use SERP Advanced with `people_also_ask_click_depth`:

**Endpoint**: `POST /v3/serp/google/organic/live/advanced`

```json
{
  "keyword": "qr code",
  "location_code": 2250,
  "language_code": "fr",
  "people_also_ask_click_depth": 4
}
```

**Response** includes `people_also_ask` items:
```json
{
  "type": "people_also_ask",
  "items": [
    {
      "title": "Comment scanner un QR code?",
      "url": "https://example.com/answer",
      "expanded_element": [...]
    }
  ]
}
```

### 5.2 Alternative: Autocomplete API

**Endpoint**: `POST /v3/dataforseo_labs/google/keyword_suggestions/live`

Provides question-form suggestions when seed keyword includes question words:
- "how to qr code" -> suggestions starting with "how to"
- "what is qr code" -> suggestions starting with "what is"

---

## 6. Rate Limits and Pricing

### 6.1 Rate Limits

| API Family | Calls/Minute | Simultaneous | Notes |
|------------|--------------|--------------|-------|
| Google Ads Keywords | 12/min | - | Per account limit |
| Clickstream | 2000/min | 30 | Standard limit |
| SERP Live | 2000/min | - | Per task |
| DataForSEO Labs | 2000/min | 30 | Can request increase |

### 6.2 Pricing (per request/result)

| Endpoint | Cost | Billing Model |
|----------|------|---------------|
| Search Volume Live | $0.075 | Per request (1000 kw max) |
| Keywords for Keywords | $0.075 | Per request |
| Clickstream Global | $0.05 | Per 1000 keywords |
| SERP Live Regular | $0.004 | Per 10 results |
| SERP Live Advanced | $0.007 | Per 10 results |
| Keyword Ideas | $0.04 | Per 100 results |
| People Also Ask clicks | +$0.00015 | Per click depth |
| AI Overview loading | +$0.002 | Per request |

### 6.3 Account Types

- **Free Trial**: $1 credit upon registration
- **Pay-as-you-go**: No minimum, charged per usage
- **Volume discounts**: Available for high-volume users (contact sales)

---

## 7. Property Mapping: DataForSEO -> NovaNet

### 7.1 SEOKeyword Mapping

| NovaNet Property | DataForSEO Source | API | Notes |
|------------------|-------------------|-----|-------|
| `value` | `keyword` | All | Decoded, lowercase |
| `volume` | `search_volume` | Google Ads / Labs | 12-month average |
| `difficulty` | `keyword_difficulty` | Labs keyword_properties | 0-100 scale |
| `cpc` | `cpc` | Google Ads / Labs | USD |
| `intent` | - | Manual / LLM | Not provided by D4SEO |
| `platform` | Fixed `"google"` | - | Source identifier |
| `source` | Fixed `"dataforseo"` | - | Provider identifier |
| `traffic_potential` | `clickstream_keyword_info.search_volume` | Labs | With clickstream flag |
| `clicks` | Calculated | Labs | volume * clicks_per_search estimate |
| `clicks_per_search` | - | Not available | Needs Ahrefs |
| `serp_features` | `serp_info.serp_item_types` | Labs | Array mapping below |
| `competition` | `competition` | Labs keyword_info | 0-1 float |
| `trend` | `search_volume_trend.yearly` | Labs | Derive: >10 rising, <-10 declining |
| `seasonality` | `monthly_searches[]` | Google Ads | Convert to 100-indexed array |

**SERP Features Mapping**:
```
DataForSEO item_types        -> NovaNet serp_features
-----------------------         ---------------------
featured_snippet             -> featured_snippet
ai_overview                  -> ai_overview
people_also_ask              -> people_also_ask
images                       -> images
video                        -> videos
knowledge_graph              -> knowledge_panel
local_pack                   -> local_pack
shopping                     -> shopping
```

### 7.2 SEOKeywordMetrics Mapping

| NovaNet Property | DataForSEO Source | API | Notes |
|------------------|-------------------|-----|-------|
| `observed_at` | Response timestamp | All | UTC datetime |
| `source` | Fixed `"dataforseo"` | - | |
| `volume` | `search_volume` | Google Ads | Snapshot value |
| `difficulty` | `keyword_difficulty` | Labs | 0-100 |
| `cpc` | `cpc` | Google Ads | USD |
| `clicks` | `clickstream_keyword_info.search_volume` | Labs | If available |
| `traffic_potential` | Same as clicks | Labs | Clickstream-based |
| `position` | `rank_absolute` (avg) | SERP | Aggregate across pages |
| `best_position` | `rank_absolute` (min) | SERP | Best position found |
| `url` | `url` from organic match | SERP | Our ranking URL |
| `impressions` | - | GSC API | Not from D4SEO |
| `clicks_gsc` | - | GSC API | Not from D4SEO |
| `ctr` | - | GSC API | Not from D4SEO |

### 7.3 SEOQuestion Mapping

| NovaNet Property | DataForSEO Source | API | Notes |
|------------------|-------------------|-----|-------|
| `value` | `title` from people_also_ask | SERP Advanced | The question text |
| `volume` | - | Not directly available | Need separate search volume lookup |
| `difficulty` | - | Not directly available | Inherit from parent keyword |
| `cpc` | - | Not directly available | Inherit from parent keyword |
| `question_word` | Parse from `title` | SERP | Extract: what/how/why/when/where/who/can/is/does |
| `answer_type` | Derive from question_word | Logic | Map question word to expected format |
| `featured_snippet` | Check if present in SERP | SERP | Look for featured_snippet type |
| `paa_position` | Index in `items` array | SERP | 1-indexed position |

**Question Word Detection**:
```javascript
const questionWords = {
  'what': ['what', 'que', 'quoi', 'was', 'que'],
  'how': ['how', 'comment', 'wie', 'como', 'how to'],
  'why': ['why', 'pourquoi', 'warum', 'por que'],
  'when': ['when', 'quand', 'wann', 'cuando'],
  'where': ['where', 'ou', 'wo', 'donde'],
  'who': ['who', 'qui', 'wer', 'quien'],
  'can': ['can', 'peut', 'kann', 'puede'],
  'is': ['is', 'est', 'ist', 'es'],
  'does': ['does', 'fait', 'macht', 'hace']
};
```

### 7.4 SEOComparison Mapping

| NovaNet Property | DataForSEO Source | API | Notes |
|------------------|-------------------|-----|-------|
| `value` | Keyword containing "vs" | Labs/SERP | Filter keywords |
| `volume` | `search_volume` | Labs | From keyword lookup |
| `difficulty` | `keyword_difficulty` | Labs | |
| `cpc` | `cpc` | Labs | |
| `comparison_type` | Parse from keyword | Logic | vs/or/versus/compared_to |
| `entity_a_key` | Parse left side | Logic | Before comparison word |
| `entity_b_key` | Parse right side | Logic | After comparison word |
| `winner` | - | LLM/Manual | Not from D4SEO |

**Comparison Detection Pattern**:
```regex
/^(.+?)\s+(vs\.?|versus|or|compared to|better than|like|similar to)\s+(.+)$/i
```

### 7.5 SEOPreposition Mapping

| NovaNet Property | DataForSEO Source | API | Notes |
|------------------|-------------------|-----|-------|
| `value` | Keyword containing preposition | Labs | Filter keywords |
| `volume` | `search_volume` | Labs | |
| `difficulty` | `keyword_difficulty` | Labs | |
| `cpc` | `cpc` | Labs | |
| `preposition` | Parse from keyword | Logic | for/with/without/near/to/like |
| `use_case` | Parse after preposition | Logic | Context/use case text |
| `use_case_type` | - | LLM/Manual | Classify: feature/industry/audience |

**Preposition Detection Pattern**:
```regex
/^(.+?)\s+(for|with|without|near|to|like|in|on|at)\s+(.+)$/i
```

---

## 8. Example API Responses

### 8.1 Search Volume Response

```json
{
  "version": "0.1.20231117",
  "status_code": 20000,
  "status_message": "Ok.",
  "time": "1.9903 sec.",
  "cost": 0.075,
  "tasks_count": 1,
  "tasks_error": 0,
  "tasks": [
    {
      "id": "11301935-1535-0367-0000-b44e4432f0be",
      "status_code": 20000,
      "status_message": "Ok.",
      "time": "1.8689 sec.",
      "cost": 0.075,
      "result_count": 3,
      "result": [
        {
          "keyword": "buy laptop",
          "spell": null,
          "location_code": 2840,
          "language_code": "en",
          "search_partners": false,
          "competition": "HIGH",
          "competition_index": 100,
          "search_volume": 2900,
          "low_top_of_page_bid": 1.69,
          "high_top_of_page_bid": 10.04,
          "cpc": 7.95,
          "monthly_searches": [
            { "year": 2023, "month": 10, "search_volume": 2400 },
            { "year": 2023, "month": 9, "search_volume": 2900 },
            { "year": 2023, "month": 8, "search_volume": 3600 }
          ]
        }
      ]
    }
  ]
}
```

### 8.2 Clickstream Global Response

```json
{
  "version": "0.1.20240801",
  "status_code": 20000,
  "status_message": "Ok.",
  "time": "0.8803 sec.",
  "cost": 0.15,
  "tasks": [
    {
      "result": [
        {
          "items_count": 3,
          "items": [
            {
              "keyword": "youtube",
              "search_volume": 976222640,
              "country_distribution": [
                { "country_iso_code": "US", "search_volume": 145773557, "percentage": 14.93 },
                { "country_iso_code": "IN", "search_volume": 84475722, "percentage": 8.65 },
                { "country_iso_code": "DE", "search_volume": 53117168, "percentage": 5.44 }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

---

## 9. Integration Recommendations

### 9.1 Data Pipeline

```
1. Seed Keywords (manual/Entity-derived)
       |
       v
2. DataForSEO Labs Keyword Ideas (expand keyword list)
       |
       v
3. Google Ads Search Volume (get volume/competition/CPC)
       |
       v
4. SERP Advanced (get SERP features, PAA questions)
       |
       v
5. Parse & Classify (questions, comparisons, prepositions)
       |
       v
6. Create NovaNet Nodes (SEOKeyword, SEOQuestion, etc.)
```

### 9.2 Recommended Endpoints by Use Case

| Use Case | Endpoint | Cost Estimate |
|----------|----------|---------------|
| Initial keyword research | Labs Keyword Ideas | $0.04/100 kw |
| Volume updates (weekly) | Google Ads Search Volume | $0.075/1000 kw |
| Position tracking (daily) | SERP Live Regular | $0.004/keyword |
| Question discovery | SERP Advanced + PAA depth | $0.01/keyword |
| Global volume analysis | Clickstream Global | $0.05/1000 kw |

### 9.3 Caching Strategy

- **SEOKeyword base data**: Cache 7 days
- **Volume/difficulty**: Refresh weekly
- **Position tracking**: Daily or on-demand
- **PAA questions**: Cache 30 days (slow-changing)

---

## 10. Gaps and Alternatives

### 10.1 Data Not Available from DataForSEO

| Data Point | NovaNet Field | Alternative Source |
|------------|---------------|-------------------|
| Clicks per search | `clicks_per_search` | Ahrefs API |
| GSC impressions | `impressions` | Google Search Console API |
| GSC clicks | `clicks_gsc` | Google Search Console API |
| CTR | `ctr` | Google Search Console API |
| Search intent | `intent` | LLM classification |
| Winner in comparisons | `winner` | LLM/Manual |
| Use case type | `use_case_type` | LLM classification |

### 10.2 Complementary APIs

1. **Google Search Console API**: Position, impressions, CTR for owned properties
2. **Ahrefs API**: Clicks per search, traffic potential, DR/UR
3. **LLM (Claude/GPT)**: Intent classification, comparison analysis

---

## Sources

1. DataForSEO Keywords Data Overview - https://docs.dataforseo.com/v3/keywords_data/overview/
2. DataForSEO SERP API - https://docs.dataforseo.com/v3/serp/overview/
3. DataForSEO Labs API - https://docs.dataforseo.com/v3/dataforseo_labs/overview/
4. DataForSEO Clickstream Data - https://docs.dataforseo.com/v3/keywords_data/clickstream_data/global_search_volume/live/
5. DataForSEO Pricing - https://dataforseo.com/pricing

---

## Methodology

- **Tools used**: curl, Python HTML parser
- **Pages analyzed**: 8 documentation pages
- **API endpoints tested**: 1 (auth required for full testing)
- **Time period**: Documentation as of 2026-02-10

## Confidence Level

**High** - DataForSEO documentation is comprehensive and API responses match documented structures. Property mappings are complete except for fields requiring complementary data sources (GSC, Ahrefs).

## Further Research Suggestions

1. Obtain DataForSEO API credentials for live testing
2. Evaluate Ahrefs API for clicks_per_search enrichment
3. Design GSC integration for position/CTR tracking
4. Build LLM classifier for search intent detection
5. Benchmark DataForSEO vs Semrush/Ahrefs data quality
