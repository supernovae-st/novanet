# NovaNet Cleanup Plan - Dead Code & Branding

> Date: 2026-01-26
> Status: In Progress
> Execution: Subagent-driven with code review

## Overview

Clean up dead code and standardize branding after jungo → novanet migration.

## Tasks

### Batch 1: Core Dead Code Removal

#### Task 1.1: Remove unused Zod types from prompts.schema.ts
- **File:** `core/src/schemas/prompts.schema.ts`
- **Action:** Remove lines 62-64 (PagePromptZod, BlockPromptZod, BlockRulesZod)
- **Reason:** Duplicates of manually defined types in types/prompts.ts
- **Risk:** Low - grep confirms no imports

#### Task 1.2: Remove unused relation validation functions
- **File:** `core/src/schemas/relations.schema.ts`
- **Action:** Remove validateRelation(), getRelationProps(), getAllRelationTypes()
- **Lines:** ~564-590
- **Reason:** Never called anywhere in codebase
- **Risk:** Low - grep confirms no usages

---

### Batch 2: Studio Dead Code Removal

#### Task 2.1: Remove unused hooks
- **Files:** `studio/src/hooks/useKeyboardHandler.ts`, `studio/src/hooks/useLatestRef.ts`, `studio/src/hooks/useViewportInsets.ts`
- **Action:** Remove useKeyPress, useLatestCallback, useFitViewConfig exports (or keep if useful for future)
- **Risk:** Low - not imported anywhere

#### Task 2.2: Remove unused constants
- **File:** `studio/src/config/constants.ts`
- **Action:** Remove BUTTON_SIZES, BUTTON_CLASSES, GLASS_CLASSES
- **Lines:** 22-30
- **Reason:** Never referenced
- **Risk:** Low

#### Task 2.3: Remove unused icon types
- **File:** `studio/src/config/iconSystem.ts`
- **Action:** Remove GraphIconKey, ActionIconKey, StatusIconKey, NavIconKey, ContentIconKey, DomainIconKey, IconColorKey, IconSizeKey type exports
- **Lines:** 279-286
- **Reason:** Types never imported
- **Risk:** Low

---

### Batch 3: Branding Standardization (linear → novanet)

#### Task 3.1: Rename LINEAR_COLORS constant
- **File:** `studio/src/config/constants.ts`
- **Action:** Rename LINEAR_COLORS → ACCENT_COLORS (keep as secondary palette)
- **Reason:** "linear" is Linear app branding, not NovaNet

#### Task 3.2: Update CSS variables
- **File:** `studio/src/app/globals.css`
- **Action:** Rename --linear-* → --accent-* CSS variables
- **Lines:** 650-655

#### Task 3.3: Update Tailwind config
- **File:** `studio/tailwind.config.ts`
- **Action:** Rename `linear` color group → `accent`
- **Lines:** 82-89

#### Task 3.4: Update component classes
- **Files:** Multiple (19 files)
- **Action:** Replace all `linear-blue`, `linear-orange`, etc. → `accent-blue`, `accent-orange`
- **Approach:** Global find/replace with verification

---

### Batch 4: Deprecated Code Cleanup

#### Task 4.1: Remove deprecated toolbar export
- **File:** `studio/src/design/tokens.ts`
- **Action:** Remove deprecated toolbar alias (lines 137-138, 320)
- **Verify:** No imports of `toolbar` from tokens

---

## Verification

After each batch:
1. Run `npm test` - all 459 tests must pass
2. Run `npm run type-check` - no TypeScript errors
3. Run `npm run lint` - no lint errors
4. Code review by subagent

## Rollback

All changes are atomic per batch. If tests fail, revert batch and investigate.
