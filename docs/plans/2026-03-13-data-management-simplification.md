# Data Management Simplification

**Date**: 2026-03-13
**Status**: Approved (brainstorm with Thibaut)
**Version**: v0.19.0

## Problem

The 3-command data management system (`export` / `diff` / `promote`) was incomprehensible
to the user despite multiple UX improvement rounds. The core confusion:

1. **Mental model**: No clear picture of WHERE data lives and WHERE it comes from
2. **Intermediate copy**: `~/.novanet/export/` added confusion — why 2 YAML copies?
3. **Jargon**: "export", "promote" are developer terms, not user-friendly

## Decisions

### D1: Simplify from 3 commands to 2

```
BEFORE (3 commands, intermediate copy):
  Neo4j ──export──> ~/.novanet/export/ ──promote──> private-data/data/
                          └──diff──┘

AFTER (2 commands, direct):
  Neo4j ──backup──> private-data/data/
            └──status──┘
```

- `novanet data backup` → Save Neo4j data directly to `private-data/data/`
- `novanet data status` → Compare `private-data/data/` vs Neo4j (what changed?)
- `novanet data promote` → **REMOVED** (no more intermediate step)

### D2: Remove intermediate `~/.novanet/export/`

The export directory was a staging area for review before promoting. In practice:
- Users never reviewed the intermediate copy
- It added confusion about which copy is "real"
- Direct backup to `private-data/data/` is simpler and still allows `git diff`

### D3: Backup ALL data from Neo4j

- Shared data (expressions, patterns, culture) grows over time via Nika workflows
- Org data (entities, pages, blocks) is obviously private
- Both need backup — no realm distinction in the backup command

### D4: Manual backup (not automatic)

- For now, `novanet data backup` is manual
- Auto-backup can be added later if needed

## What's Public vs Private

The numbering system in `packages/db/seed/` creates the boundary:

| Range | Content | Git Status |
|-------|---------|------------|
| 00-02 | Schema (taxonomy, classes, arc-classes) | PUBLIC |
| 20-29 | Locale knowledge (locales, culture, expressions) | PUBLIC |
| 27-29 | Geography (countries, taxonomy) | PUBLIC |
| 10-11 | Entity bootstraps | PRIVATE (gitignored) |
| 30-39 | Org config, project, brand | PRIVATE (gitignored) |
| 40-49 | Pages, blocks, instructions | PRIVATE (gitignored) |
| 50-5x | SEO keywords, GEO queries | PRIVATE (gitignored) |
| 99    | Constraints, autowire | PUBLIC |

The `content/` symlink in the seed directory points to `private-data/seed/content/`
for entity/page/org data that needs private storage.

## Implementation

### Files to modify

1. **Rename** `data_export.rs` → `data_backup.rs` (change target to `private-data/data/`)
2. **Rename** `data_diff.rs` → `data_status.rs` (change source to `private-data/data/`)
3. **Delete** `data_promote.rs` (no longer needed)
4. **Update** `commands/mod.rs` and `main.rs` for new command names
5. **Remove** checkpoint logic (no intermediate staging)

### UX Pattern (unchanged)

```
Banner → Steps → Summary Box → Next Steps
```

Human-readable labels, no jargon.
