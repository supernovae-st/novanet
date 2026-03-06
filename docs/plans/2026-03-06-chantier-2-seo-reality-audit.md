# Chantier 2: SEO vs Reality Audit - Wikipedia Trap Detection

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Detect and fix "Wikipedia traps" where academic terms don't match real user search behavior

**Architecture:** Use web research (Perplexity, Ahrefs, Google Trends) to discover REAL search volumes and compare against our EntityNative denomination_forms. We don't have SEOKeyword data yet - we need to gather it.

**Tech Stack:** Perplexity MCP, Firecrawl MCP, Web research, NovaNet MCP (novanet_query for current state)

---

## Context

### The Wikipedia Trap Problem

Wikipedia and academic sources often use "official" terminology that differs from real-world usage:

| Language | Wikipedia Says | People Actually Search |
|----------|----------------|----------------------|
| French | "code QR" | "QR code" |
| German | "QR-Kode" | "QR-Code" |
| Japanese | "QRコード" | "QRコード" ✓ (same) |
| Spanish | "código QR" | "código QR" ✓ (same) |

**Spanish "código QR" is GOOD** - it's actually used and sounds natural. Unlike French "code QR" which sounds academic and wrong.

### Why This Matters

If our EntityNatives use terms that nobody searches for:
1. SEO suffers (wrong keywords)
2. Content sounds unnatural
3. Users don't find us

---

## Task 1: Extract Current SEO Keywords Data

**Files:** None (MCP queries only)

**Step 1: Query all SEOKeywords with volume**

```
novanet_query cypher="
MATCH (k:SEOKeyword)-[:FOR_LOCALE]->(l:Locale)
WHERE k.search_volume > 0
RETURN k.key, k.keyword, k.search_volume, l.key as locale
ORDER BY k.search_volume DESC
LIMIT 100
"
```

**Step 2: Document top keywords per locale**

Create a working document with actual search volumes.

---

## Task 2: Extract EntityNative denomination_forms

**Step 1: Query denomination_forms for QR-related entities**

```
novanet_query cypher="
MATCH (en:EntityNative)-[:FOR_LOCALE]->(l:Locale)
WHERE en.key CONTAINS 'qr-code' OR en.key CONTAINS 'qr'
RETURN en.key, en.denomination_forms, l.key as locale
ORDER BY l.key, en.key
"
```

**Step 2: Extract text forms for comparison**

Build a comparison table: what we say vs what people search.

---

## Task 3: Cross-Reference SEO vs EntityNative

**Step 1: Find mismatches**

```
novanet_query cypher="
// Find EntityNatives whose text form doesn't match any SEOKeyword
MATCH (en:EntityNative)-[:FOR_LOCALE]->(l:Locale)
WHERE en.denomination_forms IS NOT NULL
OPTIONAL MATCH (k:SEOKeyword)-[:FOR_LOCALE]->(l)
WHERE k.keyword CONTAINS en.denomination_forms.text
RETURN
  en.key,
  en.denomination_forms.text as our_term,
  l.key as locale,
  CASE WHEN k IS NULL THEN 'NO_MATCH' ELSE k.keyword END as seo_match,
  CASE WHEN k IS NULL THEN 0 ELSE k.search_volume END as volume
ORDER BY volume DESC
"
```

**Expected output:** Table showing where our terms don't match search behavior.

---

## Task 4: Validate French "QR code" vs "code QR"

**Step 1: Check French SEO data**

```
novanet_query cypher="
MATCH (k:SEOKeyword)-[:FOR_LOCALE]->(l:Locale {key: 'fr-FR'})
WHERE k.keyword CONTAINS 'qr' OR k.keyword CONTAINS 'QR'
RETURN k.keyword, k.search_volume
ORDER BY k.search_volume DESC
"
```

**Step 2: Check French EntityNative forms**

```
novanet_query cypher="
MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
RETURN en.denomination_forms
"
```

**Step 3: Web research validation**

Use web search to verify:
- Google Trends: "QR code" vs "code QR" in France
- Actual French websites: what do they use?

**Expected finding:** "QR code" is more searched than "code QR" in France.

---

## Task 5: Validate Spanish "código QR"

**Step 1: Check Spanish SEO data**

```
novanet_query cypher="
MATCH (k:SEOKeyword)-[:FOR_LOCALE]->(l:Locale)
WHERE l.key IN ['es-ES', 'es-MX', 'es-AR', 'es-CO']
AND (k.keyword CONTAINS 'qr' OR k.keyword CONTAINS 'QR' OR k.keyword CONTAINS 'código')
RETURN k.keyword, k.search_volume, l.key
ORDER BY k.search_volume DESC
"
```

**Step 2: Verify Spanish forms are correct**

Spanish "código QR" should be KEPT because:
- It's actually used in real searches
- It sounds natural to Spanish speakers
- It's not a Wikipedia-only term

**Expected finding:** Spanish forms are correct, no changes needed.

---

## Task 6: Audit German QR Terms

**Step 1: Check German SEO data**

```
novanet_query cypher="
MATCH (k:SEOKeyword)-[:FOR_LOCALE]->(l:Locale)
WHERE l.key IN ['de-DE', 'de-AT', 'de-CH']
AND (k.keyword CONTAINS 'qr' OR k.keyword CONTAINS 'QR')
RETURN k.keyword, k.search_volume, l.key
ORDER BY k.search_volume DESC
"
```

**Step 2: Web research for German usage**

Verify if Germans say:
- "QR-Code" (most likely)
- "QR-Kode" (academic)
- "QR Code" (English import)

---

## Task 7: Audit Japanese QR Terms

**Step 1: Check Japanese SEO data**

```
novanet_query cypher="
MATCH (k:SEOKeyword)-[:FOR_LOCALE]->(l:Locale {key: 'ja-JP'})
WHERE k.keyword CONTAINS 'QR' OR k.keyword CONTAINS 'キュー'
RETURN k.keyword, k.search_volume
ORDER BY k.search_volume DESC
"
```

**Step 2: Verify Japanese forms**

Japanese should use "QRコード" which is correct.

---

## Task 8: Create Findings Report

**Files:**
- Create: `docs/audits/2026-03-06-seo-reality-audit.md`

**Step 1: Document all findings**

```markdown
# SEO vs Reality Audit - 2026-03-06

## Summary

| Locale | Our Term | Real Usage | Action |
|--------|----------|------------|--------|
| fr-FR | code QR | QR code | FIX |
| es-MX | código QR | código QR | OK |
| de-DE | QR-Code | QR-Code | OK |
| ja-JP | QRコード | QRコード | OK |

## Detailed Findings

### French (fr-FR)
- Wikipedia says: "code QR"
- Google searches: "QR code" (10x more volume)
- Real websites: Use "QR code"
- **Action:** Update denomination_forms.text to "QR code"

### Spanish (es-MX, es-ES, es-AR, es-CO)
- Wikipedia says: "código QR"
- Google searches: "código QR" (matches!)
- Real websites: Use "código QR"
- **Action:** KEEP as-is, this is correct

...
```

---

## Task 9: Fix French denomination_forms

**Files:**
- Create: `brain/seed/migrations/034-fix-french-qr-terms.cypher`

**Step 1: Update French EntityNative**

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// MIGRATION 034: Fix French QR Terminology (Wikipedia Trap)
// ═══════════════════════════════════════════════════════════════════════════════
// Issue: "code QR" is academic, nobody uses it
// Fix: Use "QR code" which is what people actually search

MATCH (en:EntityNative {key: 'entity:qr-code@fr-FR'})
SET en.denomination_forms = {
  text: 'QR code',           // What people actually say
  title: 'QR Code',          // Capitalized for headers
  abbrev: 'QR',              // Short form
  url: 'qr-code'             // URL-safe
},
en.updated_at = datetime();

// Also update any expressions that use "code QR"
MATCH (e:Expression)
WHERE e.key STARTS WITH 'fr-FR/'
AND e.text CONTAINS 'code QR'
SET e.text = replace(e.text, 'code QR', 'QR code'),
    e.updated_at = datetime();
```

---

## Task 10: Verify Fixes

**Step 1: Run novanet_audit**

```
novanet_audit target=all
```

**Step 2: Query updated EntityNatives**

```
novanet_query cypher="
MATCH (en:EntityNative)
WHERE en.key CONTAINS '@fr-FR'
RETURN en.key, en.denomination_forms
"
```

---

## Task 11: Commit and Push

**Step 1: Commit audit report and migration**

```bash
cd /Users/thibaut/dev/supernovae/brain

git add docs/audits/2026-03-06-seo-reality-audit.md
git add seed/migrations/034-fix-french-qr-terms.cypher

git commit -m "fix(i18n): fix French QR terminology (Wikipedia trap)

French 'code QR' is academic Wikipedia terminology that nobody uses.
Real French users search for 'QR code' (borrowed English term).

Audit findings:
- fr-FR: code QR → QR code (FIXED)
- es-MX: código QR → OK (actually used)
- de-DE: QR-Code → OK (actually used)
- ja-JP: QRコード → OK (actually used)

Co-Authored-By: Claude <noreply@anthropic.com>
Co-Authored-By: Nika <nika@supernovae.studio>"
```

---

## Success Criteria

- [ ] All major locales audited for Wikipedia traps
- [ ] SEO data used as ground truth for real usage
- [ ] French QR terminology fixed
- [ ] Audit report documented
- [ ] No other locales need fixing (Spanish, German, Japanese OK)
- [ ] Changes committed and pushed

---

## Future Work

This audit process should be repeated for:
- Other entity types (barcode, landing-page, etc.)
- Other high-volume keywords
- New locales added to the system

Consider automating with a periodic novanet_audit extension that compares denomination_forms against SEOKeyword search volumes.
