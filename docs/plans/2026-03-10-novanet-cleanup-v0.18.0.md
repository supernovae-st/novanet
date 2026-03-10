# NovaNet v0.18.0 Complete Cleanup Plan

**Date**: 2026-03-10
**Status**: In Progress
**Objective**: Codebase propre, cohérente, sans bruit

---

## Executive Summary

After updating all ADRs for trait consistency (ADR-024), we now need to ensure the **code** matches the documentation. This plan uses 10 parallel Opus 4.5 agents to audit and fix the entire NovaNet codebase.

## Success Criteria

- [ ] Zero references to old trait names in active code
- [ ] All seeds have `created_by` provenance
- [ ] No orphan files (YAML, Cypher, tests)
- [ ] No overlapping migrations
- [ ] All tests pass
- [ ] No unused dependencies

---

## Agent Assignments

### A1: Rust TUI/CLI - Old Trait Names

**Scope**: `novanet/tools/novanet/src/`

**Search for**:
```
- "invariant" (should be "defined")
- "localized" (should be "authored")
- "knowledge" as trait (should be "imported")
- "aggregated" (should be "retrieved")
```

**Files to check**:
- `tui/theme.rs` - trait colors
- `tui/icons.rs` - trait icons
- `tui/data.rs` - trait display
- `commands/*.rs` - CLI output

**Action**: Replace with correct trait names

---

### A2: Rust MCP Server - Provenance

**Scope**: `novanet/tools/novanet-mcp/src/`

**Verify**:
- `novanet_write` generates `created_by` automatically
- `novanet_check` validates `created_by` presence
- `novanet_audit` checks provenance

**Action**: Add missing provenance handling

---

### A3: TypeScript - Legacy Types

**Scope**: `novanet/packages/core/src/`, `novanet/apps/studio/src/`

**Search for**:
- Old trait enums/types
- Legacy naming in interfaces
- UI components with old terminology

**Action**: Update to match ADR-024

---

### A4: YAML Node Classes - Orphans

**Scope**: `brain/models/node-classes/`

**Check**:
- Files not in `_index.yaml`
- Files that should be in `_archive/`
- Duplicate definitions

**Action**: Archive or delete orphans

---

### A5: YAML Arc Classes - Obsolete

**Scope**: `brain/models/arc-classes/`

**Search for**:
- `HAS_CONTENT` (replaced by `HAS_NATIVE`)
- `CONTENT_OF` (replaced by `NATIVE_OF`)
- `HAS_GENERATED` (replaced by `HAS_NATIVE`)
- Unused arc definitions

**Action**: Move to `_archive/` or delete

---

### A6: Cypher Seeds - Legacy Patterns

**Scope**: `novanet/packages/db/seed/`

**Check**:
- MERGE statements without `created_by`
- References to old node types
- Orphan seed files

**Action**: Add `created_by`, remove obsolete files

---

### A7: Migrations - Duplicates & Overlap

**Scope**: `novanet/packages/db/seed/migrations/`

**Check**:
- Migrations doing the same thing
- Migrations for removed features
- Migrations that can be consolidated

**Action**: Consolidate or remove redundant migrations

---

### A8: Tests - Dead Code

**Scope**: `novanet/tools/novanet/tests/`, `novanet/packages/*/tests/`

**Check**:
- Tests for removed features
- Tests with old trait names
- Snapshot tests needing update

**Action**: Remove or update tests

---

### A9: Dependencies - Unused

**Scope**: `Cargo.toml`, `package.json`

**Tools**:
- `cargo machete` (Rust)
- `depcheck` (Node)

**Action**: Remove unused dependencies

---

### A10: Final Coherence

**Scope**: Entire NovaNet

**Verify**:
- YAML schema matches code
- Code matches Neo4j schema
- All tests pass
- No lint errors

**Action**: Fix remaining inconsistencies

---

## Execution Order

```
Phase 1 (Parallel): A1, A2, A3, A4, A5 - Audit & identify
Phase 2 (Parallel): A6, A7, A8, A9 - Cleanup & fixes
Phase 3 (Sequential): A10 - Final verification
```

## Commit Strategy

Each significant fix gets its own commit:
```
fix(tui): replace legacy trait names with ADR-024 terminology
fix(mcp): add created_by provenance to novanet_write
fix(schema): archive obsolete HAS_CONTENT arc class
fix(seed): add created_by to all MERGE statements
fix(migrations): consolidate overlapping migrations 070-085
chore(deps): remove unused dependencies
```

---

## Rollback Plan

All changes are atomic commits. If issues arise:
```bash
git revert <commit-sha>
```

---

## Notes

- Private content files (10-*, 3[1-9]-*, 4*, 5*) are gitignored
- Focus on public schema and code
- Maintain backward compatibility where possible
