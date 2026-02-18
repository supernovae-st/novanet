# Nika — Specification & Architecture

**Native Intelligence Kernel Agent**

**Version**: 0.1.0 | **License**: AGPL-3.0 | **Language**: Rust

---

## Executive Summary

Nika is a **DAG workflow runner for AI tasks**. It executes YAML-defined workflows as directed acyclic graphs (DAGs), supporting LLM inference, shell commands, and HTTP requests with data flow between tasks.

```yaml
schema: "nika/workflow@0.1"
provider: claude

tasks:
  - id: greet
    infer:
      prompt: "Say hello in French"
```

```bash
nika run hello.nika.yaml
```

---

## Core Concept

**What Nika Does**: Orchestrate multi-step AI workflows with dependencies.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NIKA WORKFLOW EXECUTION                                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────┐     ┌─────────┐     ┌─────────┐                                │
│  │ weather │     │ flights │     │         │                                │
│  │ (infer) │     │ (fetch) │     │         │                                │
│  └────┬────┘     └────┬────┘     │         │                                │
│       │               │          │         │                                │
│       └───────┬───────┘          │         │                                │
│               ▼                  │         │                                │
│         ┌──────────┐             │         │                                │
│         │recommend │             │         │                                │
│         │ (infer)  │             │         │                                │
│         └──────────┘             │         │                                │
│                                                                             │
│  - Parallel execution when no dependencies                                  │
│  - Data flows between tasks via `use:` blocks                               │
│  - Templates substitute values: {{use.alias}}                               │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Features

| Feature | Description |
|---------|-------------|
| **3 Actions** | `infer:` (LLM) • `exec:` (shell) • `fetch:` (HTTP) |
| **DAG Execution** | Parallel processing when dependencies allow |
| **Data Flow** | `use:` blocks + `{{use.alias}}` templates |
| **Providers** | Claude, OpenAI, Mock |
| **Output Formats** | text, json (with JSON Schema validation) |
| **Error Codes** | Structured NIKA-XXX error codes |

---

## Workflow Structure

```yaml
schema: "nika/workflow@0.1"    # Required - must match exactly
provider: claude               # Default LLM provider
model: claude-sonnet-4-20250514  # Optional model override

tasks:
  - id: task_name              # snake_case identifier
    use:                       # Data dependencies
      alias: other_task.path
    infer:                     # Action (infer/exec/fetch)
      prompt: "..."
    output:                    # Output format
      format: json

flows:                         # DAG edges
  - source: task_a
    target: task_b
```

---

## Actions

### infer (LLM Call)

```yaml
infer:
  prompt: "Recommend a restaurant in {{use.city}}"
  provider: openai     # Optional override
  model: gpt-4o-mini   # Optional override
```

### exec (Shell Command)

```yaml
exec:
  command: "npm run build"
```

### fetch (HTTP Request)

```yaml
fetch:
  url: "https://api.example.com/data"
  method: POST           # Default: GET
  headers:
    Authorization: "Bearer {{use.token}}"
  body: '{"name": "{{use.name}}"}'
```

---

## Data Flow System

### Use Block

Declares data dependencies with path syntax:

```yaml
use:
  # Simple path
  forecast: weather.summary

  # Nested path
  price: flights.cheapest.price

  # Entire task output
  raw: weather

  # Array index
  first: results.items.0

  # With defaults
  score: game.score ?? 0
  name: user.name ?? "Anonymous"
```

### Templates

Syntax: `{{use.alias}}` or `{{use.alias.field}}`

```yaml
use:
  city: location.name
  temp: weather.temp
infer:
  prompt: |
    City: {{use.city}}
    Temperature: {{use.temp}}C
```

### Value Conversion

| Type | Output |
|------|--------|
| String | As-is |
| Number | `to_string()` |
| Boolean | `"true"` / `"false"` |
| Null | Error NIKA-072 |
| Object/Array | Compact JSON |

---

## Flow (DAG)

```yaml
flows:
  # Simple dependency
  - source: task_a
    target: task_b

  # Fan-out (parallel)
  - source: start
    target: [a, b, c]

  # Fan-in (aggregate)
  - source: [a, b]
    target: aggregate
```

### Execution Rules

1. Tasks with no dependencies run immediately (parallel)
2. Tasks wait for ALL upstream dependencies
3. Tasks only run if ALL dependencies succeeded

---

## Output Policy

```yaml
output:
  format: json                        # text (default) | json
  schema: .nika/schemas/result.json   # Optional JSON Schema
```

| Format | Stored As | Path Access |
|--------|-----------|-------------|
| `text` | `Value::String` | No |
| `json` | `Value::Object` | Yes |

**Rule:** Use `format: json` when downstream tasks need path access.

---

## Providers

| Provider | API Key Env | Models |
|----------|-------------|--------|
| `claude` | `ANTHROPIC_API_KEY` | claude-sonnet-4-*, claude-haiku-* |
| `openai` | `OPENAI_API_KEY` | gpt-4o, gpt-4o-mini |
| `mock` | - | (testing) |

---

## Code Architecture

```
┌─────────────────────────────────────────┐
│           DOMAIN MODEL (ast/)           │
│  Workflow, Task, TaskAction, Output     │
└─────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────┐
│         APPLICATION LAYER               │
│  runtime/  → Runner, TaskExecutor       │
│  dag/      → FlowGraph, validation      │
│  binding/  → UseWiring, templates       │
└─────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────┐
│        INFRASTRUCTURE LAYER             │
│  store/    → DataStore, TaskResult      │
│  event/    → EventLog, EventKind        │
│  provider/ → Claude, OpenAI, Mock       │
│  util/     → jsonpath, interner         │
└─────────────────────────────────────────┘
```

### Source Structure

```
src/
├── main.rs           # CLI entry point
├── lib.rs            # Library exports
├── error.rs          # NikaError enum (NIKA-XXX codes)
├── ast/              # Domain model
│   ├── workflow.rs   # Workflow, Task
│   ├── action.rs     # TaskAction (infer/exec/fetch)
│   └── output.rs     # OutputPolicy
├── dag/              # DAG validation
│   ├── flow.rs       # Flow, FlowEndpoint
│   └── validate.rs   # Cycle detection
├── runtime/          # Execution engine
│   ├── runner.rs     # Workflow runner
│   ├── executor.rs   # Task executor
│   └── output.rs     # Output processing
├── binding/          # Data flow
│   ├── entry.rs      # UseEntry, UseWiring
│   ├── template.rs   # {{use.alias}} substitution
│   ├── resolve.rs    # Path resolution
│   └── validate.rs   # Use block validation
├── store/            # Runtime state
│   └── datastore.rs  # TaskResult storage
├── provider/         # LLM providers
│   ├── claude.rs     # Anthropic API
│   └── openai.rs     # OpenAI API
├── event/            # Logging
│   └── log.rs        # EventLog, EventKind
└── util/             # Utilities
    ├── jsonpath.rs   # Path resolution
    ├── interner.rs   # String interning
    └── constants.rs  # Shared constants
```

---

## Key Types

```rust
// Workflow definition
pub struct Workflow {
    pub schema: String,
    pub provider: String,
    pub model: Option<String>,
    pub tasks: Vec<Arc<Task>>,
    pub flows: Vec<Flow>,
}

// Single task
pub struct Task {
    pub id: String,
    pub use_wiring: Option<UseWiring>,
    pub output: Option<OutputPolicy>,
    pub action: TaskAction,
}

// Task action (one of three)
pub enum TaskAction {
    Infer { infer: InferParams },
    Exec { exec: ExecParams },
    Fetch { fetch: FetchParams },
}

// DAG edge
pub struct Flow {
    pub source: FlowEndpoint,
    pub target: FlowEndpoint,
}

// Runtime result
pub struct TaskResult {
    pub output: Arc<Value>,
    pub duration: Duration,
    pub status: TaskStatus,
}
```

---

## Error Codes

### Schema (010)
| Code | Error | Fix |
|------|-------|-----|
| NIKA-010 | Invalid schema version | Use `"nika/workflow@0.1"` |

### Path (050-056)
| Code | Error | Fix |
|------|-------|-----|
| NIKA-050 | Invalid path syntax | Use `task.field.subfield` |
| NIKA-051 | Task not found | Verify task exists |
| NIKA-052 | Path not found | Add `?? default` |
| NIKA-055 | Invalid task ID | Use snake_case |

### Use Block (070-074)
| Code | Error | Fix |
|------|-------|-----|
| NIKA-070 | Duplicate alias | Use unique names |
| NIKA-071 | Unknown alias | Declare in `use:` block |
| NIKA-072 | Null value | Add `?? default` |
| NIKA-073 | Invalid traversal | Cannot access `.field` on primitive |

### DAG (080-082)
| Code | Error | Fix |
|------|-------|-----|
| NIKA-080 | Task not in DAG | Verify task exists |
| NIKA-081 | Task not upstream | Add flow or change source |
| NIKA-082 | Circular dependency | Remove cycle |

---

## Use Cases

### 1. Code Review Automation

```yaml
schema: "nika/workflow@0.1"
provider: claude

tasks:
  - id: get_diff
    exec:
      command: "git diff HEAD~1"

  - id: review
    use:
      diff: get_diff
    infer:
      prompt: |
        Review this code diff:
        ```diff
        {{use.diff}}
        ```

flows:
  - source: get_diff
    target: review
```

### 2. API Data Pipeline

```yaml
tasks:
  - id: fetch_users
    fetch:
      url: "https://jsonplaceholder.typicode.com/users"
    output:
      format: json

  - id: analyze
    use:
      users: fetch_users
    infer:
      prompt: "Analyze these users: {{use.users}}"
    output:
      format: json

  - id: save
    use:
      report: analyze
    exec:
      command: "echo '{{use.report}}' > report.json"

flows:
  - source: fetch_users
    target: analyze
  - source: analyze
    target: save
```

### 3. Content Generation Pipeline

```yaml
tasks:
  - id: outline
    infer:
      prompt: 'Create blog outline as JSON: {"title", "sections"}'
    output:
      format: json

  - id: write_intro
    use:
      title: outline.title
    infer:
      prompt: "Write intro for: {{use.title}}"

  - id: write_conclusion
    use:
      title: outline.title
    infer:
      prompt: "Write conclusion for: {{use.title}}"

  - id: assemble
    use:
      title: outline.title
      intro: write_intro
      conclusion: write_conclusion
    exec:
      command: |
        echo "# {{use.title}}"
        echo "{{use.intro}}"
        echo "{{use.conclusion}}"

flows:
  - source: outline
    target: [write_intro, write_conclusion]
  - source: [write_intro, write_conclusion]
    target: assemble
```

Diamond DAG: `outline` → parallel `write_intro` + `write_conclusion` → `assemble`

---

## Dependencies

| Crate | Purpose |
|-------|---------|
| clap | CLI argument parsing |
| tokio | Async runtime |
| serde | Serialization |
| serde_yaml | YAML parsing |
| reqwest | HTTP client |
| dashmap | Concurrent hashmap |
| thiserror | Error types |
| tracing | Logging |

---

## Commands

```bash
nika run <workflow.yaml>      # Execute workflow
nika validate <workflow.yaml> # Validate only
```

---

## Getting Started

```bash
# Install
cargo install --path .

# Set API key
export ANTHROPIC_API_KEY=your-key

# Run example
nika run examples/hello.yaml
```

---

*Nika v0.1.0 — SuperNovae Studio — AGPL-3.0*
