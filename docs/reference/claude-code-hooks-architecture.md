# Claude Code Hooks: Architecture Diagrams & Flows

Visual reference for hook architecture, event flows, and decision trees.

---

## 1. Hook Execution Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│                       CLAUDE CODE CLI                                │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  HOOK MANAGER                                               │   │
│  │  ────────────────────────────────────────────────────────── │   │
│  │                                                             │   │
│  │  1. Load hook config                                        │   │
│  │     ├─ ~/.claude/settings.json (global)                    │   │
│  │     ├─ .claude/settings.json (project)                     │   │
│  │     └─ .claude/settings.local.json (local override)        │   │
│  │                                                             │   │
│  │  2. Filter by event type                                    │   │
│  │     └─ Match all hooks registered for "PreToolUse", etc.   │   │
│  │                                                             │   │
│  │  3. Filter by matcher                                       │   │
│  │     ├─ PreToolUse: matcher = tool_name (regex)             │   │
│  │     ├─ SessionStart: matcher = source (startup/compact)    │   │
│  │     └─ Notification: matcher = type (idle_prompt, etc.)    │   │
│  │                                                             │   │
│  │  4. Deduplicate identical handlers                          │   │
│  │     └─ Multiple hooks w/ same command = run once           │   │
│  │                                                             │   │
│  │  5. Execute handlers                                        │   │
│  │     ├─ Sync: Wait for completion before proceeding         │   │
│  │     └─ Async: Start and continue immediately               │   │
│  │                                                             │   │
│  │  6. Process outputs                                         │   │
│  │     ├─ Exit code (0 = allow, 2 = deny)                     │   │
│  │     ├─ JSON output (if exit 0)                             │   │
│  │     └─ Stderr (reason for denial)                          │   │
│  │                                                             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘

              ↓

┌──────────────────────────────────────────────────────────────────────┐
│                    HANDLER EXECUTION LAYER                           │
├──────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  type: "command"                                                     │
│  ┌─────────────────────────────────┐                                │
│  │  Shell Script Execution         │                                │
│  │  • Spawn shell subprocess       │                                │
│  │  • Pass event JSON to stdin     │                                │
│  │  • Wait for exit code + stdout  │                                │
│  │  • Timeout: 600s default        │                                │
│  └─────────────────────────────────┘                                │
│                                                                      │
│  type: "prompt"                                                      │
│  ┌─────────────────────────────────┐                                │
│  │  LLM Decision (Haiku)           │                                │
│  │  • Send event + prompt to Claude│                                │
│  │  • Expect: {"ok": bool, ...}    │                                │
│  │  • Single turn, no tools        │                                │
│  │  • Timeout: 30s default         │                                │
│  └─────────────────────────────────┘                                │
│                                                                      │
│  type: "agent"                                                       │
│  ┌─────────────────────────────────┐                                │
│  │  Agent Verification             │                                │
│  │  • Spawn subagent with tools    │                                │
│  │  • Multi-turn reasoning         │                                │
│  │  • Up to 50 tool turns          │                                │
│  │  • Timeout: 60s default         │                                │
│  └─────────────────────────────────┘                                │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 2. Event Filtering Flow

```
┌─────────────────────────────────────────────────────────┐
│  HOOK EVENT FIRES                                       │
│  (SessionStart, PreToolUse, PostToolUse, Stop, etc.)    │
└────────────────┬────────────────────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────────────────────┐
│  LOAD HOOK CONFIGURATION                                │
│  All "hooks" entries for this event type                │
└────────────────┬────────────────────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────────────────────┐
│  FOR EACH MATCHER GROUP:                                │
│  └─ Check matcher (if present)                          │
│                                                         │
│  Match field varies by event:                           │
│  ├─ PreToolUse: tool_name (e.g., "Bash")               │
│  ├─ SessionStart: source (e.g., "compact")              │
│  ├─ Notification: notification_type (e.g., idle_prompt)│
│  └─ (some events don't support matchers)                │
└────────────────┬────────────────────────────────────────┘
                 │
                 ↓
         ┌───────┴────────┐
         │                │
      [Match]         [No Match]
         │                │
         ↓                ↓
    Execute         Skip group
    handlers
         │
         ↓
    [All matchers
      processed]
```

---

## 3. JSON Input/Output Flow

```
┌─────────────────────────────────────────────────────────┐
│  EVENT FIRES                                            │
│  Claude Code generates event-specific JSON              │
└────────────────┬────────────────────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────────────────────┐
│  PASS TO HANDLER VIA STDIN                              │
│  {                                                      │
│    "session_id": "...",                                 │
│    "hook_event_name": "PreToolUse",                     │
│    "tool_name": "Bash",      ← Event-specific          │
│    "tool_input": {...},      ← Event-specific          │
│    ... (common fields)                                  │
│  }                                                      │
└────────────────┬────────────────────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────────────────────┐
│  HANDLER PROCESSES & OUTPUTS                            │
│                                                         │
│  Option A: Exit code only (simple)                      │
│  ──────────────────────────────────                     │
│  exit 0        → Allow/proceed                          │
│  exit 2        → Deny/block                             │
│  exit 1,3,...  → Non-blocking error                     │
│                                                         │
│  Option B: Exit 0 + JSON output (structured)            │
│  ──────────────────────────────────────────────         │
│  {                                                      │
│    "hookSpecificOutput": {                              │
│      "hookEventName": "PreToolUse",                     │
│      "permissionDecision": "allow|deny|ask",            │
│      "permissionDecisionReason": "...",                 │
│      "updatedInput": { ... }                            │
│    },                                                   │
│    "additionalContext": "...",                          │
│    "systemMessage": "..."                               │
│  }                                                      │
│  exit 0        → JSON parsed and acted upon             │
│                                                         │
│  Option C: Exit 2 + stderr (simple deny)                │
│  ────────────────────────────────────────               │
│  echo "reason" >&2                                      │
│  exit 2        → reason shown to Claude                 │
│                                                         │
│  DO NOT: Exit 2 + JSON (JSON ignored)                   │
│                                                         │
└────────────────┬────────────────────────────────────────┘
                 │
                 ↓
┌─────────────────────────────────────────────────────────┐
│  CLAUDE CODE ACTS ON RESPONSE                           │
│  ├─ Allow → proceed with action                         │
│  ├─ Deny → block, explain reason to Claude              │
│  ├─ Ask → show user permission prompt                   │
│  ├─ Modified input → use updated tool_input             │
│  └─ systemMessage → inject into Claude's context        │
└─────────────────────────────────────────────────────────┘
```

---

## 4. PreToolUse Decision Tree

```
┌────────────────────────────────────────────────────────────────┐
│ HOOK EVENT: PreToolUse                                         │
│ WHEN: Before tool execution (can block)                        │
└─────────────────┬──────────────────────────────────────────────┘
                  │
                  ↓
         ┌────────────────────┐
         │ Permission Decision│
         │ (3 options)        │
         └────────┬───────────┘
                  │
      ┌───────────┼────────────┐
      │           │            │
    deny        ask          allow
      │           │            │
      ↓           ↓            ↓
   BLOCK    PROMPT USER    EXECUTE
   ├─ Stop  ├─ Show dialog └─ Run tool
   ├─ Explain│─ User chooses  └─ Proceed
   │  reason│      yes/no
   └─ Show  └─ Act on choice
      reason

Optional: Modify input before execution
├─ updatedInput: { file_path: "/new/path", ... }
└─ Only if permissionDecision: "allow"
```

---

## 5. Complete Hook Lifecycle

```
┌────────────────────────────────────────────────────────────────┐
│ SESSION START                                                  │
├────────────────────────────────────────────────────────────────┤
│ ↓ SessionStart hook (matcher: startup|resume|clear|compact)   │
│ ├─ Can inject: additionalContext                              │
│ ├─ Set: CLAUDE_ENV_FILE for env vars                          │
│ └─ [Can block with exit 2 to restart session]                │
└─────────────────┬──────────────────────────────────────────────┘
                  │
                  ↓
┌────────────────────────────────────────────────────────────────┐
│ USER SUBMITS PROMPT                                            │
├────────────────────────────────────────────────────────────────┤
│ ↓ UserPromptSubmit hook (NO matcher support)                   │
│ ├─ Can inject: additionalContext                              │
│ └─ Can block with exit 2 (rejects entire prompt)              │
└─────────────────┬──────────────────────────────────────────────┘
                  │
                  ↓
    ╔════════════════════════════════════════════════════╗
    ║        AGENTIC LOOP (repeats until Stop)           ║
    ╠════════════════════════════════════════════════════╣
    │                                                    │
    │  ↓ PreToolUse hook                                │
    │  ├─ Matcher: tool_name                            │
    │  ├─ CAN block (deny)                              │
    │  ├─ CAN allow (skip permission dialog)            │
    │  ├─ CAN modify input (updatedInput)               │
    │  ├─ CAN inject context (additionalContext)        │
    │  └─ Response: permissionDecision + reason         │
    │                                                    │
    │  ↓ [Tool executes]                                │
    │                                                    │
    │  ├─ PostToolUse hook (if success)                 │
    │  │  ├─ Matcher: tool_name                         │
    │  │  ├─ Cannot undo (tool ran)                     │
    │  │  ├─ Can inject: additionalContext              │
    │  │  └─ tool_response available                    │
    │  │                                                 │
    │  └─ PostToolUseFailure hook (if failed)           │
    │     ├─ Matcher: tool_name                         │
    │     ├─ Error details available                    │
    │     └─ Can inject: additionalContext              │
    │                                                    │
    │  ↓ [Optional: PermissionRequest hook]             │
    │  ├─ If user permission needed                     │
    │  ├─ Can auto-approve/deny                         │
    │  └─ Can modify permissions                        │
    │                                                    │
    │  ↓ [Loop back or proceed to Stop]                 │
    │                                                    │
    ╚════════════════════════════════════════════════════╝
                  │
                  ↓
┌────────────────────────────────────────────────────────────────┐
│ CLAUDE FINISHES RESPONSE (Stop)                                │
├────────────────────────────────────────────────────────────────┤
│ ↓ Stop hook (NO matcher support)                               │
│ ├─ check: stop_hook_active to prevent infinite loops          │
│ ├─ CAN block (decision: block) to continue working             │
│ ├─ CAN inject: systemMessage                                   │
│ └─ Exit 2 = block → Claude keeps working                      │
└─────────────────┬──────────────────────────────────────────────┘
                  │
                  ↓
    ╔════════════════════════════════════════════════════╗
    ║     [OPTIONAL] SUBAGENT LIFECYCLE                  ║
    ╠════════════════════════════════════════════════════╣
    │ SubagentStart: agent spawned                       │
    │ - Matcher: agent_type (Bash, Explore, Plan)       │
    │ - Can inject: additionalContext                   │
    │                                                    │
    │ SubagentStop: agent finished                       │
    │ - Matcher: agent_type                             │
    │ - Can access: agent transcript                    │
    │ - Can block result                                │
    ╚════════════════════════════════════════════════════╝
                  │
                  ↓
┌────────────────────────────────────────────────────────────────┐
│ [OPTIONAL] CONTEXT COMPACTION                                  │
├────────────────────────────────────────────────────────────────┤
│ ↓ PreCompact hook                                              │
│ ├─ Matcher: manual|auto                                        │
│ └─ Can block or archive full transcript                        │
│                                                                │
│ ↓ SessionStart hook again                                      │
│ ├─ Matcher: compact                                            │
│ ├─ Re-injects critical context                                │
│ └─ Prevents context loss after summarization                  │
└─────────────────┬──────────────────────────────────────────────┘
                  │
                  ↓
┌────────────────────────────────────────────────────────────────┐
│ NOTIFICATIONS & SESSION END                                    │
├────────────────────────────────────────────────────────────────┤
│ ↓ Notification hook                                            │
│ ├─ Matcher: notification_type                                  │
│ └─ Can send to external services (Slack, webhooks)             │
│                                                                │
│ ↓ SessionEnd hook                                              │
│ ├─ Matcher: reason (clear, logout, other)                      │
│ ├─ Cannot block (session ending)                               │
│ └─ Good for cleanup/logging                                   │
└────────────────────────────────────────────────────────────────┘
```

---

## 6. Handler Type Decision Tree

```
                   ┌─────────────────┐
                   │ What to enforce?│
                   └────────┬────────┘
                            │
            ┌───────────────┼───────────────┐
            │               │               │
     Deterministic    Judgment-based   Complex
      rule/check       decision       verification
            │               │               │
            ↓               ↓               ↓
        [Command]      [Prompt Hook]  [Agent Hook]
            │               │               │
        Exit codes      Single LLM     Multi-turn
        & JSON out      call w/        tool-using
                        $ARGUMENTS      agent
            │               │               │
     ┌──────┴──────┐   ┌────┴─────┐   ┌────┴─────┐
     │             │   │          │   │          │
   Fast         Complex Model sees Full file    Needs
   rules        logic   hook input inspection   tools
     │             │   │          │   │          │
   Exit 0 or    Exit 0 &         │ Exit 0 &     │
   Exit 2       JSON             │ JSON         │
                                 │              │
                            Default:      Timeout:
                            Claude-3-5    120s max
                            Sonnet

Examples:
─────────
Command: "block if .env"      → Simple regex
Prompt: "all tests pass?"      → Needs reasoning
Agent: "verify schema valid"   → Needs Cypher queries
```

---

## 7. Configuration Priority & Merging

```
┌───────────────────────────────────────────────────────┐
│ HOOK CONFIGURATION RESOLUTION (Priority Order)        │
├───────────────────────────────────────────────────────┤
│                                                       │
│ 1. .claude/settings.local.json (highest priority)     │
│    └─ Project-specific overrides (not in git)         │
│                                                       │
│ 2. .claude/settings.json                              │
│    └─ Team-shared hooks (committed to repo)           │
│                                                       │
│ 3. ~/.claude/settings.json                            │
│    └─ User global hooks (local machine only)          │
│                                                       │
│ 4. Managed policy (org-wide)                          │
│    └─ Enterprise policies (admin-controlled)          │
│                                                       │
│ 5. Plugin hooks/hooks.json                            │
│    └─ Plugin-provided hooks (when enabled)            │
│                                                       │
│ 6. Skill/Agent frontmatter                            │
│    └─ Skill-specific hooks (while skill active)       │
│                                                       │
│ Result: Hooks MERGED (not replaced)                   │
│ ├─ Same event can have hooks from multiple levels     │
│ ├─ All applicable hooks run in order                  │
│ ├─ Identical commands deduplicated                    │
│ └─ Deduplication happens WITHIN level, then merge     │
│                                                       │
└───────────────────────────────────────────────────────┘
```

---

## 8. Error Handling & Exit Codes

```
┌────────────────────────────────────────────────────────┐
│ HANDLER EXECUTION                                      │
└────────────────────┬─────────────────────────────────┘
                     │
                     ↓
          ┌──────────────────────┐
          │  Spawn subprocess    │
          │  Pass JSON to stdin  │
          │  Capture exit code   │
          └──────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
      Exit 0       Exit 2      Exit 1/3+
        │            │            │
        ↓            ↓            ↓
    Success       Deny/Block   Non-blocking
                                  error
        │            │            │
        ├─ Parse   ├─ Ignore    └─ Log stderr
        │  JSON    │  JSON        (not to
        │  output  │              Claude)
        │          ├─ Use stderr  │
        ├─ Act on  │  as reason   └─ Proceed
        │  decision├─ Claude     (action not
        │          │  sees reason blocked)
        │          │
        └─ Allow   └─ Block
           action     action

Error cases:
─────────────
Timeout (> 600s) → Block → "hook timeout"
Crash (SIGSEGV) → Block → "hook error"
JSON parse error → Block → "invalid JSON"
Missing jq → Error → "command not found"
Unreadable file → Block → "permission denied"
```

---

## 9. Matcher Regex Engine

```
┌─────────────────────────────────────────────────────────┐
│ MATCHER PATTERN MATCHING                                │
│ (Rust regex engine)                                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│ Pattern        │ Matches                               │
│ ──────────────────────────────────────────────────     │
│ Bash           │ Tool named exactly "Bash"            │
│ Write|Edit     │ Tool named "Write" OR "Edit"         │
│ ^Bash          │ Tool starting with "Bash"            │
│ .*test.*       │ Tool containing "test" anywhere      │
│ ^mcp__         │ All MCP tools (start with mcp__)     │
│ mcp__github__.*│ Any GitHub MCP tool                  │
│ mcp__.*__write │ Write-like tools across MCP          │
│ (empty)        │ Matches all (no filter)              │
│                                                         │
│ Case-sensitive: "bash" ≠ "Bash"                        │
│ Escaped pipes: "Write\|Edit" (literal pipe)            │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## 10. JSON Output Structure (PreToolUse Focus)

```
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",          ← Always match input
    "permissionDecision": "allow|deny|ask", ← Required
    "permissionDecisionReason": "...",      ← Required for deny
    "updatedInput": {                        ← Optional
      "command": "safer-cmd",
      "file_path": "/new/path"
    }
  },
  "additionalContext": "...",               ← Optional
  "systemMessage": "...",                   ← Optional
  "continue": true,                         ← Optional
  "suppressOutput": false                   ← Optional
}
```

---

## 11. MCP Tool Naming Pattern

```
┌──────────────────────────────────────────────────────────┐
│ MCP TOOL NAMING: mcp__<server>__<tool>                   │
├──────────────────────────────────────────────────────────┤
│                                                          │
│ Example MCP server "github":                             │
│ ├─ mcp__github__search_repositories                      │
│ ├─ mcp__github__create_pull_request                      │
│ ├─ mcp__github__get_issue                                │
│ └─ ...                                                   │
│                                                          │
│ Example MCP server "memory":                             │
│ ├─ mcp__memory__create_entities                          │
│ ├─ mcp__memory__update_relationship                      │
│ └─ ...                                                   │
│                                                          │
│ Matcher patterns:                                        │
│ ├─ "mcp__github__.*" → all GitHub tools                  │
│ ├─ "^mcp__" → all MCP tools                              │
│ ├─ "mcp__.*__write" → write tools across servers         │
│ └─ "mcp__.*__search" → search tools across servers       │
│                                                          │
│ Server name comes from mcpServers config key             │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## 12. Deduplication Logic

```
┌────────────────────────────────────────┐
│ MULTIPLE HOOKS SAME EVENT              │
├────────────────────────────────────────┤
│                                        │
│ Before deduplication:                  │
│ PostToolUse: [                          │
│   { command: "prettier --write" },     │
│   { command: "prettier --write" },     │
│   { command: "eslint --fix" }          │
│ ]                                      │
│                                        │
│ After deduplication:                   │
│ PostToolUse: [                          │
│   { command: "prettier --write" },     │
│   { command: "eslint --fix" }          │
│ ]                                      │
│                                        │
│ Rule: Identical command strings = 1   │
│ ├─ Dedup happens WITHIN level        │
│ ├─ Then merged across levels          │
│ └─ Execution order preserved           │
│                                        │
└────────────────────────────────────────┘
```

---

## 13. Async Hook Flow

```
┌──────────────────────────────────────────────────────┐
│ SYNC HOOK (default)                                  │
├──────────────────────────────────────────────────────┤
│                                                      │
│ Claude Code: PostToolUse event fires                │
│      │                                               │
│      ├─ Start hook                                   │
│      ├─ Wait for completion                         │
│      ├─ Read stdout/exit code                       │
│      └─ Act on response (formatter done before continuing)
│                                                      │
│ Best for: Blocking operations (validation, format)  │
│ Timeout: 600s (10m)                                 │
│                                                      │
└──────────────────────────────────────────────────────┘


┌──────────────────────────────────────────────────────┐
│ ASYNC HOOK (async: true)                             │
├──────────────────────────────────────────────────────┤
│                                                      │
│ Claude Code: PostToolUse event fires                │
│      │                                               │
│      ├─ Start hook                                   │
│      ├─ Continue immediately (don't wait)           │
│      ├─ Hook runs in background                     │
│      ├─ Results delivered via systemMessage         │
│      │  on next conversation turn                   │
│      │                                               │
│      └─ Next user prompt or Claude response         │
│         (hook results injected)                     │
│                                                      │
│ Best for: Non-blocking tasks (logging, background)  │
│ Timeout: 300s (5m, configurable)                    │
│ Note: Cannot modify behavior (action already done)  │
│                                                      │
└──────────────────────────────────────────────────────┘
```

---

## 14. Stop Hook Infinite Loop Prevention

```
┌────────────────────────────────────────────────────────┐
│ STOP HOOK INFINITE LOOP RISK                           │
├────────────────────────────────────────────────────────┤
│                                                        │
│ Problem:                                               │
│ ─────────                                              │
│ Stop hook returns: decision: "block" (continue)        │
│ Claude gets: "continue working on X"                  │
│ Claude stops again...                                 │
│ Stop hook fires again...                              │
│ → Infinite loop                                        │
│                                                        │
│ Solution:                                              │
│ ────────────                                           │
│ Check "stop_hook_active" field in input:              │
│                                                        │
│ #!/bin/bash                                            │
│ INPUT=$(cat)                                           │
│ IS_ACTIVE=$(echo "$INPUT" | jq -r '.stop_hook_active')│
│                                                        │
│ if [ "$IS_ACTIVE" = "true" ]; then                    │
│   exit 0  # Let Claude stop (already continuing)      │
│ fi                                                     │
│                                                        │
│ # ... rest of logic                                   │
│                                                        │
│ Behavior:                                              │
│ ─────────────────────────                              │
│ 1st Stop hook: stop_hook_active = false               │
│    └─ Can return decision: "block" to continue        │
│                                                        │
│ 2nd Stop hook (continuation): stop_hook_active = true │
│    └─ Must return exit 0 (allow stop)                 │
│                                                        │
└────────────────────────────────────────────────────────┘
```

---

## Summary: Hook Architecture

**Key takeaways:**

1. **Hooks are deterministic**: They enforce rules, not LLM decisions
2. **Events drive execution**: SessionStart → UserPromptSubmit → PreToolUse → ... → SessionEnd
3. **Matchers filter hooks**: Tool names (PreToolUse), session sources (SessionStart), etc.
4. **Exit codes control flow**: 0 = allow, 2 = deny, other = non-blocking error
5. **JSON output for flexibility**: Structured decisions beyond just allow/deny
6. **Configuration merges**: Hooks from multiple sources combine, duplicates deduplicated
7. **Async support**: Non-blocking background tasks for logging, testing, etc.
8. **Three handler types**: Command (scripts), Prompt (LLM judgment), Agent (complex verification)

