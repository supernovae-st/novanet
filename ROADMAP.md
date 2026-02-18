# supernovae-agi Master Roadmap

**Last Updated:** 2026-02-18
**Status:** Active - MVP-Based Execution

---

## Overview

Ce document est le "plan des plans" - il orchestre tous les plans de développement de supernovae-agi (NovaNet + Nika) selon une approche **MVP incrémentale**.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  supernovae-agi MVP EXECUTION ORDER                                                    │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ✅ FOUNDATION (DONE)                                                           │
│  ├── NovaNet Schema v0.13.1                                                     │
│  ├── NovaNet MCP Server (7 tools)                                               │
│  └── Nika v0.1 (infer, exec, fetch)                                             │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  MVP 0: DX SETUP CORE (~2-3h)                              ▶ CURRENT   │   │
│  │  ├── Cargo.toml dependencies                                            │   │
│  │  ├── CLAUDE.md + .claude/ rules                                         │   │
│  │  ├── Error codes (NIKA-000 to NIKA-112)                                 │   │
│  │  └── Test utilities                                                     │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                          │                                                      │
│                          ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  MVP 1: INVOKE VERB (~4-6h)                                             │   │
│  │  ├── MCP client (types, client)                                         │   │
│  │  ├── invoke: verb (AST + execution)                                     │   │
│  │  ├── Workflow MCP config                                                │   │
│  │  └── Integration test with NovaNet                                      │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                          │                                                      │
│                          ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  MVP 2: AGENT + OBSERVABILITY (~6-8h)                                   │   │
│  │  ├── Provider tool calling                                              │   │
│  │  ├── agent: verb + AgentLoop                                            │   │
│  │  ├── EventLog v2 (16 variants)                                          │   │
│  │  └── NDJSON trace writer                                                │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                          │                                                      │
│                          ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  MVP 3: TUI + CLI TRACE (~8-10h)                                        │   │
│  │  ├── 4-panel TUI (ratatui)                                              │   │
│  │  ├── Real-time event streaming                                          │   │
│  │  └── CLI trace commands (list, show, export)                            │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  🎯 MILESTONE: Nika v0.2 with full NovaNet observability                       │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                          │                                                      │
│                          ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  MVP 4: REAL INTEGRATION (~4-6h)                                       │   │
│  │  ├── Remove mock mode, test with real NovaNet MCP                      │   │
│  │  ├── End-to-end tests with Neo4j running                               │   │
│  │  ├── denomination_forms ADR-033 validation                             │   │
│  │  └── CI pipeline for integration tests                                 │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                          │                                                      │
│                          ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  MVP 5: PRODUCTION HARDENING (~6-8h)                                   │   │
│  │  ├── OpenAI provider tool calling                                      │   │
│  │  ├── Error recovery patterns (retry, circuit breaker)                  │   │
│  │  ├── Rate limiting / exponential backoff                               │   │
│  │  └── Performance metrics & benchmarks                                  │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                          │                                                      │
│                          ▼                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  MVP 6: V0.3 FEATURES (~8-10h)                                         │   │
│  │  ├── for_each parallelism (parallel task execution)                    │   │
│  │  ├── Workspace split (multi-crate Cargo)                               │   │
│  │  ├── User documentation (README, examples/, guides)                    │   │
│  │  └── NovaNet MCP enhancements (context_build_log)                      │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  🎯 RESULT: Nika v0.3 production-ready with full feature set                   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Foundation (COMPLETED)

Ces éléments sont déjà implémentés et fonctionnels.

### NovaNet
| Item | Status | Location |
|------|--------|----------|
| Schema v0.13.1 (62 nodes) | ✅ Done | `novanet-dev/.claude/rules/schema-standard.md` |
| Neo4j + Seed data | ✅ Done | `novanet-dev/packages/db/` |
| MCP Server (7 tools) | ✅ Done | `novanet-dev/tools/novanet-mcp/` |
| TUI (novanet tui) | ✅ Done | `novanet-dev/tools/novanet/src/tui/` |

### Nika v0.1
| Item | Status | Location |
|------|--------|----------|
| Core (AST, DAG, Runtime) | ✅ Done | `nika-dev/tools/nika/src/` |
| Verbs: infer, exec, fetch | ✅ Done | `nika-dev/tools/nika/src/ast/action.rs` |
| EventLog (10 variants) | ✅ Done | `nika-dev/tools/nika/src/event/log.rs` |
| Providers: Claude, OpenAI | ✅ Done | `nika-dev/tools/nika/src/provider/` |

---

## MVP 0: DX Setup Core

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp0-dx-setup-core.md`
**Estimated:** 2-3 hours
**Prerequisites:** None

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Update Cargo.toml with v0.2 deps | ⏳ Pending |
| 2 | Create CLAUDE.md | ⏳ Pending |
| 3 | Create deny.toml | ⏳ Pending |
| 4 | Create .claude/ directory | ⏳ Pending |
| 5 | Add error codes to NikaError | ⏳ Pending |
| 6 | Create test utilities module | ⏳ Pending |

### Deliverables
- [ ] `cargo check` passes
- [ ] CLAUDE.md provides AI context
- [ ] Error codes NIKA-000 to NIKA-112
- [ ] Test fixtures available

---

## MVP 1: Invoke Verb

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp1-invoke-verb.md`
**Estimated:** 4-6 hours
**Prerequisites:** MVP 0

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Create MCP types module | ⏳ Pending |
| 2 | Create MCP client | ⏳ Pending |
| 3 | Add InvokeParams to AST | ⏳ Pending |
| 4 | Add Invoke to TaskAction | ⏳ Pending |
| 5 | Add MCP config to Workflow | ⏳ Pending |
| 6 | Implement invoke execution | ⏳ Pending |
| 7 | Add MCP events to EventLog | ⏳ Pending |
| 8 | Create example workflow | ⏳ Pending |
| 9 | Integration test | ⏳ Pending |

### Deliverables
- [ ] `invoke:` verb works
- [ ] MCP client connects to NovaNet
- [ ] `examples/invoke-novanet.yaml` functional
- [ ] Events: McpToolCalled, McpToolResponded

---

## MVP 2: Agent + Observability

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp2-agent-observability.md`
**Estimated:** 6-8 hours
**Prerequisites:** MVP 1

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Add provider types for tools | ⏳ Pending |
| 2 | Update LlmProvider trait | ⏳ Pending |
| 3 | Implement chat for Claude | ⏳ Pending |
| 4 | Create AgentParams | ⏳ Pending |
| 5 | Create AgentLoop | ⏳ Pending |
| 6 | Add agent execution to executor | ⏳ Pending |
| 7 | Enhanced EventLog (generation_id, tokens) | ⏳ Pending |
| 8 | Create NDJSON trace writer | ⏳ Pending |
| 9 | Create example agent workflow | ⏳ Pending |

### Deliverables
- [ ] `agent:` verb with tool calling
- [ ] Multi-turn agentic execution
- [ ] `.nika/traces/*.ndjson` files
- [ ] Events: AgentTurnStarted/Completed, ContextAssembled

---

## MVP 3: TUI + CLI Trace

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp3-tui-trace.md`
**Estimated:** 8-10 hours
**Prerequisites:** MVP 2

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Setup TUI feature flag | ⏳ Pending |
| 2 | Create App state machine | ⏳ Pending |
| 3 | Create UI renderer (4 panels) | ⏳ Pending |
| 4 | Create panels module | ⏳ Pending |
| 5 | Create event loop module | ⏳ Pending |
| 6 | Add CLI trace commands | ⏳ Pending |
| 7 | Integration test for TUI | ⏳ Pending |
| 8 | Add TUI entry point to CLI | ⏳ Pending |

### Deliverables
- [ ] `nika tui workflow.yaml` runs 4-panel TUI
- [ ] `nika trace list` shows traces
- [ ] `nika trace show <id>` displays events
- [ ] `nika trace export <id>` exports JSON/YAML
- [ ] Real-time event updates in TUI

---

## MVP 4: Real Integration

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp4-real-integration.md`
**Prerequisites:** MVP 3

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Remove mock mode from McpClient | ⏳ Pending |
| 2 | Create integration test infrastructure | ⏳ Pending |
| 3 | Test invoke: with real NovaNet MCP | ⏳ Pending |
| 4 | Test agent: with real NovaNet MCP | ⏳ Pending |
| 5 | Validate denomination_forms (ADR-033) | ⏳ Pending |
| 6 | End-to-end workflow tests with Neo4j | ⏳ Pending |
| 7 | Setup CI pipeline for integration tests | ⏳ Pending |

### Deliverables
- [ ] Real NovaNet MCP connection works
- [ ] `cargo test --features integration` passes
- [ ] CI runs integration tests on PR
- [ ] denomination_forms validated in responses

---

## MVP 5: Production Hardening

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp5-production-hardening.md`
**Prerequisites:** MVP 4

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Add tool calling to OpenAI provider | ⏳ Pending |
| 2 | Create retry module with backoff | ⏳ Pending |
| 3 | Add circuit breaker pattern | ⏳ Pending |
| 4 | Implement rate limiting for providers | ⏳ Pending |
| 5 | Add performance metrics collection | ⏳ Pending |
| 6 | Create benchmarks | ⏳ Pending |
| 7 | Error recovery patterns | ⏳ Pending |

### Deliverables
- [ ] OpenAI agent: works with tool calling
- [ ] Automatic retry with exponential backoff
- [ ] Circuit breaker prevents cascade failures
- [ ] `cargo bench` shows performance baseline
- [ ] Graceful degradation on provider errors

---

## MVP 6: v0.3 Features

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp6-v03-features.md`
**Prerequisites:** MVP 5

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Design for_each parallelism | ⏳ Pending |
| 2 | Implement parallel task execution | ⏳ Pending |
| 3 | Split into multi-crate workspace | ⏳ Pending |
| 4 | Write user documentation (README) | ⏳ Pending |
| 5 | Create examples/ directory with guides | ⏳ Pending |
| 6 | NovaNet: add context_build_log to generate | ⏳ Pending |
| 7 | Final integration validation | ⏳ Pending |

### Deliverables
- [ ] `for_each:` enables parallel execution
- [ ] Workspace: nika-core, nika-mcp, nika-tui crates
- [ ] README.md with getting started guide
- [ ] examples/ with 5+ documented workflows
- [ ] context_build_log in NovaNet responses

---

## Plans Index

### MVP Plans (Execute in Order)
| MVP | Plan File | Status | Milestone |
|-----|-----------|--------|-----------|
| 0 | `nika-dev/docs/plans/2026-02-18-mvp0-dx-setup-core.md` | ⏳ Pending | v0.2 |
| 1 | `nika-dev/docs/plans/2026-02-18-mvp1-invoke-verb.md` | ⏳ Pending | v0.2 |
| 2 | `nika-dev/docs/plans/2026-02-18-mvp2-agent-observability.md` | ⏳ Pending | v0.2 |
| 3 | `nika-dev/docs/plans/2026-02-18-mvp3-tui-trace.md` | ⏳ Pending | v0.2 |
| 4 | `nika-dev/docs/plans/2026-02-18-mvp4-real-integration.md` | ⏳ Pending | v0.2.1 |
| 5 | `nika-dev/docs/plans/2026-02-18-mvp5-production-hardening.md` | ⏳ Pending | v0.2.2 |
| 6 | `nika-dev/docs/plans/2026-02-18-mvp6-v03-features.md` | ⏳ Pending | v0.3 |

### Reference Documents
| Document | Location | Description |
|----------|----------|-------------|
| Observability Design | `supernovae-agi/docs/brainstorming/2026-02-18-nika-novanet-observability.md` | Full design spec |
| Integration Research | `nika-dev/docs/research/2026-02-18-nika-novanet-integration.md` | MCP patterns |
| DX Setup (Full) | `nika-dev/docs/plans/2026-02-18-nika-v02-dx-setup.md` | Extended DX reference |
| NovaNet Integration (Full) | `nika-dev/docs/plans/2026-02-18-nika-v02-novanet-integration.md` | Extended reference |

### Legacy Plans (Review/Archive)
| Category | Files | Recommendation |
|----------|-------|----------------|
| Nika v0.1 refactoring | `2026-01-0*.md` | Review for v0.2 applicability |
| NovaNet schema v9-v11 | 20+ files | Archive (superseded by v0.13.1) |
| NovaNet Studio UI | 20+ files | Review status |

---

## Dependencies

```
┌──────────────────────────────────────────────────────────────────────────┐
│  RUST DEPENDENCIES (Cargo.toml)                                          │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  CORE (existing):                                                        │
│  ├── tokio, serde, serde_yaml, serde_json                               │
│  ├── clap, reqwest, regex, colored                                      │
│  ├── thiserror, anyhow, tracing                                         │
│  └── parking_lot, dashmap, rustc-hash                                   │
│                                                                          │
│  NEW for v0.2:                                                           │
│  ├── rmcp = "0.1" (MCP client)                                          │
│  ├── ratatui = "0.29" (TUI, optional)                                   │
│  ├── crossterm = "0.28" (TUI, optional)                                 │
│  ├── uuid = "1.0" (call IDs)                                            │
│  ├── xxhash-rust = "0.8" (prompt hashing)                               │
│  ├── chrono = "0.4" (timestamps)                                        │
│  └── rand = "0.8" (generation IDs)                                      │
│                                                                          │
│  DEV-DEPENDENCIES:                                                       │
│  ├── proptest = "1.4" (fuzzing)                                         │
│  ├── insta = "1.34" (snapshots)                                         │
│  └── pretty_assertions = "1.4"                                          │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## Architecture Target

```
┌─────────────────────────────────────────────────────────────────────────┐
│  NIKA v0.2 ARCHITECTURE                                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  tools/nika/src/                                                        │
│  ├── main.rs              # CLI (run, validate, tui, trace)            │
│  ├── lib.rs               # Public API                                 │
│  ├── error.rs             # NikaError (40+ variants with codes)        │
│  │                                                                      │
│  ├── ast/                 # YAML → Rust structs                        │
│  │   ├── workflow.rs      # Workflow + MCP config                      │
│  │   ├── action.rs        # TaskAction (5 variants)                    │
│  │   ├── invoke.rs        # InvokeParams                               │
│  │   └── agent.rs         # AgentParams                                │
│  │                                                                      │
│  ├── mcp/                 # MCP Client (NEW v0.2)                      │
│  │   ├── types.rs         # McpConfig, ToolResult                      │
│  │   └── client.rs        # McpClient                                  │
│  │                                                                      │
│  ├── runtime/             # Execution engine                           │
│  │   ├── executor.rs      # Task dispatch (5 verbs)                    │
│  │   ├── runner.rs        # Workflow orchestration                     │
│  │   └── agent_loop.rs    # Agentic execution (NEW v0.2)               │
│  │                                                                      │
│  ├── provider/            # LLM providers                              │
│  │   ├── types.rs         # Message, ToolCall, Usage (NEW)             │
│  │   ├── claude.rs        # +chat() with tools                         │
│  │   └── openai.rs        # +chat() with tools                         │
│  │                                                                      │
│  ├── event/               # Observability                              │
│  │   ├── log.rs           # EventLog (16 variants)                     │
│  │   └── trace.rs         # NDJSON writer (NEW v0.2)                   │
│  │                                                                      │
│  └── tui/                 # Terminal UI (feature-gated)                │
│      ├── app.rs           # State machine                              │
│      ├── ui.rs            # 4-panel renderer                           │
│      └── panels/          # Individual panels                          │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## How to Execute

### For AI (Claude Code)

```
To execute a plan:
1. Read the plan file
2. Use superpowers:executing-plans skill
3. Execute task-by-task with TDD
4. Update ROADMAP status after each MVP
```

### For Humans

```bash
# Check current status
cat ROADMAP.md | grep "▶ CURRENT"

# Execute MVP 0
cd nika-dev/tools/nika
# Follow 2026-02-18-mvp0-dx-setup-core.md

# Execute MVP 1
# Follow 2026-02-18-mvp1-invoke-verb.md

# And so on...
```

---

## Success Criteria

### MVP 0 Done When:
- `cargo check` passes with new deps
- `cargo test error` passes
- CLAUDE.md exists

### MVP 1 Done When:
- `cargo run -- run examples/invoke-novanet.yaml` succeeds (with mock)
- `invoke:` verb parses and executes
- MCP events logged

### MVP 2 Done When:
- `cargo run -- run examples/agent-novanet.yaml` succeeds (with mock)
- `.nika/traces/` contains NDJSON files
- `agent:` verb with tool calling works

### MVP 3 Done When:
- `cargo run -- tui examples/invoke-novanet.yaml` shows 4-panel UI
- `cargo run -- trace list` shows traces
- Real-time updates work

### MVP 4 Done When:
- `cargo test --features integration` passes with real NovaNet
- denomination_forms appear correctly in responses
- CI pipeline runs integration tests

### MVP 5 Done When:
- OpenAI provider works with `agent:` verb
- Retry/backoff handles transient failures
- `cargo bench` baseline established

### MVP 6 Done When:
- `for_each:` executes tasks in parallel
- Workspace split into 3 crates compiles
- README + examples/ fully documented
- v0.3.0 tag created

---

## Notes

- **Parallelizable:** NovaNet MCP extensions (context_build_log) can be done in parallel
- **Optional:** Workspace split (multi-crate) deferred to v0.3 if complexity justifies
- **Testing:** TDD for all MVPs - write failing test first
- **Commits:** Atomic commits per task, conventional commit format
