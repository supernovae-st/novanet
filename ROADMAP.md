# supernovae-agi Master Roadmap

**Last Updated:** 2026-02-19
**Status:** Active - MVP 0-7 Complete, MVP 8 RLM Enhancements Next

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
│  ├── NovaNet MCP Server (8 tools)                                               │
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
│  ✅ MVP 4: REAL INTEGRATION                                        ✓ DONE      │
│  ├── ✅ NovaNet MCP v0.14.0 (denomination_forms, context_build_log)            │
│  ├── ✅ Real MCP connection tested (rig_integration_test.rs)                   │
│  └── ✅ ADR-033 + ADR-035 compliance validated                                 │
│                                                                                 │
│  ✅ MVP 5: PRODUCTION HARDENING                                    ✓ DONE      │
│  ├── OpenAI provider tool calling (openai.rs)                                   │
│  ├── Error recovery patterns (resilience/retry.rs - 21 tests)                   │
│  ├── Circuit breaker (resilience/circuit_breaker.rs - 12 tests)                 │
│  ├── Rate limiting (resilience/rate_limiter.rs - 11 tests)                      │
│  └── Performance metrics (resilience/metrics.rs)                                │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  ✅ MVP 6: V0.3 FEATURES                                       ✓ DONE      │
│  ├── ✅ for_each parallelism (tokio::spawn JoinSet)                         │
│  ├── ✅ rig-core 0.31 + rmcp 0.16 in Cargo.toml                             │
│  ├── ✅ RigProvider wrapper (provider/rig.rs - 761 lines)                   │
│  ├── ✅ v0.3 showcase examples (4 demos + UC1-UC10)                         │
│  └── ✅ examples/README.md quick-start guide                                │
│                                                                                 │
│  🎯 MILESTONE: Nika v0.3 ✅ ACHIEVED                                            │
│                                                                                 │
│  ✅ MVP 7: RIG-CORE MIGRATION                                    ✓ DONE      │
│  ├── ✅ RigAgentLoop with rig::AgentBuilder (runtime/rig_agent_loop.rs)      │
│  ├── ✅ Deleted ClaudeProvider, OpenAIProvider, old AgentLoop                │
│  ├── ✅ RigProvider wrapper (provider/rig.rs)                                │
│  ├── ✅ NikaMcpTool implements rig::ToolDyn                                  │
│  └── ✅ 679 tests passing, v0.4.1 complete                                   │
│                                                                                 │
│  🎯 MILESTONE: Nika v0.4 ✅ ACHIEVED (pure rig-core)                          │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────┐   │
│  │  MVP 8: RLM ENHANCEMENTS (60%)                             ▶ CURRENT   │   │
│  │  ├── ✅ Phase 1: Reasoning capture (thinking field in AgentTurn)       │   │
│  │  ├── ⏳ Phase 2: Nested agents (spawn_agent internal tool)             │   │
│  │  ├── ⏳ Phase 3: Schema introspection (novanet_introspect MCP)         │   │
│  │  ├── ✅ Phase 4: Dynamic decomposition (decompose: modifier)           │   │
│  │  └── ✅ Phase 5: Lazy context loading (lazy: binding modifier)         │   │
│  └─────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  🎯 RESULT: Nika v0.5 with full RLM-on-KG capabilities                        │
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
| MCP Server (8 tools) | ✅ Done | `novanet-dev/tools/novanet-mcp/` |
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

## MVP 4: Real Integration ✅

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp4-real-integration.md`
**Status:** COMPLETE
**Completed:** 2026-02-19

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Remove mock mode from McpClient | ✅ Done (explicit mode required) |
| 2 | Create integration test infrastructure | ✅ Done |
| 3 | Test invoke: with real NovaNet MCP | ✅ Works (binaries built) |
| 4 | Test agent: with real NovaNet MCP | ✅ Works (binaries built) |
| 5 | Validate denomination_forms (ADR-033) | ✅ Done (NovaNet MCP v0.14.0) |
| 6 | Validate context_build_log (ADR-035) | ✅ Done (NovaNet MCP v0.14.0) |
| 7 | End-to-end workflow tests with Neo4j | ✅ Done (rig_integration_test.rs) |

### Deliverables
- [x] Real NovaNet MCP connection works
- [x] `cargo test` passes (602 tests)
- [x] denomination_forms returned by novanet_generate
- [x] context_build_log returned for observability

### NovaNet MCP v0.14.0 (2026-02-19)

All critical gaps resolved:

| Feature | Status | Location |
|---------|--------|----------|
| `denomination_forms` (ADR-033) | ✅ Implemented | `generate.rs:127-174` |
| `context_build_log` (ADR-035) | ✅ Implemented | `generate.rs:109-125` |
| 7/7 MCP tools complete | ✅ Done | `tools/novanet-mcp/src/tools/` |

### Remaining Polish (Non-Blocking)

| Item | Severity | Status |
|------|----------|--------|
| TUI→Runner connection | 🟡 Medium | Deferred to v0.3.1 |
| UC1-UC5 transport paths | 🟢 Low | Documentation update |
| Mock client 7/7 tools | 🟢 Low | Nice-to-have |

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

## MVP 6: v0.3 Features ✅

**Plan:** `nika-dev/docs/plans/2026-02-18-mvp6-v03-features.md`
**Status:** COMPLETE
**Completed:** 2026-02-19
**Prerequisites:** MVP 5 ✅, MVP 4 ✅

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Design for_each parallelism | ✅ Done |
| 2 | Implement parallel task execution | ✅ Done (`tokio::spawn` JoinSet) |
| 3 | Add rig-core 0.31 + rmcp 0.16 | ✅ Done (Cargo.toml) |
| 4 | Create RigProvider wrapper | ✅ Done (`provider/rig.rs` - 761 lines, 50+ tests) |
| 5 | v0.3 showcase examples | ✅ Done (4 v03-* demos) |
| 6 | NovaNet: context_build_log + denomination_forms | ✅ Done (MCP v0.14.0) |
| 7 | examples/README.md quick-start guide | ✅ Done |

### Deliverables
- [x] `for_each:` enables parallel execution (truly concurrent with tokio::spawn)
- [x] rig-core 0.31 in Cargo.toml with `rmcp` feature
- [x] RigProvider wrapper ready for AgentLoop migration
- [x] NovaNet MCP v0.14.0 with full ADR-033/035 compliance
- [x] examples/README.md with getting started guide
- [x] v0.3 showcase examples with for_each + invoke + agent

### v0.3 Showcase Examples
4 feature demos in `nika-dev/tools/nika/examples/`:
- `v03-parallel-locales.yaml` - for_each with concurrency: 5
- `v03-denomination-forms.yaml` - ADR-033 prescriptive naming
- `v03-entity-pipeline.yaml` - invoke + for_each + infer pipeline
- `v03-agent-with-tools.yaml` - multi-turn agent with MCP tools

Plus UC1-UC10 production workflow patterns.

---

## MVP 7: Rig-Core Migration ✅ (DONE)

**Plan:** `docs/plans/2026-02-19-rig-core-migration.md`
**Status:** COMPLETE (100%)
**Completed:** 2026-02-19
**Prerequisites:** MVP 6 ✅
**Effort:** ~8-12 hours

### Overview
Replace custom LLM provider implementations with `rig-core` v0.31.0 for:
- Native `rmcp` v0.16 integration via `.rmcp_tools()`
- 20+ built-in LLM providers
- Built-in retry, streaming, agent workflows
- ~1,420 lines code reduction

### Tasks
| Task | Description | Status |
|------|-------------|--------|
| 1 | Dependencies in Cargo.toml (rig-core 0.31, rmcp 0.16) | ✅ Done |
| 2 | RigProvider wrapper (provider/rig.rs) | ✅ Done |
| 3 | RigAgentLoop with rig::AgentBuilder | ✅ Done |
| 4 | Delete ClaudeProvider, OpenAIProvider | ✅ Done |
| 5 | Delete old AgentLoop | ✅ Done |
| 6 | Update tests for RigAgentLoop | ✅ Done |
| 7 | NikaMcpTool implements rig::ToolDyn | ✅ Done |

### Files Removed (v0.4)
| File | Status |
|------|--------|
| `provider/claude.rs` | ✅ Deleted |
| `provider/openai.rs` | ✅ Deleted |
| `provider/types.rs` | ✅ Deleted (minimal compat types in mod.rs) |
| `runtime/agent_loop.rs` | ✅ Deleted (replaced by rig_agent_loop.rs) |
| `tests/claude_chat_test.rs` | ✅ Deleted |
| `tests/agent_edge_cases_test.rs` | ✅ Deleted |

### Success Criteria
- [x] `cargo build` succeeds
- [x] All 679 tests pass
- [x] Agent workflows work with RigAgentLoop
- [x] MCP integration works via NikaMcpTool
- [x] CLAUDE.md updated with v0.4 changes

---

## MVP 8: RLM Enhancements ⏳ (CURRENT)

**Plan:** `docs/research/rlm-knowledge-graph-patterns-2025.md` (Section 11)
**Status:** IN PROGRESS (Phases 1, 4, 5 ✅ complete — 60%)
**Prerequisites:** MVP 7 ✅

### Overview
Enhance NovaNet + Nika with full RLM-on-KG (Recursive Language Model on Knowledge Graph) capabilities.

Research finding: NovaNet + Nika is ALREADY RLM-on-KG but missing key features from rig-rlm pattern.

### Phases
| Phase | Feature | Target | Effort | Files | Status |
|-------|---------|--------|--------|-------|--------|
| 1 | **Reasoning capture** | v0.4.1 | Low | `rig_agent_loop.rs`, `tests/thinking_capture_test.rs` | ✅ Done |
| 2 | **Nested agents** (`spawn_agent` tool) | v0.5 | Medium | `runtime/spawn.rs`, `executor.rs` | ⏳ Stub |
| 3 | **Schema introspection** (`novanet_introspect`) | v0.5 | Medium | NovaNet MCP server | ⏳ Pending |
| 4 | **Dynamic decomposition** (`decompose:` modifier) | v0.5 | High | `ast/decompose.rs`, `runtime/decomposer.rs` | ✅ Done |
| 5 | **Lazy bindings** (`lazy: true`) | v0.5 | Medium | `binding/entry.rs`, `binding/resolve.rs` | ✅ Done |

### Phase 1: Reasoning Capture (v0.4.1) ✅ COMPLETE

Token tracking and thinking capture now work correctly:

| Feature | Status |
|---------|--------|
| Token extraction from streaming | ✅ Done (via `GetTokenUsage` trait) |
| `AgentTurnMetadata.input_tokens` | ✅ Populated correctly |
| `AgentTurnMetadata.output_tokens` | ✅ Populated correctly |
| `AgentTurnMetadata.thinking` | ✅ Already implemented |
| Integration tests | ✅ `tests/thinking_capture_test.rs` |

**Files changed:**
- `rig_agent_loop.rs` - Extract tokens from `StreamedAssistantContent::Final`
- `tests/thinking_capture_test.rs` - Integration tests for token capture

### Phase 2: Nested Agent Spawning (v0.5)
Enable true recursion with `spawn_agent` internal tool:

```rust
pub struct SpawnAgentParams {
    pub task_id: String,
    pub prompt: String,
    pub context: Option<serde_json::Value>,
    pub max_turns: Option<u32>,
    pub depth_limit: u32,  // Prevent infinite recursion
}
```

### Phase 3: Schema Introspection (v0.5 - NovaNet side)
New MCP tool for agents to query graph SCHEMA:

```json
{
  "tool": "novanet_introspect",
  "params": {
    "node_class": "Entity",
    "include": ["arcs", "properties", "constraints"]
  }
}
```

### Phase 4: Dynamic Decomposition (v0.5) ✅ COMPLETE
New `decompose:` modifier for runtime DAG expansion:

```yaml
tasks:
  - id: generate_all
    decompose:
      strategy: semantic  # Use graph arcs for subtask discovery
      traverse: "HAS_CHILD"
      source: $category
    infer: "Generate for {{item}}"
```

### Phase 5: Lazy Bindings (v0.5) ✅ COMPLETE
Defer binding resolution until first access:

```yaml
use:
  eager: task1.result           # Resolved immediately
  lazy_val:
    path: future.result
    lazy: true                  # Resolved on access
    default: "fallback"         # Optional default
```

**Files changed:**
- `binding/entry.rs` - UseEntry with `lazy: bool` field
- `binding/resolve.rs` - LazyBinding enum (Resolved | Pending)
- `tests/lazy_binding_test.rs` - 15 integration tests

### Success Criteria
- [x] Phase 1: `thinking` captured in NDJSON traces ✅
- [ ] Phase 2: Nested agents work with depth limit
- [ ] Phase 3: `novanet_introspect` returns schema info
- [x] Phase 4: `decompose:` creates runtime subtasks ✅
- [x] Phase 5: Lazy bindings defer context loading ✅

---

## Plans Index

### MVP Plans (Execute in Order)
| MVP | Plan File | Status | Milestone |
|-----|-----------|--------|-----------|
| 0 | `nika-dev/docs/plans/2026-02-18-mvp0-dx-setup-core.md` | ✅ Done | v0.2 |
| 1 | `nika-dev/docs/plans/2026-02-18-mvp1-invoke-verb.md` | ✅ Done | v0.2 |
| 2 | `nika-dev/docs/plans/2026-02-18-mvp2-agent-observability.md` | ✅ Done | v0.2 |
| 3 | `nika-dev/docs/plans/2026-02-18-mvp3-tui-trace.md` | ✅ Done | v0.2 |
| 4 | `nika-dev/docs/plans/2026-02-18-mvp4-real-integration.md` | ✅ Done | v0.2.1 |
| 5 | `nika-dev/docs/plans/2026-02-18-mvp5-production-hardening.md` | ✅ Done | v0.2.2 |
| 6 | `nika-dev/docs/plans/2026-02-18-mvp6-v03-features.md` | ✅ Done | v0.3 |
| 7 | `docs/plans/2026-02-19-rig-core-migration.md` | ✅ Done | v0.4 |
| 8 | `docs/research/rlm-knowledge-graph-patterns-2025.md` (Sec 11) | ⏳ Not Started | v0.5 |

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
│  RUST DEPENDENCIES (Cargo.toml) - Nika v0.3                              │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  CORE:                                                                   │
│  ├── tokio = "1.49" (async runtime)                                     │
│  ├── serde, serde_yaml, serde_json                                      │
│  ├── clap, reqwest, regex, colored                                      │
│  ├── thiserror, anyhow, tracing                                         │
│  └── parking_lot, dashmap, rustc-hash                                   │
│                                                                          │
│  MCP + LLM (v0.3):                                                       │
│  ├── rmcp = "0.16" (MCP SDK with client transport)                      │
│  ├── rig-core = "0.31" (LLM framework with rmcp feature)                │
│  └── jsonschema = "0.26" (tool schema validation)                       │
│                                                                          │
│  TUI (feature-gated):                                                    │
│  ├── ratatui = "0.30" (terminal UI)                                     │
│  └── crossterm = "0.29" (terminal control)                              │
│                                                                          │
│  UTILITIES:                                                              │
│  ├── uuid = "1.0" (call IDs)                                            │
│  ├── xxhash-rust = "0.8" (prompt hashing)                               │
│  ├── chrono = "0.4" (timestamps)                                        │
│  └── rand = "0.8" (generation IDs)                                      │
│                                                                          │
│  DEV-DEPENDENCIES:                                                       │
│  ├── proptest = "1.4" (fuzzing)                                         │
│  ├── insta = "1.34" (snapshots)                                         │
│  ├── criterion = "0.5" (benchmarks)                                     │
│  └── pretty_assertions = "1.4"                                          │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## Architecture Target

```
┌─────────────────────────────────────────────────────────────────────────┐
│  NIKA v0.4 ARCHITECTURE                                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  tools/nika/src/                                                        │
│  ├── main.rs              # CLI (run, validate, tui, trace)            │
│  ├── lib.rs               # Public API                                 │
│  ├── error.rs             # NikaError (40+ variants with codes)        │
│  │                                                                      │
│  ├── ast/                 # YAML → Rust structs                        │
│  │   ├── workflow.rs      # Workflow + MCP config                      │
│  │   ├── action.rs        # TaskAction (5 variants + for_each)         │
│  │   ├── invoke.rs        # InvokeParams                               │
│  │   └── agent.rs         # AgentParams                                │
│  │                                                                      │
│  ├── mcp/                 # MCP Client (rmcp 0.16)                     │
│  │   ├── types.rs         # McpConfig, ToolResult                      │
│  │   ├── client.rs        # McpClient                                  │
│  │   └── rmcp_adapter.rs  # rmcp conversion layer (NEW v0.3)           │
│  │                                                                      │
│  ├── runtime/             # Execution engine                           │
│  │   ├── executor.rs      # Task dispatch (5 verbs + for_each)         │
│  │   ├── runner.rs        # Workflow orchestration                     │
│  │   └── agent_loop.rs    # Agentic execution                          │
│  │                                                                      │
│  ├── provider/            # LLM providers (v0.4 - rig-core only)       │
│  │   ├── mod.rs           # Provider trait + factory                   │
│  │   └── rig.rs           # RigProvider wrapper (761 lines)            │
│  │                                                                      │
│  ├── event/               # Observability                              │
│  │   ├── log.rs           # EventLog (16 variants)                     │
│  │   └── trace.rs         # NDJSON writer                              │
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

### MVP 6 Done When: ✅ ACHIEVED
- ✅ `for_each:` executes tasks in parallel
- ✅ rig-core 0.31 in Cargo.toml
- ✅ RigProvider wrapper implemented
- ✅ examples/README.md with quick-start guide
- ✅ 4 v0.3 showcase examples (parallel-locales, denomination-forms, entity-pipeline, agent-with-tools)

### MVP 7 Done When: ✅ ACHIEVED
- ✅ RigAgentLoop with rig::AgentBuilder
- ✅ Old providers deleted (claude.rs, openai.rs, types.rs)
- ✅ 679 tests passing

### MVP 8 Done When: (60% complete)
- [x] Phase 1: `thinking` field captured in AgentTurn events ✅
- [ ] Phase 2: Nested agents with `spawn_agent` tool
- [ ] Phase 3: `novanet_introspect` MCP tool in NovaNet
- [x] Phase 4: `decompose:` modifier for dynamic DAG ✅
- [x] Phase 5: `lazy: true` binding modifier ✅

---

## Notes

- **MVP 4-7 Complete:** NovaNet MCP v0.14.0 + Nika v0.4 (pure rig-core)
- **MVP 6 Complete:** 4 v0.3 showcase examples + README quick-start guide
- **MVP 7 Complete:** v0.4 achieved - pure rig-core, 611+ tests passing
- **MVP 8 Next:** RLM enhancements based on rig-rlm research
- **RLM Insight:** NovaNet + Nika is ALREADY RLM-on-KG with better safety/observability than rig-rlm
- **Resilience Module Removed:** Deleted in v0.4 cleanup (was never wired into runtime)
- **Testing:** TDD for all MVPs - write failing test first
- **Commits:** Atomic commits per task, conventional commit format
