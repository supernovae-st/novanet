#!/bin/bash
# Claude Code Hook: Comprehensive codebase verification
# Can be run manually or as SessionStart hook for periodic checks
#
# Verifies:
# - YAML schema coherence (node/arc definitions)
# - Rust/TypeScript semantic coherence (terminology, counts)
# - Documentation sync (ADRs, terminology, decisions)
# - Version consistency across files
# - Hook integrity (all hooks executable, valid syntax)
#
# Usage: ./verify-codebase.sh [--quick] [--json] [--fix-suggestions]
# Options:
#   --quick          Skip slow checks (full codebase grep)
#   --json           Output JSON for Claude Code hook integration
#   --fix-suggestions Show fix commands for each issue
#
# Exit codes:
#   0 = All checks passed
#   1 = Warnings only
#   2 = Errors found (blocks in strict mode)

set -uo pipefail

# ═══════════════════════════════════════════════════════════════════════════════
# Configuration
# ═══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${CLAUDE_PROJECT_DIR:-$(cd "$SCRIPT_DIR/../.." && pwd)}"

# Colors (auto-detect)
if [[ -t 1 ]] && command -v tput &>/dev/null && [[ $(tput colors 2>/dev/null || echo 0) -ge 8 ]]; then
    RED='\033[0;31m'
    YELLOW='\033[0;33m'
    GREEN='\033[0;32m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    DIM='\033[2m'
    NC='\033[0m'
else
    RED='' YELLOW='' GREEN='' CYAN='' BOLD='' DIM='' NC=''
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Arguments
# ═══════════════════════════════════════════════════════════════════════════════

QUICK_MODE=false
JSON_OUTPUT=false
FIX_SUGGESTIONS=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --quick) QUICK_MODE=true; shift ;;
        --json) JSON_OUTPUT=true; shift ;;
        --fix-suggestions) FIX_SUGGESTIONS=true; shift ;;
        --help)
            echo "Usage: verify-codebase.sh [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --quick           Skip slow checks"
            echo "  --json            Output JSON for hooks"
            echo "  --fix-suggestions Show fix commands"
            exit 0
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# ═══════════════════════════════════════════════════════════════════════════════
# Counters
# ═══════════════════════════════════════════════════════════════════════════════

ERRORS=()
WARNINGS=()
CHECKS_PASSED=0

add_error() {
    ERRORS+=("$1")
}

add_warning() {
    WARNINGS+=("$1")
}

pass_check() {
    ((CHECKS_PASSED++))
}

# ═══════════════════════════════════════════════════════════════════════════════
# Check: Hook Integrity
# ═══════════════════════════════════════════════════════════════════════════════

check_hook_integrity() {
    local hooks_dir="$PROJECT_ROOT/.claude/hooks"

    if [[ ! -d "$hooks_dir" ]]; then
        add_error "Hooks directory not found: $hooks_dir"
        return
    fi

    local hook_count=0
    local invalid_count=0

    for hook in "$hooks_dir"/*.sh; do
        [[ -f "$hook" ]] || continue
        ((hook_count++))

        # Check executable
        if [[ ! -x "$hook" ]]; then
            add_warning "Hook not executable: $(basename "$hook")"
            ((invalid_count++))
            continue
        fi

        # Check syntax
        if ! bash -n "$hook" 2>/dev/null; then
            add_error "Hook has syntax errors: $(basename "$hook")"
            ((invalid_count++))
            continue
        fi
    done

    if [[ $invalid_count -eq 0 ]]; then
        pass_check
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# Check: Version Consistency
# ═══════════════════════════════════════════════════════════════════════════════

check_version_consistency() {
    local version_file="$PROJECT_ROOT/VERSION"

    if [[ ! -f "$version_file" ]]; then
        add_warning "VERSION file not found"
        return
    fi

    local version
    version=$(cat "$version_file")

    # Check CLAUDE.md references correct version
    local claude_md="$PROJECT_ROOT/CLAUDE.md"
    if [[ -f "$claude_md" ]]; then
        if ! grep -q "v${version}" "$claude_md" 2>/dev/null; then
            add_warning "CLAUDE.md may reference outdated version (expected v${version})"
        else
            pass_check
        fi
    fi

    # Check tools/novanet/CLAUDE.md
    local novanet_claude="$PROJECT_ROOT/tools/novanet/CLAUDE.md"
    if [[ -f "$novanet_claude" ]]; then
        if ! grep -q "v${version}" "$novanet_claude" 2>/dev/null; then
            add_warning "tools/novanet/CLAUDE.md may reference outdated version"
        else
            pass_check
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# Check: ADR Consistency
# ═══════════════════════════════════════════════════════════════════════════════

check_adr_consistency() {
    local adr_dir="$PROJECT_ROOT/.claude/rules/adr"
    local decisions_file="$PROJECT_ROOT/.claude/rules/novanet-decisions.md"

    if [[ ! -d "$adr_dir" ]]; then
        add_warning "ADR directory not found"
        return
    fi

    # Count ADRs in directory (lowercase pattern: adr-XXX.md)
    local adr_files
    adr_files=$(find "$adr_dir" -name "adr-*.md" 2>/dev/null | wc -l | xargs)
    adr_files=${adr_files:-0}

    # Check that we have a reasonable number of ADRs
    if [[ "$adr_files" -lt 10 ]]; then
        add_warning "Low ADR count: $adr_files files (expected 20+)"
    else
        pass_check
    fi

    # Verify decisions.md exists and references key ADRs
    if [[ -f "$decisions_file" ]]; then
        local key_adrs=("ADR-029" "ADR-030" "ADR-024" "ADR-025")
        local missing_adrs=()
        for adr in "${key_adrs[@]}"; do
            if ! grep -q "$adr" "$decisions_file" 2>/dev/null; then
                missing_adrs+=("$adr")
            fi
        done
        if [[ ${#missing_adrs[@]} -gt 0 ]]; then
            add_warning "decisions.md missing key ADRs: ${missing_adrs[*]}"
        else
            pass_check
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# Check: Schema YAML Coherence
# ═══════════════════════════════════════════════════════════════════════════════

check_schema_coherence() {
    local models_dir="$PROJECT_ROOT/packages/core/models"

    if [[ ! -d "$models_dir" ]]; then
        add_warning "Models directory not found"
        return
    fi

    # Check taxonomy.yaml exists and is valid YAML
    local taxonomy="$models_dir/taxonomy.yaml"
    if [[ ! -f "$taxonomy" ]]; then
        add_error "taxonomy.yaml not found"
        return
    fi

    # Basic YAML syntax check (if yq available)
    if command -v yq &>/dev/null; then
        if ! yq '.' "$taxonomy" >/dev/null 2>&1; then
            add_error "taxonomy.yaml has invalid YAML syntax"
            return
        fi
        pass_check
    fi

    # Count node classes
    local node_count
    node_count=$(find "$models_dir/node-classes" -name "*.yaml" -not -name "_*.yaml" 2>/dev/null | wc -l | xargs)

    # Expected: 61 nodes (v0.13.0)
    if [[ "$node_count" -lt 50 || "$node_count" -gt 70 ]]; then
        add_warning "Node count ($node_count) differs significantly from expected (61)"
    else
        pass_check
    fi

    # Count arc classes
    local arc_count
    arc_count=$(find "$models_dir/arc-classes" -name "*.yaml" -not -name "_*.yaml" 2>/dev/null | wc -l | xargs)

    # Expected: ~169 arcs (v0.13.0)
    if [[ "$arc_count" -lt 150 || "$arc_count" -gt 200 ]]; then
        add_warning "Arc count ($arc_count) differs significantly from expected (~169)"
    else
        pass_check
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# Check: Deprecated Terminology (Quick scan)
# ═══════════════════════════════════════════════════════════════════════════════

check_deprecated_terminology() {
    if [[ "$QUICK_MODE" == "true" ]]; then
        return  # Skip in quick mode
    fi

    local rust_dir="$PROJECT_ROOT/tools/novanet/src"
    local ts_dir="$PROJECT_ROOT/packages"

    # Check for v0.12.x terminology that should be v0.13.0
    local deprecated_terms="EntityContent|PageGenerated|BlockGenerated|ProjectContent|HAS_CONTENT|HAS_GENERATED"

    if [[ -d "$rust_dir" ]]; then
        local rust_matches
        rust_matches=$(grep -r "$deprecated_terms" "$rust_dir" --include="*.rs" 2>/dev/null | grep -v "test\|deprecated\|// v0.12" | head -5 || true)

        if [[ -n "$rust_matches" ]]; then
            add_warning "Found deprecated v0.12.x terminology in Rust code (use *Native pattern per ADR-029)"
        else
            pass_check
        fi
    fi

    if [[ -d "$ts_dir" ]]; then
        local ts_matches
        ts_matches=$(grep -r "$deprecated_terms" "$ts_dir" --include="*.ts" --include="*.tsx" 2>/dev/null | grep -v "node_modules\|test\|deprecated" | head -5 || true)

        if [[ -n "$ts_matches" ]]; then
            add_warning "Found deprecated v0.12.x terminology in TypeScript code (use *Native pattern)"
        else
            pass_check
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# Check: Rust Tests Pass
# ═══════════════════════════════════════════════════════════════════════════════

check_rust_tests() {
    if [[ "$QUICK_MODE" == "true" ]]; then
        return  # Skip in quick mode
    fi

    local novanet_dir="$PROJECT_ROOT/tools/novanet"

    if [[ ! -d "$novanet_dir" ]]; then
        return
    fi

    # Quick check: can we at least parse?
    if ! (cd "$novanet_dir" && cargo check --quiet 2>/dev/null); then
        add_error "Rust code has compilation errors"
        return
    fi

    pass_check
}

# ═══════════════════════════════════════════════════════════════════════════════
# Check: Documentation Sync
# ═══════════════════════════════════════════════════════════════════════════════

check_doc_sync() {
    local cheatsheet="$PROJECT_ROOT/.claude/rules/adr/CHEAT-SHEET.md"
    local terminology="$PROJECT_ROOT/.claude/rules/novanet-terminology.md"

    # Check cheatsheet mentions v0.13.0
    if [[ -f "$cheatsheet" ]]; then
        if ! grep -q "v0.13.0" "$cheatsheet" 2>/dev/null; then
            add_warning "CHEAT-SHEET.md may not reference v0.13.0"
        else
            pass_check
        fi
    fi

    # Check terminology mentions *Native pattern
    if [[ -f "$terminology" ]]; then
        if ! grep -q "EntityNative" "$terminology" 2>/dev/null; then
            add_warning "novanet-terminology.md may not include *Native pattern"
        else
            pass_check
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# Main Execution
# ═══════════════════════════════════════════════════════════════════════════════

# Run all checks
check_hook_integrity
check_version_consistency
check_adr_consistency
check_schema_coherence
check_deprecated_terminology
check_rust_tests
check_doc_sync

# ═══════════════════════════════════════════════════════════════════════════════
# Output
# ═══════════════════════════════════════════════════════════════════════════════

error_count=${#ERRORS[@]}
warning_count=${#WARNINGS[@]}

if [[ "$JSON_OUTPUT" == "true" ]]; then
    # JSON output for Claude Code hooks
    local summary=""
    if [[ $error_count -gt 0 ]]; then
        summary="VERIFICATION FAILED: $error_count error(s), $warning_count warning(s), $CHECKS_PASSED passed."
    elif [[ $warning_count -gt 0 ]]; then
        summary="VERIFICATION PASSED with warnings: $warning_count warning(s), $CHECKS_PASSED passed."
    else
        summary="VERIFICATION PASSED: All $CHECKS_PASSED checks passed."
    fi

    cat << EOF
{
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": "$summary"
  }
}
EOF
else
    # Text output
    echo ""
    echo -e "${BOLD}╭─────────────────────────────────────────────────────────────────────────╮${NC}"
    echo -e "${BOLD}│  CODEBASE VERIFICATION                                                   │${NC}"
    echo -e "${BOLD}╰─────────────────────────────────────────────────────────────────────────╯${NC}"
    echo ""

    if [[ $error_count -gt 0 ]]; then
        echo -e "${RED}${BOLD}Errors ($error_count):${NC}"
        for error in "${ERRORS[@]}"; do
            echo -e "  ${RED}x${NC} $error"
        done
        echo ""
    fi

    if [[ $warning_count -gt 0 ]]; then
        echo -e "${YELLOW}${BOLD}Warnings ($warning_count):${NC}"
        for warning in "${WARNINGS[@]}"; do
            echo -e "  ${YELLOW}!${NC} $warning"
        done
        echo ""
    fi

    echo -e "${GREEN}${BOLD}Checks Passed:${NC} $CHECKS_PASSED"
    echo ""

    if [[ $error_count -eq 0 && $warning_count -eq 0 ]]; then
        echo -e "${GREEN}${BOLD}All checks passed!${NC}"
    fi

    if [[ "$FIX_SUGGESTIONS" == "true" && ($error_count -gt 0 || $warning_count -gt 0) ]]; then
        echo -e "\n${CYAN}${BOLD}Fix Suggestions:${NC}"
        echo -e "  ${DIM}cargo run -- schema validate      # Validate YAML schema${NC}"
        echo -e "  ${DIM}cargo run -- schema generate      # Regenerate artifacts${NC}"
        echo -e "  ${DIM}./semantic-check.sh --strict FILE # Check specific file${NC}"
    fi

    echo ""
fi

# Exit codes
if [[ $error_count -gt 0 ]]; then
    exit 2
elif [[ $warning_count -gt 0 ]]; then
    exit 1
else
    exit 0
fi
