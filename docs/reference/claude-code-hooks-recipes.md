# Claude Code Hooks: Quick Reference & Recipes

Practical, copy-paste-ready hook configurations for common use cases.

---

## Quick Configuration Template

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "tool_name_regex",
        "hooks": [
          {
            "type": "command|prompt|agent",
            "command": "/path/to/script.sh",
            "timeout": 600,
            "async": false
          }
        ]
      }
    ]
  },
  "disableAllHooks": false
}
```

---

## Recipe 1: Block Dangerous Commands

**Use case**: Prevent `rm -rf /`, destructive SQL, etc.

`.claude/hooks/block-dangerous.sh`:
```bash
#!/bin/bash
INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

DANGEROUS=(
  "rm -rf /"
  "dd if=/dev/zero"
  "DROP TABLE"
  "DELETE FROM"
  "curl.*-X.*DELETE"
)

for pattern in "${DANGEROUS[@]}"; do
  if [[ "$CMD" =~ $pattern ]]; then
    echo "BLOCKED: $pattern" >&2
    exit 2
  fi
done

exit 0
```

`.claude/settings.json`:
```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/block-dangerous.sh"
          }
        ]
      }
    ]
  }
}
```

---

## Recipe 2: Protect Files

**Use case**: Prevent modification of `.env`, secrets, locks

`.claude/hooks/protect-files.sh`:
```bash
#!/bin/bash
INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Match patterns, not full paths
if [[ "$FILE" == *".env"* ]] || \
   [[ "$FILE" == *"package-lock.json"* ]] || \
   [[ "$FILE" == *".git/"* ]] || \
   [[ "$FILE" == *".ssh"* ]] || \
   [[ "$FILE" == *"credentials"* ]] || \
   [[ "$FILE" == *"secrets"* ]]; then
  echo "Protected: cannot modify $FILE" >&2
  exit 2
fi

exit 0
```

`.claude/settings.json`:
```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Write|Edit|Delete",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/protect-files.sh"
          }
        ]
      }
    ]
  }
}
```

---

## Recipe 3: Auto-Format Code

**Use case**: Run Prettier/rustfmt/go fmt after edits

`.claude/settings.json`:
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write 2>/dev/null || true"
          }
        ]
      }
    ]
  }
}
```

**For Rust**:
```json
{
  "type": "command",
  "command": "jq -r '.tool_input.file_path' | xargs rustfmt 2>/dev/null || true"
}
```

---

## Recipe 4: Log All Tool Usage

**Use case**: Audit trail, compliance, debugging

`.claude/hooks/audit-log.sh`:
```bash
#!/bin/bash
INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name')
TIMESTAMP=$(date -u +'%Y-%m-%dT%H:%M:%SZ')
SESSION=$(echo "$INPUT" | jq -r '.session_id')

# Log to file
echo "[$TIMESTAMP] $SESSION: $TOOL" >> ~/.claude/audit.log

exit 0
```

`.claude/settings.json`:
```json
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/audit-log.sh"
          }
        ]
      }
    ]
  }
}
```

---

## Recipe 5: Require Tests Before Stopping

**Use case**: Ensure test suite passes before Claude stops

`.claude/hooks/verify-tests.sh`:
```bash
#!/bin/bash
INPUT=$(cat)

# Check if already in a Stop continuation to prevent infinite loop
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # Allow stop
fi

# Run tests
if npm test 2>/dev/null; then
  exit 0  # All pass, allow stop
else
  echo "Tests must pass before stopping" >&2
  exit 2  # Block, continue
fi
```

`.claude/settings.json`:
```json
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/verify-tests.sh",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

---

## Recipe 6: Desktop Notifications

**Use case**: Alert when Claude needs input (macOS)

```json
{
  "hooks": {
    "Notification": [
      {
        "matcher": "idle_prompt",
        "hooks": [
          {
            "type": "command",
            "command": "osascript -e 'display notification \"Claude Code waiting for input\" with title \"Claude Code\"'"
          }
        ]
      }
    ]
  }
}
```

**Linux**:
```json
{
  "type": "command",
  "command": "notify-send 'Claude Code' 'Waiting for your input'"
}
```

**Windows**:
```json
{
  "type": "command",
  "command": "powershell -Command \"[System.Windows.Forms.MessageBox]::Show('Claude Code waiting for input', 'Claude Code')\""
}
```

---

## Recipe 7: Auto-Lint Before Stopping

**Use case**: Run ESLint/Clippy and report issues

`.claude/hooks/pre-stop-lint.sh`:
```bash
#!/bin/bash
INPUT=$(cat)

if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0
fi

LINT_OUTPUT=$(npm run lint 2>&1 || true)
LINT_COUNT=$(echo "$LINT_OUTPUT" | grep -c "error" || echo 0)

if [ $LINT_COUNT -gt 0 ]; then
  cat << EOF
{
  "systemMessage": "Lint issues found: $LINT_COUNT errors. Please fix before proceeding.",
  "decision": "block",
  "reason": "Fix lint errors"
}
EOF
  exit 0  # JSON output, don't block
else
  exit 0  # No errors
fi
```

---

## Recipe 8: Inject Context After Compaction

**Use case**: Remind Claude of important rules after context reset

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "cat << 'EOF'\n📋 IMPORTANT REMINDERS:\n- Use pnpm, not npm\n- Run tests before committing\n- Follow ADR-028 for entity architecture\n- Check CLAUDE.md for conventions\n\nRecent commits:\nEOF\ngit log --oneline -5"
          }
        ]
      }
    ]
  }
}
```

---

## Recipe 9: Validate File Paths

**Use case**: Only allow edits in specific directories

`.claude/hooks/validate-paths.sh`:
```bash
#!/bin/bash
INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Allowed: src, docs, tests
# Blocked: everything else
ALLOWED_DIRS=("src/" "docs/" "tests/" ".claude/")

ALLOWED=0
for dir in "${ALLOWED_DIRS[@]}"; do
  if [[ "$FILE" == "$dir"* ]]; then
    ALLOWED=1
    break
  fi
done

if [ $ALLOWED -eq 0 ]; then
  echo "Cannot edit $FILE (allowed: src, docs, tests)" >&2
  exit 2
fi

exit 0
```

---

## Recipe 10: Auto-Update Timestamps

**Use case**: Update modified_at in JSON files after writes

`.claude/hooks/update-timestamps.sh`:
```bash
#!/bin/bash
INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Only for JSON files
if [[ ! "$FILE" =~ \.json$ ]]; then
  exit 0
fi

# Update timestamp in file
jq --arg ts "$(date -u +'%Y-%m-%dT%H:%M:%SZ')" \
   '.modified_at = $ts' \
   "$FILE" > "$FILE.tmp" && mv "$FILE.tmp" "$FILE"

exit 0
```

---

## Recipe 11: Multi-Hook Chain (Security → Lint → Format)

**Use case**: Apply comprehensive validation

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/protect-files.sh"
          },
          {
            "type": "command",
            "command": "npx eslint --fix $(jq -r '.tool_input.file_path')"
          },
          {
            "type": "command",
            "command": "npx prettier --write $(jq -r '.tool_input.file_path')"
          }
        ]
      }
    ]
  }
}
```

**Execution order**: protect-files → eslint → prettier (in sequence)

---

## Recipe 12: Permission Decision with Structured Output

**Use case**: Auto-approve safe operations, ask for others

`.claude/hooks/permission-logic.sh`:
```bash
#!/bin/bash
INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name')

case "$TOOL" in
  Read|Glob|Grep)
    # Auto-approve read-only tools
    cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "permissionDecisionReason": "Read-only tool auto-approved"
  }
}
EOF
    exit 0
    ;;
  Write|Edit)
    # Ask for file write operations
    cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "ask",
    "permissionDecisionReason": "User confirmation needed for file modification"
  }
}
EOF
    exit 0
    ;;
  Bash)
    # Ask for Bash (could be dangerous)
    cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "ask"
  }
}
EOF
    exit 0
    ;;
esac
```

---

## Recipe 13: MCP Tool-Specific Hooks

**Use case**: Log all GitHub API calls

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "mcp__github__.*",
        "hooks": [
          {
            "type": "command",
            "command": "echo \"[$(date)] GitHub API: $(jq -r '.tool_name')\" >> ~/.claude/github-api.log"
          }
        ]
      }
    ]
  }
}
```

**Match all MCP tools**:
```json
{
  "matcher": "^mcp__"
}
```

---

## Recipe 14: Dynamic Input Modification

**Use case**: Redirect file paths to sandbox directory

`.claude/hooks/sandbox-redirect.sh`:
```bash
#!/bin/bash
INPUT=$(cat)

if [ "$(echo "$INPUT" | jq -r '.hook_event_name')" != "PreToolUse" ]; then
  exit 0
fi

FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Redirect to sandbox if it's a write
TOOL=$(echo "$INPUT" | jq -r '.tool_name')
if [ "$TOOL" = "Write" ]; then
  SANDBOX_FILE="/tmp/claude-sandbox$FILE"

  cat << EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "allow",
    "updatedInput": {
      "file_path": "$SANDBOX_FILE",
      "content": "$(echo "$INPUT" | jq -r '.tool_input.content')"
    }
  }
}
EOF
  exit 0
fi

exit 0
```

---

## Recipe 15: Git Pre-Commit Hook Integration

**Use case**: Ensure git hooks pass before allowing shell commands

`.claude/hooks/pre-commit-check.sh`:
```bash
#!/bin/bash
INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Only check if trying to commit
if [[ ! "$CMD" =~ "git commit" ]]; then
  exit 0
fi

# Run pre-commit checks
pre-commit run --all-files 2>/dev/null
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
  echo "Pre-commit hooks failed" >&2
  exit 2
fi

exit 0
```

---

## Recipe 16: Conditional Async Logging

**Use case**: Background test run after file save

```json
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "npm test 2>&1 | tail -10 >> ~/.claude/test-runs.log",
            "async": true,
            "timeout": 300
          }
        ]
      }
    ]
  }
}
```

---

## Recipe 17: Rate Limiting Tool Usage

**Use case**: Limit Claude to 5 Bash commands per minute

`.claude/hooks/rate-limit.sh`:
```bash
#!/bin/bash
INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name')

[ "$TOOL" != "Bash" ] && exit 0

# Count Bash calls in last 60 seconds from transcript
TRANSCRIPT=$(echo "$INPUT" | jq -r '.transcript_path')
RECENT_CALLS=$(tail -100 "$TRANSCRIPT" 2>/dev/null | \
  grep -c '"tool_name": "Bash"' || echo 0)

if [ $RECENT_CALLS -gt 5 ]; then
  echo "Rate limit: 5 Bash commands per minute exceeded" >&2
  exit 2
fi

exit 0
```

---

## Recipe 18: MCP-Specific Authorization

**Use case**: Allow only specific MCP servers

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "^mcp__",
        "hooks": [
          {
            "type": "command",
            "command": "$CLAUDE_PROJECT_DIR/.claude/hooks/mcp-authz.sh"
          }
        ]
      }
    ]
  }
}
```

`.claude/hooks/mcp-authz.sh`:
```bash
#!/bin/bash
INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name')

ALLOWED_MCP_SERVERS=("github" "filesystem")

SERVER=$(echo "$TOOL" | cut -d'_' -f3)
ALLOWED=0

for server in "${ALLOWED_MCP_SERVERS[@]}"; do
  if [ "$server" = "$SERVER" ]; then
    ALLOWED=1
    break
  fi
done

if [ $ALLOWED -eq 0 ]; then
  echo "MCP server not authorized: $SERVER" >&2
  exit 2
fi

exit 0
```

---

## Common Matchers Reference

| Pattern | Matches | Example |
|---------|---------|---------|
| `Bash` | Bash tool only | `./cleanup.sh` |
| `Write\|Edit` | File writes | Editing files |
| `Read\|Glob\|Grep` | Read-only | Inspecting code |
| `^mcp__` | Any MCP tool | `mcp__github__search` |
| `mcp__github__.*` | GitHub MCP | `mcp__github__create_pr` |
| `mcp__.*__write.*` | Write tools across MCP | Any MCP write |
| `startup\|resume` | Session startup | SessionStart hook |
| `manual\|auto` | Compaction trigger | PreCompact hook |
| `` (empty) | All occurrences | Every event |

---

## Debugging Hooks

**Check hook status**:
```bash
# In Claude Code CLI
/hooks
```

**Test script manually**:
```bash
echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
echo $?  # Check exit code
```

**View hook output**:
```bash
# In Claude Code, toggle verbose mode
Ctrl+O  # Toggles verbose transcript view
```

**Check logs**:
```bash
# Look for hook execution messages
tail -50 ~/.claude/command-log.txt
cat ~/.claude/audit.log
```

---

## Performance Tips

- Use `-r` flag with jq to avoid shell escaping issues
- Cache computed values in files rather than recalculating
- Use `|| true` to prevent hook failures from blocking
- Set reasonable timeouts (default 600s = 10 minutes)
- Use `async: true` for non-blocking background tasks
- Match tools strictly to avoid unnecessary hook execution

---

## Error Handling Patterns

**Safe JSON fallback**:
```bash
#!/bin/bash
set -e
trap 'echo "Hook error" >&2; exit 1' ERR

INPUT=$(cat) || exit 1
jq -e '.tool_name' <<< "$INPUT" || exit 1

# Your logic
exit 0
```

**Graceful degradation**:
```bash
RESULT=$(my-command 2>/dev/null || echo "default")
echo "$RESULT"
exit 0  # Never fail
```

**Required dependencies**:
```bash
command -v jq &> /dev/null || {
  echo "jq required" >&2
  exit 1
}
```

---

## Environment Variables in Hooks

| Variable | Value | Example |
|----------|-------|---------|
| `$CLAUDE_PROJECT_DIR` | Project root | `$CLAUDE_PROJECT_DIR/.claude` |
| `$CLAUDE_ENV_FILE` | Env file path | Set in SessionStart |
| `$HOME` | User home | `~/.claude` |
| `$PWD` | Current directory | From hook input `cwd` |

---

## Further Reading

- `/hooks` menu in Claude Code CLI
- Official hooks guide: https://code.claude.com/docs/en/hooks-guide
- Hook reference: https://code.claude.com/docs/en/hooks
- Settings docs: https://code.claude.com/docs/en/settings
