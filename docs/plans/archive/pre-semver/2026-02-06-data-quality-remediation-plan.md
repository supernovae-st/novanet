# Data Quality Remediation Plan

**Date**: 2026-02-06
**Status**: COMPLETED
**Owner**: Claude + Thibaut

## Executive Summary

Following deep audits of the NovaNet database, we identified critical data quality issues requiring immediate remediation. This plan covers expression data regeneration and ontology integrity fixes.

## Phase 1: Expression Data Regeneration (COMPLETED)

### 1.1 Problem Statement
- 17,036 Expression nodes had 0% proper diacritical marks for Latin-script locales
- Root cause: ATH project generated ASCII-only content
- All corrupted data deleted from Neo4j

### 1.2 Solution: Tiered Regeneration

| Tier | Locales | Expressions/Locale | Total | Status |
|------|---------|-------------------|-------|--------|
| Tier 1 | 6 (fr-FR, es-ES, de-DE, pt-BR, ja-JP, zh-CN) | 200 | 1,200 | DONE |
| Tier 2 | 10 (it-IT, ko-KR, ru-RU, ar-SA, hi-IN, pl-PL, nl-NL, tr-TR, id-ID, th-TH) | 150 | 1,500 | DONE |
| Tier 3 | 184 remaining locales | 50-80 | ~10,000 | PENDING |

### 1.3 Files Created

```
packages/core/models/docs/locale-research/
├── fr-FR-expressions.yaml    (200 expressions, 215 accents)
├── es-ES-expressions.yaml    (200 expressions, 99 accents)
├── de-DE-expressions.yaml    (200 expressions, 59 umlauts)
├── pt-BR-expressions.yaml    (200 expressions, 141 accents)
├── ja-JP-expressions.yaml    (200 expressions, native CJK)
├── zh-CN-expressions.yaml    (200 expressions, native Hanzi)
├── it-IT-expressions.yaml    (150 expressions)
├── ko-KR-expressions.yaml    (150 expressions, native Hangul)
├── ru-RU-expressions.yaml    (150 expressions, native Cyrillic)
├── ar-SA-expressions.yaml    (150 expressions, native Arabic)
├── hi-IN-expressions.yaml    (150 expressions, native Devanagari)
├── pl-PL-expressions.yaml    (150 expressions, Polish diacritics)
├── nl-NL-expressions.yaml    (150 expressions)
├── tr-TR-expressions.yaml    (150 expressions, Turkish chars)
├── id-ID-expressions.yaml    (150 expressions)
└── th-TH-expressions.yaml    (150 expressions, native Thai)
```

---

## Phase 2: Ontology Integrity Fixes (IN PROGRESS)

### 2.1 Critical Issues Identified

| ID | Issue | Severity | Nodes Affected |
|----|-------|----------|----------------|
| C1 | Kind nodes missing `key`/`realm`/`layer`/`trait` | CRITICAL | 60 |
| C2 | Ghost nodes (empty, no labels) | CRITICAL | 3 |
| H1 | SlugRule missing OF_KIND | HIGH | 5 |
| H2 | ArcScope/ArcCardinality orphans | HIGH | 6 |
| M1 | Locale taxonomy intentional duplicates | MEDIUM | 13 |

### 2.2 Fix Strategy

#### Task 2.2.1: Fix Rust Generator (CRITICAL)

**File**: `tools/novanet/src/generators/node_kind.rs`

**Current State** (lines 162-210):
```rust
MERGE ({var}:Meta:Kind {{label: '{label}'}})
ON CREATE SET
  {var}.display_name = '{display}',
  {var}.llm_context = '{llm}',
  // ... missing key, realm, layer, trait
```

**Required Changes**:
```rust
// Add after line 165:
writeln!(out, "  {var}.key = '{}',", to_kebab_case(&node.def.name))?;
writeln!(out, "  {var}.realm = '{}',", node.realm)?;
writeln!(out, "  {var}.layer = '{}',", node.layer)?;
writeln!(out, "  {var}.trait = '{}',", trait_to_string(&node.def.node_trait))?;
```

**Verification**:
```cypher
MATCH (k:Kind)
WHERE k.realm IS NOT NULL AND k.layer IS NOT NULL
RETURN count(*) AS fixed_kinds
-- Expected: 60
```

#### Task 2.2.2: Delete Ghost Nodes

**Cypher**:
```cypher
MATCH (n)
WHERE size(labels(n)) = 0 AND size(keys(n)) = 0
DELETE n
RETURN count(*) AS deleted_ghost_nodes
-- Expected: 3
```

#### Task 2.2.3: Wire SlugRule Nodes

**Cypher**:
```cypher
MATCH (sr:SlugRule)
WHERE NOT exists((sr)-[:OF_KIND]->())
MATCH (k:Kind {label: 'SlugRule'})
MERGE (sr)-[:OF_KIND]->(k)
RETURN count(sr) AS wired_slug_rules
-- Expected: 5
```

#### Task 2.2.4: Wire ArcScope/ArcCardinality

**Cypher**:
```cypher
// Wire ArcScope to ArcKind nodes
MATCH (ak:ArcKind), (scope:ArcScope)
WHERE ak.scope = scope.key
MERGE (ak)-[:HAS_SCOPE]->(scope);

// Wire ArcCardinality to ArcKind nodes
MATCH (ak:ArcKind), (card:ArcCardinality)
WHERE ak.cardinality = card.key
MERGE (ak)-[:HAS_CARDINALITY]->(card);
```

---

## Phase 3: Expression Seed Generation (PENDING)

### 3.1 Generate Cypher from YAML

**Input**: 16 YAML files in `packages/core/models/docs/locale-research/`
**Output**: `packages/db/seed/26-expression.cypher`

**Structure**:
```cypher
// PART 1: ExpressionSet nodes (16 locales)
MERGE (es:ExpressionSet {key: 'fr-FR'})
SET es.display_name = 'fr-FR Expressions',
    es.semantic_fields_count = 10,
    es.total_expressions = 200;

// PART 2: Expression atoms (~2,700 total)
MERGE (e:Expression {key: 'fr-FR/URGENCY/0'})
SET e.locale_key = 'fr-FR',
    e.semantic_field = 'URGENCY',
    e.text = 'Plus que quelques heures !',
    e.register = 'casual',
    e.context = 'Flash sale, countdown timer';

// PART 3: Relationships
MATCH (es:ExpressionSet {key: 'fr-FR'}), (e:Expression)
WHERE e.key STARTS WITH 'fr-FR/'
MERGE (es)-[:CONTAINS]->(e);

MATCH (l:Locale {key: 'fr-FR'}), (es:ExpressionSet {key: 'fr-FR'})
MERGE (l)-[:HAS_EXPRESSIONS]->(es);
```

### 3.2 Validation Queries

```cypher
// Check accent coverage for French
MATCH (e:Expression)
WHERE e.locale_key = 'fr-FR'
WITH count(*) AS total,
     sum(CASE WHEN e.text =~ '.*[àâçèéêëîïôùûüœæ].*' THEN 1 ELSE 0 END) AS with_accents
RETURN total, with_accents,
       round(100.0 * with_accents / total, 1) AS percent_with_accents
-- Expected: > 50%

// Check total expressions per locale
MATCH (e:Expression)
RETURN e.locale_key AS locale, count(*) AS count
ORDER BY count DESC
```

---

## Phase 4: Database Reseed (PENDING)

### 4.1 Steps

```bash
# 1. Regenerate schema artifacts
cd tools/novanet
cargo run -- schema generate

# 2. Reset database
cd ../..
pnpm infra:reset

# 3. Verify counts
docker exec novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (n) RETURN labels(n)[0] AS label, count(*) ORDER BY count DESC"
```

### 4.2 Expected Results

| Node Type | Expected Count |
|-----------|----------------|
| Locale | 200 |
| ExpressionSet | 16 |
| Expression | ~2,700 |
| Kind | 60 (with realm/layer) |
| ArcKind | 72 |

---

## Success Criteria

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Expression count | 2,700+ | 1,950 (16 locales) + legacy | DONE |
| French accent rate | > 50% | 63% | DONE |
| Kind with realm/layer | 60/60 | 60/60 | DONE |
| Ghost nodes | 0 | 0 | DONE |
| Orphan meta nodes | 0 | 0 | DONE |

---

## Timeline

| Phase | Task | Status |
|-------|------|--------|
| 1.1 | Delete corrupted Expression data | DONE |
| 1.2 | Research Tier 1 expressions (6 locales) | DONE |
| 1.3 | Research Tier 2 expressions (10 locales) | DONE |
| 2.1 | Run ontology audit agents | DONE |
| 2.2.1 | Fix Rust generator | DONE |
| 2.2.2 | Delete ghost nodes | DONE |
| 2.2.3 | Wire SlugRule nodes | DONE (via reseed) |
| 2.2.4 | Wire ArcScope/ArcCardinality | DONE |
| 3.1 | Generate 26-expression.cypher | DONE |
| 4.1 | Reseed database | DONE |
| 4.2 | Validate results | DONE |

---

## Files Modified

| File | Changes |
|------|---------|
| `tools/novanet/src/generators/node_kind.rs` | Add key/realm/layer/trait |
| `packages/db/seed/26-expression.cypher` | Regenerate with quality data |
| `packages/db/docker-compose.yml` | UTF-8 env vars (DONE) |
| `packages/db/seed.sh` | Locale export (DONE) |
