# Claude Code DX Overview

NovaNet uses Claude Code as the primary development interface. This section documents our custom skills, agents, and advanced patterns.

## Architecture (v0.12.0)

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {
  'primaryColor': '#06b6d4',
  'secondaryColor': '#8b5cf6',
  'tertiaryColor': '#10b981'
}}}%%
flowchart TB
    subgraph CLAUDE[".claude/ Directory"]
        direction TB
        SETTINGS["settings.json\nPermissions · Env · Hooks"]
        HOOKS["hooks/\nSessionStart · PostToolUse"]
        RULES["rules/\nrust.md · typescript.md · cypher.md"]
        SKILLS["skills/\n5 skills"]
        AGENTS["agents/\n2 agents"]
        COMMANDS["commands/\n8 commands"]
    end

    subgraph FEATURES["Features"]
        AUTO["Auto-Format\nRust · TypeScript"]
        CONTEXT["Context Injection\n@imports"]
        SCOPED["Path-Scoped Rules\nLanguage-specific"]
    end

    HOOKS --> AUTO
    SETTINGS --> CONTEXT
    RULES --> SCOPED

    style SETTINGS fill:#06b6d4,color:#fff
    style SKILLS fill:#8b5cf6,color:#fff
    style AGENTS fill:#10b981,color:#fff
```

## Quick Reference

| Component | Count | Purpose |
|-----------|-------|---------|
| **Skills** | 5 | Automatic context injection |
| **Agents** | 2 | Specialized subagents |
| **Commands** | 8 | Slash commands (/schema, /novanet-*) |
| **Hooks** | 3 | SessionStart, PostToolUse, keybindings |
| **Rules** | 4 | Path-specific (Rust, TS, Cypher, decisions) |

## Directory Structure

```
.claude/
├── settings.json          # Permissions, env, hooks config
├── hooks/
│   ├── session-start.sh   # Show project status on start
│   ├── post-edit-format.sh# Auto-format after edits
│   └── keybindings-reminder.sh  # TUI keybindings check
├── rules/
│   ├── rust.md            # tools/novanet/**/*.rs
│   ├── typescript.md      # packages/, apps/**/*.ts
│   ├── cypher.md          # packages/db/seed/**/*.cypher
│   └── novanet-decisions.md  # ADR decisions
│   └── novanet-terminology.md # Terminology reference
├── skills/
│   ├── novanet-architecture/
│   ├── novanet-sync/
│   ├── novanet-tui/       # TUI exploration skill
│   ├── codebase-audit/    # "Ralph Wiggum Loop"
│   └── token-audit/
├── agents/
│   ├── neo4j-architect.md
│   └── code-reviewer.md
└── commands/
    ├── novanet-arch.md
    ├── novanet-sync.md
    ├── schema.md
    ├── schema-add-node.md
    ├── schema-edit-node.md
    ├── schema-add-arc.md
    ├── security-audit.md
    └── codebase-audit.md
```

## Key Features

### 1. Auto-Imports (@imports)

CLAUDE.md automatically imports README, ROADMAP, and CHANGELOG:

```markdown
## Auto-Imported Context

@README.md @ROADMAP.md @CHANGELOG.md
```

### 2. Hooks

- **SessionStart**: Displays `NovaNet v0.12.0 | Branch: main | Uncommitted: X`
- **PostToolUse**: Auto-formats Rust (rustfmt) and TypeScript (prettier)
- **keybindings**: Reminds to check KEYBINDINGS.md when editing TUI files

### 3. Path-Specific Rules

Rules apply only when working with matching files:

```yaml
---
paths:
  - "tools/novanet/**/*.rs"
---
# Rust-specific rules here
```

### 4. NovaNet-Specific Skills

| Skill | Purpose |
|-------|---------|
| `novanet-architecture` | Display schema architecture diagrams |
| `novanet-sync` | Validate/regenerate from YAML sources |
| `novanet-tui` | Launch interactive TUI exploration |
| `codebase-audit` | Ralph Wiggum iterative audit loop |
| `token-audit` | Check design token adoption |

## Advanced Patterns

NovaNet uses several advanced Claude Code patterns:

- **[Ultrathink](./ultrathink.md)** — Extended thinking for complex decisions
- **[Context7](./context7.md)** — Live documentation lookup
- **[Ralph Wiggum Loop](./ralph-wiggum.md)** — Iterative codebase auditing
- **[Devil's Advocate](./devils-advocate.md)** — Challenging assumptions

## Schema Commands (v0.12.0)

| Command | Description |
|---------|-------------|
| `/schema:add-node` | Add new node type via Socratic discovery |
| `/schema:edit-node` | Modify existing node type |
| `/schema:add-arc` | Add new arc type |
| `/novanet-arch` | Display architecture diagrams |
| `/novanet-sync` | Validate and regenerate artifacts |
| `/security-audit` | Run comprehensive security audit |
| `/codebase-audit` | Run Ralph Wiggum audit loop |

## Related Documentation

- **[Skills](./skills.md)** — Detailed skill documentation
- **[Agents](./agents.md)** — Specialized subagent patterns
- **[Hooks & Rules](./hooks-rules.md)** — Automation configuration
- **[Advanced Patterns](./advanced-patterns.md)** — Pattern combinations
