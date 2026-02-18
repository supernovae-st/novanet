# USES_CONCEPT → USES_ENTITY Migration Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete the v10.3 migration by renaming USES_CONCEPT to USES_ENTITY in all TypeScript code.

**Architecture:** The YAML sources and Cypher seeds already use USES_ENTITY. Only TypeScript code needs updating. This is a pure rename with no logic changes.

**Tech Stack:** TypeScript, Neo4j (already migrated), NovaNet Core, NovaNet Studio

---

## Task 1: Update Core Schema Definition

**Files:**
- Modify: `packages/core/src/schemas/relations.schema.ts`

**Step 1: Update RelationType enum (line 112)**

Replace:
```typescript
USES_CONCEPT: 'USES_CONCEPT',     // Page|Block → Concept (v7.0.0: unified)
```

With:
```typescript
USES_ENTITY: 'USES_ENTITY',       // Page|Block → Entity (v10.3: renamed from USES_CONCEPT)
```

**Step 2: Update section comment (lines 110-111)**

Replace:
```typescript
// CONCEPT USAGE (v7.0.0: unified USES_CONCEPT)
```

With:
```typescript
// ENTITY USAGE (v10.3: renamed from USES_CONCEPT)
```

**Step 3: Update historical comment (line 45)**

Replace:
```typescript
//   - PAGE_USES_CONCEPT + BLOCK_USES_CONCEPT → USES_CONCEPT (unified)
```

With:
```typescript
//   - PAGE_USES_CONCEPT + BLOCK_USES_CONCEPT → USES_CONCEPT (v7.0) → USES_ENTITY (v10.3)
```

**Step 4: Update inverse comment (line 180)**

Replace:
```typescript
USED_BY: 'USED_BY',                   // Concept → Page|Block (inverse of USES_CONCEPT)
```

With:
```typescript
USED_BY: 'USED_BY',                   // Entity → Page|Block (inverse of USES_ENTITY)
```

**Step 5: Update RelationDefinition object key (lines 511-514)**

Replace:
```typescript
// CONCEPT USAGE (v7.0.0: unified USES_CONCEPT)

[RelationType.USES_CONCEPT]: {
  type: RelationType.USES_CONCEPT,
```

With:
```typescript
// ENTITY USAGE (v10.3: renamed from USES_CONCEPT)

[RelationType.USES_ENTITY]: {
  type: RelationType.USES_ENTITY,
```

**Step 6: Update RelationDefinition 'to' field and description**

Change `to: 'Concept'` → `to: 'Entity'`
Change description reference from USES_CONCEPT to USES_ENTITY (line 728)

**Verification:** `pnpm type-check --filter=@novanet/core` - expect errors in dependent files

---

## Task 2: Update Core Schema Tests

**Files:**
- Modify: `packages/core/src/__tests__/schemas.test.ts`

**Step 1: Update enum assertion (line 185)**

Replace:
```typescript
expect(RelationType.USES_CONCEPT).toBe('USES_CONCEPT');
```

With:
```typescript
expect(RelationType.USES_ENTITY).toBe('USES_ENTITY');
```

**Step 2: Update variant assertion (lines 247-248)**

Replace:
```typescript
// USES_CONCEPT is active (v7.9.0: USED_SEO_KEYWORD/USED_GEO_SEED removed)
expect(usesVariants).toContain('USES_CONCEPT');
```

With:
```typescript
// USES_ENTITY is active (v10.3: renamed from USES_CONCEPT)
expect(usesVariants).toContain('USES_ENTITY');
```

**Verification:** `pnpm test --filter=@novanet/core` - tests should pass

---

## Task 3: Update Studio Relationship Types Config

**Files:**
- Modify: `apps/studio/src/config/relationshipTypes.ts`

**Step 1: Update semantic array (line 68)**

Replace `'USES_CONCEPT'` with `'USES_ENTITY'` in the array.

**Step 2: Update object key and properties (lines 297-302)**

Replace:
```typescript
USES_CONCEPT: {
  type: 'USES_CONCEPT',
  label: 'Uses Concept',
```

With:
```typescript
USES_ENTITY: {
  type: 'USES_ENTITY',
  label: 'Uses Entity',
```

**Verification:** TypeScript compiles

---

## Task 4: Update Studio Filter Adapter

**Files:**
- Modify: `apps/studio/src/lib/filterAdapter.ts`

**Step 1: Update RELATION_TO_FILTER_KEY mapping (line 62)**

Replace:
```typescript
USES_CONCEPT: 'concept',
```

With:
```typescript
USES_ENTITY: 'entity',
```

**Step 2: Update RELATION_TO_NODE_TYPE mapping (line 90)**

Replace:
```typescript
USES_CONCEPT: 'Concept',
```

With:
```typescript
USES_ENTITY: 'Entity',
```

**Step 3: Update spreading activation check (line 429)**

Replace:
```typescript
if (include.relation === 'USES_CONCEPT' && include.depth && include.depth > 1) {
```

With:
```typescript
if (include.relation === 'USES_ENTITY' && include.depth && include.depth > 1) {
```

Also update `Concept` → `Entity` in the SEMANTIC_LINK query if present.

**Verification:** TypeScript compiles

---

## Task 5: Update Studio Edge System

**Files:**
- Modify: `apps/studio/src/components/graph/edges/system/types.ts`
- Modify: `apps/studio/src/components/graph/edges/system/arcFamilyPalettes.ts`
- Modify: `apps/studio/src/components/graph/edges/system/themes.ts`

**Step 1: Update types.ts type union (line 44)**

Replace `'USES_CONCEPT'` with `'USES_ENTITY'`

**Step 2: Update types.ts comment (line 19)**

Replace:
```typescript
| 'semantic'      // USES_CONCEPT, SEMANTIC
```

With:
```typescript
| 'semantic'      // USES_ENTITY, SEMANTIC
```

**Step 3: Update arcFamilyPalettes.ts mapping (line 143)**

Replace:
```typescript
USES_CONCEPT: 'semantic',
```

With:
```typescript
USES_ENTITY: 'semantic',
```

**Step 4: Update pattern matching if exists (line ~185)**

If there's a pattern `/CONCEPT/`, update to `/ENTITY/`

**Step 5: Update themes.ts mapping (line 219)**

Replace:
```typescript
USES_CONCEPT: {
```

With:
```typescript
USES_ENTITY: {
```

**Verification:** TypeScript compiles

---

## Task 6: Update Studio Tests

**Files:**
- Modify: `apps/studio/src/lib/__tests__/neo4j.test.ts`
- Modify: `apps/studio/src/hooks/__tests__/useHoverHighlight.test.ts`

**Step 1: Update neo4j.test.ts (lines 178, 188)**

Replace all `type: 'USES_CONCEPT'` with `type: 'USES_ENTITY'`

**Step 2: Update useHoverHighlight.test.ts (line 19)**

Replace:
```typescript
{ id: 'e2', source: 'node-a', target: 'node-c', type: 'USES_CONCEPT' },
```

With:
```typescript
{ id: 'e2', source: 'node-a', target: 'node-c', type: 'USES_ENTITY' },
```

**Verification:** `pnpm test --filter=@novanet/studio`

---

## Task 7: Update API Documentation

**Files:**
- Modify: `apps/studio/src/app/api/chat/route.ts`

**Step 1: Update documentation string (line 92)**

Replace:
```typescript
**Semantic**: USES_CONCEPT, SEMANTIC_LINK, SUBTOPIC_OF, LINKS_TO, SATISFIES_INTENT
```

With:
```typescript
**Semantic**: USES_ENTITY, SEMANTIC_LINK, SUBTOPIC_OF, LINKS_TO, SATISFIES_INTENT
```

---

## Task 8: Final Verification

**Step 1: Type check all packages**

```bash
pnpm type-check
```

Expected: PASS

**Step 2: Run all tests**

```bash
pnpm test
```

Expected: ALL PASS

**Step 3: Build all packages**

```bash
pnpm build
```

Expected: SUCCESS

**Step 4: Verify no remaining USES_CONCEPT in code**

```bash
grep -r "USES_CONCEPT" packages/core/src apps/studio/src --include="*.ts" --include="*.tsx" | grep -v node_modules | grep -v ".test." | grep -v "__tests__"
```

Expected: No results (only historical comments in documentation)

**Step 5: Commit**

```bash
git add -A
git commit -m "refactor(schema): rename USES_CONCEPT → USES_ENTITY (v10.3 cleanup)

Complete the v10.3 Entity-Centric Architecture migration by updating
all TypeScript code to use USES_ENTITY instead of USES_CONCEPT.

YAML sources and Cypher seeds were already migrated. This commit
updates:
- packages/core: RelationType enum, schema tests
- apps/studio: filterAdapter, relationshipTypes, edge system, tests

No backward compatibility needed - Neo4j database already uses USES_ENTITY.

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Risk Assessment

**Overall Risk: LOW**

- No database migration needed (already using USES_ENTITY)
- Pure TypeScript rename
- Type safety catches missing renames
- Test coverage verifies correctness

**Estimated time:** 20-30 minutes
