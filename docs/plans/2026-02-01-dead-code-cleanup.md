# Dead Code & Legacy Cleanup Plan

> **For Claude:** Execute tasks sequentially. Each task is independent.

**Goal:** Remove all dead code, fix color mismatches, and correct stale documentation identified by the Ralph Wiggum codebase audit.

**Scope:** `apps/studio/` only (core packages were clean)

---

## Audit Findings Summary

| Severity | Count | Category |
|----------|-------|----------|
| HIGH | 2 | Color mismatch, stale docs |
| MEDIUM | 4 | Dead tokens, dead exports |
| LOW | 1 | eslint version drift |

---

## Task 1: Remove 4 Dead Design Tokens from `tokens.ts`

**Severity:** MEDIUM
**Evidence:** grep confirms these are only referenced in `tokens.ts` itself — never imported anywhere else.

**Files:**
- Modify: `apps/studio/src/design/tokens.ts`

**Dead exports to remove:**
1. `filterTreeClasses` (lines 871-900) — legacy alias for `sidebarTokens`, never imported
2. `glowEffects` (lines 275-289) — never imported outside tokens.ts
3. `sectionHeaderClasses` (lines 218-223) — never imported outside tokens.ts
4. `badgeClasses` (lines 190-197) — never imported outside tokens.ts

**Steps:**
1. Delete `filterTreeClasses` export (lines 871-900)
2. Delete `glowEffects` export (lines 275-289)
3. Delete `sectionHeaderClasses` export (lines 218-223)
4. Delete `badgeClasses` export (lines 190-197)
5. Remove all 4 from the `tokens` aggregate object (lines 941, 958, 960, 962)
6. Clean up orphaned comments/section headers

**Verify:** `grep -r 'filterTreeClasses\|glowEffects\|sectionHeaderClasses\|badgeClasses' apps/studio/src --include='*.ts' --include='*.tsx'` returns nothing

---

## Task 2: Update `scopeAccents` to Solarized Colors

**Severity:** HIGH
**Evidence:** `scopeAccents` in tokens.ts uses Tailwind colors (violet-500, emerald-500, amber-500), but `ResultsOverview.tsx` and `Graph2D.tsx` now use Solarized palette (#6c71c4, #2aa198, #cb4b16). This creates visual inconsistency between Schema filter panel and the graph/results.

**Files:**
- Modify: `apps/studio/src/design/tokens.ts` (lines 905-924)

**Change:**
```typescript
// BEFORE
project: { color: '#8b5cf6', bg: 'bg-violet-500/20', text: 'text-violet-400', border: 'border-violet-500/30' }
global:  { color: '#10b981', bg: 'bg-emerald-500/20', text: 'text-emerald-400', border: 'border-emerald-500/30' }
shared:  { color: '#f59e0b', bg: 'bg-amber-500/20', text: 'text-amber-400', border: 'border-amber-500/30' }

// AFTER (Solarized)
project: { color: '#6c71c4', bg: 'bg-[#6c71c4]/20', text: 'text-[#6c71c4]', border: 'border-[#6c71c4]/30' }
global:  { color: '#2aa198', bg: 'bg-[#2aa198]/20', text: 'text-[#2aa198]', border: 'border-[#2aa198]/30' }
shared:  { color: '#cb4b16', bg: 'bg-[#cb4b16]/20', text: 'text-[#cb4b16]', border: 'border-[#cb4b16]/30' }
```

**Verify:** Visual check on Schema filter panel — accents should match Graph2D attractor nodes.

---

## Task 3: Remove Dead Skeleton Barrel Exports

**Severity:** MEDIUM
**Evidence:** `SkeletonText`, `SkeletonCircle`, `SkeletonCard`, `ShimmerLoader`, `NodeSkeleton`, `PanelSkeleton` are defined in `Skeleton.tsx` and re-exported from `ui/index.ts`, but never imported by any other component. Only the base `Skeleton` component is used externally.

**Files:**
- Modify: `apps/studio/src/components/ui/index.ts` (lines 26-34)

**Change:** Reduce Skeleton export to just `Skeleton`:
```typescript
// BEFORE
export {
  Skeleton,
  SkeletonText,
  SkeletonCircle,
  SkeletonCard,
  ShimmerLoader,
  NodeSkeleton,
  PanelSkeleton,
} from './Skeleton';

// AFTER
export { Skeleton } from './Skeleton';
```

**Note:** Keep the components in `Skeleton.tsx` — they have tests and may be used in the future. Only remove the barrel re-exports since they bloat the public API.

**Verify:** `pnpm type-check --filter=@novanet/studio`

---

## Task 4: Update CLAUDE.md — Fix Route Count & Add Missing Route

**Severity:** HIGH
**Evidence:** CLAUDE.md lists "9 routes" but there are 10. Missing: `/api/graph/organizing-principles`.

**Files:**
- Modify: `apps/studio/CLAUDE.md`

**Changes:**
1. Update `### API Routes (9 routes)` → `### API Routes (10 routes)`
2. Add `- /api/graph/organizing-principles - Organizing principles (scopes, subcategories)` after the ontology entry

---

## Task 5: Fix eslint-config-next Version Drift

**Severity:** LOW
**Evidence:** `next` is `^16.1.4` but `eslint-config-next` is `15.1.4`. Should match major version.

**Files:**
- Modify: `apps/studio/package.json`

**Change:** Update `"eslint-config-next": "15.1.4"` → `"eslint-config-next": "^16.1.4"`

**Verify:** `pnpm install && pnpm lint --filter=@novanet/studio`

---

## Task 6: Verification Loop

**Steps:**
1. Run `pnpm type-check --filter=@novanet/studio`
2. Run grep for all removed tokens — confirm zero matches
3. Verify dev server loads without errors
4. Visual check: Schema filter panel uses Solarized colors

---

## Not Fixed (Intentional)

| Item | Reason |
|------|--------|
| Skeleton variants in `Skeleton.tsx` | Have tests, may be used later. Only barrel re-exports removed. |
| `iconButtonClasses` in tokens.ts | Actively used — confirmed by grep |
| react19 catalog entry | pnpm workspace config, not dead |
