# Gap Correction Plan — 2026-02-19

**Status:** Active — CRITICAL compilation error + comprehensive gaps
**Source:** 6 parallel explorer agents + 24 planning documents
**Priority:** CRITICAL > HIGH > MEDIUM > LOW

---

## Executive Summary

**UPDATE 12:10:** Comprehensive multi-agent analysis expanded scope significantly:

| Category | Gaps Found | Source Agent |
|----------|------------|--------------|
| CRITICAL | 1 (Nika compilation) | ad6730a |
| HIGH | 8 (MCP tests, API tests, CI) | acd65ac, a7bff50 |
| MEDIUM | 12 (benchmarks, fuzz, workflows) | a4dd765, aeef144 |
| LOW | 6 (Windows CI, MSRV, stale) | a7bff50 |

```
╔═══════════════════════════════════════════════════════════════════════════════╗
║  CRITICAL: Nika tui/state.rs:542 — `tokens` field mismatch                   ║
║  Impact: ALL 406 Nika tests BLOCKED                                          ║
║  Fix: Update EventKind::AgentTurn destructuring                              ║
╚═══════════════════════════════════════════════════════════════════════════════╝
```

---

## CRITICAL: Fix Immediately

### C.1 Nika Compilation Error (15 min)

**File:** `nika-dev/tools/nika/src/tui/state.rs:542`
**Error:** `EventKind::AgentTurn` variant does not have field `tokens`

```rust
// Current (BROKEN):
EventKind::AgentTurn {
    turn_index,
    kind,
    tokens,  // ← Field doesn't exist in v0.4.1
    ..
}

// Fix: Check log.rs for AgentTurnMetadata structure
```

**Impact:** 406 tests cannot run
**Action:** Check `event/log.rs` and update destructuring pattern

---

## HIGH Priority: Sprint 1 Additions

### H.1 NovaNet MCP Test Coverage (2-3 days) — FROM acd65ac

**Current:** 23 files, 14 with tests (61%)

| Critical Gap | File | Tests |
|--------------|------|-------|
| Request routing | `handler.rs` | 0 |
| Server lifecycle | `state.rs` | 0 |
| Error mapping | `error.rs` | 0 |
| Generation logic | `tools/generate.rs` | 0 |

**Target:** Add ~40 tests, reach 85% coverage

### H.2 Studio API Route Tests (2-3 days) — FROM acd65ac

**Current:** 13 API endpoints with ZERO tests

```
app/api/graph/route.ts         # 0 tests
app/api/schema/route.ts        # 0 tests
app/api/tree/[id]/children/    # 0 tests
... (10 more)
```

**Target:** 2 tests per route = 26 new tests

### H.3 GitHub Dependency Review (30 min) — FROM a7bff50

Create `.github/workflows/dependency-review.yml`:
```yaml
name: Dependency Review
on: [pull_request]
jobs:
  dependency-review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/dependency-review-action@v4
        with:
          fail-on-severity: high
```

### H.4 Nika: Activate proptest/insta (2-4h) — FROM aeef144

**Gap:** Dependencies declared but never used
- `proptest = "1.4"` → No `proptest!` macros
- `insta = "1.34"` → No `.snap` files

**Action:** Add property-based tests for binding module

---

## MEDIUM Priority: Sprint 2 Additions

### M.1 NovaNet Benchmark Harness — FROM a4dd765

Add to `novanet-dev/tools/novanet/Cargo.toml`:
```toml
[[bench]]
name = "benchmarks"
harness = false

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

### M.2 Nika Deprecated Provider Cleanup — FROM aeef144

**Location:** `resilience/provider.rs:527-528`
```rust
stop_reason: StopReason::EndTurn,  // deprecated
usage: Usage::new(10, 10),          // deprecated
```
**Action:** Migrate to rig-core types

### M.3 CodeQL Rust Support — FROM a7bff50

Update `.github/workflows/codeql.yml`:
```yaml
strategy:
  matrix:
    language: ['typescript', 'rust']  # Add 'rust'
```

### M.4 SBOM Generation — FROM a7bff50

Add to release workflow:
```yaml
- uses: anchore/sbom-action@v0
```

### M.5 Reusable Workflows — FROM a7bff50

Create `.github/workflows/reusable-rust-setup.yml` to DRY CI

### M.6 Fuzz Testing — FROM a4dd765, aeef144

Add `cargo-fuzz` targets for:
- YAML parsers (both projects)
- Binding expressions (Nika)
- Template substitution (Nika)

---

## LOW Priority: Sprint 3 Additions

### L.1 Nika Windows CI — FROM aeef144
### L.2 Nika MSRV Check — FROM aeef144
### L.3 PR Title Linting — FROM a7bff50
### L.4 Stale Issue Management — FROM a7bff50

---

## Agent Sources

| Agent | Focus | Status | Key Finding |
|-------|-------|--------|-------------|
| a7bff50 | GitHub CI/CD | ✅ | 6 workflow improvements |
| a1b276c | Main Project | ✅ | Integration test gaps |
| a4dd765 | NovaNet Rust | ✅ | 86% coverage, need benchmarks |
| aeef144 | Nika Rust | ✅ | Unused deps, deprecated types |
| acd65ac | NovaNet Tests | ✅ | MCP 61%, Studio 11% |
| ad6730a | Nika Tests | ⏳ | Found compilation error |

---

## Original Phase 1 (Previous Analysis)

---

## Phase 1: Quick Wins (Today)

### 1.1 Create Missing Nika Skills (30 min) ✅ P0

**Gap:** 5 skills referenced in plans but never created.

| Skill | Purpose | File |
|-------|---------|------|
| `nika-arch` | Architecture diagram | `nika-dev/.claude/skills/nika-arch/SKILL.md` |
| `nika-diagnose` | Workflow diagnosis checklist | `nika-dev/.claude/skills/nika-diagnose/SKILL.md` |
| `nika-run` | Run workflow with validation | `nika-dev/.claude/skills/nika-run/SKILL.md` |
| `nika-debug` | Debug failing workflows | `nika-dev/.claude/skills/nika-debug/SKILL.md` |
| `nika-binding` | Binding syntax reference | `nika-dev/.claude/skills/nika-binding/SKILL.md` |

**Deliverable:** 5 SKILL.md files + INDEX.md

### 1.2 Add cargo-deny to CI (15 min) ✅ P1

**Gap:** No supply chain security or license compliance.

**Files:**
- `nika-dev/tools/nika/deny.toml`
- Update `nika-dev/tools/nika/.github/workflows/ci.yml`

### 1.3 Switch CI to nextest (10 min) ✅ P1

**Gap:** `cargo test` is slower and less informative than `cargo nextest`.

**Files:**
- Update `nika-dev/tools/nika/.github/workflows/ci.yml`

### 1.4 Create Nika ADR Directory (30 min) ✅ P1

**Gap:** Nika has architectural decisions but no ADR documentation.

| ADR | Title | Decision |
|-----|-------|----------|
| ADR-001 | 5 Semantic Verbs | Why exactly infer/exec/fetch/invoke/agent |
| ADR-002 | YAML-First | YAML as source of truth for workflows |
| ADR-003 | MCP-Only Integration | Zero Cypher rule enforced |

**Files:**
- `nika-dev/tools/nika/.claude/rules/adr/adr-001-5-semantic-verbs.md`
- `nika-dev/tools/nika/.claude/rules/adr/adr-002-yaml-first.md`
- `nika-dev/tools/nika/.claude/rules/adr/adr-003-mcp-only.md`

### 1.5 Add Coverage Job to CI (20 min) ✅ P2

**Gap:** No coverage visibility.

**Files:**
- Update `nika-dev/tools/nika/.github/workflows/ci.yml`

### 1.6 Document denomination_forms Gap (10 min) ✅ P2

**Gap:** Known gap mentioned in ROADMAP but not tracked.

**Action:** Add explicit section to ROADMAP with NovaNet action items.

---

## Phase 2: Medium Fixes (This Week)

### 2.1 Fix Silent Mock Fallback (1h) — P0

**Gap:** When MCP config is missing, executor silently uses mock instead of erroring.

**Location:** `nika-dev/tools/nika/src/runtime/executor.rs`

**Fix:**
```rust
// BEFORE: Silent fallback
let client = self.get_or_create_mcp_client(server_name).await?;

// AFTER: Explicit error
let config = self.mcp_configs.get(server_name)
    .ok_or_else(|| NikaError::McpNotConfigured {
        server: server_name.to_string()
    })?;
```

**Test:** `test_invoke_missing_mcp_config_returns_error`

### 2.2 Fix for_each Result Aggregation (2h) — P0

**Gap:** All for_each iterations write to same key, only last survives.

**Location:** `nika-dev/tools/nika/src/runtime/runner.rs:307-339`

**Fix:** Use `Vec<(usize, TaskResult)>` instead of single key.

### 2.3 Add call_id to McpInvoke Events (1h) — P1

**Gap:** Cannot correlate MCP calls with responses in traces.

**Location:** `nika-dev/tools/nika/src/event/log.rs`

**Fix:** Add `call_id: Uuid` field to `McpInvoke` and `McpResponse` variants.

### 2.4 Fix UC1-UC5 Transport Paths (30 min) — P1

**Gap:** Use-case examples reference TypeScript server that doesn't exist.

**Location:** `nika-dev/tools/nika/examples/uc*.nika.yaml`

**Fix:** Update `command:` to use Rust binary path.

---

## Phase 3: Larger Refactors (Next Sprint)

### 3.1 Connect TUI to Real Runner — P0 (MVP 3 incomplete)

The TUI `execute_workflow` is a stub. Needs full integration with Runner.

### 3.2 Implement context_build_log in NovaNet MCP — P0

NovaNet MCP must return context_build_log for observability.

### 3.3 Add denomination_forms to novanet_generate Response — P0

Per ADR-033, denomination_forms must be returned by MCP.

### 3.4 Add ForEach to TaskAction Enum — P0 (MVP 6)

`TaskAction` enum needs `ForEach` variant for for_each verb to work.

---

## Verification Checklist

After Phase 1:

- [ ] `ls nika-dev/.claude/skills/` shows 7+ skills
- [ ] `cargo deny check` passes in CI
- [ ] `cargo nextest run` used in CI
- [ ] `ls nika-dev/tools/nika/.claude/rules/adr/` shows 3 ADRs
- [ ] Coverage job appears in CI workflow

After Phase 2:

- [ ] `cargo test test_invoke_missing_mcp_config` passes
- [ ] for_each with 3 items returns 3 results
- [ ] NDJSON traces show `call_id` on MCP events
- [ ] UC1 workflow runs against real NovaNet

---

## Gap Cross-Reference

| Gap ID | Description | Phase | Status |
|--------|-------------|-------|--------|
| 1 | rmcp unused | 3 | Defer |
| 2 | McpClient API change | 3 | Defer |
| 3 | TUI stub | 3 | Phase 3.1 |
| 4 | ForEach not in TaskAction | 3 | Phase 3.4 |
| 5 | context_build_log missing | 3 | Phase 3.2 |
| 6 | Entity denomination_forms | 3 | Phase 3.3 |
| 7 | UC transport paths | 2 | Phase 2.4 |
| 8 | Mock coverage 3/7 | 2 | Defer |
| 9 | Missing skills | 1 | Phase 1.1 |
| 10 | Silent mock fallback | 2 | Phase 2.1 |
| 11 | Wrong denomination schema | 2 | Defer |
| 12-30 | See agent reports | 2-3 | Various |

---

## Execution Log

| Time | Action | Status |
|------|--------|--------|
| 2026-02-19 | Plan created | ✅ |
| 2026-02-19 | Phase 1.1 Skills | 🔄 |
| 2026-02-19 | Phase 1.2 cargo-deny | ⏳ |
| 2026-02-19 | Phase 1.3 nextest | ⏳ |
| 2026-02-19 | Phase 1.4 ADRs | ⏳ |
| 2026-02-19 | Phase 1.5 Coverage | ⏳ |
| 2026-02-19 | Phase 1.6 ROADMAP | ⏳ |
