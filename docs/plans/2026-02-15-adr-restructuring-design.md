# ADR Restructuring Design

**Date**: 2026-02-15
**Status**: Draft
**Goal**: Reduce `novanet-decisions.md` from 99k to <40k chars via domain-based split

---

## Problem Statement

```
Current:  novanet-decisions.md = 99k chars (2.5x limit)
Target:   Each file < 40k chars
Impact:   Claude Code performance degradation on every conversation
```

---

## Proposed Structure

```
.claude/rules/
├── novanet-decisions.md          # Slim index (~30k) - Quick Ref + TOC
├── novanet-terminology.md        # Current terms (~20k) - deduplicated
├── arc-design-guide.md           # Keep as-is (7k)
├── security.md                   # Keep as-is (5k)
├── cypher.md                     # Keep as-is (2k)
├── rust.md                       # Keep as-is (1k)
├── typescript.md                 # Keep as-is (1k)
│
└── adr/                          # NEW: Domain-based ADR split
    ├── _index.yaml               # Machine-readable metadata (all 32 ADRs)
    │
    ├── core-principles/          # 5 ADRs (~12k)
    │   ├── README.md             # Domain intro
    │   ├── adr-001-arc-terminology.md
    │   ├── adr-003-yaml-first.md
    │   ├── adr-007-generation-not-translation.md
    │   ├── adr-010-skill-first-dx.md
    │   └── adr-021-query-first.md
    │
    ├── schema-architecture/      # 6 ADRs (~18k)
    │   ├── README.md
    │   ├── adr-006-realm-scope.md
    │   ├── adr-012-two-realm.md
    │   ├── adr-017-entity-category.md
    │   ├── adr-028-page-entity.md
    │   ├── adr-029-native-pattern.md
    │   └── adr-030-slug-ownership.md
    │
    ├── node-classification/      # 5 ADRs (~15k)
    │   ├── README.md
    │   ├── adr-002-symmetric-taxonomy.md
    │   ├── adr-023-class-instance.md
    │   ├── adr-024-trait-data-origin.md
    │   └── adr-025-instruction-layer.md
    │
    ├── arc-design/               # 5 ADRs (~12k)
    │   ├── README.md
    │   ├── adr-015-unidirectional-ownership.md
    │   ├── adr-016-type-constrained-containers.md
    │   ├── adr-026-inverse-arc-policy.md
    │   └── adr-027-generation-family.md
    │
    ├── visual-encoding/          # 4 ADRs (~10k)
    │   ├── README.md
    │   ├── adr-004-no-color-duplication.md
    │   ├── adr-005-trait-visual-encoding.md
    │   ├── adr-009-terminal-colors.md
    │   └── adr-013-icons-source.md
    │
    ├── ux-architecture/          # 3 ADRs (~10k)
    │   ├── README.md
    │   ├── adr-008-invariant-structure.md
    │   └── adr-022-unified-tree.md
    │
    ├── seo-geo/                  # 2 ADRs (~8k)
    │   ├── README.md
    │   ├── adr-031-seo-pillar-cluster.md
    │   └── adr-032-url-slugification.md
    │
    └── deprecated/               # 4 ADRs (~8k) - Historical only
        ├── README.md
        ├── adr-011-company-project.md      # Superseded by 012
        ├── adr-014-l10n-to-content.md      # Superseded by 029
        ├── adr-018-classification-refinement.md
        ├── adr-019-layer-reorganization.md
        └── adr-020-schema-refinement.md
```

---

## New novanet-decisions.md (Slim Index)

Target: ~30k chars (currently 99k)

```markdown
# NovaNet Architecture Decisions (v0.13.0)

## Quick Reference

[Keep existing Quick Reference section - ~5k chars]

## ADR Index by Domain

### Core Principles (5 ADRs)
| ADR | Title | Status |
|-----|-------|--------|
| 001 | Arc Terminology | ✅ Stable |
| 003 | YAML-First Architecture | ✅ Core |
| 007 | Generation, Not Translation | ✅ Core |
| 010 | Skill-First DX | ✅ Stable |
| 021 | Query-First Architecture | ✅ Active |

[Similar tables for other 7 domains]

## Decision Log (Chronological)

[Keep existing Decision Log table - ~2k chars]

## Where to Find Details

- **Full ADR content**: `.claude/rules/adr/{domain}/adr-XXX-*.md`
- **Machine-readable index**: `.claude/rules/adr/_index.yaml`
- **Terminology**: `.claude/rules/novanet-terminology.md`
- **Arc design guide**: `.claude/rules/arc-design-guide.md`
```

---

## _index.yaml Schema

```yaml
# .claude/rules/adr/_index.yaml
version: "0.13.0"
generated: "2026-02-15"

domains:
  core-principles:
    description: "Foundational philosophy and methodology"
    adrs: [1, 3, 7, 10, 21]

  schema-architecture:
    description: "Realm, layer, and node organization"
    adrs: [6, 12, 17, 28, 29, 30]

  # ... etc

adrs:
  - id: 1
    name: "Arc Terminology"
    version: "v9.5"
    status: stable
    domain: core-principles
    file: "core-principles/adr-001-arc-terminology.md"
    summary: "Use 'Arc' (not Edge/Relation) for directed links"
    depends_on: []
    superseded_by: null

  - id: 29
    name: "*Native Pattern"
    version: "v0.12.5"
    status: active
    domain: schema-architecture
    file: "schema-architecture/adr-029-native-pattern.md"
    summary: "Unified *Native suffix for locale-specific nodes"
    depends_on: [14, 24]
    supersedes: 14

  # ... all 32 ADRs
```

---

## DX Updates Required

### 1. Skills to Update/Create

```
.claude/skills/
├── novanet-architecture/SKILL.md  # UPDATE: Reference new ADR paths
├── novanet-adr/SKILL.md           # NEW: ADR discovery skill
└── INDEX.md                       # UPDATE: Add novanet-adr
```

**New skill: novanet-adr**
```markdown
---
name: novanet-adr
description: Find and display NovaNet ADR documentation
triggers: [adr, decision, why, rationale, architecture decision]
---

# NovaNet ADR Navigation

## Quick Lookup
- ADR by number: `Read .claude/rules/adr/_index.yaml` → find file path
- ADR by topic: Search domain folders
- Active ADRs: Filter by `status: active`

## Domain Quick Reference
- Core principles: 001, 003, 007, 010, 021
- Schema: 006, 012, 017, 028, 029, 030
- Classification: 002, 023, 024, 025
- Arcs: 015, 016, 026, 027
- Visual: 004, 005, 009, 013
- UX: 008, 022
- SEO: 031, 032
```

### 2. CLAUDE.md Updates

**Root CLAUDE.md** - Update ADR reference section:
```markdown
## Architecture Decisions

ADRs are organized by domain in `.claude/rules/adr/`:
- **Quick ref**: `.claude/rules/novanet-decisions.md`
- **Full details**: `.claude/rules/adr/{domain}/adr-XXX-*.md`
- **Machine index**: `.claude/rules/adr/_index.yaml`

Key ADRs for v0.13.0:
- ADR-029: *Native Pattern (EntityNative, PageNative, etc.)
- ADR-030: Slug Ownership (Page owns URL, Entity owns semantics)
- ADR-024: Trait = Data Origin (defined/authored/imported/generated/retrieved)
```

**tools/novanet/CLAUDE.md** - Add ADR quick reference

### 3. Commands to Add

```
.claude/commands/
└── adr.md    # NEW: /adr command for quick ADR lookup
```

```markdown
---
name: adr
description: Look up NovaNet ADR by number or topic
arguments:
  - name: query
    description: ADR number (e.g., "029") or keyword (e.g., "native")
---

# ADR Lookup

1. Read `.claude/rules/adr/_index.yaml`
2. Find matching ADR(s)
3. Display summary + file path
4. Optionally read full ADR content
```

---

## Migration Checklist

### Phase 1: Create Structure (30 min)
- [ ] Create `.claude/rules/adr/` directory
- [ ] Create 8 domain subdirectories
- [ ] Create `_index.yaml` with all 32 ADRs

### Phase 2: Split ADRs (2h)
- [ ] Extract each ADR to individual file
- [ ] Add frontmatter (id, title, version, status, domain)
- [ ] Create domain README.md files
- [ ] Verify no content lost

### Phase 3: Create Slim Index (1h)
- [ ] Rewrite `novanet-decisions.md` as index only
- [ ] Keep Quick Reference section
- [ ] Add domain tables with links
- [ ] Verify < 35k chars

### Phase 4: Deduplicate (1h)
- [ ] Remove duplicates from `novanet-terminology.md`
- [ ] Add cross-references instead of duplicating
- [ ] Verify consistency

### Phase 5: Update DX (1h)
- [ ] Create `novanet-adr` skill
- [ ] Create `/adr` command
- [ ] Update CLAUDE.md files (root, tools/novanet, packages/core)
- [ ] Update `.claude/README.md`
- [ ] Update `novanet-architecture` skill

### Phase 6: Validate (30 min)
- [ ] Run `cargo test` (Rust references to ADRs)
- [ ] Run `pnpm type-check` (TS references)
- [ ] Verify all cross-references work
- [ ] Test new skill/command

---

## File Size Projections

| File | Current | After |
|------|---------|-------|
| novanet-decisions.md | 99k | ~30k |
| novanet-terminology.md | 23k | ~18k |
| adr/_index.yaml | 0 | ~8k |
| adr/core-principles/* | 0 | ~12k |
| adr/schema-architecture/* | 0 | ~18k |
| adr/node-classification/* | 0 | ~15k |
| adr/arc-design/* | 0 | ~12k |
| adr/visual-encoding/* | 0 | ~10k |
| adr/ux-architecture/* | 0 | ~10k |
| adr/seo-geo/* | 0 | ~8k |
| adr/deprecated/* | 0 | ~8k |
| **TOTAL** | **122k** | **~149k** |

Note: Total increases but **each file < 40k** = no performance warning.

---

## Rollback Plan

If issues arise:
1. Git revert the commit
2. Original `novanet-decisions.md` restored
3. New `adr/` folder can be deleted

Low risk: purely additive structure change.

---

## Success Criteria

- [ ] No file > 40k chars in `.claude/rules/`
- [ ] All 32 ADRs accessible via `_index.yaml`
- [ ] `/adr 029` command works
- [ ] `cargo test` passes
- [ ] `pnpm type-check` passes
- [ ] TUI Nexus ADR list still works (reads from data.rs, not decisions.md)
