#!/bin/bash
# Claude Code Hook: Semantic coherence check for Rust files
# Triggered: PostToolUse on Write|Edit of *.rs files in tools/novanet/
#
# Validates:
# - No deprecated realm names (global/tenant)
# - No edge-family terminology (should be arc-family)
# - No outdated node counts in new/modified content

set -euo pipefail

# Get the file that was edited from environment
FILE="${CLAUDE_FILE_PATH:-}"

if [ -z "$FILE" ]; then
    exit 0
fi

# Only check Rust files in tools/novanet
if [[ ! "$FILE" =~ tools/novanet.*\.rs$ ]]; then
    exit 0
fi

WARNINGS=""

# Check for deprecated realm names
if grep -q "\"global\"\|\"tenant\"\|global.*realm\|tenant.*realm" "$FILE" 2>/dev/null; then
    # Exclude test files and version comments
    if ! grep -q "test\|// v11\|#\[cfg(test)\]" "$FILE" 2>/dev/null; then
        WARNINGS="${WARNINGS}\n⚠️  Deprecated realm name found in $FILE (use 'shared'/'org' instead of 'global'/'tenant')"
    fi
fi

# Check for edge-family terminology
if grep -qi "edge.family\|edge-family" "$FILE" 2>/dev/null; then
    WARNINGS="${WARNINGS}\n⚠️  'edge-family' terminology found in $FILE (use 'arc-family')"
fi

# Check for outdated node counts in doc comments
if grep -E "//.*\b(42|43|44|45)\b.*node" "$FILE" 2>/dev/null | grep -qiv "v10\|v9\|deprecated"; then
    WARNINGS="${WARNINGS}\n⚠️  Possible outdated node count in $FILE (v11.5 has 60 nodes)"
fi

# Output warnings if any
if [ -n "$WARNINGS" ]; then
    echo -e "\n╭─────────────────────────────────────────────────────────────────────────╮"
    echo -e "│  🔍 SEMANTIC CHECK WARNING                                              │"
    echo -e "╰─────────────────────────────────────────────────────────────────────────╯"
    echo -e "$WARNINGS"
    echo -e "\nRun 'cargo make semantic-audit' for full validation."
    echo ""
fi

exit 0
