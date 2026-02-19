#!/bin/bash
# supernovae-agi Comprehensive Verification Script
# Usage: ./scripts/verify.sh [--quick|--full|--security]
# v1.0

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

MODE="${1:---quick}"
ERRORS=0
WARNINGS=0
START_TIME=$(date +%s)

header() {
    echo -e "\n${BLUE}════════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
}

check() {
    local name=$1
    local command=$2
    local workdir=$3

    printf "  %-40s" "$name"

    if [ -n "$workdir" ]; then
        cd "$workdir"
    fi

    if eval "$command" > /tmp/verify_output.txt 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        cd "$ROOT_DIR"
        return 0
    else
        echo -e "${RED}✗ FAIL${NC}"
        cd "$ROOT_DIR"
        return 1
    fi
}

warn_check() {
    local name=$1
    local command=$2
    local workdir=$3

    printf "  %-40s" "$name"

    if [ -n "$workdir" ]; then
        cd "$workdir"
    fi

    if eval "$command" > /tmp/verify_output.txt 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        cd "$ROOT_DIR"
        return 0
    else
        echo -e "${YELLOW}⚠ WARN${NC}"
        cd "$ROOT_DIR"
        WARNINGS=$((WARNINGS + 1))
        return 0  # Don't fail on warnings
    fi
}

# =============================================================================
# QUICK CHECKS
# =============================================================================

header "Quick Verification (lint + format + type-check)"

echo -e "\n${YELLOW}📦 NovaNet${NC}"

check "TypeScript type-check" "pnpm type-check" "$ROOT_DIR/novanet-dev" || ERRORS=$((ERRORS + 1))
check "novanet CLI clippy" "cargo clippy --quiet -- -D warnings" "$ROOT_DIR/novanet-dev/tools/novanet" || ERRORS=$((ERRORS + 1))
check "novanet-mcp clippy" "cargo clippy --quiet -- -D warnings" "$ROOT_DIR/novanet-dev/tools/novanet-mcp" || ERRORS=$((ERRORS + 1))
warn_check "novanet CLI fmt" "cargo fmt --check" "$ROOT_DIR/novanet-dev/tools/novanet"
warn_check "novanet-mcp fmt" "cargo fmt --check" "$ROOT_DIR/novanet-dev/tools/novanet-mcp"

echo -e "\n${YELLOW}🦾 Nika${NC}"

check "nika clippy" "cargo clippy --quiet -- -D warnings" "$ROOT_DIR/nika-dev/tools/nika" || ERRORS=$((ERRORS + 1))
warn_check "nika fmt" "cargo fmt --check" "$ROOT_DIR/nika-dev/tools/nika"

# =============================================================================
# FULL CHECKS (if requested)
# =============================================================================

if [ "$MODE" == "--full" ] || [ "$MODE" == "--security" ]; then
    header "Full Verification (tests)"

    echo -e "\n${YELLOW}📦 NovaNet Tests${NC}"
    check "novanet CLI tests" "cargo test --quiet" "$ROOT_DIR/novanet-dev/tools/novanet" || ERRORS=$((ERRORS + 1))
    check "novanet-mcp tests" "cargo test --quiet" "$ROOT_DIR/novanet-dev/tools/novanet-mcp" || ERRORS=$((ERRORS + 1))

    echo -e "\n${YELLOW}🦾 Nika Tests${NC}"
    check "nika tests" "cargo test --quiet" "$ROOT_DIR/nika-dev/tools/nika" || ERRORS=$((ERRORS + 1))
fi

# =============================================================================
# SECURITY CHECKS (if requested)
# =============================================================================

if [ "$MODE" == "--security" ]; then
    header "Security Verification"

    echo -e "\n${YELLOW}🔒 Security Audits${NC}"
    warn_check "novanet-mcp cargo audit" "cargo audit" "$ROOT_DIR/novanet-dev/tools/novanet-mcp"
    warn_check "nika cargo audit" "cargo audit" "$ROOT_DIR/nika-dev/tools/nika"

    if command -v cargo-deny &> /dev/null; then
        warn_check "novanet-mcp cargo deny" "cargo deny check" "$ROOT_DIR/novanet-dev/tools/novanet-mcp"
        warn_check "nika cargo deny" "cargo deny check" "$ROOT_DIR/nika-dev/tools/nika"
    else
        echo -e "  ${YELLOW}⊘ cargo-deny not installed (skip)${NC}"
    fi
fi

# =============================================================================
# SUMMARY
# =============================================================================

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

header "Summary"

echo -e "  Mode:      ${CYAN}$MODE${NC}"
echo -e "  Duration:  ${CYAN}${DURATION}s${NC}"
echo -e "  Errors:    ${ERRORS:-0}"
echo -e "  Warnings:  ${WARNINGS:-0}"

if [ $ERRORS -gt 0 ]; then
    echo -e "\n${RED}❌ Verification FAILED with $ERRORS error(s)${NC}"
    exit 1
elif [ $WARNINGS -gt 0 ]; then
    echo -e "\n${YELLOW}⚠️  Verification PASSED with $WARNINGS warning(s)${NC}"
else
    echo -e "\n${GREEN}✅ All checks passed!${NC}"
fi

echo -e "\n${BLUE}Usage: ./scripts/verify.sh [--quick|--full|--security]${NC}"
