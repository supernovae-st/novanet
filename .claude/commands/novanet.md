---
description: Start a NovaNet Studio development session with context loading and quick actions
allowed-tools: Bash, Read, Glob
---

# NovaNet Studio Session Start

Initialize a development session in the NovaNet monorepo.

## Workflow

### Step 1: Greet and Check Git Status

```bash
git status --short
git log --oneline -3
```

Report current branch, uncommitted changes, recent commits.

### Step 2: Run Type Check

```bash
pnpm type-check
```

If type errors exist, report them before proceeding.

### Step 3: Load Context

Read essential context files:
- `CLAUDE.md` - Project conventions
- `ROADMAP.md` - Current priorities
- `CHANGELOG.md` - Recent changes

### Step 4: Show Quick Actions Menu

```
+===============================================================================+
|                         NOVANET QUICK ACTIONS                                 |
+===============================================================================+

  [1] /novanet-arch          View architecture diagrams
  [2] /novanet-sync          Validate/regenerate from YAML
  [3] /schema status         Schema statistics (61 nodes, 169 arcs)
  [4] /ontology-audit        Full synchronization audit

  Schema:
  [5] /schema:add-node       Add new node type (Socratic)
  [6] /schema:edit-node      Modify existing node
  [7] /schema:add-arc        Add new arc type

  ADRs:
  [8] /adr must-know         Essential v0.13.0 ADRs
  [9] /adr list              All 32 ADRs

  End:
  [0] /novanet-bye           Clean up and sign off

+===============================================================================+
```

## Example Session Start

```
Hello! Starting NovaNet Studio session.

Git Status:
  Branch: feature/new-node
  3 uncommitted changes

Type Check: Passing

Context Loaded:
  - v0.13.0 (*Native Pattern)
  - 61 nodes, 169 arcs, 10 layers

What would you like to work on?
```
