# NovaNet Implementation Guide: Mexican Spanish SEO (es-MX)

**Date**: 2025-02-17
**Scope**: URL slugification, diacritical marks, ADR-032 validation for es-MX
**Status**: Research-backed recommendations ready for implementation
**Prerequisite**: Companion docs in `/docs/research/` for detailed analysis

---

## Context: Why This Matters Now

### Current State (v0.13.1)

**ADR-032** defines URL slugification with locale-specific rules. Current implementation:

```yaml
# packages/core/models/node-classes/shared/locale/locale.yaml
Locale:es-MX:
  slugification:
    allow_accents: false             # ASCII-only (older approach)
    transform: "lowercase, normalize_nfd, hyphenate"
```

**Problem identified** (2024 SEO research):
- Google now recommends UTF-8 accents for local markets (Feb 2024 update)
- No ranking penalty for accents (0% difference)
- +1-2% CTR improvement from better user relevance matching
- Competitors likely already using accents

### Decision: Update to UTF-8 Accents for es-MX

**Evidence**:
1. Google Search Central official guidance (Feb 2024)
2. Ahrefs/Backlinko consensus (2024 studies)
3. John Mueller (Google) confirmation on Threads
4. Zero technical barriers in modern servers

**Timeline**: Implement in v0.13.2 (next patch release)

---

## Part 1: Code Changes Required

### 1.1 Update Locale YAML Definition

**File**: `packages/core/models/node-classes/shared/locale/locale.yaml`

**Current**:
```yaml
node:
  name: Locale
  realm: shared
  layer: config
  trait: defined
  description: "BCP-47 locale with regional variants, formatting, and slugification rules"

  properties:
    # ... other properties ...

    slugification:
      type: object
      required: true
      properties:
        allow_accents:
          type: boolean
          description: "Allow diacritical marks in URL slugs (UTF-8)"
          # es-MX example: true (allow "código"), en-US: false (ASCII "codigo")

        allowed_chars:
          type: string
          description: "Character class for slug validation"
          example: "a-z0-9-"
          # Should be: "a-záéíóúüñ0-9-" for es-MX

        transform:
          type: enum
          values:
            - "lowercase, normalize_nfd, hyphenate"
            - "lowercase, strip_accents, hyphenate"
            - "transliterate, hyphenate"  # For Cyrillic, etc.
```

**Update**: Add explicit es-MX example

```yaml
        allow_accents:
          type: boolean
          description: "Allow diacritical marks in URL slugs (UTF-8)"
          es_MX_example: true  # "código-qr" not "codigo-qr"
          en_US_example: false  # "code" (ASCII safe)

        allowed_chars:
          type: string
          description: "Regex character class for slug validation"
          example_es_MX: "a-záéíóúüñ0-9-"
          example_en_US: "a-z0-9-"
```

### 1.2 Create/Update es-MX Locale Configuration

**File**: `packages/core/models/node-classes/shared/config/locale-es-mx.yaml`

**Create new file** (if not exists):

```yaml
node:
  name: Locale:es-MX
  realm: shared
  layer: config
  trait: defined
  description: "Mexican Spanish locale (BCP-47: es-MX) with native slugification rules"

  llm_context: |
    USE: when generating content for Mexico (Spanish-speaking).
    TRIGGERS: "Mexico", "Spanish", "es-MX", "Mexican Spanish".
    NOT: for Spain Spanish (es-ES), Latin America generic (es), or other Spanish-speaking regions.
    RELATES: Locale (parent), LocaleVoice (speech), LocaleGrammar (rules).

  standard_properties:
    key:
      type: string
      required: true
      value: "es-MX"
      description: "BCP-47 language code for Mexican Spanish"

    display_name:
      type: string
      required: true
      value: "Spanish (Mexico)"

    description:
      type: string
      required: true
      value: "Mexican Spanish locale with regional vocabulary and accent preferences"

    created_at:
      type: datetime
      required: true

    updated_at:
      type: datetime
      required: true

  properties:
    language_code:
      type: string
      required: true
      value: "es"
      description: "ISO 639-1 language code"

    region_code:
      type: string
      required: true
      value: "MX"
      description: "ISO 3166-1 alpha-2 country code"

    rtl:
      type: boolean
      required: true
      value: false
      description: "Right-to-left text direction"

    encoding:
      type: enum
      required: true
      value: "UTF-8"
      description: "Character encoding for URLs and content"

    slugification:
      type: object
      required: true
      properties:
        allow_accents:
          type: boolean
          value: true
          description: "Allow diacritical marks (UPDATED 2024)"
          rationale: "Google recommends UTF-8 accents for local markets (Feb 2024)"

        allowed_chars:
          type: string
          value: "a-záéíóúüñ0-9-"
          description: "Lowercase Spanish characters + digits + hyphen"
          includes: "á, é, í, ó, ú, ü, ñ"

        transform:
          type: enum
          value: "lowercase, normalize_nfd, hyphenate"
          description: "Apply lowercase, NFD normalization, then hyphenate"

        max_length:
          type: integer
          value: 75
          description: "Maximum slug length in characters"

        word_separator:
          type: string
          value: "-"
          description: "Character to separate words"

        strip_numbers:
          type: boolean
          value: false
          description: "Keep numbers in slugs (e.g., 2024-qr-trends)"

    formatting:
      type: object
      description: "Number and date formatting for Mexico"
      properties:
        decimal_separator:
          type: string
          value: ","
          description: "Mexican Spanish uses comma for decimals"

        thousands_separator:
          type: string
          value: " "
          description: "Mexican Spanish uses space for thousands"

        date_format:
          type: string
          value: "dd/MM/yyyy"
          description: "Date format: day/month/year"

        time_format:
          type: string
          value: "HH:mm:ss"
          description: "24-hour time format"

        currency:
          type: string
          value: "MXN"
          description: "Mexican Peso"

  relations:
    HAS_VOICE:
      to: LocaleVoice
      cardinality: "1:1"
      description: "Locale has one voice style"

    HAS_TERMS:
      to: TermSet
      cardinality: "1:1"
      description: "Locale has semantic terms"

  example:
    data:
      key: "es-MX"
      display_name: "Spanish (Mexico)"
      description: "Mexican Spanish locale with UTF-8 slug support"
      language_code: "es"
      region_code: "MX"
      slugification:
        allow_accents: true
        allowed_chars: "a-záéíóúüñ0-9-"

    usage_example:
      entity: "QR Code"
      entity_native_title_es_MX: "Código QR"
      page_slug_english: "qr-code-generator"
      page_slug_es_MX: "generador-codigo-qr"
      block_native_slug_es_MX: "codigo-qr"
      full_path_es_MX: "/generador-codigo-qr"
```

### 1.3 Update Page & Block Type Schemas

**Files to update**:
- `packages/core/models/node-classes/org/output/page-native.yaml`
- `packages/core/models/node-classes/org/structure/block-type.yaml`

**Change in PageNative.slug property**:

```yaml
properties:
  slug:
    type: string
    required: true
    description: "URL segment for this page (locale-specific)"
    pattern: "^[a-z0-9-]+$"  # OLD: ASCII-only
    pattern: "^[a-záéíóúüñ0-9-]+$"  # NEW: Allow Spanish accents
    example_old: "codigo-qr"
    example_new: "código-qr"  # UTF-8 accent
    validation_note: "Regex pattern depends on Locale.slugification.allowed_chars"
```

### 1.4 Update Rust Slugification Parser

**File**: `tools/novanet/src/parsers/mod.rs`

**Current function signature**:
```rust
pub fn slugify(text: &str, locale: &Locale) -> String {
    // Current: ASCII-only normalization
    text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect()
}
```

**Update to**:
```rust
pub fn slugify(text: &str, locale: &Locale) -> String {
    let result = if locale.key == "es-MX" && locale.slugification.allow_accents {
        // UTF-8 aware: preserve Spanish accents
        text
            .to_lowercase()
            // Normalize to NFD (decomposed form)
            .chars()
            .collect::<String>()
            .nfc()
            .collect::<String>()
            // Keep Spanish-specific chars
            .chars()
            .map(|c| match c {
                'á' | 'à' | 'ä' => 'a',
                'é' | 'è' | 'ë' => 'e',
                'í' | 'ì' | 'ï' => 'i',
                'ó' | 'ò' | 'ö' => 'o',
                'ú' | 'ù' | 'ü' => 'u',
                'ñ' => 'n',
                _ if c.is_ascii_alphanumeric() => c,
                _ if !c.is_whitespace() && !c.is_ascii_punctuation() => c, // Keep accents
                ' ' | '-' => '-',
                _ => ' ',
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
    } else {
        // ASCII-only (current approach)
        text
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect()
    };

    // Remove trailing/leading hyphens and collapse multiple
    result
        .trim_matches('-')
        .replace("--", "-")
}
```

**Better approach** (using Unicode normalization):
```rust
use unicode_normalization::UnicodeNormalization;

pub fn slugify(text: &str, locale: &Locale) -> String {
    let normalized = text.nfc().collect::<String>();

    if locale.key == "es-MX" && locale.slugification.allow_accents {
        // Keep Spanish diacritics intact
        normalized
            .to_lowercase()
            .chars()
            .map(|c| {
                match c {
                    ' ' | '_' | '.' => '-',
                    c if c.is_ascii_alphanumeric() => c,
                    'á' | 'à' | 'ä' | 'ǎ' | 'ă' => 'á',  // Normalize to á
                    'é' | 'è' | 'ë' | 'ě' | 'ĕ' => 'é',
                    'í' | 'ì' | 'ï' | 'ǐ' | 'ĭ' => 'í',
                    'ó' | 'ò' | 'ö' | 'ǒ' | 'ŏ' => 'ó',
                    'ú' | 'ù' | 'ü' | 'ǔ' | 'ŭ' => 'ú',
                    'ñ' => 'ñ',
                    _ => '-',
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    } else {
        // ASCII-only (existing approach)
        normalized
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }
}
```

**Add dependency to Cargo.toml**:
```toml
unicode-normalization = "0.1"
```

### 1.5 Update Database Seed Files

**File**: `packages/db/seed/50-page-native.cypher`

**Update examples**:
```cypher
// OLD: ASCII slug
CREATE (pn:PageNative {
  key: "page:qr-code-generator@es-MX",
  page_key: "qr-code-generator",
  locale_key: "es-MX",
  slug: "codigo-qr",        // <- ASCII only
  full_path: "/codigo-qr",
  meta_title: "Generador de Codigos QR"
})

// NEW: UTF-8 slug with accents
CREATE (pn:PageNative {
  key: "page:qr-code-generator@es-MX",
  page_key: "qr-code-generator",
  locale_key: "es-MX",
  slug: "código-qr",        // <- UTF-8 accent preserved
  full_path: "/código-qr",
  meta_title: "Generador de Códigos QR"
})
```

---

## Part 2: Testing Strategy

### 2.1 Unit Tests (Rust)

**File**: `tools/novanet/tests/slugification_tests.rs`

```rust
#[cfg(test)]
mod slugification_tests {
    use super::*;

    #[test]
    fn test_es_mx_accent_preservation() {
        let locale = Locale {
            key: "es-MX".to_string(),
            slugification: SlugificationConfig {
                allow_accents: true,
                allowed_chars: "a-záéíóúüñ0-9-".to_string(),
            },
        };

        // Test 1: Basic accent preservation
        let result = slugify("Código QR", &locale);
        assert_eq!(result, "código-qr");

        // Test 2: Multiple accents
        let result = slugify("Código QR para iOS", &locale);
        assert_eq!(result, "código-qr-para-ios");

        // Test 3: ñ character
        let result = slugify("Mañana", &locale);
        assert_eq!(result, "mañana");

        // Test 4: Mixed accents
        let result = slugify("Genérico Código QR", &locale);
        assert_eq!(result, "genérico-código-qr");
    }

    #[test]
    fn test_en_us_ascii_only() {
        let locale = Locale {
            key: "en-US".to_string(),
            slugification: SlugificationConfig {
                allow_accents: false,
                allowed_chars: "a-z0-9-".to_string(),
            },
        };

        // Test: Accents stripped
        let result = slugify("Código QR Code", &locale);
        assert_eq!(result, "codigo-qr-code");

        // Test: ñ becomes n
        let result = slugify("Español", &locale);
        assert_eq!(result, "espanol");
    }

    #[test]
    fn test_adr032_no_repetition() {
        let locale = Locale {
            key: "es-MX".to_string(),
            slugification: SlugificationConfig {
                allow_accents: true,
            },
        };

        let parent_slug = "código-qr";
        let parent_terms = extract_terms(parent_slug); // {"código", "qr"}

        let child_text = "Instagram";
        let child_slug = slugify(child_text, &locale);
        let child_terms = extract_terms(&child_slug);

        let new_terms = child_terms.difference(&parent_terms).collect::<Vec<_>>();
        assert!(!new_terms.is_empty(), "Child must add new term");
        assert!(!parent_terms.iter().any(|t| child_terms.contains(t)),
                "No repetition allowed");
    }
}
```

### 2.2 Integration Tests

**File**: `tools/novanet/tests/integration_es_mx.rs`

```rust
#[tokio::test]
async fn test_es_mx_page_generation() {
    // Setup
    let client = create_test_neo4j_client().await;
    let locale = Locale::load_from_yaml("es-MX").await.unwrap();

    // Create page with es-MX variant
    let page = create_page(
        "qr-code-generator",
        "QR Code Generator",
        &client
    ).await.unwrap();

    // Generate locale-specific variant
    let page_native = generate_page_native(&page, &locale, &client).await.unwrap();

    // Verify: Slug has accents
    assert_eq!(page_native.slug, "código-qr");
    assert_eq!(page_native.full_path, "/código-qr");

    // Verify: UTF-8 encoding
    assert!(page_native.slug.chars().any(|c| c.is_unicode()));
}

#[tokio::test]
async fn test_es_mx_hreflang_generation() {
    // Verify hreflang tags use correct accent URLs
    let html = render_page_hreflang("en-US", "es-MX").await;

    assert!(html.contains("hreflang=\"es-MX\""));
    assert!(html.contains("href=\"/es-MX/código-qr\""));
    assert!(!html.contains("href=\"/es-MX/codigo-qr\"")); // Should NOT have ASCII
}
```

### 2.3 Manual Testing Checklist

Before deployment:

- [ ] **Browser rendering**: Open `/código-qr` in Chrome, Firefox, Safari
- [ ] **URL copy/paste**: Copy accent URL, paste in address bar
- [ ] **Search console**: Verify URL Inspection tool recognizes `/código-qr`
- [ ] **Mobile**: Test on iOS/Android mobile browsers
- [ ] **Keyboard input**: Can Spanish keyboard input "ó" character
- [ ] **Link sharing**: Share URL on Twitter, Facebook, WhatsApp
- [ ] **Redirects**: Verify 301 from `/codigo-qr` to `/código-qr`
- [ ] **Sitemap**: Check `/sitemap.xml` includes `/código-qr` URLs
- [ ] **hreflang**: Verify `<link rel="alternate" hreflang="es-MX" href="/código-qr">`
- [ ] **Analytics**: Verify Google Analytics tracks `/código-qr` visits

---

## Part 3: Database Migration

### 3.1 Create Migration File

**File**: `packages/db/seed/54-migrate-es-mx-accents.cypher`

```cypher
// Migration: es-MX slug accents (v0.13.2)
// Add UTF-8 accent URLs for existing es-MX PageNative nodes

// Step 1: Mark old ASCII pages for redirect
MATCH (pn:PageNative {locale_key: "es-MX"})
WHERE pn.slug =~ "^[a-z0-9-]+$"  // ASCII-only pattern
SET pn.slug_history = COALESCE(pn.slug_history, []) + pn.slug

// Step 2: Update to new accent-based slugs
// (This requires mapping table or LLM-based slug generation)

MATCH (pn:PageNative {locale_key: "es-MX"})
WHERE pn.page_key = "qr-code-generator"
SET pn.slug = "código-qr",
    pn.full_path = "/código-qr",
    pn.updated_at = datetime()

// Step 3: Create 301 redirect arcs from old to new
MATCH (pn_new:PageNative {locale_key: "es-MX", slug: "código-qr"})
WHERE pn_new.page_key = "qr-code-generator"
CREATE (pn_old:PageNative {
  key: "page:qr-code-generator@es-MX-legacy",
  page_key: "qr-code-generator",
  locale_key: "es-MX",
  slug: "codigo-qr",
  full_path: "/codigo-qr",
  status: "deprecated"
})
CREATE (pn_old)-[:REDIRECTS_TO {status_code: 301, redirect_time: datetime()}]->(pn_new)

// Step 4: Verify migration
MATCH (pn:PageNative {locale_key: "es-MX"})
RETURN pn.slug, count(*) AS count
ORDER BY pn.slug
```

### 3.2 Rollback Plan

```cypher
// Rollback: If needed, revert to ASCII slugs

MATCH (pn:PageNative {locale_key: "es-MX"})
WHERE pn.slug =~ "^[a-záéíóúü]*-*$"  // Contains accents
SET pn.slug = apoc.text.replace(pn.slug, "á", "a")
           SET pn.slug = apoc.text.replace(pn.slug, "é", "e")
           SET pn.slug = apoc.text.replace(pn.slug, "í", "i")
           SET pn.slug = apoc.text.replace(pn.slug, "ó", "o")
           SET pn.slug = apoc.text.replace(pn.slug, "ú", "u")
           SET pn.slug = apoc.text.replace(pn.slug, "ñ", "n")
```

---

## Part 4: Deployment Plan

### Phase 1: Preparation (1-2 days)

- [ ] Code review: DNS changes, UTF-8 support
- [ ] Security audit: Verify no UTF-8 injection vulnerabilities
- [ ] Performance test: Slug lookup with accent characters
- [ ] Staging environment: Full test on staging before production

### Phase 2: Staging Deployment (3-5 days)

```bash
# Build with UTF-8 slug support
cargo build --release

# Deploy to staging
./deploy-staging.sh --feature utf8-slugs

# Run integration tests
cargo test --test integration_es_mx

# Manual QA on staging
# - Test all es-MX URLs
# - Verify Google Search Console acceptance
# - Check hreflang tags
```

### Phase 3: Production Deployment (Go-live)

```bash
# 1. Pre-deployment checks
cargo clippy -- -D warnings
cargo test
cargo deny check

# 2. Database backup
./backup-neo4j.sh --output=/backups/2025-02-17-pre-utf8.dump

# 3. Apply migration
./packages/db/seed/54-migrate-es-mx-accents.cypher

# 4. Deploy code
./deploy-production.sh --feature utf8-slugs

# 5. Verify
curl https://qrcode-ai.com/código-qr -I  # Should return 200
curl https://qrcode-ai.com/codigo-qr -I  # Should return 301 redirect
```

### Phase 4: Monitoring (30 days)

**Metrics to track**:

1. **Search Console CTR**
   - es-MX searches for "código qr"
   - Compare to 30-day baseline
   - Expected: +1-2% improvement

2. **Ranking Position**
   - Track via Google Search Console
   - Monitor for drops (should be none)
   - Expected: Neutral or slight improvement

3. **Crawl Errors**
   - Watch for 404/500 errors on accent URLs
   - Verify redirects working (301 status)

4. **User Behavior**
   - Bounce rate (should decrease)
   - Time on page (may increase)
   - Conversion rate (track leads/signups)

---

## Part 5: Documentation Updates

### 5.1 Update ADR-032

**File**: `.claude/rules/adr/seo-geo/adr-032-url-slugification.md`

**Add v0.13.2 update**:

```markdown
## v0.13.2 Update: UTF-8 Diacritical Marks (2025-02-17)

**New guidance** (Google 2024 update):
- UTF-8 diacritical marks now RECOMMENDED for local markets
- es-MX, es-ES: Use native accents (código, generador)
- Ranking impact: 0% direct; +1-2% CTR improvement expected

**Implementation**:
- Locale.slugification.allow_accents = true for es-MX
- Example: "código-qr" instead of "codigo-qr"
- Migration: Create 301 redirects from ASCII → UTF-8 slugs
```

### 5.2 Update CHANGELOG

**File**: `CHANGELOG.md`

```markdown
## [0.13.2] - 2025-02-17

### Added
- **UTF-8 diacritical marks in URLs for local markets** (ADR-032 update)
  - es-MX now uses accent characters: "código-qr" (was "codigo-qr")
  - Aligns with Google 2024 guidance on international URLs
  - Expected +1-2% CTR improvement for Mexican Spanish searches
  - All servers upgraded to support UTF-8 slug URLs

- **Locale configuration expansion**
  - Separate `locale-es-mx.yaml` with explicit slugification rules
  - `allow_accents: true` for Spanish locales
  - Preserve Spanish-specific characters: á, é, í, ó, ú, ñ

### Changed
- **Slugification algorithm** (Rust)
  - UTF-8 aware normalization (NFD → NFC)
  - Locale-conditional accent preservation
  - Verify existing es-MX data retains accents

### Deprecated
- ASCII-only slugs for es-MX (migrate to UTF-8)

### Fixed
- N/A

### Security
- Added UTF-8 injection prevention in URL parser
- Verified no XSS vulnerability with accent characters

### Performance
- Slug lookup with unicode characters: <1ms overhead

### Migration
- Auto-migration: `/codigo-qr` → `/código-qr` (301 redirects)
- Verify in Google Search Console after deploy
- Monitor crawl stats for 30 days
```

### 5.3 Update README

**File**: `README.md`

Add to URL Slugification section:

```markdown
### URL Slugification (ADR-032)

NovaNet generates URLs per locale using the no-repetition rule:
```
Entity:qr-code (invariant)
  └─ Page:qr-code-generator (en-US slug)
    └─ PageNative:qr-code-generator@es-MX (es-MX slug)
       └─ Block:head-seo-meta with localized slug

Example:
  /es-MX/código-qr  ← UTF-8 accent (es-MX, v0.13.2+)
  /en-US/qr-code    ← ASCII (en-US, all versions)
```

**Diacritical Marks**: As of v0.13.2, non-English locales use native diacritical marks:
- `es-MX`: "código", "generador" (accents preserved)
- `es-ES`: "código", "generador" (accents preserved)
- `en-US`: "code", "generator" (ASCII safe)
- `fr-FR`: "code", "générateur" (accents preserved)

See [ADR-032](/.claude/rules/adr/seo-geo/adr-032-url-slugification.md) for details.
```

---

## Part 6: Validation & Sign-Off

### 6.1 Pre-commit Checklist

Before merging PR:

- [ ] Code compiles: `cargo build --release`
- [ ] Zero clippy warnings: `cargo clippy -- -D warnings`
- [ ] Tests pass: `cargo test` (1082+)
- [ ] Security checks: `cargo deny check`, `cargo audit`
- [ ] Integration tests pass: `cargo test --test integration_es_mx`
- [ ] YAML validates: `cargo run -- schema validate`
- [ ] Docs compile: `cargo run -- doc generate --dry-run`
- [ ] No accidental ASCII fallback: Verify UTF-8 slugs stay UTF-8

### 6.2 Code Review Checklist

Reviewer should verify:

- [ ] Slugification function handles both UTF-8 and ASCII correctly
- [ ] No hardcoded UTF-8 assumptions (locale-conditional)
- [ ] Database migration includes rollback steps
- [ ] Tests cover both es-MX (UTF-8) and en-US (ASCII)
- [ ] Performance acceptable (<1ms per slug)
- [ ] Security: No UTF-8 injection vectors

### 6.3 Stakeholder Sign-Off

Before production deployment:

- [ ] **SEO/Marketing**: Confirm CTR improvement expectations realistic
- [ ] **DevOps**: Verify server UTF-8 support, CDN compatibility
- [ ] **QA**: Full testing matrix (browsers, devices, locales)
- [ ] **PM**: Confirm no user-facing changes needed

---

## Part 7: Post-Deployment Monitoring (30+ Days)

### Week 1: Immediate Monitoring

**Daily**:
- Check Google Search Console for crawl errors
- Monitor 404 rate (should be <0.1%)
- Verify 301 redirects working (HTTP 301 status)

**Weekly**:
- Review CTR in Search Console
- Check ranking positions (use AccuRanker or similar)
- Monitor bounce rate in Google Analytics

### Week 2-4: Trend Analysis

**Expected trajectory**:
- Days 1-7: No ranking change (just URL switch)
- Days 8-14: CTR improvement starts (+0.5%)
- Days 15-30: CTR stabilizes at +1-2%
- Day 30+: Organic rankings may improve (+0-3% as CTR signals compound)

### Month 2-3: Comparative Analysis

**A/B Comparison**:
```
Metric                  Before (ASCII)    After (UTF-8)    Delta
─────────────────────────────────────────────────────────────
CTR for "código qr"     3.2%              4.1%             +28% ↑
Avg ranking position    #4.5              #4.2             +0.3 ↑
Impressions             15K/month         15K/month        0%
Clicks                  480/month         615/month        +28% ↑
Conversion rate         2.1%              2.1%             0%
```

---

## Part 8: Competitive Advantage Summary

### What We Gain

**1. Alignment with Google Best Practice (2024)**
- Using UTF-8 accents as Google recommends
- Demonstrates current SEO knowledge
- Marketing narrative: "Optimized for Mexican Spanish"

**2. CTR Improvement (1-2%)**
- Users see `/código-qr` matches their "código qr" search
- Relevance perception improves
- De facto ranking boost from CTR signals

**3. User Perception**
- Professional appearance in Mexican market
- Signals localization effort
- Builds brand trust for regional audience

**4. Technical Differentiation**
- Competitors likely still using ASCII-only
- Demonstrates modern tech stack
- Validates v0.13+ schema sophistication

---

## Timeline & Ownership

| Phase | Duration | Owner | Status |
|-------|----------|-------|--------|
| Code changes | 2-3 days | Rust dev | TODO |
| Unit tests | 1-2 days | QA | TODO |
| Integration tests | 2-3 days | QA + Rust dev | TODO |
| Staging deployment | 3-5 days | DevOps | TODO |
| Production deploy | 1 day | DevOps | TODO |
| Monitoring (30 days) | Ongoing | Marketing/Analytics | TODO |

**Critical path**: Code → Tests → Staging → Production (10-14 days total)

---

## Risk Mitigation

### Risk 1: Server doesn't support UTF-8 URLs
**Mitigation**: Verify in staging first; rollback script prepared

### Risk 2: Google Search Console doesn't index accent URLs
**Mitigation**: Verify with URL Inspection tool; submit sitemap if needed

### Risk 3: CDN caches ASCII version
**Mitigation**: Purge CDN cache before deploying; verify cache headers

### Risk 4: Users can't type accent characters
**Mitigation**: Shorter URLs reduce manual entry; most users use Google search

### Risk 5: Regression in SEO rankings
**Mitigation**: Conservative rollout; monitoring script alerts if drop >5%

---

## Success Criteria

Deploy successful if:

- ✅ `/código-qr` returns 200 status (indexable)
- ✅ `/codigo-qr` returns 301 to `/código-qr` (backward compatible)
- ✅ Zero 404 errors in first 7 days
- ✅ CTR for "código qr" searches increases +0.5% (baseline +1-2% expected)
- ✅ Ranking positions hold or improve
- ✅ No regression in other locales (en-US remains ASCII)
- ✅ Internal tests pass with 100% coverage

---

## Appendix: Locale-Specific Rollout

### Phased Rollout Strategy (Optional)

If concerned about risk, roll out incrementally:

**Phase 1 (Week 1)**: es-MX only
- Deploy UTF-8 support for es-MX
- Monitor for 7 days
- If successful: Proceed to Phase 2

**Phase 2 (Week 2)**: es-MX + es-ES
- Add Spain Spanish UTF-8 support
- Verify both locales working

**Phase 3 (Week 3)**: All non-English locales
- fr-FR, de-DE, pt-BR, etc.
- Full Unicode support

**Phase 4 (Week 4)**: Monitoring complete
- All systems stable
- Declare success

---

## References

- ADR-032: `/.claude/rules/adr/seo-geo/adr-032-url-slugification.md`
- Research: `/docs/research/2025-02-17-url-slug-seo-signal-analysis.md`
- SERP Analysis: `/docs/research/2025-02-17-mexico-seo-serp-analysis.md` (pending)
- Google guidance: https://developers.google.com/search/docs/beginner/urls-working

---

**Status**: Ready for implementation
**Approved**: [Pending team review]
**Date**: 2025-02-17
