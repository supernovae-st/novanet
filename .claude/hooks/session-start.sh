#!/bin/bash
# NovaNet Session Start Hook
# Displays project status and version info

set -e

# Get current directory
cd "$(dirname "$0")/../.."

# Get version from package.json
VERSION=$(jq -r '.version // "unknown"' packages/core/package.json 2>/dev/null || echo "unknown")

# Get git branch
BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

# Get uncommitted changes count
CHANGES=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')

# Output status (shown to Claude as context)
cat << EOF
NovaNet v${VERSION} | Branch: ${BRANCH} | Uncommitted: ${CHANGES} files
EOF
