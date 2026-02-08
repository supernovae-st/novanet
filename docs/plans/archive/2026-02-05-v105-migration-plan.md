# NovaNet v10.5 Migration Plan: 3-Realm Architecture

**Date:** 2026-02-05
**Version:** 10.5.0
**Status:** Ready for Implementation

## Executive Summary

This plan completes the v10.5 3-Realm Architecture migration. The YAML source of truth and Rust generators are fully updated. Remaining work is propagating changes to Neo4j seeds, TypeScript types, and Studio UI.

**Key Pattern: Company Project**
```
Organization ──[:HAS_COMPANY_PROJECT]──> Project (org-wide entities)
             ──[:HAS_PROJECT]──────────> Project[] (product projects)
```

Organization does NOT contain Entity directly. Entities live in Project realm (either company project for org-wide, or product projects for product-specific).

---

## Phase 1: Neo4j Schema (Critical)

### 1.1 Add Organization Constraint

**File:** `packages/db/seed/00-constraints.cypher`

```cypher
// Add after existing constraints
CREATE CONSTRAINT organization_key IF NOT EXISTS
FOR (o:Organization) REQUIRE o.key IS UNIQUE;
```

**Verification:**
```bash
# After seeding
MATCH (o:Organization) RETURN o.key, count(*);
```

---

### 1.2 Regenerate Arc-Kinds Seed

**Issue:** HAS_COMPANY_PROJECT, HAS_PROJECT, BELONGS_TO_ORG missing from 02-arc-kinds.cypher

**Action:**
```bash
cargo run -- schema generate
```

**Verify these arcs appear in 02-arc-kinds.cypher:**
- HAS_COMPANY_PROJECT (Organization → Project, 1:1)
- HAS_PROJECT (Organization → Project, 1:N)
- BELONGS_TO_ORG (Project → Organization, N:1)

---

### 1.3 Create Organization Seed File

**File:** `packages/db/seed/03-organization.cypher` (NEW)

```cypher
// =============================================================================
// ORGANIZATION DATA (v10.5)
// =============================================================================
// Creates Organization nodes and links them to Projects via:
// - HAS_COMPANY_PROJECT (1:1 - org branding project)
// - HAS_PROJECT (1:N - product projects)

// -----------------------------------------------------------------------------
// SuperNovae Studio Organization
// -----------------------------------------------------------------------------
CREATE (org:Organization {
  key: 'org-supernovae-studio',
  display_name: 'SuperNovae Studio',
  description: 'AI-powered native content generation for global markets.',
  legal_name: 'SuperNovae Studio SAS',
  website_url: 'https://supernovae.studio',
  industry: 'AI / SaaS',
  llm_context: 'SuperNovae Studio builds AI tools for native content generation. Our flagship product is QR Code AI. Company values: quality over speed, native generation over translation.',
  created_at: datetime(),
  updated_at: datetime()
});

// -----------------------------------------------------------------------------
// Company Project (org-wide branding)
// -----------------------------------------------------------------------------
CREATE (companyProject:Project {
  key: 'project-supernovae-company',
  display_name: 'SuperNovae Company',
  description: 'Organization-level branding and shared entities.',
  llm_context: 'This project holds org-wide Entity nodes: brand values, cross-product concepts, company identity. Used for spreading activation across all product projects.',
  created_at: datetime(),
  updated_at: datetime()
});

// Link Organization to Company Project
MATCH (org:Organization {key: 'org-supernovae-studio'})
MATCH (cp:Project {key: 'project-supernovae-company'})
CREATE (org)-[:HAS_COMPANY_PROJECT]->(cp);

// -----------------------------------------------------------------------------
// Link existing QR Code AI project to Organization
// -----------------------------------------------------------------------------
MATCH (org:Organization {key: 'org-supernovae-studio'})
MATCH (p:Project {key: 'project-qrcode-ai'})
CREATE (org)-[:HAS_PROJECT]->(p)
CREATE (p)-[:BELONGS_TO_ORG]->(org);

// Link company project to organization too
MATCH (org:Organization {key: 'org-supernovae-studio'})
MATCH (cp:Project {key: 'project-supernovae-company'})
CREATE (cp)-[:BELONGS_TO_ORG]->(org);

// -----------------------------------------------------------------------------
// Company Project Entity (org-wide brand values)
// -----------------------------------------------------------------------------
MATCH (cp:Project {key: 'project-supernovae-company'})
CREATE (cp)-[:HAS_ENTITY]->(e:Entity {
  key: 'entity-brand-values',
  display_name: 'Brand Values',
  description: 'Core values that guide all SuperNovae products.',
  llm_context: 'Quality over speed. Native generation over translation. Research before assumption. Clarity over complexity.',
  created_at: datetime(),
  updated_at: datetime()
});

// Create EntityL10n for each supported locale
MATCH (e:Entity {key: 'entity-brand-values'})
MATCH (l:Locale {key: 'en-US'})
CREATE (e)-[:HAS_L10N]->(el:EntityL10n {
  key: 'entity-brand-values-en-us',
  display_name: 'Brand Values',
  content: 'Quality over speed. Native generation over translation.',
  llm_context: 'English expression of SuperNovae brand values.',
  created_at: datetime(),
  updated_at: datetime()
})-[:FOR_LOCALE]->(l);

MATCH (e:Entity {key: 'entity-brand-values'})
MATCH (l:Locale {key: 'fr-FR'})
CREATE (e)-[:HAS_L10N]->(el:EntityL10n {
  key: 'entity-brand-values-fr-fr',
  display_name: 'Valeurs de marque',
  content: 'La qualité avant la vitesse. La génération native avant la traduction.',
  llm_context: 'Expression française des valeurs SuperNovae.',
  created_at: datetime(),
  updated_at: datetime()
})-[:FOR_LOCALE]->(l);
```

**Seed order update in pnpm scripts:**
```json
"infra:seed": "... 03-organization.cypher before 04-project-qrcode-ai.cypher"
```

---

### 1.4 Update Project Seed (QR Code AI)

**File:** `packages/db/seed/04-project-qrcode-ai.cypher`

**Change:** Remove CREATE, use MATCH (project now created in 03-organization.cypher context)

```cypher
// Before: CREATE (p:Project {...})
// After: Ensure project exists and add product-specific data
MATCH (p:Project {key: 'project-qrcode-ai'})
// ... rest of seed continues with MATCH instead of CREATE for the project node
```

Actually, keep CREATE but ensure 03-organization runs first and links after.

---

## Phase 2: TypeScript Types (High Priority)

### 2.1 Add Organization Arc Types

**File:** `packages/core/src/schemas/relations.schema.ts`

```typescript
// Add to RelationType enum
export type RelationType =
  | 'HAS_COMPANY_PROJECT'  // NEW
  | 'HAS_PROJECT'          // NEW
  | 'BELONGS_TO_ORG'       // NEW
  | ... existing types;

// Add to RELATION_REGISTRY
export const RELATION_REGISTRY: Record<RelationType, RelationMeta> = {
  HAS_COMPANY_PROJECT: {
    family: 'ownership',
    scope: 'cross_realm',
    source: ['Organization'],
    target: ['Project'],
    cardinality: 'one_to_one',
  },
  HAS_PROJECT: {
    family: 'ownership',
    scope: 'cross_realm',
    source: ['Organization'],
    target: ['Project'],
    cardinality: 'one_to_many',
  },
  BELONGS_TO_ORG: {
    family: 'ownership',
    scope: 'cross_realm',
    source: ['Project'],
    target: ['Organization'],
    cardinality: 'many_to_one',
  },
  // ... existing
};
```

---

### 2.2 Add Realm-Based Filtering

**File:** `packages/core/src/filters/types.ts`

```typescript
export interface FilterCriteria {
  // Existing
  nodeTypes?: NodeType[];
  excludeTypes?: NodeType[];
  locale?: string;
  // NEW
  realms?: Realm[];  // Filter by realm(s)
  // ... rest
}
```

---

### 2.3 Add fromOrganization to NovaNetFilter

**File:** `packages/core/src/filters/NovaNetFilter.ts`

```typescript
// Add alongside existing from* methods
static fromOrganization(key: string): NovaNetFilter {
  return new NovaNetFilter({
    rootType: 'Organization',
    rootKey: key,
    traversals: [],
  });
}

// Add traversal method
includeProjects(): NovaNetFilter {
  return this.traverse('HAS_PROJECT');
}

includeCompanyProject(): NovaNetFilter {
  return this.traverse('HAS_COMPANY_PROJECT');
}
```

---

### 2.4 Fix KIND_ICONS

**File:** `packages/core/src/graph/visual-encoding.ts`

```typescript
// Add Organization to KIND_ICONS map
export const KIND_ICONS: Record<string, string> = {
  Organization: 'building-2',  // ADD THIS
  // ... existing
};
```

---

## Phase 3: Studio UI (High Priority)

### 3.1 Fix FacetFilterPanel

**File:** `apps/studio/src/components/sidebar/FacetFilterPanel.tsx`

```typescript
// Line 58-61: Add organization
const REALMS: { key: Realm; label: string; icon: LucideIcon }[] = [
  { key: 'global', label: 'Global', icon: Globe },
  { key: 'organization', label: 'Organization', icon: Building2 },  // ADD
  { key: 'project', label: 'Project', icon: Package },
];
```

---

### 3.2 Fix Navigation API

**File:** `apps/studio/src/app/api/graph/navigation/route.ts`

```typescript
// Line 17: Add organization
const VALID_REALMS: Realm[] = ['global', 'organization', 'project'];
```

**File:** `apps/studio/src/app/api/graph/navigation/__tests__/route.test.ts`

Add test cases for organization realm.

---

### 3.3 Fix ResultsOverview

**File:** `apps/studio/src/components/query/ResultsOverview.tsx`

```typescript
// Lines 38-41: Add organization
const REALM_CONFIG = {
  global: { emoji: '🌍', color: '#2aa198' },
  organization: { emoji: '🏢', color: '#0ea5e9' },  // ADD
  project: { emoji: '📦', color: '#6c71c4' },
} as const;

// Lines 273-298: Update realm array
{(['global', 'organization', 'project'] as const).map((scope, i) => (
```

---

### 3.4 Update Documentation Comments

**Files to update (change "2 realms" → "3 realms", "42/45 nodes" → "43 nodes"):**

| File | Line | Change |
|------|------|--------|
| `apps/studio/src/config/nodeTypes.ts` | 4 | "42 NovaNet node types (v10.4: 8 layers, 2 realms)" → "43 node types (v10.5: 9 layers, 3 realms)" |
| `apps/studio/src/components/sidebar/FacetFilterPanel.tsx` | 7-10 | "Realms (2): global, project" → "Realms (3): global, organization, project" |
| `apps/studio/src/components/sidebar/SidebarTabs.tsx` | 96 | "42 types · 2 realms" → "43 types · 3 realms" |

---

## Phase 4: Arc-Kinds Index Sync (Medium)

### 4.1 Update _index.yaml

**File:** `packages/core/models/arc-kinds/_index.yaml`

```yaml
# Update ownership count: 25 → 28 (or recalculate)
ownership:
  count: 28  # was 25
  arcs:
    # Add missing:
    - has-company-project
    - has-project
    - belongs-to-org
    - has-entity  # if missing
```

Actually, run this to get correct count:
```bash
find packages/core/models/arc-kinds/ownership -name "*.yaml" ! -name "_index.yaml" | wc -l
```

---

## Phase 5: Verification

### 5.1 Run Audits

```bash
# Schema validation
cargo run -- schema validate

# Documentation audit
./tools/scripts/doc-audit.sh

# Skill audit
./tools/scripts/skill-audit.sh
```

### 5.2 Run Tests

```bash
# Rust tests
cargo test

# TypeScript tests
pnpm test

# Studio tests
pnpm --filter=@novanet/studio test
```

### 5.3 Verify Neo4j

```bash
# Seed database
pnpm infra:reset

# Verify organization exists
MATCH (o:Organization) RETURN o;

# Verify company project link
MATCH (o:Organization)-[:HAS_COMPANY_PROJECT]->(p:Project) RETURN o.key, p.key;

# Verify project links
MATCH (o:Organization)-[:HAS_PROJECT]->(p:Project) RETURN o.key, collect(p.key);
```

---

## Implementation Order

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  BATCH 1: Schema Foundation (blocks everything)                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  1. cargo run -- schema generate (regenerate arc-kinds)                       ║
║  2. Add Organization constraint to 00-constraints.cypher                      ║
║  3. Create 03-organization.cypher seed file                                   ║
║  4. pnpm infra:reset && verify Organization in Neo4j                          ║
╚═══════════════════════════════════════════════════════════════════════════════╝

╔═══════════════════════════════════════════════════════════════════════════════╗
║  BATCH 2: TypeScript Types (can parallel with Batch 3)                        ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  1. Add org arcs to relations.schema.ts                                       ║
║  2. Add realms to FilterCriteria                                              ║
║  3. Add fromOrganization() to NovaNetFilter                                   ║
║  4. Fix KIND_ICONS                                                            ║
║  5. pnpm test --filter=@novanet/core                                          ║
╚═══════════════════════════════════════════════════════════════════════════════╝

╔═══════════════════════════════════════════════════════════════════════════════╗
║  BATCH 3: Studio UI (can parallel with Batch 2)                               ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  1. Fix FacetFilterPanel.tsx (add organization)                               ║
║  2. Fix navigation/route.ts (add organization)                                ║
║  3. Fix ResultsOverview.tsx (add organization)                                ║
║  4. Update comments (2 realms → 3 realms)                                     ║
║  5. pnpm test --filter=@novanet/studio                                        ║
╚═══════════════════════════════════════════════════════════════════════════════╝

╔═══════════════════════════════════════════════════════════════════════════════╗
║  BATCH 4: Verification & Cleanup                                              ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  1. Update arc-kinds/_index.yaml                                              ║
║  2. Run doc-audit.sh && skill-audit.sh                                        ║
║  3. Full test suite: cargo test && pnpm test                                  ║
║  4. Manual verification in Studio UI                                          ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Breaking existing filters | Low | High | FilterCriteria.realms is optional, defaults to all |
| Neo4j seed order issues | Medium | Medium | Explicit MATCH/CREATE ordering, test after each batch |
| UI regressions | Low | Medium | Existing tests + manual QA |
| Arc-kind generation drift | Low | Low | schema validate catches mismatches |

---

## Success Criteria

- [ ] `cargo run -- schema validate` passes
- [ ] `./tools/scripts/doc-audit.sh` passes
- [ ] `./tools/scripts/skill-audit.sh` passes
- [ ] `cargo test` — all 246 tests pass
- [ ] `pnpm test` — all packages pass
- [ ] Neo4j: Organization node exists with HAS_COMPANY_PROJECT → Project
- [ ] Studio: Can filter by organization realm in Query mode
- [ ] Studio: Organization nodes visible in graph visualization

---

## Files Changed Summary

**New Files:**
- `packages/db/seed/03-organization.cypher`

**Modified Files:**
- `packages/db/seed/00-constraints.cypher` (add org constraint)
- `packages/core/src/schemas/relations.schema.ts` (add org arcs)
- `packages/core/src/filters/types.ts` (add realms filter)
- `packages/core/src/filters/NovaNetFilter.ts` (add fromOrganization)
- `packages/core/src/graph/visual-encoding.ts` (fix KIND_ICONS)
- `apps/studio/src/components/sidebar/FacetFilterPanel.tsx` (add org realm)
- `apps/studio/src/app/api/graph/navigation/route.ts` (add org realm)
- `apps/studio/src/components/query/ResultsOverview.tsx` (add org realm)
- `packages/core/models/arc-kinds/_index.yaml` (sync counts)
- 10+ files with comment updates (2 realms → 3 realms)

**Estimated Time:** 2-3 hours for complete implementation
