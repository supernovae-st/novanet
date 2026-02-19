# NovaNet + Nika Integration Guide

**Version:** 1.0.0
**Last Updated:** 2026-02-19
**NovaNet:** v0.14.0 | **Nika:** v0.4.1

This document provides a comprehensive technical guide for integrating NovaNet (the Brain) with Nika (the Body) in the supernovae-agi architecture.

---

## Table of Contents

1. [Brain and Body Architecture](#1-brain-and-body-architecture)
2. [MCP Protocol Integration](#2-mcp-protocol-integration)
3. [The invoke: Verb Deep-Dive](#3-the-invoke-verb-deep-dive)
4. [The agent: Verb with MCP Tools](#4-the-agent-verb-with-mcp-tools)
5. [Zero Cypher Rule (ADR-003)](#5-zero-cypher-rule-adr-003)
6. [Denomination Forms Integration](#6-denomination-forms-integration)
7. [Workflow Patterns](#7-workflow-patterns)
8. [RLM-on-KG Pattern](#8-rlm-on-kg-pattern)
9. [Observability Integration](#9-observability-integration)
10. [Error Handling](#10-error-handling)
11. [Development Workflow](#11-development-workflow)
12. [Debugging Integration Issues](#12-debugging-integration-issues)

---

## 1. Brain and Body Architecture

NovaNet and Nika form a complementary architecture where NovaNet provides semantic knowledge and Nika executes AI workflows.

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                       SUPERNOVAE-AGI: BRAIN + BODY                                  │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│  ┌─────────────────────────────┐                  ┌─────────────────────────────┐  │
│  │         NOVANET             │    MCP Protocol  │           NIKA              │  │
│  │         (Brain)             │◄────────────────►│          (Body)             │  │
│  ├─────────────────────────────┤                  ├─────────────────────────────┤  │
│  │                             │   JSON-RPC 2.0   │                             │  │
│  │  ┌───────────────────────┐  │    over stdio    │  ┌───────────────────────┐  │  │
│  │  │    Knowledge Graph    │  │                  │  │    YAML Workflows     │  │  │
│  │  │  ├─ 61 NodeClasses    │  │   novanet_*      │  │  ├─ 5 Semantic Verbs  │  │  │
│  │  │  ├─ 182 ArcClasses    │  │   tools (7)      │  │  ├─ DAG Execution     │  │  │
│  │  │  └─ Neo4j Backend     │  │                  │  │  └─ for_each Parallel │  │  │
│  │  └───────────────────────┘  │                  │  └───────────────────────┘  │  │
│  │                             │                  │                             │  │
│  │  ┌───────────────────────┐  │                  │  ┌───────────────────────┐  │  │
│  │  │     MCP Server        │  │                  │  │     MCP Client        │  │  │
│  │  │  ├─ rmcp v0.16        │  │                  │  │  ├─ rmcp v0.16        │  │  │
│  │  │  ├─ 7 Tools           │  │                  │  │  ├─ RmcpClientAdapter │  │  │
│  │  │  └─ Resources/Prompts │  │                  │  │  └─ NikaMcpTool       │  │  │
│  │  └───────────────────────┘  │                  │  └───────────────────────┘  │  │
│  │                             │                  │                             │  │
│  │  ┌───────────────────────┐  │                  │  ┌───────────────────────┐  │  │
│  │  │    Rust CLI + TUI     │  │                  │  │    rig-core v0.31    │  │  │
│  │  │  ├─ novanet tui       │  │                  │  │  ├─ RigProvider       │  │  │
│  │  │  └─ novanet schema    │  │                  │  │  └─ RigAgentLoop      │  │  │
│  │  └───────────────────────┘  │                  │  └───────────────────────┘  │  │
│  │                             │                  │                             │  │
│  └─────────────────────────────┘                  └─────────────────────────────┘  │
│              │                                                  │                  │
│              ▼                                                  ▼                  │
│       novanet-dev/                                        nika-dev/                │
│       tools/novanet-mcp/                                  tools/nika/              │
│                                                                                     │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

### Key Responsibilities

| Component | Responsibility | Technology |
|-----------|---------------|------------|
| **NovaNet** | Semantic knowledge, entity definitions, locale context, graph traversal | Neo4j, Rust, TypeScript |
| **Nika** | Workflow execution, LLM orchestration, parallel processing, agentic loops | Rust, rig-core, tokio |
| **MCP** | Communication protocol, tool invocation, resource access | rmcp v0.16, JSON-RPC 2.0 |

### Why This Architecture?

1. **Separation of Concerns**: Knowledge (NovaNet) vs. Execution (Nika)
2. **Protocol-Based Integration**: MCP provides a standard interface
3. **Independent Scaling**: Each component can evolve independently
4. **Security**: Nika never has direct database access (Zero Cypher Rule)
5. **Observability**: MCP calls are fully traceable

---

## 2. MCP Protocol Integration

### Protocol Overview

The Model Context Protocol (MCP) is used for all communication between Nika and NovaNet.

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                              MCP PROTOCOL FLOW                                   │
├──────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│  Nika (Client)                                           NovaNet (Server)       │
│       │                                                        │                │
│       │  1. Spawn Server Process                               │                │
│       │ ─────────────────────────────────────────────────────► │                │
│       │                                                        │                │
│       │  2. MCP Initialize (protocol version, capabilities)   │                │
│       │ ◄─────────────────────────────────────────────────────►│                │
│       │                                                        │                │
│       │  3. tools/list                                         │                │
│       │ ─────────────────────────────────────────────────────► │                │
│       │  4. Tool definitions (7 tools with JSON schemas)       │                │
│       │ ◄───────────────────────────────────────────────────── │                │
│       │                                                        │                │
│       │  5. tools/call (novanet_generate)                      │                │
│       │ ─────────────────────────────────────────────────────► │                │
│       │                                      ┌─────────────┐   │                │
│       │                                      │   Neo4j     │   │                │
│       │                                      │   Query     │   │                │
│       │                                      └─────────────┘   │                │
│       │  6. Tool result (JSON content)                         │                │
│       │ ◄───────────────────────────────────────────────────── │                │
│       │                                                        │                │
│       ▼                                                        ▼                │
│                                                                                  │
└──────────────────────────────────────────────────────────────────────────────────┘
```

### rmcp v0.16 Integration

Both projects use the Anthropic official rmcp SDK v0.16.

**NovaNet (Server Side):**
```rust
// tools/novanet-mcp/src/server/handler.rs
use rmcp::service::{RoleServer, ServiceExt};
use rmcp::transport::TokioChildProcess;

// Server implements ServiceHandler for MCP protocol
impl ServiceHandler for NovaNetServer {
    async fn call_tool(&self, request: CallToolRequest) -> Result<CallToolResult> {
        match request.name.as_ref() {
            "novanet_generate" => self.handle_generate(request.params).await,
            "novanet_describe" => self.handle_describe(request.params).await,
            // ... 7 tools total
        }
    }
}
```

**Nika (Client Side):**
```rust
// tools/nika/src/mcp/rmcp_adapter.rs
use rmcp::service::{RoleClient, RunningService};
use rmcp::transport::TokioChildProcess;

pub struct RmcpClientAdapter {
    service: AsyncMutex<Option<RunningService<RoleClient, ()>>>,
}

impl RmcpClientAdapter {
    pub async fn connect(&self) -> Result<()> {
        let transport = TokioChildProcess::new(cmd)?;
        let service = ().serve(transport).await?;
        *self.service.lock().await = Some(service);
        Ok(())
    }

    pub async fn call_tool(&self, name: &str, params: Value) -> Result<ToolCallResult> {
        let request = CallToolRequestParams {
            name: name.to_string().into(),
            arguments: params.as_object().cloned(),
            ..Default::default()
        };
        let result = self.service.call_tool(request).await?;
        // Convert rmcp result to Nika's ToolCallResult
        Ok(ToolCallResult { content, is_error })
    }
}
```

### NovaNet MCP Server Tools (7 Tools)

| Tool | Purpose | Key Parameters |
|------|---------|----------------|
| `novanet_describe` | Schema/entity discovery | `target`, `entity_key`, `filters` |
| `novanet_search` | Fulltext + property search | `query`, `mode`, `kinds`, `limit` |
| `novanet_traverse` | Graph traversal | `start_key`, `max_depth`, `direction`, `arc_families` |
| `novanet_assemble` | Token-aware context assembly | `focus_key`, `locale`, `token_budget`, `strategy` |
| `novanet_atoms` | Knowledge atoms retrieval | `locale`, `atom_type`, `domain` |
| `novanet_generate` | Full RLM context for generation | `focus_key`, `locale`, `mode`, `forms` |
| `novanet_query` | Advanced Cypher queries | `cypher`, `params` (restricted access) |

### Server Spawning and Lifecycle

Nika spawns the NovaNet MCP server as a child process:

```yaml
# Workflow YAML configuration
mcp:
  novanet:
    command: cargo
    args:
      - run
      - --manifest-path
      - ../novanet-dev/tools/novanet-mcp/Cargo.toml
    env:
      NOVANET_MCP_NEO4J_URI: bolt://localhost:7687
      NOVANET_MCP_NEO4J_USER: neo4j
      NOVANET_MCP_NEO4J_PASSWORD: password
      RUST_LOG: info
```

**Lifecycle:**
1. Workflow execution starts
2. Nika spawns MCP server process
3. MCP handshake (initialize/initialized)
4. Tool calls during workflow execution
5. Server terminates when workflow completes (or on error)

---

## 3. The invoke: Verb Deep-Dive

The `invoke:` verb is Nika's primary mechanism for calling MCP tools.

### Syntax

```yaml
tasks:
  - id: get_entity
    invoke:
      mcp: novanet              # MCP server name (from mcp: config)
      tool: novanet_generate    # Tool to call
      params:                   # Tool parameters
        entity: "qr-code"
        locale: "fr-FR"
        forms: ["text", "title", "url"]
    output:
      format: json              # Output format (optional)
```

### AST Representation

```rust
// tools/nika/src/ast/invoke.rs
#[derive(Debug, Clone, Deserialize)]
pub struct InvokeParams {
    /// MCP server name (must match a key in workflow's `mcp` config)
    pub mcp: String,

    /// Tool name to call (mutually exclusive with `resource`)
    pub tool: Option<String>,

    /// Parameters to pass to the tool
    pub params: Option<serde_json::Value>,

    /// Resource URI to read (mutually exclusive with `tool`)
    pub resource: Option<String>,
}
```

### Execution Flow

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                         invoke: VERB EXECUTION FLOW                            │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                │
│  Workflow YAML                                                                 │
│       │                                                                        │
│       ▼                                                                        │
│  ┌─────────────────┐                                                          │
│  │   Parse YAML    │   InvokeParams { mcp: "novanet", tool: "...", ... }     │
│  └────────┬────────┘                                                          │
│           │                                                                    │
│           ▼                                                                    │
│  ┌─────────────────┐                                                          │
│  │ Resolve use:    │   {{use.page.key}} → "homepage"                         │
│  │   bindings      │   Template engine resolves variables                     │
│  └────────┬────────┘                                                          │
│           │                                                                    │
│           ▼                                                                    │
│  ┌─────────────────┐                                                          │
│  │  Get McpClient  │   DashMap lookup by server name                          │
│  │  from registry  │   McpClient::connect() if not connected                  │
│  └────────┬────────┘                                                          │
│           │                                                                    │
│           ▼                                                                    │
│  ┌─────────────────┐                                                          │
│  │  Emit McpInvoke │   EventLog: call_id, tool, params                        │
│  │     event       │                                                          │
│  └────────┬────────┘                                                          │
│           │                                                                    │
│           ▼                                                                    │
│  ┌─────────────────┐                                                          │
│  │   call_tool()   │   rmcp SDK: JSON-RPC 2.0 over stdio                     │
│  │   via rmcp      │   Waits for response                                     │
│  └────────┬────────┘                                                          │
│           │                                                                    │
│           ▼                                                                    │
│  ┌─────────────────┐                                                          │
│  │ Emit McpResponse│   EventLog: output_len, duration_ms, cached              │
│  │     event       │                                                          │
│  └────────┬────────┘                                                          │
│           │                                                                    │
│           ▼                                                                    │
│  ┌─────────────────┐                                                          │
│  │  Store output   │   DataStore: task_id → result (for use: bindings)       │
│  │  in DataStore   │                                                          │
│  └─────────────────┘                                                          │
│                                                                                │
└────────────────────────────────────────────────────────────────────────────────┘
```

### Parameter Passing with use: Bindings

```yaml
tasks:
  - id: discover
    invoke:
      mcp: novanet
      tool: novanet_describe
      params:
        target: entity
        filters:
          key: "qr-code"

  - id: generate
    use:
      entity: discover    # Bind output of 'discover' to 'entity'
    invoke:
      mcp: novanet
      tool: novanet_generate
      params:
        # Access properties from previous task output
        entity: "{{use.entity.key}}"
        locale: "fr-FR"
        forms: "{{use.entity.supported_forms}}"

flows:
  - source: discover
    target: generate
```

### Response Handling

Tool responses are automatically parsed and stored:

```rust
// Simplified response handling in executor.rs
let result = mcp_client.call_tool(&tool_name, params).await?;

if result.is_error {
    return Err(NikaError::McpToolError {
        tool: tool_name,
        reason: result.text(),
    });
}

// Extract text content, parse as JSON if output.format == "json"
let output = match output_format {
    Some("json") => serde_json::from_str(&result.text())?,
    _ => serde_json::Value::String(result.text()),
};

// Store in DataStore for downstream tasks
data_store.insert(task_id, output);
```

### Error Propagation

| Error Type | Cause | NikaError Variant |
|------------|-------|-------------------|
| Server not found | MCP server name not in config | `McpNotConnected` |
| Connection failed | Server process failed to start | `McpStartError` |
| Tool not found | Invalid tool name | `McpToolError` |
| Invalid params | Schema validation failed | `McpToolError` |
| Execution error | Tool threw an error | `McpToolError` |
| Timeout | Tool took too long | `McpToolError` |

---

## 4. The agent: Verb with MCP Tools

The `agent:` verb enables multi-turn agentic execution where an LLM can autonomously call MCP tools.

### Syntax

```yaml
tasks:
  - id: research_agent
    agent:
      prompt: |
        Research QR code entities and generate a comprehensive summary.
        Use novanet_search to find related entities.
        Use novanet_traverse to explore relationships.
        When done, output your findings and say "RESEARCH_COMPLETE".

      provider: claude
      model: claude-sonnet-4
      mcp:
        - novanet                # MCP servers available to the agent
      max_turns: 10              # Maximum agentic turns
      token_budget: 50000        # Total token budget (optional)
      stop_conditions:           # Early stop triggers
        - "RESEARCH_COMPLETE"
      extended_thinking: true    # Enable Claude's reasoning capture (v0.4+)
      thinking_budget: 4096      # Tokens for thinking (default: 4096)

    output:
      format: json
```

### NikaMcpTool: rig-core Integration

Nika wraps MCP tools to integrate with rig-core's AgentBuilder:

```rust
// tools/nika/src/provider/rig.rs

/// Tool definition for Nika MCP tools
pub struct NikaMcpToolDef {
    pub name: String,           // e.g., "novanet_generate"
    pub description: String,    // Tool description for LLM
    pub input_schema: Value,    // JSON Schema for parameters
}

/// MCP tool wrapper implementing rig-core's ToolDyn trait
pub struct NikaMcpTool {
    definition: NikaMcpToolDef,
    client: Option<Arc<McpClient>>,
}

impl ToolDyn for NikaMcpTool {
    fn name(&self) -> String {
        self.definition.name.clone()
    }

    fn definition(&self, _prompt: String) -> BoxFuture<'_, ToolDefinition> {
        Box::pin(async move {
            ToolDefinition {
                name: self.definition.name.clone(),
                description: self.definition.description.clone(),
                parameters: self.definition.input_schema.clone(),
            }
        })
    }

    fn call(&self, args: String) -> BoxFuture<'_, Result<String, ToolError>> {
        Box::pin(async move {
            let params: Value = serde_json::from_str(&args)?;
            let result = self.client.call_tool(&self.tool_name, params).await?;
            Ok(result.text())
        })
    }
}
```

### RigAgentLoop Execution

```rust
// tools/nika/src/runtime/rig_agent_loop.rs

pub struct RigAgentLoop {
    task_id: String,
    params: AgentParams,
    event_log: EventLog,
    mcp_clients: FxHashMap<String, Arc<McpClient>>,
    tools: Vec<Box<dyn rig::tool::ToolDyn>>,
}

impl RigAgentLoop {
    pub async fn run_claude(&mut self) -> Result<RigAgentLoopResult, NikaError> {
        let client = anthropic::Client::from_env();
        let model = client.completion_model(CLAUDE_3_5_SONNET);

        // Build agent with tools
        let agent = AgentBuilder::new(model)
            .preamble(&self.params.prompt)
            .tools(std::mem::take(&mut self.tools))
            .build();

        // Run multi-turn execution
        let response = agent
            .prompt(&self.params.prompt)
            .max_turns(self.params.effective_max_turns())
            .await?;

        // Check stop conditions
        let status = if self.check_stop_conditions(&response) {
            RigAgentStatus::StopConditionMet
        } else {
            RigAgentStatus::NaturalCompletion
        };

        Ok(RigAgentLoopResult { status, turns, final_output, total_tokens })
    }
}
```

### Agent Loop with MCP Tools Flow

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                        AGENT LOOP WITH MCP TOOLS                               │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                │
│  ┌─────────────┐                                                              │
│  │  AgentStart │  EventLog: task_id, max_turns, mcp_servers                   │
│  └──────┬──────┘                                                              │
│         │                                                                      │
│         ▼                                                                      │
│  ┌──────────────────────────────────────────────────────────────────────────┐ │
│  │                        TURN 1                                             │ │
│  │  ┌─────────────────┐                                                     │ │
│  │  │ LLM: Analyze    │  "I need to search for QR code entities..."        │ │
│  │  │     prompt      │                                                     │ │
│  │  └────────┬────────┘                                                     │ │
│  │           │                                                               │ │
│  │           ▼                                                               │ │
│  │  ┌─────────────────┐                                                     │ │
│  │  │ Tool Call:      │  novanet_search { query: "qr*", kinds: ["Entity"] } │ │
│  │  │ novanet_search  │                                                     │ │
│  │  └────────┬────────┘                                                     │ │
│  │           │                                                               │ │
│  │           ▼                                                               │ │
│  │  ┌─────────────────┐                                                     │ │
│  │  │  MCP Call via   │  NikaMcpTool.call() → McpClient.call_tool()         │ │
│  │  │  NikaMcpTool    │                                                     │ │
│  │  └────────┬────────┘                                                     │ │
│  │           │                                                               │ │
│  │           ▼                                                               │ │
│  │  ┌─────────────────┐                                                     │ │
│  │  │  Tool Result    │  { entities: ["qr-code", "qr-code-generator", ...]} │ │
│  │  └─────────────────┘                                                     │ │
│  └──────────────────────────────────────────────────────────────────────────┘ │
│         │                                                                      │
│         ▼                                                                      │
│  ┌──────────────────────────────────────────────────────────────────────────┐ │
│  │                        TURN 2                                             │ │
│  │  ┌─────────────────┐                                                     │ │
│  │  │ LLM: Process    │  "Found 5 entities. Let me traverse relationships" │ │
│  │  │     results     │                                                     │ │
│  │  └────────┬────────┘                                                     │ │
│  │           │                                                               │ │
│  │           ▼                                                               │ │
│  │  ┌─────────────────┐                                                     │ │
│  │  │ Tool Call:      │  novanet_traverse { start_key: "qr-code", ... }     │ │
│  │  │ novanet_traverse│                                                     │ │
│  │  └─────────────────┘                                                     │ │
│  └──────────────────────────────────────────────────────────────────────────┘ │
│         │                                                                      │
│         ▼                                                                      │
│        ...  (continues until stop condition or max_turns)                      │
│         │                                                                      │
│         ▼                                                                      │
│  ┌─────────────┐                                                              │
│  │AgentComplete│  EventLog: turns, stop_reason, final_output                 │
│  └─────────────┘                                                              │
│                                                                                │
└────────────────────────────────────────────────────────────────────────────────┘
```

### Extended Thinking (v0.4+)

When `extended_thinking: true` is set, Claude's reasoning process is captured:

```yaml
tasks:
  - id: analysis
    agent:
      prompt: "Analyze the QR code market"
      provider: claude
      extended_thinking: true
      thinking_budget: 8192     # More tokens for deeper analysis
```

The thinking is captured in `AgentTurnMetadata`:

```rust
pub struct AgentTurnMetadata {
    pub thinking: Option<String>,    // Claude's <thinking> content
    pub response_text: String,       // Main response
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub cache_read_tokens: u32,
    pub stop_reason: String,
}
```

---

## 5. Zero Cypher Rule (ADR-003)

**Critical Architecture Decision:** Nika workflows NEVER use raw Cypher queries.

### Why?

| Factor | Direct Neo4j | MCP-Only |
|--------|--------------|----------|
| **Coupling** | Tight | Loose |
| **Schema changes** | Break Nika | Transparent |
| **Security** | Cypher injection risk | Validated tools |
| **Caching** | Manual | MCP server handles |
| **Observability** | Manual | MCP events |

### What's Prohibited

```yaml
# WRONG - Direct Cypher via exec:
- id: bad_query
  exec:
    command: "cypher-shell 'MATCH (e:Entity) RETURN e'"

# WRONG - Embedded Cypher in tool params
- id: also_bad
  invoke:
    mcp: novanet
    tool: novanet_query
    params:
      cypher: "MATCH (e:Entity {key: 'qr-code'}) RETURN e"
```

### What's Correct

```yaml
# RIGHT - Semantic MCP tools
- id: get_entity
  invoke:
    mcp: novanet
    tool: novanet_describe
    params:
      target: entity
      filters:
        key: "qr-code"

# RIGHT - Graph traversal via tool
- id: traverse
  invoke:
    mcp: novanet
    tool: novanet_traverse
    params:
      start_key: "qr-code"
      direction: outgoing
      arc_families: ["localization", "ownership"]
      max_depth: 2
```

### Enforcement

- Workflow validation rejects `cypher:` parameters
- `novanet_query` tool has restricted access (audit logging)
- CI/CD checks for prohibited patterns

---

## 6. Denomination Forms Integration

ADR-033 defines prescriptive canonical forms for entity references.

### What are Denomination Forms?

Each entity has locale-specific naming forms:

| Form | Purpose | Example (fr-FR) |
|------|---------|-----------------|
| `text` | Running prose | "code QR" |
| `title` | Headlines, H1 | "Code QR" |
| `abbrev` | Short reference | "QR" |
| `url` | URL slug | "code-qr" |
| `mixed` | Variable casing | "Code QR" |
| `base` | Invariant key | "qr-code" |

### novanet_generate Returns denomination_forms

```json
{
  "prompt": "# Generation Context for qr-code (fr-FR)...",
  "denomination_forms": {
    "text": "code QR",
    "title": "Code QR",
    "abbrev": "QR",
    "url": "code-qr",
    "mixed": "Code QR",
    "base": "qr-code"
  },
  "context_build_log": [
    { "step": "entity_lookup", "duration_ms": 12 },
    { "step": "native_load", "duration_ms": 8 }
  ],
  "locale_context": { "locale_key": "fr-FR", "language": "French" }
}
```

### Using denomination_forms in Workflows

```yaml
tasks:
  - id: get_context
    invoke:
      mcp: novanet
      tool: novanet_generate
      params:
        entity: "qr-code"
        locale: "fr-FR"
        forms: ["text", "title", "abbrev"]

  - id: generate_hero
    use:
      ctx: get_context
    infer:
      prompt: |
        Generate a hero section for QR Code.

        DENOMINATION FORMS (USE EXACTLY):
        - Title: {{use.ctx.denomination_forms.title}}
        - Text: {{use.ctx.denomination_forms.text}}
        - Abbreviation: {{use.ctx.denomination_forms.abbrev}}

        RULES:
        - Use ONLY these forms for entity references
        - NO invention, NO paraphrase
        - Maintain consistent casing

        Output JSON: { title, subtitle, description }
```

### context_build_log for Observability

The `context_build_log` shows how context was assembled:

```json
{
  "context_build_log": [
    { "step": "entity_lookup", "duration_ms": 12, "result": "found" },
    { "step": "native_load", "duration_ms": 8, "locale": "fr-FR" },
    { "step": "knowledge_atoms", "duration_ms": 45, "count": 23 },
    { "step": "traversal", "duration_ms": 67, "depth": 2, "nodes": 15 }
  ]
}
```

---

## 7. Workflow Patterns

### Pattern 1: Simple Entity Lookup

```yaml
# Minimal invoke: to get entity details
schema: "nika/workflow@0.2"
provider: claude

mcp:
  novanet:
    command: cargo
    args: ["run", "--manifest-path", "../novanet-mcp/Cargo.toml"]
    env:
      NOVANET_MCP_NEO4J_URI: bolt://localhost:7687

tasks:
  - id: lookup
    invoke:
      mcp: novanet
      tool: novanet_describe
      params:
        target: entity
        filters:
          key: "qr-code"
```

### Pattern 2: Entity Pipeline (invoke + infer)

```yaml
# Two-step: get context, then generate
tasks:
  - id: get_context
    invoke:
      mcp: novanet
      tool: novanet_generate
      params:
        entity: "qr-code"
        locale: "fr-FR"
        forms: ["text", "title"]

  - id: generate_content
    use:
      ctx: get_context
    infer:
      prompt: |
        Generate landing page content using this context:
        {{use.ctx}}

        Output JSON with: title, subtitle, description
    output:
      format: json

flows:
  - source: get_context
    target: generate_content
```

### Pattern 3: Multi-Entity with for_each

```yaml
# Parallel processing of multiple locales
tasks:
  - id: generate_all
    for_each:
      items: ["fr-FR", "en-US", "es-ES", "de-DE", "ja-JP"]
      as: locale
      concurrency: 5      # All run in parallel
      fail_fast: true
    invoke:
      mcp: novanet
      tool: novanet_generate
      params:
        entity: "qr-code"
        locale: "{{use.locale}}"
        token_budget: 2000
    output:
      format: json

  - id: summary
    use:
      results: generate_all
    infer:
      prompt: |
        Summarize generation results for {{use.results | length}} locales.
        Results: {{use.results}}

flows:
  - source: generate_all
    target: summary
```

### for_each Parallel Execution

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                    for_each PARALLEL EXECUTION (concurrency: 5)                │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                │
│  items: ["fr-FR", "en-US", "es-ES", "de-DE", "ja-JP"]                         │
│                                                                                │
│         │                                                                      │
│         ▼                                                                      │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │                      tokio::spawn JoinSet                               │  │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────┐ │  │
│  │  │  fr-FR    │ │  en-US    │ │  es-ES    │ │  de-DE    │ │  ja-JP    │ │  │
│  │  │  invoke   │ │  invoke   │ │  invoke   │ │  invoke   │ │  invoke   │ │  │
│  │  │  MCP call │ │  MCP call │ │  MCP call │ │  MCP call │ │  MCP call │ │  │
│  │  └─────┬─────┘ └─────┬─────┘ └─────┬─────┘ └─────┬─────┘ └─────┬─────┘ │  │
│  │        │             │             │             │             │       │  │
│  │        │             │             │             │             │       │  │
│  │  ┌─────▼─────┐ ┌─────▼─────┐ ┌─────▼─────┐ ┌─────▼─────┐ ┌─────▼─────┐ │  │
│  │  │  Result   │ │  Result   │ │  Result   │ │  Result   │ │  Result   │ │  │
│  │  │  fr-FR    │ │  en-US    │ │  es-ES    │ │  de-DE    │ │  ja-JP    │ │  │
│  │  └───────────┘ └───────────┘ └───────────┘ └───────────┘ └───────────┘ │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
│                                      │                                         │
│                                      ▼                                         │
│                        ┌─────────────────────────┐                            │
│                        │  Collect all results    │                            │
│                        │  (original order)       │                            │
│                        └─────────────────────────┘                            │
│                                                                                │
└────────────────────────────────────────────────────────────────────────────────┘
```

### Pattern 4: Agentic Research

```yaml
# Agent autonomously explores the knowledge graph
tasks:
  - id: agent_research
    agent:
      prompt: |
        Research all QR code related entities and their relationships.

        Use these tools:
        1. novanet_search - Find entities matching "qr*"
        2. novanet_traverse - Explore relationships from entities
        3. novanet_describe - Get detailed entity information

        Create a comprehensive report with:
        - Entity hierarchy
        - Key relationships
        - Locale coverage

        When complete, say "RESEARCH_COMPLETE"

      mcp:
        - novanet
      max_turns: 10
      stop_conditions:
        - "RESEARCH_COMPLETE"

    output:
      format: json
```

---

## 8. RLM-on-KG Pattern

NovaNet + Nika implements the **Recursive Language Model on Knowledge Graph** pattern.

### What is RLM-on-KG?

A pattern where an LLM agent recursively queries and updates a knowledge graph to solve complex tasks.

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                           RLM-on-KG ARCHITECTURE                               │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                │
│                         ┌─────────────────────┐                               │
│                         │    LLM Agent        │                               │
│                         │    (Policy)         │                               │
│                         └──────────┬──────────┘                               │
│                                    │                                          │
│               ┌────────────────────┼────────────────────┐                     │
│               │                    │                    │                     │
│               ▼                    ▼                    ▼                     │
│      ┌─────────────┐      ┌─────────────┐      ┌─────────────┐               │
│      │   Observe   │      │    Act      │      │   Reward    │               │
│      │  (traverse) │      │  (generate) │      │ (evaluate)  │               │
│      └──────┬──────┘      └──────┬──────┘      └──────┬──────┘               │
│             │                    │                    │                       │
│             └────────────────────┼────────────────────┘                       │
│                                  │                                            │
│                                  ▼                                            │
│                         ┌─────────────────────┐                               │
│                         │   Knowledge Graph   │                               │
│                         │   (MDP State)       │                               │
│                         │  ┌───────────────┐  │                               │
│                         │  │ 61 NodeClasses│  │                               │
│                         │  │ 182 ArcClasses│  │                               │
│                         │  │ Neo4j Backend │  │                               │
│                         │  └───────────────┘  │                               │
│                         └─────────────────────┘                               │
│                                                                                │
└────────────────────────────────────────────────────────────────────────────────┘
```

### NovaNet + Nika vs. rig-rlm

| Aspect | rig-rlm | NovaNet + Nika |
|--------|---------|----------------|
| Graph Structure | Generic | Domain-specific (61 NodeClasses) |
| Schema | Implicit | Explicit ADRs (32 decisions) |
| Observability | Basic | Full EventLog (16 variants) |
| Safety | Runtime | Compile-time + runtime |
| Locality | Implicit | BCP-47 first-class |

### Why NovaNet + Nika is Better

1. **Explicit Schema**: 61 NodeClasses, 182 ArcClasses with semantic meaning
2. **ADR-Driven Design**: 32 Architecture Decision Records
3. **Full Observability**: Every MCP call traced in EventLog
4. **Locality First**: denomination_forms, locale context
5. **Zero Cypher Rule**: Security through abstraction

---

## 9. Observability Integration

### EventLog Captures MCP Calls

```rust
// Event variants for MCP operations
pub enum EventKind {
    // MCP tool call initiated
    McpInvoke {
        task_id: Arc<str>,
        call_id: String,          // UUID for correlation
        mcp_server: String,       // "novanet"
        tool: Option<String>,     // "novanet_generate"
        resource: Option<String>, // Or resource URI
    },

    // MCP operation completed
    McpResponse {
        task_id: Arc<str>,
        call_id: String,          // Correlates with McpInvoke
        output_len: usize,        // Response size
        duration_ms: u64,         // Call duration
        cached: bool,             // Cache hit
        is_error: bool,           // Tool error
    },
}
```

### NDJSON Trace Format

```json
{"id":0,"timestamp_ms":0,"kind":{"type":"workflow_started","task_count":3}}
{"id":1,"timestamp_ms":12,"kind":{"type":"task_started","task_id":"get_context"}}
{"id":2,"timestamp_ms":15,"kind":{"type":"mcp_invoke","task_id":"get_context","call_id":"abc123","mcp_server":"novanet","tool":"novanet_generate"}}
{"id":3,"timestamp_ms":234,"kind":{"type":"mcp_response","task_id":"get_context","call_id":"abc123","output_len":4523,"duration_ms":219,"cached":false,"is_error":false}}
{"id":4,"timestamp_ms":235,"kind":{"type":"task_completed","task_id":"get_context","duration_ms":223}}
```

### context_build_log from NovaNet

NovaNet's `novanet_generate` returns observability data:

```json
{
  "context_build_log": [
    {
      "step": "entity_lookup",
      "duration_ms": 12,
      "cypher": "MATCH (e:Entity {key: $key}) RETURN e",
      "result": "found"
    },
    {
      "step": "native_load",
      "duration_ms": 8,
      "locale": "fr-FR",
      "result": "found"
    },
    {
      "step": "knowledge_atoms",
      "duration_ms": 45,
      "count": 23,
      "types": ["Term", "Expression", "CultureRef"]
    },
    {
      "step": "traversal",
      "duration_ms": 67,
      "depth": 2,
      "nodes_visited": 15,
      "arcs_followed": 12
    }
  ]
}
```

### Trace Correlation

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                         TRACE CORRELATION                                      │
├────────────────────────────────────────────────────────────────────────────────┤
│                                                                                │
│  Nika EventLog                         NovaNet context_build_log              │
│  ─────────────                         ──────────────────────────             │
│                                                                                │
│  McpInvoke {                                                                   │
│    call_id: "abc123"  ─────────────┐                                          │
│    tool: "novanet_generate"        │                                          │
│  }                                 │                                          │
│        │                           │                                          │
│        │                           └──► context_build_log: [                  │
│        │                                  { step: "entity_lookup", ... },     │
│        │                                  { step: "native_load", ... },       │
│        │                                  { step: "knowledge_atoms", ... }    │
│        │                                ]                                     │
│        │                                                                      │
│        ▼                                                                      │
│  McpResponse {                                                                 │
│    call_id: "abc123"  ◄────────────── Response includes context_build_log    │
│    duration_ms: 219                                                           │
│  }                                                                            │
│                                                                                │
└────────────────────────────────────────────────────────────────────────────────┘
```

---

## 10. Error Handling

### Error Types and Propagation

| Error Source | NikaError Variant | HTTP-like Code |
|--------------|-------------------|----------------|
| MCP server not found | `McpNotConnected` | 503 |
| MCP server start failed | `McpStartError` | 503 |
| Tool not found | `McpToolError` | 404 |
| Invalid parameters | `McpToolError` | 400 |
| Tool execution error | `McpToolError` | 500 |
| Connection timeout | `McpToolError` | 504 |
| Entity not found | `McpToolError` | 404 |

### Error Handling in Workflows

```yaml
tasks:
  - id: get_entity
    invoke:
      mcp: novanet
      tool: novanet_describe
      params:
        target: entity
        filters:
          key: "nonexistent-entity"
    # If this fails, the workflow stops (fail_fast: true by default)

  - id: fallback
    use:
      error: get_entity
    when: "{{use.error}}"  # Conditional execution on error
    infer:
      prompt: "Handle missing entity: {{use.error}}"
```

### Retry Behavior

By default, MCP calls do NOT retry (to avoid duplicate side effects). For idempotent operations, consider:

```yaml
# Manual retry pattern
tasks:
  - id: attempt_1
    invoke:
      mcp: novanet
      tool: novanet_describe
      params: { target: entity, filters: { key: "qr-code" } }

  - id: attempt_2
    when: "{{use.attempt_1.error}}"
    invoke:
      mcp: novanet
      tool: novanet_describe
      params: { target: entity, filters: { key: "qr-code" } }
```

---

## 11. Development Workflow

### Step 1: Start Neo4j

```bash
# Using Docker
docker run -d \
  --name neo4j \
  -p 7474:7474 -p 7687:7687 \
  -e NEO4J_AUTH=neo4j/novanetpassword \
  neo4j:5.15

# Or using neo4j-admin
neo4j start
```

### Step 2: Seed Database

```bash
cd novanet-dev
cargo run -- db reset
cargo run -- db seed
```

### Step 3: Test MCP Server Standalone

```bash
cd novanet-dev/tools/novanet-mcp
NOVANET_MCP_NEO4J_URI=bolt://localhost:7687 \
NOVANET_MCP_NEO4J_USER=neo4j \
NOVANET_MCP_NEO4J_PASSWORD=novanetpassword \
cargo run

# In another terminal, test with MCP client
echo '{"method":"tools/list","params":{},"id":1}' | nc localhost 3000
```

### Step 4: Configure Workflow

```yaml
# examples/test-integration.nika.yaml
schema: "nika/workflow@0.2"
provider: claude

mcp:
  novanet:
    command: cargo
    args:
      - run
      - --manifest-path
      - ../../../novanet-dev/tools/novanet-mcp/Cargo.toml
    env:
      NOVANET_MCP_NEO4J_URI: bolt://localhost:7687
      NOVANET_MCP_NEO4J_USER: neo4j
      NOVANET_MCP_NEO4J_PASSWORD: novanetpassword
      RUST_LOG: info

tasks:
  - id: test
    invoke:
      mcp: novanet
      tool: novanet_describe
      params:
        target: schema
```

### Step 5: Run Workflow

```bash
cd nika-dev/tools/nika
cargo run -- run examples/test-integration.nika.yaml
```

### Step 6: Inspect Traces

```bash
# List traces
cargo run -- trace list

# Show specific trace
cargo run -- trace show <trace-id>

# Export as JSON
cargo run -- trace export <trace-id> --format json
```

---

## 12. Debugging Integration Issues

### Common Problems and Solutions

#### Problem: MCP Server Not Starting

```
Error: McpStartError { name: "novanet", reason: "Failed to create transport" }
```

**Solutions:**
1. Check if Cargo.toml path is correct (relative to workflow location)
2. Verify Neo4j environment variables are set
3. Check if port 7687 is accessible

```bash
# Test Neo4j connection
cypher-shell -u neo4j -p novanetpassword -a bolt://localhost:7687 "RETURN 1"
```

#### Problem: Tool Not Found

```
Error: McpToolError { tool: "novanet_generates", reason: "Tool not found" }
```

**Solution:** Check tool name spelling. Valid tools:
- `novanet_describe` (not `describe`)
- `novanet_generate` (not `generates`)
- `novanet_search`, `novanet_traverse`, `novanet_assemble`, `novanet_atoms`, `novanet_query`

#### Problem: Entity Not Found

```json
{"error": "Entity 'unknown-entity' not found in knowledge graph"}
```

**Solutions:**
1. Verify entity exists: `cargo run -- tui` in novanet-dev
2. Check entity key (case-sensitive)
3. Seed database if empty: `cargo run -- db seed`

#### Problem: Empty denomination_forms

```json
{"denomination_forms": {}}
```

**Solutions:**
1. Entity exists but has no EntityNative for requested locale
2. Check locale code is valid BCP-47 (e.g., "fr-FR" not "french")
3. Seed locale-specific content

### Enable Debug Logging

```yaml
mcp:
  novanet:
    env:
      RUST_LOG: debug  # or trace for maximum detail
```

### MCP Server Logs

NovaNet MCP server logs to stderr:

```bash
# See server logs during workflow execution
cargo run -- run workflow.yaml 2>&1 | grep -E "(novanet|MCP|Neo4j)"
```

### Trace Analysis

```bash
# Full trace with all events
cargo run -- trace show <id> --format yaml

# Filter to MCP events only
cargo run -- trace show <id> | grep -E "(mcp_invoke|mcp_response)"
```

---

## Quick Reference

### MCP Tools Summary

| Tool | Use When |
|------|----------|
| `novanet_describe` | Bootstrapping agent understanding, schema discovery |
| `novanet_search` | Finding entities by query, fulltext search |
| `novanet_traverse` | Exploring relationships, graph navigation |
| `novanet_assemble` | Building token-aware context |
| `novanet_atoms` | Getting locale-specific knowledge atoms |
| `novanet_generate` | Full context for content generation |
| `novanet_query` | Advanced queries (restricted) |

### Verb Selection Guide

| Use Case | Verb |
|----------|------|
| Single MCP tool call | `invoke:` |
| LLM text generation | `infer:` |
| Shell command | `exec:` |
| HTTP request | `fetch:` |
| Multi-turn with tools | `agent:` |

### Key ADRs

| ADR | Topic |
|-----|-------|
| ADR-003 | Zero Cypher Rule |
| ADR-029 | *Native Pattern (EntityNative, PageNative) |
| ADR-030 | Slug Ownership (Page owns URL) |
| ADR-033 | Denomination Forms (prescriptive naming) |

---

## Related Documentation

- `/novanet-dev/CLAUDE.md` - NovaNet developer guide
- `/nika-dev/CLAUDE.md` - Nika developer guide
- `/ROADMAP.md` - MVP status and roadmap
- `/docs/research/rlm-knowledge-graph-patterns-2025.md` - RLM-on-KG research
- `.claude/rules/adr/` - All 32 Architecture Decision Records
