# Legacy Cleanup Plan: v0 = No Backward Compatibility

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to execute tasks.

**Goal:** Remove ALL legacy patterns, backward compatibility shims, deprecated code, and historical comments. We're v0 - clean slate.

**Principle:** If it's not actively used, DELETE it. No "kept for compatibility", no "deprecated but still works".

**Tech Stack:** Grep/Glob (discovery), Edit (removal), Rust/TS validation

---

## What to Remove

### 1. Legacy Terminology
- `*L10n` patterns (except where actively used)
- `HAS_L10N` arc references
- Old naming: `EdgeKind`, `Relation`, `Scope` (for realm)
- `DataMode` (should be `NavigationMode`)
- `category` (should be `trait`)

### 2. Backward Compatibility Code
- `// kept for backward compatibility` comments
- `// deprecated` markers with unused code
- Re-exports for renamed types
- Fallback code paths for old formats
- `_legacy/` directory contents (archive externally or delete)

### 3. Historical Comments
- `// v9.0: renamed from...` (we don't need history in code)
- `// TODO: remove after migration` (migrate NOW)
- Version change logs in code (belongs in CHANGELOG.md)
- `// RENAMED:` comments

### 4. Unused Code
- Dead functions/structs
- Unused imports
- Test helpers that aren't used
- Old seed files that aren't executed

### 5. Old Patterns
- 3-realm references (now 2 realms: global/tenant)
- `organization` realm (merged into tenant)
- `project` realm (merged into tenant)
- Old layer counts (was 10, now 9)

---

## Sniper Agent Assignments

| Sniper | Focus Area | Search Patterns |
|--------|------------|-----------------|
| 1 | Legacy L10n in Rust | `L10n`, `l10n`, `HAS_L10N` in `tools/novanet/src/` |
| 2 | Legacy L10n in TypeScript | `L10n`, `l10n` in `packages/*/src/` |
| 3 | Legacy L10n in YAML | `L10n`, `l10n` in `packages/core/models/` |
| 4 | Legacy L10n in Cypher | `L10n`, `l10n`, `HAS_L10N` in `packages/db/seed/` |
| 5 | Backward compat comments | `backward`, `compat`, `deprecated`, `legacy` |
| 6 | Old realm references | `organization`, `project` realm, 3-realm |
| 7 | Version history comments | `v9`, `v10.0`, `v10.1`, `RENAMED:`, `renamed from` |
| 8 | Unused/dead code Rust | `#[allow(dead_code)]`, unused warnings |
| 9 | _legacy directory audit | Everything in `_legacy/` directories |
| 10 | Old terminology | `EdgeKind`, `Relation`, `DataMode`, `Subcategory` |

---

## Success Criteria

| Metric | Target |
|--------|--------|
| `L10n` occurrences (non-EntityContent) | 0 |
| `backward` / `compat` comments | 0 |
| `deprecated` markers | 0 |
| `_legacy/` files | Archived or deleted |
| `organization` realm refs | 0 |
| `project` realm refs | 0 |
| Old terminology | 0 |
| Schema validates | 0 errors |
| Tests pass | All green |

---

## Execution Order

```
Phase 1: Discovery (10 snipers in parallel)
    ↓
Phase 2: Triage (categorize findings by severity)
    ↓
Phase 3: Cleanup (remove/update in batches)
    ↓
Phase 4: Validation (tests + schema validate)
    ↓
Phase 5: Commit (one commit per logical cleanup)
```
