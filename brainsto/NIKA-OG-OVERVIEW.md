# Nika OG — Quick Overview

**"The Terraform for AI Agents"**

## What Is It?

**Nika OG** = Production-grade AI workflow orchestration with explicit agent authorization, multi-provider support, and runtime advisory system.

## The Core Idea

```yaml
schema: "nika/manifest@7.0"
name: my-project
version: "1.0.0"

providers:
  claude:
    model: claude-sonnet-4-20250514
  openai:
    model: gpt-4o

powers:
  - shell:read
  - shell:write
  - mcp:filesystem

workflows:
  - workflows/*.nika.yaml
```

**Result**: Declarative AI agent infrastructure with explicit permissions and multi-provider orchestration.

## Key Features

| Feature | Description |
|---------|-------------|
| **5 Semantic Verbs** | `agent:` `exec:` `fetch:` `invoke:` `infer:` |
| **4 Scope Presets** | `full` `minimal` `debug` `default` |
| **Two-File Architecture** | `nika.yaml` (manifest) + `workflows/*.nika.yaml` |
| **SHAKA System** | Runtime sidecar for advisory/monitoring |
| **Powers & MCP** | Explicit capability authorization |
| **7-Layer Validation** | Schema → Semantic → Resource → Security → Logic → Runtime → Output |

## Quick Example

```bash
# Install
cargo install --path .

# Initialize project
nika init my-project

# Run workflow
nika run code-review.nika.yaml
```

## The 5 Semantic Verbs

```yaml
# Agent orchestration
agent:
  prompt: "Review this code"
  provider: claude
  scope: minimal

# Shell execution
exec:
  command: "npm run build"
  timeout: 300

# HTTP requests
fetch:
  url: "https://api.example.com"
  method: POST

# Sub-workflow invocation
invoke:
  workflow: helper.nika.yaml
  with:
    input: "{{use.data}}"

# LLM inference (simple)
infer:
  prompt: "Summarize: {{use.content}}"
```

## Scope Presets

| Scope | Description | Use Case |
|-------|-------------|----------|
| `full` | All tools + MCP | Complex autonomous tasks |
| `minimal` | Text-only, no tools | Simple Q&A, summaries |
| `debug` | Full + verbose logging | Development/troubleshooting |
| `default` | Balanced subset | General workflows |

## SHAKA System

```
┌─────────────────┐
│   NIKA CORE     │
│   (Execution)   │
└────────┬────────┘
         │ Advisory Protocol
         ▼
┌─────────────────┐
│     SHAKA       │
│   (Sidecar)     │
│                 │
│ • Risk Analysis │
│ • Monitoring    │
│ • Suggestions   │
└─────────────────┘

"SHAKA proposes. NIKA disposes."
```

## Powers Authorization

```yaml
powers:
  # Shell access
  - shell:read           # Read files
  - shell:write          # Modify files
  - shell:execute        # Run commands

  # MCP servers
  - mcp:filesystem       # File operations
  - mcp:github           # GitHub API
  - mcp:database         # Database access

  # Network
  - network:fetch        # HTTP requests
  - network:websocket    # WebSocket connections
```

## Architecture

```
nika.yaml (Manifest)
     │
     ├── providers/      Provider configurations
     ├── powers/         Capability authorization
     └── workflows/      Workflow references
           │
           ▼
workflows/*.nika.yaml
     │
     ├── tasks/          Task definitions
     ├── flows/          DAG edges
     └── guards/         Conditional execution
```

## vs Basic Nika (v0.1)

| Aspect | Basic Nika | Nika OG |
|--------|------------|---------|
| Verbs | 3 (infer, exec, fetch) | 5 (+agent, invoke) |
| Scopes | None | 4 presets |
| Architecture | Single file | Manifest + workflows |
| Authorization | None | Powers & MCP |
| Runtime | Direct execution | SHAKA advisory |
| Validation | Basic | 7-layer system |

---

*See NIKA-OG-SPEC.md for full specification.*
