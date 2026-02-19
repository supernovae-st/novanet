# Sniper Cleanup Plan v11.6.2

> Generated: 2026-02-10
> Status: In Progress
> Estimated: 3 batches

## Executive Summary

Comprehensive cleanup of deprecated nomenclature, unsafe patterns, and technical debt identified by 10 sniper agents.

---

## Batch 1: Critical Fixes (L10n + Rust Unsafe)

### 1.1 L10n → Content/Generated Renames

| File | Change | Status |
|------|--------|--------|
| `packages/core/src/filters/CypherGenerator.ts:38` | `'l10n'` → `'content'` | [ ] |
| `packages/core/src/filters/ViewLoader.ts:232` | `includeL10n()` → `includeContent()` | [ ] |
| `packages/core/src/filters/NovaNetFilter.ts:207` | Method rename + comment update | [ ] |
| `apps/studio/src/lib/filterAdapter.ts:519` | Description update | [ ] |
| `apps/studio/src/hooks/__tests__/useFocusMode.test.ts` | `l10n-1` → `content-1` | [ ] |
| `apps/studio/tailwind.config.ts:32` | `conceptL10n` → `entityContent` | [ ] |
| `tools/novanet/src/commands/node.rs:218` | Test assertion update | [ ] |

### 1.2 Comment Updates (L10n references)

| File | Lines | Status |
|------|-------|--------|
| `packages/core/src/types/project.ts` | 4, 44, 68, 73, 146 | [ ] |
| `packages/core/src/types/index.ts` | 88, 114, 182, 225, 312 | [ ] |
| `packages/core/src/schemas/relations.schema.ts` | 13, 28, 32, 92, 94, 448 | [ ] |
| `apps/studio/src/components/graph/nodes/NodeConfig.ts:54` | Comment | [ ] |

### 1.3 YAML Updates

| File | Change | Status |
|------|--------|--------|
| `packages/core/models/views/project-layer.yaml:165` | `*L10n` → `*Content/*Generated` | [ ] |
| `packages/core/models/views/_registry.yaml:76` | `L10n` → `Content` | [ ] |
| `packages/core/models/views/entity-ecosystem.yaml:5` | `L10n` → `Content` | [ ] |

### 1.4 Rust .unwrap() → Proper Error Handling

| File | Pattern | Status |
|------|---------|--------|
| `src/parsers/market.rs` | 9 section unwraps → `ok_or()` | [ ] |
| `src/parsers/slugification.rs` | 11 caps.get() → `ok_or()` | [ ] |
| `src/parsers/formatting.rs` | 25 Regex → `LazyLock` + `expect()` | [ ] |

---

## Batch 2: Medium Priority (Naming + Dead Code)

### 2.1 Global/Tenant → Shared/Org

| File | Changes | Status |
|------|---------|--------|
| `tools/novanet/src/tui/ui/graph.rs` | Variable renames in tests | [ ] |
| `tools/novanet/src/tui/ui/status.rs` | Function renames | [ ] |
| `tools/novanet/src/tui/data.rs` | Variable renames | [ ] |
| `tools/novanet/src/tui/app.rs` | Variable renames | [ ] |
| `packages/core/models/views/seo-keywords.yaml` | Comment updates | [ ] |

### 2.2 Dead Code Removal

| Item | Action | Status |
|------|--------|--------|
| `packages/core/src/types/task-types.ts` | DELETE entire file | [ ] |
| `packages/core/src/types/index.ts:400-407` | Remove re-exports | [ ] |
| `packages/core/src/types/index.ts:34,41` | Remove commented imports | [ ] |
| `packages/core/src/types/locale-knowledge.ts:12-13` | Remove commented types | [ ] |
| `packages/core/src/schemas/locale-knowledge.schema.ts:11-14` | Remove commented schemas | [ ] |
| `packages/core/src/schemas/prompts.schema.ts:7` | Remove commented import | [ ] |

### 2.3 Hardcoded Count Updates

| File | Old | New | Status |
|------|-----|-----|--------|
| `packages/core/src/graph/generator.ts:135,144,152` | 61 | 60 | [ ] |
| `packages/core/src/graph/generator.ts:216` | 32+29 | 39+21 | [ ] |
| `packages/core/src/graph/types.ts:115,121` | 61, 43 | 60, dynamic | [ ] |
| `packages/core/src/index.ts:63` | 43+77 | 60+dynamic | [ ] |

---

## Batch 3: Low Priority (Polish)

### 3.1 Console.log Cleanup

| File | Line | Status |
|------|------|--------|
| `apps/studio/src/components/sidebar/tabs/GraphTab.tsx` | 234 | [ ] |
| `apps/studio/src/components/sidebar/TabbedDetailPanel.tsx` | 209 | [ ] |

### 3.2 Import Consistency

| File | Fix | Status |
|------|-----|--------|
| `apps/studio/src/components/sidebar/SchemaCardView.tsx:20` | Layer from /types | [ ] |
| `apps/studio/src/components/query/ResultsOverview.tsx:18` | Layer from /types | [ ] |
| `apps/studio/src/hooks/useFilteredGraph.ts:20` | Layer from /types | [ ] |
| `apps/studio/src/config/relationshipTypes.ts:7` | RelationType from /schemas | [ ] |

### 3.3 Schema Drift Fix

| Item | Action | Status |
|------|--------|--------|
| `tools/novanet/src/parsers/visual_encoding.rs` | Add `counts` field | [ ] |
| `tools/novanet/src/generators/visual_encoding.rs` | Generate COUNTS_ICONS | [ ] |

---

## Verification Checklist

- [ ] `pnpm type-check` passes
- [ ] `pnpm test` passes (core + studio)
- [ ] `cargo test --lib` passes
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `pnpm validate:design-system` passes
- [ ] `/codebase-audit` (Ralph Wiggum) clean

---

## Post-Cleanup

1. Commit: `chore: sniper cleanup v11.6.2`
2. Run Ralph Wiggum audit
3. Update CHANGELOG.md
