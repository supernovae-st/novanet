#!/bin/bash
# NovaNet Post-Edit Format Hook
# Auto-formats files after Write/Edit operations
# Receives JSON input via stdin with tool_input.file_path

set -e

# Read JSON input from stdin
INPUT=$(cat)

# Extract file path from tool input
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // .tool_input.filePath // empty' 2>/dev/null)

if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Get file extension
EXT="${FILE_PATH##*.}"

# Format based on file type (silent, non-blocking)
case "$EXT" in
  rs)
    # Rust files - run cargo fmt on the file
    if command -v rustfmt &> /dev/null; then
      rustfmt --edition 2024 "$FILE_PATH" 2>/dev/null || true
    fi
    ;;
  ts|tsx|js|jsx|json)
    # TypeScript/JavaScript - run prettier if available
    if [ -f "node_modules/.bin/prettier" ]; then
      node_modules/.bin/prettier --write "$FILE_PATH" 2>/dev/null || true
    fi
    ;;
esac

exit 0
