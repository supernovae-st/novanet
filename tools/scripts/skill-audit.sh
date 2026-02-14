#!/bin/bash
# NovaNet Skill & Documentation Audit Script
# Validates that skills, commands, rules, and docs follow YAML sources of truth
# Usage: ./tools/scripts/skill-audit.sh [--yaml-check]

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

echo -e "${BLUE}NovaNet Skill & Documentation Audit${NC}"
echo "========================================"
echo "Version: $VERSION"
echo ""

# Print taxonomy summary (sets NODE_COUNT, ARC_COUNT, etc.)
print_taxonomy_summary

# Optional: Validate YAML syntax
if [ "$YAML_CHECK" = true ]; then
  echo -e "${BLUE}Validating YAML syntax...${NC}"
  validate_yaml_directory "packages/core/models/node-classes" "Node classes"
  validate_yaml_directory "packages/core/models/arc-classes" "Arc classes"
  echo ""
fi

# Function to check a file for issues
check_file() {
  local file="$1"
  local file_issues=0

  if [ ! -f "$file" ]; then
    return 0
  fi

  # Check for deprecated YAML paths
  if grep -qE "nodes/\s*←|nodes/.*\.yaml|models/nodes/" "$file" 2>/dev/null; then
    # Exclude historical notes and migration documentation
    if ! grep -E "nodes/\s*←|nodes/.*\.yaml|models/nodes/" "$file" 2>/dev/null | grep -qE "→|->|renamed|was |former|deprecated|legacy"; then
      echo -e "${RED}ERROR${NC}: $file references deprecated 'nodes/' path (should be 'node-classes/')"
      ((ISSUES++))
      file_issues=1
    fi
  fi

  # Check for deprecated organizing-principles.yaml reference
  if grep -qE "organizing-principles\.yaml" "$file" 2>/dev/null; then
    if ! grep -E "organizing-principles\.yaml" "$file" 2>/dev/null | grep -qE "→|->|renamed|was |former|deprecated|legacy|\[→"; then
      echo -e "${RED}ERROR${NC}: $file references deprecated 'organizing-principles.yaml' (should be 'taxonomy.yaml')"
      ((ISSUES++))
      file_issues=1
    fi
  fi

  # Check for correct source structure references
  if grep -qE "\b(42|43|44|45|46|59|60)\s*(nodes?|Classes?|NodeClasses?|fichiers)" "$file" 2>/dev/null; then
    local actual_in_file
    actual_in_file=$(grep -oE "\b(42|43|44|45|46|59|60)\s*(nodes?|Classes?|NodeClasses?|fichiers)" "$file" 2>/dev/null | head -1 | grep -oE "^[0-9]+")
    if [ "$actual_in_file" != "$NODE_COUNT" ] 2>/dev/null; then
      echo -e "${YELLOW}WARN${NC}: $file mentions ${actual_in_file:-?} nodes (YAML has $NODE_COUNT)"
      ((WARNINGS++))
      file_issues=1
    fi
  fi

  # Check for deprecated v9 references when we're on v10.5
  if grep -qE "v9\.(0|5).*current|current.*v9\.(0|5)" "$file" 2>/dev/null; then
    echo -e "${YELLOW}WARN${NC}: $file marks v9.x as 'current' (should be v$VERSION)"
    ((WARNINGS++))
    file_issues=1
  fi

  # Check for deprecated Concept terminology in skills
  if grep -E ":Concept[^A-Za-z]|USES_CONCEPT|ConceptL10n|includeConcepts" "$file" 2>/dev/null | grep -vqE "→|->|renamed|was |former|deprecated"; then
    echo -e "${RED}ERROR${NC}: $file uses deprecated 'Concept' terminology (should be 'Entity')"
    ((ISSUES++))
    file_issues=1
  fi

  return $file_issues
}

echo -e "${BLUE}Checking skills...${NC}"
for file in $(find .claude/skills -name "*.md" -type f 2>/dev/null); do
  check_file "$file"
done

echo ""
echo -e "${BLUE}Checking commands...${NC}"
for file in $(find .claude/commands -name "*.md" -type f 2>/dev/null); do
  check_file "$file"
done

echo ""
echo -e "${BLUE}Checking rules...${NC}"
for file in $(find .claude/rules -name "*.md" -type f 2>/dev/null); do
  check_file "$file"
done

echo ""
echo -e "${BLUE}Checking agents...${NC}"
for file in $(find .claude/agents -name "*.md" -type f 2>/dev/null); do
  check_file "$file"
done

echo ""
echo -e "${BLUE}Checking app-specific skills...${NC}"
for file in $(find apps -path "*/node_modules" -prune -o -path "*/.claude/skills/*.md" -print 2>/dev/null); do
  check_file "$file"
done

# Print final result
print_audit_result "Skill Audit"
exit_code=$?

if [ $exit_code -ne 0 ]; then
  echo ""
  echo "Source of Truth:"
  echo "  packages/core/models/"
  echo "  ├── node-classes/       ← $NODE_COUNT NodeClass YAMLs"
  echo "  ├── arc-classes/        ← $ARC_COUNT ArcClass YAMLs"
  echo "  ├── taxonomy.yaml     ← Realms, Layers, Traits, ArcFamilies"
  echo "  └── relations.yaml    ← (deprecated, use arc-classes/)"
  echo ""
  echo "To fix:"
  echo "  - 'nodes/' → 'node-classes/'"
  echo "  - 'organizing-principles.yaml' → 'taxonomy.yaml'"
  echo "  - 'Concept' → 'Entity'"
  echo "  - 'USES_CONCEPT' → 'USES_ENTITY'"
fi

exit $exit_code
