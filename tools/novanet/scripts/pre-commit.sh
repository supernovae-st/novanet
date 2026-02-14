#!/bin/bash
# Pre-commit validation script for NovaNet Rust codebase
# Install with: ln -sf ../../tools/novanet/scripts/pre-commit.sh .git/hooks/pre-commit
#
# Validates:
# - Code formatting (cargo fmt)
# - Linting (cargo clippy)
# - Tests pass (cargo test)
# - Semantic coherence (v11.5 terminology, counts)

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Find repo root
REPO_ROOT=$(git rev-parse --show-toplevel)
NOVANET_DIR="$REPO_ROOT/tools/novanet"

# Check if Rust files are staged
STAGED_RS=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.rs$' || true)
STAGED_YAML=$(git diff --cached --name-only --diff-filter=ACM | grep -E 'models/.*\.yaml$' || true)

if [ -z "$STAGED_RS" ] && [ -z "$STAGED_YAML" ]; then
    echo -e "${CYAN}No Rust or YAML model files staged, skipping pre-commit checks${NC}"
    exit 0
fi

echo -e "${CYAN}╔═══════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  🔍 NovaNet Pre-Commit Validation                                     ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

cd "$NOVANET_DIR"

# 1. Format check
echo -e "${YELLOW}► Checking formatting...${NC}"
if ! cargo fmt --check >/dev/null 2>&1; then
    echo -e "${RED}  ✗ Formatting issues found. Run 'cargo fmt' to fix.${NC}"
    exit 1
fi
echo -e "${GREEN}  ✓ Formatting OK${NC}"

# 2. Clippy
echo -e "${YELLOW}► Running clippy...${NC}"
if ! cargo clippy --quiet -- -D warnings 2>/dev/null; then
    echo -e "${RED}  ✗ Clippy warnings found. Fix them before committing.${NC}"
    exit 1
fi
echo -e "${GREEN}  ✓ Clippy OK${NC}"

# 3. Quick tests (lib only for speed)
echo -e "${YELLOW}► Running tests...${NC}"
if ! cargo test --lib --quiet 2>/dev/null; then
    echo -e "${RED}  ✗ Tests failed. Fix them before committing.${NC}"
    exit 1
fi
echo -e "${GREEN}  ✓ Tests pass${NC}"

# 4. Semantic checks (only if Rust files are staged)
if [ -n "$STAGED_RS" ]; then
    echo -e "${YELLOW}► Checking semantic coherence...${NC}"

    # Check for deprecated terms in staged files only
    DEPRECATED=""
    for file in $STAGED_RS; do
        if [ -f "$REPO_ROOT/$file" ]; then
            if grep -q '"global"\|"tenant"' "$REPO_ROOT/$file" 2>/dev/null; then
                if ! grep -q "test\|// v11" "$REPO_ROOT/$file" 2>/dev/null; then
                    DEPRECATED="$DEPRECATED\n  - $file"
                fi
            fi
        fi
    done

    if [ -n "$DEPRECATED" ]; then
        echo -e "${YELLOW}  ⚠ Deprecated realm names found:${NC}"
        echo -e "$DEPRECATED"
        echo -e "${YELLOW}  Consider using 'shared'/'org' instead of 'global'/'tenant'${NC}"
    fi

    # Check for edge-family terminology
    for file in $STAGED_RS; do
        if [ -f "$REPO_ROOT/$file" ]; then
            if grep -qi "edge.family\|edge-family" "$REPO_ROOT/$file" 2>/dev/null; then
                echo -e "${RED}  ✗ 'edge-family' terminology found in $file (use 'arc-family')${NC}"
                exit 1
            fi
        fi
    done

    echo -e "${GREEN}  ✓ Semantic coherence OK${NC}"
fi

# 5. YAML count validation (only if YAML files are staged)
if [ -n "$STAGED_YAML" ]; then
    echo -e "${YELLOW}► Validating YAML schema...${NC}"
    YAML_COUNT=$(find "$REPO_ROOT/packages/core/models/node-classes" -name "*.yaml" | wc -l | tr -d ' ')
    if [ "$YAML_COUNT" -ne 60 ]; then
        echo -e "${RED}  ✗ YAML node count: $YAML_COUNT (expected 60 for v11.5)${NC}"
        exit 1
    fi
    echo -e "${GREEN}  ✓ YAML count OK ($YAML_COUNT nodes)${NC}"
fi

echo ""
echo -e "${GREEN}════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ All pre-commit checks passed${NC}"
echo -e "${GREEN}════════════════════════════════════════════════════════════════════════${NC}"
exit 0
