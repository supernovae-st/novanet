# Rig-Core Migration Plan

**Date:** 2026-02-19
**Status:** Draft
**Affects:** Nika (nika-dev), NovaNet MCP (novanet-dev/tools/novanet-mcp)

---

## Executive Summary

Migration from custom LLM provider implementation to `rig-core` v0.31.0, which provides:
- Native `rmcp` v0.16 integration via `.rmcp_tools()` method
- 20+ built-in LLM providers with unified interface
- Built-in retry, streaming, and agent workflows
- Significant code reduction (~1500+ lines)

---

## Dependency Updates

### Nika (`nika-dev/tools/nika/Cargo.toml`)

```toml
# BEFORE
tokio = { version = "1.48", features = ["full"] }
ratatui = "0.29"
crossterm = "0.28"
rmcp = "0.16"
# (custom provider implementation)

# AFTER
tokio = { version = "1.49", features = ["full"] }
ratatui = "0.30"
crossterm = "0.29"
rmcp = "0.16"
rig-core = "0.31"  # NEW
```

### NovaNet Workspace (`novanet-dev/tools/Cargo.toml`)

```toml
# BEFORE
tokio = "1.43"

# AFTER
tokio = "1.49"
```

### NovaNet MCP (`novanet-dev/tools/novanet-mcp/Cargo.toml`)

```toml
# BEFORE
rmcp = "0.15"
tokio = "1.43"

# AFTER
rmcp = "0.16"
tokio = "1.49"
```

---

## Code Impact Analysis

### Files to REMOVE (replaced by rig-core)

| File | Lines | Replacement |
|------|-------|-------------|
| `provider/claude.rs` | ~325 | `rig::providers::anthropic::Client` |
| `provider/openai.rs` | ~280 | `rig::providers::openai::Client` |
| `provider/types.rs` | ~765 | `rig::completion::*` types |
| `provider/mod.rs` | ~50 | Thin wrapper around rig providers |

**Total reduction:** ~1420 lines

### Files to SIMPLIFY

| File | Before | After | Notes |
|------|--------|-------|-------|
| `runtime/agent_loop.rs` | ~717 lines | ~200 lines | Use rig's AgentBuilder with `.rmcp_tools()` |
| `mcp/client.rs` | ~400 lines | Keep | MCP client stays, feeds tools to rig |

---

## Migration Architecture

### Current Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  CURRENT: Custom Provider + AgentLoop                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  provider/mod.rs          → Provider trait (infer, chat)                    │
│  provider/claude.rs       → ClaudeProvider (raw reqwest)                    │
│  provider/openai.rs       → OpenAIProvider (raw reqwest)                    │
│  provider/types.rs        → Message, ToolCall, Usage, etc.                  │
│  runtime/agent_loop.rs    → AgentLoop (multi-turn, tool calling)            │
│  mcp/client.rs            → McpClient (connects to MCP servers)             │
│                                                                             │
│  Flow:                                                                      │
│  Workflow → AgentLoop → Provider.chat() → HTTP → Claude/OpenAI              │
│                ↓                                                            │
│           McpClient → MCP Server (NovaNet)                                  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Target Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  TARGET: Rig-Core + Native RMCP                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  provider/mod.rs          → RigProvider wrapper (thin)                      │
│  runtime/agent_loop.rs    → Uses rig::Agent with .rmcp_tools()              │
│  mcp/client.rs            → McpClient (unchanged, feeds rmcp::ClientTools)  │
│                                                                             │
│  Flow:                                                                      │
│  Workflow → rig::Agent                                                      │
│                ↓                                                            │
│           .rmcp_tools(mcp_tools)  ← Native MCP integration                  │
│                ↓                                                            │
│           rig::providers::anthropic::Client → Claude API                    │
│           rig::providers::openai::Client    → OpenAI API                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Implementation Steps

### Phase 1: Dependency Updates (No Code Changes)

1. **Update Nika Cargo.toml**
   - Add `rig-core = "0.31"`
   - Bump `tokio = "1.49"`
   - Bump `ratatui = "0.30"` (requires Rust 1.86+)
   - Bump `crossterm = "0.29"`

2. **Update NovaNet workspace Cargo.toml**
   - Bump `tokio = "1.49"`

3. **Update NovaNet MCP Cargo.toml**
   - Bump `rmcp = "0.16"`
   - Bump `tokio = "1.49"`

4. **Verify builds**
   ```bash
   cd nika-dev/tools/nika && cargo build
   cd novanet-dev/tools && cargo build
   cd novanet-dev/tools/novanet-mcp && cargo build
   ```

### Phase 2: Create Rig Provider Wrapper

Create new `provider/rig.rs`:

```rust
use rig::providers::{anthropic, openai};
use rig::completion::CompletionModel;

pub enum RigProvider {
    Claude(anthropic::Client),
    OpenAI(openai::Client),
}

impl RigProvider {
    pub fn claude() -> Self {
        let client = anthropic::Client::from_env();
        RigProvider::Claude(client)
    }

    pub fn openai() -> Self {
        let client = openai::Client::from_env();
        RigProvider::OpenAI(client)
    }

    pub fn model(&self, model_id: &str) -> impl CompletionModel {
        match self {
            RigProvider::Claude(c) => c.completion_model(model_id),
            RigProvider::OpenAI(c) => c.completion_model(model_id),
        }
    }
}
```

### Phase 3: Rewrite AgentLoop with Rig

Replace `runtime/agent_loop.rs` with rig-native implementation:

```rust
use rig::agent::AgentBuilder;
use rmcp::service::ClientTools;

pub struct RigAgentLoop {
    task_id: TaskId,
    params: AgentParams,
    event_log: EventLog,
    mcp_tools: ClientTools,
}

impl RigAgentLoop {
    pub async fn run(&self) -> Result<AgentResult> {
        // Get rig provider based on params
        let provider = RigProvider::claude();
        let model = provider.model(&self.params.model);

        // Build agent with MCP tools via native rmcp integration
        let agent = AgentBuilder::new(model)
            .preamble(&self.params.system_prompt)
            .rmcp_tools(self.mcp_tools.clone())  // Native MCP!
            .max_turns(self.params.max_turns.unwrap_or(10))
            .build();

        // Run agent
        let result = agent.prompt(&self.params.prompt).await?;

        Ok(AgentResult {
            output: result.to_string(),
            turns: agent.turns(),
            tokens: agent.usage(),
        })
    }
}
```

### Phase 4: Remove Old Provider Code

1. Delete `provider/claude.rs`
2. Delete `provider/openai.rs`
3. Delete `provider/types.rs`
4. Update `provider/mod.rs` to re-export rig wrapper
5. Update all imports across codebase

### Phase 5: Update Tests

1. Update provider tests to use rig mocks
2. Update agent_loop tests
3. Run full test suite: `cargo nextest run`

### Phase 6: Update MCP Client Integration

Ensure `mcp/client.rs` returns `rmcp::service::ClientTools` compatible format:

```rust
impl McpClient {
    pub async fn get_tools(&self) -> Result<ClientTools> {
        // Connect to MCP server
        // Return ClientTools for use with .rmcp_tools()
    }
}
```

---

## Rig-Core Key APIs

### Provider Setup

```rust
// Claude
let client = rig::providers::anthropic::Client::from_env();
let model = client.completion_model("claude-sonnet-4-20250514");

// OpenAI
let client = rig::providers::openai::Client::from_env();
let model = client.completion_model("gpt-4o");
```

### Agent with MCP Tools

```rust
use rig::agent::AgentBuilder;
use rmcp::service::ClientTools;

// Get MCP tools from client
let mcp_tools: ClientTools = mcp_client.get_tools().await?;

// Build agent with native MCP support
let agent = AgentBuilder::new(model)
    .preamble("You are a helpful assistant")
    .rmcp_tools(mcp_tools)  // Native rmcp integration!
    .max_turns(10)
    .build();

// Run
let response = agent.prompt("Generate content for qr-code entity").await?;
```

### Streaming

```rust
let stream = agent.prompt_stream("Generate content").await?;
while let Some(chunk) = stream.next().await {
    // Handle chunk
}
```

---

## Breaking Changes

| Change | Migration Path |
|--------|----------------|
| `Provider` trait removed | Use `RigProvider` wrapper |
| `Message` type changes | Use `rig::completion::Message` |
| `AgentLoop::run()` signature | Returns `rig::agent::AgentOutput` |
| `ChatResponse` removed | Use rig's response types |

---

## Testing Plan

1. **Unit tests**: New provider wrapper tests
2. **Integration tests**: Agent with mock MCP server
3. **E2E tests**: Full workflow with real NovaNet MCP

```bash
# Run all tests
cargo nextest run

# Run with real MCP
cargo test --features integration
```

---

## Rollback Plan

If issues arise:
1. Revert Cargo.toml changes
2. Restore deleted provider files from git
3. Re-run `cargo build`

All deleted code is preserved in git history.

---

## Timeline Estimate

| Phase | Effort |
|-------|--------|
| Phase 1: Deps | 30 min |
| Phase 2: Wrapper | 2 hours |
| Phase 3: AgentLoop | 4 hours |
| Phase 4: Cleanup | 1 hour |
| Phase 5: Tests | 2 hours |
| Phase 6: MCP | 2 hours |
| **Total** | **~12 hours** |

---

## Success Criteria

- [ ] All Cargo.toml updated with new versions
- [ ] `cargo build` succeeds for all projects
- [ ] `cargo nextest run` passes (1000+ tests)
- [ ] Agent workflows work with rig-core
- [ ] MCP integration works via `.rmcp_tools()`
- [ ] No performance regression (benchmark)

---

## References

- [rig-core crates.io](https://crates.io/crates/rig-core)
- [rig-core docs.rs](https://docs.rs/rig-core/0.31.0)
- [rmcp crates.io](https://crates.io/crates/rmcp)
- [rmcp MCP integration](https://docs.rs/rmcp/0.16.0/rmcp/service/struct.ClientTools.html)
