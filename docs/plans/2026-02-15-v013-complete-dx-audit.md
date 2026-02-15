# v0.13.0 Complete DX Audit Plan

## Objective

Ultra-thorough verification that v0.13.0 *Native Pattern is synchronized across ALL layers:
- Schema (YAML nodes, arcs, taxonomy)
- Generated artifacts (Cypher, TypeScript, Mermaid, Rust)
- Documentation (CLAUDE.md, README.md, ADRs, skills)
- Tests (Rust, TypeScript, snapshots)
- Neo4j (labels, constraints, seeds, live data)

## Verification Matrix

### 1. YAML Schema Layer

| Check | File Pattern | Expected |
|-------|--------------|----------|
| *Native nodes exist | `node-classes/org/*/*.yaml` | EntityNative, ProjectNative, PageNative, BlockNative |
| Old names absent | `node-classes/**/*.yaml` | NO EntityContent, ProjectContent, PageGenerated, BlockGenerated |
| HAS_NATIVE arc | `arc-classes/ownership/has-native.yaml` | Unified arc with locale property |
| NATIVE_OF arc | `arc-classes/ownership/native-of.yaml` | Inverse arc |
| Old arcs absent | `arc-classes/**/*.yaml` | NO HAS_CONTENT, HAS_GENERATED, CONTENT_OF, GENERATED_FOR |
| Taxonomy v0.13.0 | `taxonomy.yaml` | Version header, trait definitions |
| Views updated | `views/*.yaml` | Use *Native labels |

### 2. Generated Artifacts

| Artifact | Location | Check |
|----------|----------|-------|
| Cypher seeds | `packages/db/seed/*.cypher` | *Native labels, HAS_NATIVE arcs |
| Cypher constraints | `packages/db/seed/00-constraints.cypher` | *Native constraints |
| TypeScript types | `packages/core/src/types/*.ts` | *Native type exports |
| TypeScript filters | `packages/core/src/filters/*.ts` | *Native patterns |
| Mermaid docs | `packages/core/models/docs/*.md` | *Native in diagrams |
| Rust icons | `tools/novanet/src/tui/icons.rs` | Generated constants |

### 3. Documentation Layer

| Document | Location | Check |
|----------|----------|-------|
| Root CLAUDE.md | `CLAUDE.md` | v0.13.0 version, *Native terminology |
| Core CLAUDE.md | `packages/core/CLAUDE.md` | *Native pattern documented |
| Studio CLAUDE.md | `apps/studio/CLAUDE.md` | *Native in schema section |
| Novanet CLAUDE.md | `tools/novanet/CLAUDE.md` | v0.13.0 status, *Native |
| Terminology | `.claude/rules/novanet-terminology.md` | Deprecated table correct |
| Decisions | `.claude/rules/novanet-decisions.md` | ADR-029, ADR-030 complete |
| README.md | `README.md` | Version badge, *Native mentions |
| CHANGELOG.md | `CHANGELOG.md` | v0.13.0 entry |
| ROADMAP.md | `ROADMAP.md` | v0.13.0 milestone |

### 4. Skills & Commands

| Skill | Location | Check |
|-------|----------|-------|
| schema-add-node | `.claude/commands/schema-add-node.md` | *Native option |
| schema-edit-node | `.claude/commands/schema-edit-node.md` | *Native handling |
| novanet-arch | `.claude/commands/novanet-arch.md` | *Native in architecture |
| novanet-sync | `.claude/commands/novanet-sync.md` | *Native validation |

### 5. Test Coverage

| Test Suite | Location | Check |
|------------|----------|-------|
| Rust unit tests | `tools/novanet/src/**/*.rs` | *Native in test names |
| Rust snapshots | `tools/novanet/src/**/*.snap` | *Native in snapshots |
| TypeScript core | `packages/core/src/__tests__/*.ts` | *Native type tests |
| TypeScript filters | `packages/core/src/filters/__tests__/*.ts` | *Native filter tests |
| Studio tests | `apps/studio/src/__tests__/*.ts` | *Native handling |

### 6. Neo4j Live Data

| Check | Query | Expected |
|-------|-------|----------|
| *Native labels exist | `MATCH (n) WHERE n:EntityNative OR n:PageNative RETURN count(n)` | > 0 |
| Old labels absent | `MATCH (n) WHERE n:EntityContent OR n:PageGenerated RETURN count(n)` | 0 |
| HAS_NATIVE arcs | `MATCH ()-[r:HAS_NATIVE]->() RETURN count(r)` | > 0 |
| Old arcs absent | `MATCH ()-[r:HAS_CONTENT]->() RETURN count(r)` | 0 |
| Constraints exist | `SHOW CONSTRAINTS` | *Native constraints |

## Agent Distribution

| # | Agent | Focus | Files |
|---|-------|-------|-------|
| 1 | YAML-SCHEMA | Node/Arc YAML files | `models/node-classes/`, `models/arc-classes/` |
| 2 | CYPHER-GEN | Generated Cypher | `packages/db/seed/` |
| 3 | TS-TYPES | TypeScript types/filters | `packages/core/src/` |
| 4 | RUST-CODE | Rust generators/TUI | `tools/novanet/src/` |
| 5 | CLAUDE-MD | All CLAUDE.md files | `**/CLAUDE.md` |
| 6 | RULES-ADR | Rules and ADRs | `.claude/rules/` |
| 7 | SKILLS-CMD | Skills and commands | `.claude/commands/`, `.claude/skills/` |
| 8 | README-DOCS | README, CHANGELOG, ROADMAP | Root docs |
| 9 | TESTS | All test files | `**/__tests__/`, `**/*.test.ts`, `**/*_test.rs` |
| 10 | NEO4J | Live database state | Neo4j queries |

## Success Criteria

- [ ] 0 references to deprecated EntityContent/ProjectContent/PageGenerated/BlockGenerated
- [ ] 0 references to deprecated HAS_CONTENT/HAS_GENERATED/CONTENT_OF/GENERATED_FOR
- [ ] All CLAUDE.md files show v0.13.0 version
- [ ] All tests pass (Rust + TypeScript)
- [ ] Schema validation: 0 errors, 0 warnings
- [ ] Blueprint validation: passed = true
- [ ] Neo4j schema matches YAML definitions
