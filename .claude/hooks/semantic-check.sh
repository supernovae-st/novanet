#!/bin/bash
# Claude Code Hook: Semantic coherence check for Rust/TypeScript files
# Triggered: PostToolUse on Write|Edit of *.rs/*.ts files in tools/novanet/ and packages/
#
# Validates:
# - No deprecated realm names (global/tenant)
# - No edge-family terminology (should be arc-family)
# - No outdated node counts in new/modified content
# - No TODO/FIXME without issue references
# - No panic!/unwrap without context
# - No deprecated L10n terminology (use Content/Generated)
# - No outdated layer names (locale-knowledge)
# - No hardcoded magic numbers for node counts
#
# Usage: ./semantic-check.sh [--fix] [--strict] [--no-cache] [file]

set -uo pipefail

# ═══════════════════════════════════════════════════════════════════════════════
# Configuration
# ═══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CACHE_DIR="${SCRIPT_DIR}/.semantic-cache"
CACHE_TTL=300  # 5 minutes

# Colors (auto-detect terminal support)
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

# Severity configuration (error or warning)
get_severity() {
    local check="$1"
    case "$check" in
        deprecated_realm|edge_family|l10n_terminology|locale_knowledge_layer)
            echo "error"
            ;;
        *)
            echo "warning"
            ;;
    esac
}

# ═══════════════════════════════════════════════════════════════════════════════
# Argument parsing
# ═══════════════════════════════════════════════════════════════════════════════

FIX_MODE=false
STRICT_MODE=false
NO_CACHE=false
EXPLICIT_FILE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --fix) FIX_MODE=true; shift ;;
        --strict) STRICT_MODE=true; shift ;;
        --no-cache) NO_CACHE=true; shift ;;
        --help)
            echo "Usage: semantic-check.sh [OPTIONS] [FILE]"
            echo ""
            echo "Options:"
            echo "  --fix       Show fix suggestions (advisory only)"
            echo "  --strict    Exit with error code on errors"
            echo "  --no-cache  Skip cache check"
            echo "  --help      Show this help"
            exit 0
            ;;
        --*) echo "Unknown option: $1"; exit 1 ;;
        *) EXPLICIT_FILE="$1"; shift ;;
    esac
done

# Get the file to check
FILE="${EXPLICIT_FILE:-${CLAUDE_FILE_PATH:-}}"

if [ -z "$FILE" ]; then
    exit 0
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Cache management
# ═══════════════════════════════════════════════════════════════════════════════

get_cache_key() {
    local file="$1"
    local hash
    hash=$(echo "$file" | md5 2>/dev/null || echo "$file" | md5sum 2>/dev/null | cut -d' ' -f1)
    echo "${CACHE_DIR}/${hash}.cache"
}

is_cache_valid() {
    local file="$1"
    local cache_file
    cache_file=$(get_cache_key "$file")
    
    if [[ "$NO_CACHE" == "true" ]]; then
        return 1
    fi
    
    if [[ ! -f "$cache_file" ]]; then
        return 1
    fi
    
    local cache_mtime file_mtime now
    cache_mtime=$(stat -f %m "$cache_file" 2>/dev/null || stat -c %Y "$cache_file" 2>/dev/null || echo 0)
    file_mtime=$(stat -f %m "$file" 2>/dev/null || stat -c %Y "$file" 2>/dev/null || echo 0)
    now=$(date +%s)
    
    # Cache is valid if: file hasn't changed AND cache isn't expired
    if [[ "$file_mtime" -lt "$cache_mtime" ]] && [[ $((now - cache_mtime)) -lt $CACHE_TTL ]]; then
        return 0
    fi
    return 1
}

update_cache() {
    local file="$1"
    local result="$2"
    mkdir -p "$CACHE_DIR"
    echo "$result" > "$(get_cache_key "$file")"
}

read_cache() {
    local file="$1"
    cat "$(get_cache_key "$file")" 2>/dev/null || echo ""
}

# ═══════════════════════════════════════════════════════════════════════════════
# File filtering
# ═══════════════════════════════════════════════════════════════════════════════

# Only check relevant files
should_check_file() {
    local file="$1"
    
    # Rust files in tools/novanet
    if [[ "$file" =~ tools/novanet.*\.rs$ ]]; then
        return 0
    fi
    
    # TypeScript files in packages (excluding node_modules, generated)
    if [[ "$file" =~ packages/.*\.(ts|tsx)$ ]] && [[ ! "$file" =~ node_modules|\.generated\. ]]; then
        return 0
    fi
    
    # YAML model files
    if [[ "$file" =~ packages/core/models/.*\.yaml$ ]]; then
        return 0
    fi
    
    return 1
}

if ! should_check_file "$FILE"; then
    exit 0
fi

# ═══════════════════════════════════════════════════════════════════════════════
# Check functions
# ═══════════════════════════════════════════════════════════════════════════════

ERRORS=()
WARNINGS=()

add_issue() {
    local severity="$1"
    local message="$2"
    local fix="${3:-}"
    local line="${4:-}"
    
    local entry=""
    if [[ -n "$line" ]]; then
        entry="$message ${DIM}(line $line)${NC}"
    else
        entry="$message"
    fi
    
    if [[ -n "$fix" ]]; then
        entry="$entry\n   ${CYAN}Fix:${NC} $fix"
    fi
    
    if [[ "$severity" == "error" ]]; then
        ERRORS+=("$entry")
    else
        WARNINGS+=("$entry")
    fi
}

check_deprecated_realms() {
    local file="$1"
    local matches
    
    # Skip test files and version comments
    if grep -q "#\[cfg(test)\]" "$file" 2>/dev/null; then
        return
    fi
    
    matches=$(grep -n '"global"\|"tenant"\|global_realm\|tenant_realm\|NodeRealm::Global\|NodeRealm::Tenant' "$file" 2>/dev/null | grep -v "// v10\|// v11\|// deprecated\|test" || true)
    
    if [[ -n "$matches" ]]; then
        while IFS= read -r match; do
            local line_num="${match%%:*}"
            add_issue "$(get_severity deprecated_realm)" \
                "Deprecated realm name: ${BOLD}global/tenant${NC} -> use ${GREEN}shared/org${NC}" \
                "Replace 'global' with 'shared', 'tenant' with 'org'" \
                "$line_num"
        done <<< "$matches"
    fi
}

check_edge_family() {
    local file="$1"
    local matches
    
    matches=$(grep -in 'edge.family\|edge-family\|EdgeFamily' "$file" 2>/dev/null | grep -v "// React Flow\|// deprecated" || true)
    
    if [[ -n "$matches" ]]; then
        while IFS= read -r match; do
            local line_num="${match%%:*}"
            add_issue "$(get_severity edge_family)" \
                "Wrong terminology: ${BOLD}edge-family${NC} -> use ${GREEN}arc-family${NC}" \
                "Replace 'Edge' with 'Arc' (except React Flow code)" \
                "$line_num"
        done <<< "$matches"
    fi
}

check_l10n_terminology() {
    local file="$1"
    local matches
    
    # Check for deprecated L10n node names
    matches=$(grep -n 'EntityL10n\|PageL10n\|BlockL10n\|ProjectL10n\|HAS_L10N\|HAS_OUTPUT' "$file" 2>/dev/null | grep -v "deprecated\|migration\|// v10" || true)
    
    if [[ -n "$matches" ]]; then
        while IFS= read -r match; do
            local line_num="${match%%:*}"
            local content="${match#*:}"
            local fix_suggestion=""
            
            if [[ "$content" =~ EntityL10n ]]; then
                fix_suggestion="Replace 'EntityL10n' with 'EntityContent'"
            elif [[ "$content" =~ PageL10n ]]; then
                fix_suggestion="Replace 'PageL10n' with 'PageGenerated'"
            elif [[ "$content" =~ BlockL10n ]]; then
                fix_suggestion="Replace 'BlockL10n' with 'BlockGenerated'"
            elif [[ "$content" =~ ProjectL10n ]]; then
                fix_suggestion="Replace 'ProjectL10n' with 'ProjectContent'"
            elif [[ "$content" =~ HAS_L10N ]]; then
                fix_suggestion="Replace 'HAS_L10N' with 'HAS_CONTENT'"
            elif [[ "$content" =~ HAS_OUTPUT ]]; then
                fix_suggestion="Replace 'HAS_OUTPUT' with 'HAS_GENERATED'"
            fi
            
            add_issue "$(get_severity l10n_terminology)" \
                "Deprecated L10n terminology found (v10.9 renamed)" \
                "$fix_suggestion" \
                "$line_num"
        done <<< "$matches"
    fi
}

check_locale_knowledge_layer() {
    local file="$1"
    local matches
    
    matches=$(grep -n 'locale-knowledge\|locale_knowledge\|LocaleKnowledge' "$file" 2>/dev/null | grep -v "// v11.2\|deprecated" || true)
    
    if [[ -n "$matches" ]]; then
        while IFS= read -r match; do
            local line_num="${match%%:*}"
            add_issue "$(get_severity locale_knowledge_layer)" \
                "Outdated layer: ${BOLD}locale-knowledge${NC} split in v11.3" \
                "Use 'locale', 'geography', or 'knowledge' layer instead" \
                "$line_num"
        done <<< "$matches"
    fi
}

check_outdated_node_counts() {
    local file="$1"
    local matches
    
    # Check for old node counts in comments/docs (v11.5 = 60 nodes)
    matches=$(grep -En '//.*\b(42|43|44|45|61|62|63|64|65)\b.*node|#.*\b(42|43|44|45|61|62|63|64|65)\b.*node' "$file" 2>/dev/null | grep -iv "v10\|v9\|v11\.[0-4]\|deprecated" || true)
    
    if [[ -n "$matches" ]]; then
        while IFS= read -r match; do
            local line_num="${match%%:*}"
            add_issue "$(get_severity outdated_node_count)" \
                "Possibly outdated node count (v11.5 has ${GREEN}60 nodes${NC}: 39 shared + 21 org)" \
                "Update comment to reflect v11.5 architecture" \
                "$line_num"
        done <<< "$matches"
    fi
}

check_magic_numbers() {
    local file="$1"
    local matches
    
    # Check for hardcoded magic numbers that look like node counts
    # Skip array indices, common values, and version patterns
    matches=$(grep -En '\b(42|43|44|45|61|62|63|64|65)\b' "$file" 2>/dev/null | \
        grep -v '//\|#\|test\|\[.*\]\|v1[01]\|0x\|version\|port\|timeout\|limit\|size\|index\|offset' || true)
    
    if [[ -n "$matches" ]]; then
        while IFS= read -r match; do
            local line_num="${match%%:*}"
            local content="${match#*:}"
            
            # Skip if it's clearly not a node count
            if [[ "$content" =~ (assert|expect|len|count|==|!=) ]]; then
                add_issue "$(get_severity magic_numbers)" \
                    "Magic number may be outdated node count" \
                    "Use named constant or verify against v11.5 schema (60 nodes)" \
                    "$line_num"
            fi
        done <<< "$matches"
    fi
}

check_todo_without_issue() {
    local file="$1"
    local matches
    
    # Find TODO/FIXME without issue reference (e.g., TODO(#123) or TODO: GH-123)
    matches=$(grep -En '\b(TODO|FIXME|HACK|XXX)\b' "$file" 2>/dev/null | \
        grep -v '#[0-9]\+\|GH-[0-9]\+\|JIRA-\|issue\|ticket' || true)
    
    if [[ -n "$matches" ]]; then
        while IFS= read -r match; do
            local line_num="${match%%:*}"
            add_issue "$(get_severity todo_without_issue)" \
                "TODO/FIXME without issue reference" \
                "Add issue reference: TODO(#123) or TODO: GH-123" \
                "$line_num"
        done <<< "$matches"
    fi
}

check_panic_unwrap() {
    local file="$1"
    
    # Only check Rust files
    if [[ ! "$file" =~ \.rs$ ]]; then
        return
    fi
    
    local matches
    
    # Check for unwrap/expect/panic without surrounding context comment
    matches=$(grep -En '\.unwrap\(\)|\.expect\(|panic!\(' "$file" 2>/dev/null | \
        grep -v 'test\|#\[cfg(test)\]\|// SAFETY\|// INVARIANT\|// OK:\|// unwrap ok\|unreachable' || true)
    
    if [[ -n "$matches" ]]; then
        # Limit to first 3 to avoid noise
        local count=0
        while IFS= read -r match && [[ $count -lt 3 ]]; do
            local line_num="${match%%:*}"
            add_issue "$(get_severity panic_unwrap)" \
                "panic!/unwrap/expect without safety comment" \
                "Add // SAFETY: or // INVARIANT: comment, or use ? operator" \
                "$line_num"
            ((count++)) || true
        done <<< "$matches"
        
        local total
        total=$(echo "$matches" | wc -l | tr -d ' ')
        if [[ $total -gt 3 ]]; then
            add_issue "warning" \
                "${DIM}... and $((total - 3)) more panic!/unwrap instances${NC}" \
                ""
        fi
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# Main execution
# ═══════════════════════════════════════════════════════════════════════════════

# Check cache first
if is_cache_valid "$FILE"; then
    cached_result=$(read_cache "$FILE")
    if [[ "$cached_result" == "clean" ]]; then
        exit 0
    fi
fi

# Run all checks
check_deprecated_realms "$FILE"
check_edge_family "$FILE"
check_l10n_terminology "$FILE"
check_locale_knowledge_layer "$FILE"
check_outdated_node_counts "$FILE"
check_magic_numbers "$FILE"
check_todo_without_issue "$FILE"
check_panic_unwrap "$FILE"

# Determine exit status
error_count=${#ERRORS[@]}
warning_count=${#WARNINGS[@]}
total_count=$((error_count + warning_count))

# Update cache
if [[ $total_count -eq 0 ]]; then
    update_cache "$FILE" "clean"
    exit 0
fi

update_cache "$FILE" "issues:$error_count:$warning_count"

# ═══════════════════════════════════════════════════════════════════════════════
# Output
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo -e "${BOLD}╭─────────────────────────────────────────────────────────────────────────╮${NC}"
echo -e "${BOLD}│  SEMANTIC CHECK${NC}                                                          ${BOLD}│${NC}"
echo -e "${BOLD}╰─────────────────────────────────────────────────────────────────────────╯${NC}"
echo -e "${DIM}File: $(basename "$FILE")${NC}"
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

# Summary
echo -e "${DIM}────────────────────────────────────────────────────────────────────────────${NC}"
if [[ $error_count -gt 0 ]]; then
    echo -e "${RED}${BOLD}$error_count error(s)${NC}, ${YELLOW}$warning_count warning(s)${NC}"
else
    echo -e "${YELLOW}${BOLD}$warning_count warning(s)${NC}"
fi

echo -e "${DIM}Run 'cargo make semantic-audit' for full codebase validation${NC}"

if [[ "$FIX_MODE" == "true" ]]; then
    echo -e "\n${CYAN}Note:${NC} --fix mode is advisory only. Manual fixes required."
fi

echo ""

# Exit with error in strict mode if there are errors
if [[ "$STRICT_MODE" == "true" ]] && [[ $error_count -gt 0 ]]; then
    exit 1
fi

exit 0
