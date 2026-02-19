# Gap Correction Plan — 2026-02-19

**Status:** Active — Quick Wins Execution
**Source:** Multi-agent analysis of 24 plans from 2026-02-18
**Priority:** P0 blockers first, then P1 important, then P2 nice-to-have

---

## Executive Summary

Analysis of 24 planning documents revealed **30 gaps** across Nika MVPs, NovaNet schema, integration, and DX. This plan prioritizes quick wins (< 1 hour each) that unblock development.

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
