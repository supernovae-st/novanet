# Expression Data Regeneration Plan

**Date**: 2026-02-06
**Status**: In Progress
**Owner**: Claude + Thibaut

## Executive Summary

All 17,036 Expression nodes were corrupted (0% proper diacritical marks for Latin-script locales). Data has been deleted and needs complete regeneration with quality-first approach.

## Tiered Regeneration Strategy

Based on locale importance and market size:

### Tier 1: Primary Markets (6 locales)
**Target: 200-300 expressions each**

| Locale | Language | Market | Priority |
|--------|----------|--------|----------|
| en-US | English (US) | Global default | P0 |
| fr-FR | French (France) | EU, Africa | P0 |
| de-DE | German | DACH | P0 |
| es-ES | Spanish (Spain) | EU, LATAM base | P0 |
| ja-JP | Japanese | Asia | P0 |
| zh-CN | Chinese (Simplified) | China | P0 |

### Tier 2: Major Markets (20 locales)
**Target: 100-150 expressions each**

| Region | Locales |
|--------|---------|
| Europe | pt-PT, it-IT, nl-NL, pl-PL, sv-SE, da-DK, no-NO, fi-FI, cs-CZ, el-GR |
| Americas | pt-BR, es-MX, es-AR, fr-CA, en-CA |
| Asia | ko-KR, zh-TW, th-TH, vi-VN, id-ID |

### Tier 3: Growth Markets (174 locales)
**Target: 50-80 expressions each**

All remaining locales from LOCALES-200 list.

## Semantic Fields (10 domains)

Each locale needs expressions across all 10 semantic fields:

| Field | Description | Use Cases |
|-------|-------------|-----------|
| URGENCY | Time-limited, scarcity | CTAs, promotions |
| VALUE | Savings, benefits | Pricing, offers |
| TRUST | Authority, social proof | Testimonials, guarantees |
| SUCCESS | Achievement, winning | Gamification, results |
| QUALITY | Premium, excellence | Product descriptions |
| SPEED | Fast, instant | Convenience features |
| SIMPLICITY | Easy, effortless | Onboarding, UX |
| INNOVATION | New, cutting-edge | Tech products |
| EXCLUSIVITY | Limited, members-only | Premium tiers |
| COMMUNITY | Belonging, shared | Social features |

## Quality Requirements

### Encoding
- [x] UTF-8 NFC normalized
- [x] All diacritical marks preserved
- [x] Script matches locale (Cyrillic, CJK, Arabic, etc.)

### Linguistic
- [x] Native expressions (NOT translations)
- [x] Register-appropriate (formal/casual/neutral)
- [x] Regional variant correct (fr-FR vs fr-CA)
- [x] No calques (literal idiom translations)

### Structure
- [x] 3-10 words per expression
- [x] Includes context and example sentence
- [x] Tagged with register and tone

## Research Sources

### Primary (Perplexity Deep Research)
1. Native marketing corpora
2. Regional advertising databases
3. Social media expression mining
4. Competitor localization analysis

### Secondary (Context7)
1. CLDR locale specifications
2. ICU exemplar character sets
3. Regional formatting patterns

### Tertiary (Manual Validation)
1. Native speaker review (Tier 1)
2. Spot check sampling (Tier 2-3)

## Implementation Phases

### Phase 1: Research (Current)
- [ ] Research Tier 1 expressions with Perplexity
- [ ] Gather CLDR validation rules from Context7
- [ ] Create expression templates per semantic field

### Phase 2: Generation
- [ ] Generate Tier 1 expressions (1,200-1,800 total)
- [ ] Validate UTF-8 encoding and diacritics
- [ ] Generate Tier 2 expressions (2,000-3,000 total)
- [ ] Generate Tier 3 expressions (8,700-13,920 total)

### Phase 3: Seed
- [ ] Create 26-expression.cypher with new data
- [ ] Verify MERGE statements for idempotency
- [ ] Run pnpm infra:seed
- [ ] Validate counts and quality in Neo4j

### Phase 4: Verification
- [ ] Run accent validation queries
- [ ] Spot check expressions per locale
- [ ] Update documentation

## Expression Record Structure

```yaml
expression:
  key: "fr-FR/URGENCY/0"
  locale_key: "fr-FR"
  semantic_field: "URGENCY"
  intention: "Create sense of scarcity"
  text: "Plus que quelques heures !"
  register: "casual"
  context: "Flash sale, limited time"
  example: "Plus que quelques heures pour profiter de -50% !"
  llm_context: "USE: fr-FR expression for urgency..."
```

## Validation Queries

```cypher
// Check accent coverage for French
MATCH (e:Expression)
WHERE e.locale_key = 'fr-FR'
WITH count(*) AS total,
     sum(CASE WHEN e.text =~ '.*[àâçèéêëîïôùûüœæ].*' THEN 1 ELSE 0 END) AS with_accents
RETURN total, with_accents,
       round(100.0 * with_accents / total, 1) AS percent_with_accents

// Should be > 50% for French
```

## Success Criteria

| Metric | Target |
|--------|--------|
| Total expressions | 12,000-18,000 |
| Tier 1 coverage | 100% (6 locales × 200+ each) |
| Tier 2 coverage | 100% (20 locales × 100+ each) |
| Tier 3 coverage | 100% (174 locales × 50+ each) |
| French accent rate | > 50% |
| Spanish accent rate | > 40% |
| German umlaut rate | > 20% |
| Duplicate rate | < 1% |

## Files to Update

1. `packages/db/seed/26-expression.cypher` - New expression data
2. `tools/novanet/src/generators/expression.rs` - Add validation
3. `packages/db/seed.sh` - Already fixed (UTF-8 export)
4. `packages/db/docker-compose.yml` - Already fixed (UTF-8 env)
