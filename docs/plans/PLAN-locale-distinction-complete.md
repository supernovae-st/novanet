# PLAN: Complete Locale Distinction Audit & Population

**Status**: READY FOR EXECUTION
**Critical Goal**: TRUE locale-specific differentiation across ALL data types
**Scope**: Adaptation, Style, Expression, CultureRef, Taboo, AudienceTrait, Pattern, Culture, Formatting, Slugification
**Priority**: CRITICAL

---

## Core Principle

```
╔═══════════════════════════════════════════════════════════════════════════╗
║  EVERY LOCALE DATA MUST BE DISTINCT                                       ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                           ║
║  fr-FR Expression  ≠  fr-BE Expression  ≠  fr-CH Expression              ║
║  es-ES CultureRef  ≠  es-MX CultureRef  ≠  es-AR CultureRef              ║
║  en-US Pattern     ≠  en-GB Pattern     ≠  en-AU Pattern                 ║
║                                                                           ║
║  If data is identical across locales → SEO cannibalization               ║
║  If data is generic → Lost opportunity for locale targeting              ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

---

## Data Types to Audit & Populate

### Currently Empty (MUST POPULATE)

| Type | Count | Status | Priority |
|------|-------|--------|----------|
| **Adaptation** | 0/204 | ❌ EMPTY | CRITICAL |
| **Style** | 0/204 | ❌ EMPTY | CRITICAL |

### Currently Populated (MUST AUDIT for distinctions)

| Type | Count | Status | Priority |
|------|-------|--------|----------|
| **Expression** | 17,038 | ⚠️ AUDIT | HIGH |
| **CultureRef** | 2,654 | ⚠️ AUDIT | HIGH |
| **Taboo** | 990 | ⚠️ AUDIT | HIGH |
| **AudienceTrait** | 707 | ⚠️ AUDIT | MEDIUM |
| **Pattern** | 234 | ⚠️ AUDIT | HIGH |
| **Culture** | 203 | ⚠️ AUDIT | MEDIUM |
| **Formatting** | 203 | ⚠️ AUDIT | LOW |
| **Slugification** | 203 | ⚠️ AUDIT | LOW |

---

## Phase 1: Comprehensive Audit of Existing Data

### Task 1.1: Expression Distinctiveness Audit

**Query**: Find identical expressions across same-language locales
```cypher
// Identical text across related locales
MATCH (e1:Expression), (e2:Expression)
WHERE substring(e1.locale, 0, 2) = substring(e2.locale, 0, 2)
  AND e1.locale <> e2.locale
  AND e1.text = e2.text
RETURN e1.locale AS locale1, e2.locale AS locale2, e1.text AS duplicate_text
ORDER BY substring(e1.locale, 0, 2), e1.text
LIMIT 500;
```

**Expected Issues**:
- Same greetings copied across fr-FR/fr-BE/fr-CH
- Same expressions without regional adaptations
- Missing locale-specific idioms

**Fix Strategy**: Research locale-specific alternatives via Perplexity

### Task 1.2: CultureRef Distinctiveness Audit

**Query**: Find generic CultureRefs that apply to multiple locales
```cypher
// CultureRefs with identical values across related locales
MATCH (c1:CultureRef), (c2:CultureRef)
WHERE substring(c1.locale, 0, 2) = substring(c2.locale, 0, 2)
  AND c1.locale <> c2.locale
  AND (c1.text = c2.text OR c1.value = c2.value)
RETURN c1.locale, c2.locale, c1.text, c1.value
LIMIT 200;
```

**Expected Issues**:
- "Family values" copied to all Spanish locales (but Mexico is different from Spain!)
- Missing region-specific cultural references

### Task 1.3: Taboo Distinctiveness Audit

**Query**: Find identical taboos (important - taboos vary SIGNIFICANTLY by region!)
```cypher
MATCH (t1:Taboo), (t2:Taboo)
WHERE substring(t1.locale, 0, 2) = substring(t2.locale, 0, 2)
  AND t1.locale <> t2.locale
  AND t1.topic = t2.topic
RETURN t1.locale, t2.locale, t1.topic
LIMIT 200;
```

**Critical**: Taboos are highly region-specific!
- Religious taboos vary (ar-SA vs ar-LB)
- Political taboos vary (zh-CN vs zh-TW)
- Social taboos vary (es-ES vs es-MX)

### Task 1.4: Pattern Distinctiveness Audit

**Query**: Find identical CTA patterns
```cypher
MATCH (p1:Pattern), (p2:Pattern)
WHERE substring(p1.locale, 0, 2) = substring(p2.locale, 0, 2)
  AND p1.locale <> p2.locale
  AND p1.template = p2.template
RETURN p1.locale, p2.locale, p1.template, p1.type
LIMIT 200;
```

**Expected Issues**:
- Same CTA templates copied without regional adaptation
- Missing formality adjustments (tú vs usted for Spanish variants)

### Task 1.5: AudienceTrait Distinctiveness Audit

```cypher
MATCH (a1:AudienceTrait), (a2:AudienceTrait)
WHERE substring(a1.locale, 0, 2) = substring(a2.locale, 0, 2)
  AND a1.locale <> a2.locale
  AND a1.trait = a2.trait
  AND a1.description = a2.description
RETURN a1.locale, a2.locale, a1.trait
LIMIT 200;
```

### Task 1.6: Culture Node Audit

Check if Culture nodes have locale-specific values or generic ones:
```cypher
MATCH (c1:Culture), (c2:Culture)
WHERE substring(c1.key, 0, 2) = substring(c2.key, 0, 2)
  AND c1.key <> c2.key
RETURN c1.key, c2.key,
       c1.high_context_score = c2.high_context_score AS same_context,
       c1.formality_default = c2.formality_default AS same_formality,
       c1.collectivism_score = c2.collectivism_score AS same_collectivism
LIMIT 100;
```

---

## Phase 2: Research Missing & Fix Duplicates

### Wave Structure (COMPREHENSIVE)

Each wave researches ALL data types for a locale group:

| Wave | Locales | Data Types | Agents |
|------|---------|------------|--------|
| **W01** | fr-FR, fr-BE, fr-CH, fr-CA, fr-LU | ALL | 5 |
| **W02a** | es-ES, es-MX, es-AR | ALL | 3 |
| **W02b** | es-CO, es-CL, es-PE, es-VE | ALL | 4 |
| **W03a** | en-US, en-GB, en-AU | ALL | 3 |
| **W03b** | en-CA, en-IN, en-ZA, en-NZ, en-IE | ALL | 5 |
| **W04** | pt-BR, pt-PT | ALL | 2 |
| **W05** | de-DE, de-AT, de-CH | ALL | 3 |
| **W06** | nl-NL, nl-BE | ALL | 2 |
| **W07a** | ar-SA, ar-AE, ar-EG | ALL | 3 |
| **W07b** | ar-MA, ar-DZ, ar-TN | ALL | 3 |
| **W07c** | ar-IQ, ar-JO, ar-LB, ar-SY | ALL | 4 |
| **W08** | zh-CN, zh-TW, zh-HK, zh-SG | ALL | 4 |
| **W09** | ms-MY, id-ID | ALL | 2 |
| **W10** | hi-IN, ur-PK | ALL | 2 |
| **W11** | bn-IN, bn-BD | ALL | 2 |
| **W12** | sr-RS, hr-HR, bs-BA | ALL | 3 |
| **W13** | ta-IN, ta-LK | ALL | 2 |
| **W14** | sw-KE, sw-TZ | ALL | 2 |
| **W15-W40** | Single-locale languages | ALL | 1 each |

### Research Prompt Template (COMPREHENSIVE)

```markdown
# Complete Locale Research: {LOCALE_CODE}

## Context
- Locale: {LOCALE_CODE} ({LANGUAGE} as spoken in {REGION})
- Related locales: {RELATED_LOCALES}
- Goal: Identify UNIQUE characteristics that differentiate from {RELATED_LOCALES}

---

## PART 1: ADAPTATION (Content Adaptation Rules)

### 1.1 Technical Terms Approach
- Does {REGION} prefer English tech terms (QR code, URL, API)?
- Or localized terms? What are the local equivalents?
- Influence from other languages? (German in fr-CH, English in fr-CA)

### 1.2 Illustration Density
- High visual (prefer images/diagrams)?
- Medium (balanced)?
- Low (text-heavy, detailed explanations)?

### 1.3 Content Classification
- What content types MUST remain factual/literal?
- What can be culturally adapted with local examples?

### 1.4 Common Errors & False Friends
- Words that look similar to {RELATED_LOCALES} but mean different things
- Common translation mistakes for this specific locale

---

## PART 2: STYLE (Communication Style)

For each score, provide JUSTIFICATION comparing to {RELATED_LOCALES}:

### 2.1 Formality (1-10)
- Score: X/10
- Why: "Compared to {RELATED_LOCALE} which is Y/10 because..."
- T-V distinction: When to use formal/informal

### 2.2 Directness (1-10)
- Score: X/10
- Direct, indirect, or context-dependent?
- How to soften requests in {REGION}

### 2.3 Hierarchy Importance
- High / Medium / Low
- How to address authority figures
- B2B vs B2C differences

### 2.4 Warmth Level
- Warm / Neutral / Reserved
- Relationship-building expectations
- Appropriate expressions of warmth

### 2.5 Individualism vs Collectivism
- Individualist / Collectivist / Balanced
- "You" vs "We" language preferences
- Personal testimonials vs community proof

---

## PART 3: EXPRESSIONS (Locale-Specific)

Provide 10-15 expressions UNIQUE to {REGION}:

### 3.1 Greetings
- Formal greeting unique to {REGION}
- Casual greeting unique to {REGION}
- How it differs from {RELATED_LOCALES}

### 3.2 CTAs (Call to Action)
- Persuasive phrases that work in {REGION}
- Urgency expressions
- Trust-building phrases

### 3.3 Idioms & Sayings
- Common idioms used in {REGION} marketing
- Proverbs that resonate locally

### 3.4 Regional Slang (if appropriate for marketing)
- Modern expressions
- Tech-related slang

---

## PART 4: CULTURE REFERENCES (Locale-Specific)

Provide 5-10 cultural references UNIQUE to {REGION}:

### 4.1 Core Values
- What values resonate most in {REGION}?
- How do they differ from {RELATED_LOCALES}?
- Marketing angles for each value

### 4.2 Local References
- Holidays unique to {REGION}
- Sports/entertainment references
- Historical/cultural touchpoints

### 4.3 Local Heroes/Influencers
- Who do people trust in {REGION}?
- Authority figures for endorsements

---

## PART 5: TABOOS (Locale-Specific)

Provide 5-10 taboos SPECIFIC to {REGION}:

### 5.1 Topics to Avoid
- Political taboos
- Religious sensitivities
- Social taboos
- Historical sensitivities

### 5.2 Visual Taboos
- Colors to avoid
- Gestures to avoid
- Imagery to avoid

### 5.3 Linguistic Taboos
- Words that sound offensive in {REGION}
- False friends that create issues

---

## PART 6: PATTERNS (CTA Templates)

Provide 5-10 CTA patterns that work specifically in {REGION}:

### 6.1 Urgency Patterns
- Template: "{native_urgency_phrase}"
- Translation: "{english_translation}"
- When to use: {context}

### 6.2 Trust Patterns
- Template: "{native_trust_phrase}"
- Context: {when_to_use}

### 6.3 Value Proposition Patterns
- Template: "{native_value_phrase}"
- Formality: formal/casual

---

## PART 7: AUDIENCE TRAITS

Describe the typical {REGION} digital audience:

### 7.1 Digital Behavior
- Mobile vs desktop preferences
- Social media platform preferences
- E-commerce habits

### 7.2 Decision-Making Style
- Impulse vs deliberate
- Influence of family/peers
- Research behavior

### 7.3 Trust Signals
- What builds trust in {REGION}?
- Certifications, testimonials, guarantees

---

## PART 8: SEO DIFFERENTIATION

List keywords/phrases that:
- Are UNIQUE to {REGION} (not used in {RELATED_LOCALES})
- Are commonly searched in {REGION}
- Would differentiate content for SEO
```

---

## Phase 3: Generate & Insert Data

### Task 3.1: Create Comprehensive Generator Script

**File**: `packages/db/scripts/generate-locale-complete.js`

Features:
- Generates Adaptation nodes
- Generates Style nodes
- Updates existing Expressions with locale-specific versions
- Updates CultureRefs with regional details
- Updates Taboos with locale-specific topics
- Updates Patterns with formality adjustments
- Validates distinctiveness before inserting

### Task 3.2: Create Update Scripts for Existing Data

**File**: `packages/db/scripts/update-expressions-distinct.js`
- Reads research JSON
- Updates expressions to be locale-specific
- Adds `regional_variant` property

**File**: `packages/db/scripts/update-culturerefs-distinct.js`
- Updates CultureRefs with regional marketing angles
- Adds `distinction_from_*` properties

**File**: `packages/db/scripts/update-patterns-distinct.js`
- Adjusts patterns for formality (tú/usted, tu/vous, etc.)
- Adds locale-specific CTAs

---

## Phase 4: Comprehensive Verification

### Task 4.1: CSR Audit
```
novanet_audit(target="all") → CSR = 100%
```

### Task 4.2: Distinctiveness Audit - ALL Types

```cypher
// MASTER DISTINCTIVENESS CHECK
// Find ANY identical data across related locales

// Expressions
MATCH (e1:Expression), (e2:Expression)
WHERE substring(e1.locale, 0, 2) = substring(e2.locale, 0, 2)
  AND e1.locale <> e2.locale
  AND e1.text = e2.text
RETURN 'Expression' AS type, e1.locale AS loc1, e2.locale AS loc2, count(*) AS duplicates
UNION
// CultureRefs
MATCH (c1:CultureRef), (c2:CultureRef)
WHERE substring(c1.locale, 0, 2) = substring(c2.locale, 0, 2)
  AND c1.locale <> c2.locale
  AND c1.text = c2.text
RETURN 'CultureRef' AS type, c1.locale AS loc1, c2.locale AS loc2, count(*) AS duplicates
UNION
// Patterns
MATCH (p1:Pattern), (p2:Pattern)
WHERE substring(p1.locale, 0, 2) = substring(p2.locale, 0, 2)
  AND p1.locale <> p2.locale
  AND p1.template = p2.template
RETURN 'Pattern' AS type, p1.locale AS loc1, p2.locale AS loc2, count(*) AS duplicates
UNION
// Styles
MATCH (s1:Style), (s2:Style)
WHERE substring(s1.key, 6, 2) = substring(s2.key, 6, 2)
  AND s1.key <> s2.key
  AND s1.formality_score = s2.formality_score
  AND s1.directness_score = s2.directness_score
RETURN 'Style' AS type, s1.key AS loc1, s2.key AS loc2, count(*) AS duplicates
ORDER BY duplicates DESC;

// Expected: 0 duplicates for all types
```

### Task 4.3: Coverage Check

```cypher
// Every locale MUST have:
MATCH (l:Locale)
OPTIONAL MATCH (l)-[:HAS_ADAPTATION]->(a:Adaptation)
OPTIONAL MATCH (l)-[:HAS_STYLE]->(s:Style)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(c:Culture)
OPTIONAL MATCH (es:ExpressionSet)-[:FOR_LOCALE]->(l)
WITH l,
     a IS NOT NULL AS has_adaptation,
     s IS NOT NULL AS has_style,
     c IS NOT NULL AS has_culture,
     es IS NOT NULL AS has_expressions
WHERE NOT (has_adaptation AND has_style AND has_culture AND has_expressions)
RETURN l.key, has_adaptation, has_style, has_culture, has_expressions;

// Expected: 0 rows (all locales complete)
```

---

## Execution Timeline

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  PHASE 1: AUDIT (Run audit queries, identify issues)                       │
│  Duration: 30 minutes                                                       │
│  Output: List of duplicates/missing per locale                              │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 2: RESEARCH (Perplexity agents in waves)                             │
│  Duration: ~2-3 hours (parallel agents)                                     │
│  Waves: 40 waves, 2-5 agents per wave                                       │
│  Output: research/*.json files                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 3: GENERATE (Create Cypher from research)                            │
│  Duration: 30 minutes                                                       │
│  Output: seed/27-*.cypher files                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  PHASE 4: SEED & VERIFY                                                     │
│  Duration: 30 minutes                                                       │
│  Output: CSR 100%, 0 duplicates                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Success Criteria

### Must Have (Blocking)
- [ ] 204 Adaptation nodes (1 per locale)
- [ ] 204 Style nodes (1 per locale)
- [ ] CSR = 100%
- [ ] 0 identical data pairs across same-language locales

### Should Have
- [ ] All Expressions have `regional_variant` property where applicable
- [ ] All CultureRefs have `distinction_notes` property
- [ ] All Patterns adjusted for locale-specific formality
- [ ] All Taboos are truly locale-specific (not just language-level)

### Nice to Have
- [ ] SEO differentiation keywords for each locale
- [ ] `confidence_score` on each researched property
- [ ] Source citations from Perplexity

---

## Files to Create

| Phase | File | Purpose |
|-------|------|---------|
| 1 | `scripts/audit-locale-distinctiveness.js` | Run all audit queries |
| 2 | `scripts/templates/comprehensive-research-prompt.md` | Research prompt template |
| 2 | `scripts/data/locale-relationships.json` | Related locale mapping |
| 2 | `research/complete/*.json` | Research results per wave |
| 3 | `scripts/generate-locale-complete.js` | Generate all Cypher |
| 3 | `scripts/update-expressions-distinct.js` | Update existing expressions |
| 3 | `scripts/update-culturerefs-distinct.js` | Update existing CultureRefs |
| 3 | `scripts/update-patterns-distinct.js` | Update existing patterns |
| 3 | `seed/27-adaptation-style.cypher` | New Adaptation/Style nodes |
| 3 | `seed/27.1-expression-updates.cypher` | Expression updates |
| 3 | `seed/27.2-cultureref-updates.cypher` | CultureRef updates |
| 3 | `seed/27.3-pattern-updates.cypher` | Pattern updates |
| 4 | `scripts/verify-locale-distinctiveness.js` | Final verification |

---

## Risk Mitigation

### Risk: Research returns generic data
**Mitigation**: Prompt explicitly asks for DIFFERENCES from related locales

### Risk: Too many Perplexity queries
**Mitigation**: Batch by locale group, parallelize waves

### Risk: Breaking existing data
**Mitigation**: All updates use MERGE, not DELETE/CREATE

### Risk: CSR drops during updates
**Mitigation**: Run `novanet_audit` after each major batch

---

## Rollback Plan

```cypher
// If issues occur, restore from backup
// (Assuming we export before starting)

// To rollback Adaptation/Style:
MATCH (a:Adaptation) DETACH DELETE a;
MATCH (s:Style) DETACH DELETE s;

// To rollback expression updates:
// Keep backup of original values in `_original_text` property
MATCH (e:Expression)
WHERE e._original_text IS NOT NULL
SET e.text = e._original_text
REMOVE e._original_text, e.regional_variant;
```

---

## Example: fr-FR vs fr-BE vs fr-CH Research Output

```json
{
  "locale": "fr-BE",
  "related_locales": ["fr-FR", "fr-CH", "fr-CA"],

  "adaptation": {
    "technical_terms_approach": "keep-english",
    "vs_fr-FR": "More English terms due to proximity to Netherlands/Germany",
    "vs_fr-CH": "Similar English preference but different number format",
    "illustration_density": "medium",
    "regional_vocabulary": ["septante", "nonante", "GSM", "kot", "ring"]
  },

  "style": {
    "formality_score": 6,
    "vs_fr-FR": "-1 (6 vs 7) - More direct, less hierarchical",
    "vs_fr-CH": "-2 (6 vs 8) - Much more casual",
    "directness_score": 7,
    "vs_fr-FR": "+2 (7 vs 5) - Germanic directness influence",
    "warmth_level": "warm",
    "vs_fr-FR": "Warmer, more relationship-oriented",
    "context_matrix": {
      "B2B": "neutral-formal",
      "B2C": "casual-warm",
      "technical": "keep-english"
    }
  },

  "expressions_unique": [
    {"text": "Allez, hop!", "context": "encouragement", "vs_fr-FR": "More casual than French 'Allez'"},
    {"text": "Une fois", "context": "emphasis", "vs_fr-FR": "Belgian-specific intensifier"}
  ],

  "culturerefs_unique": [
    {"value": "Manneken Pis humor", "marketing_angle": "Self-deprecating humor works well"},
    {"value": "Frituur culture", "marketing_angle": "Food-related references resonate"}
  ],

  "taboos_specific": [
    {"topic": "Language conflict", "severity": "critical", "reason": "Flemish/Walloon tensions"},
    {"topic": "Monarchy criticism", "severity": "medium", "reason": "Generally well-liked"}
  ],

  "patterns_adjusted": [
    {"template": "Découvrez {product} une fois!", "formality": "casual", "vs_fr-FR": "Added 'une fois'"},
    {"template": "Allez, inscrivez-vous!", "formality": "casual", "vs_fr-FR": "'Allez' more common in BE"}
  ],

  "seo_keywords_unique": ["code QR Belgique", "générateur QR belge", "septante"]
}
```
