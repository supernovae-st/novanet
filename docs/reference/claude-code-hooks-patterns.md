# Claude Code Hooks: Architecture, Patterns & Best Practices

Complete analysis of Claude Code hook design patterns, JSON I/O formats, event filtering, and performance considerations based on official documentation.

---

## 1. Hook Architecture Overview

### Core Principles

Hooks provide **deterministic control** over Claude Code's behavior at specific lifecycle points. They execute user-defined shell commands or LLM prompts automatically when events fire, enabling:

- Security enforcement (block dangerous operations)
- Deterministic automation (rules-based, not LLM decisions)
- Tool integration and data transformation
- Audit logging and compliance tracking

### Hook Lifecycle

```
SessionStart
    ↓
UserPromptSubmit (user input) → Hook fires
    ↓
[Agentic Loop]
    ├─ PreToolUse → Hook fires (can block)
    ├─ [Tool executes]
    ├─ PostToolUse → Hook fires
    ├─ PostToolUseFailure → Hook fires
    └─ PermissionRequest → Hook fires
    ↓
Stop → Hook fires (can block/continue)
    ↓
SessionEnd → Hook fires

[Optional]
SubagentStart → Hook fires
SubagentStop → Hook fires
PreCompact → Hook fires
Notification → Hook fires
```

**Key insight**: Hooks fire at **specific points** — they're deterministic control points, not event listeners watching from the sidelines.

---

## 2. Optimal Hook Structure & Organization

### Three-Level Configuration Hierarchy

```
Level 1: Hook Event       (When: PreToolUse, Stop, SessionStart, etc.)
    ├─ Level 2: Matcher   (Filter: tool name regex, session source, etc.)
    │   ├─ Level 3: Handler (What: command, prompt, or agent)
    │   │   ├─ type: "command" | "prompt" | "agent"
    │   │   ├─ Inputs: JSON via stdin
    │   │   └─ Outputs: exit code + JSON stdout
    │   └─ [Multiple handlers run in parallel]
    └─ [Multiple matcher groups]
```

### Configuration Structure

**JSON settings files** are the single source of truth:

```json
{
  "hooks": {
    "EventName": [
      {
        "matcher": "tool_name|regex",
        "hooks": [
          {
            "type": "command|prompt|agent",
            "command": "/path/to/script.sh",
            "timeout": 600,
            "async": false,
            "statusMessage": "Running validation..."
          }
        ]
      }
    ]
  },
  "disableAllHooks": false
}
```

### Storage Locations (Priority Order)

| Location | Scope | Shareable | Format |
|----------|-------|-----------|--------|
| `~/.claude/settings.json` | User (all projects) | No | JSON |
| `.claude/settings.json` | Project (repo) | Yes | JSON |
| `.claude/settings.local.json` | Project (not git) | No | JSON |
| Plugin `hooks/hooks.json` | When enabled | Yes | JSON + description |
| Skill/Agent frontmatter | While active | Yes | YAML |
| Managed policy (org) | Organization-wide | Yes | Admin-controlled |

**Best practice**: Use `.claude/settings.json` for team-shared hooks, `~/.claude/settings.json` for personal workflows.

---

## 3. JSON Input/Output Format Patterns

### Common Input Fields (All Events)

```json
{
  "session_id": "abc123",
  "transcript_path": "/Users/.../.claude/projects/.../transcript.jsonl",
  "cwd": "/current/working/directory",
  "permission_mode": "default|plan|acceptEdits|dontAsk|bypassPermissions",
  "hook_event_name": "PreToolUse"
}
```

### Event-Specific Input Examples

#### PreToolUse (Tool Execution)

```json
{
  "tool_name": "Bash",
  "tool_input": {
    "command": "npm test",
    "description": "Run test suite",
    "timeout": 120000,
    "run_in_background": false
  },
  "tool_use_id": "toolu_01ABC123..."
}
```

**Tool input schema varies by tool:**

| Tool | Key Fields |
|------|-----------|
| `Bash` | `command`, `description`, `timeout`, `run_in_background` |
| `Write` | `file_path`, `content` |
| `Edit` | `file_path`, `old_string`, `new_string`, `replace_all` |
| `Read` | `file_path`, `offset`, `limit` |
| `Glob` | `pattern`, `path` |
| `Grep` | `pattern`, `path`, `glob`, `output_mode`, `-i`, `multiline` |
| `WebFetch` | `url`, `prompt` |
| `WebSearch` | `query`, `allowed_domains`, `blocked_domains` |
| `Task` | `prompt`, `description`, `subagent_type`, `model` |

#### UserPromptSubmit (User Input)

```json
{
  "hook_event_name": "UserPromptSubmit",
  "prompt": "Write a function to calculate factorial",
  "session_id": "abc123"
}
```

#### PostToolUse (Tool Success)

```json
{
  "hook_event_name": "PostToolUse",
  "tool_name": "Write",
  "tool_input": { "file_path": "/path/to/file.txt", "content": "..." },
  "tool_response": { "filePath": "/path/to/file.txt", "success": true },
  "tool_use_id": "toolu_01ABC123..."
}
```

#### SessionStart (Session Initialization)

```json
{
  "hook_event_name": "SessionStart",
  "source": "startup|resume|clear|compact",
  "model": "claude-sonnet-4-5-20250929"
}
```

**Special: `CLAUDE_ENV_FILE` variable available** for persisting environment variables across session.

### Standard Output Patterns

#### Pattern 1: Exit Code Only

```bash
exit 0  # Allow action
exit 2  # Block action (stderr becomes reason)
exit 1  # Non-blocking error (logged, not shown to Claude)
```

#### Pattern 2: JSON with Exit 0

```json
{
  "continue": true,
  "stopReason": "why session stopped",
  "suppressOutput": false,
  "systemMessage": "message for Claude",
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow|deny|ask",
    "permissionDecisionReason": "why",
    "updatedInput": { "field": "new_value" },
    "additionalContext": "context for Claude"
  }
}
```

### Exit Code Semantics

| Exit Code | Behavior | Output Processing | Use Case |
|-----------|----------|-------------------|----------|
| **0** | Success/Allow | Parse JSON if present | Normal operation, additions via stdout |
| **2** | Block/Deny | Ignore JSON, use stderr | Permission denial, blocking operations |
| **Other** | Non-blocking error | Log stderr only | Warnings, non-critical failures |

**Critical rule**: Choose ONE strategy per hook:
- Use exit codes alone, OR
- Exit 0 with JSON output

Never mix them.

---

## 4. Event Filtering (Matchers & Path Patterns)

### Matcher Types by Event

| Event | Matcher Field | Example Values | Regex Support |
|-------|---------------|-----------------|---------------|
| `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` | `tool_name` | `Bash`, `Write\|Edit`, `mcp__.*` | Yes |
| `SessionStart` | `source` | `startup`, `resume`, `clear`, `compact` | Yes (rarely needed) |
| `SessionEnd` | `reason` | `clear`, `logout`, `prompt_input_exit`, `other` | Yes |
| `Notification` | `notification_type` | `permission_prompt`, `idle_prompt`, `auth_success` | Yes |
| `SubagentStart`, `SubagentStop` | `agent_type` | `Bash`, `Explore`, `Plan`, custom names | Yes |
| `PreCompact` | `trigger` | `manual`, `auto` | No |
| `UserPromptSubmit`, `Stop`, `TeammateIdle`, `TaskCompleted` | N/A | Always fires | No matcher support |

### Matcher Patterns

**Empty matcher = matches all** (useful for fallback hooks):

```json
{
  "matcher": "",        // Empty = match all
  "matcher": "*",       // Wildcard = match all
  // omit matcher   // No field = match all
}
```

**Regex patterns are powerful**:

```json
{
  "matcher": "Write|Edit",         // OR: Write OR Edit
  "matcher": "^mcp__",             // Prefix: all MCP tools
  "matcher": "mcp__github__.*",    // Specific MCP server
  "matcher": "mcp__.*__write.*",   // Cross-server pattern
  "matcher": "Bash|Explore|Plan"   // Multiple agents
}
```

### MCP Tool Naming Convention

MCP tools follow: `mcp__<server>__<tool>`

```
mcp__memory__create_entities
mcp__filesystem__read_file
mcp__github__search_repositories
```

**Pattern matching examples**:

```json
{
  "matcher": "mcp__memory__.*",      // All memory tools
  "matcher": "mcp__.*__write.*",     // Any write tools
  "matcher": "^mcp__",               // All MCP tools
  "matcher": "mcp__github__search.*" // Specific patterns
}
```

### Matcher is Case-Sensitive

```json
{
  "matcher": "Bash"    // ✓ Correct (matches Bash tool)
  "matcher": "bash"    // ✗ Wrong (no match)
  "matcher": "BASH"    // ✗ Wrong (no match)
}
```

---

## 5. Hook Handler Types

### Type 1: Command Hooks (Shell Scripts)

**Best for**: Deterministic rules, file operations, system integration

```json
{
  "type": "command",
  "command": "/path/to/script.sh",
  "timeout": 600,
  "async": false,
  "statusMessage": "Running linter..."
}
```

**Shell script template**:

```bash
#!/bin/bash
set -euo pipefail

# Read JSON from stdin
INPUT=$(cat)

# Parse JSON with jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name')

# Your logic
if [[ "$COMMAND" == *"rm -rf"* ]]; then
  echo "Blocked: dangerous command" >&2
  exit 2  # Block
fi

# Optional: Return structured decision
echo '{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "Command is safe"
  }
}'
exit 0  # Success
```

### Type 2: Prompt Hooks (Single-Turn LLM)

**Best for**: Judgment-based decisions, approval gates

```json
{
  "type": "prompt",
  "prompt": "Should Claude proceed? Check if all tasks are complete. $ARGUMENTS",
  "model": "claude-haiku-4-5-20251001",
  "timeout": 30
}
```

**Response format** (model must return):

```json
{
  "ok": true,
  "reason": "All tasks complete"
}
```

**Placeholder**: `$ARGUMENTS` is replaced with hook input JSON.

### Type 3: Agent Hooks (Multi-Turn with Tools)

**Best for**: Complex verification requiring file inspection

```json
{
  "type": "agent",
  "prompt": "Verify that tests pass. Run npm test and check results. $ARGUMENTS",
  "timeout": 120
}
```

**Response format** (agent must return):

```json
{
  "ok": true,
  "reason": "All tests passing"
}
```

**Capabilities**: Agent can use Read, Grep, Glob, Bash, up to 50 tool turns.

---

## 6. Decision Control Patterns

### Pattern 1: Top-Level Decision (PostToolUse, Stop, UserPromptSubmit)

```json
{
  "decision": "block",
  "reason": "Test suite must pass before proceeding"
}
```

**Values**:
- Omit `decision` = allow
- `"decision": "block"` = block

### Pattern 2: Permission Decision (PreToolUse)

```json
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow|deny|ask",
    "permissionDecisionReason": "why",
    "updatedInput": { "command": "safer command" }
  }
}
```

**permissionDecision values**:
- `"allow"` — bypass permission prompt
- `"deny"` — block operation, explain to Claude
- `"ask"` — show normal permission dialog

### Pattern 3: Permission Request Decision (PermissionRequest)

```json
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow|deny",
      "updatedInput": { "field": "new_value" },
      "updatedPermissions": [{ "type": "toolAlwaysAllow", "tool": "Bash" }]
    }
  }
}
```

### Pattern 4: Stop Hook Continuation

```json
{
  "decision": "block",
  "reason": "Three more tests must pass"
}
```

**Check for infinite loops**:

```bash
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Already continuing, allow stop
fi
```

---

## 7. Configuration-Driven Patterns

### Pattern: Multi-Hook Chaining

Multiple hooks run **in parallel** and execute in definition order. Each hook can be independent:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command", "command": "./rate-limiter.sh" },
          { "type": "command", "command": "./auth-check.sh" },
          { "type": "command", "command": "./input-sanitizer.sh" }
        ]
      }
    ]
  }
}
```

**Execution order**: rate-limiter → auth-check → input-sanitizer

**Deduplication**: Identical handlers are automatically deduplicated.

### Pattern: Tool-Specific Hooks

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          { "type": "command", "command": "prettier --write" }
        ]
      },
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command", "command": "log-bash-commands.sh" }
        ]
      },
      {
        "matcher": "",  // No matcher = all tools
        "hooks": [
          { "type": "command", "command": "global-audit.sh" }
        ]
      }
    ]
  }
}
```

### Pattern: Session-Scoped Hooks

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: Use npm, not yarn' && git log --oneline -5"
          }
        ]
      }
    ]
  }
}
```

---

## 8. Performance Considerations

### Timeout Management

| Handler Type | Default | Max Recommended | When to Increase |
|--------------|---------|-----------------|------------------|
| Command | 600s (10m) | 1800s (30m) | External API calls, slow tests |
| Prompt | 30s | 60s | Complex reasoning |
| Agent | 60s | 300s (5m) | Complex file inspection |

**Per-hook override**:

```json
{
  "type": "command",
  "command": "slow-validation.sh",
  "timeout": 1800
}
```

### Async Hooks (Non-Blocking)

Useful for long-running tasks that don't need to block Claude:

```json
{
  "type": "command",
  "command": "npm test",
  "async": true,
  "timeout": 300
}
```

**Behavior**:
- Hook starts, Claude continues immediately
- Results delivered on next conversation turn via `systemMessage` or `additionalContext`
- Cannot block or control behavior (action already happened)
- Only available on `type: "command"` hooks

**Example use case**: Background test runner after file writes

```bash
#!/bin/bash
INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path')

npm test 2>&1 > /tmp/test-results.txt
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo '{"systemMessage": "Tests passed after editing '"$FILE"'"}'
else
  TEST_OUTPUT=$(cat /tmp/test-results.txt)
  echo '{"systemMessage": "Tests failed: '"$TEST_OUTPUT"'"}'
fi
```

### Parallel Execution

- All hooks for an event run in **parallel**
- Deduplication applies across all hooks
- Each hook gets full input independently

### Performance Anti-Patterns

❌ **Avoid**:
- Blocking external API calls without timeout
- Large JSON processing without `jq` streaming
- Unnecessary file reads in hooks that fire frequently
- Synchronous shells with initialization overhead

✅ **Prefer**:
- Cached computations when possible
- Efficient JSON parsing (`jq` one-liners)
- Matcher filtering to reduce hook frequency
- Async hooks for non-blocking operations

---

## 9. Practical Implementation Examples

### Example 1: Protect Sensitive Files

```bash
#!/bin/bash
# .claude/hooks/protect-files.sh
INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

PROTECTED=(".env" "package-lock.json" ".git" ".ssh")
for pattern in "${PROTECTED[@]}"; do
  if [[ "$FILE" == *"$pattern"* ]]; then
    echo "Protected file: $FILE cannot be modified" >&2
    exit 2  # Block
  fi
done

exit 0
```

**Configuration**:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
          }
        ]
      }
    ]
  }
}
```

### Example 2: Auto-Format Code

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

### Example 3: Desktop Notifications

```json
{
  "hooks": {
    "Notification": [
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "osascript -e 'display notification \"Claude waiting for input\" with title \"Claude Code\"'"
          }
        ]
      }
    ]
  }
}
```

### Example 4: Re-Inject Context After Compaction

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Use pnpm, run tests before commit' && git log --oneline -5"
          }
        ]
      }
    ]
  }
}
```

### Example 5: Structured Permission Decision

```bash
#!/bin/bash
# Validate file path before write
INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path')

if [[ "$FILE" == "/etc/"* ]]; then
  cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "System directories are protected"
  }
}
EOF
  exit 0
fi

exit 0  # Allow
```

---

## 10. Best Practices Summary

### Design

- ✅ One responsibility per hook
- ✅ Use matchers to reduce frequency
- ✅ Chain hooks for complex logic
- ✅ Prefer sync hooks for blocking, async for background
- ✅ Exit early when not applicable

### Security

- ✅ Validate and sanitize inputs
- ✅ Quote shell variables: `"$VAR"` not `$VAR`
- ✅ Check for path traversal: reject `..`
- ✅ Use absolute paths with `$CLAUDE_PROJECT_DIR`
- ✅ Skip sensitive files (`.env`, `.git`, keys)

### JSON Output

- ✅ Use jq for reliable parsing
- ✅ Return valid JSON only
- ✅ Choose: exit codes OR JSON, not both
- ✅ Always include `hookEventName` in hookSpecificOutput
- ✅ Exit 0 = parse JSON, Exit 2 = ignore JSON

### Performance

- ✅ Set reasonable timeouts
- ✅ Use async for non-blocking operations
- ✅ Minimize subprocess spawning
- ✅ Cache when possible
- ✅ Log warnings, not debug info

### Maintenance

- ✅ Comment why each hook exists
- ✅ Test scripts before adding to hooks
- ✅ Review hooks quarterly
- ✅ Use `/hooks` menu for introspection
- ✅ Document matcher rationale

---

## 11. Hook Lifecycle Diagram

```
┌─────────────────────────────────────────────────────────────┐
│ SESSION START                                               │
├─────────────────────────────────────────────────────────────┤
│ ↓                                                           │
│ [SessionStart hook fires]                                   │
│ - matcher: startup|resume|clear|compact                    │
│ - Can inject additionalContext via stdout                  │
│ - Can set CLAUDE_ENV_FILE for env vars                     │
│ ↓                                                           │
│ USER PROMPT SUBMISSION                                      │
│ ↓                                                           │
│ [UserPromptSubmit hook fires]                               │
│ - No matcher support                                        │
│ - Can add context via stdout or additionalContext          │
│ - Exit 2 blocks prompt entirely                             │
│ ↓                                                           │
│ ┌─ AGENTIC LOOP ──────────────────────────────────────────┐ │
│ │ ↓                                                        │ │
│ │ [PreToolUse hook fires]                                 │ │
│ │ - matcher: tool_name (regex)                            │ │
│ │ - CAN block tool call                                   │ │
│ │ - CAN modify tool input                                 │ │
│ │ ↓                                                        │ │
│ │ [Tool executes]                                         │ │
│ │ ↓                                                        │ │
│ │ [PostToolUse OR PostToolUseFailure hook fires]          │ │
│ │ - Cannot undo (tool already executed)                   │ │
│ │ - Can inject additionalContext                          │ │
│ │ ↓                                                        │ │
│ │ [PermissionRequest hook fires (if needed)]              │ │
│ │ - Can auto-approve/deny on behalf of user               │ │
│ │ - Can modify input with updatedPermissions              │ │
│ │ ↓                                                        │ │
│ │ Loop until Stop event fires                             │ │
│ └──────────────────────────────────────────────────────────┘ │
│ ↓                                                           │
│ [Stop hook fires]                                           │
│ - No matcher support                                        │
│ - check stop_hook_active to prevent infinite loops         │
│ - CAN block to continue working                            │
│ ↓                                                           │
│ [Optional: Subagent lifecycle]                              │
│ - SubagentStart hook fires                                 │
│ - SubagentStop hook fires                                  │
│ ↓                                                           │
│ [Optional: Context compaction]                              │
│ - PreCompact hook fires                                    │
│ - matcher: manual|auto                                     │
│ ↓                                                           │
│ [Notification hook fires]                                   │
│ - Cannot block                                              │
│ ↓                                                           │
│ SESSION END                                                 │
│ ↓                                                           │
│ [SessionEnd hook fires]                                     │
│ - matcher: clear|logout|prompt_input_exit|other            │
│ - Cannot block                                              │
│ - Good for cleanup/logging                                 │
└─────────────────────────────────────────────────────────────┘
```

---

## 12. Troubleshooting Checklist

| Problem | Check |
|---------|-------|
| Hook not firing | `/hooks` menu shows hook? Matcher correct? Case-sensitive? Right event? |
| JSON parsing error | Shell profile has echo statements? Wrap in `if [[ $- == *i* ]]` |
| Script not found | Absolute path? Escape spaces in `$CLAUDE_PROJECT_DIR`? |
| jq not found | Install jq. Or parse JSON with Python/Node instead |
| Script not executable | `chmod +x hook-script.sh` |
| Infinite Stop loop | Check `stop_hook_active == true` and exit early |
| Async hook never completes | Logs to file instead of stdout? Use `systemMessage` in JSON |
| Permission prompt still shows | PreToolUse returns `ask`? Use `deny` to block |
| Hook timeout | Increase `timeout` field or optimize script |

---

## References

- **Hooks Reference**: https://code.claude.com/docs/en/hooks
- **Hooks Guide**: https://code.claude.com/docs/en/hooks-guide
- **Agent SDK Hooks**: https://docs.anthropic.com/en/docs/agent-sdk/hooks
- **Settings**: https://code.claude.com/docs/en/settings#hook-configuration
- **Example Implementation**: https://github.com/anthropics/claude-code/tree/main/examples/hooks
