#!/bin/bash
# Claude Code Hook: Query-First Architecture Reminder
# Triggered when editing view-related files
# Reminds about Query-First principles (ADR-021)

set -e

# Read hook input from stdin
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)

if [ -z "$FILE_PATH" ]; then
  exit 0
fi

# Check if this is a view-related file
case "$FILE_PATH" in
  *viewQueries.ts|*ViewQueries.ts)
    echo "DEPRECATED_VIEW_QUERIES_DETECTED"
    echo ""
    echo "You are editing viewQueries.ts which is DEPRECATED."
    echo ""
    echo "QUERY-FIRST ARCHITECTURE (ADR-021):"
    echo "  - All views should be defined in YAML files"
    echo "  - Location: packages/core/models/views/*.yaml"
    echo "  - No hardcoded Cypher queries in TypeScript"
    echo ""
    echo "To migrate a view to YAML:"
    echo "  1. Create packages/core/models/views/contextual/<view-name>.yaml"
    echo "  2. Add entry to packages/core/models/views/_registry.yaml"
    echo "  3. Remove the view from viewQueries.ts"
    echo ""
    echo "Reference: .claude/rules/novanet-decisions.md (ADR-021)"
    ;;
  *packages/core/models/views/*.yaml)
    echo "VIEW_YAML_EDIT_DETECTED"
    echo ""
    echo "You are editing a view YAML file: $(basename "$FILE_PATH")"
    echo ""
    echo "QUERY-FIRST CHECKLIST:"
    echo "  [ ] View registered in _registry.yaml?"
    echo "  [ ] Cypher query tested against Neo4j?"
    echo "  [ ] Parameters documented (e.g., \$nodeKey)?"
    echo "  [ ] Category set correctly (global/contextual/generation/mining)?"
    echo "  [ ] applicable_types set if contextual?"
    echo ""
    echo "To test the view:"
    echo "  1. Start Studio: pnpm dev"
    echo "  2. Open ViewPicker"
    echo "  3. Click the view to auto-execute"
    echo "  4. Ctrl+click to edit before running"
    ;;
  *viewStore.ts|*queryStore.ts|*graphStore.ts)
    echo "STORE_EDIT_DETECTED"
    echo ""
    echo "You are editing a Zustand store: $(basename "$FILE_PATH")"
    echo ""
    echo "QUERY-FIRST STORE RESPONSIBILITIES:"
    echo "  - viewStore: Active view, params, executeView(), loadQueryOnly()"
    echo "  - queryStore: Current cypher, execution state, results"
    echo "  - graphStore: Nodes, edges (populated by query results)"
    echo ""
    echo "IMPORTANT: Graph should ONLY display what the query returns."
    echo "No hidden filtering or mode-specific logic in stores."
    echo ""
    echo "Reference: .claude/rules/novanet-decisions.md (ADR-021)"
    ;;
  *)
    exit 0
    ;;
esac
