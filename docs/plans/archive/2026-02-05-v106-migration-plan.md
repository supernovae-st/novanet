# NovaNet v10.6 Migration Plan: 2-Realm Architecture

**Date:** 2026-02-05
**Version:** 10.6.0
**Status:** Ready for Implementation

## Executive Summary

This plan migrates from v10.5 (3 realms) to v10.6 (2 realms), simplifying the architecture:

```
v10.5: GLOBAL / ORGANIZATION / PROJECT  (3 realms)
v10.6: GLOBAL / TENANT                  (2 realms)
```

**Key Insight:** Organization and Project are NOT separate realms. They both live in TENANT realm. Multi-tenant isolation happens at the TENANT boundary, not between Organization and Project.

**Key Pattern:**
```
TENANT realm:
  Organization ──[:HAS_ENTITY]─────────> Entity (org-wide, can be used by 0-N projects)
              ──[:HAS_COMPANY_PROJECT]──> Project (org branding, 1:1)
              ──[:HAS_PROJECT]──────────> Project[] (products, 1:N)

  Page/Block ──[:USES_ENTITY]──────────> Entity (reference, N:M)
```

**Entity Location:** Entity lives at Organization level, NOT Project level. Pages/Blocks reference Entity via USES_ENTITY (not ownership).

---

## Architecture Overview

### 2-Realm Model

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  GLOBAL REALM (read-only, shared)                                             ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  Layers:                                                                      ║
║    - config: Locale, Formatting, Style, Slugification, Adaptation             ║
║    - locale-knowledge: TermSet, Term, ExpressionSet, Expression, etc.         ║
║    - seo: SEOKeyword, SEOKeywordMetrics, SEOMiningRun                         ║
║                                                                               ║
║  Traversal: ALL tenants can READ global nodes                                 ║
╚═══════════════════════════════════════════════════════════════════════════════╝

╔═══════════════════════════════════════════════════════════════════════════════╗
║  TENANT REALM (multi-tenant isolated)                                         ║
╠═══════════════════════════════════════════════════════════════════════════════╣
║  Layers:                                                                      ║
║    - config: Organization                                                     ║
║    - semantic: Entity, EntityL10n (org-wide, referenced by projects)          ║
║    - foundation: Project, Brand, ProjectL10n                                  ║
║    - structure: Page, Block, PageType, BlockType                              ║
║    - instruction: PagePrompt, BlockPrompt, BlockRules                         ║
║    - output: PageL10n, BlockL10n, GenerationRun, etc.                         ║
║                                                                               ║
║  Traversal: Tenant A CANNOT see Tenant B                                      ║
║  AI Context: Spreading activation from Page/Block → Entity → EntityL10n      ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

### AI Context Isolation

When AI works on a project:
1. Starts from Page or Block (entry point)
2. Follows arcs (spreading activation)
3. Reaches Entity via USES_ENTITY
4. Reaches EntityL10n via HAS_L10N
5. Can traverse to Global realm for locale knowledge
6. CANNOT traverse UP to Organization (one-way isolation)
7. CANNOT see other Tenants (realm boundary)

---

## Phase 1: YAML Source Updates

### 1.1 Update taxonomy.yaml

**File:** `packages/core/models/taxonomy.yaml`

```yaml
version: "10.6.0"

node_realms:
  - key: global
    display_name: Global
    emoji: "🌍"
    color: "#2aa198"
    llm_context: |
      Shared across ALL tenants. Universal locale knowledge and SEO data.
      READ-ONLY. No Entity here - Entity lives in TENANT realm.
    layers:
      - key: config
        # ... (unchanged)
      - key: locale-knowledge
        # ... (unchanged)
      - key: seo
        # ... (unchanged)

  - key: tenant
    display_name: Tenant
    emoji: "🏢"
    color: "#6c71c4"
    llm_context: |
      Multi-tenant isolated realm. Contains Organization, Projects, Entities,
      Pages, Blocks, and all generated outputs. Tenant A cannot see Tenant B.
      AI traversal starts from Page/Block and follows arcs via spreading activation.
    layers:
      - key: config
        display_name: Tenant Configuration
        emoji: "⚙️"
        color: "#64748b"
        llm_context: |
          Organization root node. Entry point for tenant-level traversal.
          Links to Entity (HAS_ENTITY), company project (HAS_COMPANY_PROJECT),
          and product projects (HAS_PROJECT).

      - key: semantic
        display_name: Semantic Layer
        emoji: "💡"
        color: "#f97316"
        llm_context: |
          Invariant entities (Entity) and their localizations (EntityL10n).
          Entity lives at Organization level but can be referenced by 0-N projects.
          Pages/Blocks use USES_ENTITY arc (reference, not ownership).

      - key: foundation
        display_name: Foundation
        emoji: "🏛️"
        color: "#3b82f6"
        llm_context: |
          Project identity. Project, Brand, ProjectL10n. Each Organization
          has 1 company project (branding) + N product projects.

      - key: structure
        display_name: Structure
        emoji: "🏗️"
        color: "#06b6d4"
        llm_context: |
          Information architecture. Page, Block, PageType, BlockType.
          Defines the skeleton of each project's website.

      - key: instruction
        display_name: Instructions
        emoji: "📝"
        color: "#eab308"
        llm_context: |
          Generation directives. PagePrompt, BlockPrompt, BlockRules.
          Guides LLM during content generation.

      - key: output
        display_name: Generated Output
        emoji: "✨"
        color: "#22c55e"
        llm_context: |
          LLM-generated content. PageL10n, BlockL10n, GenerationRun.
          Final localized content ready for rendering.
```

**Changes:**
- `organization` + `project` realms → `tenant` realm
- Layers reorganized: config, semantic, foundation, structure, instruction, output
- Entity/EntityL10n moved to tenant/semantic

---

### 1.2 Reorganize node-classes Directory

**Before (v10.5):**
```
models/node-classes/
├── global/
│   ├── config/
│   ├── locale-knowledge/
│   └── seo/
├── organization/
│   └── config/
└── project/
    ├── foundation/
    ├── structure/
    ├── semantic/
    ├── instruction/
    └── output/
```

**After (v10.6):**
```
models/node-classes/
├── global/
│   ├── config/
│   ├── locale-knowledge/
│   └── seo/
└── tenant/
    ├── config/         # Organization
    ├── semantic/       # Entity, EntityL10n (moved from project)
    ├── foundation/     # Project, Brand, ProjectL10n
    ├── structure/      # Page, Block, PageType, BlockType
    ├── instruction/    # PagePrompt, BlockPrompt, BlockRules
    └── output/         # PageL10n, BlockL10n, GenerationRun
```

**Commands:**
```bash
# Merge organization + project into tenant
mv models/node-classes/organization models/node-classes/tenant
mv models/node-classes/project/* models/node-classes/tenant/

# Update realm: field in all YAML files
find models/node-classes/tenant -name "*.yaml" -exec sed -i '' 's/realm: organization/realm: tenant/g' {} \;
find models/node-classes/tenant -name "*.yaml" -exec sed -i '' 's/realm: project/realm: tenant/g' {} \;
```

---

### 1.3 Update Arc-Kinds

**Key Arc Updates:**

| Arc | Source | Target | Change |
|-----|--------|--------|--------|
| HAS_ENTITY | Organization | Entity | scope: intra_realm (was cross_realm) |
| HAS_COMPANY_PROJECT | Organization | Project | scope: intra_realm (was cross_realm) |
| HAS_PROJECT | Organization | Project | scope: intra_realm (was cross_realm) |
| BELONGS_TO_ORG | Project | Organization | scope: intra_realm (was cross_realm) |
| USES_ENTITY | Page/Block | Entity | scope: intra_realm (semantic reference) |

All organization → project arcs become intra_realm since they're now in the same TENANT realm.

---

## Phase 2: Rust Generator Updates

### 2.1 Update src/parsers/taxonomy.rs

Change realm enum:
```rust
pub enum NodeRealm {
    Global,
    Tenant,  // was: Organization, Project
}
```

### 2.2 Update src/generators/organizing.rs

Update REALM_ORDER and layer mappings.

### 2.3 Update src/tui/data.rs

Update TaxonomyTree to use 2 realms.

---

## Phase 3: TypeScript Updates

### 3.1 Update types/index.ts

```typescript
export type Realm = 'global' | 'tenant';

export type TenantLayer =
  | 'config'
  | 'semantic'
  | 'foundation'
  | 'structure'
  | 'instruction'
  | 'output';

export type GlobalLayer =
  | 'config'
  | 'locale-knowledge'
  | 'seo';
```

### 3.2 Update NovaNetFilter.ts

```typescript
static fromOrganization(key: string): NovaNetFilter {
  return new NovaNetFilter({
    rootType: 'Organization',
    rootKey: key,
    traversals: [],
  });
}

// Traversal to Entity (direct from Organization)
includeEntities(): NovaNetFilter {
  return this.traverse('HAS_ENTITY');
}
```

---

## Phase 4: Studio UI Updates

### 4.1 Update FacetFilterPanel.tsx

```typescript
const REALMS: { key: Realm; label: string; icon: LucideIcon }[] = [
  { key: 'global', label: 'Global', icon: Globe },
  { key: 'tenant', label: 'Tenant', icon: Building2 },
];
```

### 4.2 Update ResultsOverview.tsx

```typescript
const REALM_CONFIG = {
  global: { emoji: '🌍', color: '#2aa198' },
  tenant: { emoji: '🏢', color: '#6c71c4' },
} as const;
```

---

## Phase 5: Neo4j Updates

### 5.1 Update Constraints

No change needed - Organization constraint already exists.

### 5.2 Update Seeds

Organization seed creates Entity directly linked:

```cypher
// Organization → Entity (direct ownership)
MATCH (org:Organization {key: 'org-supernovae-studio'})
CREATE (org)-[:HAS_ENTITY]->(e:Entity {
  key: 'entity-brand-values',
  display_name: 'Brand Values',
  // ...
});
```

---

## Phase 6: Documentation Updates

### 6.1 Files to Update

| File | Change |
|------|--------|
| `README.md` | 3 realms → 2 realms |
| `ROADMAP.md` | Add v10.6 milestone |
| `CLAUDE.md` | Update architecture section |
| `packages/core/CLAUDE.md` | Update v10.6 architecture |
| `packages/core/README.md` | 3 realms → 2 realms |
| `apps/studio/CLAUDE.md` | Update realm references |
| `.claude/rules/novanet-terminology.md` | Update realm definitions |
| `.claude/rules/novanet-decisions.md` | Add ADR for 2-realm decision |

---

## Verification

### Run Audits

```bash
# Schema validation
cargo run -- schema validate

# Documentation audit
./tools/scripts/doc-audit.sh

# Skill audit
./tools/scripts/skill-audit.sh
```

### Run Tests

```bash
# Rust tests
cargo test

# TypeScript tests
pnpm test

# Full suite
pnpm type-check && pnpm lint && pnpm test
```

---

## Success Criteria

- [ ] `cargo run -- schema validate` passes
- [ ] `./tools/scripts/doc-audit.sh` — 100% pass
- [ ] `./tools/scripts/skill-audit.sh` — 100% pass
- [ ] `cargo test` — all tests pass
- [ ] `pnpm test` — all packages pass
- [ ] Studio: Can filter by 2 realms (global, tenant)
- [ ] Neo4j: Organization → HAS_ENTITY → Entity works

---

## Statistics

| Metric | v10.5 | v10.6 | Change |
|--------|-------|-------|--------|
| Realms | 3 | 2 | -1 |
| Layers | 9 | 7 | -2 |
| Nodes | 43 | 43 | 0 |
| Arcs | 63 | 63 | 0 |

---

## Files Changed Summary

**Renamed:**
- `models/node-classes/organization/` → `models/node-classes/tenant/config/`
- `models/node-classes/project/` → `models/node-classes/tenant/*`

**Modified:**
- `packages/core/models/taxonomy.yaml` (2 realms)
- `packages/core/src/types/index.ts` (Realm type)
- `tools/novanet/src/parsers/taxonomy.rs` (NodeRealm enum)
- All documentation files (3 → 2 realms)
- All Studio UI files referencing realms
