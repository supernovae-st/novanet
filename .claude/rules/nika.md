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

**v0.3** | 5 verbs + for_each | TUI | 16 EventLog variants | Real-time tracing

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
├── ast/          # YAML → Rust structs
├── mcp/          # MCP client
├── runtime/      # Execution engine
├── provider/     # LLM providers (rig-core)
├── event/        # Observability (16 variants)
├── binding/      # Data flow ({{use.alias}})
└── tui/          # Terminal UI (4 panels)
```

## for_each Parallelism (v0.3)

Parallel iteration over arrays with `tokio::spawn` JoinSet:

```yaml
tasks:
  - id: generate_pages
    for_each:
      items: $locales
      as: locale
      concurrency: 5
    invoke:
      mcp: novanet
      tool: novanet_generate
      params:
        entity: "qr-code"
        locale: "{{locale}}"
```

| Property | Default | Description |
|----------|---------|-------------|
| `items` | required | Array to iterate over |
| `as` | required | Loop variable name |
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

- Roadmap: `../../ROADMAP.md` (MVP 4-6 status)
- Error codes: `src/error.rs` (NikaError 40+ variants)
- Examples: `tools/nika/examples/`
