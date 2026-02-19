---
paths:
  - "nika-dev/**"
---

# Nika Project Rules

These rules only load when working in `nika-dev/`.

## Core Philosophy

Nika is the **Body** — a semantic YAML workflow engine that executes multi-step AI workflows.

```
Workflow (YAML) → DAG → Execution → Results

Nika is the MCP CLIENT that consumes NovaNet (MCP SERVER) knowledge.
```

## Current Version

**v0.5.0** | rig-core v0.31 | RigAgentLoop | RigProvider | 683+ tests | MVP 8 100% complete

### v0.5.0 Features (MVP 8 RLM Enhancements)

| Feature | Status | Description |
|---------|--------|-------------|
| Reasoning capture | ✅ | `thinking` field in AgentTurn events |
| spawn_agent | ✅ | Nested agents with depth protection (rig::ToolDyn) |
| novanet_introspect | ✅ | 8th MCP tool for schema queries |
| decompose: | ✅ | Runtime DAG expansion via MCP traversal |
| lazy: bindings | ✅ | Deferred binding resolution until access |

### Known Limitations

- **spawn_agent production path**: Uses `run_mock()` for testing; production requires API key config
- **Token tracking (non-thinking)**: `run_claude()` returns `total_tokens: 0`; use `extended_thinking: true` for accurate counts

### v0.4 Changes (rig-core Migration)

| Component | Old (v0.3) | New (v0.4) |
|-----------|------------|------------|
| Provider | ClaudeProvider, OpenAIProvider | `RigProvider::claude()`, `RigProvider::openai()` |
| Agent Loop | AgentLoop (custom) | `RigAgentLoop` with rig::AgentBuilder |
| MCP Tools | Manual wrapper | `NikaMcpTool` implements `rig::ToolDyn` |
| Test Count | ~500 | 621+ passing |

**Removed in v0.4:**
- `provider/claude.rs` (deleted)
- `provider/openai.rs` (deleted)
- `provider/types.rs` (deleted)
- `runtime/agent_loop.rs` (replaced by `rig_agent_loop.rs`)
- `resilience/` module (entire module deleted)

## Verbs (5)

| Verb | Purpose | Example |
|------|---------|---------|
| `infer:` | LLM text generation | `infer: "Summarize the entity"` |
| `exec:` | Shell command | `exec: "npm run build"` |
| `fetch:` | HTTP request | `fetch: { url: "...", method: "GET" }` |
| `invoke:` | MCP tool call | `invoke: novanet_generate` |
| `agent:` | Multi-turn agentic loop | `agent: { goal: "...", tools: [...] }` |

## MCP Integration

Nika connects to NovaNet via MCP — **NEVER direct Neo4j access**.

```yaml
workflow: generate-page
mcp:
  servers:
    novanet:
      command: "cargo run --manifest-path ../novanet-dev/tools/novanet-mcp/Cargo.toml"

tasks:
  - invoke: novanet_generate
    params:
      entity: "qr-code"
      locale: "fr-FR"
      forms: ["text", "title"]
    use.ctx: entity_context

  - infer: "Generate landing page"
    context: $entity_context
```

## Key Commands

```bash
# Run workflow
cargo run -- run examples/workflow.yaml

# Validate workflow
cargo run -- validate examples/workflow.yaml

# TUI
cargo run -- tui examples/workflow.yaml

# Traces
cargo run -- trace list
cargo run -- trace show <id>
```

## Architecture

```
tools/nika/src/
├── ast/          # YAML → Rust structs (Workflow, Task, TaskAction)
├── mcp/          # MCP client (rmcp v0.16)
│   └── client.rs # McpClient with DashMap + OnceCell caching
├── runtime/      # Execution engine
│   ├── runner.rs       # Workflow orchestration
│   ├── executor.rs     # Task dispatch (5 verbs + for_each)
│   └── rig_agent_loop.rs  # RigAgentLoop with rig::AgentBuilder
├── provider/     # LLM providers (rig-core v0.31 ONLY)
│   └── rig.rs    # RigProvider wrapper (761 lines) + NikaMcpTool
├── event/        # Observability (16 variants)
│   ├── log.rs    # EventLog with EventKind enum
│   └── trace.rs  # NDJSON writer
├── binding/      # Data flow ({{use.alias}})
└── tui/          # Terminal UI (4 panels, feature-gated)
```

## for_each Parallelism (v0.3)

Parallel iteration over arrays with `tokio::spawn` JoinSet:

```yaml
tasks:
  - id: generate_pages
    for_each: ["fr-FR", "en-US", "de-DE"]  # Array or binding expression
    as: locale                              # Loop variable name
    concurrency: 5                          # Max parallel executions
    fail_fast: true                         # Stop on first error
    invoke:
      mcp: novanet
      tool: novanet_generate
      params:
        entity: "qr-code"
        locale: "{{use.locale}}"
```

Binding expressions are supported (resolved at runtime):
```yaml
    for_each: "{{use.items}}"   # Reference to array in context
    for_each: "$items"          # Alternative binding syntax
```

| Property | Default | Description |
|----------|---------|-------------|
| `for_each` | required | Array or binding expression to iterate over |
| `as` | "item" | Loop variable name |
| `concurrency` | 1 | Max parallel executions |
| `fail_fast` | true | Stop all on first error |

## EventLog Variants (16)

`WorkflowStarted`, `TaskStarted`, `TaskCompleted`, `TaskFailed`,
`InferStarted`, `InferCompleted`, `McpToolCalled`, `McpToolResponded`,
`AgentTurnStarted`, `AgentTurnCompleted`, `ContextAssembled`, ...

## Zero Cypher Rule

Nika workflows NEVER use raw Cypher:

```yaml
# WRONG
- exec: "MATCH (e:Entity) RETURN e"

# RIGHT
- invoke: novanet_traverse
  params:
    start: "entity:qr-code"
    arc: "HAS_NATIVE"
```

## References

- Roadmap: `../../ROADMAP.md` (MVP 8 complete, v0.5 ready for release)
- Error codes: `src/error.rs` (NikaError 40+ variants)
- Examples: `tools/nika/examples/`
- CLAUDE.md: `tools/nika/CLAUDE.md` (v0.5 spawn_agent + decompose guide)
- rig-core: https://github.com/0xPlaygrounds/rig (20+ LLM providers)
