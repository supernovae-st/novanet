# Plan: Studio v0.13.0 Terminology Update

**Date**: 2026-02-16
**Status**: Ready for Implementation
**Scope**: 15 files, ~60 changes

---

## Executive Summary

Audit of 10 parallel explorer agents found **45+ issues** across Studio codebase requiring updates to align with v0.13.0 terminology (ADR-023, ADR-024, ADR-029, ADR-030).

**Categories**:
- 🔴 **CRITICAL** (5): Code that will break or display wrong data
- 🟡 **HIGH** (8): User-visible incorrect labels
- 🟢 **LOW** (7): Comments and documentation

---

## Phase 1: Critical Code Fixes (Priority 1)

### 1.1 Duplicate Arc in Cypher Query

**File**: `src/lib/cypher/viewQueries.ts:247`

```typescript
// BEFORE (line 247):
OPTIONAL MATCH (n)-[r:HAS_NATIVE|HAS_NATIVE]->(content)
                          ^^^^^^  ^^^^^^ DUPLICATE!

// AFTER:
OPTIONAL MATCH (n)-[r:HAS_NATIVE]->(content)
```

**Impact**: Redundant query, potential performance issue

---

### 1.2 Dead Code Removal

**File**: `src/lib/layout.ts:85-86`

```typescript
// REMOVE these lines (nodes no longer exist in v0.13.0):
PageOutput: { width: 200, height: 110 },
BlockOutput: { width: 190, height: 105 },
```

**Impact**: Dead code referencing removed node types

---

### 1.3 Org Realm Color Mismatch

**File**: `src/design/colors/generated.ts:31`

```typescript
// CURRENT (WRONG):
'org': { color: '#6c71c4', ... }  // purple

// EXPECTED (per CLAUDE.md):
'org': { color: '#0ea5e9', ... }  // sky blue
```

**Fix**: Regenerate from taxonomy.yaml:
```bash
cd tools/novanet && cargo run -- schema generate
```

**Verify**: `grep -A5 "'org':" apps/studio/src/design/colors/generated.ts`

---

## Phase 2: Display Labels (Priority 2)

### 2.1 Node Type Labels (ADR-029 *Native Pattern)

**File**: `src/config/nodeTypes.ts`

| Line | Current | Correct |
|------|---------|---------|
| 430 | `'Project Content'` | `'Project Native'` |
| 483 | `'Entity Content'` | `'Entity Native'` |
| 632 | `'Page Generated'` | `'Page Native'` |
| 641 | `'Block Generated'` | `'Block Native'` |

---

### 2.2 Arc Label

**File**: `src/config/relationshipTypes.ts:376`

```typescript
// BEFORE:
label: 'Belongs To Project Content'

// AFTER:
label: 'Belongs To Project Native'
```

---

### 2.3 Mode Labels (ADR-022 Unified Tree)

**File**: `src/config/actions.ts`

| Line | Current | Correct |
|------|---------|---------|
| 103 | `"Meta Mode"` | `"Schema Mode"` |
| 104 | `"META"` | `"SCHEMA"` |
| 113 | `"Data Mode"` | `"Graph Mode"` |
| 114 | `"DATA"` | `"GRAPH"` |

---

### 2.4 Trait Cycle Description (ADR-024)

**File**: `src/config/shortcuts.ts:125`

```typescript
// BEFORE:
"Cycle through Invariant → Localized → Knowledge → Generated → Aggregated → None"

// AFTER:
"Cycle through Defined → Authored → Imported → Generated → Retrieved → None"
```

---

### 2.5 Mode Cycle Description

**File**: `src/config/shortcuts.ts:113`

```typescript
// BEFORE:
"Cycle through Meta → Data"

// AFTER:
"Cycle through Schema → Graph"
```

---

## Phase 3: Test Fixtures (Priority 3)

### 3.1 E2E Test Updates

**File**: `e2e/schema-mode.spec.ts`

| Line(s) | Change |
|---------|--------|
| 11, 141, 146 | `42 nodes` → `61 nodes` |
| 127, 137, 254, 296 | Fix historical comments (misleading) |
| 136, 256, 348 | `#section-label-global` → `#section-label-shared` |
| 293 | `📦 Project · 🌍 Global` → `📦 Org · 🌍 Shared` |
| 310-311 | `/GLOBAL/` regex → `/SHARED/` |

---

### 3.2 Schema Generator Comments

**File**: `src/lib/schemaGenerator.ts`

| Line | Current | Correct |
|------|---------|---------|
| 12, 15, 54, 55, 63 | `42 node types` | `61 node types` |
| 15, 55 | `8 layers, 2 realms` | `10 layers, 2 realms` |

---

### 3.3 Test Layer Counts

**File**: `__tests__/config/color-sync.test.ts`

| Line | Current | Correct |
|------|---------|---------|
| 43 | `"all 9 layers"` | `"all 10 layers"` |
| 80 | `"all 9 layers"` | `"all 10 layers"` |

**File**: `__tests__/config/icon-sync.test.ts:16`

| Line | Current | Correct |
|------|---------|---------|
| 16 | `"all 9 layer icons"` | `"all 10 layer icons"` |

---

### 3.4 Test Fixture Label

**File**: `src/lib/__tests__/schemaLayoutELK.test.ts`

| Line | Current | Correct |
|------|---------|---------|
| 83 | `"11 layers (3 shared + 8 org)"` | `"10 layers (4 shared + 6 org)"` |
| 160 | `label: 'Project Content'` | `label: 'Project Native'` |

---

## Phase 4: Comments (Priority 4)

### 4.1 Kind → Class Comments

**File**: `src/components/graph/Graph2D.tsx`

| Line | Change |
|------|--------|
| 394 | `Kind` → `Class` |
| 405 | `Kind` → `Class` |
| 416 | `Kind nodes` → `Class nodes` |

**File**: `src/hooks/useFilteredGraph.ts`

| Line | Change |
|------|--------|
| 122 | `Kind` → `Class` |
| 124 | `Kind` → `Class` |

---

## Phase 5: Visual Encoding Gaps (Future)

These require more substantial implementation work:

### 5.1 Arc Stroke Styles Not Applied

**File**: `src/components/graph/edges/FloatingEdge.tsx:443-450`

**Issue**: `ARC_STROKES` defined in palette.ts but never applied to edge rendering.

**Required**:
```tsx
// Add strokeDasharray based on arc family:
const arcStroke = getArcStroke(relationType);
<path
  d={edgePath}
  strokeDasharray={arcStroke.pattern}  // ADD THIS
  strokeWidth={arcStroke.width}        // USE THIS
  ...
/>
```

---

### 5.2 Icon Mismatches (8 icons)

**File**: `src/design/icons/nodeIcons.generated.ts`

| Node | Generated | Should Be (YAML) |
|------|-----------|------------------|
| Adaptation | `droplet` | `sliders` |
| Brand | `circle` | `palette` |
| BrandDesign | `circle` | `brush` |
| BrandPrinciples | `circle` | `heart-handshake` |
| ChannelSurface | `radio` | `monitor` |
| SEOKeywordMetrics | `camera` | `bar-chart-2` |
| Style | `drama` | `palette` |

**Fix**: Update YAML source or regenerate icons

---

### 5.3 Localization Stroke Style Conflict

**File**: `src/components/graph/edges/system/themes.ts:96`

```typescript
// CURRENT (WRONG):
localization: { lineStyle: 'double' }

// YAML SOURCE (CORRECT):
localization: { pattern: '6 3', width: 2 }  // dashed
```

---

## Implementation Order

```
┌─────────────────────────────────────────────────────────────────┐
│  RECOMMENDED EXECUTION ORDER                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. 🔴 viewQueries.ts:247        (fix duplicate HAS_NATIVE)     │
│  2. 🔴 layout.ts:85-86           (remove dead code)             │
│  3. 🔴 Regenerate colors         (cargo run -- schema generate) │
│  4. 🟡 nodeTypes.ts              (4 label changes)              │
│  5. 🟡 relationshipTypes.ts:376  (1 label change)               │
│  6. 🟡 actions.ts                (4 mode label changes)         │
│  7. 🟡 shortcuts.ts              (2 description changes)        │
│  8. 🟢 E2E tests                 (12+ changes)                  │
│  9. 🟢 schemaGenerator.ts        (5 comment changes)            │
│  10. 🟢 Test files               (layer count assertions)       │
│  11. 🟢 Graph2D.tsx              (3 comment changes)            │
│  12. 🟢 useFilteredGraph.ts      (2 comment changes)            │
│                                                                 │
│  FUTURE (separate PR):                                          │
│  - FloatingEdge.tsx arc strokes                                 │
│  - Icon regeneration                                            │
│  - themes.ts stroke fix                                         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Verification Commands

```bash
# After changes, run these to verify:

# 1. Type check
pnpm type-check --filter=@novanet/studio

# 2. Tests
pnpm test --filter=@novanet/studio

# 3. Search for remaining old terms
grep -r "EntityContent\|PageGenerated\|BlockGenerated" apps/studio/src/
grep -r "'invariant'\|'localized'\|'knowledge'\|'aggregated'" apps/studio/src/ --include="*.ts" --include="*.tsx"
grep -r "HAS_CONTENT\|HAS_GENERATED\|CONTENT_OF" apps/studio/src/
grep -r "Meta Mode\|Data Mode" apps/studio/src/

# 4. Verify org color
grep -A2 "'org'" apps/studio/src/design/colors/generated.ts
# Should show: color: '#0ea5e9'
```

---

## Files Summary

| File | Changes | Priority |
|------|---------|----------|
| `src/lib/cypher/viewQueries.ts` | 1 | 🔴 Critical |
| `src/lib/layout.ts` | 2 lines removed | 🔴 Critical |
| `src/design/colors/generated.ts` | Regenerate | 🔴 Critical |
| `src/config/nodeTypes.ts` | 4 labels | 🟡 High |
| `src/config/relationshipTypes.ts` | 1 label | 🟡 High |
| `src/config/actions.ts` | 4 labels | 🟡 High |
| `src/config/shortcuts.ts` | 2 descriptions | 🟡 High |
| `e2e/schema-mode.spec.ts` | 12+ changes | 🟢 Low |
| `src/lib/schemaGenerator.ts` | 5 comments | 🟢 Low |
| `__tests__/config/color-sync.test.ts` | 2 descriptions | 🟢 Low |
| `__tests__/config/icon-sync.test.ts` | 1 description | 🟢 Low |
| `src/lib/__tests__/schemaLayoutELK.test.ts` | 2 changes | 🟢 Low |
| `src/components/graph/Graph2D.tsx` | 3 comments | 🟢 Low |
| `src/hooks/useFilteredGraph.ts` | 2 comments | 🟢 Low |

**Total**: 15 files, ~60 individual changes

---

## ADR References

- **ADR-023**: Class/Instance Terminology (Kind→Class, Meta→Schema)
- **ADR-024**: Trait = Data Origin (invariant→defined, localized→authored, knowledge→imported, aggregated→retrieved)
- **ADR-029**: *Native Pattern (EntityContent→EntityNative, HAS_CONTENT→HAS_NATIVE)
- **ADR-022**: Unified Tree Architecture (5→2 modes: Graph + Nexus)
- **ADR-012**: 2-Realm Architecture (61 nodes, 10 layers)
