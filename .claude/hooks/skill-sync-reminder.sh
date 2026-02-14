#!/bin/bash
# Claude Code Hook: Skill Sync Reminder
# Triggered when editing skill, command, or agent files
# Reminds to verify against YAML sources of truth

set -e

# Read hook input from stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)

if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Get project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Read current counts from YAML
NODE_COUNT=$(find "$PROJECT_ROOT/packages/core/models/node-classes" -name "*.yaml" 2>/dev/null | wc -l | tr -d ' ')
ARC_COUNT=$(find "$PROJECT_ROOT/packages/core/models/arc-classes" -name "*.yaml" 2>/dev/null | wc -l | tr -d ' ')
VERSION=$(cat "$PROJECT_ROOT/VERSION" 2>/dev/null || echo "unknown")

# Check if this is a skill, command, or agent file
case "$FILE_PATH" in
  *.claude/skills/*|*.claude/commands/*|*.claude/agents/*|*.claude/rules/*)
    cat << EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PostToolUse",
    "additionalContext": "Skill/Command/Rule modified. YAML Source of Truth: $NODE_COUNT node-classes, $ARC_COUNT arc-classes, version $VERSION. Verify paths use 'node-classes/' not 'nodes/', and 'taxonomy.yaml' not 'organizing-principles.yaml'. Run 'pnpm skill:audit' to validate."
  }
}
EOF
    ;;
  *)
    exit 0
    ;;
esac
