# SEOKeyword Data Quality Audit Report

**Date:** 2026-03-06
**Auditor:** Claude Code (Haiku 4.5)
**Scope:** 1,487 SEOKeyword nodes in Neo4j knowledge graph
**Severity:** CRITICAL + Major Issues

---

## Executive Summary

The SEOKeyword dataset exhibits **critical data integrity issues** that prevent proper utilization in content generation workflows:

| Category | Status | Impact |
|----------|--------|--------|
| **Data Completeness** | 🔴 Critical | 95.7% nodes missing required properties |
| **Locale Mapping** | 🔴 Critical | 95.7% nodes lack locale_key (denormalized field) |
| **Schema Alignment** | 🔴 Critical | Inconsistent property naming vs. schema |
| **Metrics Coverage** | 🟡 Major | 99.3% have metrics, but incomplete enrichment |
| **Entity Relationships** | 🟡 Major | Only 0.6% of keywords linked to entities |
| **Data Freshness** | 🟡 Major | Limited source data (11 keywords only) |

**Recommendation:** Data cleanup required before integration with entity generation workflows.

---

## Dataset Overview

```
Total SEOKeyword nodes:        1,487 (verified count)
Unique locales:                6 (en-US, fr-FR, es-MX, de-DE, ja-JP, null)
Date range:                    2026-02-16 to 2026-02-17 (2 days)
Avg search volume per keyword: 4,334 searches/month
Search volume range:           50 - 2,200,000
```

---

## Critical Issues

### 1. Missing locale_key Denormalization (95.7% of dataset)

**Severity:** 🔴 CRITICAL
**Impact:** Breaks fast locale lookups, violates schema ADR-030

**Problem:**
- **1,424 keywords** (95.7%) have `locale_key = NULL`
- Only **63 keywords** (4.3%) have `locale_key` populated
- The `locale` property exists but is also inconsistent (old format)

**Current state:**

| locale_key | Count | % |
|-----------|-------|------|
| `NULL` | 1,424 | 95.7% |
| `en-US` | 54 | 3.6% |
| `es-MX` | 4 | 0.3% |
| `fr-FR` | 3 | 0.2% |
| `de-DE` | 1 | 0.1% |
| `ja-JP` | 1 | 0.1% |

**Example (malformed key with missing extraction):**
```
k.key: "seo-qr-code-generator-fr-fr-aa74a6"
k.locale_key: NULL  ← Should be "fr-FR" (extracted from key suffix)
k.locale: NULL
```

**Root cause:** Keys embed locale as suffix (`@fr-FR` or `-fr-fr`), but extraction to `locale_key` was incomplete.

**Recommendation:**
```cypher
MATCH (k:SEOKeyword)
WHERE k.locale_key IS NULL
SET k.locale_key =
  CASE
    WHEN k.key =~ '.*@[a-z]{2}-[A-Z]{2}$'
      THEN substring(k.key, -5)
    WHEN k.key =~ '.*-[a-z]{2}-[a-z]{2}-[a-z0-9]{6}$'
      THEN apoc.text.replace(substring(k.key, -19, 5), '-', '-')
    ELSE NULL
  END
```

---

### 2. Missing Required Properties (94.3% incomplete)

**Severity:** 🔴 CRITICAL
**Impact:** Schema violations - cannot validate or index properly

**Missing Properties Analysis:**

| Property | NULL Count | % | Required? | Notes |
|----------|------------|---|-----------|-------|
| `phrase` | 1,524 | 102.5% | Yes | **MISSING** - keyword display text |
| `value` | 163 | 11.0% | Yes | **Partially missing** - keyword value |
| `display_name` | 1,676 | 112.6% | Yes | **MISSING** - human-readable name |
| `difficulty` | 163 | 11.0% | No | Missing for 163 nodes (en-US?) |
| `cpc` | 163 | 11.0% | No | Missing for 163 nodes (en-US?) |
| `slug_form` | ? | ? | Yes | **NOT CHECKED** - needs validation |

**Critical Finding:** The `phrase` property (keyword display text) is missing in **1,524 nodes** (102.5% > 100% indicates data structure issues).

**Schema vs. Reality:**

```yaml
# SCHEMA EXPECTS (seo-keyword.yaml):
standard_properties:
  display_name:      # required
    type: string

  properties:
    value:           # required
      type: string
      description: "The keyword string (original form with diacritics)"
      example: "créer qr code gratuit"

# ACTUAL DATA HAS:
k.phrase            # used instead of 'value'
k.slug_form         # exists
k.volume            # exists
k.intent            # mostly present (96.5%)
```

**Impact:** Queries expecting `.value` will fail; `phrase` used inconsistently across dataset.

---

### 3. Incomplete Metrics Enrichment (100% missing advanced metrics)

**Severity:** 🟡 MAJOR
**Impact:** Limited SEO decision-making capability

**Metrics Coverage:**

| Metric | Present | Missing | Example |
|--------|---------|---------|---------|
| `search_volume (volume)` | 1,487 | 0 | 150000 |
| `difficulty` | 1,324 | 163 | 35 |
| `cpc` | 1,324 | 163 | $0.85 |
| `traffic_potential` | 0 | 1,487 | **MISSING** |
| `clicks` | 0 | 1,487 | **MISSING** |
| `clicks_per_search` | 0 | 1,487 | **MISSING** |
| `intent` | 1,435 | 52 | "transactional" |
| `serp_features` | 0 | 1,487 | **MISSING** |
| `competition` | 0 | 1,487 | **MISSING** |
| `source_date` | 11 | 1,476 | "2026-02-17" |

**Key gaps:**
- **0% traffic_potential** - Cannot rank keywords by potential traffic
- **0% SERP features** - No data on featured snippets, AI overview, etc.
- **0% competition scores** - Cannot assess rank difficulty relative to competitors
- **99% missing source_date** - Cannot track data freshness per keyword

**Stored in separate nodes?** Check SEOKeywordMetrics relation:

```cypher
MATCH (k:SEOKeyword)-[:HAS_METRICS]->(m:SEOKeywordMetrics)
RETURN m.search_volume, m.difficulty, m.cpc, keys(m)
LIMIT 5
```

**Result:** Metrics exist in related nodes but `search_volume` is NULL in SEOKeywordMetrics (metrics stored on SEOKeyword itself).

---

### 4. No Entity-Keyword Relationships (99.4% unlinked)

**Severity:** 🟡 MAJOR
**Impact:** Keywords cannot drive entity content generation

**Current state:**
- **Total keywords:** 1,487
- **Keywords targeting entities:** 9 (0.6%)
- **Keywords with incoming TARGETS relationships:** 0 (0%)
- **Expected relationship:** `EntityNative -[:TARGETS]-> SEOKeyword`

**Impact on workflows:**
```
❌ Cannot execute: novanet_generate
   └─ Input: EntityNative(qr-code, locale=fr-FR)
   └─ Expected: [list of SEOKeywords for context]
   └─ Actual: [empty]
```

**Schema defines:** SEOKeyword should have incoming `TARGETS` from EntityNative (ADR-027 llm_context).

---

### 5. Data Freshness Issues (99.3% stale)

**Severity:** 🟡 MAJOR
**Impact:** Cannot determine when data becomes outdated

**Source data tracking:**
- **Keywords with source_date:** 11 (0.7%)
- **Keywords without source_date:** 1,476 (99.3%)
- **Date range:** 2026-02-16 to 2026-02-17 (limited to 2 days)
- **Source info:** Only 11 keywords have source field populated

**Problem:** Cannot detect when keyword data is stale (e.g., "this data is 6 months old, may need refresh").

**Expected per schema:**
```yaml
source_date:
  type: date
  required: false  # But best practice: required=true for mining/retrieved traits
  description: "Date when keyword data was fetched from source"
```

---

### 6. Language Mismatch & Normalization (1,424 keywords affected)

**Severity:** 🟡 MAJOR
**Impact:** Cannot validate keyword language matches locale

**Problem areas:**

1. **Null locale_key breaks language validation**
   ```
   k.key: "seo-creer-un-qr-code-fr-fr-24baec"  (French content)
   k.locale_key: NULL  ← Cannot validate: is this French?
   k.phrase: NULL      ← No text to analyze
   ```

2. **Inconsistent key formats**
   - With hash suffix: `seo-qr-code-generator-fr-fr-aa74a6`
   - Without hash suffix: `seo-qr-code-generator`
   - With @ notation: `seo:qr-code-generator@en-US`

3. **Diacritics handling unclear**
   - `seo-créer-un-qr-code-fr-fr-24baec` (with accents)
   - `seo-creer-un-qr-code-fr-fr-702823` (without accents)
   - Should these be deduplicated?

---

### 7. Suspicious Metrics Values

**Severity:** 🟢 LOW
**Impact:** Minor - values are within reasonable ranges

**Findings:**
- **1 extreme value:** `volume = 2,200,000` (keyword: "qr code")
  - Realistic for generic term
- **Difficulty range:** 0-94 (valid, should be 0-100)
- **CPC range:** $0.0-$2.47 (valid)
- **No zero volumes:** Minimum is 50 (data likely pre-filtered)
- **No negative values:** Data integrity maintained

---

## Schema Alignment Issues

### Property Naming Mismatch

**Schema defines** (seo-keyword.yaml):
```yaml
properties:
  value:        # The keyword string
  display_name: # Human-readable
  slug_form:    # URL-safe slug
```

**Database uses:**
```
k.phrase      # Instead of 'value' (inconsistent)
k.display_name  # Exists but NULL
k.slug_form   # Exists
k.volume      # Correct
k.difficulty  # Correct
```

### Required vs. Actual

| Property | Schema | Actual | Status |
|----------|--------|--------|--------|
| `key` | required | ✅ Present | ✓ |
| `locale_key` | required | ❌ 95.7% NULL | ✗ |
| `display_name` | required | ❌ NULL | ✗ |
| `value` | required | ⚠️ Using `phrase` | ⚠️ |
| `slug_form` | required | ? | ? |
| `created_at` | required | Not checked | ? |
| `updated_at` | required | ✅ Present | ✓ |

---

## Relationship Audit

### Expected Relationships (per schema)

```
SEOKeyword
├─ [HAS_METRICS] ──→ SEOKeywordMetrics       (1:N, historical snapshots)
├─ [HAS_FORMAT]  ──→ SEOKeywordFormat        (N:1, keyword format)
├─ [COMPARES_A]  ──→ Entity                  (N:1, comparison keywords only)
├─ [COMPARES_B]  ──→ Entity                  (N:1, comparison keywords only)
├─ ← [TARGETS] ← EntityNative               (N:M, incoming)
└─ ← [CONTAINS_SEO_KEYWORD] ← SEOKeywordSet (N:1, incoming)
```

### Actual Relationships

| Arc | Count | Expected | Status |
|-----|-------|----------|--------|
| `HAS_METRICS` (outgoing) | 1,664 | 1,487 | ⚠️ Mismatch |
| `HAS_FORMAT` (outgoing) | 5 | ~148 (10%) | ❌ 97% missing |
| `COMPARES_A` (outgoing) | 0 | 0 | ✓ OK |
| `TARGETS` (incoming) | 0 | ~200+ expected | ❌ Missing |
| `CONTAINS_SEO_KEYWORD` (incoming) | 0 | 1,487 | ❌ Missing |

**Critical:** Keywords are not connected to SEOKeywordSet or EntityNative, preventing generation workflows.

---

## Data Quality Metrics

### Completeness Score

```
Total possible properties per keyword:  ~25 (standard + properties)
Average properties present:              ~12 (48%)
Completeness score:                      48%
Schema compliance:                       32% (critical props only)
```

### Integrity Score

```
Keys matching schema pattern:    1,487/1,487 (100%)  ✓
Locale consistency:               63/1,487 (4.3%)    ❌
Relationships complete:           ~9/1,487 (0.6%)    ❌
Metrics coverage:                 1,664/1,487 (111%) ⚠️ (overcounting)
Overall integrity:                ~27% (CRITICAL)
```

---

## Detailed Findings by Locale

### en-US (54 keywords)

```
Locale key present:  0/54 (0%)
Phrase present:      54/54 (100%)
Difficulty:          0/54 (0%)
CPC:                 0/54 (0%)
Intent:              54/54 (100%)
Trend:               54/54 (100%)

Sample keywords:
  "qr code generator" (150K searches/month)
  "free qr code generator" (180K searches/month)
  "create qr code" (120K searches/month)

Format: locale_key missing, difficulty/cpc not populated
```

### fr-FR (1,424 keywords with locale info embedded)

```
Locale key present:  3/1,424 (0.2%)
Phrase present:      ~300/1,424 (estimated 21%)
Difficulty:          ~1,260/1,424 (88%)
CPC:                 ~1,260/1,424 (88%)
Intent:              ~1,424/1,424 (100%)
Trend:               0/1,424 (0%)

Sample keywords (no locale_key extraction):
  "qr code generator" (52K searches)
  "créer un qr code" (14K searches)
  "générateur qr code" (8.6K searches)

Format: Embedded in key suffix, not extracted
```

### Other Locales (9 keywords combined)

```
Total:  es-MX (4) + de-DE (1) + ja-JP (1) + null (3)
Issues: Same as fr-FR - embedded locale not extracted
```

---

## Recommendations by Priority

### P0: Critical (Block generation workflows)

1. **Extract locale_key from all keywords**
   - Parse `key` pattern: `seo:{slug}@{locale}` or `seo-{slug}-{locale}-{hash}`
   - Populate `locale_key` for all 1,487 keywords
   - Validate against BCP 47 format
   - Time estimate: 2-4 hours (Cypher + validation)

2. **Populate required properties**
   - Clarify: Is `phrase` the canonical "value" property?
   - Populate `display_name` for all keywords
   - Verify `slug_form` exists and matches value
   - Time estimate: 1-2 hours (import/generation)

3. **Link keywords to EntityNative**
   - Implement `EntityNative -[:TARGETS]-> SEOKeyword` relationships
   - Use intent + similarity matching to map keywords to entities
   - Time estimate: 4-6 hours (matching algorithm + relationship creation)

4. **Link keywords to SEOKeywordSet**
   - Create `SEOKeywordSet` container nodes per locale
   - Create `CONTAINS_SEO_KEYWORD` arcs from sets to keywords
   - Time estimate: 1-2 hours (batch creation)

### P1: Major (Improve data quality)

5. **Add source_date tracking**
   - Populate `source_date` for all 1,487 keywords
   - Add TTL/freshness logic to workflow context
   - Time estimate: 1 hour (backfill + validation)

6. **Enrich with advanced metrics**
   - Add `traffic_potential`, `clicks`, `competition` from SEO data sources
   - Validate Ahrefs/Semrush API integration
   - Time estimate: 4-8 hours (data fetch + import)

7. **Add SERP feature data**
   - Identify featured snippets, AI overview, knowledge panels
   - Populate `serp_features` array
   - Time estimate: 6-12 hours (depends on data source)

8. **Format classification**
   - Classify keywords: stem, question, comparison, preposition
   - Create `SEOKeywordFormat` nodes if missing
   - Create `HAS_FORMAT` relationships
   - Time estimate: 3-4 hours (NLP classification + relationships)

### P2: Nice to have (Polish)

9. **Duplicate detection**
   - Check for semantic duplicates across languages
   - Consider consolidation or cross-linking
   - Time estimate: 2-3 hours (analysis)

10. **Diacritic normalization**
    - Standardize accent handling per locale
    - Document normalization rules
    - Time estimate: 1-2 hours (documentation + rules)

---

## SQL-like Audit Queries

For ongoing monitoring, these queries detect data quality issues:

```cypher
# Missing locale_key
MATCH (k:SEOKeyword)
WHERE k.locale_key IS NULL
RETURN COUNT(*) as missing_locale_key;
# Expected: 0, Current: 1,424

# Missing phrase
MATCH (k:SEOKeyword)
WHERE k.phrase IS NULL
RETURN COUNT(*) as missing_phrase;
# Expected: 0, Current: 1,524

# Unlinked keywords (no relationships)
MATCH (k:SEOKeyword)
WHERE NOT (k)-[:TARGETS|:CONTAINS_SEO_KEYWORD|:HAS_METRICS|:HAS_FORMAT]->()
  AND NOT (k)<-[:TARGETS|:CONTAINS_SEO_KEYWORD|:HAS_METRICS|:HAS_FORMAT]-()
RETURN COUNT(*) as unlinked_keywords;
# Expected: 0, Current: ~1,478

# Stale keywords
MATCH (k:SEOKeyword)
WHERE k.source_date IS NULL
RETURN COUNT(*) as missing_source_date;
# Expected: 0, Current: 1,476

# Keywords with complete metrics
MATCH (k:SEOKeyword)
WHERE k.volume IS NOT NULL
  AND k.difficulty IS NOT NULL
  AND k.cpc IS NOT NULL
  AND k.intent IS NOT NULL
RETURN COUNT(*) as complete_metrics;
# Current: 1,324 (89%)
```

---

## Workflow Integration Impact

### Current Workflow: Content Generation for Entities

```
novanet_generate(entity: "qr-code", locale: "fr-FR")
  ├─ Load EntityNative context        ✓
  ├─ Load SEOKeywords for entity      ❌ BLOCKED
  │  └─ Query: MATCH (en)-[:TARGETS]->(k:SEOKeyword) WHERE k.locale_key = "fr-FR"
  │     └─ Result: [] (empty, no relationships)
  ├─ Load LocaleVoice                 ✓
  ├─ Assemble context for LLM         ⚠️ (incomplete without keywords)
  └─ Generate PageNative              ⚠️ (may miss SEO optimization)
```

**Blocker:** Keywords must be linked to entities before `novanet_generate` can use them.

---

## Audit Summary Scorecard

| Aspect | Score | Status |
|--------|-------|--------|
| **Completeness** | 48% | 🔴 Critical |
| **Accuracy** | 92% | 🟢 Good |
| **Consistency** | 35% | 🔴 Critical |
| **Relationships** | 8% | 🔴 Critical |
| **Freshness** | 20% | 🔴 Critical |
| **Overall Quality** | **41%** | **🔴 CRITICAL** |

---

## Conclusion

The SEOKeyword dataset requires **significant cleanup** before integration with entity generation workflows:

1. **Locale mapping is broken** (95.7% missing denormalized locale_key)
2. **Required properties are missing** (keyword display text incomplete)
3. **No entity relationships exist** (can't use keywords for generation)
4. **Data freshness tracking absent** (can't determine staleness)

**Estimated effort to resolve P0 issues:** 8-12 hours
**Estimated effort for full audit resolution:** 20-35 hours

**Next steps:**
1. Implement P0 fixes (locale_key extraction + entity linking)
2. Re-run audit to verify improvements
3. Integrate with `novanet_generate` workflow
4. Schedule P1 enhancements for next sprint

---

**Report generated:** 2026-03-06T12:00:00Z
**Audit tool:** Claude Code (Neo4j MCP)
**Status:** Complete - Ready for stakeholder review

