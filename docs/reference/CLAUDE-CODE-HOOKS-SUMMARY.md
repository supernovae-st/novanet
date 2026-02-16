# Claude Code Hooks: Executive Summary

Comprehensive analysis of Claude Code hook architecture, patterns, and best practices. This document is the entry point for understanding hooks.

---

## What Are Claude Code Hooks?

**Hooks are deterministic automation rules** that execute shell commands or LLM prompts at specific points in Claude Code's lifecycle. Unlike LLM-based decisions, hooks always enforce policies consistently.

### Key Characteristics

- **Event-driven**: Fire at specific lifecycle points (PreToolUse, Stop, SessionStart, etc.)
- **Deterministic**: Enforce rules, don't rely on LLM judgment (though prompt/agent hooks can use LLM)
- **Composable**: Multiple hooks run in parallel, can be chained
- **Powerful**: Can block operations, modify inputs, inject context, log actions
- **Configurable**: File-based YAML/JSON configuration, multiple scope levels

---

## Core Concepts

### 1. The Three-Layer Architecture

```
Hook Event (When)
  ├─ Matcher (Filter: tool name, session source, etc.)
  │   ├─ Handler Type (What to run)
  │   │  ├─ command: Shell script
  │   │  ├─ prompt: Single LLM call
  │   │  └─ agent: Multi-turn subagent
```

### 2. Hook Lifecycle Events

| Event | When | Typical Use Case |
|-------|------|------------------|
| **SessionStart** | Session begins | Inject context, set env vars |
| **UserPromptSubmit** | User submits prompt | Validate input, add context |
| **PreToolUse** | Before tool runs | Block dangerous commands, modify input |
| **PostToolUse** | After tool succeeds | Format code, log actions |
| **PostToolUseFailure** | Tool failed | Log errors, suggest fixes |
| **PermissionRequest** | Permission needed | Auto-approve/deny, modify perms |
| **Stop** | Claude stops | Require tests pass, block until done |
| **SubagentStart/Stop** | Subagent lifecycle | Track parallel tasks |
| **PreCompact** | Context compacting | Archive transcript |
| **Notification** | Status message | Send to Slack, webhooks |
| **SessionEnd** | Session closes | Cleanup, final logging |

### 3. Decision Flow: PreToolUse Example

```
Tool call requested
    ↓
PreToolUse hook fires
    ├─ matcher: "Write|Edit" → matches
    └─ Handler runs:
       ├─ Checks if file is protected
       ├─ Returns permission decision:
       │  ├─ deny → blocked, reason to Claude
       │  ├─ allow → bypass permission prompt
       │  └─ ask → show normal permission dialog
       └─ Optional: modify input (safer file path, etc.)
    ↓
Claude Code acts on decision
```

---

## JSON I/O Format

### Input (Standard for All Events)

```json
{
  "session_id": "abc123",
  "cwd": "/current/directory",
  "hook_event_name": "PreToolUse",
  "tool_name": "Bash",           // ← Event-specific
  "tool_input": {                // ← Event-specific
    "command": "npm test"
  }
}
```

### Output (Three Strategies)

**Strategy 1: Exit Code Only** (Simple)
```bash
exit 0   # Allow
exit 2   # Deny (stderr becomes reason)
exit 1   # Non-blocking error (logged)
```

**Strategy 2: Exit 0 + JSON** (Structured)
```json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Protected file"
  }
}
exit 0
```

**Strategy 3: Exit 2 + Stderr** (Simple Deny)
```bash
echo "Reason for denial" >&2
exit 2
```

**Critical Rule**: Choose ONE strategy. Never mix exit codes and JSON.

---

## Configuration

### File Locations (Priority)

```
.claude/settings.local.json     ← Project local override (highest)
.claude/settings.json           ← Project shared (team)
~/.claude/settings.json         ← User global (lowest)
+ Managed policies (org)
+ Plugin hooks
+ Skill frontmatter
```

### Basic Structure

```json
{
  "hooks": {
    "EventName": [
      {
        "matcher": "tool_name_regex",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/script.sh",
            "timeout": 600
          }
        ]
      }
    ]
  }
}
```

### Matchers by Event

| Event | Matcher Field | Examples |
|-------|---------------|----------|
| PreToolUse | tool_name | `Bash`, `Write\|Edit`, `mcp__.*` |
| SessionStart | source | `startup`, `compact` |
| Notification | type | `idle_prompt`, `permission_prompt` |
| SubagentStart | agent_type | `Bash`, `Plan`, custom |
| (No matcher) | N/A | UserPromptSubmit, Stop, TaskCompleted |

---

## Three Handler Types

### Type 1: Command Hooks (Most Common)

**Run shell scripts** with deterministic logic:

```bash
#!/bin/bash
INPUT=$(cat)  # JSON from stdin
TOOL=$(echo "$INPUT" | jq -r '.tool_name')

if [[ "$TOOL" == "Bash" ]]; then
  echo '{"hookSpecificOutput": {...}}'
  exit 0
fi

exit 0
```

**Use when**: Rules-based filtering, file operations, system integration

### Type 2: Prompt Hooks (Judgment)

**Ask Claude a question**, single turn:

```json
{
  "type": "prompt",
  "prompt": "Are all tests passing? $ARGUMENTS",
  "model": "claude-haiku-4-5-20251001",
  "timeout": 30
}
```

**Use when**: Requires reasoning but no tool access

### Type 3: Agent Hooks (Complex Verification)

**Spawn subagent with tools**, multi-turn:

```json
{
  "type": "agent",
  "prompt": "Verify tests pass. Run test suite and check. $ARGUMENTS",
  "timeout": 120
}
```

**Use when**: Needs file inspection, running commands, complex logic

---

## Event Filtering (Matchers)

### How Matchers Work

1. Event fires (e.g., PreToolUse)
2. Claude Code checks hook configuration for "PreToolUse" entries
3. For each matcher group, evaluates matcher pattern
4. If matcher matches → runs hooks, otherwise skips group

### Matcher Patterns

```
Bash              Exact tool name
Write|Edit        OR patterns (pipe)
^mcp__            All MCP tools (^ = start)
mcp__github__.*   Specific MCP server
.*test.*          Substring match
(empty)           Match all (no filter)
```

### Important: Matchers Are Case-Sensitive

```
✓ "Bash"  → matches Bash tool
✗ "bash"  → no match
✗ "BASH"  → no match
```

### MCP Tool Naming

```
mcp__<server>__<tool>

Examples:
mcp__github__search_repositories
mcp__memory__create_entities
mcp__filesystem__read_file

Matcher patterns:
^mcp__                  All MCP tools
mcp__github__.*         All GitHub tools
mcp__.*__write.*        Write tools across MCP
```

---

## Performance & Configuration

### Timeouts by Handler Type

| Type | Default | Max Recommended | When to Increase |
|------|---------|-----------------|------------------|
| Command | 600s (10m) | 1800s (30m) | External API calls |
| Prompt | 30s | 60s | Complex reasoning |
| Agent | 60s | 300s (5m) | File inspection |

### Async Hooks (Non-Blocking)

```json
{
  "type": "command",
  "command": "npm test",
  "async": true,
  "timeout": 300
}
```

**Behavior**: Starts hook, Claude continues immediately. Results delivered via systemMessage on next turn.

**Use when**: Non-critical background tasks (logging, testing) that shouldn't block

### Deduplication

Identical hooks across different configuration sources are automatically deduplicated:

```
Config A: prettier --write
Config B: prettier --write
Config C: eslint --fix

Result:
✓ prettier --write (run once, not twice)
✓ eslint --fix
```

---

## Common Patterns

### 1. Block Dangerous Commands

```bash
INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command')
if [[ "$CMD" == *"rm -rf /"* ]]; then
  echo "BLOCKED" >&2
  exit 2
fi
exit 0
```

### 2. Protect Sensitive Files

```bash
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path')
if [[ "$FILE" == *".env"* ]]; then
  echo "Protected file" >&2
  exit 2
fi
exit 0
```

### 3. Auto-Format Code

```json
{
  "matcher": "Write|Edit",
  "hooks": [
    {
      "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
    }
  ]
}
```

### 4. Require Tests Pass

```bash
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Already continuing, allow stop
fi
npm test || exit 2  # Block if tests fail
```

### 5. Re-Inject Context After Compaction

```json
{
  "SessionStart": [
    {
      "matcher": "compact",
      "hooks": [
        {
          "command": "echo 'Reminder: Use pnpm, run tests' && git log -5"
        }
      ]
    }
  ]
}
```

---

## Best Practices

### Design

✅ One responsibility per hook
✅ Use matchers to reduce frequency
✅ Chain hooks for complex logic
✅ Test scripts before deploying
✅ Exit early when not applicable

### JSON Output

✅ Always use `jq` for reliable parsing
✅ Return valid JSON only
✅ Include `hookEventName` in `hookSpecificOutput`
✅ Choose exit codes OR JSON, not both
✅ Use exit 0 for JSON, exit 2 for denial

### Security

✅ Validate file paths (reject `..`)
✅ Use absolute paths with `$CLAUDE_PROJECT_DIR`
✅ Protect sensitive files (.env, .git, secrets)
✅ Quote shell variables: `"$VAR"` not `$VAR`
✅ Test on untrusted input

### Performance

✅ Set reasonable timeouts
✅ Use async for non-blocking operations
✅ Minimize subprocess spawning
✅ Cache computations when possible
✅ Match tools specifically (not broadly)

---

## Troubleshooting

| Problem | Check |
|---------|-------|
| Hook not firing | `/hooks` menu shows it? Matcher correct? Case-sensitive? |
| JSON parsing error | Shell profile echo statements? Wrap in `if [[ $- == *i* ]]` |
| Script not found | Absolute path? Escaped spaces? |
| `jq: command not found` | Install jq or use Python/Node |
| Permission denied | `chmod +x hook-script.sh` |
| Infinite Stop loop | Check `stop_hook_active == true` |
| Timeout | Increase `timeout` field or optimize script |

---

## Documentation Map

### 1. **claude-code-hooks-patterns.md** (This file's deep dive)
- Complete hook architecture
- JSON I/O format specifications
- Event filtering details
- Decision control patterns
- Performance considerations
- Lifecycle diagrams

### 2. **claude-code-hooks-recipes.md** (Copy-paste ready)
- 18 ready-to-use hook recipes
- Bash templates for each pattern
- Configuration examples
- Common matchers reference
- Error handling patterns

### 3. **claude-code-hooks-architecture.md** (Visual reference)
- Execution flow diagrams
- Lifecycle flowcharts
- Decision trees
- Configuration resolution
- Handler type selection
- Matcher regex engine

---

## Key Takeaways

1. **Hooks enforce policies deterministically** — use for security, formatting, validation
2. **Events + Matchers = precise filtering** — avoid running hooks unnecessarily
3. **Three handler types for different needs** — command (rules), prompt (judgment), agent (complex)
4. **Simple I/O model** — JSON stdin, exit codes + stdout, predictable behavior
5. **Configuration merges from multiple sources** — project, user, org, plugins, skills
6. **Async support for background tasks** — don't block Claude unnecessarily
7. **Deduplication automatic** — no harm in multiple configs with same hook
8. **Stop hooks need special handling** — check `stop_hook_active` to prevent loops

---

## Quick Reference Commands

```bash
# View hook configuration (interactive)
/hooks

# Test hook script manually
echo '{"tool_name":"Bash"}' | ./my-hook.sh
echo $?

# Toggle verbose mode (see hook output)
Ctrl+O

# Run diagnostic
claude --debug
```

---

## Official Resources

- **Hooks Guide**: https://code.claude.com/docs/en/hooks-guide
- **Hooks Reference**: https://code.claude.com/docs/en/hooks (full event schemas)
- **Agent SDK Hooks**: https://docs.anthropic.com/en/docs/agent-sdk/hooks
- **Example Implementation**: https://github.com/anthropics/claude-code/tree/main/examples/hooks

---

## Next Steps

1. **Read**: `claude-code-hooks-patterns.md` for deep understanding
2. **Browse**: `claude-code-hooks-recipes.md` for copy-paste examples
3. **Visualize**: `claude-code-hooks-architecture.md` for diagrams
4. **Implement**: Start with simple pattern (block dangerous commands)
5. **Test**: Use `/hooks` menu and manual script testing
6. **Deploy**: Add to `.claude/settings.json` for team

---

## Version

- **Analysis Date**: February 2026
- **Claude Code Version**: Latest (as of knowledge cutoff)
- **Hook Events Covered**: 14 events (SessionStart, UserPromptSubmit, PreToolUse, PostToolUse, etc.)
- **Handler Types**: 3 (command, prompt, agent)
- **Total Recipes**: 18 copy-paste patterns
