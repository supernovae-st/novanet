#!/bin/bash
# Hook: Provide context when reading ADR files
# Triggered on: Read of .claude/rules/adr/**/*.md files

FILE_PATH="$CLAUDE_FILE_PATH"

# Skip if not an ADR file
if [[ ! "$FILE_PATH" =~ \.claude/rules/adr/.*\.md$ ]]; then
    exit 0
fi

# Extract ADR number from filename
ADR_NUM=$(echo "$FILE_PATH" | grep -oE 'adr-0*([0-9]+)' | grep -oE '[0-9]+' | sed 's/^0*//')

if [[ -z "$ADR_NUM" ]]; then
    exit 0
fi

# Read the index to find related ADRs
INDEX_FILE="$(dirname "$FILE_PATH")/../_index.yaml"
if [[ ! -f "$INDEX_FILE" ]]; then
    INDEX_FILE="$(dirname "$FILE_PATH")/../../_index.yaml"
fi

# Find dependencies and supersession info
DEPS=$(grep -A 1 "id: $ADR_NUM" "$INDEX_FILE" 2>/dev/null | grep "depends_on:" | sed 's/.*\[//' | sed 's/\].*//')
SUPERSEDES=$(grep -A 10 "id: $ADR_NUM" "$INDEX_FILE" 2>/dev/null | grep "supersedes:" | head -1 | awk '{print $2}')
SUPERSEDED_BY=$(grep -A 10 "id: $ADR_NUM" "$INDEX_FILE" 2>/dev/null | grep "superseded_by:" | head -1 | awk '{print $2}')

# Build context message
MSG=""

if [[ -n "$DEPS" && "$DEPS" != "[]" ]]; then
    MSG="📚 **Related ADRs**: This ADR depends on: $DEPS"
fi

if [[ -n "$SUPERSEDES" && "$SUPERSEDES" != "null" ]]; then
    MSG="${MSG:+$MSG | }⚠️ **Supersedes**: ADR-$SUPERSEDES (check deprecated/ for history)"
fi

if [[ -n "$SUPERSEDED_BY" && "$SUPERSEDED_BY" != "null" ]]; then
    MSG="${MSG:+$MSG | }🔄 **Superseded by**: ADR-$SUPERSEDED_BY (use that instead!)"
fi

if [[ -n "$MSG" ]]; then
    echo "$MSG"
fi
