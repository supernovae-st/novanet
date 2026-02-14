#!/bin/bash
# Claude Code Hook: Documentation Sync Reminder
# Triggered when editing documentation files
# Reminds to keep version numbers and counts consistent

set -e

# Read hook input from stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)

if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Get project root
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(pwd)}"
VERSION=$(cat "$PROJECT_ROOT/VERSION" 2>/dev/null || echo "10.6.0")

# Check if this is a documentation file that needs consistency
case "$FILE_PATH" in
  *CLAUDE.md|*README.md|*.claude/*)
    # Output reminder as additionalContext
    cat << EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Documentation file edited. Ensure version ($VERSION) and counts (61 nodes, 128 arcs, 2 realms, 10 layers) are consistent. Run 'pnpm doc:audit' to verify."
  }
}
EOF
    ;;
  *)
    exit 0
    ;;
esac
