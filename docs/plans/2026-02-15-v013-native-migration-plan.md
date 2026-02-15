# v0.13 *Native Migration Plan

**Date**: 2026-02-15
**Version**: v0.13.0 "Native Pattern"
**Author**: Claude Code + Thibaut
**Status**: IN PROGRESS

## Executive Summary

The v0.13 *Native migration (ADR-029 + ADR-030) introduces a unified naming pattern:
- `*Content` + `*Generated` → `*Native` (single node per locale behavior)
- `HAS_CONTENT` + `HAS_GENERATED` → `HAS_NATIVE` (unified ownership arc)
- Slug ownership moved from EntityNative to PageNative (ADR-030)

### Audit Results (10 Snipers)

| Area | Files Affected | Occurrences | Priority |
|------|----------------|-------------|----------|
| TypeScript types/schemas | 55 files | ~380 | P0 - Critical |
| Arc references (code) | 40+ files | ~481 | P0 - Critical |
| Seed data (Cypher) | 15 files | ~200 | P1 - High |
| Documentation | 8 files | ~100 | P1 - High |
| Views YAML | 5 files | ~30 | P2 - Medium |
| Studio components | Clean | 0 | Done |
| Zod schemas | Clean | 0 | Done |
| Test assertions | Clean | 0 | Done |

---

## Phase 1: TypeScript Core Types (P0)

**Goal**: Update all TypeScript type definitions and schemas.

### Batch 1.1: Node Type Definitions

**Files to update:**
- `packages/core/src/types/nodes.ts`
- `packages/core/src/types/index.ts`
- `packages/core/src/types/generated.ts`

**Changes:**
```typescript
// OLD
export type LocalizedNodeType =
  | 'EntityContent'
  | 'ProjectContent';

export type GeneratedNodeType =
  | 'PageGenerated'
  | 'BlockGenerated';

// NEW (v0.13 ADR-029)
export type NativeNodeType =
  | 'EntityNative'
  | 'ProjectNative'
  | 'PageNative'
  | 'BlockNative';
```

### Batch 1.2: Arc Type Definitions

**Files to update:**
- `packages/core/src/types/arcs.ts`
- `packages/core/src/types/relationships.ts`

**Changes:**
```typescript
// OLD
export type ContentArc = 'HAS_CONTENT' | 'CONTENT_OF';
export type GeneratedArc = 'HAS_GENERATED' | 'GENERATED_FOR';

// NEW (v0.13 ADR-029)
export type NativeArc = 'HAS_NATIVE' | 'NATIVE_OF';
```

### Batch 1.3: CypherGenerator Updates

**Files to update:**
- `packages/core/src/filters/CypherGenerator.ts`

**Changes:**
- Replace all `HAS_CONTENT` → `HAS_NATIVE`
- Replace all `CONTENT_OF` → `NATIVE_OF`
- Replace all `HAS_GENERATED` → `HAS_NATIVE`
- Replace all `GENERATED_FOR` → `NATIVE_OF`
- Update `EntityContent` → `EntityNative` in patterns
- Update `PageGenerated` → `PageNative` in patterns

### Batch 1.4: FilterAdapter Updates

**Files to update:**
- `packages/core/src/filters/FilterAdapter.ts`
- `packages/core/src/filters/index.ts`

---

## Phase 2: Seed Data Migration (P1)

**Goal**: Update all Cypher seed files with new node labels and relationships.

### Batch 2.1: Schema Seed Files

**Files to update:**
- `packages/db/seed/00-schema.cypher`
- `packages/db/seed/01-constraints.cypher`

**Changes:**
```cypher
// OLD
CREATE CONSTRAINT entity_content_key IF NOT EXISTS
FOR (n:EntityContent) REQUIRE n.key IS UNIQUE;

// NEW
CREATE CONSTRAINT entity_native_key IF NOT EXISTS
FOR (n:EntityNative) REQUIRE n.key IS UNIQUE;
```

### Batch 2.2: Data Seed Files

**Files to update:**
- `packages/db/seed/10-entities.cypher`
- `packages/db/seed/11-entity-content.cypher` → rename to `11-entity-native.cypher`
- `packages/db/seed/20-pages.cypher`
- `packages/db/seed/21-page-generated.cypher` → rename to `21-page-native.cypher`
- `packages/db/seed/30-blocks.cypher`
- `packages/db/seed/31-block-generated.cypher` → rename to `31-block-native.cypher`

**Changes:**
```cypher
// OLD
CREATE (ec:EntityContent {
  key: 'entity:qr-generator@fr-FR',
  title: '...'
})
WITH ec
MATCH (e:Entity {key: 'qr-generator'})
CREATE (e)-[:HAS_CONTENT]->(ec);

// NEW (v0.13)
CREATE (en:EntityNative {
  key: 'entity:qr-generator@fr-FR',
  title: '...'
})
WITH en
MATCH (e:Entity {key: 'qr-generator'})
CREATE (e)-[:HAS_NATIVE]->(en);
```

### Batch 2.3: Relationship Seed Files

**Files to update:**
- All files with `:HAS_CONTENT`, `:CONTENT_OF`, `:HAS_GENERATED`, `:GENERATED_FOR`

**Search pattern:**
```bash
grep -r "HAS_CONTENT\|CONTENT_OF\|HAS_GENERATED\|GENERATED_FOR" packages/db/seed/
```

---

## Phase 3: Documentation Updates (P1)

**Goal**: Update all CLAUDE.md files and ADR documentation.

### Batch 3.1: Root CLAUDE.md

**File**: `/CLAUDE.md`

**Updates needed:**
- Architecture diagram
- Node count: 61 nodes (unchanged, but names change)
- Arc count: 171 → 169
- Nomenclature section

### Batch 3.2: Core Package CLAUDE.md

**File**: `packages/core/CLAUDE.md`

**Updates needed:**
- Useful Cypher Queries section (update relationship names)
- Nomenclature section
- File Structure section (node-kinds paths)

### Batch 3.3: ADR Documentation

**File**: `.claude/rules/novanet-decisions.md`

**Updates needed:**
- Quick Reference table (currently shows old examples)
- ADR-029 section (verify complete)
- ADR-030 section (verify complete)
- Migration commands section

### Batch 3.4: Terminology Documentation

**File**: `.claude/rules/novanet-terminology.md`

**Updates needed:**
- Core Vocabulary section
- Node Naming Convention section
- Deprecated Terms table (add EntityContent, etc.)
- Summary section

---

## Phase 4: Views and Cypher Queries (P2)

**Goal**: Update all view definitions with new node/arc names.

### Batch 4.1: Views Registry

**File**: `packages/core/models/views/_registry.yaml`

**Updates needed:**
- Update Cypher patterns in all contextual views
- Update applicable_types arrays

### Batch 4.2: Individual View Files

**Files to update:**
- `packages/core/models/views/composition.yaml`
- `packages/core/models/views/generation.yaml`
- `packages/core/models/views/localization.yaml`
- `packages/core/models/views/knowledge.yaml`

**Example change:**
```yaml
# OLD
cypher: |
  MATCH (e:Entity)-[:HAS_CONTENT]->(ec:EntityContent)
  WHERE ec.key STARTS WITH 'entity:' + $entityKey + '@'
  RETURN e, ec

# NEW
cypher: |
  MATCH (e:Entity)-[:HAS_NATIVE]->(en:EntityNative)
  WHERE en.key STARTS WITH 'entity:' + $entityKey + '@'
  RETURN e, en
```

### Batch 4.3: Layout Configuration

**File**: `apps/studio/src/lib/graph/layout.ts`

**Updates needed:**
- Remove stale `GEO*` entries if present
- Update any hardcoded node type references

---

## Phase 5: Studio API Routes (P2)

**Goal**: Update API routes that reference old node/arc names.

### Batch 5.1: Graph API Routes

**Files to check:**
- `apps/studio/src/app/api/graph/nodes-by-types/route.ts`
- `apps/studio/src/app/api/graph/search/route.ts`
- `apps/studio/src/app/api/views/[id]/query/route.ts`

### Batch 5.2: Filter API Routes

**Files to check:**
- `apps/studio/src/app/api/filters/route.ts`

---

## Phase 6: Final Validation (P0)

**Goal**: Run all tests, regenerate artifacts, validate schema.

### Batch 6.1: Schema Regeneration

```bash
cd tools/novanet
cargo run -- schema generate
cargo run -- schema validate --strict
```

### Batch 6.2: Test Suites

```bash
# Rust tests
cargo nextest run

# TypeScript tests
pnpm test

# Type checking
pnpm type-check

# Linting
pnpm lint
```

### Batch 6.3: Neo4j Migration

```bash
# Reset database with new seeds
pnpm infra:reset
pnpm infra:seed

# Verify node counts
docker exec -it novanet-neo4j cypher-shell -u neo4j -p novanetpassword \
  "MATCH (n:EntityNative) RETURN count(n)"
```

---

## Migration Checklist

### Phase 1: TypeScript Core Types
- [ ] 1.1 Node type definitions updated
- [ ] 1.2 Arc type definitions updated
- [ ] 1.3 CypherGenerator updated
- [ ] 1.4 FilterAdapter updated

### Phase 2: Seed Data
- [ ] 2.1 Schema seed files updated
- [ ] 2.2 Data seed files renamed and updated
- [ ] 2.3 Relationship seed files updated

### Phase 3: Documentation
- [ ] 3.1 Root CLAUDE.md updated
- [ ] 3.2 Core package CLAUDE.md updated
- [ ] 3.3 ADR documentation updated
- [ ] 3.4 Terminology documentation updated

### Phase 4: Views and Queries
- [ ] 4.1 Views registry updated
- [ ] 4.2 Individual view files updated
- [ ] 4.3 Layout configuration updated

### Phase 5: Studio API
- [ ] 5.1 Graph API routes updated
- [ ] 5.2 Filter API routes updated

### Phase 6: Validation
- [ ] 6.1 Schema regenerated and validated
- [ ] 6.2 All test suites pass
- [ ] 6.3 Neo4j migration successful

---

## Backward Compatibility

### Deprecation Strategy

1. **TypeScript**: Add type aliases for backward compatibility
   ```typescript
   /** @deprecated Use EntityNative */
   export type EntityContent = EntityNative;
   ```

2. **Cypher**: No backward compat - clean break

3. **Documentation**: Mark old terms in Deprecated Terms table

### Removal Timeline

- v0.13.0: Introduce *Native pattern, add deprecation warnings
- v0.14.0: Remove deprecated type aliases
- v1.0.0: Final cleanup, no legacy references

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Type errors in Studio | High | Run type-check after each batch |
| Seed data breaks Neo4j | High | Test on fresh database first |
| Views return no data | Medium | Test each view individually |
| Documentation drift | Low | Update docs alongside code |

---

## Estimated Effort

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| Phase 1 | 4 batches | 2-3 hours |
| Phase 2 | 3 batches | 1-2 hours |
| Phase 3 | 4 batches | 1 hour |
| Phase 4 | 3 batches | 1 hour |
| Phase 5 | 2 batches | 30 minutes |
| Phase 6 | 3 batches | 1 hour |
| **Total** | **19 batches** | **6-8 hours** |

---

## Next Steps

1. **Begin Phase 1**: Update TypeScript core types
2. **Commit after each batch** with clear messages
3. **Run tests** after each phase
4. **Update this plan** as batches complete

---

## References

- ADR-029: *Native Pattern (`novanet-decisions.md`)
- ADR-030: Slug Ownership (`novanet-decisions.md`)
- v0.13 Design: `docs/plans/2026-02-14-schema-completion-v0125-plan.md`
