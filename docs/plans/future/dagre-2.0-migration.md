# dagre 2.0 Migration Plan

**Status**: Deferred
**Priority**: Low
**Blocked by**: Breaking API changes in @dagrejs/dagre 2.0

## Context

In v0.14.1, we reverted `@dagrejs/dagre` from 2.0.4 to 1.1.8 due to breaking changes that caused test failures in `schemaLayoutELK.test.ts`.

## Breaking Changes in dagre 2.0

### API Changes

1. **`dagre.layout()` signature changed**
   - 1.x: `dagre.layout(g)` mutates graph in place
   - 2.0: May have different behavior or return value

2. **Graph construction**
   - Node/edge attribute handling may differ
   - Default values for `rankdir`, `ranksep`, `nodesep` may have changed

3. **TypeScript types**
   - New `@dagrejs/dagre` replaces `dagre` package
   - Type definitions may have incompatibilities

## Affected Files

- `apps/studio/src/lib/layouts/schemaLayoutELK.ts` - Main layout engine
- `apps/studio/src/lib/layouts/__tests__/schemaLayoutELK.test.ts` - Tests
- Any component using dagre-based layouts

## Migration Steps (When Prioritized)

1. **Read dagre 2.0 changelog/migration guide**
   - Check https://github.com/dagrejs/dagre for release notes
   - Identify all API changes

2. **Update layout engine**
   ```typescript
   // Review these patterns:
   const g = new dagre.graphlib.Graph();
   g.setGraph({ rankdir: 'TB', ranksep: 50, nodesep: 30 });
   g.setDefaultEdgeLabel(() => ({}));
   // ... add nodes/edges
   dagre.layout(g);
   ```

3. **Update tests**
   - `schemaLayoutELK.test.ts` needs assertion updates
   - May need to adjust expected positions/dimensions

4. **Verify visual output**
   - Test layouts in Studio manually
   - Compare before/after screenshots

## Decision

Deferring migration because:
- Current 1.1.8 works correctly
- No new features needed from 2.0
- Higher priority work exists

## Re-evaluate When

- dagre 1.x reaches EOL
- New layout features needed only in 2.0
- Major Studio refactoring planned
