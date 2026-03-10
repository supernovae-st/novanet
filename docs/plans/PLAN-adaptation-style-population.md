# PLAN: Populate Adaptation & Style with TRUE Locale Distinctions

**Status**: READY FOR EXECUTION
**Critical Goal**: Anti-SEO Cannibalization via TRUE locale-specific differentiation
**Priority**: CRITICAL

---

## Core Principle

```
╔═══════════════════════════════════════════════════════════════════════════╗
║  LOCALE ≠ LANGUAGE                                                        ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  fr-FR (France)  ≠  fr-BE (Belgium)  ≠  fr-CH (Switzerland)              ║
║  es-ES (Spain)   ≠  es-MX (Mexico)   ≠  es-AR (Argentina)                ║
║  en-US (USA)     ≠  en-GB (UK)       ≠  en-AU (Australia)                ║
║  pt-BR (Brazil)  ≠  pt-PT (Portugal)                                      ║
║                                                                           ║
║  Each locale = unique cultural identity = unique SEO value                ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

**Why this matters**: If fr-FR and fr-BE have identical Style/Adaptation, Google sees duplicate content → SEO cannibalization. We MUST differentiate.

---

## Strategy: Per-Locale Research with Distinction Focus

### Research Approach

For EACH locale, the Perplexity query will:
1. **Identify the specific locale** (not just language)
2. **Compare to related locales** (what makes it DIFFERENT)
3. **Extract unique characteristics** (formality differences, vocabulary, cultural references)
4. **Document distinctions explicitly**

### Example: French Locales Distinction

| Aspect | fr-FR (France) | fr-BE (Belgium) | fr-CH (Switzerland) | fr-CA (Canada) |
|--------|----------------|-----------------|---------------------|----------------|
| **Formality** | 7/10 (formal) | 6/10 (warmer) | 8/10 (very formal) | 5/10 (casual) |
| **Numbers** | 1 000,50 | 1.000,50 | 1'000.50 | 1 000,50 |
| **Vocabulary** | "soixante-dix" | "septante" | "septante" | "soixante-dix" |
| **Directness** | Indirect | Direct | Very direct | Direct |
| **Tech terms** | Mixed | Keep English | German influence | English influence |
| **Warmth** | Reserved | Warm | Reserved | Very warm |

---

## Phase 1: Audit Existing Data for Inconsistencies

Before adding new data, review existing Knowledge Atoms to ensure locale distinctions exist.

### Task 1.1: Audit Expression distinctiveness
```cypher
// Find Expressions that are too similar across locales of same language
MATCH (e1:Expression), (e2:Expression)
WHERE e1.locale STARTS WITH substring(e2.locale, 0, 2)
  AND e1.locale <> e2.locale
  AND e1.text = e2.text
RETURN e1.locale, e2.locale, e1.text
LIMIT 100;
```

### Task 1.2: Audit CultureRef distinctiveness
Check if CultureRefs are truly locale-specific or just copied across language variants.

### Task 1.3: Audit Pattern distinctiveness
Same patterns shouldn't exist verbatim for fr-FR and fr-BE.

---

## Phase 2: Per-Locale Research (204 Individual Queries)

### Wave Structure: By Language Family + Region

Instead of batching by language, we research **locale groups** where distinction is critical:

| Wave | Focus | Locales | Distinction Priority |
|------|-------|---------|---------------------|
| W01 | French variants | fr-FR, fr-BE, fr-CH, fr-CA, fr-LU | CRITICAL |
| W02 | Spanish variants | es-ES, es-MX, es-AR, es-CO, es-CL, es-PE, es-VE, es-EC, es-UY | CRITICAL |
| W03 | English variants | en-US, en-GB, en-AU, en-CA, en-IN, en-ZA, en-NZ, en-IE, en-SG | CRITICAL |
| W04 | Portuguese variants | pt-BR, pt-PT, pt-AO, pt-MZ | CRITICAL |
| W05 | German variants | de-DE, de-AT, de-CH, de-LU | HIGH |
| W06 | Dutch variants | nl-NL, nl-BE | HIGH |
| W07 | Arabic variants | ar-SA, ar-AE, ar-EG, ar-MA, ar-DZ, ar-TN, ar-IQ, ar-JO, ar-LB, ar-SY, ar-KW, ar-QA, ar-BH, ar-OM, ar-YE | CRITICAL |
| W08 | Chinese variants | zh-CN, zh-TW, zh-HK, zh-SG, zh-MO | CRITICAL |
| W09 | Malay/Indonesian | ms-MY, ms-SG, ms-BN, id-ID | HIGH |
| W10 | Hindi/Urdu variants | hi-IN, ur-PK, ur-IN | HIGH |
| W11 | Bengali variants | bn-IN, bn-BD | HIGH |
| W12 | Tamil variants | ta-IN, ta-LK, ta-SG, ta-MY | HIGH |
| W13 | Swahili variants | sw-KE, sw-TZ, sw-UG | MEDIUM |
| W14 | Korean variants | ko-KR, ko-KP | MEDIUM |
| W15 | Serbian/Croatian/Bosnian | sr-RS, hr-HR, bs-BA, sr-ME | CRITICAL |
| W16 | Norwegian variants | nb-NO, nn-NO, no-NO | MEDIUM |
| W17-W30 | Individual unique locales | ja-JP, th-TH, vi-VN, etc. | STANDARD |

### Research Prompt Template (Per-Locale)

```markdown
# Locale-Specific Research: {LOCALE_CODE}

## Context
This locale is {LANGUAGE} as spoken in {REGION}.
Related locales in the same language: {RELATED_LOCALES}

## CRITICAL: Focus on DISTINCTIONS

### Part 1: How is {LOCALE} DIFFERENT from {RELATED_LOCALES}?

1. **Vocabulary Differences**
   - Words/expressions unique to {REGION}
   - Words that have different meanings in {REGION} vs other {LANGUAGE}-speaking regions
   - False friends or confusing terms

2. **Formality Differences**
   - Is {REGION} more/less formal than other {LANGUAGE} regions?
   - Different honorific systems?
   - T-V distinction usage differences?

3. **Directness & Communication Style**
   - More direct or indirect than other {LANGUAGE} regions?
   - Business communication differences
   - Customer service tone expectations

4. **Cultural Context Differences**
   - Local references that wouldn't work in other {LANGUAGE} regions
   - Humor differences
   - Taboo topics specific to {REGION}

5. **Number/Date/Currency Formatting**
   - Specific to {REGION} (even if same language)

### Part 2: Adaptation Rules for {LOCALE}

1. **Technical Terms Approach**
   - Does {REGION} prefer English tech terms or localized?
   - Is there influence from other languages (German in fr-CH, English in fr-CA)?

2. **Illustration Density**
   - Visual vs text preferences specific to {REGION}

3. **Content Adaptation**
   - What examples/analogies work specifically in {REGION}?
   - What should NEVER be adapted (legal, regulatory)?

### Part 3: Style for {LOCALE}

Provide SPECIFIC scores (1-10) with JUSTIFICATION comparing to related locales:

- formality_score: X/10 (because... compared to {RELATED_LOCALE} which is Y/10)
- directness_score: X/10 (because...)
- warmth_level: warm/neutral/reserved (because...)
- hierarchy_importance: high/medium/low (because...)
- individualism_level: individualist/collectivist/balanced (because...)

### Part 4: SEO Differentiation Keywords

List keywords/phrases that are:
- Unique to {REGION} (not used in other {LANGUAGE} regions)
- Commonly searched in {REGION}
- Would differentiate content for SEO
```

---

## Phase 3: Generate Per-Locale Data

### Task 3.1: Enhanced Generator Script

**File**: `packages/db/scripts/generate-adaptation-style-v2.js`

Key changes from v1:
- Validates DISTINCTIVENESS between related locales
- Flags if two locales have identical scores
- Includes `distinction_notes` field
- Includes `related_locales` and `differentiation_factors`

### Data Structure Enhancement

```json
{
  "locale": "fr-BE",
  "related_locales": ["fr-FR", "fr-CH", "fr-CA"],
  "adaptation": {
    "technical_terms_approach": "keep-english",
    "distinction_from_fr-FR": "Belgian French uses more English tech terms, Germanic influence on directness",
    "distinction_from_fr-CH": "Less formal than Swiss French, different number formatting",
    "illustration_density": "medium",
    "facts_classification": {...},
    "regional_vocabulary": ["septante", "nonante", "GSM (not portable)"],
    "false_friends_with_fr-FR": ["déjeuner means lunch in BE, breakfast in FR"]
  },
  "style": {
    "formality_score": 6,
    "formality_vs_fr-FR": "-1 (more casual)",
    "formality_vs_fr-CH": "-2 (much more casual)",
    "directness_score": 7,
    "directness_vs_fr-FR": "+2 (more direct, Germanic influence)",
    "warmth_level": "warm",
    "warmth_vs_fr-FR": "warmer, more relationship-oriented",
    "context_matrix": {
      "B2B": "neutral",
      "B2C": "casual",
      "technical": "keep-english"
    },
    "seo_differentiation_keywords": [
      "GSM", "septante", "frituur", "terril"
    ]
  }
}
```

---

## Phase 4: Verification with Distinction Checks

### Task 4.1: CSR Audit
```
novanet_audit(target="all") → CSR = 100%
```

### Task 4.2: Distinctiveness Audit (NEW)

```cypher
// Check that no two related locales have identical Style scores
MATCH (s1:Style), (s2:Style)
WHERE substring(s1.key, 6, 2) = substring(s2.key, 6, 2)  // Same language prefix
  AND s1.key <> s2.key
  AND s1.formality_score = s2.formality_score
  AND s1.directness_score = s2.directness_score
  AND s1.warmth_level = s2.warmth_level
RETURN s1.key, s2.key, 'IDENTICAL SCORES - NEEDS DIFFERENTIATION' AS warning;
// Expected: 0 rows (all related locales should differ)
```

### Task 4.3: Review flagged similarities
If any locales have identical data, re-research with distinction focus.

---

## Execution Waves (Detailed)

### Wave Priority Order

**CRITICAL WAVES (Multi-variant languages - SEO cannibalization risk)**

| Wave | Locales | Agent Count | Notes |
|------|---------|-------------|-------|
| W01 | fr-FR, fr-BE, fr-CH, fr-CA, fr-LU | 5 parallel | Each needs distinct research |
| W02a | es-ES, es-MX, es-AR | 3 parallel | Major markets |
| W02b | es-CO, es-CL, es-PE, es-VE, es-EC | 5 parallel | Secondary markets |
| W03a | en-US, en-GB, en-AU | 3 parallel | Major markets |
| W03b | en-CA, en-IN, en-ZA, en-NZ, en-IE, en-SG | 6 parallel | Secondary markets |
| W04 | pt-BR, pt-PT | 2 parallel | Very different |
| W05 | de-DE, de-AT, de-CH | 3 parallel | |
| W06 | nl-NL, nl-BE | 2 parallel | |
| W07a | ar-SA, ar-AE, ar-EG | 3 parallel | Major Gulf + Egypt |
| W07b | ar-MA, ar-DZ, ar-TN | 3 parallel | Maghreb |
| W07c | ar-IQ, ar-JO, ar-LB, ar-SY | 4 parallel | Levant |
| W08 | zh-CN, zh-TW, zh-HK, zh-SG | 4 parallel | CRITICAL differences |
| W09 | ms-MY, id-ID | 2 parallel | Related but distinct |
| W10 | hi-IN, ur-PK | 2 parallel | |
| W11 | bn-IN, bn-BD | 2 parallel | |
| W12 | sr-RS, hr-HR, bs-BA | 3 parallel | Post-Yugoslav distinctions |

**STANDARD WAVES (Single-locale languages)**

| Wave | Locales | Notes |
|------|---------|-------|
| W20 | ja-JP | Unique |
| W21 | ko-KR | Unique (ignore ko-KP for now) |
| W22 | th-TH | Unique |
| W23 | vi-VN | Unique |
| W24 | tr-TR | Unique |
| W25-W40 | Remaining single-locale languages | 1 per wave |

---

## Review of Existing Data

### Task R1: Expression Similarity Check
Flag expressions that are identical across locale variants.

### Task R2: CultureRef Overlap Check
Ensure CultureRefs are truly locale-specific.

### Task R3: Pattern Template Check
Patterns should have locale-specific nuances (formal "vous" vs casual "tu" for fr-FR vs fr-CA).

### Task R4: Force Distinctions
For any flagged duplicates:
- Research locale-specific alternatives
- Update or create distinct versions
- Document why they're different

---

## Success Criteria

- [ ] 204 Adaptation nodes with DISTINCT data per locale
- [ ] 204 Style nodes with DISTINCT scores per locale
- [ ] NO two related locales have identical formality/directness/warmth
- [ ] Each locale has `distinction_notes` explaining how it differs
- [ ] CSR = 100%
- [ ] Distinctiveness audit passes (0 identical pairs)

---

## Files to Create

| File | Purpose |
|------|---------|
| `scripts/templates/locale-distinction-prompt.md` | Research prompt template |
| `scripts/data/locale-relationships.json` | Maps related locales |
| `scripts/generate-adaptation-style-v2.js` | Generator with distinction checks |
| `scripts/audit-locale-distinctiveness.js` | Validates no duplicates |
| `research/adaptation-style/*.json` | Per-wave research results |
| `seed/27-adaptation-style.cypher` | Final seed file |

---

## Anti-Patterns to Avoid

```
❌ WRONG: Copy fr-FR data to fr-BE with minor changes
✅ RIGHT: Research fr-BE specifically, compare to fr-FR, document differences

❌ WRONG: Same formality_score for es-ES and es-MX
✅ RIGHT: es-ES=7 (formal), es-MX=5 (casual, tuteo common)

❌ WRONG: Generic "Spanish communication style" for all es-* locales
✅ RIGHT: "Mexican communication style emphasizes warmth and relationship-building,
          unlike Spanish directness"
```

---

## Estimated Output

- **Adaptation nodes**: 204 (one per locale)
- **Style nodes**: 204 (one per locale)
- **Total new nodes**: 408
- **Arcs created**: 816 (HAS_ADAPTATION, ADAPTATION_OF, HAS_STYLE, STYLE_OF)
- **Research queries**: ~204 (some batched for unique locales)
- **Distinction guarantees**: No two same-language locales have identical scores
