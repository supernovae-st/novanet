---
name: codebase-audit
description: Comprehensive codebase audit for dead code, inconsistencies, and legacy patterns. Use before releases, after refactoring, or for periodic maintenance. AKA "Ralph Wiggum Loop".
user-invocable: true
---

# Codebase Audit (Ralph Wiggum Loop)

Systematic parallel analysis of codebase health using multiple specialized agents.

## When to Use

- Before major releases (ensure clean state)
- After large refactoring (verify no regressions)
- When inheriting unfamiliar codebase (understand health)
- Periodic maintenance (monthly/quarterly)
- After dependency updates (check for breakage)

## Quick Reference

| Phase | Action | Tool | Model |
|-------|--------|------|-------|
| **SCAN** | Launch parallel agents | Task (Explore) | haiku |
| **SYNTHESIZE** | Combine findings | TodoWrite | - |
| **FIX** | Apply corrections | Edit/Bash | - |
| **VERIFY** | Re-run scan | Task (Explore) | haiku |

## Process

```
┌─────────────────────────────────────────────────────────────┐
│                     CODEBASE AUDIT                          │
├─────────────────────────────────────────────────────────────┤
│  SCAN (parallel)                                            │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐           │
│  │Dead Code│ │Pkg.json │ │TSConfig │ │CLAUDE.md│           │
│  │ Agent   │ │ Agent   │ │ Agent   │ │ Agent   │           │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘           │
│       │           │           │           │                 │
│       └───────────┴─────┬─────┴───────────┘                 │
│                         │                                   │
│  SYNTHESIZE ────────────▼─────────────────────────────────  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ Prioritize: CRITICAL → HIGH → MEDIUM → LOW             ││
│  │ Group by category, create TodoWrite items              ││
│  └─────────────────────────────────────────────────────────┘│
│                         │                                   │
│  FIX ───────────────────▼─────────────────────────────────  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ Apply fixes, run tests, commit                         ││
│  └─────────────────────────────────────────────────────────┘│
│                         │                                   │
│  VERIFY ────────────────▼─────────────────────────────────  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ Re-run scan → Loop until CLEAN                         ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Phase 1: SCAN (Parallel Agents)

Launch **4-6 parallel Task agents** in a **single message** with multiple tool calls.

**CRITICAL**: All Task calls must be in ONE message for parallel execution.

### Tool Parameters

| Parameter | Value |
|-----------|-------|
| `subagent_type` | `Explore` |
| `model` | `haiku` |
| `description` | Short 3-5 word summary |
| `prompt` | Agent-specific analysis prompt |

### Agent 1: Dead Code Analysis

**Description**: `Find dead code exports`

**Prompt**:
```
Analyze the codebase for dead code:
1. Exports in index.ts files never imported elsewhere
2. Functions/classes defined but never used
3. Dead imports (imported but never used)
4. References to deleted/moved files

Focus on: src/**/*.ts, src/**/*.tsx
Report each finding with file:line format.
```

### Agent 2: Package.json Audit

**Description**: `Audit package scripts`

**Prompt**:
```
Review ALL package.json files for:
1. Scripts that reference non-existent files
2. Scripts never called (not in turbo, CI, or other scripts)
3. Unused dependencies
4. Version inconsistencies between packages
5. Missing catalog: usage where expected

Check: root, all packages/*, all apps/*
List findings with evidence.
```

### Agent 3: TypeScript Config Audit

**Description**: `Verify tsconfig consistency`

**Prompt**:
```
Analyze ALL tsconfig.json files for:
1. Path aliases pointing to wrong locations
2. Missing baseUrl where paths are used
3. Inconsistent compiler options between packages
4. References to non-existent files
5. Outdated or conflicting settings

Report inconsistencies with file paths.
```

### Agent 4: Documentation Accuracy

**Description**: `Verify CLAUDE.md accuracy`

**Prompt**:
```
Cross-reference CLAUDE.md files with codebase:
1. Commands that don't exist
2. Files/exports mentioned that don't exist
3. Counts that are wrong (e.g., "32 types" when 25)
4. Outdated architecture descriptions
5. Missing documentation for new packages

Report inaccuracies with evidence.
```

### Agent 5: Test Health (optional)

**Description**: `Check dead tests`

**Prompt**:
```
Search ALL test files for:
1. Skipped tests (describe.skip, it.skip, xit)
2. Tests with TODO/FIXME comments
3. Empty test blocks
4. Tests importing non-existent files
5. Dead test utilities never used

Report each finding with file path.
```

### Agent 6: Index Exports (optional)

**Description**: `Check index.ts exports`

**Prompt**:
```
Verify ALL index.ts barrel files:
1. Every export points to existing file
2. No circular dependencies
3. No exports referencing deleted files
4. Type exports match actual definitions

Report broken/dead exports with file:line.
```

## Phase 2: SYNTHESIZE

After all agents complete, combine findings:

### TodoWrite Structure

Create one todo per category with sub-items:

```typescript
TodoWrite([
  { content: "Fix CRITICAL issues (2 items)", status: "pending", activeForm: "Fixing CRITICAL issues" },
  { content: "Fix HIGH issues (3 items)", status: "pending", activeForm: "Fixing HIGH issues" },
  { content: "Clean MEDIUM issues (8 items)", status: "pending", activeForm: "Cleaning MEDIUM issues" },
  { content: "Address LOW issues (4 items)", status: "pending", activeForm: "Addressing LOW issues" },
  { content: "Run verification loop", status: "pending", activeForm: "Running verification loop" }
])
```

### Severity Classification

| Severity | Criteria | Examples |
|----------|----------|----------|
| **CRITICAL** | Breaks build/tests, undefined refs | Missing imports, broken paths |
| **HIGH** | Incorrect behavior, major inconsistency | Version mismatch, wrong docs |
| **MEDIUM** | Dead code, unused exports | Unused functions, deprecated options |
| **LOW** | Style, minor inconsistency | Missing sourceMap, cosmetic |

### Output Format

```markdown
## CRITICAL (Fix immediately)
| Issue | Location | Description |
|-------|----------|-------------|
| ... | file:line | ... |

## HIGH (Fix soon)
...

## MEDIUM (Clean up)
...

## LOW (Nice to have)
...
```

## Phase 3: FIX

For each severity level, starting with CRITICAL:

1. **Mark todo as in_progress**
2. **Apply fix** using Edit tool
3. **Run tests** to verify no regression
4. **Mark todo complete** immediately after

### Fix Guidelines

- CRITICAL: Fix immediately, one by one, test after each
- HIGH: Fix in batches of 2-3, test after batch
- MEDIUM: Can batch multiple fixes, test at end
- LOW: Optional, batch freely

### Verification Commands

```bash
pnpm type-check        # TypeScript
pnpm test              # Unit tests
pnpm lint              # Linting
pnpm build             # Full build
```

## Phase 4: VERIFY

After fixes, re-run SCAN phase:

1. **Launch same parallel agents** (copy exact prompts)
2. **Compare to previous findings**
3. **If new issues**: Go to FIX phase
4. **If clean**: Report success, mark verification todo complete

### Loop Termination

Stop when:
- All CRITICAL and HIGH issues fixed
- Tests pass
- User says "good enough"

## Example Session

```
User: Run codebase audit

Claude: Starting codebase audit...

        [23:45:01] Creating TodoWrite for scan phases...
        [23:45:02] Launching 4 parallel Explore agents (haiku)...

        [23:45:34] Agent 1 complete: Dead Code (12 findings)
        [23:45:38] Agent 2 complete: Package.json (5 findings)
        [23:45:42] Agent 3 complete: TSConfig (2 findings)
        [23:45:45] Agent 4 complete: CLAUDE.md (4 findings)

        Synthesizing results...

        ## Audit Results

        | Severity | Count |
        |----------|-------|
        | CRITICAL | 2     |
        | HIGH     | 3     |
        | MEDIUM   | 8     |
        | LOW      | 4     |

        ### CRITICAL
        1. validate-docs.ts:94 - undefined RELATIONS_PATH
        2. tsconfig.json:30 - path alias points to /dist/src/

        Shall I fix these issues?

User: fix

Claude: [23:46:01] Marking "Fix CRITICAL issues" as in_progress
        [23:46:05] Fixing validate-docs.ts:94...
        [23:46:08] Fixing tsconfig.json:30...
        [23:46:12] Running type-check... PASS
        [23:46:15] Running tests... 264 passed
        [23:46:18] Marking "Fix CRITICAL issues" as completed

        [23:46:20] Marking "Fix HIGH issues" as in_progress
        ...

        [23:48:00] Starting verification loop...
        [23:48:30] Verification complete

        ## Verification Complete

        ✅ All CRITICAL fixed
        ✅ All HIGH fixed
        ✅ Tests pass (797/797)
        ⚠️ 4 LOW remaining (cosmetic)

        Codebase is clean!
```

## Integration with Other Skills

| Skill | When to Use Together |
|-------|---------------------|
| `novanet-sync` | If audit finds YAML/TypeScript sync issues |
| `novanet-architecture` | To understand system before auditing |

## Common Findings by Category

| Category | Common Issues |
|----------|---------------|
| **Dead Code** | Unused exports, orphaned functions, stale imports |
| **Package.json** | Version drift, broken scripts, unused deps |
| **TypeScript** | Path alias errors, missing options, version mismatch |
| **Documentation** | Outdated counts, wrong commands, missing packages |
| **Tests** | Skipped tests, empty blocks, broken imports |

## Anti-Patterns to Avoid

**Sequential analysis**: Don't analyze one aspect at a time. Launch parallel agents for speed.

**Fixing without tests**: Always run tests after fixes. Even "safe" changes can break things.

**Ignoring LOW**: While not urgent, LOW issues accumulate. Clean them periodically.

**Manual verification**: Use the verification loop. Don't trust "I think it's fixed."

**Single-message agents**: Always launch all agents in ONE message for true parallelism.

## Why "Ralph Wiggum"?

Internal team reference. The verification loop keeps going until the codebase is actually clean. It's persistent and catches things others miss.
