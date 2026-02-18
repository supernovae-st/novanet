# Ralph Wiggum Audit - Consolidated Fix Plan

**Date**: 2026-02-07
**Version**: v10.6.0 → v10.7.0 (production-ready)
**Methodology**: 10-Agent Parallel Audit + TDD

---

## Executive Summary

10 parallel audit agents analyzed the NovaNet codebase and found:

| Category | Critical | High | Medium | Low |
|----------|----------|------|--------|-----|
| Dead Code | 1 | 1 | 0 | 0 |
| Legacy Patterns | 0 | 1 | 3 | 2 |
| Code Quality | 0 | 0 | 3 | 1 |
| Test Coverage | 0 | 1 | 2 | 0 |
| Security | 1 | 1 | 0 | 0 |
| DX | 0 | 3 | 5 | 3 |
| Documentation | 0 | 2 | 4 | 2 |
| TS/YAML Sync | 1 | 0 | 0 | 0 |
| Performance | 0 | 2 | 6 | 2 |
| **TOTAL** | **3** | **11** | **23** | **10** |

---

## CRITICAL FIXES (Must Fix Before Production)

### C1: TypeScript Out of Sync with YAML (Agent 9)
**Files**: `packages/core/src/types/nodes.ts`
**Issue**: 12 node types missing from TypeScript (geographic/linguistic taxonomy)
**Missing**: Continent, GeoRegion, GeoSubRegion, IncomeGroup, LendingCategory, EconomicRegion, LanguageFamily, LanguageBranch, CulturalRealm, CulturalSubRealm, PopulationCluster, PopulationSubCluster
**Fix**: Run `cargo run -- schema generate`
**TDD**: Verify TypeScript compiles, Studio can import all node types

### C2: Dead Code - Adaptation Module (Agent 1)
**Files**: `src/parsers/adaptation.rs` (816 lines)
**Issue**: Module is NEVER called anywhere in codebase
**Fix**: Delete entire file
**TDD**: `cargo test` still passes, no compile errors

### C3: Dead Code - Retrieval Module (Agent 1)
**Files**: `src/retrieval/*` (5 files, ~400 lines)
**Issue**: Module is never integrated into commands
**Fix**: Delete entire directory
**TDD**: `cargo test` still passes, no compile errors

---

## HIGH PRIORITY FIXES

### H1: Hardcoded Default Password (Agent 5)
**File**: `src/main.rs:25`
**Issue**: `default_value = "novanetpassword"` in CLI args
**Fix**: Remove default_value, require explicit password or env var
**TDD**: CLI errors without NEO4J_PASSWORD set

### H2: Hardcoded Absolute Path (Agent 2)
**File**: `src/generators/culture.rs:15`
**Issue**: `/Users/thibaut/Projects/traduction_ai/...` hardcoded
**Fix**: Remove or make configurable via env var
**TDD**: Generator works without local path

### H3: Sequential Realm Details N+1 (Agent 10)
**File**: `src/tui/data.rs:901-962`
**Issue**: 2 sequential queries could be 1
**Fix**: Combine into single aggregation query
**TDD**: Same results, fewer round-trips (measure with timing)

### H4: Context Engine BFS N+1 (Agent 10)
**File**: `src/retrieval/engine.rs:50-68`
**Issue**: Each node in BFS queue triggers separate query
**Fix**: Batch neighbor fetching
**TDD**: Profile shows reduced query count

### H5: Outdated "project" Realm References (Agent 2)
**Files**: Multiple snapshots and test files
**Issue**: v10.5 "project" realm should be "tenant"
**Fix**: Update snapshots and test data
**TDD**: `cargo insta review` shows no project refs

### H6-H8: DX P1 Issues (Agent 6)
- Root discovery error needs next steps
- Node/Arc validation should suggest valid values
- JSON parse errors should show position

---

## MEDIUM PRIORITY FIXES

### M1-M3: Documentation Count Fixes (Agents 7, 8)
**Files**: `.claude/README.md`, CLAUDE.md, skills
**Issue**: Node/arc counts wrong (46/51 → 60/92)
**Fix**: Update all count references

### M4: Filter Example Uses "project" (Agent 7)
**File**: `tools/novanet/CLAUDE.md:141`
**Issue**: Example uses `"project"` instead of `"tenant"`
**Fix**: Change to `{"realms":["tenant"]}`

### M5-M10: Performance Improvements (Agent 10)
- Vec::new() without capacity
- Generator sequential execution
- TUI format! allocations
- Missing fulltext search index
- Redundant kind count computation

### M11: TUI Help Missing Mode 5-6 (Agent 6)
**File**: `src/tui/ui/overlays.rs:189-197`
**Fix**: Change "1-4" to "1-6" for Atlas/Audit modes

### M12: db reset Needs --confirm (Agent 6)
**Fix**: Add confirmation flag like node delete

### M13: time Crate DoS Vulnerability (Agent 5)
**Issue**: RUSTSEC-2026-0009
**Fix**: Update dependency or document in deny.toml

---

## LOW PRIORITY FIXES

### L1: Long Functions (Agent 3)
- `run_app` (168 lines)
- `render_tree` (200+ lines)

### L2: 576 .unwrap() Calls (Agent 3)
Most in generators, not critical path

### L3: Color Legend Missing Trait Borders (Agent 6)

### L4: eprintln vs tracing Inconsistency (Agent 6)

### L5: YAML Cache Full Clone (Agent 10)

---

## Execution Order

### Batch 1: Critical Fixes (TDD)
1. [ ] Run `cargo run -- schema generate` (C1)
2. [ ] Delete `src/parsers/adaptation.rs` (C2)
3. [ ] Delete `src/retrieval/` directory (C3)
4. [ ] Verify: `cargo test`, `pnpm type-check`

### Batch 2: High Priority Fixes (TDD)
5. [ ] Remove hardcoded password default (H1)
6. [ ] Remove hardcoded culture.rs path (H2)
7. [ ] Update outdated realm references (H5)
8. [ ] Update snapshots: `cargo insta review`

### Batch 3: Documentation Sync
9. [ ] Update node/arc counts in all docs
10. [ ] Fix filter example realm
11. [ ] Update CLAUDE.md test counts

### Batch 4: Performance Fixes
12. [ ] Combine realm details queries (H3)
13. [ ] Add Vec::with_capacity() hints
14. [ ] Parallelize generators with rayon

### Batch 5: DX Improvements
15. [ ] Improve error messages (H6-H8)
16. [ ] Add db reset confirmation
17. [ ] Update help overlay for modes 5-6

### Batch 6: Test Coverage
18. [ ] Add generator snapshot tests
19. [ ] Add db.rs unit tests
20. [ ] Add commands integration tests

---

## Verification Commands

```bash
# After Batch 1
cargo test
cargo clippy -- -D warnings
pnpm type-check

# After Batch 2
cargo deny check

# After Batch 3
grep -r "\"project\"" packages/core/models/ --include="*.yaml" | wc -l  # Should be 0 for realm refs

# After Batch 4
cargo nextest run --profile ci

# After all
cargo build --release
pnpm build
```

---

## Success Criteria

- [ ] All 499+ tests pass
- [ ] Zero clippy warnings
- [ ] TypeScript compiles with 60 node types
- [ ] No hardcoded paths or credentials
- [ ] All CLAUDE.md counts accurate
- [ ] Security audit passes (cargo deny check)
- [ ] Documentation reflects v10.6 reality
