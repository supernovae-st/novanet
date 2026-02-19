# MVP 8: Phase 2 & Phase 3 Execution Plan

**Date:** 2026-02-19
**Author:** Claude Opus 4.5 + Thibaut
**Status:** Ready for Execution

---

## Executive Summary

Based on codebase audit, Phase 3 is 80% complete (just needs GREEN phase), while Phase 2 is 60% scaffolded.

**Recommended execution order:**
1. **Phase 3 first** (2-4 hours) - Quick win, unblock novanet_introspect MCP tool
2. **Phase 2 second** (8-12 hours) - More complex, builds on completed infrastructure

---

## Phase 3: novanet_introspect (GREEN Phase)

### Current State

| Component | Status | Location |
|-----------|--------|----------|
| `introspect.rs` | ✅ 460 lines | `novanet-dev/tools/novanet-mcp/src/tools/introspect.rs` |
| Tool registration | ✅ Done | `handler.rs` - 8th tool registered |
| Unit tests | ✅ 6 passing | `introspect.rs` inline tests |
| Integration tests | ❌ 6 `#[ignore]` | Need Neo4j state setup |
| Documentation | ❌ Missing | CLAUDE.md lists 7 tools |

### Tasks

| # | Task | Effort | Files |
|---|------|--------|-------|
| 1 | Review introspect.rs implementation | 15 min | `tools/introspect.rs` |
| 2 | Create test fixtures for Neo4j state | 30 min | `tests/fixtures/` |
| 3 | Wire up integration tests | 1 hour | `tests/introspect_test.rs` |
| 4 | Remove `#[ignore]` annotations | 15 min | inline |
| 5 | Run full test suite | 30 min | verify 6 tests pass |
| 6 | Update MCP documentation | 30 min | `CLAUDE.md`, `README.md` |

### Test Plan

```rust
// 6 integration tests to wire up:
#[tokio::test]
async fn test_introspect_all_classes()           // All 61 node classes
#[tokio::test]
async fn test_introspect_filtered_by_realm()     // Filter by shared/org
#[tokio::test]
async fn test_introspect_specific_class()        // Single class + arcs
#[tokio::test]
async fn test_introspect_arc_families()          // Filter by family
#[tokio::test]
async fn test_introspect_class_not_found()       // Error handling
#[tokio::test]
async fn test_introspect_token_estimate()        // Token counting
```

### Verification

```bash
cd novanet-dev/tools/novanet-mcp
cargo test introspect --all-features
cargo test --all-features  # 8 tools, all tests green
```

---

## Phase 2: spawn_agent (Full Implementation)

### Current State

| Component | Status | Location |
|-----------|--------|----------|
| `SpawnAgentTool` struct | ✅ Exists | `nika-dev/tools/nika/src/runtime/spawn.rs` |
| `AgentSpawned` event | ✅ Exists | `nika-dev/tools/nika/src/event/log.rs` |
| `depth_limit` field | ✅ In AgentParams | `nika-dev/tools/nika/src/ast/agent.rs` |
| `rig::ToolDyn` impl | ❌ Stub only | Missing trait implementation |
| Child agent spawning | ❌ Not implemented | Core functionality missing |
| MCP client injection | ❌ Missing | Need to pass clients to child |
| RigAgentLoop integration | ❌ Missing | Not wired into agent loop |

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SPAWN_AGENT ARCHITECTURE                                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Parent Agent (depth=0)                                                     │
│  ├── Tools: [MCP tools, spawn_agent]                                        │
│  │                                                                          │
│  └── spawn_agent({ task_id: "sub-1", prompt: "...", depth_limit: 2 })       │
│       │                                                                     │
│       ▼                                                                     │
│  Child Agent (depth=1)                                                      │
│  ├── Inherits: MCP clients, provider                                        │
│  ├── Tools: [MCP tools, spawn_agent] (if depth < limit)                     │
│  │                                                                          │
│  └── spawn_agent({ task_id: "sub-2", prompt: "...", depth_limit: 1 })       │
│       │                                                                     │
│       ▼                                                                     │
│  Grandchild Agent (depth=2)                                                 │
│  ├── Tools: [MCP tools] (NO spawn_agent - at depth limit)                   │
│  └── Returns result up the chain                                            │
│                                                                             │
│  SAFETY: depth_limit prevents infinite recursion                            │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Tasks (TDD Order)

| # | Task | Effort | Files |
|---|------|--------|-------|
| **RED Phase** |
| 1 | Write failing test: `SpawnAgentTool` implements `ToolDyn` | 30 min | `tests/spawn_agent_test.rs` |
| 2 | Write failing test: depth limit prevents spawning | 30 min | `tests/spawn_agent_test.rs` |
| 3 | Write failing test: child agent executes | 1 hour | `tests/spawn_agent_test.rs` |
| 4 | Write failing test: `AgentSpawned` event emitted | 30 min | `tests/spawn_agent_test.rs` |
| **GREEN Phase** |
| 5 | Implement `ToolDyn` for `SpawnAgentTool` | 2 hours | `runtime/spawn.rs` |
| 6 | Add MCP client injection | 1 hour | `runtime/spawn.rs` |
| 7 | Create child `RigAgentLoop` | 2 hours | `runtime/spawn.rs` |
| 8 | Wire into parent agent tool list | 1 hour | `runtime/rig_agent_loop.rs` |
| **REFACTOR Phase** |
| 9 | Extract common agent creation logic | 1 hour | `runtime/mod.rs` |
| 10 | Add tracing instrumentation | 30 min | all files |
| 11 | Update documentation | 30 min | `CLAUDE.md`, examples |

### Code Structure

```rust
// runtime/spawn.rs - Final implementation

use rig::tool::{ToolDyn, ToolDefinition, ToolError};
use crate::mcp::McpClient;
use crate::event::EventLog;
use crate::runtime::RigAgentLoop;

#[derive(Debug, Clone, Deserialize)]
pub struct SpawnAgentParams {
    pub task_id: String,
    pub prompt: String,
    pub context: Option<serde_json::Value>,
    pub max_turns: Option<u32>,
}

pub struct SpawnAgentTool {
    current_depth: u32,
    max_depth: u32,
    parent_task_id: Arc<str>,
    event_sender: Option<mpsc::Sender<EventLog>>,
    mcp_clients: Arc<DashMap<String, Arc<McpClient>>>,
    provider: Arc<RigProvider>,
}

impl ToolDyn for SpawnAgentTool {
    fn name(&self) -> &str { "spawn_agent" }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "spawn_agent".into(),
            description: "Spawn a sub-agent to handle a delegated task".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "task_id": { "type": "string", "description": "Unique ID for child task" },
                    "prompt": { "type": "string", "description": "Task prompt for child agent" },
                    "context": { "type": "object", "description": "Optional context data" },
                    "max_turns": { "type": "integer", "description": "Max turns for child" }
                },
                "required": ["task_id", "prompt"]
            }),
        }
    }

    async fn call(&self, args: String) -> Result<String, ToolError> {
        let params: SpawnAgentParams = serde_json::from_str(&args)?;

        // Safety check
        if self.current_depth >= self.max_depth {
            return Err(ToolError::new(format!(
                "Cannot spawn agent: depth limit {} reached", self.max_depth
            )));
        }

        // Emit event
        if let Some(tx) = &self.event_sender {
            let _ = tx.send(EventLog::AgentSpawned {
                parent_task_id: self.parent_task_id.clone(),
                child_task_id: params.task_id.clone().into(),
                depth: self.current_depth + 1,
            });
        }

        // Create child agent
        let child = RigAgentLoop::new_child(
            params.task_id,
            params.prompt,
            self.current_depth + 1,
            self.max_depth,
            self.mcp_clients.clone(),
            self.provider.clone(),
            self.event_sender.clone(),
        );

        // Execute and return result
        let result = child.run().await?;
        Ok(serde_json::to_string(&result)?)
    }
}
```

### Verification

```bash
cd nika-dev/tools/nika
cargo test spawn_agent --all-features
cargo test agent -- --nocapture  # Integration tests
cargo run -- run examples/v05-nested-agent.nika.yaml
```

---

## Timeline

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  EXECUTION TIMELINE                                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Phase 3 (novanet_introspect GREEN)                    ~3 hours            │
│  ├── Review implementation                             15 min              │
│  ├── Create test fixtures                              30 min              │
│  ├── Wire integration tests                            1 hour              │
│  ├── Run and verify                                    30 min              │
│  └── Update documentation                              30 min              │
│                                                                             │
│  Phase 2 (spawn_agent FULL)                            ~10 hours           │
│  ├── RED: Write 4 failing tests                        2.5 hours           │
│  ├── GREEN: Implement + wire up                        6 hours             │
│  └── REFACTOR: Clean up + docs                         2 hours             │
│                                                                             │
│  TOTAL                                                 ~13 hours           │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Success Criteria

### Phase 3 Complete When:
- [ ] `cargo test introspect` passes (6 tests)
- [ ] `cargo test --all-features` passes (all 8 MCP tools)
- [ ] Documentation updated to show 8 tools

### Phase 2 Complete When:
- [ ] `cargo test spawn_agent` passes (4+ tests)
- [ ] Depth limit prevents infinite recursion
- [ ] `AgentSpawned` events appear in traces
- [ ] Example workflow `v05-nested-agent.nika.yaml` runs successfully
- [ ] Documentation updated with spawn_agent usage

---

## Risk Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| Neo4j state not available for introspect tests | Medium | Use mock state or separate test DB |
| Infinite recursion in spawn_agent | Critical | Enforce depth_limit at ToolDyn::call entry |
| MCP client ownership conflicts | Medium | Use Arc<DashMap> for shared clients |
| Event channel overflow | Low | Use bounded channel with backpressure |

---

## Files Changed Summary

### Phase 3
- `novanet-dev/tools/novanet-mcp/src/tools/introspect.rs` - Remove #[ignore]
- `novanet-dev/tools/novanet-mcp/tests/introspect_test.rs` - Add fixtures
- `novanet-dev/CLAUDE.md` - Update to 8 tools (already done)

### Phase 2
- `nika-dev/tools/nika/src/runtime/spawn.rs` - Full implementation
- `nika-dev/tools/nika/src/runtime/rig_agent_loop.rs` - Wire spawn_agent
- `nika-dev/tools/nika/src/runtime/mod.rs` - Export spawn module
- `nika-dev/tools/nika/tests/spawn_agent_test.rs` - Integration tests
- `nika-dev/tools/nika/examples/v05-nested-agent.nika.yaml` - Example workflow
