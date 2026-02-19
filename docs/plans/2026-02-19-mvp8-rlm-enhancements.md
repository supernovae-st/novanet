# MVP 8: RLM Enhancements Implementation Plan

**Date:** 2026-02-19
**Status:** Ready for Implementation
**Target Version:** v0.4.1 → v0.6
**Research:** `docs/research/rlm-knowledge-graph-patterns-2025.md` (Section 11)

---

## Overview

Enhance NovaNet + Nika with full RLM-on-KG (Recursive Language Model on Knowledge Graph) capabilities.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  MVP 8: RLM ENHANCEMENTS PHASES                                                 │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  Phase 1: Reasoning Capture ────────────────────────────── v0.4.1 (Low)        │
│  ├── Add thinking field to AgentTurn events                                    │
│  ├── Switch from Prompt to ChatCompletion API                                  │
│  └── Token tracking per turn                                                   │
│                                                                                 │
│  Phase 2: Nested Agents ────────────────────────────────── v0.5 (Medium)       │
│  ├── spawn_agent internal tool                                                 │
│  ├── Depth limiting (max 3)                                                    │
│  └── Event propagation                                                         │
│                                                                                 │
│  Phase 3: Schema Introspection ─────────────────────────── v0.5 (Medium)       │
│  ├── novanet_introspect MCP tool (NovaNet side)                               │
│  ├── novanet_explain MCP tool                                                 │
│  └── Graph schema as environment                                               │
│                                                                                 │
│  Phase 4: Dynamic Decomposition ────────────────────────── v0.6 (High)         │
│  ├── decompose: YAML modifier                                                  │
│  ├── 3 strategies: llm, fixed, semantic                                        │
│  └── Graph-native subtask discovery                                            │
│                                                                                 │
│  Phase 5: Lazy Context Loading ─────────────────────────── v0.6 (Medium)       │
│  ├── lazy: true binding modifier                                               │
│  └── Deferred context assembly                                                 │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Reasoning Capture (v0.4.1)

**Goal:** Capture Claude's `<thinking>` blocks in NDJSON traces for debugging.

### Task 1.1: Add AgentTurnMetadata Struct

**File:** `src/event/log.rs`

```rust
/// Agent turn response metadata (v0.4.1)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentTurnMetadata {
    /// Thinking content (if extended thinking enabled)
    pub thinking: Option<String>,
    /// Main response text
    pub response_text: String,
    /// Input tokens used
    pub input_tokens: u32,
    /// Output tokens used
    pub output_tokens: u32,
    /// Cache read tokens (if any)
    pub cache_read_tokens: u32,
    /// Stop reason ("end_turn", "tool_use", "max_tokens", etc.)
    pub stop_reason: String,
}
```

**Changes:**
- Add `AgentTurnMetadata` struct (line ~180)
- Update `EventKind::AgentTurn` variant:
  ```rust
  AgentTurn {
      task_id: Arc<str>,
      turn_index: u32,
      kind: String,
      metadata: Option<AgentTurnMetadata>,  // NEW (replaces tokens: Option<u32>)
  },
  ```

### Task 1.2: Switch to ChatCompletion API

**File:** `src/runtime/rig_agent_loop.rs`

**Current** (line 252-281):
```rust
let response = agent.prompt(&self.params.prompt).max_turns(max_turns).await?;
// Returns String only - metadata lost
```

**New:**
```rust
use rig::completion::{ChatCompletion, ContentBlock};

let completion = model
    .chat(messages)
    .tools(tools)
    .max_tokens(Some(4096))
    .await?;  // Returns Message with content_blocks + usage

// Extract thinking + response
let mut thinking = None;
let mut response_text = String::new();

for block in &completion.content {
    match block {
        ContentBlock::Thinking(content) => thinking = Some(content.clone()),
        ContentBlock::Text(content) => response_text.push_str(content),
        _ => {}
    }
}

let metadata = AgentTurnMetadata {
    thinking,
    response_text: response_text.clone(),
    input_tokens: completion.usage.input_tokens,
    output_tokens: completion.usage.output_tokens,
    cache_read_tokens: completion.usage.cache_read_tokens.unwrap_or(0),
    stop_reason: completion.stop_reason.clone(),
};
```

### Task 1.3: Update EventLog Emission

**File:** `src/runtime/rig_agent_loop.rs` (line 284-297)

```rust
self.event_log.emit(EventKind::AgentTurn {
    task_id: Arc::from(self.task_id.as_str()),
    turn_index: 1,
    kind: format!("{:?}", status),
    metadata: Some(metadata),  // NEW
});
```

### Task 1.4: Update Tests

**File:** `tests/agent_loop_test.rs`

Add tests:
- `test_agent_turn_captures_thinking()`
- `test_agent_turn_tokens_tracked()`
- `test_trace_includes_thinking_ndjson()`

### Files Modified (Phase 1)

| File | Lines Changed | Description |
|------|---------------|-------------|
| `src/event/log.rs` | +25 | AgentTurnMetadata struct |
| `src/runtime/rig_agent_loop.rs` | +40, -15 | ChatCompletion API |
| `tests/agent_loop_test.rs` | +60 | New tests |

**Effort:** ~4 hours
**Risk:** Low (isolated to agent loop)

---

## Phase 2: Nested Agent Spawning (v0.5)

**Goal:** Enable agents to spawn child agents for sub-tasks.

### Task 2.1: Add Depth Tracking to RigAgentLoop

**File:** `src/runtime/rig_agent_loop.rs`

```rust
pub struct RigAgentLoop {
    task_id: String,
    params: AgentParams,
    event_log: EventLog,
    mcp_clients: FxHashMap<String, Arc<McpClient>>,
    tools: Vec<Box<dyn rig::tool::ToolDyn>>,
    depth: usize,      // NEW: Current nesting depth
    max_depth: usize,  // NEW: Max allowed (default: 3)
}

impl RigAgentLoop {
    pub fn new(...) -> Result<Self, NikaError> {
        Self::with_depth(task_id, params, event_log, mcp_clients, 0)
    }

    pub fn with_depth(
        task_id: String,
        params: AgentParams,
        event_log: EventLog,
        mcp_clients: FxHashMap<String, Arc<McpClient>>,
        depth: usize,
    ) -> Result<Self, NikaError> {
        let max_depth = 3;  // Configurable via AgentParams later

        if depth > max_depth {
            return Err(NikaError::AgentValidationError {
                reason: format!("Agent nesting depth {} exceeds max {}", depth, max_depth),
            });
        }

        // Build tools including spawn_agent
        let mut tools = Self::build_tools(&params.mcp, &mcp_clients)?;

        // Add spawn_agent internal tool
        tools.push(Box::new(SpawnAgentTool {
            parent_task_id: task_id.clone(),
            event_log: event_log.clone(),
            mcp_clients: mcp_clients.clone(),
            depth,
            max_depth,
        }));

        Ok(Self {
            task_id,
            params,
            event_log,
            mcp_clients,
            tools,
            depth,
            max_depth,
        })
    }
}
```

### Task 2.2: Implement SpawnAgentTool

**File:** `src/runtime/spawn.rs` (NEW)

```rust
use rig::tool::{Tool, ToolDyn, ToolError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SpawnAgentParams {
    pub prompt: String,
    #[serde(default)]
    pub mcp: Vec<String>,
    #[serde(default)]
    pub max_turns: Option<u32>,
}

pub struct SpawnAgentTool {
    pub parent_task_id: String,
    pub event_log: EventLog,
    pub mcp_clients: FxHashMap<String, Arc<McpClient>>,
    pub depth: usize,
    pub max_depth: usize,
}

impl Tool for SpawnAgentTool {
    const NAME: &'static str = "spawn_agent";

    type Error = ToolError;
    type Args = SpawnAgentParams;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "spawn_agent".to_string(),
            description: "Spawn a child agent for sub-tasks. Returns child's final output.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "prompt": {
                        "type": "string",
                        "description": "Task prompt for child agent"
                    },
                    "mcp": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "MCP servers to enable (default: inherit from parent)"
                    },
                    "max_turns": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 10,
                        "description": "Max turns for child (default: 5)"
                    }
                },
                "required": ["prompt"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let child_depth = self.depth + 1;

        // Check depth limit
        if child_depth > self.max_depth {
            self.event_log.emit(EventKind::AgentDepthExceeded {
                task_id: Arc::from(self.parent_task_id.as_str()),
                depth: child_depth,
                max_depth: self.max_depth,
            });
            return Err(ToolError::ToolCallError(Box::new(
                NikaError::AgentValidationError {
                    reason: format!("Max depth {} exceeded", self.max_depth),
                }
            )));
        }

        let child_task_id = format!("{}:child-{}", self.parent_task_id, child_depth);

        // Emit spawn event
        self.event_log.emit(EventKind::SpawnedAgent {
            parent_task_id: Arc::from(self.parent_task_id.as_str()),
            child_task_id: Arc::from(child_task_id.as_str()),
            depth: child_depth,
        });

        // Create child params
        let child_params = AgentParams {
            prompt: args.prompt,
            mcp: if args.mcp.is_empty() {
                // Inherit from parent's available MCP servers
                self.mcp_clients.keys().cloned().collect()
            } else {
                args.mcp
            },
            max_turns: Some(args.max_turns.unwrap_or(5).min(10)),
            ..Default::default()
        };

        // Create and run child agent
        let child_agent = RigAgentLoop::with_depth(
            child_task_id.clone(),
            child_params,
            self.event_log.clone(),
            self.mcp_clients.clone(),
            child_depth,
        ).map_err(|e| ToolError::ToolCallError(Box::new(e)))?;

        let result = child_agent.run_claude().await
            .map_err(|e| ToolError::ToolCallError(Box::new(e)))?;

        // Emit completion event
        self.event_log.emit(EventKind::AgentAggregated {
            parent_task_id: Arc::from(self.parent_task_id.as_str()),
            child_task_id: Arc::from(child_task_id.as_str()),
        });

        Ok(serde_json::to_string(&result.final_output)?)
    }
}
```

### Task 2.3: Add Event Variants

**File:** `src/event/log.rs`

```rust
// Add to EventKind enum:

/// Agent spawned a child agent (v0.5)
SpawnedAgent {
    parent_task_id: Arc<str>,
    child_task_id: Arc<str>,
    depth: usize,
},

/// Agent depth limit exceeded (v0.5)
AgentDepthExceeded {
    task_id: Arc<str>,
    depth: usize,
    max_depth: usize,
},

/// Child agent completed and results aggregated (v0.5)
AgentAggregated {
    parent_task_id: Arc<str>,
    child_task_id: Arc<str>,
},
```

### Task 2.4: Add Error Code

**File:** `src/error.rs`

```rust
#[error("[NIKA-114] Agent nesting depth {depth} exceeds maximum {max_depth}")]
AgentDepthExceeded { depth: usize, max_depth: usize },
```

### Files Modified (Phase 2)

| File | Lines Changed | Description |
|------|---------------|-------------|
| `src/runtime/spawn.rs` | +150 (NEW) | SpawnAgentTool implementation |
| `src/runtime/rig_agent_loop.rs` | +30 | Depth tracking, spawn_agent tool |
| `src/runtime/mod.rs` | +1 | Export spawn module |
| `src/event/log.rs` | +20 | 3 new event variants |
| `src/error.rs` | +3 | NIKA-114 error code |
| `tests/spawn_agent_test.rs` | +100 (NEW) | Integration tests |

**Effort:** ~8 hours
**Risk:** Medium (new execution path, but isolated)

---

## Phase 3: Schema Introspection (v0.5 - NovaNet Side)

**Goal:** Let agents query the NovaNet schema, not just data.

### Task 3.1: Add novanet_introspect MCP Tool

**File:** `novanet-dev/tools/novanet-mcp/src/tools/introspect.rs` (NEW)

```rust
pub struct IntrospectTool;

#[derive(Deserialize)]
pub struct IntrospectParams {
    pub node_class: Option<String>,
    pub arc_class: Option<String>,
    #[serde(default)]
    pub include: Vec<String>,  // "arcs", "properties", "constraints"
}

#[derive(Serialize)]
pub struct IntrospectResult {
    pub node_class: Option<NodeClassInfo>,
    pub arc_class: Option<ArcClassInfo>,
}

#[derive(Serialize)]
pub struct NodeClassInfo {
    pub name: String,
    pub realm: String,
    pub layer: String,
    pub trait_type: String,
    pub description: String,
    pub llm_context: String,
    pub properties: Vec<PropertyInfo>,
    pub outgoing_arcs: Vec<ArcInfo>,
    pub incoming_arcs: Vec<ArcInfo>,
}

impl McpTool for IntrospectTool {
    fn name(&self) -> &str { "novanet_introspect" }

    fn description(&self) -> &str {
        "Query NovaNet schema. Returns node/arc class definitions, properties, and constraints."
    }

    async fn execute(&self, params: Value) -> Result<Value, McpError> {
        let params: IntrospectParams = serde_json::from_value(params)?;

        if let Some(class_name) = params.node_class {
            // Read from YAML schema files
            let node_info = self.load_node_class(&class_name)?;
            Ok(serde_json::to_value(IntrospectResult {
                node_class: Some(node_info),
                arc_class: None,
            })?)
        } else if let Some(arc_name) = params.arc_class {
            let arc_info = self.load_arc_class(&arc_name)?;
            Ok(serde_json::to_value(IntrospectResult {
                node_class: None,
                arc_class: Some(arc_info),
            })?)
        } else {
            Err(McpError::InvalidParams("Specify node_class or arc_class".into()))
        }
    }
}
```

### Task 3.2: Add novanet_explain MCP Tool

**File:** `novanet-dev/tools/novanet-mcp/src/tools/explain.rs` (NEW)

```rust
pub struct ExplainTool;

#[derive(Deserialize)]
pub struct ExplainParams {
    pub from: String,      // "entity:qr-code"
    pub to: String,        // "locale:fr-FR"
    pub via: String,       // "HAS_NATIVE"
}

impl McpTool for ExplainTool {
    fn name(&self) -> &str { "novanet_explain" }

    fn description(&self) -> &str {
        "Explain the semantic meaning of a graph path in natural language."
    }

    async fn execute(&self, params: Value) -> Result<Value, McpError> {
        let params: ExplainParams = serde_json::from_value(params)?;

        // Load arc class info
        let arc_info = self.load_arc_class(&params.via)?;

        // Generate explanation using llm_context
        let explanation = format!(
            "Path: {} --[{}]--> {}\n\n{}\n\nThis relationship means: {}",
            params.from,
            params.via,
            params.to,
            arc_info.description,
            arc_info.llm_context
        );

        Ok(serde_json::json!({ "explanation": explanation }))
    }
}
```

### Files Modified (Phase 3 - NovaNet)

| File | Lines Changed | Description |
|------|---------------|-------------|
| `novanet-mcp/src/tools/introspect.rs` | +150 (NEW) | Schema introspection |
| `novanet-mcp/src/tools/explain.rs` | +80 (NEW) | Path explanation |
| `novanet-mcp/src/tools/mod.rs` | +2 | Export new tools |
| `novanet-mcp/src/server.rs` | +10 | Register tools |

**Effort:** ~6 hours
**Risk:** Low (read-only, isolated to NovaNet)

---

## Phase 4: Dynamic Decomposition (v0.6)

**Goal:** Runtime DAG expansion based on graph structure or LLM decomposition.

### Task 4.1: Create DecomposeSpec Struct

**File:** `src/ast/decompose.rs` (NEW)

```rust
use serde::Deserialize;

/// Strategy for decomposing a task
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "strategy", rename_all = "lowercase")]
pub enum DecomposeStrategy {
    /// Ask LLM to break down task into sub-tasks
    Llm {
        #[serde(default)]
        prompt_template: Option<String>,
    },

    /// Split by a delimiter (for text/structured input)
    Fixed {
        delimiter: String,
    },

    /// Use NovaNet graph structure to decompose
    Semantic {
        arc: String,
        #[serde(default)]
        source: Option<String>,
        #[serde(default)]
        limit: Option<usize>,
    },
}

/// Decomposition modifier spec
#[derive(Debug, Clone, Deserialize)]
pub struct DecomposeSpec {
    #[serde(flatten)]
    pub strategy: DecomposeStrategy,

    #[serde(default, rename = "as")]
    pub decompose_as: Option<String>,

    #[serde(default)]
    pub concurrency: Option<usize>,

    #[serde(default = "default_true")]
    pub fail_fast: bool,
}

fn default_true() -> bool { true }

impl DecomposeSpec {
    pub fn decompose_var(&self) -> &str {
        self.decompose_as.as_deref().unwrap_or("item")
    }

    pub fn effective_concurrency(&self) -> usize {
        self.concurrency.unwrap_or(1)
    }

    pub fn validate(&self) -> Result<(), String> {
        match &self.strategy {
            DecomposeStrategy::Semantic { source, .. } => {
                if source.is_none() || source.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
                    return Err("semantic strategy requires 'source' field".to_string());
                }
            }
            DecomposeStrategy::Fixed { delimiter } => {
                if delimiter.is_empty() {
                    return Err("fixed strategy requires non-empty 'delimiter'".to_string());
                }
            }
            DecomposeStrategy::Llm { .. } => {}
        }
        Ok(())
    }
}
```

### Task 4.2: Add to Task Struct

**File:** `src/ast/workflow.rs`

```rust
#[derive(Debug, Deserialize)]
pub struct Task {
    pub id: String,

    #[serde(default, rename = "use")]
    pub use_wiring: Option<WiringSpec>,

    #[serde(default)]
    pub output: Option<OutputPolicy>,

    #[serde(default)]
    pub for_each: Option<serde_json::Value>,

    #[serde(default, rename = "as")]
    pub for_each_as: Option<String>,

    // NEW: Phase 4 decompose modifier
    #[serde(default)]
    pub decompose: Option<DecomposeSpec>,

    #[serde(flatten)]
    pub action: TaskAction,
}

impl Task {
    pub fn validate_decompose(&self) -> Result<(), NikaError> {
        if let Some(decompose) = &self.decompose {
            decompose.validate()
                .map_err(|e| NikaError::ValidationError { reason: e })
        } else {
            Ok(())
        }
    }

    pub fn has_decompose(&self) -> bool {
        self.decompose.is_some()
    }
}
```

### Task 4.3: Implement Decomposition Execution

**File:** `src/runtime/decomposer.rs` (NEW)

```rust
pub struct Decomposer {
    executor: Arc<TaskExecutor>,
    mcp_clients: FxHashMap<String, Arc<McpClient>>,
}

impl Decomposer {
    pub async fn decompose(
        &self,
        spec: &DecomposeSpec,
        bindings: &Bindings,
    ) -> Result<Vec<Value>, NikaError> {
        match &spec.strategy {
            DecomposeStrategy::Semantic { arc, source, limit } => {
                self.decompose_semantic(arc, source.as_deref(), *limit, bindings).await
            }
            DecomposeStrategy::Llm { prompt_template } => {
                self.decompose_llm(prompt_template.as_deref(), bindings).await
            }
            DecomposeStrategy::Fixed { delimiter } => {
                self.decompose_fixed(delimiter, bindings).await
            }
        }
    }

    async fn decompose_semantic(
        &self,
        arc: &str,
        source: Option<&str>,
        limit: Option<usize>,
        bindings: &Bindings,
    ) -> Result<Vec<Value>, NikaError> {
        // Resolve source binding
        let source_key = source.ok_or_else(|| NikaError::DecomposeError {
            strategy: "semantic".into(),
            reason: "source is required".into(),
        })?;

        let source_value = bindings.resolve(source_key)?;

        // Call novanet_traverse via MCP
        let novanet = self.mcp_clients.get("novanet")
            .ok_or_else(|| NikaError::McpNotConnected { name: "novanet".into() })?;

        let result = novanet.call_tool("novanet_traverse", serde_json::json!({
            "start": source_value,
            "arc_filter": [arc],
            "depth": 1,
        })).await?;

        // Extract items from subgraph
        let nodes = result["nodes"].as_array()
            .ok_or_else(|| NikaError::DecomposeError {
                strategy: "semantic".into(),
                reason: "traverse returned no nodes".into(),
            })?;

        let items: Vec<Value> = nodes.iter()
            .take(limit.unwrap_or(100))
            .cloned()
            .collect();

        Ok(items)
    }

    async fn decompose_llm(
        &self,
        prompt_template: Option<&str>,
        bindings: &Bindings,
    ) -> Result<Vec<Value>, NikaError> {
        let prompt = prompt_template.unwrap_or(
            "Break down this task into 3-5 subtasks. Return JSON array."
        );

        let resolved = bindings.resolve_template(prompt)?;

        // Call infer via provider
        let response = self.executor.execute_infer(&resolved, bindings).await?;

        // Parse JSON array
        let items: Vec<Value> = serde_json::from_str(&response)
            .map_err(|e| NikaError::DecomposeError {
                strategy: "llm".into(),
                reason: format!("Invalid JSON: {}", e),
            })?;

        Ok(items)
    }

    async fn decompose_fixed(
        &self,
        delimiter: &str,
        bindings: &Bindings,
    ) -> Result<Vec<Value>, NikaError> {
        let source = bindings.get("source")?;
        let text = source.as_str().ok_or_else(|| NikaError::DecomposeError {
            strategy: "fixed".into(),
            reason: "source must be string".into(),
        })?;

        let items: Vec<Value> = text
            .split(delimiter)
            .map(|s| Value::String(s.trim().to_string()))
            .collect();

        Ok(items)
    }
}
```

### Task 4.4: Update Runner for Decompose

**File:** `src/runtime/runner.rs`

```rust
// In execute_workflow loop, add before for_each check:

if let Some(decompose) = &task.decompose {
    // Phase 4: Dynamic decomposition
    let decomposer = Decomposer::new(
        Arc::clone(&self.executor),
        self.mcp_clients.clone(),
    );

    let items = decomposer.decompose(decompose, &bindings).await?;

    // Same pattern as for_each: spawn parallel tasks
    let var_name = decompose.decompose_var().to_string();
    let concurrency = decompose.effective_concurrency();

    // Use semaphore for concurrency control
    let semaphore = Arc::new(Semaphore::new(concurrency));

    for (idx, item) in items.iter().enumerate() {
        let permit = semaphore.clone().acquire_owned().await?;
        let task_id = intern(&format!("{}[{}]", task.id, idx));

        join_set.spawn(async move {
            let _permit = permit;  // Hold permit during execution
            Self::execute_task_iteration(
                task,
                task_id,
                parent_task_id,
                datastore,
                executor,
                event_log,
                Some((var_name, item, idx)),
            ).await
        });
    }
} else if let Some(for_each) = &task.for_each {
    // Existing for_each logic...
}
```

### Task 4.5: Add Error Codes

**File:** `src/error.rs`

```rust
#[error("[NIKA-140] Decompose strategy '{strategy}' failed: {reason}")]
DecomposeError { strategy: String, reason: String },

#[error("[NIKA-141] Decompose traversal failed: {reason}")]
DecomposeTraversalFailed { reason: String },
```

### YAML Usage Examples

```yaml
# Semantic decomposition (graph-native)
tasks:
  - id: get_category
    invoke:
      mcp: novanet
      tool: novanet_describe
      params:
        class: EntityCategory
    use.ctx: result

  - id: generate_children
    decompose:
      strategy: semantic
      arc: HAS_CHILD
      source: $category
      concurrency: 3
      as: child
    infer: "Generate content for {{use.child.key}}"

# LLM decomposition
tasks:
  - id: break_down
    decompose:
      strategy: llm
      as: subtask
    infer: "Execute: {{use.subtask}}"

# Fixed delimiter
tasks:
  - id: process_lines
    decompose:
      strategy: fixed
      delimiter: "\n"
      as: line
    exec: "process {{use.line}}"
```

### Files Modified (Phase 4)

| File | Lines Changed | Description |
|------|---------------|-------------|
| `src/ast/decompose.rs` | +150 (NEW) | DecomposeSpec struct |
| `src/ast/workflow.rs` | +15 | Add decompose field |
| `src/ast/mod.rs` | +1 | Export decompose |
| `src/runtime/decomposer.rs` | +200 (NEW) | Decomposition logic |
| `src/runtime/runner.rs` | +50 | Decompose execution |
| `src/error.rs` | +6 | Error codes |
| `tests/decompose_test.rs` | +200 (NEW) | Integration tests |

**Effort:** ~16 hours
**Risk:** High (new execution path, MCP integration)

---

## Phase 5: Lazy Context Loading (v0.6)

**Goal:** Defer context assembly until first access for performance.

### Task 5.1: Add Lazy Binding Modifier

**File:** `src/binding/entry.rs`

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct WiringEntry {
    pub from: String,

    #[serde(default)]
    pub lazy: bool,  // NEW: Defer loading until access

    #[serde(default)]
    pub transform: Option<String>,
}
```

### Task 5.2: Implement LazyValue Wrapper

**File:** `src/binding/lazy.rs` (NEW)

```rust
pub enum LazyValue {
    Resolved(Value),
    Pending {
        source: String,
        resolver: Arc<dyn Fn() -> BoxFuture<'static, Result<Value, NikaError>> + Send + Sync>,
    },
}

impl LazyValue {
    pub async fn resolve(&mut self) -> Result<&Value, NikaError> {
        match self {
            LazyValue::Resolved(v) => Ok(v),
            LazyValue::Pending { source, resolver } => {
                let value = resolver().await?;
                *self = LazyValue::Resolved(value);
                if let LazyValue::Resolved(v) = self {
                    Ok(v)
                } else {
                    unreachable!()
                }
            }
        }
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self, LazyValue::Resolved(_))
    }
}
```

### Task 5.3: Update Bindings to Support Lazy

**File:** `src/binding/resolver.rs`

```rust
impl Bindings {
    pub fn get_lazy(&mut self, key: &str) -> Result<&Value, NikaError> {
        if let Some(entry) = self.entries.get_mut(key) {
            if let Some(lazy_val) = &mut entry.lazy_value {
                // Resolve on first access
                tokio::runtime::Handle::current().block_on(lazy_val.resolve())
            } else {
                Ok(&entry.value)
            }
        } else {
            Err(NikaError::BindingNotFound { key: key.into() })
        }
    }
}
```

### Files Modified (Phase 5)

| File | Lines Changed | Description |
|------|---------------|-------------|
| `src/binding/entry.rs` | +5 | lazy field |
| `src/binding/lazy.rs` | +80 (NEW) | LazyValue wrapper |
| `src/binding/resolver.rs` | +30 | Lazy resolution |
| `tests/lazy_binding_test.rs` | +60 (NEW) | Tests |

**Effort:** ~6 hours
**Risk:** Medium (async complexity)

---

## Summary

| Phase | Version | Effort | Files New | Files Modified | Tests |
|-------|---------|--------|-----------|----------------|-------|
| 1 | v0.4.1 | 4h | 0 | 3 | 4 |
| 2 | v0.5 | 8h | 2 | 5 | 6 |
| 3 | v0.5 | 6h | 2 (NovaNet) | 2 (NovaNet) | 4 |
| 4 | v0.6 | 16h | 3 | 4 | 8 |
| 5 | v0.6 | 6h | 1 | 2 | 3 |
| **Total** | | **40h** | **8** | **16** | **25** |

---

## Execution Order

```
Phase 1 (v0.4.1) ─────────────────────────────────────────────────────────────────
    │
    ├── Task 1.1: AgentTurnMetadata struct
    ├── Task 1.2: Switch to ChatCompletion API
    ├── Task 1.3: Update EventLog emission
    └── Task 1.4: Tests
    │
    ▼ Tag v0.4.1

Phase 2 (v0.5) ───────────────────────────────────────────────────────────────────
    │
    ├── Task 2.1: Depth tracking in RigAgentLoop
    ├── Task 2.2: SpawnAgentTool implementation
    ├── Task 2.3: Event variants
    └── Task 2.4: Error code + tests
    │
    │ (can run in parallel with Phase 3)
    ▼

Phase 3 (v0.5) ───────────────────────────────────────────────────────────────────
    │
    ├── Task 3.1: novanet_introspect MCP tool
    └── Task 3.2: novanet_explain MCP tool
    │
    ▼ Tag v0.5.0

Phase 4 (v0.6) ───────────────────────────────────────────────────────────────────
    │
    ├── Task 4.1: DecomposeSpec struct
    ├── Task 4.2: Add to Task struct
    ├── Task 4.3: Decomposer implementation
    ├── Task 4.4: Runner integration
    └── Task 4.5: Error codes + tests

Phase 5 (v0.6) ───────────────────────────────────────────────────────────────────
    │
    ├── Task 5.1: Lazy binding modifier
    ├── Task 5.2: LazyValue wrapper
    └── Task 5.3: Bindings update
    │
    ▼ Tag v0.6.0
```

---

## Success Criteria

- [ ] **Phase 1:** NDJSON traces contain `thinking` blocks
- [ ] **Phase 2:** Agent can spawn child agents with depth limit
- [ ] **Phase 3:** `novanet_introspect` returns schema info
- [ ] **Phase 4:** `decompose:` creates runtime subtasks
- [ ] **Phase 5:** Lazy bindings defer loading until access
- [ ] All tests pass (600+ existing + 25 new)
- [ ] No performance regression on benchmarks
