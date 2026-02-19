#!/bin/bash
# Pre-commit validation hook
# Runs quick checks before allowing commits

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Running pre-commit checks...${NC}"

# Detect project context based on staged files
NOVANET_FILES=$(git diff --cached --name-only | grep -E "^novanet-dev/" || true)
NIKA_FILES=$(git diff --cached --name-only | grep -E "^nika-dev/" || true)

ERRORS=0

# NovaNet checks
if [ -n "$NOVANET_FILES" ]; then
    echo -e "${YELLOW}NovaNet files staged - running checks...${NC}"

    # TypeScript type-check
    if cd novanet-dev && pnpm type-check > /dev/null 2>&1; then
        echo -e "${GREEN}TypeScript type-check passed${NC}"
    else
        echo -e "${RED}TypeScript type-check failed${NC}"
        ERRORS=$((ERRORS + 1))
    fi
    cd ..

    # Rust clippy
    if cd novanet-dev/tools/novanet && cargo clippy --quiet -- -D warnings > /dev/null 2>&1; then
        echo -e "${GREEN}Rust clippy passed${NC}"
    else
        echo -e "${RED}Rust clippy failed${NC}"
        ERRORS=$((ERRORS + 1))
    fi
    cd ../../..
fi

# Nika checks
if [ -n "$NIKA_FILES" ]; then
    echo -e "${YELLOW}Nika files staged - running checks...${NC}"

    # Rust clippy
    if cd nika-dev/tools/nika && cargo clippy --quiet -- -D warnings > /dev/null 2>&1; then
        echo -e "${GREEN}Rust clippy passed${NC}"
    else
        echo -e "${RED}Rust clippy failed${NC}"
        ERRORS=$((ERRORS + 1))
    fi
    cd ../../..
fi

if [ $ERRORS -gt 0 ]; then
    echo -e "${RED}Pre-commit checks failed with $ERRORS error(s)${NC}"
    exit 1
fi

echo -e "${GREEN}All pre-commit checks passed${NC}"
