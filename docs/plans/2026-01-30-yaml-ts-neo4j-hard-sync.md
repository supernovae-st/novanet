# YAML-TS-Neo4j Hard Sync Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove deprecated properties (icon, priority, freshness) from entire stack, align TypeScript and seeds with YAML v7.11.0 as source of truth.

**Architecture:** YAML models are the single source of truth. TypeScript types must match YAML. Neo4j seeds must match TypeScript. Studio uses configuration for presentation concerns (icons) instead of database properties.

**Tech Stack:** TypeScript 5.9, Zod, Neo4j (Cypher), React (Studio)

---

## Phase 1: Create Icon Configuration (Presentation Layer)

### Task 1.1: Create NODE_ICONS config file

**Files:**
- Create: `packages/core/src/config/nodeIcons.ts`
- Modify: `packages/core/src/config/index.ts`

**Step 1: Create nodeIcons.ts**

```typescript
// packages/core/src/config/nodeIcons.ts
// Presentation layer configuration for node icons
// Moved from database properties per YAML v7.11.0

import type { NodeType } from '../types/nodes.js';

/**
 * Node icons for UI rendering.
 * YAML v7.11.0: icon removed from standard properties (UI concern).
 * Icons are now in presentation layer, not data model.
 */
export const NODE_ICONS: Record<NodeType, string> = {
  // PROJECT SCOPE (3)
  Project: '📦',
  BrandIdentity: '🎨',
  ProjectL10n: '🌐',

  // CONTENT (6)
  Concept: '💡',
  ConceptL10n: '💬',
  Page: '📄',
  PageType: '📐',
  Block: '🧱',
  BlockType: '📋',

  // LOCALE (15)
  Locale: '🌍',
  LocaleIdentity: '🆔',
  LocaleVoice: '🎭',
  LocaleCulture: '🏛️',
  LocaleCultureReferences: '🎭',
  LocaleMarket: '📈',
  LocaleLexicon: '📚',
  LocaleRulesAdaptation: '🔄',
  LocaleRulesFormatting: '📝',
  LocaleRulesSlug: '🔗',
  Expression: '💭',
  Reference: '📍',
  Metaphor: '🎨',
  Pattern: '🔣',
  Constraint: '⚠️',

  // GENERATION (5)
  PagePrompt: '📝',
  BlockPrompt: '📝',
  BlockRules: '📏',
  PageL10n: '📃',
  BlockL10n: '📝',

  // SEO (3)
  SEOKeywordL10n: '🔍',
  SEOKeywordMetrics: '📊',
  SEOMiningRun: '⚙️',

  // GEO (3)
  GEOSeedL10n: '🤖',
  GEOSeedMetrics: '📊',
  GEOMiningRun: '⚙️',
};

/**
 * Get icon for a node type with fallback.
 */
export function getNodeIcon(type: NodeType): string {
  return NODE_ICONS[type] ?? '❓';
}
```

**Step 2: Export from config/index.ts**

Add to `packages/core/src/config/index.ts`:

```typescript
export { NODE_ICONS, getNodeIcon } from './nodeIcons.js';
```

**Step 3: Verify build**

Run: `cd packages/core && pnpm build`
Expected: BUILD SUCCESS

**Step 4: Commit**

```bash
git add packages/core/src/config/nodeIcons.ts packages/core/src/config/index.ts
git commit -m "feat(core): add NODE_ICONS config for presentation layer

YAML v7.11.0 removed icon from standard properties (UI concern).
Icons now live in presentation config, not data model.

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 2: Update TypeScript Types

### Task 2.1: Remove Priority and Freshness types from locale-knowledge.ts

**Files:**
- Modify: `packages/core/src/types/locale-knowledge.ts`

**Step 1: Update file header and remove type exports**

Replace lines 1-8:

```typescript
// novanet-core/src/types/locale-knowledge.ts
// Locale Knowledge types v8.2.0
//
// v8.2.0 CHANGES:
//   - REMOVED: icon, priority, freshness from all interfaces (YAML v7.11.0 alignment)
//   - Standard properties now: key, display_name, description, llm_context, created_at, updated_at
//
// v7.11.0 STANDARD PROPERTIES (all nodes):
//   key, display_name, description, llm_context, created_at, updated_at

// REMOVED v8.2.0: Priority and Freshness types (never implemented, YAGNI)
// export type Priority = 'critical' | 'high' | 'medium' | 'low';
// export type Freshness = 'realtime' | 'hourly' | 'daily' | 'static';
```

**Step 2: Update Locale interface (remove icon, priority, freshness)**

Replace Locale interface:

```typescript
export interface Locale {
  // Standard properties (v8.2.0)
  key: string;               // BCP 47: "fr-FR"
  display_name: string;      // "French (France)"
  description: string;       // "French locale for France market"
  llm_context: string;       // "USE: French content. TRIGGERS: fr-FR. NOT: Canadian French."

  // Locale-specific
  language_code: string;     // ISO 639-1: "fr"
  country_code: string;      // ISO 3166-1: "FR"
  name_native: string;
  is_primary: boolean;
  fallback_chain: string[];
  created_at: Date;
  updated_at: Date;
}
```

**Step 3: Update LocaleIdentity interface**

Replace LocaleIdentity interface (remove icon, priority, freshness):

```typescript
export interface LocaleIdentity {
  // Standard properties (v8.2.0 - no key, linked via HAS_IDENTITY)
  display_name: string;      // "French Identity"
  description: string;       // "Identity characteristics for fr-FR"
  llm_context: string;       // "USE: script/encoding decisions."

  // Script & Writing
  script_code: string;
  script_name: string;
  script_direction: 'ltr' | 'rtl';
  has_case: boolean;
  special_characters: string;
  diacritics: boolean;
  ligatures: boolean;

  // Geographic
  continent: string;
  region: string;
  capital: string;
  timezone: string;
  utc_offset: string;
  dst_observed: boolean;

  // Technical
  keyboard_layout: string;
  encoding: string;

  // Language family
  language_family: string;
  related_languages: string[];

  created_at: Date;
  updated_at: Date;
}
```

**Step 4: Update all remaining interfaces**

Apply same pattern to ALL interfaces in locale-knowledge.ts:
- `LocaleVoice` - remove icon, priority, freshness
- `LocaleCulture` - remove icon, priority, freshness
- `LocaleMarket` - remove icon, priority, freshness
- `LocaleLexicon` - remove icon, priority, freshness
- `Expression` - remove icon, priority, freshness
- `LocaleRulesAdaptation` - remove (entire interface uses standard props)
- `LocaleRulesFormatting` - remove
- `LocaleRulesSlug` - remove
- `LocaleCultureReferences` - remove

**Step 5: Update LocaleKnowledgeNode union type**

No changes needed - union type references updated interfaces.

**Step 6: Type-check**

Run: `cd packages/core && pnpm type-check`
Expected: ERRORS (expected - index.ts still imports Priority/Freshness)

---

### Task 2.2: Update types/index.ts

**Files:**
- Modify: `packages/core/src/types/index.ts`

**Step 1: Remove Priority/Freshness re-exports (line 70)**

Remove:
```typescript
export type { Priority, Freshness } from './locale-knowledge.js';
```

**Step 2: Remove Priority/Freshness import (line 76)**

Remove:
```typescript
import type { Priority, Freshness } from './locale-knowledge.js';
```

**Step 3: Update StandardNodeProperties interface (lines 78-88)**

Replace:

```typescript
export interface StandardNodeProperties {
  key: string;
  display_name: string;
  description: string;
  llm_context: string;
  created_at: Date;
  updated_at: Date;
}
```

**Step 4: Update all node interfaces that use removed types**

Search for `priority: Priority` and `freshness: Freshness` and `icon: string` in the file and remove from:
- ConceptL10n
- PageL10n
- BlockL10n
- SEOKeywordL10n
- SEOKeywordMetrics
- SEOMiningRun
- GEOSeedL10n
- GEOSeedMetrics
- GEOMiningRun

**Step 5: Type-check**

Run: `cd packages/core && pnpm type-check`
Expected: ERRORS (schemas still reference removed types)

---

### Task 2.3: Update types/project.ts

**Files:**
- Modify: `packages/core/src/types/project.ts`

**Step 1: Remove Priority/Freshness import (line 8)**

Remove:
```typescript
import type { Priority, Freshness } from './locale-knowledge.js';
```

**Step 2: Update Project interface**

Remove `icon`, `priority`, `freshness` properties.

**Step 3: Update ProjectL10n interface**

Remove `icon`, `priority`, `freshness` properties.

**Step 4: Update BrandIdentity interface**

Remove `icon`, `priority`, `freshness` properties.

---

## Phase 3: Update Zod Schemas

### Task 3.1: Update locale-knowledge.schema.ts

**Files:**
- Modify: `packages/core/src/schemas/locale-knowledge.schema.ts`

**Step 1: Remove Priority/Freshness schemas (lines 10-14)**

Remove:
```typescript
export const PrioritySchema = z.enum(['critical', 'high', 'medium', 'low']);
export const FreshnessSchema = z.enum(['realtime', 'hourly', 'daily', 'static']);

export type Priority = z.infer<typeof PrioritySchema>;
export type Freshness = z.infer<typeof FreshnessSchema>;
```

**Step 2: Update all schema objects**

Remove any `.extend()` or properties that add `icon`, `priority`, `freshness`.

**Step 3: Build and type-check**

Run: `cd packages/core && pnpm build && pnpm type-check`
Expected: SUCCESS (all types aligned)

**Step 4: Run tests**

Run: `cd packages/core && pnpm test`
Expected: Some failures (tests may reference removed types)

**Step 5: Fix failing tests**

Update test files that reference `Priority`, `Freshness`, or `icon` properties.

**Step 6: Commit Phase 2-3**

```bash
git add packages/core/src/types/ packages/core/src/schemas/
git commit -m "refactor(core): remove icon/priority/freshness from types

BREAKING CHANGE: Removed deprecated properties per YAML v7.11.0

- Removed: Priority, Freshness types
- Removed: icon, priority, freshness from all node interfaces
- Updated: StandardNodeProperties (6 props instead of 9)
- Updated: All Zod schemas

Use NODE_ICONS config for icon display instead.

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 4: Update Neo4j Seeds

### Task 4.1: Update 01-concepts-mvp.cypher

**Files:**
- Modify: `packages/db/seed/01-concepts-mvp.cypher`

**Step 1: Update file header**

```cypher
// NovaNet MVP Seed v8.2.0 - YAML v7.11.0 Aligned
//
// v8.2.0 CHANGES:
//   - REMOVED: icon, priority, freshness from all nodes (YAML v7.11.0)
//   - Standard properties: key, display_name, description, llm_context, created_at, updated_at
```

**Step 2: Update Locale nodes**

Before:
```cypher
CREATE (:Locale {
  key: "en-US",
  display_name: "English (US)",
  icon: "🇺🇸",
  description: "American English locale for United States market",
  llm_context: "USE: primary English locale...",
  priority: "critical",
  freshness: "static",
  language_code: "en",
  ...
});
```

After:
```cypher
CREATE (:Locale {
  key: "en-US",
  display_name: "English (US)",
  description: "American English locale for United States market",
  llm_context: "USE: primary English locale...",
  language_code: "en",
  ...
});
```

**Step 3: Apply to ALL Locale nodes** (6 locales)

Remove `icon`, `priority`, `freshness` from: en-US, fr-FR, fr-CA, es-ES, de-DE, ja-JP

**Step 4: Update ALL Concept nodes**

Remove `icon`, `priority`, `freshness` from all Concept CREATE statements.

**Step 5: Update ALL other nodes**

Remove from: Project, BrandIdentity, Page, Block, BlockType, etc.

---

### Task 4.2: Update 02-locale-knowledge.cypher

**Files:**
- Modify: `packages/db/seed/02-locale-knowledge.cypher`

**Step 1: Remove from ALL LocaleIdentity nodes**
**Step 2: Remove from ALL LocaleVoice nodes**
**Step 3: Remove from ALL LocaleCulture nodes**
**Step 4: Remove from ALL LocaleMarket nodes**
**Step 5: Remove from ALL LocaleLexicon nodes**
**Step 6: Remove from ALL Expression nodes**

---

### Task 4.3: Update 03-prompts-v720.cypher

**Files:**
- Modify: `packages/db/seed/03-prompts-v720.cypher`

Remove `icon`, `priority`, `freshness` from all PagePrompt, BlockPrompt, BlockRules nodes.

---

### Task 4.4: Update 04-project-qrcode-ai.cypher

**Files:**
- Modify: `packages/db/seed/04-project-qrcode-ai.cypher`

Remove `icon`, `priority`, `freshness` from all nodes.

---

### Task 4.5: Commit seed updates

```bash
git add packages/db/seed/
git commit -m "refactor(db): remove deprecated props from seeds

BREAKING CHANGE: Seeds aligned with YAML v7.11.0

- Removed: icon, priority, freshness from all 195+ occurrences
- Updated: All 4 seed files

Existing databases need migration (see 004-remove-deprecated.cypher).

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 5: Database Migration Script

### Task 5.1: Create migration script

**Files:**
- Create: `packages/db/migrations/004-remove-deprecated-properties.cypher`

```cypher
// Migration 004: Remove deprecated properties (v8.2.0)
// YAML v7.11.0 alignment - removes icon, priority, freshness from all nodes
//
// Run with: cypher-shell -u neo4j -p novanetpassword < migrations/004-remove-deprecated-properties.cypher

// Count affected nodes before migration
MATCH (n)
WHERE n.icon IS NOT NULL OR n.priority IS NOT NULL OR n.freshness IS NOT NULL
RETURN labels(n)[0] AS label, count(*) AS count
ORDER BY count DESC;

// Remove properties from ALL nodes
MATCH (n)
WHERE n.icon IS NOT NULL OR n.priority IS NOT NULL OR n.freshness IS NOT NULL
REMOVE n.icon, n.priority, n.freshness
RETURN count(n) AS nodes_updated;

// Verify no remaining deprecated properties
MATCH (n)
WHERE n.icon IS NOT NULL OR n.priority IS NOT NULL OR n.freshness IS NOT NULL
RETURN count(n) AS remaining;
// Expected: 0
```

**Step 2: Commit migration**

```bash
git add packages/db/migrations/
git commit -m "feat(db): add migration to remove deprecated properties

Run after updating to v8.2.0 to clean existing databases.

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 6: Update Studio App

### Task 6.1: Update Studio to use NODE_ICONS

**Files:**
- Modify: `apps/studio/src/config/nodeTypes.ts`

**Step 1: Import NODE_ICONS from core**

Add:
```typescript
import { NODE_ICONS } from '@novanet/core/config';
```

**Step 2: Update nodeTypeConfigs to use imported icons**

Replace hardcoded icon values with `NODE_ICONS[type]` or keep as-is (Studio has its own copy which is fine).

Actually, Studio already has its own `icon` in `nodeTypeConfigs` - this is the presentation layer config. No change needed.

---

### Task 6.2: Update Studio types

**Files:**
- Modify: `apps/studio/src/types/index.ts`

**Step 1: Remove Priority/Freshness imports if present**

Check and remove any imports of Priority or Freshness.

**Step 2: Update GraphNode interface if it references removed properties**

The `GraphNode` interface has optional `priority` and `freshness` - these can remain optional for backward compatibility with existing data, but remove them to align:

```typescript
export interface GraphNode {
  id: string;
  type: NodeType;
  key: string;
  displayName: string;
  description?: string;
  llmContext?: string;
  createdAt?: string;
  updatedAt?: string;
  /** Additional properties not in standard fields */
  data?: Record<string, unknown>;
}
```

---

### Task 6.3: Verify Studio builds

Run: `cd apps/studio && pnpm build`
Expected: SUCCESS

**Step 4: Commit Studio changes**

```bash
git add apps/studio/
git commit -m "refactor(studio): align types with core v8.2.0

Removed deprecated priority/freshness from GraphNode interface.
Icons continue using nodeTypeConfigs (presentation layer).

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 7: Update YAML Examples

### Task 7.1: Update YAML node examples

**Files:**
- Modify: All 35 files in `packages/core/models/nodes/**/*.yaml`

**Step 1: Update example.data sections**

Remove `icon`, `priority`, `freshness` from ALL YAML example sections.

Example before:
```yaml
example:
  data:
    key: "page-pricing"
    display_name: "Pricing Page"
    icon: "📄"
    description: "..."
    priority: "critical"
    freshness: "static"
```

Example after:
```yaml
example:
  data:
    key: "page-pricing"
    display_name: "Pricing Page"
    description: "..."
    llm_context: "..."
```

**Step 2: Commit YAML updates**

```bash
git add packages/core/models/
git commit -m "docs(models): remove deprecated props from YAML examples

Aligned examples with v7.11.0 standard properties.

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Phase 8: Final Verification

### Task 8.1: Full build verification

**Step 1: Clean and rebuild all**

```bash
pnpm clean && pnpm install && pnpm build
```

Expected: SUCCESS

**Step 2: Run all tests**

```bash
pnpm test
```

Expected: SUCCESS (or fix any remaining failures)

**Step 3: Type-check all**

```bash
pnpm type-check
```

Expected: SUCCESS

**Step 4: Reset and reseed database**

```bash
pnpm infra:reset
```

Expected: SUCCESS with 0 icon/priority/freshness properties

**Step 5: Verify in Neo4j**

```cypher
MATCH (n) WHERE n.icon IS NOT NULL RETURN count(n);
// Expected: 0

MATCH (n) WHERE n.priority IS NOT NULL RETURN count(n);
// Expected: 0

MATCH (n) WHERE n.freshness IS NOT NULL RETURN count(n);
// Expected: 0
```

---

### Task 8.2: Update _index.yaml changelog

**Files:**
- Modify: `packages/core/models/_index.yaml`

Add v8.2.0 changes section:

```yaml
# v8.2.0 CHANGES (from v8.1.0)
# ═══════════════════════════════════════════════════════════════════════════════
#
# YAML v7.11.0 Alignment Complete:
#   - REMOVED: icon, priority, freshness from ALL TypeScript interfaces
#   - REMOVED: Priority, Freshness type definitions
#   - REMOVED: icon, priority, freshness from ALL Neo4j seeds
#   - ADDED: NODE_ICONS config in presentation layer
#   - ADDED: Migration script 004-remove-deprecated-properties.cypher
#
# Standard properties (6 total):
#   key, display_name, description, llm_context, created_at, updated_at
#
# Statistics: 35 nodes (unchanged), 50 relations (unchanged)
```

**Step 2: Final commit**

```bash
git add packages/core/models/_index.yaml
git commit -m "docs(models): document v8.2.0 alignment changes

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

---

## Summary

| Phase | Tasks | Estimated Time |
|-------|-------|----------------|
| 1. NODE_ICONS config | 1 | 10 min |
| 2. TypeScript types | 3 | 30 min |
| 3. Zod schemas | 1 | 15 min |
| 4. Neo4j seeds | 5 | 45 min |
| 5. Migration script | 1 | 10 min |
| 6. Studio app | 3 | 20 min |
| 7. YAML examples | 1 | 30 min |
| 8. Final verification | 2 | 15 min |
| **Total** | **17 tasks** | **~3 hours** |

---

## Rollback Plan

If issues arise:

1. **Revert commits**: `git revert HEAD~N`
2. **Restore database**: `pnpm infra:reset` with original seeds
3. **Document decision**: Update `_index.yaml` to mark properties as NOT deprecated

---

*Plan created: 2026-01-30*
*Target version: v8.2.0*
