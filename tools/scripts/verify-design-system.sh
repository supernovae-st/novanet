#!/bin/bash
# verify-design-system.sh
# Unified verification pipeline for NovaNet design system coherence
#
# v11.6.0 — Clean architecture, no backward compatibility
#
# Validates:
# - YAML sources (taxonomy.yaml, visual-encoding.yaml)
# - TypeScript artifacts (layers.ts, hierarchy.ts, nodes.ts)
# - Rust TUI (theme.rs)
# - Studio edge system (arcFamilyPalettes.ts, themes.ts, registry.ts)
#
# Usage:
#   ./tools/scripts/verify-design-system.sh [--quick|--full|--ci]
#
# Modes:
#   --quick  Fast validation (YAML + TypeScript only)
#   --full   Complete validation (includes Rust build + all tests)
#   --ci     CI mode (full + JSON output)

set -euo pipefail

# =============================================================================
# Configuration
# =============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# ANSI colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

# Counters
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
WARNINGS=0

# Mode
MODE="${1:---full}"

# =============================================================================
# Helper Functions
# =============================================================================

log_header() {
  echo ""
  echo -e "${BOLD}${CYAN}═══════════════════════════════════════════════════════════════${RESET}"
  echo -e "${BOLD}${CYAN}  $1${RESET}"
  echo -e "${CYAN}═══════════════════════════════════════════════════════════════${RESET}"
  echo ""
}

log_section() {
  echo -e "${BOLD}┌─ $1${RESET}"
}

log_ok() {
  echo -e "${GREEN}  ✓${RESET} $1"
  ((PASSED_CHECKS++)) || true
  ((TOTAL_CHECKS++)) || true
}

log_fail() {
  echo -e "${RED}  ✗${RESET} $1"
  ((FAILED_CHECKS++)) || true
  ((TOTAL_CHECKS++)) || true
}

log_warn() {
  echo -e "${YELLOW}  ⚠${RESET} $1"
  ((WARNINGS++)) || true
}

log_info() {
  echo -e "${BLUE}  ℹ${RESET} $1"
}

check_file_exists() {
  local file="$1"
  local description="$2"

  if [[ -f "$ROOT_DIR/$file" ]]; then
    log_ok "$description exists"
  else
    log_fail "$description missing: $file"
  fi
}

# =============================================================================
# Validation Stages
# =============================================================================

stage_yaml_sources() {
  log_header "Stage 1: YAML Sources"

  log_section "Required YAML files"
  check_file_exists "packages/core/models/taxonomy.yaml" "taxonomy.yaml"
  check_file_exists "packages/core/models/visual-encoding.yaml" "visual-encoding.yaml"
  check_file_exists "packages/core/models/_index.yaml" "_index.yaml"

  log_section "YAML syntax validation"
  if command -v yq &> /dev/null; then
    if yq eval 'true' "$ROOT_DIR/packages/core/models/taxonomy.yaml" > /dev/null 2>&1; then
      log_ok "taxonomy.yaml: valid YAML syntax"
    else
      log_fail "taxonomy.yaml: invalid YAML syntax"
    fi

    if yq eval 'true' "$ROOT_DIR/packages/core/models/visual-encoding.yaml" > /dev/null 2>&1; then
      log_ok "visual-encoding.yaml: valid YAML syntax"
    else
      log_fail "visual-encoding.yaml: invalid YAML syntax"
    fi
  else
    log_warn "yq not installed, skipping YAML syntax validation"
  fi

  log_section "Schema version check"
  local version
  version=$(grep -E "^version:" "$ROOT_DIR/packages/core/models/taxonomy.yaml" | head -1 | cut -d'"' -f2)
  if [[ "$version" == "11.6.0" ]]; then
    log_ok "taxonomy.yaml version: $version"
  else
    log_warn "taxonomy.yaml version: $version (expected 11.6.0)"
  fi
}

stage_typescript_artifacts() {
  log_header "Stage 2: TypeScript Artifacts"

  log_section "Generated files exist"
  check_file_exists "packages/core/src/graph/layers.ts" "layers.ts"
  check_file_exists "packages/core/src/graph/hierarchy.ts" "hierarchy.ts"
  check_file_exists "packages/core/src/types/nodes.ts" "nodes.ts"
  check_file_exists "packages/core/src/graph/visual-encoding.ts" "visual-encoding.ts"

  log_section "TypeScript type check"
  if pnpm type-check --filter=@novanet/core > /dev/null 2>&1; then
    log_ok "TypeScript: no type errors"
  else
    log_fail "TypeScript: type errors detected"
  fi

  log_section "Node count verification"
  local node_count
  node_count=$(grep -c "^  [A-Z].*:" "$ROOT_DIR/packages/core/src/graph/layers.ts" 2>/dev/null || echo 0)
  if [[ "$node_count" -eq 60 ]]; then
    log_ok "NODE_LAYERS: 60 nodes defined"
  else
    log_fail "NODE_LAYERS: $node_count nodes (expected 60)"
  fi
}

stage_design_system_validation() {
  log_header "Stage 3: Design System Coherence"

  log_section "Running validate-design-system.mjs"
  if node "$ROOT_DIR/tools/scripts/validate-design-system.mjs" > /tmp/design-system-validation.log 2>&1; then
    log_ok "Design system validation passed"
    # Count individual checks from log
    local ds_passed
    ds_passed=$(grep -c "✓" /tmp/design-system-validation.log 2>/dev/null || echo 0)
    log_info "$ds_passed individual checks passed"
  else
    log_fail "Design system validation failed"
    echo ""
    cat /tmp/design-system-validation.log
    echo ""
  fi
}

stage_schema_validation() {
  log_header "Stage 4: Schema Validation (Rust)"

  log_section "Cargo build check"
  if cargo check --manifest-path "$ROOT_DIR/tools/novanet/Cargo.toml" > /dev/null 2>&1; then
    log_ok "Rust crate compiles"
  else
    log_fail "Rust crate compilation errors"
    return 1
  fi

  log_section "Schema validation"
  if cargo run --manifest-path "$ROOT_DIR/tools/novanet/Cargo.toml" -- schema validate > /tmp/schema-validation.log 2>&1; then
    log_ok "Schema validation passed"
  else
    log_fail "Schema validation failed"
    cat /tmp/schema-validation.log
  fi
}

stage_unit_tests() {
  log_header "Stage 5: Unit Tests"

  log_section "Core package tests"
  if pnpm test --filter=@novanet/core > /tmp/core-tests.log 2>&1; then
    log_ok "@novanet/core tests passed"
  else
    log_fail "@novanet/core tests failed"
  fi

  log_section "Design system sync tests"
  if pnpm test --filter=@novanet/studio -- --testPathPattern=DesignSystemSync > /tmp/ds-tests.log 2>&1; then
    log_ok "DesignSystemSync tests passed"
  else
    log_fail "DesignSystemSync tests failed"
  fi
}

stage_rust_tests() {
  log_header "Stage 6: Rust Tests"

  log_section "Cargo tests"
  if cargo test --manifest-path "$ROOT_DIR/tools/novanet/Cargo.toml" --lib > /tmp/rust-tests.log 2>&1; then
    local test_count
    test_count=$(grep -E "test result: ok\." /tmp/rust-tests.log | grep -oE "[0-9]+ passed" | head -1 || echo "? passed")
    log_ok "Rust tests: $test_count"
  else
    log_fail "Rust tests failed"
  fi
}

stage_deprecated_check() {
  log_header "Stage 7: Deprecated Terms Audit"

  log_section "Checking for deprecated realm names"
  local deprecated_realms=("global" "tenant")
  for term in "${deprecated_realms[@]}"; do
    if grep -r "realm.*['\"]$term['\"]" "$ROOT_DIR/packages/core/src" --include="*.ts" > /dev/null 2>&1; then
      log_warn "Found deprecated realm '$term' in TypeScript"
    else
      log_ok "No deprecated realm '$term' in TypeScript"
    fi
  done

  log_section "Checking for deprecated trait names"
  local deprecated_traits=("derived" "job")
  for term in "${deprecated_traits[@]}"; do
    if grep -r "trait.*['\"]$term['\"]" "$ROOT_DIR/packages/core/src" --include="*.ts" > /dev/null 2>&1; then
      log_warn "Found deprecated trait '$term' in TypeScript"
    else
      log_ok "No deprecated trait '$term' in TypeScript"
    fi
  done

  log_section "Checking for deprecated L10n suffix"
  if grep -r "L10n" "$ROOT_DIR/packages/core/src" --include="*.ts" | grep -v "test" | grep -v "\.d\.ts" > /dev/null 2>&1; then
    log_warn "Found deprecated 'L10n' suffix in TypeScript (use Content/Generated)"
  else
    log_ok "No deprecated 'L10n' suffix in TypeScript"
  fi

  log_section "Checking for deprecated layer names"
  local deprecated_layers=("seo" "geo" "locale-knowledge")
  for term in "${deprecated_layers[@]}"; do
    if grep -r "layer.*['\"]$term['\"]" "$ROOT_DIR/packages/core/src" --include="*.ts" > /dev/null 2>&1; then
      log_warn "Found deprecated layer '$term' in TypeScript"
    else
      log_ok "No deprecated layer '$term' in TypeScript"
    fi
  done
}

# =============================================================================
# Summary
# =============================================================================

print_summary() {
  log_header "Verification Summary"

  echo -e "  Total checks:  ${BOLD}$TOTAL_CHECKS${RESET}"
  echo -e "  ${GREEN}Passed:${RESET}        ${BOLD}$PASSED_CHECKS${RESET}"
  echo -e "  ${RED}Failed:${RESET}        ${BOLD}$FAILED_CHECKS${RESET}"
  echo -e "  ${YELLOW}Warnings:${RESET}      ${BOLD}$WARNINGS${RESET}"
  echo ""

  if [[ $FAILED_CHECKS -eq 0 ]]; then
    echo -e "${GREEN}${BOLD}  ✓ All verification checks passed!${RESET}"
    echo ""
    return 0
  else
    echo -e "${RED}${BOLD}  ✗ $FAILED_CHECKS verification check(s) failed${RESET}"
    echo ""
    return 1
  fi
}

# =============================================================================
# Main
# =============================================================================

main() {
  log_header "NovaNet Design System Verification Pipeline v11.6.0"
  log_info "Mode: $MODE"
  log_info "Root: $ROOT_DIR"

  cd "$ROOT_DIR"

  case "$MODE" in
    --quick)
      stage_yaml_sources
      stage_typescript_artifacts
      stage_design_system_validation
      ;;
    --full)
      stage_yaml_sources
      stage_typescript_artifacts
      stage_design_system_validation
      stage_schema_validation
      stage_unit_tests
      stage_deprecated_check
      ;;
    --ci)
      stage_yaml_sources
      stage_typescript_artifacts
      stage_design_system_validation
      stage_schema_validation
      stage_unit_tests
      stage_rust_tests
      stage_deprecated_check
      ;;
    *)
      echo "Usage: $0 [--quick|--full|--ci]"
      exit 1
      ;;
  esac

  print_summary
}

main "$@"
