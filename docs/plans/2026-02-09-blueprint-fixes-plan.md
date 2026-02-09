# Blueprint Fixes Plan

**Date**: 2026-02-09
**Status**: In Progress

## Issues Identified by 5-Agent Review

### Critical (Must Fix Now)

| # | Issue | File | Fix |
|---|-------|------|-----|
| 1 | Arc scope validation missing | validation.rs | Add `check_arc_scope_coherence()` |
| 2 | Scope field not in ArcDef | parsers/arcs.rs | Add `scope: Option<String>` to ArcDef |
| 3 | YAML arc files have stale scope | arc-kinds/*.yaml | Update 2 files to `intra_realm` |

### Medium (Should Fix)

| # | Issue | File | Fix |
|---|-------|------|-----|
| 4 | visual-encoding.yaml not loaded | sources.rs | Add to BlueprintData (optional) |
| 5 | Code duplication | views/*.rs | Centralize utilities in ascii.rs |

## Execution Plan

### Phase 1: Fix Scope in ArcDef (5 min)

1. Add `scope: Option<String>` to `ArcDef` struct in `parsers/arcs.rs`
2. Populate scope when converting from `ArcKindDef` to `ArcDef`

### Phase 2: Add Arc Scope Validation (10 min)

1. Add `check_arc_scope_coherence()` to `validation.rs`
2. Validate that declared scope matches actual source/target realms
3. Report errors when:
   - `scope: intra_realm` but source.realm ≠ target.realm
   - `scope: cross_realm` but source.realm = target.realm

### Phase 3: Fix Stale YAML Files (5 min)

Fix these 2 files that have incorrect scope after v11.0 SEO migration:

1. `arc-kinds/ownership/has-seo-keywords.yaml`
   - Source: Locale (global? or tenant?)
   - Target: SEOKeyword (tenant)
   - Current: `scope: cross_realm`
   - Fix: Check actual realms and update

2. `arc-kinds/ownership/has-geo-queries.yaml`
   - Source: Locale
   - Target: GEOQuery
   - Current: `scope: cross_realm`
   - Fix: Check actual realms and update

### Phase 4: Centralize Utilities (5 min)

Move duplicated functions to `ascii.rs`:
- `truncate(s, max_len)` - from 4 view files
- `pad_right(s, width)` - from 2 view files

### Phase 5: Verification

Run 3 parallel agents:
1. Code reviewer on validation.rs changes
2. YAML schema validator
3. Full test suite runner

## Success Criteria

- [ ] All 925+ tests pass
- [ ] Zero clippy warnings
- [ ] Arc scope validation catches mismatches
- [ ] Stale YAML files corrected
- [ ] No code duplication in views
