#!/usr/bin/env bash
# migrate-schema-format.sh — List files needing v0.13.1 BLOC format migration

set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

echo "=== Files needing v0.13.1 BLOC format migration ==="
echo ""

count=0
for f in $(find packages/core/models/node-classes -name "*.yaml" | sort); do
  if ! grep -q "# BLOC 1:" "$f" 2>/dev/null; then
    echo "  $(basename "$f" .yaml)"
    ((count++)) || true
  fi
done

echo ""
echo "Total: $count files"
