# Nika — Quick Overview

**Native Intelligence Kernel Agent**

## What Is It?

**Nika** = DAG workflow runner for AI tasks, written in Rust.

Execute YAML-defined workflows with LLM calls, shell commands, and HTTP requests — with automatic data flow between tasks.

## The Core Idea

```yaml
schema: "nika/workflow@0.1"
provider: claude

tasks:
  - id: weather
    infer:
      prompt: "Get Paris weather as JSON"
    output:
      format: json

  - id: recommend
    use:
      forecast: weather.summary     # ← Data flows from weather task
    infer:
      prompt: "Weather: {{use.forecast}}. Suggest activity."

flows:
  - source: weather
    target: recommend
```

**Result**: Tasks run in dependency order, data passes automatically.

## Key Features

| Feature | Description |
|---------|-------------|
| **3 Actions** | `infer:` (LLM), `exec:` (shell), `fetch:` (HTTP) |
| **DAG Execution** | Parallel when possible, sequential when needed |
| **Data Flow** | `use:` declares deps, `{{use.alias}}` substitutes |
| **Multi-Provider** | Claude, OpenAI, Mock |

## Quick Example

```bash
# Install
cargo install --path .

# Set API key
export ANTHROPIC_API_KEY=sk-ant-xxx

# Run workflow
nika run workflow.nika.yaml
```

## Actions

```yaml
# LLM inference
infer:
  prompt: "Your prompt here"
  provider: claude
  model: claude-sonnet-4-20250514

# Shell command
exec:
  command: "npm run build"

# HTTP request
fetch:
  url: "https://api.example.com"
  method: POST
  headers:
    Authorization: "Bearer {{use.token}}"
```

## Data Flow

```yaml
use:
  # Simple
  result: other_task

  # Nested path
  price: api_call.data.price

  # With default
  score: game.score ?? 0
```

Templates: `{{use.alias}}` in prompts, commands, URLs.

## DAG Patterns

```yaml
# Sequential
flows:
  - source: a
    target: b

# Fan-out (parallel)
flows:
  - source: start
    target: [a, b, c]

# Fan-in (aggregate)
flows:
  - source: [a, b, c]
    target: merge
```

## Architecture

```
YAML Workflow
     │
     ▼
┌─────────────────┐
│   DAG Builder   │  ← Validate dependencies
└─────────────────┘
     │
     ▼
┌─────────────────┐
│    Executor     │  ← Parallel task execution
└─────────────────┘
     │
     ├── infer → LLM Provider (Claude/OpenAI)
     ├── exec  → Shell
     └── fetch → HTTP Client
     │
     ▼
┌─────────────────┐
│   DataStore     │  ← Task results for downstream
└─────────────────┘
```

## Tech Stack

- **Language**: Rust 1.75+
- **Async**: Tokio
- **Parsing**: serde_yaml
- **HTTP**: reqwest
- **Concurrency**: dashmap, parking_lot

## Use Cases

1. **Code Review Automation** — git diff → LLM analysis
2. **API Data Pipelines** — fetch → process → save
3. **Content Generation** — outline → parallel writing → assembly
4. **DevOps Checks** — parallel system checks → aggregated report

---

*See NIKA-SPEC.md for full specification.*
