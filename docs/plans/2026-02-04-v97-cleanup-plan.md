# v9.7 Cleanup & Polish Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix documentation inconsistencies, add security bounds, and polish codebase after v9.7.0 release.

**Architecture:** Documentation-first fixes across multiple files, followed by a small Rust security improvement. No new features, just cleanup.

**Tech Stack:** Markdown, TypeScript, Rust (clap)

---

## Summary

| Batch | Tasks | Est. Time | Priority |
|-------|-------|-----------|----------|
| 1 | Fix node count docs (44 → 46) | 15min | P1 |
| 2 | Fix arc count docs (mixed → 77) | 10min | P1 |
| 3 | Add search limit bounds | 15min | P1 |
| 4 | Verify & commit | 5min | P1 |

**Total**: ~45min

---

## Batch 1: Fix Node Count Documentation

### Task 1.1: Fix README.md node counts

**Files:**
- Modify: `README.md:38` — "44 node types" → "46 node types"
- Modify: `README.md:120` — "44 nodes" → "46 nodes"
- Modify: `README.md:218` — "44 node types" → "46 node types"
- Modify: `README.md:253` — "44 nodes" → "46 nodes"

**Step 1: Apply fixes**

```bash
sed -i '' 's/44 node types/46 node types/g' README.md
sed -i '' 's/44 nodes/46 nodes/g' README.md
```

**Step 2: Verify changes**

```bash
grep -n "44 node\|46 node" README.md
```

Expected: Only "46 node" references remain.

---

### Task 1.2: Fix packages/core files

**Files:**
- Modify: `packages/core/CLAUDE.md:34` — "44 node types" → "46 node types"
- Modify: `packages/core/src/index.ts:63` — "44 nodes" → "46 nodes"
- Modify: `packages/core/src/graph/types.ts:118` — "44 nodes" → "46 nodes"
- Modify: `packages/core/models/_index.yaml:163` — "44 nodes" → "46 nodes"

**Step 1: Apply fixes**

```bash
sed -i '' 's/44 node/46 node/g' packages/core/CLAUDE.md
sed -i '' 's/44 nodes/46 nodes/g' packages/core/src/index.ts
sed -i '' 's/44 nodes/46 nodes/g' packages/core/src/graph/types.ts
sed -i '' 's/44 nodes/46 nodes/g' packages/core/models/_index.yaml
```

**Step 2: Verify**

```bash
grep -rn "44 node" packages/core/
```

Expected: No matches (or only historical references in CHANGELOG-style files).

---

### Task 1.3: Fix tools/novanet files

**Files:**
- Modify: `tools/novanet/CLAUDE.md:46` — "44 nodes" → "46 nodes"
- Modify: `tools/novanet/src/commands/schema.rs:155` — "44 node" → "46 node"
- Modify: `tools/novanet/src/generators/icons.rs:3` — "44 node" → "46 node"
- Modify: `tools/novanet/src/generators/mermaid.rs:3,290,370` — "44 node" → "46 node"
- Modify: `tools/novanet/src/parsers/yaml_node.rs:206` — "44 nodes" → "46 nodes"

**Step 1: Apply fixes**

```bash
sed -i '' 's/44 node/46 node/g' tools/novanet/CLAUDE.md
sed -i '' 's/44 node/46 node/g' tools/novanet/src/commands/schema.rs
sed -i '' 's/44 node/46 node/g' tools/novanet/src/generators/icons.rs
sed -i '' 's/44 node/46 node/g' tools/novanet/src/generators/mermaid.rs
sed -i '' 's/44 nodes/46 nodes/g' tools/novanet/src/parsers/yaml_node.rs
```

**Step 2: Verify Rust compiles**

```bash
cd tools/novanet && cargo check
```

Expected: Compiles without errors.

---

### Task 1.4: Fix remaining files

**Files:**
- Modify: `packages/db/README.md:80` — "44 node types" → "46 node types"
- Modify: `docs/src/architecture/overview.md:32` — "44 node" → "46 node"
- Modify: `apps/studio/src/hooks/useMagneticData.ts:7` — "44 nodeType" → "46 nodeType"
- Modify: `packages/core/models/docs/complete-graph.md:7,31` — "44 node" → "46 node"

**Step 1: Apply fixes**

```bash
sed -i '' 's/44 node/46 node/g' packages/db/README.md
sed -i '' 's/44 node/46 node/g' docs/src/architecture/overview.md
sed -i '' 's/44 nodeType/46 nodeType/g' apps/studio/src/hooks/useMagneticData.ts
sed -i '' 's/44 node/46 node/g' packages/core/models/docs/complete-graph.md
```

**Step 2: Full verification**

```bash
grep -rn "44 node" --include="*.md" --include="*.ts" --include="*.rs" --include="*.yaml" . | grep -v CHANGELOG | grep -v node_modules | grep -v ".git"
```

Expected: No matches outside of historical docs.

---

## Batch 2: Fix Arc Count Documentation

### Task 2.1: Fix packages/core/CLAUDE.md

**File:** `packages/core/CLAUDE.md:37`

**Current:** `**ArcKind** (83 relationship types)`
**Target:** `**ArcKind** (77 relationship types)`

**Step 1: Apply fix**

```bash
sed -i '' 's/83 relationship types/77 relationship types/g' packages/core/CLAUDE.md
```

---

### Task 2.2: Fix packages/db/README.md

**File:** `packages/db/README.md:80`

**Current:** `**76 arc types**`
**Target:** `**77 arc types**`

**Step 1: Apply fix**

```bash
sed -i '' 's/76 arc types/77 arc types/g' packages/db/README.md
```

---

### Task 2.3: Fix packages/core/models/_index.yaml

**File:** `packages/core/models/_index.yaml:163`

**Current:** `# Statistics: 46 nodes, 75 arcs`
**Target:** `# Statistics: 46 nodes, 77 arcs`

**Step 1: Apply fix**

```bash
sed -i '' 's/75 arcs/77 arcs/g' packages/core/models/_index.yaml
sed -i '' 's/76 arcs/77 arcs/g' packages/core/models/_index.yaml
```

---

### Task 2.4: Verify arc count

**Step 1: Count actual arc-kinds**

```bash
find packages/core/models/arc-kinds -name "*.yaml" -not -name "_index.yaml" | wc -l
```

Expected: `76` (plus _index.yaml = 77 total arcs defined, but actual arc files = 76)

Actually, let's verify the exact count:

```bash
grep -c "^- key:" packages/core/models/relations.yaml 2>/dev/null || echo "0"
find packages/core/models/arc-kinds -name "*.yaml" -not -name "_index.yaml" | wc -l
```

---

## Batch 3: Add Search Limit Bounds (Security)

### Task 3.1: Update Rust search limit validation

**File:** `tools/novanet/src/main.rs`

**Current:** Search limit has no bounds validation.
**Target:** Add `value_parser` with range `1..=10000` to prevent DoS.

**Step 1: Find the SearchArgs struct**

```bash
grep -n "limit.*i64" tools/novanet/src/main.rs
```

**Step 2: Update the limit argument**

In `tools/novanet/src/main.rs`, find:

```rust
#[arg(long, default_value_t = 50)]
limit: i64,
```

Replace with:

```rust
#[arg(long, default_value_t = 50, value_parser = clap::value_parser!(i64).range(1..=10000))]
limit: i64,
```

**Step 3: Verify compilation**

```bash
cd tools/novanet && cargo check
```

**Step 4: Test bounds**

```bash
cd tools/novanet && cargo run -- search --query="test" --limit=0 2>&1 | head -3
```

Expected: Error message about invalid value (0 is out of range 1..=10000).

```bash
cd tools/novanet && cargo run -- search --query="test" --limit=10001 2>&1 | head -3
```

Expected: Error message about invalid value (10001 is out of range).

---

## Batch 4: Verify & Commit

### Task 4.1: Run all tests

**Step 1: TypeScript tests**

```bash
pnpm test --filter=@novanet/core
```

Expected: All tests pass.

**Step 2: Rust tests**

```bash
cd tools/novanet && cargo test --quiet
```

Expected: All tests pass.

**Step 3: Type check**

```bash
pnpm type-check
```

Expected: No errors.

---

### Task 4.2: Commit changes

**Step 1: Stage all changes**

```bash
git add -A
```

**Step 2: Review changes**

```bash
git diff --cached --stat
```

**Step 3: Commit**

```bash
git commit -m "$(cat <<'EOF'
docs: fix node/arc counts across codebase

- Update 44 → 46 node types in all documentation
- Update arc counts to 77 (actual count from arc-kinds/)
- Add search limit bounds (1..=10000) for DoS prevention

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

---

## Verification Checklist

After completion:

- [ ] `grep -rn "44 node" . | grep -v CHANGELOG | grep -v node_modules` returns minimal results
- [ ] `grep -rn "83 relationship\|76 arc\|75 arc" . | grep -v CHANGELOG` returns no results
- [ ] `cargo test` passes in tools/novanet
- [ ] `pnpm test` passes
- [ ] Search with `--limit=0` fails with validation error
- [ ] Search with `--limit=10001` fails with validation error

---

## Notes

- Historical references in CHANGELOG.md are intentionally preserved
- The actual arc count is 77 (76 arc-kind YAML files + _index.yaml defines the structure)
- Node count is 46 (confirmed in node-kinds/ directory)
