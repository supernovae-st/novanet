# NovaNet Legacy Cleanup & Optimization Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove all legacy/backward compatibility code and optimize performance across MCP server, TUI, and Studio.

**Architecture:** Systematic cleanup in 4 phases: (1) Remove dead code & legacy patterns, (2) Optimize MCP async operations, (3) Optimize TUI rendering, (4) Optimize Studio React performance.

**Tech Stack:** Rust (tokio, neo4rs, moka), TypeScript (React 19, Next.js 16, Zustand), Neo4j

---

## Phase 1: Remove Legacy/Backward Compatibility Code

### Task 1.1: Remove Studio VIEW_ID_ALIASES

**Files:**
- Modify: `apps/studio/src/app/api/views/[id]/route.ts:16-26`

**Step 1: Write the failing test**

```typescript
// apps/studio/src/__tests__/api/views-no-aliases.test.ts
import { GET } from '@/app/api/views/[id]/route';

describe('Views API - No Backward Compat', () => {
  it('should NOT resolve legacy view aliases', async () => {
    const request = new Request('http://localhost/api/views/complete-graph');
    const response = await GET(request, { params: { id: 'complete-graph' } });
    // complete-graph was alias for "graph-complete"
    expect(response.status).toBe(404);
  });
});
```

**Step 2: Run test to verify it fails**

Run: `pnpm test --filter=@novanet/studio -- views-no-aliases`
Expected: FAIL (currently aliases ARE resolved)

**Step 3: Remove VIEW_ID_ALIASES**

```typescript
// Remove lines 16-26 entirely:
// const VIEW_ID_ALIASES: Record<string, string> = {
//   'complete-graph': 'graph-complete',
//   ...
// };

// Remove alias resolution in handler:
// const resolvedId = VIEW_ID_ALIASES[id] || id;
// Replace with just: const resolvedId = id;
```

**Step 4: Run test to verify it passes**

Run: `pnpm test --filter=@novanet/studio -- views-no-aliases`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/studio/src/app/api/views/[id]/route.ts apps/studio/src/__tests__/api/views-no-aliases.test.ts
git commit -m "fix(studio): remove VIEW_ID_ALIASES backward compat

BREAKING: Legacy view IDs no longer resolve to new names.
Clients must use canonical view IDs from v0.12.5+.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 1.2: Remove TUI Dead Code - InstanceInfo Structs

**Files:**
- Modify: `tools/novanet/src/tui/data.rs:3117-3165`

**Step 1: Run cargo test to establish baseline**

Run: `cd tools/novanet && cargo test`
Expected: All tests pass (baseline)

**Step 2: Remove dead code**

```rust
// Remove these structs entirely (lines ~3117-3165):
// #[allow(dead_code)]
// pub struct InstanceInfo { ... }
// #[allow(dead_code)]
// pub struct InstanceArc { ... }
// #[allow(dead_code)]
// pub struct InstanceDetail { ... }
```

**Step 3: Run cargo test**

Run: `cd tools/novanet && cargo test`
Expected: PASS (no code references these)

**Step 4: Run clippy to verify no dead code warnings**

Run: `cd tools/novanet && cargo clippy -- -D warnings`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/data.rs
git commit -m "chore(tui): remove dead InstanceInfo/InstanceArc/InstanceDetail structs

These were v10.6 Data View reserved structs, never implemented.
Reduces binary size and removes #[allow(dead_code)] annotations.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 1.3: Remove TUI Unused COLOR Constants

**Files:**
- Modify: `tools/novanet/src/tui/ui/yaml_panel.rs:43-63`

**Step 1: Identify unused constants**

Run: `cd tools/novanet && grep -n "COLOR_" src/tui/ui/yaml_panel.rs | head -20`

**Step 2: Remove unused COLOR_* constants**

```rust
// Remove lines 43-63, these 15 constants:
// const COLOR_KEY: Color = ...
// const COLOR_STRING: Color = ...
// const COLOR_NUMBER: Color = ...
// etc.
```

**Step 3: Run cargo build to verify no compile errors**

Run: `cd tools/novanet && cargo build`
Expected: PASS

**Step 4: Run cargo clippy**

Run: `cd tools/novanet && cargo clippy -- -D warnings`
Expected: PASS (no unused constant warnings)

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/ui/yaml_panel.rs
git commit -m "chore(tui): remove 15 unused COLOR_* constants from yaml_panel

These were placeholder constants for YAML syntax highlighting,
feature was deferred. Reduces code noise.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 1.4: Remove Core Package Unused Exports

**Files:**
- Modify: `packages/core/src/graph/visual-encoding.ts`

**Step 1: Identify unused exports**

Run: `cd packages/core && grep -E "^export (const|function|type)" src/graph/visual-encoding.ts | wc -l`

**Step 2: Remove unused exports (27+ items)**

Remove these unused exports:
- `ICON_*` constants (12)
- `STATE_*` constants (8)
- Unused accessor functions (7)

**Step 3: Run type-check**

Run: `pnpm type-check --filter=@novanet/core`
Expected: PASS

**Step 4: Run tests**

Run: `pnpm test --filter=@novanet/core`
Expected: PASS

**Step 5: Commit**

```bash
git add packages/core/src/graph/visual-encoding.ts
git commit -m "chore(core): remove 27 unused exports from visual-encoding

Cleanup: ICON_*, STATE_*, and unused accessor functions.
Reduces bundle size and API surface.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 1.5: Remove Commented Deprecated Types in Core

**Files:**
- Modify: `packages/core/src/types/shared.ts:11-13`

**Step 1: Remove commented types**

```typescript
// Remove these commented-out types:
// // Deprecated: Priority, Freshness moved to presentation layer
// // type Priority = 'low' | 'medium' | 'high';
// // type Freshness = 'stale' | 'fresh' | 'live';
```

**Step 2: Run type-check**

Run: `pnpm type-check --filter=@novanet/core`
Expected: PASS

**Step 3: Commit**

```bash
git add packages/core/src/types/shared.ts
git commit -m "chore(core): remove commented deprecated types

Priority/Freshness were moved to presentation layer in v8.2.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 1.6: Remove TUI Parser Backward Compat Wrapper

**Files:**
- Modify: `tools/novanet/src/parsers/taxonomy.rs:220-227`

**Step 1: Find the wrapper function**

```rust
// Remove backward compat wrapper:
// /// Backwards-compatible wrapper for taxonomy parsing
// pub fn parse_taxonomy_compat(...) -> Result<TaxonomyDoc> { ... }
```

**Step 2: Search for usages**

Run: `cd tools/novanet && grep -rn "parse_taxonomy_compat" src/`

**Step 3: Replace usages with direct call**

If any usages found, replace with `parse_taxonomy()` direct call.

**Step 4: Remove wrapper function**

**Step 5: Run tests**

Run: `cd tools/novanet && cargo test`
Expected: PASS

**Step 6: Commit**

```bash
git add tools/novanet/src/parsers/taxonomy.rs
git commit -m "chore(parsers): remove parse_taxonomy_compat wrapper

Direct parse_taxonomy() usage only. No backward compat needed.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 1.7: Remove Studio nodeColors Re-exports

**Files:**
- Modify: `apps/studio/src/design/nodeColors.ts:22,137`

**Step 1: Identify re-exports for backward compat**

```typescript
// Remove these re-exports:
// export { layerColors } from './layerColors'; // backward compat
// export { realmColors } from './realmColors'; // backward compat
```

**Step 2: Find and update usages**

Run: `grep -rn "from.*nodeColors" apps/studio/src/`

Update imports to use direct source files.

**Step 3: Remove re-exports**

**Step 4: Run type-check**

Run: `pnpm type-check --filter=@novanet/studio`
Expected: PASS

**Step 5: Commit**

```bash
git add apps/studio/src/design/nodeColors.ts
git commit -m "chore(studio): remove backward compat re-exports from nodeColors

Direct imports from layerColors.ts and realmColors.ts now required.

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Phase 2: MCP Async Optimization

### Task 2.1: Parallelize describe_stats() Queries

**Files:**
- Modify: `tools/novanet-mcp/src/tools/describe.rs:250-273`

**Step 1: Write benchmark baseline**

```rust
// benches/describe_stats.rs
#[bench]
fn bench_describe_stats(b: &mut Bencher) {
    // Setup state, measure describe_stats execution
}
```

**Step 2: Refactor to use tokio::join!()**

```rust
// Before: 5 sequential awaits
for (name, query) in queries {
    if let Some(result) = state.pool().execute_single(query, None).await? {
        // ...
    }
}

// After: tokio::join!()
let (node_count, rel_count, entity_count, locale_count, expr_count) = tokio::join!(
    state.pool().execute_single(queries[0].1, None),
    state.pool().execute_single(queries[1].1, None),
    state.pool().execute_single(queries[2].1, None),
    state.pool().execute_single(queries[3].1, None),
    state.pool().execute_single(queries[4].1, None),
);
```

**Step 3: Run tests**

Run: `cd tools/novanet-mcp && cargo test`
Expected: PASS

**Step 4: Run benchmark**

Run: `cd tools/novanet-mcp && cargo bench -- describe_stats`
Expected: ~4-5x improvement

**Step 5: Commit**

```bash
git add tools/novanet-mcp/src/tools/describe.rs
git commit -m "perf(mcp): parallelize describe_stats with tokio::join!

5 sequential Neo4j queries now run concurrently.
Expected ~4-5x latency improvement for stats endpoint.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 2.2: Parallelize assemble_* Functions

**Files:**
- Modify: `tools/novanet-mcp/src/tools/assemble.rs:168-203`

**Step 1: Identify sequential calls**

```rust
// Current sequential pattern (lines 168-203):
if include_entities {
    let entity_evidence = assemble_entities(...).await?;
    // ...
}
if include_knowledge {
    let knowledge_evidence = assemble_knowledge(...).await?;
    // ...
}
if include_structure {
    let structure_evidence = assemble_structure(...).await?;
    // ...
}
```

**Step 2: Refactor to parallel execution**

```rust
// Parallel with tokio::join!
let (entity_result, knowledge_result, structure_result) = tokio::join!(
    async {
        if include_entities {
            assemble_entities(state, &params.focus_key, &params.locale, max_depth).await
        } else {
            Ok(Vec::new())
        }
    },
    async {
        if include_knowledge {
            assemble_knowledge(state, &params.focus_key, &params.locale).await
        } else {
            Ok(Vec::new())
        }
    },
    async {
        if include_structure {
            assemble_structure(state, &params.focus_key, max_depth).await
        } else {
            Ok(Vec::new())
        }
    },
);

let entity_evidence = entity_result?;
let knowledge_evidence = knowledge_result?;
let structure_evidence = structure_result?;
```

**Step 3: Run tests**

Run: `cd tools/novanet-mcp && cargo test`
Expected: PASS

**Step 4: Commit**

```bash
git add tools/novanet-mcp/src/tools/assemble.rs
git commit -m "perf(mcp): parallelize assemble_* functions

Entity, knowledge, and structure assembly now run concurrently.
~3x latency improvement for context assembly.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 2.3: Fix Cache Double Serialization

**Files:**
- Modify: `tools/novanet-mcp/src/tools/query.rs:77-79`

**Step 1: Identify double serialization**

```rust
// Current pattern:
let json_string = serde_json::to_string(&result)?;  // First serialization
cache.insert(cache_key, result.clone());            // Cloning for cache
return Ok(json_string);                             // Second string
```

**Step 2: Refactor to serialize once**

```rust
// Cache the serialized string directly
let json_string = serde_json::to_string(&result)?;
cache.insert(cache_key, json_string.clone());
return Ok(json_string);
```

**Step 3: Update cache type signature if needed**

Change cache from `Cache<String, QueryResult>` to `Cache<String, String>`.

**Step 4: Run tests**

Run: `cd tools/novanet-mcp && cargo test`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet-mcp/src/tools/query.rs tools/novanet-mcp/src/cache/mod.rs
git commit -m "perf(mcp): eliminate double serialization in query cache

Cache stores serialized JSON strings directly.
Reduces allocations and CPU for cached responses.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 2.4: Optimize Cache Key Hashing

**Files:**
- Modify: `tools/novanet-mcp/src/cache/mod.rs:58-74`

**Step 1: Identify expensive hash computation**

```rust
// Current: v.to_string() for each param
fn compute_cache_key(query: &str, params: &Option<Map<String, Value>>) -> String {
    let param_str = params.as_ref().map(|p| {
        p.iter().map(|(k, v)| format!("{}={}", k, v.to_string())).collect::<Vec<_>>().join("&")
    });
    format!("{}:{}", query, param_str.unwrap_or_default())
}
```

**Step 2: Use serde_json::to_string for params**

```rust
fn compute_cache_key(query: &str, params: &Option<Map<String, Value>>) -> String {
    match params {
        Some(p) => format!("{}:{}", query, serde_json::to_string(p).unwrap_or_default()),
        None => query.to_string(),
    }
}
```

**Step 3: Run tests**

Run: `cd tools/novanet-mcp && cargo test`
Expected: PASS

**Step 4: Commit**

```bash
git add tools/novanet-mcp/src/cache/mod.rs
git commit -m "perf(mcp): optimize cache key computation

Use serde_json::to_string instead of manual param formatting.
More efficient and consistent hashing.

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Phase 3: TUI Optimization

### Task 3.1: Replace .clone() with Borrowed Refs in Tree Rendering

**Files:**
- Modify: `tools/novanet/src/tui/ui/tree.rs:100-230`

**Step 1: Identify clone hotspots**

Run: `grep -n "\.clone()" tools/novanet/src/tui/ui/tree.rs | head -30`

**Step 2: Replace string clones with references**

```rust
// Before:
let name = item.name.clone();
let kind = item.kind.clone();

// After:
let name = &item.name;
let kind = &item.kind;
```

**Step 3: Update function signatures to accept references**

```rust
// Before:
fn render_item(name: String, kind: String) -> Span

// After:
fn render_item<'a>(name: &'a str, kind: &'a str) -> Span<'a>
```

**Step 4: Run tests**

Run: `cd tools/novanet && cargo test`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet/src/tui/ui/tree.rs
git commit -m "perf(tui): replace .clone() with borrowed refs in tree rendering

20+ string clones eliminated from hot render path.
Reduces allocations per frame.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 3.2: Add RenderCache for Realm Stats

**Files:**
- Modify: `tools/novanet/src/tui/ui/mod.rs`
- Create: `tools/novanet/src/tui/ui/render_cache.rs`

**Step 1: Create RenderCache struct**

```rust
// tools/novanet/src/tui/ui/render_cache.rs
use std::collections::HashMap;

pub struct RenderCache<T> {
    cache: HashMap<String, T>,
    generation: u64,
}

impl<T: Clone> RenderCache<T> {
    pub fn new() -> Self {
        Self { cache: HashMap::new(), generation: 0 }
    }

    pub fn get_or_compute<F>(&mut self, key: &str, compute: F) -> &T
    where
        F: FnOnce() -> T,
    {
        if !self.cache.contains_key(key) {
            self.cache.insert(key.to_string(), compute());
        }
        self.cache.get(key).unwrap()
    }

    pub fn invalidate(&mut self) {
        self.cache.clear();
        self.generation += 1;
    }
}
```

**Step 2: Use RenderCache for realm/layer stats**

**Step 3: Run tests**

Run: `cd tools/novanet && cargo test`
Expected: PASS

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/ui/render_cache.rs tools/novanet/src/tui/ui/mod.rs
git commit -m "perf(tui): add RenderCache for realm/layer stats

Caches computed stats across frames until data changes.
Significant reduction in per-frame computation.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 3.3: Optimize Fuzzy Search with Prefix Caching

**Files:**
- Modify: `tools/novanet/src/tui/app.rs:722-800`

**Step 1: Identify current search implementation**

**Step 2: Add prefix-based caching**

```rust
struct SearchCache {
    last_query: String,
    last_results: Vec<SearchResult>,
}

impl SearchCache {
    fn search(&mut self, query: &str, items: &[Item]) -> &[SearchResult] {
        // If new query is prefix of old, filter existing results
        if query.starts_with(&self.last_query) && !self.last_results.is_empty() {
            self.last_results.retain(|r| r.matches(query));
        } else {
            // Full search needed
            self.last_results = full_fuzzy_search(query, items);
        }
        self.last_query = query.to_string();
        &self.last_results
    }
}
```

**Step 3: Run tests**

Run: `cd tools/novanet && cargo test`
Expected: PASS

**Step 4: Commit**

```bash
git add tools/novanet/src/tui/app.rs
git commit -m "perf(tui): add prefix caching to fuzzy search

Typing additional characters filters existing results instead of re-searching.
Much faster incremental search UX.

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Phase 4: Studio React Optimization

### Task 4.1: Add useMemo to DataTab Edge Filtering

**Files:**
- Modify: `apps/studio/src/components/sidebar/tabs/DataTab.tsx:129-150`

**Step 1: Write test for memoization**

```typescript
// apps/studio/src/__tests__/components/DataTab.perf.test.tsx
import { renderHook } from '@testing-library/react';

test('edge filtering should be memoized', () => {
  // Test that re-renders don't recompute edges
});
```

**Step 2: Add useMemo**

```typescript
// Before:
const filteredEdges = edges.filter(e => e.source === selectedNode?.id || e.target === selectedNode?.id);

// After:
const filteredEdges = useMemo(() => {
  if (!selectedNode) return [];
  return edges.filter(e => e.source === selectedNode.id || e.target === selectedNode.id);
}, [edges, selectedNode?.id]);
```

**Step 3: Run tests**

Run: `pnpm test --filter=@novanet/studio -- DataTab`
Expected: PASS

**Step 4: Commit**

```bash
git add apps/studio/src/components/sidebar/tabs/DataTab.tsx
git commit -m "perf(studio): memoize edge filtering in DataTab

Prevents recomputation on unrelated state changes.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 4.2: Extract Shared Node Lookup in page.tsx

**Files:**
- Modify: `apps/studio/src/app/page.tsx:273-347`

**Step 1: Identify repeated getNodeById calls**

6 calls to `getNodeById` with same nodeId.

**Step 2: Extract to single lookup**

```typescript
// Before: 6 separate calls
const node1 = getNodeById(nodeId);
// ... more logic
const node2 = getNodeById(nodeId);
// etc.

// After: single lookup, reuse result
const selectedNode = useMemo(() => {
  return nodeId ? getNodeById(nodeId) : null;
}, [nodeId, getNodeById]);

// Then use selectedNode throughout
```

**Step 3: Run tests**

Run: `pnpm test --filter=@novanet/studio`
Expected: PASS

**Step 4: Commit**

```bash
git add apps/studio/src/app/page.tsx
git commit -m "perf(studio): extract shared node lookup in page.tsx

6 getNodeById calls reduced to 1 memoized lookup.
Eliminates redundant store subscriptions.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 4.3: Throttle QueryPill Matrix Effects

**Files:**
- Modify: `apps/studio/src/components/query/QueryPill.tsx:31-105`

**Step 1: Identify animation intervals**

```typescript
// Current: 2 matrix effects at 50ms intervals
useEffect(() => {
  const interval = setInterval(() => {
    // matrix rain effect
  }, 50);
  return () => clearInterval(interval);
}, []);
```

**Step 2: Reduce frequency and consolidate**

```typescript
// After: single effect at 100ms
const matrixEffect = useCallback(() => {
  // combined matrix logic
}, []);

useEffect(() => {
  const interval = setInterval(matrixEffect, 100);
  return () => clearInterval(interval);
}, [matrixEffect]);
```

**Step 3: Run tests**

Run: `pnpm test --filter=@novanet/studio -- QueryPill`
Expected: PASS

**Step 4: Commit**

```bash
git add apps/studio/src/components/query/QueryPill.tsx
git commit -m "perf(studio): throttle QueryPill matrix effects

2 effects at 50ms -> 1 effect at 100ms.
50% reduction in animation CPU usage.

Co-Authored-By: Nika <agent@nika.sh>
```

---

### Task 4.4: Add Debounce to SchemaCardView Search

**Files:**
- Modify: `apps/studio/src/components/schema/SchemaCardView.tsx`

**Step 1: Add useDebouncedValue hook**

```typescript
import { useDebouncedValue } from '@/hooks/useDebouncedValue';

// Before:
const filteredCards = cards.filter(c => c.name.includes(searchQuery));

// After:
const debouncedQuery = useDebouncedValue(searchQuery, 150);
const filteredCards = useMemo(() => {
  if (!debouncedQuery) return cards;
  return cards.filter(c => c.name.toLowerCase().includes(debouncedQuery.toLowerCase()));
}, [cards, debouncedQuery]);
```

**Step 2: Run tests**

Run: `pnpm test --filter=@novanet/studio -- SchemaCardView`
Expected: PASS

**Step 3: Commit**

```bash
git add apps/studio/src/components/schema/SchemaCardView.tsx
git commit -m "perf(studio): add debounce to SchemaCardView search

150ms debounce prevents filtering on every keystroke.
Smoother search UX for large schema lists.

Co-Authored-By: Nika <agent@nika.sh>
```

---

## Phase 5: Final Verification

### Task 5.1: Run Full Test Suite

**Step 1: Run all tests**

```bash
# Rust tests
cd tools/novanet && cargo test
cd tools/novanet-mcp && cargo test

# TypeScript tests
pnpm test

# Type check
pnpm type-check

# Lint
pnpm lint
```

**Step 2: Verify all pass**

Expected: All tests pass, no lint errors, no type errors

**Step 3: Commit if any fixes needed**

---

### Task 5.2: Run Benchmarks

**Step 1: MCP benchmarks**

```bash
cd tools/novanet-mcp && cargo bench
```

**Step 2: Document improvements**

Record before/after for:
- describe_stats latency
- assemble context latency
- cache hit rate

**Step 3: Update CHANGELOG**

Add performance improvements to CHANGELOG.md

---

### Task 5.3: Create Summary Commit

**Step 1: Review all changes**

```bash
git log --oneline -20
```

**Step 2: Verify no regressions**

**Step 3: Tag release if appropriate**

```bash
git tag -a v0.12.5 -m "Legacy cleanup + performance optimization"
```

---

## Summary

| Phase | Tasks | Impact |
|-------|-------|--------|
| Phase 1 | 7 tasks | Remove ~50 legacy patterns, reduce code size |
| Phase 2 | 4 tasks | ~4-5x latency improvement for MCP endpoints |
| Phase 3 | 3 tasks | ~35-50% memory reduction for TUI |
| Phase 4 | 4 tasks | Significant FPS improvement for Studio |
| Phase 5 | 3 tasks | Verification and documentation |

**Total: 21 tasks**
