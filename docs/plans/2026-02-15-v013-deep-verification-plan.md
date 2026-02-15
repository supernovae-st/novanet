# v0.13.0 Deep Verification Plan

**Date**: 2026-02-15
**Version**: v0.13.0 *Native Pattern (ADR-029) + Slug Ownership (ADR-030)
**Method**: 20 parallel verification agents executed
**Status**: AUDIT COMPLETE - REMEDIATION PLAN READY

---

## Executive Summary

**Overall Compliance: 87%** — Schema files (YAML) are fully v0.13.0 compliant. Issues found in:
- Rust TUI display code (hardcoded old names in output strings)
- Rust generator test fixtures (deprecated terminology)
- Studio hardcoded arc/color mappings (source-of-truth violations)
- View queries (outdated arc names in Cypher)
- Documentation (ROADMAP.md outdated)

---

## Phase 1: Critical Issues (P0 - Blocks Functionality)

### Issue 1.1: Rust TUI Display Uses Deprecated Names
**Files**: 8 files, ~80 occurrences
**Impact**: User-visible output shows wrong node/arc names

| File | Lines | Issue |
|------|-------|-------|
| `blueprint/views/default.rs` | 180, 192, 229, 234, 253, 255 | EntityContent, PageGenerated, HAS_CONTENT, HAS_GENERATED in ASCII diagrams |
| `blueprint/views/flow.rs` | 45, 68, 120, 123, 181 | Flow diagrams show deprecated names |
| `tui/nexus/traits.rs` | 74, 144, 146, 151, 157, 174, 219, 221 | Trait descriptions use old names |
| `tui/nexus/i18n.rs` | 114, 126, 430, 450, 518, 538, 624, 767 | English + French i18n strings |
| `tui/data.rs` | 365, 368, 388, 390, 467, 490, 510, 513, 518, 566, 568, 684, 695 | Data display + ASCII art |

**Fix**: Search-replace with v0.13.0 names:
- `EntityContent` → `EntityNative`
- `ProjectContent` → `ProjectNative`
- `PageGenerated` → `PageNative`
- `BlockGenerated` → `BlockNative`
- `HAS_CONTENT` → `HAS_NATIVE`
- `HAS_GENERATED` → `HAS_NATIVE`
- `CONTENT_OF` → `NATIVE_OF`
- `GENERATED_FOR` → `NATIVE_OF`

### Issue 1.2: Rust Generator Test Fixtures
**Files**: 2 files, ~20 occurrences
**Impact**: Tests use wrong names (may cause test failures if assertions change)

| File | Lines | Issue |
|------|-------|-------|
| `generators/node_class.rs` | 87, 88, 154, 423, 467, 497-499, 676, 694, 698, 706 | Test data uses deprecated names |
| `generators/mermaid.rs` | 530-531, 546-550, 558, 563, 597, 600-601, 609 | Mermaid output tests |

**Fix**: Update test fixtures + regenerate snapshots with `cargo insta review`

### Issue 1.3: View Queries Use Non-Existent Arcs
**Files**: 2 files, 5 undefined arcs
**Impact**: Queries will fail at runtime

| File | Arc | Correct Name |
|------|-----|--------------|
| `viewQueries.ts:117` | `HAS_VOICE` | Remove (Brand has no voice arc) |
| `viewQueries.ts:153` | `IN_CLUSTER` | `SEO_CLUSTER_OF` or `HAS_KEYWORD` |
| `viewQueries.ts:173` | `HAS_ANSWER` | `HAS_GEO_ANSWERS` |
| `viewQueries.ts:174` | `IN_QUERY_SET` | `CONTAINS_GEO_QUERY` |
| `views.yaml:68` | `IN_FAMILY` | `BELONGS_TO_FAMILY` |

**Fix**: Update Cypher queries with correct arc names

---

## Phase 2: High Priority Issues (P1 - Source of Truth Violations)

### Issue 2.1: Studio Hardcoded Arc Family Mappings
**File**: `apps/studio/src/design/colors/palette.ts` (lines 186-248)
**Impact**: 53 arc→family mappings hardcoded instead of generated from YAML

**Fix**: Create generator `arc-families.generated.ts` from arc-class YAML definitions

### Issue 2.2: Studio Hardcoded Relationship Type Configs
**File**: `apps/studio/src/config/relationshipTypes.ts` (578 lines)
**Impact**: 53 relationships with hardcoded colors + categories

**Fix**: Reference `ARC_FAMILY_COLORS` from `colors/generated.ts` instead of literal hex values

### Issue 2.3: TUI Hardcoded Color Constants
**File**: `tools/novanet/src/tui/theme.rs` (lines 102-328)
**Impact**: 13+ hex color constants that should come from YAML

| Category | Hardcoded Constants |
|----------|-------------------|
| Realms | SHARED_HEX, ORG_HEX |
| Layers | CONFIG_HEX, LOCALE_HEX, GEOGRAPHY_HEX, KNOWLEDGE_HEX, FOUNDATION_HEX, STRUCTURE_HEX, SEMANTIC_HEX, INSTRUCTION_HEX, OUTPUT_HEX |
| Traits | DEFINED_HEX, AUTHORED_HEX, IMPORTED_HEX, GENERATED_HEX, RETRIEVED_HEX |
| Arc Families | OWNERSHIP_HEX, LOCALIZATION_HEX, SEMANTIC_HEX, GENERATION_HEX, MINING_HEX |

**Fix**: Add `theme.generated.rs` generator that reads from taxonomy YAML files

### Issue 2.4: TUI Hardcoded Icon Defaults
**File**: `tools/novanet/src/tui/theme.rs` (lines 451-514)
**Impact**: 35+ hardcoded Unicode icons in `Icons::defaults()`

**Fix**: Always load from `visual-encoding.yaml`, fail loudly if missing (no silent defaults)

### Issue 2.5: Visual Encoding Icon Violations
**Files**: Multiple across TUI
**Impact**: 6 files have hardcoded icons that should come from YAML

| File | Issue |
|------|-------|
| `parsers/organizing.rs:129` | Legacy `emoji: "⚙️"` field |
| `parsers/taxonomy.rs:470, 581` | Hardcoded fallback emoji |
| `nodeIcons.generated.ts:43, 47` | Wrong icon category for some nodes |
| `tui/icons.rs:224-298` | Duplicate icons in match statements |

**Fix**: Consolidate icon loading to single source (`visual-encoding.yaml`)

---

## Phase 3: Medium Priority Issues (P2 - Documentation/Consistency)

### Issue 3.1: ROADMAP.md Outdated
**File**: `ROADMAP.md`
**Issues**:
- Line 3: Shows v0.12.0 as current (should be v0.13.0)
- Lines 39-41: Missing v0.13.0 milestone entry
- Line 260: Missing v0.13.0 in History section

**Fix**: Update all version references and add v0.13.0 milestone

### Issue 3.2: Nexus ADRs Missing Recent Decisions
**File**: `tools/novanet/src/tui/nexus/data.rs`
**Impact**: `get_all_adrs()` missing ADR-031 (SEO Pillar) and ADR-032 (Slugification)

**Fix**: Add ADR-031 and ADR-032 to hardcoded ADR list

### Issue 3.3: Stale Comments in Cypher Seeds
**Files**: `packages/db/seed/00-constraints.cypher`, `31-project-qrcode-ai.cypher`
**Impact**: Comments reference v10.9 terminology (HAS_CONTENT, ProjectL10n)

**Fix**: Update comments to reference v0.13.0 and HAS_NATIVE

### Issue 3.4: TypeScript Test Version Header
**File**: `packages/core/src/__tests__/types.test.ts`
**Issue**: Line 1 comment says "v10.3.0" but should say "v0.12.5"

**Fix**: Update version header comment

---

## Phase 4: Low Priority Issues (P3 - Polish)

### Issue 4.1: Edge Effect Colors Hardcoded
**File**: `apps/studio/src/components/graph/edges/system/themes.ts` (lines 25-68)
**Impact**: 24 hardcoded hex colors for edge effects (PALETTES)

**Fix**: Create `edge-effects.yaml` and generate `edgeEffectColors.generated.ts`

### Issue 4.2: TUI Nexus Layer Constants
**File**: `tools/novanet/src/tui/nexus/layers.rs`
**Impact**: `SHARED_LAYERS[4]` and `ORG_LAYERS[6]` hardcoded instead of loaded from TaxonomyTree

**Fix**: Refactor to use `TaxonomyTree.realms[].layers[]` at runtime

### Issue 4.3: Test Fixture Colors Inconsistent
**Files**: `data.rs`, `app.rs`, `overlays.rs`, `graph.rs` (in `tui/`)
**Impact**: 12+ hardcoded hex colors in test fixtures that don't match theme.rs constants

**Bug Found**: `data.rs:3275` uses `#d33682` for org realm but `theme.rs:103` defines `#6c71c4`

**Fix**: Update test fixtures to use theme.rs constants

### Issue 4.4: E2E Test Legacy Terminology
**File**: `apps/studio/e2e/schema-mode.spec.ts` (lines 37-93)
**Impact**: Test references "Meta mode" and "Meta badge" (v11.6 terminology)

**Fix**: Update test to use v11.7+ "Schema view" terminology

---

## Verification Checklist

### YAML Schema (✅ COMPLIANT)
- [x] 61 nodes (40 shared + 21 org) with correct structure
- [x] 169 arcs with proper inverse declarations
- [x] HAS_NATIVE unified arc (merged HAS_CONTENT + HAS_GENERATED)
- [x] NATIVE_OF inverse arc defined
- [x] All *Native nodes have correct traits (authored/generated)
- [x] ADR-024 traits (defined/authored/imported/generated/retrieved)
- [x] ADR-026 inverse arc policy (TIER 1/2/3 complete)

### TypeScript (✅ COMPLIANT)
- [x] No deprecated node names in type definitions
- [x] No deprecated trait names in type definitions
- [x] Generated files properly auto-generated
- [x] Nomenclature tests validate v0.13.0 compliance

### Rust Production Code (✅ COMPLIANT)
- [x] NodeTrait enum has all 5 correct values
- [x] Schema validation passes
- [x] Parser loads all YAML correctly

### Cypher Seeds (✅ COMPLIANT)
- [x] EntityNative, PageNative, BlockNative labels correct
- [x] HAS_NATIVE relationships used
- [x] Trait values correct

### Areas Requiring Updates (❌ NEEDS FIX)
- [ ] Rust TUI display strings (P0)
- [ ] Rust generator test fixtures (P0)
- [ ] View queries arc names (P0)
- [ ] Studio hardcoded mappings (P1)
- [ ] TUI hardcoded colors (P1)
- [ ] TUI hardcoded icons (P1)
- [ ] ROADMAP.md (P2)
- [ ] Nexus ADRs (P2)
- [ ] Seed file comments (P2)
- [ ] Edge effect colors (P3)
- [ ] Test fixture colors (P3)

---

## Execution Plan

### Batch 1: Critical Fixes (P0) — Est. 2h
```
1. Rust TUI display strings (80 occurrences)
   - Search-replace in 8 files
   - Run cargo test to verify

2. Generator test fixtures (20 occurrences)
   - Update test data
   - cargo insta review to update snapshots

3. View queries (5 arcs)
   - Fix viewQueries.ts and views.yaml
   - Test against Neo4j
```

### Batch 2: Source of Truth (P1) — Est. 4h
```
1. Create arc-families generator
   - Add to generators/mod.rs
   - Generate apps/studio/src/design/colors/arcFamilies.generated.ts

2. Create theme generator for TUI
   - Add generators/tui_theme.rs
   - Generate tools/novanet/src/tui/theme.generated.rs

3. Consolidate icon loading
   - Remove Icons::defaults() fallbacks
   - Ensure visual-encoding.yaml always loads

4. Update Studio relationshipTypes.ts
   - Reference generated arc family colors
```

### Batch 3: Documentation (P2) — Est. 1h
```
1. Update ROADMAP.md
   - Current version → v0.13.0
   - Add milestone entry
   - Add history entry

2. Add ADR-031 and ADR-032 to Nexus

3. Update seed file comments
```

### Batch 4: Polish (P3) — Est. 2h
```
1. Create edge-effects.yaml + generator
2. Refactor nexus/layers.rs to use TaxonomyTree
3. Fix test fixture color inconsistencies
4. Update E2E test terminology
```

---

## Metrics

| Metric | Before Audit | After Phase 1 | Target |
|--------|--------------|---------------|--------|
| Deprecated node names | 80+ | 0 | 0 |
| Deprecated arc names | 25+ | 0 | 0 |
| Hardcoded colors (TUI) | 50+ | 0 | 0 |
| Hardcoded arc mappings (Studio) | 53 | 0 | 0 |
| Invalid view queries | 5 | 0 | 0 |
| Test pass rate | 100% | 100% | 100% |

---

## Agent Reports Summary

| Agent | Domain | Status | Key Findings |
|-------|--------|--------|--------------|
| 1 | v0.13 Plans | ✅ | Documented ADR-029/030, execution plan exists |
| 2 | *Native Pattern | ⚠️ | YAML correct, Rust display strings outdated |
| 3 | Taxonomy Truth | ⚠️ | YAML correct, TUI has hardcoded colors |
| 4 | Visual Encoding | ⚠️ | 7 icon violations found |
| 5 | TUI Hardcoded | ⚠️ | 42+ hardcoded values (colors/icons/names) |
| 6 | Generators | ⚠️ | Test fixtures use deprecated names |
| 7 | Tests | ✅ | TypeScript tests v0.13.0 compliant |
| 8 | Studio Sources | ⚠️ | 180+ hardcoded schema values |
| 9 | Views YAML | ⚠️ | 5 undefined arc references |
| 10 | Node Classes | ✅ | 100% compliant (61 nodes verified) |
| 11 | Arc Classes | ✅ | 100% compliant (169 arcs verified) |
| 12 | Rust Nomenclature | ⚠️ | 80+ deprecated terms in display code |
| 13 | TS Nomenclature | ✅ | 100% compliant |
| 14 | Cypher Seeds | ✅ | Correct, minor stale comments |
| 15 | CHANGELOG/ROADMAP | ⚠️ | ROADMAP outdated |
| 16 | Nexus Mode | ✅ | Fully implemented, missing ADR-031/032 |
| 17 | Icons Truth | ✅ | Single source visual-encoding.yaml |
| 18 | Color ADR-004 | ⚠️ | 94% compliant, edge effects hardcoded |
| 19 | ADR-024 Traits | ✅ | 98% compliant, minor doc issues |
| 20 | ADR-025 Instruction | ✅ | 100% compliant |

---

## Success Criteria

- [ ] 0 references to deprecated patterns (EntityContent, ProjectContent, PageGenerated, BlockGenerated)
- [ ] 0 references to deprecated arcs (HAS_CONTENT, HAS_GENERATED, CONTENT_OF, GENERATED_FOR)
- [ ] All colors loaded from YAML (no hardcoded hex in production code)
- [ ] All icons loaded from visual-encoding.yaml
- [ ] All arc mappings generated from YAML
- [ ] View queries use valid arc names
- [ ] ROADMAP.md reflects v0.13.0
- [ ] 1031 Rust tests pass
- [ ] All TypeScript tests pass
- [ ] Schema validation: 0 errors, 0 warnings

---

## Files Changed Summary

| Category | Files | Est. Changes |
|----------|-------|--------------|
| Rust TUI | 8 | ~200 lines |
| Rust Generators | 2 | ~50 lines |
| Studio TS | 3 | ~100 lines |
| Views | 2 | ~20 lines |
| Documentation | 3 | ~50 lines |
| New Generators | 2 | ~200 lines |
| **TOTAL** | **20** | **~620 lines** |
