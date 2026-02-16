#!/bin/bash
# Claude Code Hook: Views Sync Reminder
# Triggered when views.yaml is modified
# Reminds to run cross-validation between Rust and TypeScript

set -e

# Read hook input from stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)

if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Check if this is the views.yaml file
case "$FILE_PATH" in
  *packages/core/models/views.yaml)
    cat << EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "<views-modified>views.yaml was modified. Run validation:\n  novanet views validate\n  # or\n  ./tools/scripts/validate-views.sh</views-modified>"
  }
}
EOF
    ;;
  *)
    exit 0
    ;;
esac
