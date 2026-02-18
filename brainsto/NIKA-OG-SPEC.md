# Nika OG — Specification & Architecture

**"The Terraform for AI Agents"**

**Version**: 7.0 | **License**: MIT (Spec) / AGPL-3.0 (Implementation) | **Language**: Rust

---

## Executive Summary

Nika OG is a **production-grade AI workflow orchestration system** with explicit agent authorization, multi-provider support, and runtime advisory capabilities. It extends basic Nika with enterprise features for secure, observable AI agent deployment.

```yaml
schema: "nika/manifest@7.0"
name: enterprise-workflow
version: "1.0.0"

providers:
  claude:
    model: claude-sonnet-4-20250514

powers:
  - shell:read
  - mcp:filesystem
```

---

## Core Philosophy

**Terraform for AI Agents**: Just as Terraform provides declarative infrastructure-as-code, Nika OG provides declarative agent-orchestration-as-code.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  NIKA OG PRINCIPLES                                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  1. EXPLICIT AUTHORIZATION — No implicit capabilities. Powers are declared. │
│  2. PROVIDER AGNOSTIC — Claude, OpenAI, Ollama, Mistral. Same workflow.     │
│  3. OBSERVABLE EXECUTION — SHAKA sidecar provides runtime advisory.         │
│  4. LAYERED VALIDATION — 7 layers from schema to output verification.       │
│  5. COMPOSABLE WORKFLOWS — invoke: enables workflow reuse and nesting.      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Two-File Architecture

### Manifest: nika.yaml

Project-level configuration and authorization:

```yaml
schema: "nika/manifest@7.0"
name: my-project
version: "1.0.0"
description: "AI-powered code review system"

# Provider configurations
providers:
  claude:
    model: claude-sonnet-4-20250514
    max_tokens: 4096
  openai:
    model: gpt-4o
    temperature: 0.7
  ollama:
    model: llama3.2
    base_url: http://localhost:11434

# Capability authorization
powers:
  - shell:read
  - shell:write
  - shell:execute
  - mcp:filesystem
  - mcp:github
  - network:fetch

# Workflow references
workflows:
  - workflows/*.nika.yaml
  - shared/common.nika.yaml

# Environment
env:
  LOG_LEVEL: info
  TIMEOUT: 300
```

### Workflows: workflows/*.nika.yaml

Task definitions with DAG execution:

```yaml
schema: "nika/workflow@7.0"
id: code-review
description: "Automated code review pipeline"

tasks:
  - id: get_diff
    exec:
      command: "git diff HEAD~1"
    output:
      format: text

  - id: analyze
    use:
      diff: get_diff
    agent:
      prompt: |
        Review this code diff for:
        - Bugs and logic errors
        - Security vulnerabilities
        - Code style issues

        ```diff
        {{use.diff}}
        ```
      provider: claude
      scope: minimal
    output:
      format: json
      schema: review-schema.json

  - id: post_comment
    use:
      review: analyze
    fetch:
      url: "https://api.github.com/repos/{{env.REPO}}/pulls/{{env.PR}}/comments"
      method: POST
      headers:
        Authorization: "Bearer {{env.GITHUB_TOKEN}}"
      body: '{"body": "{{use.review.summary}}"}'

flows:
  - source: get_diff
    target: analyze
  - source: analyze
    target: post_comment
```

---

## 5 Semantic Verbs

### 1. agent: (AI Agent Orchestration)

Full agent capabilities with tools and MCP:

```yaml
agent:
  prompt: "Implement the feature described in {{use.spec}}"
  provider: claude
  scope: full                    # full | minimal | debug | default
  tools:
    - file_read
    - file_write
    - shell_execute
  mcp:
    - filesystem
    - github
  max_turns: 10                  # Agentic loop limit
  stop_conditions:
    - "TASK_COMPLETE"
    - "NEEDS_HUMAN_REVIEW"
```

### 2. exec: (Shell Execution)

Execute shell commands with safety controls:

```yaml
exec:
  command: "npm run build"
  working_dir: ./frontend
  timeout: 300                   # Seconds
  env:
    NODE_ENV: production
  on_error: continue             # continue | fail | retry
  retry:
    attempts: 3
    delay: 5
```

### 3. fetch: (HTTP Requests)

HTTP operations with response handling:

```yaml
fetch:
  url: "https://api.example.com/data"
  method: POST                   # GET | POST | PUT | DELETE | PATCH
  headers:
    Authorization: "Bearer {{use.token}}"
    Content-Type: application/json
  body: '{"query": "{{use.query}}"}'
  timeout: 30
  retry:
    attempts: 3
    backoff: exponential
```

### 4. invoke: (Sub-workflow Invocation)

Compose workflows by invoking others:

```yaml
invoke:
  workflow: shared/analyze.nika.yaml
  with:
    input: "{{use.data}}"
    mode: strict
  output:
    extract: result.analysis
```

### 5. infer: (Simple LLM Inference)

Lightweight LLM call without agent capabilities:

```yaml
infer:
  prompt: "Summarize this text: {{use.content}}"
  provider: claude
  model: claude-haiku-3-5-20241022    # Override manifest model
  temperature: 0.3
  max_tokens: 500
```

---

## 4 Scope Presets

| Scope | Tools | MCP | Logging | Use Case |
|-------|-------|-----|---------|----------|
| `full` | All declared | All declared | Standard | Complex autonomous tasks |
| `minimal` | None | None | Standard | Simple Q&A, summaries |
| `debug` | All declared | All declared | Verbose | Development, troubleshooting |
| `default` | Subset | Subset | Standard | General workflows |

### Scope Configuration

```yaml
scopes:
  default:
    tools:
      - file_read
      - shell_execute
    mcp:
      - filesystem
    logging: standard

  minimal:
    tools: []
    mcp: []
    logging: standard

  debug:
    tools: all
    mcp: all
    logging: verbose
    trace: true
```

---

## Powers & MCP Authorization

### Powers Categories

```yaml
powers:
  # Shell capabilities
  - shell:read           # Read files and directories
  - shell:write          # Create/modify files
  - shell:execute        # Run shell commands
  - shell:delete         # Delete files (high risk)

  # MCP servers
  - mcp:filesystem       # File system operations
  - mcp:github           # GitHub API access
  - mcp:database         # Database queries
  - mcp:browser          # Browser automation

  # Network
  - network:fetch        # HTTP requests
  - network:websocket    # WebSocket connections
  - network:dns          # DNS lookups

  # System
  - system:env           # Environment variables
  - system:process       # Process management
```

### MCP Server Configuration

```yaml
mcp:
  filesystem:
    roots:
      - ./src
      - ./docs
    allowed_operations:
      - read
      - write
      - list

  github:
    token: "{{env.GITHUB_TOKEN}}"
    repos:
      - owner/repo1
      - owner/repo2
    permissions:
      - read:code
      - write:issues
      - write:pull_requests
```

---

## SHAKA System

**SHAKA** = Runtime advisory sidecar that monitors and advises Nika execution.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  SHAKA ARCHITECTURE                                                         │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────┐                    ┌─────────────────┐                 │
│  │   NIKA CORE     │◄──Advisory Proto──►│     SHAKA       │                 │
│  │                 │                    │                 │                 │
│  │ • Task Executor │                    │ • Risk Analyzer │                 │
│  │ • DAG Scheduler │                    │ • Cost Monitor  │                 │
│  │ • Provider Pool │                    │ • Pattern Match │                 │
│  │ • Output Parser │                    │ • Suggestion AI │                 │
│  └─────────────────┘                    └─────────────────┘                 │
│                                                                             │
│  PRINCIPLE: "SHAKA proposes. NIKA disposes."                                │
│                                                                             │
│  • SHAKA NEVER blocks execution (advisory only)                             │
│  • SHAKA MAY suggest alternatives                                           │
│  • SHAKA TRACKS cost and resource usage                                     │
│  • SHAKA LOGS all decisions for audit                                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### SHAKA Configuration

```yaml
shaka:
  enabled: true
  mode: advisory                 # advisory | strict | disabled

  risk_analysis:
    enabled: true
    thresholds:
      shell_write: warn
      shell_delete: block
      network_external: warn

  cost_monitoring:
    enabled: true
    budget:
      daily: 10.00              # USD
      per_run: 1.00
    alerts:
      - threshold: 80%
        action: warn
      - threshold: 100%
        action: pause

  suggestions:
    enabled: true
    model: claude-haiku-3-5-20241022
```

---

## 7-Layer Validation

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  VALIDATION PIPELINE                                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Layer 1: SCHEMA VALIDATION                                                 │
│           └─ YAML structure, required fields, types                         │
│                                                                             │
│  Layer 2: SEMANTIC VALIDATION                                               │
│           └─ Task IDs unique, flow references valid                         │
│                                                                             │
│  Layer 3: RESOURCE VALIDATION                                               │
│           └─ Files exist, URLs reachable, providers configured              │
│                                                                             │
│  Layer 4: SECURITY VALIDATION                                               │
│           └─ Powers sufficient, secrets not exposed                         │
│                                                                             │
│  Layer 5: LOGIC VALIDATION                                                  │
│           └─ No cycles in DAG, dependencies satisfiable                     │
│                                                                             │
│  Layer 6: RUNTIME VALIDATION                                                │
│           └─ Provider connectivity, MCP server availability                 │
│                                                                             │
│  Layer 7: OUTPUT VALIDATION                                                 │
│           └─ JSON schema compliance, format verification                    │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Validation Configuration

```yaml
validation:
  strict: true
  layers:
    schema: required
    semantic: required
    resource: warn
    security: required
    logic: required
    runtime: warn
    output: required
```

---

## Multi-Provider Support

### Provider Configuration

```yaml
providers:
  claude:
    model: claude-sonnet-4-20250514
    max_tokens: 4096
    temperature: 0.7
    api_key: "{{env.ANTHROPIC_API_KEY}}"

  openai:
    model: gpt-4o
    max_tokens: 4096
    temperature: 0.7
    api_key: "{{env.OPENAI_API_KEY}}"

  ollama:
    model: llama3.2
    base_url: http://localhost:11434

  mistral:
    model: mistral-large
    api_key: "{{env.MISTRAL_API_KEY}}"
```

### Provider Selection

```yaml
tasks:
  - id: complex_task
    agent:
      prompt: "..."
      provider: claude           # Explicit selection

  - id: simple_task
    infer:
      prompt: "..."
      provider: ollama           # Local inference

  - id: fallback_task
    agent:
      prompt: "..."
      providers:                 # Fallback chain
        - claude
        - openai
        - ollama
```

---

## Data Flow System

### Use Block (Same as Basic Nika)

```yaml
use:
  # Simple reference
  result: other_task

  # Nested path
  summary: analysis.findings.summary

  # With defaults
  score: metrics.score ?? 0

  # Array access
  first_item: results.items.0
```

### Output Configuration

```yaml
output:
  format: json                   # text | json | yaml
  schema: schemas/output.json    # JSON Schema validation
  extract: data.result           # Extract nested field
  transform: |                   # jq-style transform
    .items | map(.name)
```

---

## Guards (Conditional Execution)

```yaml
tasks:
  - id: deploy
    guard:
      condition: "{{use.tests.passed}} == true"
      on_false: skip             # skip | fail | wait
    exec:
      command: "npm run deploy"

  - id: notify_failure
    guard:
      condition: "{{use.deploy.status}} == 'failed'"
    fetch:
      url: "{{env.SLACK_WEBHOOK}}"
      method: POST
      body: '{"text": "Deployment failed"}'
```

---

## Error Handling

```yaml
tasks:
  - id: risky_operation
    exec:
      command: "npm run build"
    on_error:
      action: retry              # retry | continue | fail | fallback
      retry:
        attempts: 3
        delay: 5
        backoff: exponential
      fallback:
        task: cleanup

  - id: cleanup
    exec:
      command: "npm run cleanup"
```

---

## Commands

```bash
# Project management
nika init <project-name>         # Create new project
nika validate                    # Run 7-layer validation
nika validate --strict           # Fail on warnings

# Workflow execution
nika run <workflow.nika.yaml>    # Execute workflow
nika run --dry-run               # Validate without executing
nika run --watch                 # Re-run on file changes
nika run --provider=ollama       # Override default provider

# SHAKA
nika shaka status                # SHAKA health check
nika shaka logs                  # View SHAKA logs
nika shaka cost                  # Cost report

# Debug
nika debug <workflow.nika.yaml>  # Run with verbose logging
nika trace <task-id>             # Trace single task
```

---

## Directory Structure

```
my-project/
├── nika.yaml                    # Manifest (required)
├── workflows/
│   ├── main.nika.yaml           # Primary workflow
│   ├── helpers.nika.yaml        # Helper tasks
│   └── deploy.nika.yaml         # Deployment workflow
├── schemas/
│   ├── review.json              # Output JSON schemas
│   └── report.json
├── shared/
│   └── common.nika.yaml         # Shared sub-workflows
└── .nika/
    ├── cache/                   # Execution cache
    └── logs/                    # SHAKA logs
```

---

## Comparison: Basic Nika vs Nika OG

| Feature | Basic Nika (v0.1) | Nika OG (v7.0) |
|---------|-------------------|----------------|
| **Verbs** | 3 (infer, exec, fetch) | 5 (+agent, invoke) |
| **Scopes** | None | 4 presets |
| **Architecture** | Single workflow file | Manifest + workflows |
| **Authorization** | Implicit | Explicit Powers & MCP |
| **Runtime** | Direct execution | SHAKA advisory sidecar |
| **Validation** | Basic YAML | 7-layer pipeline |
| **Providers** | 2 (Claude, OpenAI) | 4+ (Claude, OpenAI, Ollama, Mistral) |
| **Composition** | DAG only | DAG + invoke: for nesting |
| **Guards** | None | Conditional execution |
| **Error Handling** | Basic | Retry, fallback, recovery |

---

## Use Cases

### 1. CI/CD Pipeline Integration

```yaml
# ci-review.nika.yaml
tasks:
  - id: lint
    exec:
      command: "npm run lint"

  - id: test
    exec:
      command: "npm run test"

  - id: ai_review
    use:
      lint_output: lint
      test_output: test
    agent:
      prompt: |
        Review the CI results:
        Lint: {{use.lint_output}}
        Tests: {{use.test_output}}
      provider: claude
      scope: minimal
```

### 2. Documentation Generation

```yaml
# doc-gen.nika.yaml
tasks:
  - id: analyze_code
    exec:
      command: "find src -name '*.rs' -exec cat {} \\;"

  - id: generate_docs
    use:
      code: analyze_code
    agent:
      prompt: "Generate API documentation for: {{use.code}}"
      provider: claude
      scope: minimal
    output:
      format: text

  - id: save_docs
    use:
      docs: generate_docs
    exec:
      command: "echo '{{use.docs}}' > docs/API.md"
```

### 3. Multi-Agent Collaboration

```yaml
# multi-agent.nika.yaml
tasks:
  - id: architect
    agent:
      prompt: "Design system architecture for: {{input.requirements}}"
      provider: claude
      scope: full

  - id: implementer
    use:
      design: architect
    agent:
      prompt: "Implement the design: {{use.design}}"
      provider: claude
      scope: full
      tools:
        - file_write

  - id: reviewer
    use:
      implementation: implementer
    agent:
      prompt: "Review the implementation: {{use.implementation}}"
      provider: openai
      scope: minimal

flows:
  - source: architect
    target: implementer
  - source: implementer
    target: reviewer
```

---

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 1.75+ |
| Async | Tokio |
| Parsing | serde_yaml, schemars |
| HTTP | reqwest |
| CLI | clap |
| Providers | anthropic-sdk, async-openai, ollama-rs |
| Validation | jsonschema |

---

*Nika OG v7.0 — SuperNovae Studio — MIT (Spec) / AGPL-3.0 (Implementation)*
