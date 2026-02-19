# v11.0 Legacy Cleanup Plan

**Date**: 2026-02-09
**Status**: ✅ COMPLETE
**Version**: v11.0.0

## Overview

Clean up all legacy v10.x terminology and data to achieve full v11.0 compliance.

### Key Migrations (ADR-014)

| Old | New | Context |
|-----|-----|---------|
| `EntityL10n` | `EntityContent` | Node label |
| `PageL10n` | `PageGenerated` | Node label |
| `BlockL10n` | `BlockGenerated` | Node label |
| `ConceptL10n` | DELETE | Deprecated node |
| `HAS_L10N` | `HAS_CONTENT` | Arc type |
| `HAS_OUTPUT` | `HAS_GENERATED` | Arc type |

**Note**: `ProjectL10n` renamed to `ProjectContent` per v11.0 (ADR-014 updated).

---

## Phase A: TUI Rust Code Fixes

**Priority**: HIGH
**Files**: 2

### A.1 Fix guide/mod.rs Layer Counts

**File**: `src/tui/guide/mod.rs`
**Lines**: 5, 388, 1333, 1355, 1368

```
v11.0 Architecture:
- GLOBAL: 2 layers (config, locale-knowledge)
- TENANT: 7 layers (config, foundation, structure, semantic, instruction, seo, output)
```

Update all references from "3 global layers" to "2 global layers" and "6 tenant layers" to "7 tenant layers".

### A.2 Fix Atlas Demo Data

**File**: `src/tui/ui/atlas.rs`

| Line | Issue | Fix |
|------|-------|-----|
| 139, 235 | Version says "v10.6" | Change to "v11.0" |
| 153 | "per-org" terminology | Change to "per-tenant" |
| 257 | SEO in GLOBAL realm | Move to TENANT realm |

---

## Phase B: Neo4j Data Cleanup

**Priority**: CRITICAL
**Requires**: Running Neo4j instance

### B.1 Audit Current State

```cypher
-- Count legacy nodes
MATCH (n:EntityL10n) RETURN 'EntityL10n' AS label, count(n) AS count
UNION ALL
MATCH (n:ConceptL10n) RETURN 'ConceptL10n' AS label, count(n) AS count
UNION ALL
MATCH (n:PageL10n) RETURN 'PageL10n' AS label, count(n) AS count
UNION ALL
MATCH (n:BlockL10n) RETURN 'BlockL10n' AS label, count(n) AS count;

-- Count legacy arcs
MATCH ()-[r:HAS_L10N]->() RETURN 'HAS_L10N' AS type, count(r) AS count
UNION ALL
MATCH ()-[r:HAS_OUTPUT]->() RETURN 'HAS_OUTPUT' AS type, count(r) AS count;

-- Count SEOKeyword with low volume
MATCH (k:SEOKeyword) WHERE k.volume IS NULL OR k.volume < 50
RETURN count(k) AS low_volume_seo_keywords;
```

### B.2 Delete Low-Volume SEOKeyword

```cypher
-- Preview
MATCH (k:SEOKeyword) WHERE k.volume IS NULL OR k.volume < 50
RETURN k.key, k.volume LIMIT 20;

-- Execute
MATCH (k:SEOKeyword) WHERE k.volume IS NULL OR k.volume < 50
DETACH DELETE k;
```

### B.3 Delete ConceptL10n (Deprecated)

```cypher
-- Preview
MATCH (n:ConceptL10n) RETURN n.key, labels(n) LIMIT 20;

-- Execute
MATCH (n:ConceptL10n) DETACH DELETE n;
```

### B.4 Migrate EntityL10n → EntityContent

```cypher
-- Add new label
MATCH (n:EntityL10n) SET n:EntityContent;

-- Remove old label
MATCH (n:EntityL10n) REMOVE n:EntityL10n;
```

### B.5 Migrate PageL10n → PageGenerated

```cypher
MATCH (n:PageL10n) SET n:PageGenerated;
MATCH (n:PageL10n) REMOVE n:PageL10n;
```

### B.6 Migrate BlockL10n → BlockGenerated

```cypher
MATCH (n:BlockL10n) SET n:BlockGenerated;
MATCH (n:BlockL10n) REMOVE n:BlockL10n;
```

### B.7 Migrate HAS_L10N → HAS_CONTENT

```cypher
-- Create new relationships
MATCH (a)-[r:HAS_L10N]->(b)
CREATE (a)-[:HAS_CONTENT]->(b);

-- Delete old relationships
MATCH ()-[r:HAS_L10N]->() DELETE r;
```

### B.8 Migrate HAS_OUTPUT → HAS_GENERATED

```cypher
MATCH (a)-[r:HAS_OUTPUT]->(b)
CREATE (a)-[:HAS_GENERATED]->(b);

MATCH ()-[r:HAS_OUTPUT]->() DELETE r;
```

---

## Phase C: packages/db Cypher Files

**Priority**: HIGH
**Files**: 4

### C.1 block-generation.cypher

**Lines**: 17, 36
- `HAS_L10N` → `HAS_CONTENT`
- `EntityL10n` → `EntityContent`

### C.2 page-generation-context.cypher

**Line**: 16
- `HAS_L10N` → `HAS_CONTENT`

### C.3 block-semantic-network.cypher

**Line**: 33
- `EntityL10n` → `EntityContent`

### C.4 project-layer.cypher

**Line**: 53
- Review and update if needed

---

## Phase D: packages/core TypeScript Files

**Priority**: MEDIUM
**Files**: 2

### D.1 relations.schema.ts

**Lines**: Multiple (159, 176, 337, 460, 647, 696, 726)

Review each `ProjectL10n` reference - these may be VALID (ADR-014).

**Lines**: 126, 129, 144, 274, 535-536, 567, 584, 612

Check `GEOSeedL10n` and `GEOSeedMetrics` - may need migration or removal.

### D.2 CypherGenerator.ts

**Line**: 94
- Review `PageMetrics` reference

---

## Phase E: Verification (10 Sniper Agents)

**Priority**: HIGH
**Trigger**: After all previous phases complete

Launch 10 parallel agents to verify:

1. TUI app.rs - no legacy
2. TUI mod.rs - no legacy
3. TUI data.rs - no legacy
4. TUI atlas/ - no legacy
5. TUI guide/ - no legacy
6. TUI ui/ - no legacy
7. packages/db/ - no legacy
8. packages/core/ - no legacy
9. Neo4j labels - all migrated
10. Neo4j arcs - all migrated

---

## Execution Checklist

- [x] Phase A.1: Fix guide/mod.rs layer counts ✅ (2026-02-09)
- [x] Phase A.2: Fix atlas demo data ✅ (2026-02-09)
- [x] Phase B.1: Audit Neo4j current state ✅ (2026-02-09)
- [x] Phase B.2: Delete low-volume SEOKeyword ✅ (6,157 nodes + 12,340 rels deleted)
- [x] Phase B.3: Delete ConceptL10n ✅ (already 0)
- [x] Phase B.4: Migrate EntityL10n → EntityContent ✅ (281 → 399 total)
- [x] Phase B.5: Migrate PageL10n → PageGenerated ✅ (already 0)
- [x] Phase B.6: Migrate BlockL10n → BlockGenerated ✅ (already 0)
- [x] Phase B.7: Migrate HAS_L10N → HAS_CONTENT ✅ (281 → 403 total)
- [x] Phase B.8: Migrate HAS_OUTPUT → HAS_GENERATED ✅ (already 0)
- [x] Phase C: Fix packages/db Cypher files ✅ (2026-02-09)
- [x] Phase D: Fix packages/core TypeScript files ✅ (2026-02-09)
- [x] Phase E: Run 10 verification agents ✅ (2026-02-09)
- [x] Final: All Rust tests pass (949/949) ✅

---

## Notes

- `ProjectL10n` renamed to `ProjectContent` per v11.0 (ADR-014 updated)
- SEOKeyword minimum volume threshold: 50
- Always preview before destructive operations
- Run tests after each phase

## Final Results (2026-02-09)

| Metric | Before | After |
|--------|--------|-------|
| EntityL10n | 281 | 0 |
| EntityContent | 281 | 399 |
| HAS_L10N | 281 | 0 |
| HAS_CONTENT | ~285 | 403 |
| SEOKeyword (low vol) | 6,157 | 0 |
| Legacy labels | 4 types | 0 |

**v11.0 compliance achieved.**
