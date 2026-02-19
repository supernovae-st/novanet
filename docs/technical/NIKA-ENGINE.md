# Nika Engine Technical Specification

**Version:** 0.4.1
**Last Updated:** 2026-02-19
**Test Count:** 621+ tests passing

---

## Table of Contents

1. [Overview](#1-overview)
2. [AST Module Architecture](#2-ast-module-architecture)
3. [DAG Module Architecture](#3-dag-module-architecture)
4. [Runtime Architecture](#4-runtime-architecture)
5. [Provider System v0.4](#5-provider-system-v04)
6. [MCP Client Architecture](#6-mcp-client-architecture)
7. [Event System](#7-event-system)
8. [Execution Flow](#8-execution-flow)
9. [5 Verbs Implementation](#9-5-verbs-implementation)
10. [Agent Loop Implementation](#10-agent-loop-implementation)
11. [Parallelism Model](#11-parallelism-model)
12. [TUI Architecture](#12-tui-architecture)
13. [Test Architecture](#13-test-architecture)

---

## 1. Overview

Nika is a semantic YAML workflow engine that executes multi-step AI workflows. It serves as the "Body" in the NovaNet (Brain) + Nika (Body) architecture, consuming knowledge from NovaNet via MCP protocol.

```
+-----------------------------------------------------------------------------+
|                         NIKA ENGINE ARCHITECTURE                            |
+-----------------------------------------------------------------------------+
|                                                                             |
|  +--------------+    +--------------+    +--------------+                   |
|  |    YAML      |    |     AST      |    |     DAG      |                   |
|  |  Workflow    |--->|   Parsing    |--->|  Validation  |                   |
|  |    File      |    |  (serde)     |    |  (Kahn's)    |                   |
|  +--------------+    +--------------+    +--------------+                   |
|                                                 |                           |
|                                                 v                           |
|  +--------------+    +--------------+    +--------------+                   |
|  |   Results    |<---|   Runtime    |<---|   Executor   |                   |
|  |   + Traces   |    |   Runner     |    |  (5 verbs)   |                   |
|  +--------------+    +--------------+    +--------------+                   |
|         |                   |                   |                           |
|         |                   |                   |                           |
|         v                   v                   v                           |
|  +--------------+    +--------------+    +--------------+                   |
|  |  EventLog    |    | RigProvider  |    |  McpClient   |                   |
|  |  (16 kinds)  |    | (rig-core)   |    |  (rmcp)      |                   |
|  +--------------+    +--------------+    +--------------+                   |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### Core Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `rig-core` | 0.31 | LLM providers (Claude, OpenAI) |
| `rmcp` | 0.16 | MCP protocol client |
| `tokio` | 1.44 | Async runtime |
| `serde_yaml` | 0.9 | YAML parsing |
| `ratatui` | 0.29 | Terminal UI |
| `crossterm` | 0.28 | Terminal control |

---

## 2. AST Module Architecture

The AST (Abstract Syntax Tree) module transforms YAML workflow files into strongly-typed Rust structures.

### 2.1 YAML Parsing Pipeline

```
+-----------------------------------------------------------------------------+
|                        YAML PARSING PIPELINE                                |
+-----------------------------------------------------------------------------+
|                                                                             |
|  workflow.yaml                                                              |
|       |                                                                     |
|       v                                                                     |
|  +-------------+     serde_yaml::from_str()                                 |
|  | Raw YAML    |-------------------------------->+-------------+            |
|  | String      |                                 |  Workflow   |            |
|  +-------------+                                 |  (struct)   |            |
|                                                  +------+------+            |
|                                                        |                    |
|                      +-----------------------------+---+----------------+   |
|                      |                             |                    |   |
|                      v                             v                    v   |
|               +-------------+              +-------------+       +----------+
|               |    Task     |              |    Flow     |       |   Mcp    |
|               |  (struct)   |              |  (struct)   |       | (config) |
|               +------+------+              +-------------+       +----------+
|                      |                                                      |
|                      v                                                      |
|               +-------------+                                               |
|               | TaskAction  |                                               |
|               |   (enum)    |                                               |
|               +-------------+                                               |
|                      |                                                      |
|       +--------------+--------------+--------------+--------------+         |
|       v              v              v              v              v         |
|   +-------+    +-----------+  +-----------+  +-----------+  +-----------+   |
|   | Infer |    |   Exec    |  |   Fetch   |  |  Invoke   |  |   Agent   |   |
|   |Params |    |  Params   |  |  Params   |  |  Params   |  |  Params   |   |
|   +-------+    +-----------+  +-----------+  +-----------+  +-----------+   |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 2.2 Workflow Struct

**File:** `src/ast/workflow.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Schema version (e.g., "nika/workflow@0.3")
    pub schema: String,

    /// Unique workflow identifier
    pub workflow: String,

    /// Human-readable description
    #[serde(default)]
    pub description: String,

    /// List of tasks to execute
    pub tasks: Vec<Task>,

    /// Explicit flow dependencies (optional)
    #[serde(default)]
    pub flow: Vec<Flow>,

    /// MCP server configurations
    #[serde(default)]
    pub mcp: Option<McpSection>,

    /// Provider configuration (model, temperature)
    #[serde(default)]
    pub provider: Option<ProviderConfig>,
}
```

### 2.3 Task Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique task identifier within workflow
    pub id: String,

    /// Human-readable description
    #[serde(default)]
    pub description: Option<String>,

    /// The action to perform (exactly one of 5 verbs)
    #[serde(flatten)]
    pub action: TaskAction,

    /// Output bindings (use.ctx, use.result, etc.)
    #[serde(rename = "use", default)]
    pub use_block: Option<UseBlock>,

    /// Parallel iteration configuration
    #[serde(default)]
    pub for_each: Option<ForEachConfig>,

    /// Context assembly instructions
    #[serde(default)]
    pub context: Option<ContextSpec>,
}
```

### 2.4 TaskAction Enum (5 Variants)

**File:** `src/ast/action.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskAction {
    /// LLM text generation
    Infer(InferParams),

    /// Shell command execution
    Exec(ExecParams),

    /// HTTP request
    Fetch(FetchParams),

    /// MCP tool invocation
    Invoke(InvokeParams),

    /// Multi-turn agentic loop
    Agent(AgentParams),
}

impl TaskAction {
    /// Returns the verb name as a string
    pub fn verb(&self) -> &'static str {
        match self {
            TaskAction::Infer(_) => "infer",
            TaskAction::Exec(_) => "exec",
            TaskAction::Fetch(_) => "fetch",
            TaskAction::Invoke(_) => "invoke",
            TaskAction::Agent(_) => "agent",
        }
    }
}
```

### 2.5 AgentParams Struct

**File:** `src/ast/agent.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentParams {
    /// The goal or prompt for the agent
    pub prompt: String,

    /// MCP server names to use for tools
    #[serde(default)]
    pub mcp: Vec<String>,

    /// Maximum turns before stopping (default: 10)
    #[serde(default)]
    pub max_turns: Option<u32>,

    /// Provider override (claude, openai)
    #[serde(default)]
    pub provider: Option<String>,

    /// Model override
    #[serde(default)]
    pub model: Option<String>,

    /// Temperature override (0.0-1.0)
    #[serde(default)]
    pub temperature: Option<f32>,

    /// Enable extended thinking (Claude only)
    #[serde(default)]
    pub thinking: Option<ThinkingConfig>,

    /// System prompt override
    #[serde(default)]
    pub system: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingConfig {
    /// Budget tokens for thinking (default: 10000)
    #[serde(default = "default_budget")]
    pub budget_tokens: u32,
}
```

### 2.6 InvokeParams Struct

**File:** `src/ast/invoke.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeParams {
    /// MCP server name (must match mcp: config)
    pub mcp: String,

    /// Tool name to invoke
    pub tool: String,

    /// Tool parameters as JSON
    #[serde(default)]
    pub params: serde_json::Value,
}
```

### 2.7 Schema Validation

Workflow files must declare a schema version:

```yaml
schema: nika/workflow@0.3
```

The schema version determines available features:

| Version | Features |
|---------|----------|
| `@0.1` | infer, exec, fetch verbs |
| `@0.2` | +invoke, +agent verbs, +mcp config |
| `@0.3` | +for_each parallelism, rig-core integration |

---

## 3. DAG Module Architecture

The DAG (Directed Acyclic Graph) module validates workflow structure and determines execution order.

### 3.1 Dependency Resolution

**File:** `src/dag/mod.rs`, `src/dag/flow.rs`

```
+-----------------------------------------------------------------------------+
|                       DAG CONSTRUCTION PIPELINE                             |
+-----------------------------------------------------------------------------+
|                                                                             |
|  Workflow                                                                   |
|     |                                                                       |
|     +---> Extract explicit flows ----------------------------------------+  |
|     |    flow:                                                           |  |
|     |      - from: task1                                                 |  |
|     |        to: task2                                                   |  |
|     |                                                                    |  |
|     +---> Extract implicit flows ----------------------------------------+->|
|          (from use: bindings)                                            |  |
|          use:                                                            |  |
|            ctx: $task1.output <--- creates task1 -> current edge         |  |
|                                                                          |  |
|                                                                          v  |
|                                                              +-------------+|
|                                                              |   FlowSet   ||
|                                                              |  (edges)    ||
|                                                              +------+------+|
|                                                                     |       |
|                                                                     v       |
|  +------------------------------------------------------------------+----+ |
|  |                        VALIDATION CHECKS                              | |
|  +-----------------------------------------------------------------------+ |
|  |  1. All task IDs unique                                               | |
|  |  2. All flow references valid (from/to exist)                         | |
|  |  3. No cycles (Kahn's algorithm)                                      | |
|  |  4. All bindings resolvable                                           | |
|  +-----------------------------------------------------------------------+ |
|                                                                     |       |
|                                                                     v       |
|                                                              +-------------+|
|                                                              |   DagInfo   ||
|                                                              |  (sorted)   ||
|                                                              +-------------+|
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 3.2 Kahn's Algorithm for Topological Sort

```rust
/// Performs topological sort using Kahn's algorithm
/// Returns tasks in execution order or error if cycle detected
pub fn topological_sort(workflow: &Workflow) -> Result<Vec<&Task>, NikaError> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();

    // Initialize in-degree for all tasks
    for task in &workflow.tasks {
        in_degree.entry(&task.id).or_insert(0);
        adjacency.entry(&task.id).or_insert_with(Vec::new);
    }

    // Build graph from flows
    for flow in &workflow.flow {
        adjacency.get_mut(flow.from.as_str()).unwrap().push(&flow.to);
        *in_degree.get_mut(flow.to.as_str()).unwrap() += 1;
    }

    // Start with zero in-degree nodes
    let mut queue: VecDeque<&str> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(&id, _)| id)
        .collect();

    let mut sorted = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node);

        for &neighbor in adjacency.get(node).unwrap_or(&vec![]) {
            let deg = in_degree.get_mut(neighbor).unwrap();
            *deg -= 1;
            if *deg == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    // Cycle detection
    if sorted.len() != workflow.tasks.len() {
        return Err(NikaError::CycleDetected {
            tasks: find_cycle_participants(&in_degree),
        });
    }

    Ok(sorted)
}
```

### 3.3 Cycle Detection

```rust
#[derive(Debug, thiserror::Error)]
pub enum NikaError {
    #[error("[NIKA-020] Cycle detected in workflow DAG involving tasks: {tasks:?}")]
    CycleDetected { tasks: Vec<String> },

    #[error("[NIKA-021] Unknown task referenced in flow: {task_id}")]
    UnknownTaskInFlow { task_id: String },

    #[error("[NIKA-022] Duplicate task ID: {task_id}")]
    DuplicateTaskId { task_id: String },
}
```

### 3.4 Parallel Execution Groups

The DAG module identifies tasks that can run in parallel:

```rust
/// Groups tasks by execution level (tasks at same level can run in parallel)
pub fn parallel_groups(workflow: &Workflow) -> Vec<Vec<&Task>> {
    let sorted = topological_sort(workflow)?;
    let mut levels: HashMap<&str, usize> = HashMap::new();

    for task_id in &sorted {
        let max_dep_level = workflow.flow
            .iter()
            .filter(|f| f.to == *task_id)
            .map(|f| levels.get(f.from.as_str()).unwrap_or(&0))
            .max()
            .unwrap_or(&0);

        levels.insert(task_id, max_dep_level + 1);
    }

    // Group by level
    let max_level = levels.values().max().unwrap_or(&0);
    (1..=*max_level)
        .map(|level| {
            workflow.tasks
                .iter()
                .filter(|t| levels.get(t.id.as_str()) == Some(&level))
                .collect()
        })
        .collect()
}
```

---

## 4. Runtime Architecture

The runtime module orchestrates workflow execution, managing task dispatch, context assembly, and result collection.

### 4.1 Component Overview

```
+-----------------------------------------------------------------------------+
|                         RUNTIME ARCHITECTURE                                |
+-----------------------------------------------------------------------------+
|                                                                             |
|  +---------------------------------------------------------------------+    |
|  |                           RUNNER                                    |    |
|  |  (src/runtime/runner.rs)                                           |    |
|  |                                                                     |    |
|  |  Responsibilities:                                                  |    |
|  |  - Load and parse workflow YAML                                     |    |
|  |  - Initialize MCP connections                                       |    |
|  |  - Initialize LLM providers                                         |    |
|  |  - Orchestrate DAG execution                                        |    |
|  |  - Collect and format results                                       |    |
|  +----------------------------------+----------------------------------+    |
|                                     |                                       |
|                                     v                                       |
|  +---------------------------------------------------------------------+    |
|  |                          EXECUTOR                                   |    |
|  |  (src/runtime/executor.rs)                                         |    |
|  |                                                                     |    |
|  |  Responsibilities:                                                  |    |
|  |  - Dispatch tasks based on TaskAction variant                       |    |
|  |  - Handle for_each parallelism                                      |    |
|  |  - Assemble context from bindings                                   |    |
|  |  - Emit events to EventLog                                          |    |
|  +----------------------------------+----------------------------------+    |
|                                     |                                       |
|           +-------------------------+-------------------------+             |
|           |                         |                         |             |
|           v                         v                         v             |
|  +-----------------+    +-----------------+    +-----------------+          |
|  |   RigProvider   |    |    McpClient    |    |  RigAgentLoop   |          |
|  |  (infer verb)   |    |  (invoke verb)  |    |  (agent verb)   |          |
|  +-----------------+    +-----------------+    +-----------------+          |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 4.2 Executor Implementation

**File:** `src/runtime/executor.rs`

```rust
pub struct Executor {
    /// Provider for LLM operations
    provider: Arc<RigProvider>,

    /// MCP clients by server name
    mcp_clients: Arc<DashMap<String, McpClient>>,

    /// Event log for observability
    event_log: Arc<EventLog>,

    /// Data store for task outputs
    store: Arc<DataStore>,
}

impl Executor {
    /// Execute a single task based on its action type
    pub async fn execute_task(
        &self,
        task: &Task,
        context: &ExecutionContext,
    ) -> Result<TaskOutput, NikaError> {
        // Emit TaskStarted event
        self.event_log.push(EventKind::TaskStarted {
            task_id: task.id.clone(),
            verb: task.action.verb().to_string(),
            timestamp: Utc::now(),
        });

        // Handle for_each if present
        if let Some(for_each) = &task.for_each {
            return self.execute_for_each(task, for_each, context).await;
        }

        // Dispatch based on verb
        let result = match &task.action {
            TaskAction::Infer(params) => self.execute_infer(params, context).await,
            TaskAction::Exec(params) => self.execute_exec(params, context).await,
            TaskAction::Fetch(params) => self.execute_fetch(params, context).await,
            TaskAction::Invoke(params) => self.execute_invoke(params, context).await,
            TaskAction::Agent(params) => self.execute_agent(params, context).await,
        };

        // Emit completion event
        match &result {
            Ok(output) => {
                self.event_log.push(EventKind::TaskCompleted {
                    task_id: task.id.clone(),
                    duration_ms: output.duration_ms,
                    output_size: output.content.len(),
                });
            }
            Err(e) => {
                self.event_log.push(EventKind::TaskFailed {
                    task_id: task.id.clone(),
                    error: e.to_string(),
                });
            }
        }

        result
    }
}
```

### 4.3 Runner Implementation

**File:** `src/runtime/runner.rs`

```rust
pub struct Runner {
    /// Parsed workflow
    workflow: Workflow,

    /// Task executor
    executor: Executor,

    /// Event log
    event_log: Arc<EventLog>,

    /// Trace writer for NDJSON output
    trace_writer: Option<TraceWriter>,
}

impl Runner {
    /// Execute the entire workflow
    pub async fn run(&mut self) -> Result<WorkflowResult, NikaError> {
        // Emit WorkflowStarted
        self.event_log.push(EventKind::WorkflowStarted {
            workflow_id: self.workflow.workflow.clone(),
            task_count: self.workflow.tasks.len(),
            timestamp: Utc::now(),
        });

        // Get topological order
        let execution_order = topological_sort(&self.workflow)?;

        // Execute tasks in order
        let mut results = HashMap::new();
        for task_id in execution_order {
            let task = self.workflow.tasks
                .iter()
                .find(|t| t.id == task_id)
                .unwrap();

            // Build context from resolved bindings
            let context = self.build_context(task, &results)?;

            // Execute task
            let output = self.executor.execute_task(task, &context).await?;

            // Store result for downstream tasks
            results.insert(task.id.clone(), output);
        }

        // Emit WorkflowCompleted
        self.event_log.push(EventKind::WorkflowCompleted {
            workflow_id: self.workflow.workflow.clone(),
            success: true,
            total_duration_ms: calculate_duration(),
        });

        Ok(WorkflowResult { task_outputs: results })
    }
}
```

### 4.4 Context Assembly

**File:** `src/binding/mod.rs`

Context assembly resolves `use:` blocks and `{{template}}` syntax:

```rust
/// Resolves bindings and assembles execution context
pub fn assemble_context(
    task: &Task,
    store: &DataStore,
    workflow_context: &serde_json::Value,
) -> Result<ExecutionContext, NikaError> {
    let mut context = ExecutionContext::new();

    // Resolve use: block bindings
    if let Some(use_block) = &task.use_block {
        for (alias, binding) in use_block.bindings() {
            let value = resolve_binding(binding, store, workflow_context)?;
            context.set(alias, value);
        }
    }

    // Resolve context: specification
    if let Some(ctx_spec) = &task.context {
        let ctx_value = resolve_context_spec(ctx_spec, store)?;
        context.set_context(ctx_value);
    }

    Ok(context)
}

/// Resolves a single binding expression
fn resolve_binding(
    binding: &str,
    store: &DataStore,
    workflow_context: &serde_json::Value,
) -> Result<serde_json::Value, NikaError> {
    // $task_id.output -> fetch from store
    if binding.starts_with('$') {
        let parts: Vec<&str> = binding[1..].split('.').collect();
        let task_id = parts[0];
        let path = &parts[1..];

        let task_output = store.get(task_id)
            .ok_or(NikaError::BindingNotFound {
                binding: binding.to_string()
            })?;

        return extract_path(&task_output, path);
    }

    // {{use.alias}} -> template substitution handled later
    Ok(serde_json::Value::String(binding.to_string()))
}
```

---

## 5. Provider System v0.4

Nika v0.4 uses rig-core exclusively for LLM providers, removing the custom ClaudeProvider and OpenAIProvider.

### 5.1 RigProvider Architecture

**File:** `src/provider/rig.rs` (827 lines)

```
+-----------------------------------------------------------------------------+
|                        RIGPROVIDER ARCHITECTURE                             |
+-----------------------------------------------------------------------------+
|                                                                             |
|  +---------------------------------------------------------------------+    |
|  |                        RigProvider (enum)                           |    |
|  +---------------------------------------------------------------------+    |
|  |                                                                     |    |
|  |  +-----------------+              +-----------------+               |    |
|  |  | Claude variant  |              | OpenAI variant  |               |    |
|  |  |                 |              |                 |               |    |
|  |  | rig::providers  |              | rig::providers  |               |    |
|  |  | ::anthropic     |              | ::openai        |               |    |
|  |  | ::Client        |              | ::Client        |               |    |
|  |  +--------+--------+              +--------+--------+               |    |
|  |           |                                |                        |    |
|  |           +----------------+---------------+                        |    |
|  |                            |                                        |    |
|  |                            v                                        |    |
|  |           +------------------------+                                |    |
|  |           |   LlmProvider trait    |                                |    |
|  |           |                        |                                |    |
|  |           | - infer()              |                                |    |
|  |           | - chat()               |                                |    |
|  |           | - stream()             |                                |    |
|  |           +------------------------+                                |    |
|  |                                                                     |    |
|  +---------------------------------------------------------------------+    |
|                                                                             |
|  Environment Variables:                                                     |
|  +-- ANTHROPIC_API_KEY  -> RigProvider::claude()                           |
|  +-- OPENAI_API_KEY     -> RigProvider::openai()                           |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 5.2 RigProvider Implementation

```rust
/// Unified LLM provider using rig-core v0.31
#[derive(Clone)]
pub enum RigProvider {
    Claude(rig::providers::anthropic::Client),
    OpenAI(rig::providers::openai::Client),
}

impl RigProvider {
    /// Create Claude provider from environment
    pub fn claude() -> Result<Self, NikaError> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| NikaError::ProviderConfigError {
                provider: "claude".to_string(),
                reason: "ANTHROPIC_API_KEY not set".to_string(),
            })?;

        let client = rig::providers::anthropic::Client::new(&api_key);
        Ok(RigProvider::Claude(client))
    }

    /// Create OpenAI provider from environment
    pub fn openai() -> Result<Self, NikaError> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| NikaError::ProviderConfigError {
                provider: "openai".to_string(),
                reason: "OPENAI_API_KEY not set".to_string(),
            })?;

        let client = rig::providers::openai::Client::new(&api_key);
        Ok(RigProvider::OpenAI(client))
    }

    /// Simple text completion
    pub async fn infer(
        &self,
        prompt: &str,
        config: Option<InferConfig>,
    ) -> Result<String, NikaError> {
        let config = config.unwrap_or_default();

        match self {
            RigProvider::Claude(client) => {
                let model = client.completion_model(
                    &config.model_or("claude-sonnet-4-20250514")
                );
                let response = model
                    .temperature(config.temperature.unwrap_or(0.7))
                    .max_tokens(config.max_tokens.unwrap_or(4096))
                    .prompt(prompt)
                    .await
                    .map_err(|e| NikaError::ProviderError {
                        provider: "claude".to_string(),
                        reason: e.to_string(),
                    })?;
                Ok(response)
            }
            RigProvider::OpenAI(client) => {
                let model = client.completion_model(&config.model_or("gpt-4o"));
                let response = model
                    .temperature(config.temperature.unwrap_or(0.7))
                    .max_tokens(config.max_tokens.unwrap_or(4096))
                    .prompt(prompt)
                    .await
                    .map_err(|e| NikaError::ProviderError {
                        provider: "openai".to_string(),
                        reason: e.to_string(),
                    })?;
                Ok(response)
            }
        }
    }
}
```

### 5.3 LlmProvider Trait

```rust
/// Abstraction for LLM providers
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Simple text completion
    async fn infer(
        &self,
        prompt: &str,
        config: Option<InferConfig>
    ) -> Result<String, NikaError>;

    /// Multi-turn chat completion
    async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        config: Option<ChatConfig>,
    ) -> Result<ChatResponse, NikaError>;

    /// Streaming completion
    async fn stream(
        &self,
        prompt: &str,
        config: Option<InferConfig>,
    ) -> Result<impl Stream<Item = Result<String, NikaError>>, NikaError>;
}
```

### 5.4 Streaming Support

```rust
impl RigProvider {
    /// Stream completion with extended thinking support
    pub async fn stream_with_thinking(
        &self,
        prompt: &str,
        thinking_budget: u32,
    ) -> Result<impl Stream<Item = StreamEvent>, NikaError> {
        match self {
            RigProvider::Claude(client) => {
                let model = client
                    .completion_model("claude-sonnet-4-20250514")
                    .extended_thinking(thinking_budget);

                let stream = model
                    .stream(prompt)
                    .await
                    .map_err(|e| NikaError::StreamError {
                        reason: e.to_string()
                    })?;

                Ok(stream.map(|chunk| {
                    match chunk {
                        Ok(rig::completion::StreamChunk::Thinking(t)) => {
                            StreamEvent::Thinking(t)
                        }
                        Ok(rig::completion::StreamChunk::Content(c)) => {
                            StreamEvent::Content(c)
                        }
                        Ok(rig::completion::StreamChunk::Final(f)) => {
                            StreamEvent::Final {
                                input_tokens: f.usage.input_tokens,
                                output_tokens: f.usage.output_tokens,
                            }
                        }
                        Err(e) => StreamEvent::Error(e.to_string()),
                    }
                }))
            }
            _ => Err(NikaError::UnsupportedFeature {
                feature: "extended_thinking".to_string(),
                provider: "openai".to_string(),
            }),
        }
    }
}
```

---

## 6. MCP Client Architecture

The MCP (Model Context Protocol) client enables Nika to call tools exposed by MCP servers like NovaNet.

### 6.1 McpClient Overview

**File:** `src/mcp/client.rs` (718 lines)

```
+-----------------------------------------------------------------------------+
|                        MCP CLIENT ARCHITECTURE                              |
+-----------------------------------------------------------------------------+
|                                                                             |
|  +---------------------------------------------------------------------+    |
|  |                         McpClient                                   |    |
|  +---------------------------------------------------------------------+    |
|  |                                                                     |    |
|  |  +-----------------+         +-----------------+                    |    |
|  |  |   Real Mode     |         |   Mock Mode     |                    |    |
|  |  |                 |         |                 |                    |    |
|  |  | RmcpClient      |         | MockMcpClient   |                    |    |
|  |  | (rmcp v0.16)    |         | (testing)       |                    |    |
|  |  +--------+--------+         +--------+--------+                    |    |
|  |           |                           |                             |    |
|  |           +-----------+---------------+                             |    |
|  |                       |                                             |    |
|  |                       v                                             |    |
|  |  +-------------------------------------------------------------+   |    |
|  |  |                    McpClientInner (enum)                    |   |    |
|  |  |                                                             |   |    |
|  |  |  - Real(RmcpClientAdapter)                                  |   |    |
|  |  |  - Mock(MockMcpClient)                                      |   |    |
|  |  |                                                             |   |    |
|  |  |  Methods:                                                   |   |    |
|  |  |  - connect() -> spawn server process                       |   |    |
|  |  |  - list_tools() -> Vec<ToolInfo>                            |   |    |
|  |  |  - call_tool(name, params) -> ToolResult                    |   |    |
|  |  |  - read_resource(uri) -> ResourceContent                    |   |    |
|  |  |  - disconnect() -> cleanup                                  |   |    |
|  |  +-------------------------------------------------------------+   |    |
|  |                                                                     |    |
|  +---------------------------------------------------------------------+    |
|                                                                             |
|  Connection Flow:                                                           |
|  +---------+   spawn    +-------------+   stdio    +-------------+          |
|  |  Nika   |----------->| MCP Server  |<---------->|   rmcp      |          |
|  |         |            |  (NovaNet)  |            |  transport  |          |
|  +---------+            +-------------+            +-------------+          |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 6.2 McpClient Implementation

```rust
/// MCP client supporting real and mock modes
pub struct McpClient {
    /// Server name from workflow mcp: config
    name: String,

    /// Connection state
    inner: RwLock<Option<McpClientInner>>,

    /// Cached tool list
    tools: OnceCell<Vec<ToolInfo>>,

    /// Server configuration
    config: McpServerConfig,
}

enum McpClientInner {
    Real(RmcpClientAdapter),
    Mock(MockMcpClient),
}

impl McpClient {
    /// Connect to MCP server by spawning process
    pub async fn connect(&self) -> Result<(), NikaError> {
        let inner = match &self.config.mode {
            McpMode::Real => {
                // Spawn server process
                let mut cmd = Command::new(&self.config.command);
                if let Some(args) = &self.config.args {
                    cmd.args(args);
                }
                cmd.stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit());

                let child = cmd.spawn()
                    .map_err(|e| NikaError::McpSpawnError {
                        server: self.name.clone(),
                        reason: e.to_string(),
                    })?;

                // Create rmcp client over stdio
                let transport = StdioTransport::new(
                    child.stdin.take().unwrap(),
                    child.stdout.take().unwrap(),
                );

                let client = rmcp::Client::new(transport)
                    .await
                    .map_err(|e| NikaError::McpConnectionError {
                        server: self.name.clone(),
                        reason: e.to_string(),
                    })?;

                McpClientInner::Real(RmcpClientAdapter::new(client, child))
            }
            McpMode::Mock => {
                McpClientInner::Mock(MockMcpClient::new(&self.name))
            }
        };

        *self.inner.write() = Some(inner);
        Ok(())
    }

    /// Call an MCP tool
    pub async fn call_tool(
        &self,
        tool_name: &str,
        params: serde_json::Value,
    ) -> Result<ToolResult, NikaError> {
        let inner = self.inner.read();
        let inner = inner.as_ref()
            .ok_or(NikaError::McpNotConnected {
                name: self.name.clone()
            })?;

        match inner {
            McpClientInner::Real(client) => {
                client.call_tool(tool_name, params).await
            }
            McpClientInner::Mock(mock) => {
                mock.call_tool(tool_name, params).await
            }
        }
    }

    /// List available tools
    pub async fn list_tools(&self) -> Result<Vec<ToolInfo>, NikaError> {
        // Use cached value if available
        if let Some(tools) = self.tools.get() {
            return Ok(tools.clone());
        }

        let inner = self.inner.read();
        let inner = inner.as_ref()
            .ok_or(NikaError::McpNotConnected {
                name: self.name.clone()
            })?;

        let tools = match inner {
            McpClientInner::Real(client) => client.list_tools().await?,
            McpClientInner::Mock(mock) => mock.list_tools().await?,
        };

        let _ = self.tools.set(tools.clone());
        Ok(tools)
    }
}
```

### 6.3 NikaMcpTool (rig-core Integration)

**File:** `src/provider/rig.rs`

To use MCP tools with rig-core's agent builder, we implement the `ToolDyn` trait:

```rust
/// Wraps an MCP tool for use with rig-core's AgentBuilder
pub struct NikaMcpTool {
    /// Tool definition from MCP server
    definition: NikaMcpToolDef,

    /// MCP client reference
    client: Arc<McpClient>,
}

/// Tool definition compatible with rig-core
#[derive(Debug, Clone)]
pub struct NikaMcpToolDef {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

impl rig::tool::ToolDyn for NikaMcpTool {
    fn name(&self) -> &str {
        &self.definition.name
    }

    fn description(&self) -> &str {
        &self.definition.description
    }

    fn parameters(&self) -> serde_json::Value {
        self.definition.input_schema.clone()
    }

    async fn call(
        &self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, rig::tool::ToolError> {
        let result = self.client
            .call_tool(&self.definition.name, params)
            .await
            .map_err(|e| rig::tool::ToolError::ExecutionError(
                e.to_string()
            ))?;

        Ok(result.content)
    }
}
```

### 6.4 Server Spawning and Lifecycle

```
+-----------------------------------------------------------------------------+
|                     MCP SERVER LIFECYCLE                                    |
+-----------------------------------------------------------------------------+
|                                                                             |
|  1. SPAWN                                                                   |
|     Workflow config:                                                        |
|     mcp:                                                                    |
|       servers:                                                              |
|         novanet:                                                            |
|           command: "cargo"                                                  |
|           args: ["run", "--manifest-path", "novanet-mcp/Cargo.toml"]        |
|                                                                             |
|     +---------+  spawn    +-------------+                                   |
|     |  Nika   |---------->| Child Proc  |                                   |
|     +---------+           +------+------+                                   |
|                                  |                                          |
|  2. HANDSHAKE (MCP Protocol)     |                                          |
|     +---------+  initialize +----+----+                                     |
|     |  rmcp   |------------>| NovaNet |                                     |
|     | client  |<------------|   MCP   |                                     |
|     +---------+  tools/list +---------+                                     |
|                                                                             |
|  3. TOOL CALLS                                                              |
|     +---------+  tools/call +---------+  cypher   +---------+               |
|     |  Nika   |------------>| NovaNet |---------->|  Neo4j  |               |
|     |         |<------------|   MCP   |<----------|         |               |
|     +---------+   result    +---------+   data    +---------+               |
|                                                                             |
|  4. SHUTDOWN                                                                |
|     +---------+  SIGTERM   +---------+                                      |
|     |  Nika   |----------->| NovaNet |                                      |
|     |         |            |  (exit) |                                      |
|     +---------+            +---------+                                      |
|                                                                             |
+-----------------------------------------------------------------------------+
```

---

## 7. Event System

The event system provides observability through structured event logging and NDJSON trace files.

### 7.1 EventLog Architecture

**File:** `src/event/log.rs` (1085 lines)

```
+-----------------------------------------------------------------------------+
|                        EVENT SYSTEM ARCHITECTURE                            |
+-----------------------------------------------------------------------------+
|                                                                             |
|  +---------------------------------------------------------------------+    |
|  |                          EventLog                                   |    |
|  +---------------------------------------------------------------------+    |
|  |                                                                     |    |
|  |  events: RwLock<Vec<Event>>    <- Thread-safe event storage         |    |
|  |  tx: broadcast::Sender         <- Real-time streaming to TUI        |    |
|  |                                                                     |    |
|  |  Methods:                                                           |    |
|  |  - push(EventKind)            <- Add event, broadcast to subscribers|    |
|  |  - subscribe() -> Receiver    <- TUI subscribes for real-time       |    |
|  |  - iter() -> impl Iterator    <- Iterate over events                |    |
|  |  - to_ndjson() -> String      <- Export as NDJSON                   |    |
|  |                                                                     |    |
|  +---------------------------------------------------------------------+    |
|                                                                             |
|  Event Flow:                                                                |
|  +----------+  push()   +----------+  broadcast  +----------+               |
|  | Executor |---------->| EventLog |------------>|   TUI    |               |
|  +----------+           +----+-----+             +----------+               |
|                              |                                              |
|                              | to_ndjson()                                  |
|                              v                                              |
|                       +--------------+                                      |
|                       | TraceWriter  |                                      |
|                       | (.ndjson)    |                                      |
|                       +--------------+                                      |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 7.2 EventKind Enum (16+ Variants)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventKind {
    // =========================================================================
    // WORKFLOW LIFECYCLE
    // =========================================================================

    /// Workflow execution started
    WorkflowStarted {
        workflow_id: String,
        task_count: usize,
        timestamp: DateTime<Utc>,
    },

    /// Workflow execution completed
    WorkflowCompleted {
        workflow_id: String,
        success: bool,
        total_duration_ms: u64,
    },

    // =========================================================================
    // TASK LIFECYCLE
    // =========================================================================

    /// Task execution started
    TaskStarted {
        task_id: String,
        verb: String,
        timestamp: DateTime<Utc>,
    },

    /// Task execution completed successfully
    TaskCompleted {
        task_id: String,
        duration_ms: u64,
        output_size: usize,
    },

    /// Task execution failed
    TaskFailed {
        task_id: String,
        error: String,
        recoverable: bool,
    },

    // =========================================================================
    // PROVIDER EVENTS
    // =========================================================================

    /// LLM provider called
    ProviderCalled {
        provider: String,
        model: String,
        prompt_hash: String,
        token_estimate: Option<u32>,
    },

    /// LLM provider responded
    ProviderResponded {
        provider: String,
        model: String,
        input_tokens: u32,
        output_tokens: u32,
        duration_ms: u64,
    },

    // =========================================================================
    // MCP EVENTS
    // =========================================================================

    /// MCP tool invocation started
    McpInvoke {
        server: String,
        tool: String,
        params_hash: String,
    },

    /// MCP tool returned result
    McpResponse {
        server: String,
        tool: String,
        success: bool,
        duration_ms: u64,
    },

    // =========================================================================
    // AGENT EVENTS
    // =========================================================================

    /// Agent loop started
    AgentStart {
        task_id: String,
        goal: String,
        max_turns: u32,
        tools: Vec<String>,
    },

    /// Single agent turn completed
    AgentTurn {
        task_id: String,
        turn: u32,
        tool_calls: Vec<ToolCallInfo>,
        response_preview: String,
        metadata: Option<AgentTurnMetadata>,
    },

    /// Agent loop completed
    AgentComplete {
        task_id: String,
        total_turns: u32,
        final_response: String,
        total_tokens: TokenUsage,
    },

    // =========================================================================
    // CONTEXT EVENTS
    // =========================================================================

    /// Context assembled from bindings
    ContextAssembled {
        task_id: String,
        bindings_resolved: usize,
        context_size_bytes: usize,
    },

    /// Template resolved
    TemplateResolved {
        task_id: String,
        template: String,
        resolved: String,
    },

    // =========================================================================
    // FOR_EACH EVENTS
    // =========================================================================

    /// Parallel iteration started
    ForEachStarted {
        task_id: String,
        item_count: usize,
        concurrency: usize,
    },

    /// Single iteration completed
    ForEachItemCompleted {
        task_id: String,
        index: usize,
        duration_ms: u64,
    },
}
```

### 7.3 AgentTurnMetadata

```rust
/// Metadata captured during agent turn (v0.4.1+)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentTurnMetadata {
    /// Claude's extended thinking content (if enabled)
    pub thinking: Option<String>,

    /// Input tokens for this turn
    pub input_tokens: u32,

    /// Output tokens for this turn
    pub output_tokens: u32,

    /// Stop reason (end_turn, tool_use, max_tokens)
    pub stop_reason: Option<String>,

    /// Model used for this turn
    pub model: Option<String>,
}
```

### 7.4 TraceWriter

**File:** `src/event/trace.rs`

```rust
/// Writes events to NDJSON trace files
pub struct TraceWriter {
    /// Output file path
    path: PathBuf,

    /// File handle
    file: BufWriter<File>,

    /// Trace ID
    trace_id: String,
}

impl TraceWriter {
    /// Create new trace writer
    pub fn new(workflow_id: &str) -> Result<Self, NikaError> {
        let trace_id = format!("{}_{}",
            workflow_id,
            Utc::now().format("%Y%m%d_%H%M%S")
        );

        let path = PathBuf::from(".nika/traces")
            .join(format!("{}.ndjson", trace_id));

        fs::create_dir_all(path.parent().unwrap())?;
        let file = BufWriter::new(File::create(&path)?);

        Ok(Self { path, file, trace_id })
    }

    /// Write event to trace file
    pub fn write_event(&mut self, event: &Event) -> Result<(), NikaError> {
        let line = serde_json::to_string(event)?;
        writeln!(self.file, "{}", line)?;
        self.file.flush()?;
        Ok(())
    }
}
```

### 7.5 NDJSON Format

Each line in the trace file is a complete JSON object:

```json
{"type":"workflow_started","workflow_id":"generate-pages","task_count":5,"timestamp":"2026-02-19T14:30:00Z"}
{"type":"task_started","task_id":"fetch_entity","verb":"invoke","timestamp":"2026-02-19T14:30:01Z"}
{"type":"mcp_invoke","server":"novanet","tool":"novanet_generate","params_hash":"a1b2c3"}
{"type":"mcp_response","server":"novanet","tool":"novanet_generate","success":true,"duration_ms":245}
{"type":"task_completed","task_id":"fetch_entity","duration_ms":250,"output_size":1024}
{"type":"agent_turn","task_id":"generate","turn":1,"tool_calls":[{"name":"novanet_describe"}],"response_preview":"..."}
{"type":"workflow_completed","workflow_id":"generate-pages","success":true,"total_duration_ms":5432}
```

---

## 8. Execution Flow

Complete execution flow from YAML file to results.

### 8.1 Full Pipeline Diagram

```
+-----------------------------------------------------------------------------+
|                       COMPLETE EXECUTION FLOW                               |
+-----------------------------------------------------------------------------+
|                                                                             |
|  1. CLI ENTRY                                                               |
|     $ cargo run -- run workflow.nika.yaml                                   |
|           |                                                                 |
|           v                                                                 |
|  +-----------------+                                                        |
|  |    main.rs      |  Parse CLI arguments                                   |
|  |    Clap CLI     |  Load workflow file                                    |
|  +--------+--------+                                                        |
|           |                                                                 |
|  2. YAML PARSING                                                            |
|           |                                                                 |
|           v                                                                 |
|  +-----------------+                                                        |
|  |  serde_yaml     |  Deserialize to Workflow struct                        |
|  |  Workflow       |  Validate schema version                               |
|  +--------+--------+                                                        |
|           |                                                                 |
|  3. DAG CONSTRUCTION                                                        |
|           |                                                                 |
|           v                                                                 |
|  +-----------------+                                                        |
|  |  dag/flow.rs    |  Build adjacency graph                                 |
|  |  Kahn's algo    |  Topological sort                                      |
|  |                 |  Cycle detection                                       |
|  +--------+--------+                                                        |
|           |                                                                 |
|  4. INITIALIZATION                                                          |
|           |                                                                 |
|           +----------------------+----------------------+                   |
|           v                      v                      v                   |
|  +-----------------+   +-----------------+   +-----------------+            |
|  |  RigProvider    |   |   McpClient     |   |   EventLog      |            |
|  |  (from env)     |   |  (spawn/mock)   |   |   TraceWriter   |            |
|  +--------+--------+   +--------+--------+   +--------+--------+            |
|           |                     |                     |                     |
|           +---------------------+---------------------+                     |
|                                 |                                           |
|  5. TASK EXECUTION LOOP                                                     |
|                                 |                                           |
|                                 v                                           |
|  +---------------------------------------------------------------------+    |
|  |  FOR each task IN topological_order:                                |    |
|  |                                                                     |    |
|  |    +-----------------+                                              |    |
|  |    | Context Assembly|  Resolve use: bindings                       |    |
|  |    | binding/mod.rs  |  Substitute {{templates}}                    |    |
|  |    +--------+--------+                                              |    |
|  |             |                                                       |    |
|  |             v                                                       |    |
|  |    +-----------------+                                              |    |
|  |    | Task Dispatch   |  Match on TaskAction variant                 |    |
|  |    | executor.rs     |                                              |    |
|  |    +--------+--------+                                              |    |
|  |             |                                                       |    |
|  |    +--------+--------+----------+----------+----------+             |    |
|  |    v                 v          v          v          v             |    |
|  |  +-----+        +-----+    +-----+    +------+    +-----+           |    |
|  |  |infer|        |exec |    |fetch|    |invoke|    |agent|           |    |
|  |  |     |        |     |    |     |    |      |    |     |           |    |
|  |  | LLM |        |Shell|    |HTTP |    | MCP  |    |Loop |           |    |
|  |  +--+--+        +--+--+    +--+--+    +--+---+    +--+--+           |    |
|  |     |              |          |          |          |               |    |
|  |     +--------------+----------+----------+----------+               |    |
|  |                               |                                     |    |
|  |                               v                                     |    |
|  |    +-----------------+                                              |    |
|  |    | Store Output    |  Save to DataStore for downstream tasks      |    |
|  |    | store/mod.rs    |                                              |    |
|  |    +-----------------+                                              |    |
|  |                                                                     |    |
|  +---------------------------------------------------------------------+    |
|                                 |                                           |
|  6. RESULTS                     |                                           |
|                                 v                                           |
|  +---------------------------------------------------------------------+    |
|  |  WorkflowResult                                                     |    |
|  |  +-- task_outputs: HashMap<TaskId, TaskOutput>                      |    |
|  |  +-- trace_path: .nika/traces/workflow_20260219_143000.ndjson       |    |
|  |  +-- metrics: WorkflowMetrics (duration, tokens, etc.)              |    |
|  +---------------------------------------------------------------------+    |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 8.2 Example Workflow Execution

Given this workflow:

```yaml
schema: nika/workflow@0.3
workflow: generate-landing-page
mcp:
  servers:
    novanet:
      command: "cargo"
      args: ["run", "--manifest-path", "../novanet-dev/tools/novanet-mcp/Cargo.toml"]

tasks:
  - id: fetch_entity
    invoke:
      mcp: novanet
      tool: novanet_generate
      params:
        entity: "qr-code"
        locale: "fr-FR"
        forms: ["text", "title", "description"]
    use:
      ctx: entity_context

  - id: generate_hero
    infer: |
      Generate a hero section for {{entity_context.title}}.
      Context: {{entity_context.description}}
    use:
      result: hero_html

  - id: save_output
    exec: "echo '{{hero_html}}' > output/hero.html"

flow:
  - from: fetch_entity
    to: generate_hero
  - from: generate_hero
    to: save_output
```

Execution sequence:

```
T=0ms    WorkflowStarted(generate-landing-page, 3 tasks)
T=1ms    MCP connect to novanet (spawn cargo process)
T=50ms   TaskStarted(fetch_entity, invoke)
T=51ms   McpInvoke(novanet, novanet_generate)
T=300ms  McpResponse(novanet, success, 249ms)
T=301ms  TaskCompleted(fetch_entity, 251ms)
T=302ms  ContextAssembled(generate_hero, 1 binding)
T=303ms  TaskStarted(generate_hero, infer)
T=304ms  ProviderCalled(claude, claude-sonnet-4-20250514)
T=1500ms ProviderResponded(claude, 150 in, 450 out)
T=1501ms TaskCompleted(generate_hero, 1198ms)
T=1502ms TaskStarted(save_output, exec)
T=1510ms TaskCompleted(save_output, 8ms)
T=1511ms WorkflowCompleted(generate-landing-page, success, 1511ms)
```

---

## 9. 5 Verbs Implementation

Each verb has distinct execution semantics and error handling.

### 9.1 infer: - LLM Text Generation

```rust
async fn execute_infer(
    &self,
    params: &InferParams,
    context: &ExecutionContext,
) -> Result<TaskOutput, NikaError> {
    // Resolve template variables in prompt
    let prompt = resolve_template(&params.prompt, context)?;

    // Get provider configuration
    let config = InferConfig {
        model: params.model.clone(),
        temperature: params.temperature,
        max_tokens: params.max_tokens,
        system: params.system.clone(),
    };

    // Emit event
    self.event_log.push(EventKind::ProviderCalled {
        provider: self.provider.name().to_string(),
        model: config.model.clone().unwrap_or_default(),
        prompt_hash: hash_prompt(&prompt),
        token_estimate: Some(estimate_tokens(&prompt)),
    });

    // Call provider
    let start = Instant::now();
    let response = self.provider.infer(&prompt, Some(config)).await?;
    let duration = start.elapsed();

    // Emit response event
    self.event_log.push(EventKind::ProviderResponded {
        provider: self.provider.name().to_string(),
        model: config.model.clone().unwrap_or_default(),
        input_tokens: estimate_tokens(&prompt),
        output_tokens: estimate_tokens(&response),
        duration_ms: duration.as_millis() as u64,
    });

    Ok(TaskOutput {
        content: serde_json::Value::String(response),
        duration_ms: duration.as_millis() as u64,
    })
}
```

### 9.2 exec: - Shell Command

```rust
async fn execute_exec(
    &self,
    params: &ExecParams,
    context: &ExecutionContext,
) -> Result<TaskOutput, NikaError> {
    // Resolve template variables in command
    let command = resolve_template(&params.command, context)?;

    // Execute via tokio Command
    let output = tokio::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .current_dir(params.cwd.as_deref().unwrap_or("."))
        .envs(params.env.iter().flatten())
        .output()
        .await
        .map_err(|e| NikaError::ExecError {
            command: command.clone(),
            reason: e.to_string(),
        })?;

    // Check exit code
    if !output.status.success() && !params.ignore_errors.unwrap_or(false) {
        return Err(NikaError::ExecFailed {
            command,
            exit_code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(TaskOutput {
        content: serde_json::Value::String(stdout),
        duration_ms: 0,
    })
}
```

### 9.3 fetch: - HTTP Request

```rust
async fn execute_fetch(
    &self,
    params: &FetchParams,
    context: &ExecutionContext,
) -> Result<TaskOutput, NikaError> {
    // Resolve URL template
    let url = resolve_template(&params.url, context)?;

    // Build request
    let client = reqwest::Client::new();
    let mut request = match params.method.as_deref().unwrap_or("GET") {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        _ => return Err(NikaError::InvalidHttpMethod {
            method: params.method.clone()
        }),
    };

    // Add headers
    if let Some(headers) = &params.headers {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    // Add body
    if let Some(body) = &params.body {
        let body = resolve_template(&serde_json::to_string(body)?, context)?;
        request = request.body(body);
    }

    // Execute with timeout
    let timeout = Duration::from_secs(params.timeout.unwrap_or(30) as u64);
    let response = request
        .timeout(timeout)
        .send()
        .await
        .map_err(|e| NikaError::FetchError {
            url: url.clone(),
            reason: e.to_string(),
        })?;

    // Parse response
    let status = response.status();
    let body = response.text().await?;

    // Check status
    if !status.is_success() && !params.ignore_errors.unwrap_or(false) {
        return Err(NikaError::FetchFailed {
            url,
            status: status.as_u16(),
            body,
        });
    }

    // Parse as JSON if possible
    let content = serde_json::from_str(&body)
        .unwrap_or(serde_json::Value::String(body));

    Ok(TaskOutput {
        content,
        duration_ms: 0,
    })
}
```

### 9.4 invoke: - MCP Tool Call

```rust
async fn execute_invoke(
    &self,
    params: &InvokeParams,
    context: &ExecutionContext,
) -> Result<TaskOutput, NikaError> {
    // Get MCP client
    let client = self.mcp_clients
        .get(&params.mcp)
        .ok_or(NikaError::McpServerNotFound {
            name: params.mcp.clone()
        })?;

    // Resolve params template
    let tool_params = resolve_value_templates(&params.params, context)?;

    // Emit event
    self.event_log.push(EventKind::McpInvoke {
        server: params.mcp.clone(),
        tool: params.tool.clone(),
        params_hash: hash_value(&tool_params),
    });

    // Call tool
    let start = Instant::now();
    let result = client.call_tool(&params.tool, tool_params).await?;
    let duration = start.elapsed();

    // Emit response
    self.event_log.push(EventKind::McpResponse {
        server: params.mcp.clone(),
        tool: params.tool.clone(),
        success: result.is_success,
        duration_ms: duration.as_millis() as u64,
    });

    if !result.is_success {
        return Err(NikaError::McpToolFailed {
            server: params.mcp.clone(),
            tool: params.tool.clone(),
            error: result.error.unwrap_or_default(),
        });
    }

    Ok(TaskOutput {
        content: result.content,
        duration_ms: duration.as_millis() as u64,
    })
}
```

### 9.5 agent: - Multi-Turn Agentic Loop

```rust
async fn execute_agent(
    &self,
    params: &AgentParams,
    context: &ExecutionContext,
) -> Result<TaskOutput, NikaError> {
    // Build list of available tools from MCP servers
    let mut tools: Vec<Arc<dyn ToolDyn>> = Vec::new();

    for server_name in &params.mcp {
        let client = self.mcp_clients
            .get(server_name)
            .ok_or(NikaError::McpServerNotFound {
                name: server_name.clone()
            })?;

        let tool_infos = client.list_tools().await?;

        for info in tool_infos {
            tools.push(Arc::new(NikaMcpTool::new(
                NikaMcpToolDef {
                    name: info.name,
                    description: info.description,
                    input_schema: info.input_schema,
                },
                client.clone(),
            )));
        }
    }

    // Create agent loop
    let agent_loop = RigAgentLoop::new(
        context.task_id.clone(),
        params.clone(),
        self.event_log.clone(),
        tools,
    )?;

    // Run agent
    let result = if params.thinking.is_some() {
        agent_loop.run_claude_with_thinking().await?
    } else {
        agent_loop.run_claude().await?
    };

    Ok(TaskOutput {
        content: serde_json::Value::String(result.final_response),
        duration_ms: result.total_duration_ms,
    })
}
```

---

## 10. Agent Loop Implementation

The RigAgentLoop implements multi-turn agent execution using rig-core's AgentBuilder.

### 10.1 Agent Loop State Machine

```
+-----------------------------------------------------------------------------+
|                      AGENT LOOP STATE MACHINE                               |
+-----------------------------------------------------------------------------+
|                                                                             |
|                         +--------------+                                    |
|                         |    START     |                                    |
|                         +------+-------+                                    |
|                                |                                            |
|                                v                                            |
|  +---------------------------------------------------------------------+    |
|  |                         INITIALIZE                                  |    |
|  |  - Build tool list from MCP servers                                 |    |
|  |  - Create rig::AgentBuilder with tools                              |    |
|  |  - Set system prompt, temperature, max_tokens                       |    |
|  |  - Initialize turn counter                                          |    |
|  +----------------------------------+----------------------------------+    |
|                                     |                                       |
|                                     v                                       |
|  +---------------------------------------------------------------------+    |
|  |                       AGENT TURN LOOP                               |    |
|  |                                                                     |    |
|  |    +--------------+                                                 |    |
|  |    | Send Prompt  |<-----------------------------------------+      |    |
|  |    | to LLM       |                                          |      |    |
|  |    +------+-------+                                          |      |    |
|  |           |                                                  |      |    |
|  |           v                                                  |      |    |
|  |    +--------------+                                          |      |    |
|  |    | Parse LLM    |                                          |      |    |
|  |    | Response     |                                          |      |    |
|  |    +------+-------+                                          |      |    |
|  |           |                                                  |      |    |
|  |           +-------------------+------------------+           |      |    |
|  |           |                   |                  |           |      |    |
|  |           v                   v                  v           |      |    |
|  |    +------------+     +------------+     +------------+      |      |    |
|  |    | end_turn   |     | tool_use   |     | max_tokens |      |      |    |
|  |    +-----+------+     +-----+------+     +-----+------+      |      |    |
|  |          |                  |                  |             |      |    |
|  |          |                  v                  |             |      |    |
|  |          |           +------------+            |             |      |    |
|  |          |           |Execute Tool|            |             |      |    |
|  |          |           |via MCP     |            |             |      |    |
|  |          |           +-----+------+            |             |      |    |
|  |          |                 |                   |             |      |    |
|  |          |                 v                   |             |      |    |
|  |          |           +------------+            |             |      |    |
|  |          |           |Append Tool |            |             |      |    |
|  |          |           |Result      |            |             |      |    |
|  |          |           +-----+------+            |             |      |    |
|  |          |                 |                   |             |      |    |
|  |          |                 |  turn++           |             |      |    |
|  |          |                 |                   |             |      |    |
|  |          |                 |  turn < max?      |             |      |    |
|  |          |                 |                   |             |      |    |
|  |          |           +-----+-----+             |             |      |    |
|  |          |           |   YES     | NO          |             |      |    |
|  |          |           |           v             |             |      |    |
|  |          |           |    +------------+       |             |      |    |
|  |          |           |    | max_turns  |       |             |      |    |
|  |          |           |    | exceeded   |       |             |      |    |
|  |          |           |    +-----+------+       |             |      |    |
|  |          |           |          |              |             |      |    |
|  |          |           +----------+--------------+-------------+      |    |
|  |          |                                                          |    |
|  +----------+----------------------------------------------------------+    |
|             |                                                               |
|             v                                                               |
|  +---------------------------------------------------------------------+    |
|  |                         COMPLETE                                    |    |
|  |  - Emit AgentComplete event                                         |    |
|  |  - Return final response                                            |    |
|  |  - Report total tokens, turns                                       |    |
|  +---------------------------------------------------------------------+    |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 10.2 RigAgentLoop Implementation

**File:** `src/runtime/rig_agent_loop.rs`

```rust
/// Multi-turn agent execution using rig-core
pub struct RigAgentLoop {
    /// Task identifier
    task_id: String,

    /// Agent parameters from YAML
    params: AgentParams,

    /// Event log for observability
    event_log: Arc<EventLog>,

    /// Available tools (wrapped MCP tools)
    tools: Vec<Arc<dyn ToolDyn>>,
}

impl RigAgentLoop {
    /// Run agent with Claude provider
    pub async fn run_claude(&self) -> Result<AgentResult, NikaError> {
        let provider = RigProvider::claude()?;

        // Build agent with rig-core
        let agent = match &provider {
            RigProvider::Claude(client) => {
                let model = self.params.model.as_deref()
                    .unwrap_or("claude-sonnet-4-20250514");

                let mut builder = client
                    .agent(model)
                    .preamble(&self.params.system.clone().unwrap_or_default())
                    .temperature(self.params.temperature.unwrap_or(0.7));

                // Add tools
                for tool in &self.tools {
                    builder = builder.tool_dyn(tool.clone());
                }

                builder.build()
            }
            _ => return Err(NikaError::InvalidProvider {
                name: "claude".to_string()
            }),
        };

        // Emit start event
        self.event_log.push(EventKind::AgentStart {
            task_id: self.task_id.clone(),
            goal: self.params.prompt.clone(),
            max_turns: self.params.max_turns.unwrap_or(10),
            tools: self.tools.iter().map(|t| t.name().to_string()).collect(),
        });

        // Run conversation loop
        let mut turn = 0;
        let max_turns = self.params.max_turns.unwrap_or(10);
        let mut total_tokens = TokenUsage::default();
        let mut final_response = String::new();

        let mut conversation = agent.chat(&self.params.prompt).await
            .map_err(|e| NikaError::AgentError {
                task_id: self.task_id.clone(),
                reason: e.to_string(),
            })?;

        loop {
            turn += 1;

            if turn > max_turns {
                break;
            }

            // Get next response
            let response = conversation.next().await
                .map_err(|e| NikaError::AgentError {
                    task_id: self.task_id.clone(),
                    reason: e.to_string(),
                })?;

            // Track tokens
            total_tokens.input_tokens += response.usage.input_tokens;
            total_tokens.output_tokens += response.usage.output_tokens;

            // Emit turn event
            self.event_log.push(EventKind::AgentTurn {
                task_id: self.task_id.clone(),
                turn,
                tool_calls: response.tool_calls.iter().map(|tc| {
                    ToolCallInfo {
                        name: tc.name.clone(),
                        params_preview: tc.params.to_string()
                            .chars().take(100).collect(),
                    }
                }).collect(),
                response_preview: response.content.chars().take(200).collect(),
                metadata: Some(AgentTurnMetadata {
                    thinking: None,
                    input_tokens: response.usage.input_tokens,
                    output_tokens: response.usage.output_tokens,
                    stop_reason: Some(response.stop_reason.clone()),
                    model: Some(response.model.clone()),
                }),
            });

            // Check stop condition
            match response.stop_reason.as_str() {
                "end_turn" => {
                    final_response = response.content.clone();
                    break;
                }
                "tool_use" => {
                    // Tools are handled automatically by rig-core
                    continue;
                }
                "max_tokens" => {
                    final_response = response.content.clone();
                    break;
                }
                _ => continue,
            }
        }

        // Emit complete event
        self.event_log.push(EventKind::AgentComplete {
            task_id: self.task_id.clone(),
            total_turns: turn,
            final_response: final_response.chars().take(500).collect(),
            total_tokens: total_tokens.clone(),
        });

        Ok(AgentResult {
            final_response,
            total_turns: turn,
            total_tokens,
            total_duration_ms: 0,
        })
    }
}
```

### 10.3 Tool Calling Protocol

```
+-----------------------------------------------------------------------------+
|                      TOOL CALLING SEQUENCE                                  |
+-----------------------------------------------------------------------------+
|                                                                             |
|  1. LLM DECIDES TO USE TOOL                                                 |
|     +---------+                                                             |
|     |  LLM    |  "I need to query the knowledge graph"                      |
|     |         |  stop_reason: "tool_use"                                    |
|     |         |  tool_calls: [{name: "novanet_generate", params: {...}}]    |
|     +----+----+                                                             |
|          |                                                                  |
|  2. RIG-CORE INTERCEPTS                                                     |
|          |                                                                  |
|          v                                                                  |
|     +---------------------------------------------+                         |
|     |  rig::agent::Agent                          |                         |
|     |  - Parses tool_calls from response          |                         |
|     |  - Looks up tool by name in tool registry   |                         |
|     |  - Calls tool.call(params)                  |                         |
|     +---------------------+-----------------------+                         |
|                           |                                                 |
|  3. NIKAMCPTOOL EXECUTES                                                    |
|                           |                                                 |
|                           v                                                 |
|     +---------------------------------------------+                         |
|     |  NikaMcpTool.call(params)                   |                         |
|     |  - Forwards to McpClient.call_tool()        |                         |
|     |  - Waits for MCP response                   |                         |
|     |  - Returns JSON result                      |                         |
|     +---------------------+-----------------------+                         |
|                           |                                                 |
|  4. MCP CLIENT CALLS SERVER                                                 |
|                           |                                                 |
|                           v                                                 |
|     +---------------------------------------------+                         |
|     |  McpClient -> rmcp -> NovaNet MCP Server    |                         |
|     |  - JSON-RPC over stdio                      |                         |
|     |  - tools/call method                        |                         |
|     |  - Returns tool result                      |                         |
|     +---------------------+-----------------------+                         |
|                           |                                                 |
|  5. RESULT FLOWS BACK                                                       |
|                           |                                                 |
|                           v                                                 |
|     +---------------------------------------------+                         |
|     |  rig::agent::Agent                          |                         |
|     |  - Appends tool result to conversation      |                         |
|     |  - Sends updated conversation to LLM        |                         |
|     |  - LLM continues with tool result context   |                         |
|     +---------------------------------------------+                         |
|                                                                             |
+-----------------------------------------------------------------------------+
```

---

## 11. Parallelism Model

Nika supports parallel task execution via the `for_each` construct.

### 11.1 for_each Configuration

```yaml
tasks:
  - id: generate_pages
    for_each:
      items: $locales           # Array to iterate
      as: locale                # Loop variable name
      concurrency: 5            # Max parallel tasks
      fail_fast: true           # Stop on first error
    invoke:
      mcp: novanet
      tool: novanet_generate
      params:
        entity: "qr-code"
        locale: "{{locale}}"
```

### 11.2 Parallel Execution Architecture

```
+-----------------------------------------------------------------------------+
|                     FOR_EACH PARALLEL EXECUTION                             |
+-----------------------------------------------------------------------------+
|                                                                             |
|  Input: items = ["fr-FR", "de-DE", "es-ES", "it-IT", "pt-BR"]               |
|  Config: concurrency = 3, fail_fast = true                                  |
|                                                                             |
|  +---------------------------------------------------------------------+    |
|  |                        SEMAPHORE (permits: 3)                       |    |
|  +---------------------------------------------------------------------+    |
|                                                                             |
|  Time -------------------------------------------------------------->       |
|                                                                             |
|  Slot 1: |######## fr-FR ########|######## it-IT ########|                  |
|  Slot 2: |######## de-DE ########|######## pt-BR ########|                  |
|  Slot 3: |######## es-ES ########|                                          |
|                                                                             |
|  +---------------------------------------------------------------------+    |
|  |                         JoinSet                                     |    |
|  |  - Spawns tokio tasks for each item                                 |    |
|  |  - Semaphore limits concurrent tasks                                |    |
|  |  - Collects results in original order                               |    |
|  |  - Aborts remaining on error (if fail_fast)                         |    |
|  +---------------------------------------------------------------------+    |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 11.3 Implementation

```rust
async fn execute_for_each(
    &self,
    task: &Task,
    for_each: &ForEachConfig,
    context: &ExecutionContext,
) -> Result<TaskOutput, NikaError> {
    // Resolve items array
    let items = resolve_binding(&for_each.items, &self.store, context)?;
    let items = items.as_array()
        .ok_or(NikaError::ForEachItemsNotArray {
            task_id: task.id.clone()
        })?;

    let concurrency = for_each.concurrency.unwrap_or(1);
    let fail_fast = for_each.fail_fast.unwrap_or(true);

    // Emit start event
    self.event_log.push(EventKind::ForEachStarted {
        task_id: task.id.clone(),
        item_count: items.len(),
        concurrency,
    });

    // Create semaphore for concurrency limiting
    let semaphore = Arc::new(Semaphore::new(concurrency));

    // Spawn tasks
    let mut join_set = JoinSet::new();
    let results = Arc::new(Mutex::new(Vec::with_capacity(items.len())));

    for (index, item) in items.iter().enumerate() {
        let permit = semaphore.clone().acquire_owned().await?;
        let task = task.clone();
        let item = item.clone();
        let var_name = for_each.as_name.clone();
        let context = context.clone();
        let executor = self.clone();
        let results = results.clone();
        let event_log = self.event_log.clone();

        join_set.spawn(async move {
            // Create iteration context with loop variable
            let mut iter_context = context.clone();
            iter_context.set(&var_name, item);

            let start = Instant::now();
            let result = executor.execute_task_inner(&task, &iter_context).await;
            let duration = start.elapsed();

            // Emit item completed event
            event_log.push(EventKind::ForEachItemCompleted {
                task_id: task.id.clone(),
                index,
                duration_ms: duration.as_millis() as u64,
            });

            // Release permit
            drop(permit);

            // Store result with index
            let mut results = results.lock().await;
            results.push((index, result));

            Ok::<_, NikaError>(())
        });
    }

    // Wait for all tasks
    if fail_fast {
        while let Some(result) = join_set.join_next().await {
            result??; // Propagate first error
        }
    } else {
        join_set.join_all().await;
    }

    // Sort results by original index
    let mut results = Arc::try_unwrap(results)
        .map_err(|_| NikaError::InternalError {
            reason: "results still referenced".into()
        })?
        .into_inner();
    results.sort_by_key(|(idx, _)| *idx);

    // Collect outputs
    let outputs: Vec<serde_json::Value> = results
        .into_iter()
        .map(|(_, r)| r.map(|o| o.content))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TaskOutput {
        content: serde_json::Value::Array(outputs),
        duration_ms: 0,
    })
}
```

---

## 12. TUI Architecture

The Terminal UI provides real-time workflow execution monitoring with a 4-panel layout.

### 12.1 Panel Layout

**File:** `src/tui/app.rs` (542 lines)

```
+-----------------------------------------------------------------------------+
|                        TUI 4-PANEL LAYOUT                                   |
+-----------------------------------------------------------------------------+
|                                                                             |
|  +---------------------------------+-----------------------------------+    |
|  |         PROGRESS PANEL          |           DAG PANEL               |    |
|  |         (top-left)              |          (top-right)              |    |
|  |                                 |                                   |    |
|  |  * Task 1: fetch_entity [DONE]  |     +-----------+                 |    |
|  |  * Task 2: generate     [RUN]   |     |   task1   |                 |    |
|  |  o Task 3: save         [WAIT]  |     +-----+-----+                 |    |
|  |                                 |           |                       |    |
|  |  Progress: 2/3                  |           v                       |    |
|  |  [========------] 66%           |     +-----------+                 |    |
|  |                                 |     |   task2   | <-- current     |    |
|  |                                 |     +-----+-----+                 |    |
|  |                                 |           |                       |    |
|  |                                 |           v                       |    |
|  |                                 |     +-----------+                 |    |
|  |                                 |     |   task3   |                 |    |
|  |                                 |     +-----------+                 |    |
|  +---------------------------------+-----------------------------------+    |
|  |      NOVANET/CONTEXT PANEL      |       AGENT/REASONING PANEL      |    |
|  |         (bottom-left)           |        (bottom-right)            |    |
|  |                                 |                                   |    |
|  |  MCP: novanet (connected)       |  Turn 2 of 10                    |    |
|  |                                 |                                   |    |
|  |  Last call: novanet_generate    |  Thinking:                       |    |
|  |  Duration: 245ms                |  "I need to query the entity     |    |
|  |                                 |   to get the French content..."  |    |
|  |  Context:                       |                                   |    |
|  |  {                              |  Tool calls:                     |    |
|  |    "entity": "qr-code",         |  - novanet_generate (245ms)      |    |
|  |    "locale": "fr-FR",           |  - novanet_describe (120ms)      |    |
|  |    "forms": {                   |                                   |    |
|  |      "text": "QR Code",         |  Tokens: 1,245 in / 892 out      |    |
|  |      "title": "Le QR Code"      |                                   |    |
|  |    }                            |                                   |    |
|  |  }                              |                                   |    |
|  |                                 |                                   |    |
|  +---------------------------------+-----------------------------------+    |
|                                                                             |
|  +---------------------------------------------------------------------+    |
|  | STATUS BAR                                                          |    |
|  | [q] Quit  [p] Pause  [s] Step  [Up/Dn] Scroll  | 60 FPS | 5 ev/s  |    |
|  +---------------------------------------------------------------------+    |
|                                                                             |
+-----------------------------------------------------------------------------+
```

### 12.2 TUI State Machine

```rust
/// TUI application state
pub struct App {
    /// Current execution state
    state: AppState,

    /// Workflow being executed
    workflow: Workflow,

    /// Task execution status
    task_status: HashMap<String, TaskStatus>,

    /// Event receiver from executor
    event_rx: broadcast::Receiver<Event>,

    /// Current panel focus
    focus: PanelFocus,

    /// Scroll positions per panel
    scroll: HashMap<PanelFocus, usize>,

    /// Last N events for display
    recent_events: VecDeque<Event>,

    /// Agent turn history
    agent_turns: Vec<AgentTurnInfo>,

    /// MCP connection status
    mcp_status: HashMap<String, McpStatus>,

    /// Current context (from last invoke/agent)
    current_context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    /// Initial state before execution
    Ready,

    /// Workflow is running
    Running,

    /// Execution paused (step mode)
    Paused,

    /// Workflow completed successfully
    Completed,

    /// Workflow failed with error
    Failed(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelFocus {
    Progress,
    Dag,
    Context,
    Agent,
}
```

### 12.3 Event Streaming

```rust
impl App {
    /// Main event loop at 60 FPS
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<impl Backend>
    ) -> Result<(), NikaError> {
        let tick_rate = Duration::from_millis(16); // ~60 FPS
        let mut last_tick = Instant::now();

        loop {
            // Draw current state
            terminal.draw(|frame| self.draw(frame))?;

            // Poll for events
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = crossterm::event::read()? {
                    match self.handle_key(key) {
                        Some(Action::Quit) => break,
                        Some(Action::Pause) => self.toggle_pause(),
                        Some(Action::Step) => self.step(),
                        Some(Action::ScrollUp) => self.scroll_up(),
                        Some(Action::ScrollDown) => self.scroll_down(),
                        Some(Action::SwitchPanel(p)) => self.focus = p,
                        None => {}
                    }
                }
            }

            // Process workflow events
            while let Ok(event) = self.event_rx.try_recv() {
                self.process_event(event);
            }

            // Check for completion
            if matches!(self.state, AppState::Completed | AppState::Failed(_)) {
                // Wait for user to acknowledge
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    /// Process workflow event and update UI state
    fn process_event(&mut self, event: Event) {
        self.recent_events.push_front(event.clone());
        self.recent_events.truncate(100); // Keep last 100

        match &event.kind {
            EventKind::TaskStarted { task_id, .. } => {
                self.task_status.insert(
                    task_id.clone(),
                    TaskStatus::Running
                );
            }
            EventKind::TaskCompleted { task_id, .. } => {
                self.task_status.insert(
                    task_id.clone(),
                    TaskStatus::Completed
                );
            }
            EventKind::TaskFailed { task_id, error, .. } => {
                self.task_status.insert(
                    task_id.clone(),
                    TaskStatus::Failed(error.clone())
                );
            }
            EventKind::AgentTurn { turn, metadata, .. } => {
                self.agent_turns.push(AgentTurnInfo {
                    turn: *turn,
                    thinking: metadata.as_ref()
                        .and_then(|m| m.thinking.clone()),
                    tokens: metadata.as_ref()
                        .map(|m| (m.input_tokens, m.output_tokens)),
                });
            }
            EventKind::McpResponse { server, success, .. } => {
                self.mcp_status.insert(server.clone(), if *success {
                    McpStatus::Connected
                } else {
                    McpStatus::Error
                });
            }
            EventKind::WorkflowCompleted { success, .. } => {
                self.state = if *success {
                    AppState::Completed
                } else {
                    AppState::Failed("Workflow failed".into())
                };
            }
            _ => {}
        }
    }
}
```

### 12.4 Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` | Quit TUI |
| `p` | Pause/Resume execution |
| `s` | Step (execute next task when paused) |
| `Up`/`k` | Scroll up in focused panel |
| `Dn`/`j` | Scroll down in focused panel |
| `1` | Focus Progress panel |
| `2` | Focus DAG panel |
| `3` | Focus Context panel |
| `4` | Focus Agent panel |
| `Tab` | Cycle panel focus |

---

## 13. Test Architecture

Nika maintains 621+ tests across unit, integration, and property-based testing.

### 13.1 Test Organization

```
tools/nika/
+-- src/
|   +-- ast/
|   |   +-- workflow.rs      # Unit tests inline
|   |   +-- action.rs        # Unit tests inline
|   |   +-- ...
|   +-- dag/
|   |   +-- mod.rs           # DAG validation tests
|   +-- runtime/
|   |   +-- executor.rs      # Execution tests
|   |   +-- rig_agent_loop.rs # Agent loop tests
|   +-- ...
|
+-- tests/                    # Integration tests
|   +-- workflow_parsing_test.rs
|   +-- dag_validation_test.rs
|   +-- executor_test.rs
|   +-- mcp_client_test.rs
|   +-- rig_integration_test.rs
|   +-- thinking_capture_test.rs
|   +-- for_each_test.rs
|   +-- ...
|
+-- examples/                 # Example workflows (also tested)
    +-- simple-infer.nika.yaml
    +-- invoke-novanet.nika.yaml
    +-- agent-with-tools.nika.yaml
    +-- ...
```

### 13.2 Test Categories

| Category | Count | Location | Description |
|----------|-------|----------|-------------|
| Unit | ~400 | `src/**/*.rs` | In-file `#[cfg(test)]` modules |
| Integration | ~150 | `tests/*.rs` | Cross-module tests |
| Snapshot | ~50 | `tests/snapshots/` | insta YAML/JSON snapshots |
| Property | ~20 | `tests/proptest_*.rs` | proptest fuzzing |

### 13.3 Test Utilities

```rust
// tests/test_utils.rs

/// Create a mock MCP client for testing
pub fn mock_mcp_client(name: &str) -> McpClient {
    McpClient::mock(name, vec![
        MockTool::new("novanet_generate")
            .with_response(json!({
                "denomination_forms": {
                    "text": "QR Code",
                    "title": "Le QR Code"
                }
            })),
        MockTool::new("novanet_describe")
            .with_response(json!({
                "entity": "qr-code",
                "description": "A QR Code entity"
            })),
    ])
}

/// Create a test workflow with common setup
pub fn test_workflow(tasks: Vec<Task>) -> Workflow {
    Workflow {
        schema: "nika/workflow@0.3".to_string(),
        workflow: "test-workflow".to_string(),
        description: "Test workflow".to_string(),
        tasks,
        flow: vec![],
        mcp: None,
        provider: None,
    }
}

/// Assert task completed successfully
pub fn assert_task_completed(events: &[Event], task_id: &str) {
    let completed = events.iter().find(|e| {
        matches!(
            &e.kind,
            EventKind::TaskCompleted { task_id: id, .. } if id == task_id
        )
    });
    assert!(
        completed.is_some(),
        "Task {} should have completed",
        task_id
    );
}
```

### 13.4 Running Tests

```bash
# Run all tests
cargo nextest run

# Run with coverage
cargo llvm-cov nextest

# Run specific test file
cargo nextest run workflow_parsing

# Run tests matching pattern
cargo nextest run agent

# Run with verbose output
cargo nextest run --no-capture

# Update snapshots
cargo insta review
```

### 13.5 Test Coverage

Current coverage targets:
- **Overall:** 85%+
- **AST module:** 95%+
- **DAG module:** 90%+
- **Runtime:** 80%+
- **Provider:** 75%+
- **MCP client:** 85%+

---

## Appendix A: Error Codes

| Code Range | Category | Examples |
|------------|----------|----------|
| NIKA-000-009 | Workflow | ParseError, SchemaVersionError |
| NIKA-010-019 | Task | InvalidTask, DuplicateTaskId |
| NIKA-020-029 | DAG | CycleDetected, UnresolvedDependency |
| NIKA-030-039 | Provider | ApiKeyMissing, RateLimited |
| NIKA-040-049 | Binding | BindingNotFound, TemplateError |
| NIKA-100-109 | MCP | McpSpawnError, McpToolFailed |
| NIKA-110-119 | Agent | MaxTurnsExceeded, ToolNotFound |
| NIKA-130-139 | TUI | RenderError, TerminalError |

---

## Appendix B: Dependencies

```toml
[dependencies]
# Core
tokio = { version = "1.44", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0"
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"

# LLM
rig-core = { version = "0.31", features = ["rmcp"] }

# MCP
rmcp = "0.16"

# Async
futures = "0.3"
async-trait = "0.1"

# Data structures
dashmap = "6.0"
parking_lot = "0.12"
rustc-hash = "2.0"

# TUI (feature-gated)
ratatui = { version = "0.29", optional = true }
crossterm = { version = "0.28", optional = true }

# Utilities
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4"] }
reqwest = { version = "0.12", features = ["json"] }
regex = "1.10"

[dev-dependencies]
proptest = "1.4"
insta = { version = "1.34", features = ["yaml"] }
pretty_assertions = "1.4"
tokio-test = "0.4"
```

---

## Appendix C: File Index

| File | Lines | Purpose |
|------|-------|---------|
| `src/lib.rs` | ~100 | Public API exports |
| `src/main.rs` | ~200 | CLI entry point |
| `src/error.rs` | 556 | NikaError enum (40+ variants) |
| `src/ast/workflow.rs` | ~300 | Workflow struct |
| `src/ast/action.rs` | ~200 | TaskAction enum |
| `src/ast/agent.rs` | ~150 | AgentParams |
| `src/ast/invoke.rs` | ~100 | InvokeParams |
| `src/dag/mod.rs` | ~400 | DAG construction |
| `src/dag/flow.rs` | ~200 | Flow validation |
| `src/runtime/executor.rs` | ~600 | Task execution |
| `src/runtime/runner.rs` | ~400 | Workflow orchestration |
| `src/runtime/rig_agent_loop.rs` | ~500 | Agent loop |
| `src/provider/rig.rs` | 827 | RigProvider + NikaMcpTool |
| `src/mcp/client.rs` | 718 | McpClient |
| `src/event/log.rs` | 1085 | EventLog (16 variants) |
| `src/event/trace.rs` | ~150 | TraceWriter |
| `src/binding/mod.rs` | ~300 | Data binding |
| `src/tui/app.rs` | 542 | TUI state machine |

**Total:** ~7,000+ lines of Rust code

---

*Generated: 2026-02-19 | Nika v0.4.1 | 621+ tests passing*
