# SEO vs Reality Audit Results

> **Date:** 2026-03-06
> **Auditor:** Claude (using Perplexity web research)
> **Purpose:** Verify EntityNative terminology matches real-world search behavior

## Executive Summary

**Status: VALIDATED** - Our French (fr-FR) terminology is correct.

Key finding: **"QR code" is the dominant term globally**, even in non-English markets. The "Wikipedia trap" (academic translations like "code QR") does NOT reflect real user behavior.

## Audit Results by Language

| Locale | Our DB Term | Real Search Term | Status | Notes |
|--------|-------------|------------------|--------|-------|
| **en-US** | "QR code" | "QR code" | ✅ CORRECT | Standard English |
| **fr-FR** | "QR code" | "QR code" | ✅ CORRECT | Anglicism adopted, NOT "code QR" |
| **de-DE** | N/A | **"QR-Code"** | ⚠️ NEEDS CREATION | German compound with HYPHEN |
| **ja-JP** | N/A | **"QRコード"** | ⚠️ NEEDS CREATION | Katakana standard |
| **it-IT** | N/A | "QR code" | ⚠️ NEEDS CREATION | English term retained |
| **pt-BR** | N/A | "QR code" | ⚠️ NEEDS CREATION | Brazilian prefers English |
| **pt-PT** | N/A | **"código QR"** | ⚠️ NEEDS CREATION | Portugal uses translation! |
| **es-ES/MX** | N/A | **"código QR"** | ⚠️ NEEDS CREATION | Spanish uses translation |

## Key Insights

### 1. French: "QR code" NOT "code QR"
**Source:** Le Monde, French tech media, Perplexity research

> "French people predominantly search for and use 'QR code' rather than 'code QR' in everyday contexts"

- Le Monde uses "paiement par QR code"
- Government may use "codes QR" formally but not in search behavior
- **Verdict:** Our choice of "QR code" for fr-FR is CORRECT

### 2. German: "QR-Code" (WITH HYPHEN)
**Source:** TU Wien, German market reports, Duden conventions

> "Germans predominantly search for and use 'QR-Code' (with a hyphen)"

- Follows German compound word conventions (like "E-Mail")
- NOT "QR Code" (spaced) or "QR-Kode"
- **Action:** Create de-DE EntityNatives with hyphenated form

### 3. Japanese: "QRコード" (Katakana)
**Source:** JAISA, Japanese government, Japanese companies

> "'QRコード' is the standard terminology used in Japanese contexts"

- Katakana is the standard for foreign-origin tech terms
- **Action:** Create ja-JP EntityNatives with katakana

### 4. Italian: "QR code" (English retained)
**Source:** Tech guides, AIFA, Italian media

> "Italians commonly use 'QR code' in everyday language"

- "codice QR" exists in dictionaries but rarely used
- Anglicisms common in Italian tech terminology
- **Action:** Create it-IT EntityNatives with English term

### 5. Portuguese: SPLIT MARKET
**Source:** Google Trends, Brazilian/Portuguese media

| Market | Term | Volume |
|--------|------|--------|
| Brazil (pt-BR) | "QR code" | 90-95% |
| Portugal (pt-PT) | "código QR" | 70-80% |

- Brazil: Tech-savvy, global influence, Pix payments use "QR code"
- Portugal: More formalized, official sites use "código QR"
- **Action:** Create SEPARATE EntityNatives for pt-BR and pt-PT

### 6. Spanish: "código QR"
**Source:** Previous research confirmed

- Both es-ES and es-MX use "código QR"
- **Action:** Create Spanish EntityNatives with translation

## Wikipedia Trap Analysis

The "Wikipedia trap" occurs when we use academic/formal translations instead of real search terms:

| Language | Wikipedia/Formal | Real Usage | Trap? |
|----------|------------------|------------|-------|
| French | "code QR" | "QR code" | ✅ AVOIDED |
| German | "QR-Kode" | "QR-Code" | N/A |
| Italian | "codice QR" | "QR code" | POTENTIAL |
| Portuguese (PT) | "código QR" | "código QR" | NO TRAP |
| Portuguese (BR) | "código QR" | "QR code" | POTENTIAL |

## Recommended Actions

### Immediate (Migration 035)
Create EntityNatives for high-traffic locales with CORRECT terminology:

```cypher
// German - WITH HYPHEN
MERGE (en:EntityNative {key: 'qr-code@de-DE'})
SET en.display_name = 'QR-Code',
    en.denomination_forms = '{"text":"QR-Code","title":"QR-Code","abbrev":"QR","url":"qr-code"}';

// Japanese - KATAKANA
MERGE (en:EntityNative {key: 'qr-code@ja-JP'})
SET en.display_name = 'QRコード',
    en.denomination_forms = '{"text":"QRコード","title":"QRコード","abbrev":"QR","url":"qr-code"}';

// Italian - ENGLISH TERM
MERGE (en:EntityNative {key: 'qr-code@it-IT'})
SET en.display_name = 'QR code',
    en.denomination_forms = '{"text":"QR code","title":"QR Code","abbrev":"QR","url":"qr-code"}';

// Brazilian Portuguese - ENGLISH TERM
MERGE (en:EntityNative {key: 'qr-code@pt-BR'})
SET en.display_name = 'QR code',
    en.denomination_forms = '{"text":"QR code","title":"QR Code","abbrev":"QR","url":"qr-code"}';

// Portugal Portuguese - TRANSLATION
MERGE (en:EntityNative {key: 'qr-code@pt-PT'})
SET en.display_name = 'Código QR',
    en.denomination_forms = '{"text":"código QR","title":"Código QR","abbrev":"QR","url":"codigo-qr"}';

// Spanish - TRANSLATION
MERGE (en:EntityNative {key: 'qr-code@es-ES'})
SET en.display_name = 'Código QR',
    en.denomination_forms = '{"text":"código QR","title":"Código QR","abbrev":"QR","url":"codigo-qr"}';
```

### Future Locales
Apply same research methodology before creating EntityNatives for:
- Dutch (nl-NL)
- Polish (pl-PL)
- Swedish (sv-SE)
- Korean (ko-KR)
- Chinese (zh-CN, zh-TW)
- Arabic (ar-SA)

## Conclusion

**Our fr-FR terminology is CORRECT.** We successfully avoided the "Wikipedia trap" by using "QR code" instead of "code QR".

For future locales, always verify with real search data before creating EntityNatives. The pattern shows:
- **Germanic/Latin languages often keep English tech terms** (FR, IT, BR)
- **Some markets prefer translations** (DE hyphenation, ES, PT-PT)
- **Asian markets use native scripts** (JA katakana)

---

*Generated by Claude using Perplexity web research, 2026-03-06*
