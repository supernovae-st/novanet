# NovaNet Data Quality Fix Plan

**Date**: 2026-03-10
**Version**: v0.17.2 → v0.17.3
**Author**: Claude + Thibaut

---

## Executive Summary

Based on 19 sniper agent audits, we identified **20 critical/high/medium issues** affecting NovaNet data quality. This plan addresses all gaps through:

1. **Seed File Transfer** - Copy 7 files from private-data (12.7 MB of reference data)
2. **Database Re-seed** - Fresh seed with complete locale/expression data
3. **3 Migrations** - Fix arc gaps, LanguageBranch, EntityNative issues
4. **Ralph Wiggum Verification** - Full audit loop to confirm 95%+ CSR

---

## Phase 1: Seed File Transfer

### Files to Copy

| File | Size | Content | Priority |
|------|------|---------|----------|
| `20-locales.cypher` | 172 KB | 201 Locale definitions | P0 |
| `22-slugification.cypher` | 664 KB | LocaleSlugification rules | P0 |
| `23-formatting.cypher` | 806 KB | LocaleFormatting rules | P0 |
| `24-culture.cypher` | 822 KB | Culture nodes (201) | P0 |
| `26-expression.cypher` | 10 MB | 17,036 Expression atoms | P0 |
| `27-geographic-taxonomy.cypher` | 128 KB | Geographic hierarchy | P1 |
| `29-countries.cypher` | 156 KB | Country nodes | P1 |

### Commands

```bash
# Source directory
SRC="/Users/thibaut/dev/supernovae/private-data/seed"

# Destination directory
DST="/Users/thibaut/dev/supernovae/novanet/packages/db/seed"

# Copy critical files
cp "$SRC/20-locales.cypher" "$DST/"
cp "$SRC/22-slugification.cypher" "$DST/"
cp "$SRC/23-formatting.cypher" "$DST/"
cp "$SRC/24-culture.cypher" "$DST/"
cp "$SRC/26-expression.cypher" "$DST/"
cp "$SRC/27-geographic-taxonomy.cypher" "$DST/"
cp "$SRC/29-countries.cypher" "$DST/"
```

### Verification

```bash
ls -la "$DST"/*.cypher | wc -l  # Should be 11+ files
```

---

## Phase 2: Database Re-seed

### Pre-requisites

- Neo4j running (`pnpm infra:up`)
- All seed files in place

### Commands

```bash
cd /Users/thibaut/dev/supernovae/novanet
pnpm infra:reset   # Drop + reseed
```

### Expected Results

| Metric | Before | After |
|--------|--------|-------|
| Locales | 3 | 201 |
| Expressions | 0 | 17,036 |
| Culture nodes | 3 | 201 |
| ExpressionSets | 203 (empty) | 201 (populated) |

---

## Phase 3: Migration 101 - Arc Gaps

### Issues Fixed

1. **17 EntityNative missing NATIVE_OF** - Add inverse arcs
2. **Project missing HAS_DEFAULT_LOCALE** - Set default to en-US
3. **0 ProjectNative nodes** - Create for top 5 locales

### Cypher

```cypher
// 101-fix-arc-gaps.cypher

// 1. Add NATIVE_OF inverse arcs for all EntityNative nodes
MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
WHERE NOT (en)-[:NATIVE_OF]->(:Entity)
MERGE (en)-[:NATIVE_OF]->(e);

// 2. Set default locale for Project
MATCH (p:Project {key: 'supernovae-studio'})
MATCH (l:Locale {key: 'en-US'})
MERGE (p)-[:HAS_DEFAULT_LOCALE]->(l);

// 3. Create ProjectNative nodes for top locales
UNWIND ['en-US', 'fr-FR', 'de-DE', 'es-ES', 'ja-JP'] AS locale_key
MATCH (p:Project {key: 'supernovae-studio'})
MATCH (l:Locale {key: locale_key})
MERGE (pn:ProjectNative {key: 'project-native:supernovae-studio@' + locale_key})
ON CREATE SET
  pn.display_name = 'SuperNovae Studio (' + locale_key + ')',
  pn.description = 'Localized project metadata for ' + locale_key,
  pn.locale_key = locale_key,
  pn.created_at = datetime(),
  pn.updated_at = datetime()
MERGE (p)-[:HAS_NATIVE]->(pn)
MERGE (pn)-[:NATIVE_OF]->(p)
MERGE (pn)-[:FOR_LOCALE]->(l);
```

---

## Phase 4: Migration 102 - LanguageBranch Fixes

### Issues Fixed

1. **Missing tai-kadai LanguageBranch** - Create for lo-LA
2. **2 orphan locales** - Add OF_BRANCH arcs
3. **Missing iso_code** - Add to all 14 branches

### Cypher

```cypher
// 102-fix-language-branches.cypher

// 1. Create tai-kadai LanguageBranch
MERGE (lb:LanguageBranch {key: 'tai_kadai'})
ON CREATE SET
  lb.display_name = 'Tai-Kadai Languages',
  lb.description = 'Language family including Thai, Lao, and related languages of Southeast Asia',
  lb.iso_code = 'tai',
  lb.llm_context = 'USE: for Lao, Thai, Shan, Zhuang locales. TRIGGERS: Southeast Asian tonal languages. NOT: for Vietnamese (Austroasiatic).',
  lb.created_at = datetime(),
  lb.updated_at = datetime();

// 2. Connect orphan locales to their branches
MATCH (l:Locale {key: 'lo-LA'})
MATCH (lb:LanguageBranch {key: 'tai_kadai'})
MERGE (l)-[:OF_BRANCH]->(lb);

MATCH (l:Locale {key: 'or-IN'})
MATCH (lb:LanguageBranch {key: 'indo_aryan'})
MERGE (l)-[:OF_BRANCH]->(lb);

// 3. Add iso_code to all LanguageBranch nodes
MATCH (lb:LanguageBranch {key: 'romance'}) SET lb.iso_code = 'roa';
MATCH (lb:LanguageBranch {key: 'germanic'}) SET lb.iso_code = 'gem';
MATCH (lb:LanguageBranch {key: 'slavic'}) SET lb.iso_code = 'sla';
MATCH (lb:LanguageBranch {key: 'sino_tibetan'}) SET lb.iso_code = 'sit';
MATCH (lb:LanguageBranch {key: 'semitic'}) SET lb.iso_code = 'sem';
MATCH (lb:LanguageBranch {key: 'japonic'}) SET lb.iso_code = 'jpx';
MATCH (lb:LanguageBranch {key: 'koreanic'}) SET lb.iso_code = 'kor';
MATCH (lb:LanguageBranch {key: 'turkic'}) SET lb.iso_code = 'trk';
MATCH (lb:LanguageBranch {key: 'indo_aryan'}) SET lb.iso_code = 'inc';
MATCH (lb:LanguageBranch {key: 'dravidian'}) SET lb.iso_code = 'dra';
MATCH (lb:LanguageBranch {key: 'austronesian'}) SET lb.iso_code = 'map';
MATCH (lb:LanguageBranch {key: 'uralic'}) SET lb.iso_code = 'urj';
MATCH (lb:LanguageBranch {key: 'other'}) SET lb.iso_code = 'mis';
```

---

## Phase 5: Migration 103 - EntityNative Gaps

### Issues Fixed

1. **3 entities missing en-US EntityNative** - Create for landing-page, qr-code-art, smart-link
2. **3 EntityNatives missing denomination_forms** - Add to de-DE, es-MX, ja-JP
3. **entity:barcode orphan** - Add semantic link

### Cypher

```cypher
// 103-fix-entity-native-gaps.cypher

// 1. Create missing en-US EntityNatives
UNWIND [
  {entity: 'entity:landing-page', display: 'Landing Page', desc: 'Mobile-optimized destination page'},
  {entity: 'entity:qr-code-art', display: 'QR Code Art', desc: 'AI-generated artistic QR code'},
  {entity: 'entity:smart-link', display: 'Smart Link', desc: 'Intelligent shortened URL with routing'}
] AS item
MATCH (e:Entity {key: item.entity})
MATCH (l:Locale {key: 'en-US'})
MERGE (en:EntityNative {key: item.entity + '@en-US'})
ON CREATE SET
  en.display_name = item.display,
  en.description = item.desc,
  en.locale_key = 'en-US',
  en.denomination_forms = '[{"type":"text","value":"' + toLower(item.display) + '","priority":1},{"type":"title","value":"' + item.display + '","priority":1}]',
  en.llm_context = 'USE: for en-US content generation. TRIGGERS: ' + toLower(item.display) + '. RELATES: Entity (parent), Locale (en-US).',
  en.created_at = datetime(),
  en.updated_at = datetime()
MERGE (e)-[:HAS_NATIVE]->(en)
MERGE (en)-[:NATIVE_OF]->(e)
MERGE (en)-[:FOR_LOCALE]->(l);

// 2. Add denomination_forms to existing EntityNatives
MATCH (en:EntityNative {key: 'entity:qr-code@de-DE'})
WHERE en.denomination_forms IS NULL
SET en.denomination_forms = '[{"type":"text","value":"qr-code","priority":1},{"type":"title","value":"QR-Code","priority":1},{"type":"abbrev","value":"QR","priority":1}]',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'entity:qr-code@es-MX'})
WHERE en.denomination_forms IS NULL
SET en.denomination_forms = '[{"type":"text","value":"codigo qr","priority":1},{"type":"title","value":"Codigo QR","priority":1},{"type":"abbrev","value":"QR","priority":1}]',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'entity:qr-code@ja-JP'})
WHERE en.denomination_forms IS NULL
SET en.denomination_forms = '[{"type":"text","value":"qrコード","priority":1},{"type":"title","value":"QRコード","priority":1},{"type":"abbrev","value":"QR","priority":1}]',
    en.updated_at = datetime();

// 3. Connect entity:barcode with semantic link
MATCH (barcode:Entity {key: 'entity:barcode'})
MATCH (qrcode:Entity {key: 'entity:qr-code'})
MERGE (barcode)-[:SEMANTIC_LINK {temperature: 0.7, relationship: 'related_technology'}]->(qrcode);
```

---

## Phase 6: Run All Migrations

### Commands

```bash
cd /Users/thibaut/dev/supernovae/novanet

# Run new migrations only
for f in packages/db/migrations/10{1,2,3}*.cypher; do
  echo "Running $f..."
  cat "$f" | cypher-shell -u neo4j -p novanetpassword
done
```

---

## Phase 7: Ralph Wiggum Verification Loop

### Verification Queries

```cypher
// 1. Locale count
MATCH (l:Locale) RETURN count(l) AS locale_count;
// Expected: 201

// 2. Expression count
MATCH (e:Expression) RETURN count(e) AS expression_count;
// Expected: 17,036

// 3. EntityNative NATIVE_OF coverage
MATCH (en:EntityNative)
OPTIONAL MATCH (en)-[:NATIVE_OF]->(e:Entity)
RETURN count(en) AS total, count(e) AS with_native_of;
// Expected: 100% coverage

// 4. LanguageBranch orphan check
MATCH (l:Locale)
OPTIONAL MATCH (l)-[:OF_BRANCH]->(lb:LanguageBranch)
WITH count(l) AS total, count(lb) AS linked
RETURN total, linked, round(100.0 * linked / total, 1) AS pct;
// Expected: 100%

// 5. ProjectNative coverage
MATCH (p:Project)-[:HAS_NATIVE]->(pn:ProjectNative)
RETURN count(pn) AS project_native_count;
// Expected: 5

// 6. HAS_DEFAULT_LOCALE check
MATCH (p:Project)-[:HAS_DEFAULT_LOCALE]->(l:Locale)
RETURN p.key, l.key;
// Expected: supernovae-studio -> en-US
```

---

## Phase 8: Final CSR Check

### Command

```bash
# Using novanet_audit MCP tool
novanet_audit(target: "all")
```

### Expected Results

| Metric | Target | Expected |
|--------|--------|----------|
| Overall CSR | >= 95% | 97%+ |
| Coverage CSR | >= 95% | 98% |
| Orphan CSR | >= 95% | 99% |
| Integrity CSR | >= 95% | 100% |

---

## Rollback Plan

If issues occur:

```bash
# Reset to clean state
cd /Users/thibaut/dev/supernovae/novanet
pnpm infra:reset

# Remove new migrations
rm packages/db/migrations/101-fix-arc-gaps.cypher
rm packages/db/migrations/102-fix-language-branches.cypher
rm packages/db/migrations/103-fix-entity-native-gaps.cypher

# Re-seed with original data only
pnpm infra:seed
```

---

## Success Criteria

- [ ] 201 Locales in database
- [ ] 17,036 Expressions in database
- [ ] 100% EntityNative with NATIVE_OF arc
- [ ] 100% Locale with OF_BRANCH arc
- [ ] Project has HAS_DEFAULT_LOCALE to en-US
- [ ] 5 ProjectNative nodes created
- [ ] entity:barcode connected via SEMANTIC_LINK
- [ ] CSR >= 95% on novanet_audit

---

**Plan Status**: READY FOR EXECUTION
