#!/bin/bash
# Claude Code Hook: YAML Sync Reminder
# Triggered when editing YAML model files
# Reminds to regenerate artifacts

set -e

# Read hook input from stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)

if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Check if this is a YAML model file
case "$FILE_PATH" in
  *packages/core/models/*.yaml|*packages/core/models/**/*.yaml)
    cat << EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "YAML model modified. Regenerate: cargo run -- schema generate (tools/novanet)"
  }
}
EOF
    ;;
  *)
    exit 0
    ;;
esac
