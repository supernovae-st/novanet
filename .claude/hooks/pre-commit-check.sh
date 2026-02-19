#!/bin/bash
# Pre-commit validation hook
# Runs quick checks before allowing commits
# v2.0 - Enhanced with MCP checks and better error reporting

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Get script directory for absolute paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}🔍 Running pre-commit checks...${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"

# Detect project context based on staged files
NOVANET_FILES=$(git diff --cached --name-only | grep -E "^novanet-dev/" || true)
NOVANET_MCP_FILES=$(git diff --cached --name-only | grep -E "^novanet-dev/tools/novanet-mcp/" || true)
NIKA_FILES=$(git diff --cached --name-only | grep -E "^nika-dev/" || true)

ERRORS=0
WARNINGS=0

# Function to run check with proper error handling
run_check() {
    local name=$1
    local command=$2
    local workdir=$3

    if [ -n "$workdir" ]; then
        pushd "$workdir" > /dev/null 2>&1 || return 1
    fi

    if eval "$command" > /dev/null 2>&1; then
        echo -e "  ${GREEN}✓${NC} $name"
        if [ -n "$workdir" ]; then
            popd > /dev/null 2>&1
        fi
        return 0
    else
        echo -e "  ${RED}✗${NC} $name"
        if [ -n "$workdir" ]; then
            popd > /dev/null 2>&1
        fi
        return 1
    fi
}

# NovaNet checks
if [ -n "$NOVANET_FILES" ]; then
    echo -e "\n${YELLOW}📦 NovaNet${NC}"

    # TypeScript type-check
    run_check "TypeScript type-check" "pnpm type-check" "$ROOT_DIR/novanet-dev" || ERRORS=$((ERRORS + 1))

    # Rust clippy for novanet CLI
    if echo "$NOVANET_FILES" | grep -qE "tools/novanet/"; then
        run_check "novanet CLI clippy" "cargo clippy --quiet -- -D warnings" "$ROOT_DIR/novanet-dev/tools/novanet" || ERRORS=$((ERRORS + 1))
    fi

    # Rust clippy for novanet-mcp
    if [ -n "$NOVANET_MCP_FILES" ]; then
        run_check "novanet-mcp clippy" "cargo clippy --quiet -- -D warnings" "$ROOT_DIR/novanet-dev/tools/novanet-mcp" || ERRORS=$((ERRORS + 1))
    fi
fi

# Nika checks
if [ -n "$NIKA_FILES" ]; then
    echo -e "\n${YELLOW}🦾 Nika${NC}"

    # Rust clippy
    run_check "nika clippy" "cargo clippy --quiet -- -D warnings" "$ROOT_DIR/nika-dev/tools/nika" || ERRORS=$((ERRORS + 1))

    # Rust fmt check
    run_check "nika fmt" "cargo fmt --check" "$ROOT_DIR/nika-dev/tools/nika" || WARNINGS=$((WARNINGS + 1))
fi

# Summary
echo -e "\n${BLUE}════════════════════════════════════════════════════════════════${NC}"
if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}❌ Pre-commit failed: $ERRORS error(s), $WARNINGS warning(s)${NC}"
    echo -e "${YELLOW}💡 Fix errors before committing. Run individual checks for details.${NC}"
    exit 1
elif [ $WARNINGS -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Pre-commit passed with $WARNINGS warning(s)${NC}"
else
    echo -e "${GREEN}✅ All pre-commit checks passed${NC}"
fi
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
