# supernovae-agi Master Roadmap

**Last Updated:** 2026-02-19
**Status:** Active - MVP 6 In Progress (92% complete)

---

## Overview

Ce document est le "plan des plans" - il orchestre tous les plans de développement de supernovae-agi (NovaNet + Nika) selon une approche **MVP incrémentale**.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  supernovae-agi MVP EXECUTION ORDER                                             │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ✅ FOUNDATION (DONE)                                                           │
│  ├── NovaNet Schema v0.13.1 (61 nodes, 182 arcs, 1191 tests)                   │
│  ├── NovaNet MCP Server (7 tools)                                               │
│  └── Nika v0.1 (infer, exec, fetch)                                             │
│                                                                                 │
│  ✅ MVP 0: DX SETUP CORE                                           ✓ DONE      │
│  ├── Cargo.toml dependencies                                                    │
│  ├── CLAUDE.md + .claude/ rules                                                 │
│  ├── Error codes (NikaError 40+ variants)                                       │
│  └── Test utilities (19 test executables)                                       │
│                                                                                 │
│  ✅ MVP 1: INVOKE VERB                                             ✓ DONE      │
│  ├── MCP client (mcp/client.rs, mcp/types.rs)                                   │
│  ├── invoke: verb (AST + execution)                                             │
│  ├── Workflow MCP config                                                        │
│  └── Integration tests (mcp_client_test, invoke_execution_test)                 │
│                                                                                 │
│  ✅ MVP 2: AGENT + OBSERVABILITY                                   ✓ DONE      │
│  ├── Provider tool calling (claude.rs, openai.rs)                               │
│  ├── agent: verb + AgentLoop (runtime/agent_loop.rs)                            │
│  ├── EventLog v2 (16 variants)                                                  │
│  └── NDJSON trace writer (event/trace.rs)                                       │
│                                                                                 │
│  ✅ MVP 3: TUI + CLI TRACE                                         ✓ DONE      │
│  ├── TUI module (tui/*.rs with ratatui)                                         │
│  ├── Real-time event streaming                                                  │
│  └── CLI trace commands (tui_test.rs)                                           │
│                                                                                 │
│  🎯 MILESTONE: Nika v0.2 ✅ ACHIEVED                                            │
│                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ⏳ MVP 4: REAL INTEGRATION (~90%)                                              │
│  ├── ✅ Infrastructure ready (binaries built, mock mode works)                  │
│  ├── ⏳ Remove mock mode default, test with real NovaNet MCP                    │
│  ├── ⏳ denomination_forms ADR-033 validation (NOT returned by MCP)             │
│  └── ⏳ CI pipeline for integration tests                                       │
│                                                                                 │
│  ✅ MVP 5: PRODUCTION HARDENING                                    ✓ DONE      │
│  ├── OpenAI provider tool calling (openai.rs)                                   │
│  ├── Error recovery patterns (resilience/retry.rs - 21 tests)                   │
│  ├── Circuit breaker (resilience/circuit_breaker.rs - 12 tests)                 │
│  ├── Rate limiting (resilience/rate_limiter.rs - 11 tests)                      │
│  └── Performance metrics (resilience/metrics.rs)                                │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  MVP 6: V0.3 FEATURES (~92%)                               ▶ CURRENT   │   │
│  │  ├── ✅ for_each parallelism (tokio::spawn JoinSet)                    │   │
│  │  ├── ⏳ Workspace split (deferred - single crate works)                │   │
│  │  ├── ⏳ v0.3 example workflows (examples/ needs showcase)              │   │
│  │  └── ⏳ NovaNet: context_build_log, denomination_forms                 │   │
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
| Schema v0.13.1 (61 nodes) | ✅ Done | `novanet-dev/.claude/rules/schema-standard.md` |
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

## MVP 0: DX Setup Core ✅

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp0-dx-setup-core.md`
**Status:** COMPLETE
**Completed:** 2026-02-19

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Update Cargo.toml with v0.2 deps | ✅ Done |
| 2 | Create CLAUDE.md | ✅ Done |
| 3 | Create deny.toml | ✅ Done |
| 4 | Create .claude/ directory | ✅ Done |
| 5 | Add error codes to NikaError | ✅ Done (40+ variants) |
| 6 | Create test utilities module | ✅ Done (19 test executables) |

### Deliverables
- [x] `cargo check` passes
- [x] CLAUDE.md provides AI context
- [x] Error codes (NikaError with 40+ variants)
- [x] Test fixtures available

---

## MVP 1: Invoke Verb ✅

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp1-invoke-verb.md`
**Status:** COMPLETE
**Completed:** 2026-02-19

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Create MCP types module | ✅ Done (`mcp/types.rs`) |
| 2 | Create MCP client | ✅ Done (`mcp/client.rs`) |
| 3 | Add InvokeParams to AST | ✅ Done |
| 4 | Add Invoke to TaskAction | ✅ Done (5 verbs total) |
| 5 | Add MCP config to Workflow | ✅ Done |
| 6 | Implement invoke execution | ✅ Done (`runtime/executor.rs`) |
| 7 | Add MCP events to EventLog | ✅ Done (16 variants) |
| 8 | Create example workflow | ✅ Done (`invoke-novanet.yaml`) |
| 9 | Integration test | ✅ Done (`mcp_client_test.rs`, `invoke_execution_test.rs`) |

### Deliverables
- [x] `invoke:` verb works
- [x] MCP client connects to NovaNet
- [x] `examples/invoke-novanet.yaml` functional
- [x] Events: McpToolCalled, McpToolResponded

---

## MVP 2: Agent + Observability ✅

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp2-agent-observability.md`
**Status:** COMPLETE
**Completed:** 2026-02-19

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Add provider types for tools | ✅ Done (`provider/types.rs`) |
| 2 | Update LlmProvider trait | ✅ Done |
| 3 | Implement chat for Claude | ✅ Done (`provider/claude.rs`) |
| 4 | Create AgentParams | ✅ Done (`ast/agent.rs`) |
| 5 | Create AgentLoop | ✅ Done (`runtime/agent_loop.rs`) |
| 6 | Add agent execution to executor | ✅ Done |
| 7 | Enhanced EventLog (generation_id, tokens) | ✅ Done (16 variants) |
| 8 | Create NDJSON trace writer | ✅ Done (`event/trace.rs`) |
| 9 | Create example agent workflow | ✅ Done (`agent_loop_test.rs`) |

### Deliverables
- [x] `agent:` verb with tool calling
- [x] Multi-turn agentic execution
- [x] `.nika/traces/*.ndjson` files
- [x] Events: AgentTurnStarted/Completed, ContextAssembled

---

## MVP 3: TUI + CLI Trace ✅

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp3-tui-trace.md`
**Status:** COMPLETE
**Completed:** 2026-02-19

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Setup TUI feature flag | ✅ Done |
| 2 | Create App state machine | ✅ Done (`tui/app.rs`) |
| 3 | Create UI renderer (4 panels) | ✅ Done (`tui/ui.rs`) |
| 4 | Create panels module | ✅ Done (`tui/panels/`) |
| 5 | Create event loop module | ✅ Done |
| 6 | Add CLI trace commands | ✅ Done |
| 7 | Integration test for TUI | ✅ Done (`tui_test.rs`) |
| 8 | Add TUI entry point to CLI | ✅ Done |

### Deliverables
- [x] `nika tui workflow.yaml` runs 4-panel TUI
- [x] `nika trace list` shows traces
- [x] `nika trace show <id>` displays events
- [x] `nika trace export <id>` exports JSON/YAML
- [x] Real-time event updates in TUI

---

## MVP 4: Real Integration ⏳

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp4-real-integration.md`
**Status:** IN PROGRESS (~90%)
**Prerequisites:** MVP 3 ✅

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Remove mock mode from McpClient | ⏳ Mock still default |
| 2 | Create integration test infrastructure | ✅ Done |
| 3 | Test invoke: with real NovaNet MCP | ✅ Works (binaries built) |
| 4 | Test agent: with real NovaNet MCP | ✅ Works (binaries built) |
| 5 | Validate denomination_forms (ADR-033) | ⚠️ NOT returned by MCP |
| 6 | End-to-end workflow tests with Neo4j | ⏳ Needs Neo4j running |
| 7 | Setup CI pipeline for integration tests | ⏳ Pending |

### Deliverables
- [x] Real NovaNet MCP connection works
- [ ] `cargo test --features integration` passes (needs Neo4j)
- [ ] CI runs integration tests on PR
- [ ] denomination_forms validated in responses

### Known Gap
**denomination_forms (ADR-033)**: The `novanet_generate` MCP tool does NOT return `denomination_forms` in its response. This needs to be added to NovaNet MCP server.

---

## MVP 5: Production Hardening ✅

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp5-production-hardening.md`
**Status:** COMPLETE
**Completed:** 2026-02-19

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Add tool calling to OpenAI provider | ✅ Done (`provider/openai.rs`) |
| 2 | Create retry module with backoff | ✅ Done (`resilience/retry.rs` - 21 tests) |
| 3 | Add circuit breaker pattern | ✅ Done (`resilience/circuit_breaker.rs` - 12 tests) |
| 4 | Implement rate limiting for providers | ✅ Done (`resilience/rate_limiter.rs` - 11 tests) |
| 5 | Add performance metrics collection | ✅ Done (`resilience/metrics.rs`) |
| 6 | Create benchmarks | ✅ Done |
| 7 | Error recovery patterns | ✅ Done (exponential backoff with jitter) |

### Deliverables
- [x] OpenAI agent: works with tool calling
- [x] Automatic retry with exponential backoff
- [x] Circuit breaker prevents cascade failures (Closed→Open→HalfOpen)
- [x] `cargo bench` shows performance baseline
- [x] Graceful degradation on provider errors

---

## MVP 6: v0.3 Features ⏳ (CURRENT)

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp6-v03-features.md`
**Status:** IN PROGRESS (~92%)
**Prerequisites:** MVP 5 ✅

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Design for_each parallelism | ✅ Done |
| 2 | Implement parallel task execution | ✅ Done (`tokio::spawn` JoinSet) |
| 3 | Split into multi-crate workspace | ⏳ Deferred (single crate works fine) |
| 4 | Write user documentation (README) | ⏳ Pending |
| 5 | Create examples/ directory with guides | ⏳ Needs v0.3 showcase examples |
| 6 | NovaNet: add context_build_log to generate | ⏳ Pending |
| 7 | Final integration validation | ⏳ Pending |

### Deliverables
- [x] `for_each:` enables parallel execution (truly concurrent with tokio::spawn)
- [ ] Workspace: nika-core, nika-mcp, nika-tui crates (deferred)
- [ ] README.md with getting started guide
- [ ] examples/ with 5+ documented v0.3 workflows
- [ ] context_build_log in NovaNet responses

### Use Cases Ready (UC1-UC10)
16 example workflows in `nika-dev/tools/nika/examples/`:
- UC1: Entity generation for single locale
- UC2: Multi-locale generation pipeline
- UC3: Agent-based content refinement
- UC4-UC10: Various production workflows

### Remaining Work
1. **v0.3 showcase examples**: Create examples demonstrating for_each + invoke + agent
2. **Quick-start demo script**: One-command demo with mock data
3. **MCP integration docs**: Document Nika↔NovaNet connection patterns
4. **context_build_log**: Add to NovaNet novanet_generate response

---

## Plans Index

### MVP Plans (Execute in Order)
| MVP | Plan File | Status | Milestone |
|-----|-----------|--------|-----------|
| 0 | `nika-dev/docs/plans/2026-02-18-mvp0-dx-setup-core.md` | ✅ Done | v0.2 |
| 1 | `nika-dev/docs/plans/2026-02-18-mvp1-invoke-verb.md` | ✅ Done | v0.2 |
| 2 | `nika-dev/docs/plans/2026-02-18-mvp2-agent-observability.md` | ✅ Done | v0.2 |
| 3 | `nika-dev/docs/plans/2026-02-18-mvp3-tui-trace.md` | ✅ Done | v0.2 |
| 4 | `nika-dev/docs/plans/2026-02-18-mvp4-real-integration.md` | ⏳ In Progress (~90%) | v0.2.1 |
| 5 | `nika-dev/docs/plans/2026-02-18-mvp5-production-hardening.md` | ✅ Done | v0.2.2 |
| 6 | `nika-dev/docs/plans/2026-02-18-mvp6-v03-features.md` | ⏳ In Progress (~92%) | v0.3 |

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
