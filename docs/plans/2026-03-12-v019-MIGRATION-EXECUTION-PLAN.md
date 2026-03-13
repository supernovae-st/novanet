# NovaNet v0.19.0 Migration Execution Plan

**Date**: 2026-03-12
**Version**: v0.19.0
**Status**: AWAITING APPROVAL

---

## Executive Summary

Cette migration profonde aligne toute la codebase NovaNet avec la nouvelle architecture:
- **Block Architecture**: BlockType + BlockRules + BlockInstruction → BlockNative
- **Page-Entity**: [:ABOUT {weight, role}] remplace [:REPRESENTS]
- **8 Standard Properties**: sur TOUS les nodes
- **Traits DEPRECATED**: provenance per-instance

---

## 🚨 DECISION REQUIRED: Property Naming

| Node | User Spec | Current YAML | Decision Needed |
|------|-----------|--------------|-----------------|
| BlockType | `payload` (JSON schema) | `structure` | **A or B?** |
| BlockInstruction | `instruction` (markdown) | `payload` | **A or B?** |
| BlockNative | `payload` (JSON output) | `payload` | OK |

**Option A**: Rename YAML properties to match user spec
**Option B**: Keep YAML, update documentation

---

## Phase 1: ADR Updates (8 files)

### 1.1 ADR-025 (Instruction Layer)
**File**: `dx/adr/novanet/node-classification/adr-025-instruction-layer.md`
**Changes**:
- [ ] Add BlockRules node to pipeline diagram
- [ ] Add [:HAS_RULES] arc (BlockType → BlockRules)
- [ ] Clarify property names (payload vs structure)

### 1.2 ADR-027 (Generation Family)
**File**: `dx/adr/novanet/arc-design/adr-027-generation-family.md`
**Changes**:
- [ ] Update generation formula: `BlockType + BlockRules + BlockInstruction → BlockNative`
- [ ] Show all three inputs contributing to output

### 1.3 ADR-028 (Page-Entity)
**File**: `dx/adr/novanet/schema-architecture/adr-028-page-entity.md`
**Changes**:
- [ ] Mark [:REPRESENTS] section as SUPERSEDED by ADR-046
- [ ] Update diagram to show [:ABOUT {weight, role}]
- [ ] Split "constraints" into BlockType.payload + BlockRules.rules

### 1.4 ADR-044 (Standard Properties)
**File**: `dx/adr/novanet/schema-architecture/adr-044-standard-properties.md`
**Changes**:
- [ ] Add BlockType with `payload` property
- [ ] Add BlockRules with `rules` property
- [ ] Add BlockInstruction with `instruction` property
- [ ] Update BlockNative: `payload` (not `html_content`)

### 1.5 ADR-030 (Slug Ownership)
**File**: `dx/adr/novanet/schema-architecture/adr-030-slug-ownership.md`
**Changes**:
- [ ] Rename `schema:` to `payload:` in examples

### 1.6 Create ADR-047 (Block Generation Architecture)
**File**: `dx/adr/novanet/schema-architecture/adr-047-block-generation.md` (NEW)
**Content**:
- [ ] Document full Block architecture
- [ ] Document property names
- [ ] Document generation formula
- [ ] Document arc relationships

### 1.7 CHEAT-SHEET
**File**: `dx/adr/novanet/CHEAT-SHEET.md`
**Changes**:
- [ ] Line 185: Update Page-Entity to reference ADR-046

### 1.8 Schema README
**File**: `dx/adr/novanet/schema-architecture/README.md`
**Changes**:
- [ ] Update instruction layer count to include all nodes

---

## Phase 2: DX Rules Updates (11 files)

### 2.1 CRITICAL: Replace [:REPRESENTS] references
| File | Line | Change |
|------|------|--------|
| `dx/.claude/rules/adr-quick-reference.md` | 211 | [:REPRESENTS] → [:ABOUT] |
| `dx/.claude/rules/schema-standard.md` | 116-120 | Update REPRESENTS section |
| `novanet/.claude/rules/adr-quick-reference.md` | 211 | [:REPRESENTS] → [:ABOUT] |
| `novanet/.claude/rules/schema-standard.md` | 116-120 | Update REPRESENTS section |

### 2.2 CRITICAL: Remove deprecated [:HAS_TRAIT]
**File**: `dx/.claude/rules/cypher.md:18`
- [ ] Remove `[:HAS_TRAIT]` Cypher pattern

### 2.3 HIGH: Mark traits as deprecated
| File | Lines | Action |
|------|-------|--------|
| `dx/.claude/rules/novanet-decisions.md` | 38,45,54-64,111,132 | Mark traits deprecated |
| `dx/.claude/rules/schema-standard.md` | 239-247 | Mark Trait Definitions deprecated |
| `dx/.claude/rules/typescript.md` | 34 | Mark trait definition deprecated |
| `dx/.claude/rules/adr-quick-reference.md` | 201,205,209 | Mark traits deprecated |

### 2.4 MEDIUM: Add Block architecture documentation
- [ ] Add BlockType.payload documentation
- [ ] Add BlockInstruction.instruction documentation
- [ ] Add BlockRules.rules documentation
- [ ] Add generation formula

### 2.5 MEDIUM: Add [:ABOUT] arc documentation
- [ ] Document weight and role properties
- [ ] Document roles: focus, support, reference, compare
- [ ] Document validation: exactly 1 focus per Page

### 2.6 LOW: Update node/arc counts
**Files**: Multiple
- [ ] Update "57 nodes, 145 arcs" if changed

---

## Phase 3: Schema YAML Updates (6 files)

### 3.1 Block-related YAML fixes

#### block.yaml
**File**: `packages/core/models/node-classes/org/structure/block.yaml`
- [ ] Line 66: Fix example `description` → `content`

#### block-type.yaml
**File**: `packages/core/models/node-classes/org/instruction/block-type.yaml`
- [ ] Line 66-70: Rename `structure` → `payload` (if Option A)
- [ ] Line 79: Fix example `description` → `content`

#### block-instruction.yaml
**File**: `packages/core/models/node-classes/org/instruction/block-instruction.yaml`
- [ ] Line 55-75: Rename `payload` → `instruction` (if Option A)

### 3.2 Page YAML fixes

#### page.yaml
**File**: `packages/core/models/node-classes/org/structure/page.yaml`
- [ ] Line 76: Replace [:REPRESENTS] with [:ABOUT {role: "focus"}]
- [ ] Lines 82-84: Replace [:REPRESENTS] with [:ABOUT {role: "focus"}]

---

## Phase 4: Seed Cypher Updates (9 files)

### 4.1 CRITICAL: Replace [:REPRESENTS] with [:ABOUT]
| File | Line | Change |
|------|------|--------|
| `40-page-block-instances.cypher` | 35 | `[:REPRESENTS]` → `[:ABOUT {weight: 1.0, role: 'focus'}]` |
| `48-page-block-qr-code.cypher` | 32 | `[:REPRESENTS]` → `[:ABOUT {weight: 1.0, role: 'focus'}]` |

### 4.2 CRITICAL: Remove slug from Page
| File | Line | Change |
|------|------|--------|
| `40-page-block-instances.cypher` | 20 | Remove `p.slug = 'qr-code'` |

### 4.3 COMPLETE REWRITE: Files missing 3+ standard properties

#### 49-blocknative-head-seo-meta.cypher
**Add to ALL BlockNative nodes (5 locales)**:
```cypher
bn.node_class = 'BlockNative',
bn.llm_context = 'USE: for SEO metadata in generation. TRIGGERS: slug, meta. NOT: for content. RELATES: Page (HAS_NATIVE).',
bn.provenance = '{"source":"seed","version":"v0.19.0"}',
```

#### 52-knowledge-atoms-bootstrap.cypher
**Add to ALL knowledge atom nodes**:
- AudienceSet, AudienceTrait
- CultureSet, CultureRef
- TabooSet, Taboo
- PatternSet, Pattern
- GEOQuerySet, GEOQuery, GEOAnswer

Each needs: `node_class`, `llm_context`, `provenance`

#### 40-page-block-instances.cypher
**Full restructure**:
- [ ] Remove slug from Page
- [ ] Add node_class to Page
- [ ] Add node_class, llm_context, provenance to all Block nodes
- [ ] Replace [:REPRESENTS] with [:ABOUT]

### 4.4 SIGNIFICANT FIXES: Files missing 2 properties

#### 50-page-native.cypher
**Add to ALL PageNative nodes (5 locales)**:
```cypher
pn.node_class = 'PageNative',
pn.provenance = '{"source":"seed","version":"v0.19.0"}',
```

### 4.5 MINOR FIXES: Files missing key/node_class

#### 34-prompts.cypher
**Add to PageInstruction, BlockInstruction, BlockRules**:
- [ ] Add `key` property
- [ ] Add `node_class` property
- [ ] Add `provenance` property

---

## Phase 5: Masterplan Updates (2 files)

### 5.1 Fix focus weight contradiction
**Decision needed**: 0.9 or 1.0?

| File | Current | Action |
|------|---------|--------|
| `2026-03-12-ontology-architecture-ascii.md` | weight: 1.0 | Update to 0.9? |
| `2026-03-12-MASTER-PLAN-v2.0-PAGE-ENTITY-SEO.md` | weight: 0.9 | Keep? |

### 5.2 Standardize role terminology
**Decision needed**: "related" or "reference"?

---

## Phase 6: Verification (Ralph Wiggum Loop)

### 6.1 After each phase, run:
```bash
# Schema validation
cargo run -- schema validate

# Generate artifacts
cargo run -- schema generate

# Run tests
cargo test

# TypeScript checks
pnpm type-check
pnpm test
```

### 6.2 Code review checkpoints
- [ ] After Phase 1 (ADRs)
- [ ] After Phase 2 (DX Rules)
- [ ] After Phase 3 (Schema YAML)
- [ ] After Phase 4 (Seed Cypher)
- [ ] After Phase 5 (Masterplans)

### 6.3 Database reseed
```bash
# Reset and reseed with fixed seeds
pnpm infra:reset
pnpm infra:seed
```

### 6.4 Final verification
```bash
# Verify all nodes have 8 standard properties
cargo run -- audit --target=all
```

---

## Execution Order

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  EXECUTION SEQUENCE                                                           ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║                                                                               ║
║  1. DECISION: Property naming (Option A or B)                                 ║
║  2. DECISION: Focus weight (0.9 or 1.0)                                       ║
║  3. DECISION: Role terminology (related or reference)                         ║
║                                                                               ║
║  4. Phase 1: ADR Updates (8 files)                                            ║
║     └── Code review checkpoint                                                ║
║                                                                               ║
║  5. Phase 2: DX Rules Updates (11 files)                                      ║
║     └── Code review checkpoint                                                ║
║                                                                               ║
║  6. Phase 3: Schema YAML Updates (6 files)                                    ║
║     └── cargo run -- schema validate                                          ║
║     └── cargo run -- schema generate                                          ║
║     └── Code review checkpoint                                                ║
║                                                                               ║
║  7. Phase 4: Seed Cypher Updates (9 files)                                    ║
║     └── Code review checkpoint                                                ║
║                                                                               ║
║  8. Phase 5: Masterplan Updates (2 files)                                     ║
║     └── Code review checkpoint                                                ║
║                                                                               ║
║  9. Database Reseed                                                           ║
║     └── pnpm infra:reset && pnpm infra:seed                                   ║
║                                                                               ║
║  10. Final Verification                                                       ║
║      └── cargo run -- audit --target=all                                      ║
║      └── All tests pass                                                       ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Commit Strategy (Granular)

Each logical change = 1 commit:

```
Phase 1:
- fix(adr): add BlockRules to ADR-025 instruction layer
- fix(adr): update generation formula in ADR-027
- fix(adr): mark REPRESENTS as superseded in ADR-028
- fix(adr): add Block properties to ADR-044
- fix(adr): rename schema to payload in ADR-030
- feat(adr): create ADR-047 block generation architecture
- fix(adr): update CHEAT-SHEET Page-Entity reference
- fix(adr): update instruction layer count in README

Phase 2:
- fix(dx): replace REPRESENTS with ABOUT in adr-quick-reference
- fix(dx): replace REPRESENTS with ABOUT in schema-standard
- fix(dx): remove deprecated HAS_TRAIT from cypher.md
- fix(dx): mark traits deprecated in novanet-decisions
- fix(dx): mark traits deprecated in schema-standard
- docs(dx): add Block architecture documentation
- docs(dx): add ABOUT arc documentation

Phase 3:
- fix(schema): fix block.yaml example property name
- fix(schema): rename BlockType structure to payload
- fix(schema): rename BlockInstruction payload to instruction
- fix(schema): replace REPRESENTS with ABOUT in page.yaml

Phase 4:
- fix(seed): replace REPRESENTS with ABOUT in page-block-instances
- fix(seed): replace REPRESENTS with ABOUT in page-block-qr-code
- fix(seed): remove slug from Page in page-block-instances
- fix(seed): add standard properties to blocknative-head-seo-meta
- fix(seed): add standard properties to knowledge-atoms-bootstrap
- fix(seed): add standard properties to page-native
- fix(seed): add standard properties to prompts

Phase 5:
- fix(docs): align focus weight in ASCII architecture
- fix(docs): standardize role terminology
```

---

## Files Changed Summary

| Category | Files | Changes |
|----------|-------|---------|
| ADRs | 8 | Updates + 1 new |
| DX Rules | 11 | Updates |
| Schema YAML | 6 | Property renames + fixes |
| Seed Cypher | 9 | Arc changes + property additions |
| Masterplans | 2 | Consistency fixes |
| **TOTAL** | **36 files** | |

---

## Risk Assessment

| Risk | Mitigation |
|------|------------|
| Schema validation fails | Run `schema validate` after each YAML change |
| Tests break | Run `cargo test` + `pnpm test` at each checkpoint |
| Database inconsistent | Full reseed after all changes |
| Missing changes | Ralph Wiggum loop until 0 issues |

---

## Rollback Plan

If critical issues found:
1. `git stash` current changes
2. `pnpm infra:reset` to restore database
3. Review and fix issues
4. Resume from last working commit

---

## Approval Required

Before execution, confirm:

1. **Property naming**: Option A (rename YAML) or Option B (update docs)?
2. **Focus weight**: 0.9 or 1.0?
3. **Role terminology**: "related" or "reference"?

**Ready to execute on your GO.**
