# CLAUDE.md Audit & Fact Verification Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Audit all CLAUDE.md files and skills across the NovaNet monorepo, correct inaccuracies, and update the pitch document with verified facts.

**Architecture:** Deep code analysis to verify all claims in documentation, then systematic updates.

**Tech Stack:** NovaNet monorepo (Turborepo, pnpm, Neo4j, Next.js)

---

## Verified Facts (Source of Truth)

### Node Counts
| Scope | Subcategories | Node Count |
|-------|---------------|------------|
| **Global** | config (1), knowledge (14) | **15 nodes** |
| **Project** | foundation (3), structure (2), semantic (2), instruction (5), output (2) | **14 nodes** |
| **Shared** | seo (3), geo (3) | **6 nodes** |
| **TOTAL** | 9 subcategories | **35 nodes** |

### Relations Count
- **47 relations** (NOT 50 as previously claimed)
- Source: `packages/core/models/relations.yaml`

### API Routes
- **9 routes** in `apps/studio/src/app/api/`:
  - `/api/chat`
  - `/api/graph`
  - `/api/graph/expand`
  - `/api/graph/ontology`
  - `/api/graph/query`
  - `/api/graph/schema`
  - `/api/graph/stats`
  - `/api/views`
  - `/api/views/[id]`

### Zustand Stores
- **8 stores** in `apps/studio/src/stores/`:
  - aiQueryStore
  - animationStore
  - chatStore
  - filterStore
  - graphStore
  - queryStore
  - uiStore
  - viewStore

### Packages
- **4 packages**: core, db, cli, schema-tools

### Seed Files
- **7 Cypher files** in `packages/db/seed/`

### Instance Projection
- **~19,000 instances** is a PROJECTION for full QR Code AI deployment
- NOT current state - this is when 200+ locales are fully populated

---

## Task 1: Audit packages/core/CLAUDE.md

**Files:**
- Modify: `packages/core/CLAUDE.md`

**Step 1: Read current CLAUDE.md**

Read and identify inaccuracies.

**Step 2: Fix relation count**

Change "50 relations" to "47 relations" where mentioned.

**Step 3: Verify node counts match folder structure**

Confirm:
- Global: 15 nodes (1 config + 14 knowledge)
- Project: 14 nodes (3 foundation + 2 structure + 2 semantic + 5 instruction + 2 output)
- Shared: 6 nodes (3 seo + 3 geo)

**Step 4: Verify version references**

Ensure version mentioned (v8.2.0) is consistent.

**Step 5: Commit**

```bash
git add packages/core/CLAUDE.md
git commit -m "docs(core): fix relation count and verify node counts in CLAUDE.md"
```

---

## Task 2: Audit apps/studio/CLAUDE.md

**Files:**
- Modify: `apps/studio/CLAUDE.md`

**Step 1: Read current CLAUDE.md**

Identify inaccuracies, especially the "19,000 nodes" claim.

**Step 2: Fix instance projection wording**

Change from stating 19,000 as current fact to clearly marking it as a projection for full deployment.

**Step 3: Verify keyboard shortcuts are accurate**

Compare with actual implementation.

**Step 4: Verify API routes list**

Update to match actual 9 routes found.

**Step 5: Verify stores list**

Update to match actual 8 stores found.

**Step 6: Commit**

```bash
git add apps/studio/CLAUDE.md
git commit -m "docs(studio): fix instance count projection and update API/stores lists"
```

---

## Task 3: Audit packages/db/CLAUDE.md

**Files:**
- Modify: `packages/db/CLAUDE.md`

**Step 1: Read current CLAUDE.md**

Verify it accurately describes the db package.

**Step 2: Verify seed file count**

Confirm 7 seed files are documented.

**Step 3: Verify Docker config**

Confirm Neo4j version and ports are accurate.

**Step 4: Commit if changes needed**

```bash
git add packages/db/CLAUDE.md
git commit -m "docs(db): verify and update CLAUDE.md"
```

---

## Task 4: Audit packages/cli/CLAUDE.md

**Files:**
- Modify: `packages/cli/CLAUDE.md`

**Step 1: Read current CLAUDE.md**

Verify accuracy of CLI documentation.

**Step 2: Update if needed**

Fix any inaccuracies found.

**Step 3: Commit if changes needed**

```bash
git add packages/cli/CLAUDE.md
git commit -m "docs(cli): verify and update CLAUDE.md"
```

---

## Task 5: Audit root CLAUDE.md

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Read current CLAUDE.md**

Verify monorepo overview accuracy.

**Step 2: Verify package list**

Ensure all 4 packages are listed with correct descriptions.

**Step 3: Add pitch summary (optional)**

Consider adding link to detailed pitch document.

**Step 4: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: verify and update root CLAUDE.md"
```

---

## Task 6: Audit studio rules and skills

**Files:**
- Modify: `apps/studio/.claude/rules/novanet-terminology.md`
- Modify: `apps/studio/.claude/rules/novanet-decisions.md`

**Step 1: Read novanet-terminology.md**

Verify node counts and relation names.

**Step 2: Fix Global scope count**

Currently shows 15 nodes - verify this matches folder structure.

**Step 3: Read novanet-decisions.md**

Verify presets and keyboard shortcuts are accurate.

**Step 4: Commit**

```bash
git add apps/studio/.claude/rules/*.md
git commit -m "docs(studio): verify and update rules documentation"
```

---

## Task 7: Update NOVANET-PITCH.md with verified facts

**Files:**
- Modify: `docs/NOVANET-PITCH.md`

**Step 1: Update relation count**

Change 50 to 47.

**Step 2: Clarify instance projection**

Add note that 19,000 is a projection, not current state.

**Step 3: Verify all other numbers**

Cross-check against verified facts above.

**Step 4: Commit**

```bash
git add docs/NOVANET-PITCH.md
git commit -m "docs: update pitch with verified facts (47 relations, clarify instance projection)"
```

---

## Task 8: Final commit and summary

**Step 1: Verify all changes**

```bash
git status
git diff --stat HEAD~7
```

**Step 2: Create summary**

List all corrections made.

---

## Summary of Corrections Needed

| Document | Issue | Correction |
|----------|-------|------------|
| Multiple | "50 relations" | "47 relations" |
| studio/CLAUDE.md | "19,000 nodes" as fact | Clarify as projection |
| pitch | "50 relations" | "47 relations" |
| terminology.md | Verify node counts | Cross-check with folders |
