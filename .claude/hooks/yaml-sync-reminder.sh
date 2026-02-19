#!/bin/bash
# Claude Code Hook: YAML Sync Enforcement
# Triggered when editing YAML model files
# STRONGLY reminds to regenerate artifacts with validation

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
    # Track modified YAML files for batch validation
    TRACKING_FILE="/tmp/novanet-yaml-modified.txt"
    echo "$FILE_PATH" >> "$TRACKING_FILE"

    cat << 'EOF'
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "⚠️ SCHEMA SYNC REQUIRED\n\nYAML model modified. You MUST run:\n\n  cd novanet-dev/tools/novanet\n  cargo run -- schema validate\n  cargo run -- schema generate\n\nDO NOT commit without validating and regenerating artifacts.\nThis ensures TypeScript, Cypher, and Mermaid stay in sync."
  }
}
EOF
    ;;
  *)
    exit 0
    ;;
esac
