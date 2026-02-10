---
description: Comprehensive codebase audit for dead code, inconsistencies, and legacy patterns. Use before releases, after refactoring, or for periodic maintenance. AKA "Ralph Wiggum Loop".
argument-hint: [full|quick|yaml|rust|typescript|security|docs] [--fix]
allowed-tools: Read, Write, Edit, Glob, Grep, Bash, Task, TodoWrite
---

# Codebase Audit (Ralph Wiggum Loop)

Multi-agent parallel audit system for comprehensive codebase health analysis.

---

## Overview

```
+===============================================================================+
|  RALPH WIGGUM LOOP - Multi-Agent Codebase Audit                               |
+===============================================================================+
|                                                                               |
|  10 PARALLEL AGENTS --> SYNTHESIS --> FIX --> VERIFY --> LOOP                 |
|                                                                               |
|  ┌───────────────────────────────────────────────────────────────────────┐    |
|  │  SCAN PHASE (10 agents in parallel)                                   │    |
|  │                                                                       │    |
|  │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                              │    |
|  │  │YAML │ │SYNC │ │RUST │ │TEST │ │DOCS │                              │    |
|  │  └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘                              │    |
|  │     │       │       │       │       │                                  │    |
|  │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐                              │    |
|  │  │DEPS │ │PERF │ │SECU │ │DEAD │ │SEMA │                              │    |
|  │  └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘ └──┬──┘                              │    |
|  │     │       │       │       │       │                                  │    |
|  │     └───────┴───────┴───┬───┴───────┘                                  │    |
|  │                         │                                              │    |
|  │  SYNTHESIZE ────────────▼──────────────────────────────────────────    │    |
|  │  Aggregate findings, classify by severity                              │    |
|  │                         │                                              │    |
|  │  FIX ───────────────────▼──────────────────────────────────────────    │    |
|  │  CRITICAL --> HIGH --> MEDIUM (with tests)                             │    |
|  │                         │                                              │    |
|  │  VERIFY ────────────────▼──────────────────────────────────────────    │    |
|  │  Re-scan until clean or user stops                                     │    |
|  └───────────────────────────────────────────────────────────────────────┘    |
|                                                                               |
+===============================================================================+
```

---

## Quick Reference

| Mode | Agents | Focus | Time |
|------|--------|-------|------|
| `full` | 10 | Complete audit | ~5 min |
| `quick` | 4 | Essential checks only | ~1 min |
| `yaml` | 2 | YAML schema + sync | ~30 sec |
| `rust` | 2 | Rust quality + tests | ~30 sec |
| `typescript` | 2 | TypeScript + dead code | ~30 sec |
| `security` | 2 | Security + deps | ~30 sec |
| `docs` | 2 | Documentation accuracy | ~30 sec |

---

## Workflow

### Step 0: Parse Arguments

Based on `$ARGUMENTS`:

| Argument | Action |
|----------|--------|
| `full` or empty | Run all 10 agents |
| `quick` | Agents 1, 2, 4, 8 only (essential) |
| `yaml` | Agents 1, 2 only (YAML focus) |
| `rust` | Agents 3, 4 only (Rust focus) |
| `typescript` | Agents 9, 10 only (TypeScript focus) |
| `security` | Agents 6, 8 only (Security focus) |
| `docs` | Agents 5, 10 only (Documentation focus) |
| `--fix` | Auto-fix issues (append to any mode) |

---

## Phase 1: SCAN (Parallel Agents)

**CRITICAL**: Launch ALL agents in a SINGLE message with multiple Task tool calls.

Each agent uses:
- `subagent_type`: Explore
- `model`: haiku (fast, cost-effective)

### Agent 1: YAML Schema Validation

**Description**: `Validate YAML schema coherence`

**Prompt**:
```
Analyze NovaNet YAML schema for issues:

1. Run: cargo run -- schema validate
   Report any validation failures

2. Check node-kinds/**/*.yaml:
   - realm field matches file path (shared vs org)
   - layer field matches file path
   - trait is valid (invariant|localized|knowledge|generated|aggregated)
   - All referenced arcs exist in arc-kinds/

3. Check arc-kinds/**/*.yaml:
   - source/target node types exist
   - family is valid (ownership|localization|semantic|generation|mining)
   - cardinality is valid

4. Verify taxonomy.yaml:
   - All realms have colors
   - All layers have colors
   - All traits have border styles

Report findings with file:line format.
Path: packages/core/models/
```

### Agent 2: Generated Artifacts Sync

**Description**: `Check YAML-TypeScript-Cypher sync`

**Prompt**:
```
Verify generated artifacts are in sync with YAML:

1. Run: cargo run -- schema validate
   If YAML changed since last generate, report it

2. Compare TypeScript with YAML:
   - packages/core/src/graph/visual-encoding.ts matches visual-encoding.yaml
   - packages/core/src/graph/taxonomy.ts matches taxonomy.yaml
   - Node counts match between YAML and TS

3. Compare Cypher with YAML:
   - packages/db/seed/*.cypher matches YAML definitions
   - All node labels in Cypher exist in node-kinds/

4. Check Rust generated files:
   - tools/novanet/src/tui/icons.rs matches visual-encoding.yaml icons

Report any drift with specific file references.
Paths: packages/core/models/, packages/core/src/, packages/db/seed/
```

### Agent 3: Rust Code Quality

**Description**: `Audit Rust code quality`

**Prompt**:
```
Analyze tools/novanet/ Rust codebase:

1. Run: cargo clippy -- -D warnings
   Report any warnings (should be zero)

2. Check for patterns:
   - Unwrap calls without context (panic risk)
   - Clone on large types (performance)
   - Missing error context (error handling)
   - Unused imports or dead code

3. Verify module organization:
   - All pub items in mod.rs are used
   - No circular dependencies
   - Feature gates are correct

4. Check formatting:
   - Run: cargo fmt --check
   - Report any formatting issues

Report findings with file:line:column format.
Path: tools/novanet/src/
```

### Agent 4: Test Coverage Analysis

**Description**: `Analyze test coverage and health`

**Prompt**:
```
Analyze test health across codebase:

1. Run: cargo test --no-run 2>&1 | grep -E "Compiling|error"
   Report any test compilation errors

2. Search for test issues:
   - Skipped tests (describe.skip, it.skip, #[ignore])
   - TODO/FIXME comments in test files
   - Empty test blocks
   - Tests with no assertions

3. Check test organization:
   - Every src module has tests (or tests/ file)
   - Integration tests exist for main features
   - Snapshot tests are up to date

4. Coverage gaps (heuristic):
   - Public functions without test coverage
   - Error paths untested

Report findings with file:line format.
Paths: tools/novanet/src/, tools/novanet/tests/, packages/*/src/
```

### Agent 5: Documentation Freshness

**Description**: `Verify documentation accuracy`

**Prompt**:
```
Cross-reference documentation with code:

1. CLAUDE.md files:
   - Commands mentioned actually exist
   - File paths mentioned are valid
   - Version numbers are current (v11.5+)
   - Node/arc counts match YAML files

2. README.md files:
   - Installation instructions work
   - Example commands run successfully
   - Links are not broken

3. ADR files (.claude/rules/novanet-decisions.md):
   - Referenced nodes still exist
   - Counts are accurate
   - No deprecated terminology

4. CHANGELOG.md:
   - Version numbers sequential
   - Referenced PRs/issues exist

Report inaccuracies with specific evidence.
Paths: *.md, .claude/, docs/
```

### Agent 6: Dependency Audit

**Description**: `Audit dependencies for security`

**Prompt**:
```
Analyze dependency health:

1. Rust dependencies:
   - Run: cargo deny check
   - Run: cargo audit
   - Check for advisories in deny.toml ignore list that are now fixed
   - Identify outdated dependencies

2. TypeScript dependencies:
   - Run: pnpm audit
   - Check for unused dependencies
   - Version drift between packages

3. License compliance:
   - All Rust deps have allowed licenses (MIT, Apache-2.0, BSD)
   - Document any exceptions in deny.toml

4. Supply chain:
   - Check for typosquatting risks
   - Verify dependencies are from expected registries

Report security findings with severity level.
Paths: Cargo.toml, Cargo.lock, package.json, pnpm-lock.yaml
```

### Agent 7: Performance Patterns

**Description**: `Find performance anti-patterns`

**Prompt**:
```
Scan for performance issues:

1. Rust patterns:
   - Excessive cloning where borrow would work
   - Allocations in hot loops
   - Missing #[inline] on small functions
   - Vec growth without capacity hints
   - String concatenation vs format!

2. TypeScript patterns:
   - Large objects passed by value
   - Missing useMemo/useCallback in React
   - N+1 query patterns
   - Blocking operations in async code

3. Build performance:
   - Unused features enabled
   - Heavy dev dependencies
   - Slow compile times (identify heavy crates)

4. Runtime patterns:
   - Inefficient Neo4j queries (missing indexes)
   - Redundant data loading

Report with file:line and explanation of impact.
Paths: tools/novanet/src/, packages/core/src/, apps/studio/src/
```

### Agent 8: Security Patterns

**Description**: `Scan for security issues`

**Prompt**:
```
Audit codebase for security vulnerabilities:

1. Credential exposure:
   - Hardcoded passwords or API keys
   - .env files tracked in git
   - Secrets in comments or logs

2. Input validation:
   - User input used in Cypher queries (injection risk)
   - File paths from user input (traversal risk)
   - Unvalidated JSON deserialization

3. Error handling:
   - Sensitive info in error messages
   - Stack traces exposed to users
   - Missing error boundaries

4. Access control:
   - Missing authentication checks
   - Overly permissive CORS
   - Debug endpoints in production

Report with severity: CRITICAL/HIGH/MEDIUM/LOW.
Paths: **/*, .env*, *.cypher
```

### Agent 9: Dead Code Detection

**Description**: `Find unused code and exports`

**Prompt**:
```
Hunt for dead code:

1. TypeScript/JavaScript:
   - Exports in index.ts never imported elsewhere
   - Functions defined but never called
   - Imported modules never used
   - React components never rendered

2. Rust:
   - pub items never used outside module
   - Unused functions (#[allow(dead_code)] should be rare)
   - Unused dependencies (cargo machete)

3. Files:
   - Files not imported anywhere
   - Orphaned test files for deleted modules
   - Stale migration files

4. Configuration:
   - Unused scripts in package.json
   - Unused entries in tsconfig paths
   - Dead feature flags

Report each finding with file:line format.
Paths: packages/core/src/, apps/studio/src/, tools/novanet/src/
```

### Agent 10: Semantic Coherence

**Description**: `Verify architectural consistency`

**Prompt**:
```
Check semantic consistency:

1. Terminology:
   - No use of deprecated terms (Edge->Arc, global->shared, tenant->org)
   - Consistent naming (EntityContent not EntityL10n)
   - Trait naming follows convention

2. Architecture alignment:
   - Components match documented structure
   - Layers follow dependency rules (output doesn't import structure)
   - Realm separation maintained (shared readonly)

3. Pattern consistency:
   - Error handling patterns uniform
   - Logging patterns consistent
   - Configuration patterns match

4. NovaNet-specific:
   - v11.5 architecture (2 realms, 10 layers)
   - 60 node types, correct distribution
   - Arc families correct (5 families)

Report deviations with evidence.
Paths: packages/core/, tools/novanet/, apps/studio/
```

---

## Phase 2: SYNTHESIZE

After all agents complete, aggregate findings:

### Create TodoWrite

```typescript
TodoWrite([
  { content: "Audit SCAN phase complete", status: "completed", activeForm: "SCAN completed" },
  { content: "Fix CRITICAL issues (N items)", status: "pending", activeForm: "Fixing CRITICAL issues" },
  { content: "Fix HIGH issues (N items)", status: "pending", activeForm: "Fixing HIGH issues" },
  { content: "Fix MEDIUM issues (N items)", status: "pending", activeForm: "Fixing MEDIUM issues" },
  { content: "Address LOW issues (N items)", status: "pending", activeForm: "Addressing LOW issues" },
  { content: "Run verification loop", status: "pending", activeForm: "Running verification loop" }
])
```

### Severity Classification

| Severity | Criteria | Action |
|----------|----------|--------|
| **CRITICAL** | Build breaks, security vuln, data loss risk | Fix immediately, one at a time |
| **HIGH** | Incorrect behavior, sync issues, test failures | Fix soon, batch of 2-3 |
| **MEDIUM** | Dead code, missing docs, minor perf | Fix in batch, test at end |
| **LOW** | Style, cosmetic, optional improvements | Log for later |

### Summary Report

Output findings as:

```
+===============================================================================+
|  RALPH WIGGUM AUDIT REPORT                                          v11.X.X  |
+===============================================================================+

SUMMARY
-------
Agents:     10/10 completed
Duration:   X minutes
Findings:   XX total

+-------------------------------------------------------------------------------+
|  CRITICAL (X)                                                                  |
+-------------------------------------------------------------------------------+
| Agent | Issue | Location | Description |
|-------|-------|----------|-------------|
| YAML  | Missing realm | node-kinds/foo.yaml:3 | Field required |
...

+-------------------------------------------------------------------------------+
|  HIGH (X)                                                                      |
+-------------------------------------------------------------------------------+
...

+-------------------------------------------------------------------------------+
|  MEDIUM (X)                                                                    |
+-------------------------------------------------------------------------------+
...

+-------------------------------------------------------------------------------+
|  LOW (X)                                                                       |
+-------------------------------------------------------------------------------+
...

+===============================================================================+
```

---

## Phase 3: FIX

If `--fix` flag present or user confirms, fix issues:

### Fix Order

1. **CRITICAL** - One at a time, test after each
2. **HIGH** - Batches of 2-3, test after batch
3. **MEDIUM** - Larger batches, test at end
4. **LOW** - Skip unless specifically requested

### Verification Commands

```bash
# After Rust changes
cargo clippy -- -D warnings && cargo test && cargo fmt --check

# After TypeScript changes
pnpm type-check && pnpm test && pnpm lint

# After YAML changes
cargo run -- schema validate && cargo run -- schema generate --dry-run

# Full verification
pnpm build && cargo test
```

---

## Phase 4: VERIFY

After fixes, run verification loop:

1. Re-launch agents for affected categories
2. Compare findings to previous scan
3. If new CRITICAL/HIGH issues: return to FIX
4. If clean: report success

### Loop Termination

Stop when:
- All CRITICAL and HIGH issues fixed
- All tests pass
- User says "good enough"

---

## Audit History

Store audit results for tracking:

```
docs/audits/
├── 2026-02-10-full-audit.md
├── 2026-02-09-quick-audit.md
└── ...
```

Each audit file contains:
- Timestamp
- Mode used
- Findings by severity
- Fixes applied
- Remaining issues

---

## Examples

```bash
# Full audit (all 10 agents)
/codebase-audit

# Full audit with auto-fix
/codebase-audit full --fix

# Quick essential checks
/codebase-audit quick

# Focus on YAML schema only
/codebase-audit yaml

# Focus on Rust quality
/codebase-audit rust

# Security scan
/codebase-audit security

# Documentation check
/codebase-audit docs
```

---

## Integration

| Related Skill | When to Use |
|---------------|-------------|
| `novanet-sync` | If audit finds YAML/TypeScript sync issues |
| `novanet-arch` | To understand architecture before/after audit |
| `security-audit` | For deeper security analysis (cargo-deny, pnpm audit) |
| `token-audit` | For design token consistency checks |

---

## Why "Ralph Wiggum"?

The verification loop keeps going until the codebase is actually clean.
Like Ralph, it's persistent and catches things others miss.
Named after the Simpsons character known for pointing out the obvious.

---

## Troubleshooting

### Agent Timeouts

If agents timeout, run in focused mode:
```bash
/codebase-audit yaml   # Just YAML
/codebase-audit rust   # Just Rust
```

### Too Many Findings

Focus on CRITICAL first:
1. Skip `--fix` initially
2. Review report
3. Fix CRITICAL manually
4. Re-run `/codebase-audit quick`

### False Positives

If agent reports false positives:
1. Document in `.claude/rules/audit-exceptions.md`
2. Update agent prompt to exclude known patterns
