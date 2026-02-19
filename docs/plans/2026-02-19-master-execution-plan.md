# Master Execution Plan

**Date:** 2026-02-19
**Status:** Ready for Execution
**Methodology:** TDD + Ralph Wiggum Code Review + Verification Gates

---

## Overview

Ce plan orchestre l'exécution de tous les plans dans l'ordre optimal avec validation à chaque étape.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  MASTER EXECUTION PIPELINE                                                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  PHASE 0: CLEANUP (Quick Wins)                           ~2 hours      │   │
│  │  ├── Delete dead code (coherence/)                                     │   │
│  │  ├── Bump npm versions                                                 │   │
│  │  └── Install pre-commit hook                                           │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                              ↓                                                  │
│                     🔴 TDD Gate + ✅ Ralph Wiggum Review                        │
│                              ↓                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  PHASE 1: PLAN A - Token Tracking Fix                    ~2 hours      │   │
│  │  ├── Write failing test (RED)                                          │   │
│  │  ├── Fix executor.rs:380                                               │   │
│  │  └── Verify tokens > 0 (GREEN)                                         │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                              ↓                                                  │
│                     🔴 TDD Gate + ✅ Ralph Wiggum Review                        │
│                              ↓                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  PHASE 2: PLAN B - Test Coverage Gaps                    ~4-6 hours    │   │
│  │  ├── Store module tests (13 new)                                       │   │
│  │  ├── Provider module tests (7 new)                                     │   │
│  │  └── MCP tools tests (8 new)                                           │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                              ↓                                                  │
│                     🔴 TDD Gate + ✅ Ralph Wiggum Review                        │
│                              ↓                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  PHASE 3: PLAN D - Nika TUI Verification                 ~2-4 hours    │   │
│  │  ├── Manual panel verification                                         │   │
│  │  ├── Keyboard navigation tests                                         │   │
│  │  └── Integration tests (4 new)                                         │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                              ↓                                                  │
│                     🔴 TDD Gate + ✅ Ralph Wiggum Review                        │
│                              ↓                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  PHASE 4: PLAN C - QR Code AI Workflows                  ~Variable    │   │
│  │  ├── Design workflow templates                                         │   │
│  │  ├── Test with fr-FR locale                                            │   │
│  │  └── Validate generated content                                        │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                              ↓                                                  │
│                     🔴 FINAL TDD Gate + ✅ FINAL Ralph Wiggum                   │
│                              ↓                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  PHASE 5: RELEASE                                                      │   │
│  │  ├── Update CHANGELOGs                                                 │   │
│  │  ├── Bump versions                                                     │   │
│  │  └── Tag releases                                                      │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  TOTAL ESTIMATE: 12-18 hours                                                   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## TDD Methodology

Chaque tâche suit le cycle **RED → GREEN → REFACTOR**:

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  TDD CYCLE                                                                      │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  1. 🔴 RED: Write failing test first                                           │
│     └── Test MUST fail before implementation                                   │
│     └── If test passes immediately → test is wrong                             │
│                                                                                 │
│  2. 🟢 GREEN: Write minimum code to pass                                       │
│     └── No over-engineering                                                    │
│     └── Just enough to make test green                                         │
│                                                                                 │
│  3. 🔵 REFACTOR: Clean up while keeping green                                  │
│     └── Remove duplication                                                     │
│     └── Improve naming                                                         │
│     └── Tests must stay green                                                  │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Ralph Wiggum Code Review Protocol

Après chaque phase, exécuter:

```bash
# 1. Run all tests
cargo test --all-features

# 2. Run clippy (zero warnings)
cargo clippy -- -D warnings

# 3. Check formatting
cargo fmt --check

# 4. Run codebase audit skill
# /codebase-audit
```

### Review Checklist

| Check | Command | Expected |
|-------|---------|----------|
| Tests pass | `cargo test` | 0 failures |
| No warnings | `cargo clippy` | 0 warnings |
| Formatted | `cargo fmt --check` | No changes |
| No dead code | Manual review | 0 unused imports |
| No TODOs introduced | `grep -r "TODO" src/` | No new TODOs |
| Documentation | `cargo doc` | Builds clean |

---

## Phase 0: Cleanup (Quick Wins)

**Effort:** ~2 hours
**Prerequisite:** None

### Tasks

| # | Task | Command | Verification |
|---|------|---------|--------------|
| 0.1 | Delete coherence/ dead code | `rm -rf novanet-dev/tools/novanet/src/coherence/` | `cargo build` passes |
| 0.2 | Remove coherence from lib.rs | Edit `lib.rs` | `cargo build` passes |
| 0.3 | Bump npm packages | `pnpm version 0.14.0 -ws` | `pnpm list` shows 0.14.0 |
| 0.4 | Install pre-commit hook | `ln -s .claude/hooks/pre-commit-check.sh .git/hooks/pre-commit` | Hook runs on commit |
| 0.5 | Create Nika VERSION file | `echo "0.5.0" > nika-dev/VERSION` | File exists |

### TDD Gate 0

```bash
cd novanet-dev && cargo test --all-features
cd nika-dev/tools/nika && cargo test --all-features
# Both must pass with 0 failures
```

### Ralph Wiggum Review 0

```bash
cargo clippy -- -D warnings  # Both projects
cargo fmt --check            # Both projects
```

---

## Phase 1: Plan A - Token Tracking Fix

**Effort:** ~2 hours
**Prerequisite:** Phase 0 complete
**Plan:** `docs/plans/2026-02-19-plan-a-mvp8-reasoning-capture.md`

### TDD Tasks

| # | Task | Type | File |
|---|------|------|------|
| 1.1 | Write failing test | 🔴 RED | `tests/token_tracking_test.rs` |
| 1.2 | Fix token extraction | 🟢 GREEN | `runtime/executor.rs:380` |
| 1.3 | Verify integration | 🟢 GREEN | Run with real workflow |
| 1.4 | Clean up code | 🔵 REFACTOR | Remove debug code |

### Test (RED)

```rust
// tests/token_tracking_test.rs

#[tokio::test]
async fn test_executor_returns_nonzero_tokens() {
    let workflow = load_workflow("examples/simple-agent.yaml");
    let result = Runner::new(workflow).run().await.unwrap();

    let metadata = result.task_outputs.get("agent_task")
        .unwrap()
        .metadata
        .as_ref()
        .unwrap();

    // MUST be > 0 after real LLM call
    assert!(metadata.input_tokens > 0, "input_tokens was 0");
    assert!(metadata.output_tokens > 0, "output_tokens was 0");
}
```

### Implementation (GREEN)

```rust
// runtime/executor.rs - Fix lines 380-385

// BEFORE (bug)
AgentTurnMetadata {
    input_tokens: 0,
    output_tokens: 0,
    thinking: None,
}

// AFTER (fix)
AgentTurnMetadata {
    input_tokens: usage.input_tokens,
    output_tokens: usage.output_tokens,
    thinking: turn.thinking.clone(),
}
```

### TDD Gate 1

```bash
cargo test token_tracking --all-features
# Must pass with non-zero assertions
```

### Ralph Wiggum Review 1

```bash
cargo clippy -- -D warnings
cargo fmt --check
# Verify no new TODOs added
```

---

## Phase 2: Plan B - Test Coverage Gaps

**Effort:** ~4-6 hours
**Prerequisite:** Phase 1 complete
**Plan:** `docs/plans/2026-02-19-plan-b-test-coverage-gaps.md`

### TDD Tasks

| # | Task | Tests | Type |
|---|------|-------|------|
| 2.1 | Store concurrent access | 3 | 🔴→🟢 |
| 2.2 | Store serialization | 2 | 🔴→🟢 |
| 2.3 | TaskOutputStore | 5 | 🔴→🟢 |
| 2.4 | ConfigStore | 3 | 🔴→🟢 |
| 2.5 | Provider error handling | 3 | 🔴→🟢 |
| 2.6 | Provider model selection | 2 | 🔴→🟢 |
| 2.7 | Provider streaming | 2 | 🔴→🟢 |
| 2.8 | MCP describe tests | 4 | 🔴→🟢 |
| 2.9 | MCP atoms tests | 4 | 🔴→🟢 |

### Execution Order

```
For each task 2.1-2.9:
  1. Write failing test (RED)
  2. Verify test fails
  3. Implement minimum code (GREEN)
  4. Verify test passes
  5. Refactor if needed (REFACTOR)
  6. Run full test suite
```

### TDD Gate 2

```bash
cargo test --all-features
# Expected: 730+ tests (was 703)
# Verify: store, provider, mcp modules have more tests
```

### Ralph Wiggum Review 2

```bash
cargo clippy -- -D warnings
cargo fmt --check
cargo test --all-features -- --nocapture 2>&1 | grep -c "test result"
# Should show increased test count
```

---

## Phase 3: Plan D - Nika TUI Verification

**Effort:** ~2-4 hours
**Prerequisite:** Phase 2 complete
**Plan:** `docs/plans/2026-02-19-plan-d-nika-tui-verification.md`

### TDD Tasks

| # | Task | Type | File |
|---|------|------|------|
| 3.1 | TUI startup test | 🔴→🟢 | `tests/tui_integration_test.rs` |
| 3.2 | Keyboard handling test | 🔴→🟢 | `tests/tui_integration_test.rs` |
| 3.3 | Event processing test | 🔴→🟢 | `tests/tui_integration_test.rs` |
| 3.4 | Panel state test | 🔴→🟢 | `tests/tui_integration_test.rs` |
| 3.5 | Manual verification | Manual | All 4 panels |

### Manual Verification Checklist

```
[ ] Workflow panel renders tree
[ ] Task panel shows details
[ ] Log panel streams events
[ ] Status bar shows progress
[ ] Arrow keys navigate
[ ] Tab switches panels
[ ] q/Esc quits cleanly
[ ] Help (?) shows overlay
```

### TDD Gate 3

```bash
cargo test tui --features tui --all-features
# Expected: 4+ new TUI tests passing
```

### Ralph Wiggum Review 3

```bash
cargo clippy --features tui -- -D warnings
cargo fmt --check
# Manual TUI test: cargo run --features tui -- tui examples/simple.yaml
```

---

## Phase 4: Plan C - QR Code AI Workflows

**Effort:** Variable (depends on scope)
**Prerequisite:** Phase 3 complete
**Plan:** `docs/plans/2026-02-19-plan-c-qrcode-ai-production.md`

### TDD Tasks

| # | Task | Type | Description |
|---|------|------|-------------|
| 4.1 | Design workflow template | Design | YAML structure for page generation |
| 4.2 | Write workflow test | 🔴 RED | Test expected output structure |
| 4.3 | Create workflow | 🟢 GREEN | `workflows/generate-qrcode-page.yaml` |
| 4.4 | Test with fr-FR | 🟢 GREEN | Validate French content |
| 4.5 | Test with es-MX | 🟢 GREEN | Validate Spanish content |
| 4.6 | Validate denomination_forms | 🟢 GREEN | text/title/abbrev/url present |

### Example Workflow

```yaml
# workflows/generate-qrcode-page.yaml
name: generate-qrcode-page
description: Generate native page content for QR Code AI

mcp:
  servers:
    novanet:
      command: "cargo run --manifest-path ../novanet-dev/tools/novanet-mcp/Cargo.toml"

tasks:
  - id: load_context
    invoke: novanet_generate
    params:
      focus_key: "entity:qr-code"
      locale: $locale
      mode: page
      token_budget: 4000
    use:
      ctx: context

  - id: generate_hero
    infer: |
      Generate hero section for QR Code landing page.
      Use denomination_forms from context.
      Target locale: {{locale}}
    context: $ctx
    use:
      hero: output

  - id: generate_features
    infer: |
      Generate features section listing QR code capabilities.
      Use native terminology from context.
    context: $ctx
    use:
      features: output
```

### TDD Gate 4

```bash
cargo run -- run workflows/generate-qrcode-page.yaml --locale=fr-FR
# Verify output contains French content
# Verify denomination_forms used correctly
```

### Ralph Wiggum Review 4

```bash
# Review generated content quality
# Verify no English leaking into localized content
# Check denomination_forms compliance
```

---

## Phase 5: Release

**Prerequisite:** All phases complete, all gates passed

### Tasks

| # | Task | Command |
|---|------|---------|
| 5.1 | Update NovaNet CHANGELOG | Edit `novanet-dev/CHANGELOG.md` |
| 5.2 | Update Nika CHANGELOG | Edit `nika-dev/CHANGELOG.md` |
| 5.3 | Update ROADMAP | Edit `ROADMAP.md` |
| 5.4 | Bump versions | Edit Cargo.toml files |
| 5.5 | Final test run | `cargo test --all-features` both projects |
| 5.6 | Create tags | `git tag -a v0.14.1 -m "..."` |
| 5.7 | Push | `git push origin main --tags` |

### Final Verification

```bash
# NovaNet
cd novanet-dev
cargo test --all-features
cargo clippy -- -D warnings
pnpm test
pnpm type-check

# Nika
cd nika-dev/tools/nika
cargo test --all-features
cargo clippy -- -D warnings
```

---

## Success Metrics

| Metric | Before | After |
|--------|--------|-------|
| Nika tests | 703 | 735+ |
| NovaNet tests | 1,226 | 1,226 |
| Dead code lines | ~1,500 | 0 |
| npm version | 0.13.0 | 0.14.0 |
| Token tracking | Returns 0 | Returns actual |
| TUI coverage | 92% | 95%+ |
| Pre-commit hook | Not installed | Installed |

---

## Execution Commands

```bash
# Start execution
cd /Users/thibaut/supernovae-st/supernovae-agi

# Phase 0
rm -rf novanet-dev/tools/novanet/src/coherence/
# ... (edit lib.rs)
cargo test --all-features

# Phase 1
cd nika-dev/tools/nika
# Write test, implement fix
cargo test token_tracking

# Phase 2
# TDD loop for each module
cargo test store provider mcp

# Phase 3
cargo test tui --features tui
cargo run --features tui -- tui examples/simple.yaml

# Phase 4
cargo run -- run workflows/generate-qrcode-page.yaml --locale=fr-FR

# Phase 5
# Update changelogs, bump versions, tag, push
```

---

## Notes

- **Never skip TDD gates** - If tests fail, fix before proceeding
- **Ralph Wiggum reviews are mandatory** - No clippy warnings allowed
- **Commit after each phase** - Atomic commits with conventional format
- **Ask questions if blocked** - Use AskUserQuestion tool
