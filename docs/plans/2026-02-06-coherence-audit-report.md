# DB/YAML Coherence Audit Report

**Date**: 2026-02-06
**Version**: v10.6.0 (2-Realm Architecture)
**Author**: Claude Code (audit session)

---

## Executive Summary

Comprehensive audit of Neo4j database coherence with YAML schema definitions.

| Check | Status | Details |
|-------|--------|---------|
| Kind nodes vs YAML | PASS | 60/60 nodes match |
| ArcKind properties | FIXED | Added family/scope to 54 nodes |
| Legacy Neo4j labels | FIXED | Removed GeoContinent from 6 nodes |
| Seed idempotency | FIXED | 27-geographic-taxonomy.cypher uses MERGE |
| YAML path/content | PASS | All 60 node-kinds consistent |
| Arc scope logic | PASS | cross_realm/intra_realm computed correctly |

---

## Issues Found and Resolved

### 1. ArcKind Missing Properties (FIXED)

**Problem**: 54 ArcKind meta-nodes were missing `family` and `scope` properties.

**Root Cause**: Generator `arc_kind.rs` wasn't outputting these properties to Cypher.

**Fix**: Modified `tools/novanet/src/generators/arc_kind.rs`:
- Added `compute_scope()` helper function to determine arc scope
- Added `family` and `scope` to `ON CREATE SET` and `ON MATCH SET` blocks
- Scope logic: `intra_realm` when all sources/targets in same realm, `cross_realm` otherwise

**Verification**:
```cypher
MATCH (ak:ArcKind) WHERE ak.family IS NOT NULL RETURN count(ak)
-- Result: 54 (all ArcKind nodes now have family/scope)
```

### 2. Legacy GeoContinent Label (FIXED)

**Problem**: 6 Continent nodes had duplicate label `GeoContinent` (legacy from v10.5).

**Fix**: Removed legacy label:
```cypher
MATCH (c:GeoContinent) REMOVE c:GeoContinent
```

**Verification**: `MATCH (n:GeoContinent) RETURN count(n)` → 0

### 3. Non-Idempotent Seed File (FIXED)

**Problem**: `27-geographic-taxonomy.cypher` used `CREATE` which fails on re-seed.

**Error**: `Node(18386) already exists with label 'Continent' and property 'key' = 'africa'`

**Fix**: Converted all `CREATE` to `MERGE` for idempotent seeding.

---

## Issues Found (Not Yet Resolved)

### 4. Arc Definition Migration Incomplete

**Current State**:
| Source | Arc Count |
|--------|-----------|
| `relations.yaml` (deprecated) | 54 arcs |
| `arc-kinds/_index.yaml` | 74 arcs listed |
| Actual YAML files | 89 files |

**Root Cause**: Migration from `relations.yaml` to individual `arc-kinds/**/*.yaml` files is incomplete.

**Generator reads**: `relations.yaml` (deprecated, but still primary source)
**Expected source**: Individual YAML files per arc

**Missing from _index.yaml** (15 geographic arcs):
- `cluster-of.yaml`, `branch-of.yaml`, `part-of-realm.yaml`
- `in-continent.yaml`, `in-region.yaml`, `in-subregion.yaml`
- `in-economic-region.yaml`, `in-cultural-subrealm.yaml`
- `speaks-branch.yaml`, `culturally-similar.yaml`
- `has-population.yaml`, `has-primary-population.yaml`
- `has-income-level.yaml`, `has-lending-type.yaml`
- `contains.yaml`

**Recommendation**: Complete arc migration by:
1. Update generator to read from `arc-kinds/**/*.yaml` instead of `relations.yaml`
2. Update `_index.yaml` to include all 89 arc files
3. Remove `relations.yaml` when migration complete

---

## Verification Queries

### Kind Coherence
```cypher
// All 60 Kinds have required properties
MATCH (k:Kind)
WHERE k.realm IS NULL OR k.layer IS NULL OR k.trait IS NULL
RETURN count(k) -- Expected: 0
```

### ArcKind Coherence
```cypher
// All ArcKinds have family and scope
MATCH (ak:ArcKind)
WHERE ak.family IS NULL
RETURN count(ak) -- Expected: 0
```

### Cross-Realm Arcs (should be few)
```cypher
MATCH (ak:ArcKind {scope: 'cross_realm'})
RETURN ak.key, ak.family
ORDER BY ak.family, ak.key
// Expected: DEFAULT_LOCALE, FOR_LOCALE, SUPPORTS_LOCALE, INCLUDES_STYLE
```

### No Legacy Labels
```cypher
MATCH (n:GeoContinent) RETURN count(n) -- Expected: 0
MATCH (n:Scope) RETURN count(n) -- Expected: 0
MATCH (n:Subcategory) RETURN count(n) -- Expected: 0
```

---

## Files Modified

| File | Change |
|------|--------|
| `tools/novanet/src/generators/arc_kind.rs` | Added family/scope property generation |
| `packages/db/seed/02-arc-kinds.cypher` | Regenerated with new properties |
| `packages/db/seed/27-geographic-taxonomy.cypher` | CREATE → MERGE |

---

## Statistics

| Metric | Count |
|--------|-------|
| Kind nodes (DB) | 60 |
| Kind nodes (YAML) | 60 |
| ArcKind nodes (DB) | 54 |
| Arc YAML files | 89 |
| Global realm Kinds | 37 |
| Tenant realm Kinds | 23 |
| cross_realm arcs | 4 |
| intra_realm arcs | 50 |

---

## Next Steps

1. **Complete arc migration**: Update generator to read from individual YAML files
2. **Update _index.yaml**: Add 15 missing geographic arc references
3. **Deprecate relations.yaml**: After generator migration is complete
4. **Run full schema validation**: `cargo run -- schema validate --strict`
