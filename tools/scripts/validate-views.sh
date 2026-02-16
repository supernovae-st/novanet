#!/usr/bin/env bash
# tools/scripts/validate-views.sh
# Cross-validate views.yaml between Rust and TypeScript parsers

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "=== Views Cross-Validation ==="

# Export from Rust
echo "Exporting from Rust..."
RUST_JSON=$(cd "$ROOT_DIR/tools/novanet" && cargo run -q -- views export 2>/dev/null)
RUST_COUNT=$(echo "$RUST_JSON" | grep -o '"count": [0-9]*' | grep -o '[0-9]*')
echo "✓ Rust: $RUST_COUNT views"

# Export from TypeScript
echo "Exporting from TypeScript..."
TS_JSON=$(node "$ROOT_DIR/packages/core/scripts/export-views.mjs")
TS_COUNT=$(echo "$TS_JSON" | grep -o '"count": [0-9]*' | grep -o '[0-9]*')
echo "✓ TypeScript: $TS_COUNT views"

# Compare
if [ "$RUST_JSON" = "$TS_JSON" ]; then
    echo "✓ All views match!"
    exit 0
else
    echo "✗ Views mismatch!"
    echo ""
    echo "Diff (Rust vs TypeScript):"
    diff <(echo "$RUST_JSON") <(echo "$TS_JSON") || true
    exit 1
fi
