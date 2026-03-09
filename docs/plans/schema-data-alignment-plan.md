# Schema/Data Alignment Plan v0.17.3

**Date**: 2026-03-09
**Goal**: Align ALL node schemas with seed data - no missing/extra properties

## Standard Properties (ALL nodes must have these 6)

```yaml
standard_properties:
  key:
    type: string
    required: true
  display_name:
    type: string
    required: true
  description:
    type: string
    required: true
  created_at:
    type: datetime
    required: true
  updated_at:
    type: datetime
    required: true
  llm_context:
    type: string
    required: true
```

## Current State Analysis

### EntityNative

**Schema has (to REMOVE from schema)**:
- `definition` - NOT in seed data → DELETE from schema
- `purpose` - NOT in seed data → DELETE from schema
- `benefits` - NOT in seed data → DELETE from schema
- `usage_examples` - NOT in seed data → DELETE from schema

**Seed has (to REMOVE from seed)**:
- `workflow_id = 'bootstrap'` - User said DELETE this → Remove from seed

**Seed has (KEEP - matches schema)**:
- `key`, `display_name`, `description`, `llm_context`, `created_at`, `updated_at` ✓
- `denomination_forms` (JSON array of objects) ✓
- `status` ✓

### BlockNative

**Schema**: Uses `content` property ✓
**Seed**: Uses `content` property ✓
**Standard properties**: All present ✓

### PageNative

**Schema**: Has `llm_context` in standard_properties ✓
**Seed**: Missing `llm_context` → ADD to seed
**Standard properties**: Need to verify `llm_context` in seed

### Page

**Schema**: Has all 6 standard properties ✓
**Seed**: Has all 6 standard properties ✓

### Block

**Schema**: Has all 6 standard properties ✓
**Seed**: Has all 6 standard properties ✓

## Execution Plan

### Phase 1: Schema Updates (YAML)

#### 1.1 EntityNative Schema
File: `packages/core/models/node-classes/org/semantic/entity-native.yaml`

**REMOVE from properties section**:
- `definition` (lines ~105-108)
- `purpose` (lines ~109-112)
- `benefits` (lines ~113-116)
- `usage_examples` (lines ~117-120)

### Phase 2: Seed Updates (Cypher)

#### 2.1 EntityNative Seed
File: `private-data/seed/11-entity-native-bootstrap.cypher`

**REMOVE from ALL EntityNative MERGE statements**:
```cypher
-- REMOVE THIS LINE:
en.workflow_id = 'bootstrap',
```

#### 2.2 PageNative Seed
File: `private-data/seed/50-page-native.cypher`

**ADD to ALL PageNative MERGE statements**:
```cypher
pn.llm_context = 'USE: for assembled page display. TRIGGERS: page native, locale. NOT: for structure.',
```

### Phase 3: Migration (for existing data)

#### 3.1 Create Migration 066
File: `packages/db/migrations/066-remove-entitynative-workflow-id.cypher`

```cypher
// ═══════════════════════════════════════════════════════════════════════════════
// 066-remove-entitynative-workflow-id.cypher
// Cleanup: Remove workflow_id property from EntityNative nodes
// ═══════════════════════════════════════════════════════════════════════════════

MATCH (en:EntityNative)
WHERE en.workflow_id IS NOT NULL
REMOVE en.workflow_id
RETURN count(en) AS nodes_cleaned;
```

### Phase 4: Regenerate & Verify

```bash
# 1. Regenerate Cypher from YAML
cd tools/novanet && cargo run -- schema generate

# 2. Validate schema
cargo run -- schema validate

# 3. Reset and reseed
pnpm infra:reset

# 4. Run audit to verify CSR 100%
# (via MCP or manually)
```

### Phase 5: Final Verification

Run this Cypher to verify no node has missing standard properties:

```cypher
// Check EntityNative has required properties
MATCH (en:EntityNative)
WHERE en.key IS NULL
   OR en.display_name IS NULL
   OR en.description IS NULL
   OR en.llm_context IS NULL
   OR en.created_at IS NULL
   OR en.updated_at IS NULL
RETURN 'EntityNative' AS label, count(en) AS missing_props;

// Check PageNative has required properties
MATCH (pn:PageNative)
WHERE pn.key IS NULL
   OR pn.display_name IS NULL
   OR pn.description IS NULL
   OR pn.llm_context IS NULL
   OR pn.created_at IS NULL
   OR pn.updated_at IS NULL
RETURN 'PageNative' AS label, count(pn) AS missing_props;

// Check BlockNative has required properties
MATCH (bn:BlockNative)
WHERE bn.key IS NULL
   OR bn.display_name IS NULL
   OR bn.description IS NULL
   OR bn.created_at IS NULL
   OR bn.updated_at IS NULL
RETURN 'BlockNative' AS label, count(bn) AS missing_props;

// Check for extra properties (workflow_id should not exist)
MATCH (en:EntityNative)
WHERE en.workflow_id IS NOT NULL
RETURN 'EntityNative with workflow_id' AS issue, count(en) AS count;
```

## Summary of Changes

| File | Action | Details |
|------|--------|---------|
| `entity-native.yaml` | REMOVE | definition, purpose, benefits, usage_examples |
| `11-entity-native-bootstrap.cypher` | REMOVE | workflow_id from all nodes |
| `50-page-native.cypher` | ADD | llm_context to all PageNative nodes |
| `066-*.cypher` | CREATE | Migration to remove workflow_id from existing data |

## Expected Result

- All nodes have exactly 6 standard properties
- No extra properties that aren't in schema
- CSR = 100% (no validation errors)
- Database can be reset and reseeded cleanly
