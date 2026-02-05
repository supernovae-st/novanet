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
    echo "YAML_MODEL_CHANGE_DETECTED"
    echo ""
    echo "You modified a YAML model file: $(basename "$FILE_PATH")"
    echo ""
    echo "IMPORTANT: Regenerate artifacts with:"
    echo "  cargo run --manifest-path tools/novanet/Cargo.toml -- schema generate"
    echo ""
    echo "Or use pnpm:"
    echo "  pnpm schema:generate"
    echo ""
    echo "Files that will be regenerated:"
    echo "  - packages/core/src/graph/layers.ts"
    echo "  - packages/core/src/graph/hierarchy.ts"
    echo "  - packages/core/src/graph/visual-encoding.ts"
    echo "  - packages/db/seed/*.cypher"
    echo "  - apps/studio/src/design/colors/generated.ts"
    ;;
  *)
    exit 0
    ;;
esac
