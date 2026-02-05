#!/bin/bash
# NovaNet Documentation Audit Script
# Checks for consistency across all documentation files
# Usage: ./tools/scripts/doc-audit.sh [--fix]

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT"

# Read expected values from source of truth
VERSION=$(cat VERSION 2>/dev/null || echo "unknown")
NODE_COUNT=42
ARC_COUNT=77
REALM_COUNT=2
LAYER_COUNT=8

echo -e "${BLUE}NovaNet Documentation Audit${NC}"
echo "================================"
echo "Version: $VERSION"
echo "Expected: $NODE_COUNT nodes, $ARC_COUNT arcs, $REALM_COUNT realms, $LAYER_COUNT layers"
echo ""

ISSUES=0
WARNINGS=0

# Function to check a file for issues
check_file() {
  local file="$1"
  local file_issues=0

  if [ ! -f "$file" ]; then
    return
  fi

  # Check for outdated version references (Current version: vX.X, Version: vX.X patterns)
  # Skip historical references like "v9.0.0 introduced" or GitHub Milestones
  if grep -qE "(Current [Vv]ersion|[Vv]ersion:).*v10\.[0-3]" "$file" 2>/dev/null; then
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

  # Check for old realm references (shared realm was removed)
  # Exclude: file paths, migration docs (merged, removed, "into global"), doc-audit itself
  if grep -E "'shared'|\"shared\"|realm.*shared|SHARED.*realm" "$file" 2>/dev/null | grep -vqE "shared-layer|models/shared|merged|removed|into global|doc-audit"; then
    echo -e "${YELLOW}WARN${NC}: $file may reference removed 'shared' realm"
    ((WARNINGS++))
    file_issues=1
  fi

  # Check node counts (skip historical references in ROADMAP.md and CHANGELOG.md)
  if [[ "$file" != *"ROADMAP.md"* ]] && [[ "$file" != *"CHANGELOG.md"* ]]; then
    if grep -qE "4[4-6]\s*(nodes?|Kinds?|NodeKinds?)" "$file" 2>/dev/null; then
      echo -e "${YELLOW}WARN${NC}: $file may have outdated node count (should be $NODE_COUNT)"
      ((WARNINGS++))
      file_issues=1
    fi

    # Check arc counts
    if grep -qE "(6[7-9]|7[0-6]|8[0-3])\s*(arcs?|ArcKinds?|relations?)" "$file" 2>/dev/null; then
      echo -e "${YELLOW}WARN${NC}: $file may have outdated arc count (should be $ARC_COUNT)"
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

echo ""
echo "================================"
if [ $ISSUES -eq 0 ] && [ $WARNINGS -eq 0 ]; then
  echo -e "${GREEN}All documentation is consistent!${NC}"
  exit 0
elif [ $ISSUES -eq 0 ]; then
  echo -e "${YELLOW}$WARNINGS warning(s) found${NC}"
  echo "Run with --verbose for details"
  exit 0
else
  echo -e "${RED}$ISSUES error(s), $WARNINGS warning(s) found${NC}"
  echo ""
  echo "To fix terminology issues:"
  echo "  - Concept -> Entity"
  echo "  - USES_CONCEPT -> USES_ENTITY"
  echo "  - ConceptL10n -> EntityL10n"
  echo "  - shared realm removed (use global or project)"
  echo ""
  echo "Expected counts: $NODE_COUNT nodes, $ARC_COUNT arcs, $REALM_COUNT realms, $LAYER_COUNT layers"
  exit 1
fi
