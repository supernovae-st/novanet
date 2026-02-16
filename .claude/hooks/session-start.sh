#!/bin/bash
# NovaNet Session Start Hook
# Displays project status, version info, and DX reminders

set -e

# Get current directory
cd "$(dirname "$0")/../.."

# Get version from VERSION file (single source of truth)
VERSION=$(cat VERSION 2>/dev/null || echo "unknown")

# Get git branch
BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

# Get uncommitted changes count
CHANGES=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')

# Check if schema validation is needed (YAML files modified)
YAML_MODIFIED=$(git status --porcelain 2>/dev/null | grep -c '\.yaml$' || echo "0")

# Output status (shown to Claude as context)
cat << EOF
NovaNet v${VERSION} | Branch: ${BRANCH} | Uncommitted: ${CHANGES} files
EOF

# Schema sync reminder if YAML was modified
if [ "$YAML_MODIFIED" -gt 0 ]; then
    echo "⚠️  YAML files modified - run 'cargo run -- schema validate' before committing"
fi

# DX Quick Reference
cat << 'DX_TIPS'

**Quick Commands:**
- `/novanet-arch` — Architecture diagrams
- `/adr <N>` — ADR lookup (fuzzy search)
- `/schema:add-node` — Add node type
- `cargo run -- schema validate` — Validate YAML

**ADR Must-Know (v0.13.0):**
- ADR-029: *Native pattern (EntityNative, PageNative)
- ADR-030: Slug ownership (Page owns URL)
- ADR-024: Trait = Data Origin (defined/authored/imported/generated/retrieved)
DX_TIPS
