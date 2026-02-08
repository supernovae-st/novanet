#!/bin/bash
# NovaNet Documentation Audit Script
# Checks for consistency across all documentation files
# Usage: ./tools/scripts/doc-audit.sh [--yaml-check]

set -e

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT"

# Source shared library
source "$SCRIPT_DIR/lib/audit-common.sh"

# Parse arguments
YAML_CHECK=false
VERBOSE=false
for arg in "$@"; do
  case $arg in
    --yaml-check) YAML_CHECK=true ;;
    --verbose|-v) VERBOSE=true ;;
  esac
done

VERSION=$(get_version)

echo -e "${BLUE}NovaNet Documentation Audit${NC}"
echo "================================"
echo "Version: $VERSION"
echo ""

# Print taxonomy summary (sets NODE_COUNT, ARC_COUNT, etc.)
print_taxonomy_summary

# Optional: Validate YAML syntax
if [ "$YAML_CHECK" = true ]; then
  echo -e "${BLUE}Validating YAML syntax...${NC}"
  validate_yaml_directory "packages/core/models/node-kinds" "Node-kinds"
  validate_yaml_directory "packages/core/models/arc-kinds" "Arc-kinds"
  validate_yaml_directory "packages/core/models/meta" "Meta definitions"
  validate_yaml_directory "packages/core/models/views" "View definitions"
  validate_yaml_syntax "packages/core/models/taxonomy.yaml" && \
    echo -e "${GREEN}OK${NC}: taxonomy.yaml" || \
    (echo -e "${RED}ERROR${NC}: taxonomy.yaml invalid" && ((ISSUES++)))
  echo ""
fi

# Function to check a file for issues
check_file() {
  local file="$1"
  local file_issues=0

  if [ ! -f "$file" ]; then
    return
  fi

  # Check for outdated version references (Current version: vX.X, Version: vX.X patterns)
  # Skip historical references like "v9.0.0 introduced" or GitHub Milestones
  if grep -qE "(Current [Vv]ersion|[Vv]ersion:).*v10\.[0-4]" "$file" 2>/dev/null; then
    echo -e "${YELLOW}WARN${NC}: $file contains outdated current version (should be v$VERSION)"
    ((WARNINGS++))
    file_issues=1
  fi

  # Check for deprecated terminology (exclude migration documentation with → arrows)
  if grep -E ":Concept[^A-Za-z]|USES_CONCEPT|ConceptL10n" "$file" 2>/dev/null | grep -vqE "→|->|renamed|was |former"; then
    echo -e "${RED}ERROR${NC}: $file uses deprecated 'Concept' terminology (should be 'Entity')"
    ((ISSUES++))
    file_issues=1
  fi

  # Check for old realm references (shared realm was removed in v10.3)
  # Exclude: file paths, migration docs (merged, removed, "into global"), doc-audit itself
  if grep -E "'shared'|\"shared\"|realm.*shared|SHARED.*realm" "$file" 2>/dev/null | grep -vqE "shared-layer|models/shared|merged|removed|into global|doc-audit"; then
    echo -e "${YELLOW}WARN${NC}: $file may reference removed 'shared' realm"
    ((WARNINGS++))
    file_issues=1
  fi

  # Check node counts (skip historical references in ROADMAP.md and CHANGELOG.md)
  # Only match TOTAL counts, not per-realm/layer counts
  if [[ "$file" != *"ROADMAP.md"* ]] && [[ "$file" != *"CHANGELOG.md"* ]]; then
    # Match total patterns: "XX node types", "XX Kind Types", "Total: XX nodes", "XX nodes,"
    local found_node_count
    found_node_count=$(grep -oE "([0-9]+\s*(node types|Kind Types|Kinds)|\bTotal:?\s*[0-9]+\s*nodes|[0-9]+\s*nodes,)" "$file" 2>/dev/null | grep -oE "[0-9]+" | head -1)
    if [ -n "$found_node_count" ] && [ "$found_node_count" != "$NODE_COUNT" ] 2>/dev/null; then
      echo -e "${YELLOW}WARN${NC}: $file has node count $found_node_count (should be $NODE_COUNT)"
      ((WARNINGS++))
      file_issues=1
    fi

    # Check arc counts - match total patterns: "XX arcs", "XX ArcKinds"
    local found_arc_count
    found_arc_count=$(grep -oE "([0-9]+\s*(arcs|ArcKinds))" "$file" 2>/dev/null | grep -oE "[0-9]+" | head -1)
    if [ -n "$found_arc_count" ] && [ "$found_arc_count" != "$ARC_COUNT" ] 2>/dev/null; then
      echo -e "${YELLOW}WARN${NC}: $file has arc count $found_arc_count (should be $ARC_COUNT)"
      ((WARNINGS++))
      file_issues=1
    fi
  fi

  return $file_issues
}

echo -e "${BLUE}Checking documentation files...${NC}"
echo ""

# Check main docs
for file in README.md CLAUDE.md ROADMAP.md CHANGELOG.md; do
  check_file "$file"
done

# Check .claude directory
for file in $(find .claude -name "*.md" -type f 2>/dev/null); do
  check_file "$file"
done

# Check package docs
for file in $(find packages -name "CLAUDE.md" -o -name "README.md" 2>/dev/null); do
  check_file "$file"
done

# Check apps docs
for file in $(find apps -path "*/node_modules" -prune -o -name "*.md" -print 2>/dev/null | grep -E "(CLAUDE|README|skills)" | head -50); do
  check_file "$file"
done

# Check Rust docs
for file in tools/novanet/CLAUDE.md tools/novanet/README.md tools/novanet/KEYBINDINGS.md; do
  check_file "$file"
done

# Print final result
print_audit_result "Documentation Audit"
exit_code=$?

if [ $exit_code -ne 0 ]; then
  echo ""
  echo "To fix terminology issues:"
  echo "  - Concept -> Entity"
  echo "  - USES_CONCEPT -> USES_ENTITY"
  echo "  - EntityL10n -> EntityContent (v10.9)"
  echo "  - PageL10n -> PageGenerated (v10.9)"
  echo "  - BlockL10n -> BlockGenerated (v10.9)"
  echo "  - shared realm removed (use global or tenant)"
  echo ""
  echo "Expected counts: $NODE_COUNT nodes, $ARC_COUNT arcs, $REALM_COUNT realms, $LAYER_COUNT layers"
fi

exit $exit_code
