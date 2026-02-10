#!/bin/bash
# Hook: design-system-sync.sh
# Triggers when edge system files or taxonomy/visual-encoding are modified
#
# Validates synchronization between:
# - taxonomy.yaml (arc families, colors)
# - visual-encoding.yaml (icons, visual rules)
# - TypeScript edge system (effects, palettes, themes)
# - Rust TUI theme

# Files that trigger this hook:
# - packages/core/models/taxonomy.yaml
# - packages/core/models/visual-encoding.yaml
# - apps/studio/src/components/graph/edges/system/*.ts
# - tools/novanet/src/tui/theme.rs

EDGE_SYSTEM_PATTERNS=(
  "taxonomy.yaml"
  "visual-encoding.yaml"
  "edges/system/"
  "tui/theme.rs"
)

# Check if any edited file matches our patterns
MATCH=false
for pattern in "${EDGE_SYSTEM_PATTERNS[@]}"; do
  if echo "$CLAUDE_EDITED_FILES" | grep -q "$pattern"; then
    MATCH=true
    break
  fi
done

if [ "$MATCH" = true ]; then
  echo ""
  echo "┌────────────────────────────────────────────────────────────────┐"
  echo "│  🎨 DESIGN SYSTEM SYNC REMINDER                               │"
  echo "├────────────────────────────────────────────────────────────────┤"
  echo "│                                                                │"
  echo "│  You edited files that affect the design system.              │"
  echo "│                                                                │"
  echo "│  Run validation before committing:                            │"
  echo "│                                                                │"
  echo "│    node tools/scripts/validate-design-system.mjs              │"
  echo "│                                                                │"
  echo "│  This checks:                                                  │"
  echo "│  • Arc families: taxonomy.yaml ↔ TypeScript                   │"
  echo "│  • Colors: consistent across all platforms                    │"
  echo "│  • Effects: v11.6.1 arc family primitives                     │"
  echo "│  • Icons: web (Lucide) + terminal (Unicode)                   │"
  echo "│  • Registry: effect injection                                 │"
  echo "│  • TUI: Rust theme sync                                       │"
  echo "│                                                                │"
  echo "└────────────────────────────────────────────────────────────────┘"
  echo ""
fi
