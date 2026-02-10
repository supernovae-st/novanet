# DX Improvements Plan v11.5

**Date**: 2026-02-10
**Status**: In Progress
**Based on**: Brainstorming session findings

---

## Executive Summary

Clean up remaining inconsistencies, improve schema documentation quality, and enhance developer experience based on research insights from Perplexity, Context7, and Neo4j MCP.

---

## Tasks

### Phase 1: Documentation Cleanup (Priority: HIGH)

- [x] Fix arc count discrepancy (116 → 114)
- [x] Update ADR version headers (v11.3 → v11.5)
- [x] Add missing layer mappings (locale, geography)
- [ ] **1.1** Update CHANGELOG with today's fixes
- [ ] **1.2** Verify VERSION file matches all docs
- [ ] **1.3** Check novanet-terminology.md for stale refs

### Phase 2: Schema Quality (Priority: MEDIUM)

- [ ] **2.1** Audit deprecated properties in YAML (v7.2.0 refs)
- [ ] **2.2** Remove or document deprecated fields
- [ ] **2.3** Verify all arc-kinds have consistent structure
- [ ] **2.4** Check cypher_pattern consistency

### Phase 3: Rust Codebase Cleanup (Priority: MEDIUM)

- [ ] **3.1** Remove stale "global/tenant" test comments
- [ ] **3.2** Update test fixtures to use "shared/org"
- [ ] **3.3** Run clippy and fix any new warnings
- [ ] **3.4** Run cargo fmt

### Phase 4: Sync Verification (Priority: HIGH)

- [ ] **4.1** Run `cargo run -- schema validate`
- [ ] **4.2** Run `cargo run -- schema generate --dry-run`
- [ ] **4.3** Verify Neo4j schema matches YAML (60/114)
- [ ] **4.4** Run full test suite

---

## Verification Checklist

```bash
# Phase 1
cat VERSION  # Should show 11.5.0
grep -r "116" .claude/  # Should return 0 matches for arc count

# Phase 2
grep -r "deprecated" packages/core/models/node-kinds/  # Review deprecated refs

# Phase 3
cargo clippy -- -D warnings
cargo fmt --check

# Phase 4
cargo run -- schema validate
cargo test
```

---

## Success Criteria

1. All documentation shows consistent v11.5.0 version
2. Arc count is 114 everywhere (no 116 references)
3. All deprecated properties are documented or removed
4. `cargo clippy` and `cargo fmt` pass
5. `cargo test` passes (950+ tests)
6. `schema validate` passes

---

## Notes

- Keep backward compat for seo/locale-knowledge layer mappings
- Don't break existing Neo4j data during cleanup
- Commit after each phase for easy rollback
