# Master Implementation Plan - supernovae-agi v0.3

**Date:** 2026-02-19
**Status:** Ready for Execution
**Approach:** TDD + Subagent-Driven Development
**Total Tasks:** 42 bite-sized tasks (2-5 min each)
**Estimated Time:** ~3-4 hours

---

## Executive Summary

Ce plan consolide toutes les tâches restantes pour amener supernovae-agi (NovaNet + Nika) à v0.3 production-ready.

**Sources consolidées:**
- `dx-audit-fixes.md` — 12 issues (tests, docs, MCP) — **All covered**
- `gap-correction-plan.md` — 30 gaps → **This plan covers 16; Phase 1-2 infrastructure gaps (14 tasks) are addressed in Phase 6**
- `rig-core-ecosystem-strategy.md` — Integration roadmap (future sprints)

**Priorisation:**
1. **Phase 1: Fix CI** — Tests must pass first (TDD prerequisite)
2. **Phase 2: MCP Gaps** — Core functionality (ADR-033, DX-11)
3. **Phase 3: Documentation** — DX polish
4. **Phase 4: Examples** — User-facing demos
5. **Phase 5: ADR Updates** — Required for compliance
6. **Phase 6: DX Infrastructure** — gap-correction-plan.md Phase 1-2

---

## Current State

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  PROJECT STATUS (2026-02-19)                                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  NovaNet v0.13.1                     Nika v0.2.0                            │
│  ├── 61 nodes, 182 arcs, 6 families  ├── 19.5k LoC, 602 tests              │
│  ├── 1,191 Rust tests (1 failing)    ├── 5 verbs (infer/exec/fetch/invoke/agent) │
│  ├── MCP Server: 7 tools             ├── MCP Client: rmcp adapter          │
│  └── Studio: Next.js 16 + React 19   └── Resilience: retry/circuit/rate    │
│                                                                             │
│  BLOCKERS:                                                                  │
│  🔴 1 Rust test failing (novanet)                                           │
│  🔴 3 TypeScript tests failing (arc family count)                           │
│  🔴 denomination_forms NOT in MCP response                                  │
│  🔴 context_build_log NOT implemented                                       │
│  🟡 McpClient mock mode default                                             │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Fix CI (Tests Must Pass)

**Goal:** All tests green before any new code (TDD prerequisite)
**Time:** ~20 min

### Task 1.1: Fix TypeScript arc family count

**File:** `novanet-dev/packages/core/tests/design-system-coherence.test.ts`
**Problem:** Test expects 5 arc families, but v0.13.1 has 6 (added "schema" family)

```typescript
// BEFORE (line ~XX):
expect(arcFamilies.length).toBe(5);

// AFTER:
expect(arcFamilies.length).toBe(6);
```

**Verification:**
```bash
cd novanet-dev && pnpm test --filter=@novanet/core
```

### Task 1.2: Fix failing Rust test

**File:** `novanet-dev/tools/novanet/tests/schema_generate_dry_run_integration.rs`
**Test:** `schema_generate_dry_run`
**Problem:** Missing fixture file or path mismatch

```bash
cd novanet-dev/tools/novanet
cargo test schema_generate_dry_run 2>&1
```

**Fix approach:**
1. Check error message for missing file path
2. Either create missing fixture or update path in test
3. Run test again to verify

### Task 1.3: Verify all Rust tests pass

**After fixing 1.2:**

**Verification:**
```bash
cargo test
# Expected: 1191 tests passing, 0 failing
```

### Task 1.4: Run full CI check

```bash
# NovaNet
cd novanet-dev
pnpm test && pnpm lint && pnpm type-check
cargo test

# Nika
cd ../nika-dev/tools/nika
cargo test
cargo clippy -- -D warnings
```

**Checkpoint:** All tests pass → Proceed to Phase 2

---

## Phase 2: MCP Critical Gaps

**Goal:** ADR-033 compliance + observability
**Time:** ~45 min

### Task 2.1: Locate novanet_generate implementation

**File:** `novanet-dev/tools/novanet-mcp/src/tools/generate.rs`

```bash
cd novanet-dev
grep -rn "novanet_generate" tools/novanet-mcp/src/
```

### Task 2.2: Add denomination_forms to response (TDD)

**Pre-condition check:**
```bash
cd novanet-dev/tools/novanet-mcp
grep -n "struct GenerateResponse" src/server/handler.rs
cat src/server/handler.rs | head -250 | tail -50  # See current response struct
```

**Test first (add to existing test module):**
```rust
// File: novanet-dev/tools/novanet-mcp/src/server/handler.rs
// Add at end of file in #[cfg(test)] module:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_response_has_denomination_forms_field() {
        // Compilation test - verifies struct has the field
        let response = GenerateResponse {
            // ... existing fields with defaults ...
            denomination_forms: Some(HashMap::from([
                ("text".to_string(), "código qr".to_string()),
                ("title".to_string(), "Código QR".to_string()),
            ])),
            ..Default::default()
        };
        assert!(response.denomination_forms.is_some());
    }
}
```

**Run test (should fail - RED: field doesn't exist):**
```bash
cargo test generate_response_has_denomination_forms
```

**Implementation (GREEN):**
```rust
// File: novanet-dev/tools/novanet-mcp/src/server/handler.rs
// Add to GenerateResponse struct:
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct GenerateResponse {
    // ... existing fields ...
    /// Canonical name forms for LLM entity references (ADR-033)
    pub denomination_forms: Option<HashMap<String, String>>,
}

// In novanet_generate function, add before returning:
let denomination_forms = self.fetch_denomination_forms(&entity_native).await?;
```

**Verify:**
```bash
cargo test generate_response_has_denomination_forms
# Expected: PASS
```

### Task 2.3: Define context_build_log schema

**File:** `novanet-dev/tools/novanet-mcp/src/server/handler.rs`
**Note:** Add structs near top of file with other response types.

```rust
use schemars::JsonSchema;

/// Debug log showing how context was assembled (DX-11)
#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct ContextBuildLog {
    /// Steps taken to build the context
    pub phases: Vec<ContextPhase>,
    /// Token budget allocation per source
    pub token_allocation: HashMap<String, usize>,
    /// Pruning decisions made
    pub pruning_decisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContextPhase {
    pub name: String,         // e.g., "structure_phase", "entities_phase"
    pub nodes_loaded: usize,
    pub arcs_traversed: usize,
    pub duration_ms: u64,
}
```

### Task 2.4: Add context_build_log to response (TDD)

**Test first:**
```rust
#[tokio::test]
async fn test_generate_includes_context_build_log() {
    let response = generate_for_entity("qr-code", "fr-FR").await;

    assert!(response.context_build_log.is_some());
    let log = response.context_build_log.unwrap();
    assert!(!log.phases.is_empty());
    assert!(log.phases.iter().any(|p| p.name == "structure_phase"));
}
```

**Run test (RED):**
```bash
cargo test test_generate_includes_context_build_log
```

**Implementation (GREEN):** Add logging throughout generate function

### Task 2.5: Verify MCP schema auto-generation

**Note:** NovaNet MCP uses `schemars::JsonSchema` derive macro - schemas are auto-generated from Rust structs. No manual schema.json needed.

**Verification:**
```bash
cd novanet-dev/tools/novanet-mcp
# Check that JsonSchema is derived on response structs
grep -n "JsonSchema" src/server/handler.rs

# Run MCP server and check generated schema
cargo run -- --help  # Or invoke list_tools
```

**Expected:** Both `denomination_forms` and `context_build_log` appear in tool schema automatically via `#[derive(JsonSchema)]`.

### Task 2.6: Remove McpClient mock default

**File:** `nika-dev/tools/nika/src/mcp/client.rs`

```rust
// BEFORE:
pub struct McpClient {
    mock: bool,  // defaults to true
}

impl Default for McpClient {
    fn default() -> Self {
        Self { mock: true }  // ❌ WRONG
    }
}

// AFTER:
pub struct McpClient {
    #[cfg(test)]
    mock: bool,
    // Production: no mock field at all
}

// Use feature flags:
#[cfg(test)]
impl McpClient {
    pub fn mock() -> Self { /* test-only mock */ }
}
```

**Verification:**
```bash
cargo test mcp_client
cargo build --release  # Ensure no mock in release
```

---

## Phase 3: Documentation Updates

**Goal:** DX completeness
**Time:** ~30 min

### Task 3.1: Add resilience patterns to Nika CLAUDE.md

**File:** `nika-dev/tools/nika/CLAUDE.md`

Add section after "Verbs":

```markdown
## Resilience Patterns (v0.2)

Nika includes production-grade resilience:

| Pattern | Module | Description |
|---------|--------|-------------|
| **retry** | `resilience/retry.rs` | Exponential backoff with jitter (21 tests) |
| **circuit_breaker** | `resilience/circuit_breaker.rs` | Fail-fast on repeated errors (12 tests) |
| **rate_limiter** | `resilience/rate_limiter.rs` | Provider API throttling (11 tests) |

### Usage

```yaml
workflow: resilient-generation
config:
  retry:
    max_attempts: 3
    backoff: exponential
  circuit_breaker:
    failure_threshold: 5
    reset_timeout: 30s
```
```

### Task 3.2: Add for_each to Nika CLAUDE.md

**File:** `nika-dev/tools/nika/CLAUDE.md`

Add to Verbs section:

```markdown
### for_each (v0.3)

Parallel execution over collections:

```yaml
- for_each:
    items: $locales
    concurrency: 3
    task:
      invoke: novanet_generate
      params:
        entity: $entity_key
        locale: $item
```

**Key features:**
- `tokio::spawn` JoinSet for true concurrency
- Configurable `concurrency` limit
- Error collection with partial results
```

### Task 3.3: Update NovaNet CLAUDE.md MCP section

**File:** `novanet-dev/CLAUDE.md`

Ensure MCP Server section has all 7 tools with params:

```markdown
## MCP Server (v0.14.0)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `novanet_generate` | RLM-on-KG context assembly | focus_key, locale, forms |
| `novanet_describe` | Bootstrap agent understanding | describe (schema/entity/...) |
| `novanet_query` | Execute read-only Cypher | cypher, params, limit |
| `novanet_search` | Fulltext + property search | query, mode, kinds |
| `novanet_traverse` | Graph traversal | start_key, max_depth, direction |
| `novanet_assemble` | Token-aware context assembly | focus_key, token_budget |
| `novanet_atoms` | Locale knowledge atoms | locale, atom_type, domain |

**v0.14.0 additions:**
- `denomination_forms` in generate response (ADR-033)
- `context_build_log` for debugging (DX-11)
```

### Task 3.4: Fix supernovae-agi README clone URL

**File:** `supernovae-agi/README.md`

```bash
# Find current URL
grep -n "git clone" README.md

# Update to correct URL
```

### Task 3.5: Fix nika-dev README version badge

**File:** `nika-dev/README.md`

```markdown
<!-- BEFORE -->
![Version](https://img.shields.io/badge/version-0.3.0-blue)

<!-- AFTER -->
![Version](https://img.shields.io/badge/version-0.2.0-blue)
```

### Task 3.6: Fix ROADMAP.md node count

**File:** `supernovae-agi/ROADMAP.md`

```bash
# Find "62 nodes" and replace with "61 nodes"
grep -n "62 nodes" ROADMAP.md
```

---

## Phase 4: v0.3 Example Workflows

**Goal:** User-facing demos
**Time:** ~25 min

### Task 4.1: Create parallel generation example

**File:** `nika-dev/tools/nika/examples/v03-parallel-generation.yaml`

```yaml
# v03-parallel-generation.yaml
# Demonstrates for_each parallelism with NovaNet MCP

workflow: parallel-locale-generation
version: "0.3"
description: Generate content for multiple locales in parallel

inputs:
  entity_key:
    type: string
    default: "qr-code"
  locales:
    type: array
    default: ["fr-FR", "es-MX", "de-DE"]

mcp:
  servers:
    novanet:
      command: "cargo run --manifest-path ../novanet-dev/tools/novanet-mcp/Cargo.toml"

tasks:
  - id: generate_all
    for_each:
      items: $inputs.locales
      concurrency: 3
      task:
        invoke: novanet_generate
        server: novanet
        params:
          focus_key: $inputs.entity_key
          locale: $item
          forms: ["text", "title", "url"]
    use.ctx: locale_contents

  - id: summarize
    infer: |
      Summarize the generated content for all locales.
      Available content: $locale_contents
    use.result: summary

outputs:
  contents: $locale_contents
  summary: $summary
```

**Validation:**
```bash
cargo run -- validate examples/v03-parallel-generation.yaml
```

### Task 4.2: Create agent refinement example

**File:** `nika-dev/tools/nika/examples/v03-agent-refinement.yaml`

```yaml
# v03-agent-refinement.yaml
# Demonstrates agent: verb with MCP tool calling

workflow: agent-content-refinement
version: "0.3"
description: Multi-turn agent refines content using NovaNet context

inputs:
  entity_key:
    type: string
    default: "qr-code"
  locale:
    type: string
    default: "fr-FR"
  quality_target:
    type: string
    default: "professional landing page"

mcp:
  servers:
    novanet:
      command: "cargo run --manifest-path ../novanet-dev/tools/novanet-mcp/Cargo.toml"

tasks:
  - id: refine_content
    agent:
      goal: |
        Generate high-quality content for entity "$inputs.entity_key" in locale "$inputs.locale".
        Target quality: $inputs.quality_target

        Use novanet_generate to get initial context, then refine iteratively.
        Call novanet_search if you need additional entity information.
      tools:
        - novanet_generate
        - novanet_search
        - novanet_traverse
      max_turns: 5
      provider: claude
    use.result: refined_content

outputs:
  content: $refined_content
```

**Validation:**
```bash
cargo run -- validate examples/v03-agent-refinement.yaml
```

### Task 4.3: Create quick-start demo script

**File:** `nika-dev/tools/nika/examples/demo.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== Nika v0.3 Quick Demo ==="
echo ""

# Check prerequisites
command -v cargo >/dev/null 2>&1 || { echo "Error: cargo required"; exit 1; }

# Run validation
echo "1. Validating example workflows..."
cargo run -- validate examples/v03-parallel-generation.yaml
cargo run -- validate examples/v03-agent-refinement.yaml
echo "   ✓ All workflows valid"

# Show help
echo ""
echo "2. Available commands:"
cargo run -- --help | head -20

echo ""
echo "3. To run a workflow:"
echo "   cargo run -- run examples/v03-parallel-generation.yaml"
echo ""
echo "=== Demo complete ==="
```

---

## Phase 5: ADR Updates (Required)

**Goal:** ADR compliance for C3 (ADR-033) and new DX-11 observability
**Time:** ~15 min

### Task 5.1: Update ADR-033 with MCP enforcement

**File:** `docs/adr/schema-architecture/adr-033-denomination-forms.md`
**Note:** ADRs are organized in subdirectories by domain.

Add section at end:

```markdown
## MCP Enforcement (v0.13.x)

The `novanet_generate` tool MUST return `denomination_forms` in its response:

```json
{
  "denomination_forms": {
    "text": "código qr",
    "title": "Código QR",
    "abbrev": "qr",
    "url": "crear-codigo-qr"
  }
}
```

Nika workflows can directly use these forms in `infer:` tasks without additional transformations.
```

### Task 5.2: Create ADR-034 Context Build Log

**File:** `docs/adr/development-tools/adr-034-context-build-log.md`
**Note:** This is a development tools ADR, same directory as adr-033-autofix-system.md.

```markdown
# ADR-034: Context Build Log

## Status
Accepted

## Context
Debugging LLM context assembly is difficult. We need observability into:
- Which graph traversal paths were taken
- Token budget allocation decisions
- Pruning decisions

## Decision
Add `context_build_log` to `novanet_generate` response with structured phases:
- `structure_phase`: Page/Block structure loading
- `entities_phase`: Entity + EntityNative retrieval
- `atoms_phase`: Knowledge atoms (Terms, Expressions)
- `anchors_phase`: Context anchors assembly
- `token_decisions`: Final token budget allocation

## Consequences
- Positive: Full observability into context assembly
- Positive: Enables tuning of traversal parameters
- Negative: Slight response size increase (~200 bytes)
```

---

## Phase 6: DX Infrastructure (gap-correction-plan.md Phase 1-2)

**Goal:** Complete infrastructure gaps from gap-correction-plan.md
**Time:** ~60 min
**Note:** These tasks can be executed in parallel with Phases 1-5 or after.

### Task 6.1: Create Nika skills (5 skills)

**Files to create:**
- `nika-dev/.claude/skills/nika-arch.md` — Architecture diagram
- `nika-dev/.claude/skills/nika-diagnose.md` — Error diagnosis helper
- `nika-dev/.claude/skills/nika-run.md` — Workflow execution helper
- `nika-dev/.claude/skills/nika-debug.md` — Debug mode helper
- `nika-dev/.claude/skills/nika-binding.md` — Variable binding helper

**Template:**
```markdown
---
name: nika-arch
description: Display Nika workflow architecture
---

# Nika Architecture

[ASCII diagram here]
```

### Task 6.2: Add cargo-deny to Nika CI

**File:** `nika-dev/tools/nika/deny.toml`

```toml
[advisories]
vulnerability = "deny"
unmaintained = "warn"

[licenses]
allow = ["MIT", "Apache-2.0", "ISC", "BSD-3-Clause"]

[bans]
multiple-versions = "warn"
```

**CI update:** Add `cargo deny check` to workflow.

### Task 6.3: Switch Nika CI to cargo nextest

**File:** `.github/workflows/ci.yml` (Nika)

```yaml
- name: Run tests
  run: cargo nextest run --workspace
```

### Task 6.4: Create Nika ADR directory

**Files:**
- `nika-dev/docs/adr/adr-001-verb-architecture.md`
- `nika-dev/docs/adr/adr-002-mcp-integration.md`
- `nika-dev/docs/adr/adr-003-resilience-patterns.md`

### Task 6.5: Fix silent mock fallback in executor.rs

**File:** `nika-dev/tools/nika/src/runtime/executor.rs`
**Problem:** Silent fallback to mock mode

```rust
// BEFORE: Silent fallback
let client = mcp_client.unwrap_or_else(|| MockClient::new());

// AFTER: Explicit error
let client = mcp_client.ok_or_else(|| {
    NikaError::McpNotConfigured {
        message: "invoke: verb requires mcp: config in workflow".to_string(),
    }
})?;
```

### Task 6.6: Fix for_each result aggregation

**File:** `nika-dev/tools/nika/src/runtime/runner.rs` (lines 307-339)
**Problem:** Results may not aggregate correctly

Add test and fix aggregation logic.

### Task 6.7: Add call_id to MCP events

**File:** `nika-dev/tools/nika/src/event/log.rs`

```rust
pub struct McpInvoke {
    // existing fields...
    pub call_id: Uuid,  // Add unique call ID
}

pub struct McpResponse {
    // existing fields...
    pub call_id: Uuid,  // Match to invoke
}
```

### Task 6.8: Fix UC1-UC5 transport paths

**Files:** `nika-dev/tools/nika/examples/uc1-*.yaml` through `uc5-*.yaml`
**Problem:** Examples use TypeScript MCP path that doesn't exist

```yaml
# BEFORE:
mcp:
  servers:
    novanet:
      command: "npx ts-node novanet-mcp/src/index.ts"  # Wrong

# AFTER:
mcp:
  servers:
    novanet:
      command: "cargo run --manifest-path ../novanet-dev/tools/novanet-mcp/Cargo.toml"
```

### Task 6.9: Add coverage job to CI

**File:** `.github/workflows/ci.yml`

```yaml
coverage:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Install cargo-llvm-cov
      run: cargo install cargo-llvm-cov
    - name: Generate coverage
      run: cargo llvm-cov --workspace --lcov --output-path lcov.info
    - name: Upload to Codecov
      uses: codecov/codecov-action@v4
```

---

## Execution Strategy

### Option A: Subagent-Driven (Recommended)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SUBAGENT EXECUTION                                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Agent 1: spn-rust:rust-pro    │  Phase 1 (CI) + Phase 2 (MCP)              │
│  Agent 2: general-purpose      │  Phase 3 (Documentation)                   │
│  Agent 3: general-purpose      │  Phase 4 (Examples)                        │
│  Agent 4: general-purpose      │  Phase 5 (ADRs) + Phase 6.1-6.4 (DX)       │
│  Agent 5: spn-rust:rust-pro    │  Phase 6.5-6.9 (Rust fixes)                │
│  Agent 6: code-reviewer        │  Final review before commit                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Option B: Sequential Session

Execute all phases in current session with reviews between phases.

### Verification Checklist (Final)

Before marking complete:

**Phase 1-2 (CI + MCP):**
- [ ] `cd novanet-dev && pnpm test` → All pass
- [ ] `cd novanet-dev/tools/novanet && cargo test` → 1191+ pass
- [ ] `cd novanet-dev/tools/novanet-mcp && cargo test` → All pass
- [ ] `cd nika-dev/tools/nika && cargo test` → 602+ pass
- [ ] `denomination_forms` in MCP response ✓
- [ ] `context_build_log` in MCP response ✓
- [ ] McpClient uses real connection by default ✓

**Phase 3-4 (Docs + Examples):**
- [ ] NovaNet CLAUDE.md has MCP tools section
- [ ] Nika CLAUDE.md has resilience + for_each
- [ ] READMEs have correct URLs/versions
- [ ] v0.3 example workflows validate successfully

**Phase 5-6 (ADRs + Infrastructure):**
- [ ] ADR-033 updated with MCP enforcement
- [ ] ADR-034 created for context_build_log
- [ ] Nika skills created (5)
- [ ] cargo-deny configured
- [ ] UC1-UC5 use correct MCP paths

---

## Commit Strategy

**Atomic commits per phase (not bundled):**

```bash
# After Phase 1
git add -A && git commit -m "fix(ci): pass all tests (TypeScript arc families + Rust fixtures)

- Update arc family count from 5 to 6 (v0.13.1 schema family)
- Fix Rust test fixture paths

Co-Authored-By: Claude <noreply@anthropic.com>"

# After Phase 2
git add -A && git commit -m "feat(mcp): add denomination_forms + context_build_log (ADR-033, DX-11)

- denomination_forms in novanet_generate response
- context_build_log for context assembly observability
- Remove McpClient mock mode default

Co-Authored-By: Claude <noreply@anthropic.com>"

# After Phase 3
git add -A && git commit -m "docs(claude): update CLAUDE.md files with MCP + resilience

- Add resilience patterns + for_each to Nika CLAUDE.md
- Update NovaNet CLAUDE.md MCP section
- Fix README URLs and version badges

Co-Authored-By: Claude <noreply@anthropic.com>"

# After Phase 4
git add -A && git commit -m "feat(examples): add v0.3 workflow examples

- v03-parallel-generation.yaml: for_each + invoke demo
- v03-agent-refinement.yaml: agent + MCP demo
- demo.sh: quick-start script

Co-Authored-By: Claude <noreply@anthropic.com>"

# After Phase 5
git add -A && git commit -m "docs(adr): update ADR-033 + create ADR-034

- ADR-033: Add MCP enforcement section
- ADR-034: Context build log specification

Co-Authored-By: Claude <noreply@anthropic.com>"

# After Phase 6
git add -A && git commit -m "chore(dx): Nika infrastructure improvements

- Add 5 Claude Code skills for Nika
- Configure cargo-deny for security
- Fix UC1-UC5 MCP transport paths
- Add call_id to MCP events

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Success Criteria

| Metric | Target | How to Verify |
|--------|--------|---------------|
| CI status | All green | `pnpm test && cargo test` |
| MCP compliance | ADR-033 ✓ | Test denomination_forms in response |
| Observability | DX-11 ✓ | context_build_log present |
| Documentation | 100% | All CLAUDE.md sections complete |
| Examples | 2+ | v03-*.yaml files validate |

---

## Related Documents

| Document | Purpose |
|----------|---------|
| `dx-audit-fixes.md` | Original 12 issues list |
| `gap-correction-plan.md` | Original 30 gaps list |
| `rig-core-ecosystem-strategy.md` | Future GraphRAG roadmap |
| `ROADMAP.md` | Project milestones |

---

**Ready for execution.** Use `/spn-powers:planning:execute-plan` or execute tasks manually.
