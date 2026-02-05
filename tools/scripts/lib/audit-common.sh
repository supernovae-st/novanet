#!/bin/bash
# NovaNet Audit Library
# Shared functions for doc-audit.sh and skill-audit.sh
# Source this file: source "$(dirname "$0")/lib/audit-common.sh"

# Colors
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[0;33m'
export BLUE='\033[0;34m'
export CYAN='\033[0;36m'
export NC='\033[0m' # No Color

# Counter for issues and warnings
export ISSUES=0
export WARNINGS=0

# Get project root from script location
get_project_root() {
  local script_dir="$(cd "$(dirname "${BASH_SOURCE[1]}")" && pwd)"
  echo "$(cd "$script_dir/../.." && pwd)"
}

# Read version from VERSION file
get_version() {
  cat VERSION 2>/dev/null || echo "unknown"
}

# Count node-kinds (excluding _index.yaml)
count_node_kinds() {
  find packages/core/models/node-kinds -name "*.yaml" ! -name "_index.yaml" 2>/dev/null | wc -l | tr -d ' '
}

# Count arc-kinds (excluding _index.yaml)
count_arc_kinds() {
  find packages/core/models/arc-kinds -name "*.yaml" ! -name "_index.yaml" 2>/dev/null | wc -l | tr -d ' '
}

# Count realms from taxonomy.yaml (v10.6: 2 realms - global, tenant)
count_realms() {
  grep -E "^  - key: (global|tenant)$" packages/core/models/taxonomy.yaml 2>/dev/null | wc -l | tr -d ' '
}

# Count layers from taxonomy.yaml
count_layers() {
  grep -E "^      - key: " packages/core/models/taxonomy.yaml 2>/dev/null | wc -l | tr -d ' '
}

# Count traits from taxonomy.yaml
count_traits() {
  grep -E "^  - key: (invariant|localized|knowledge|derived|job)$" packages/core/models/taxonomy.yaml 2>/dev/null | wc -l | tr -d ' '
}

# Count arc families from taxonomy.yaml
count_arc_families() {
  grep -E "^  - key: (ownership|localization|semantic|generation|mining)$" packages/core/models/taxonomy.yaml 2>/dev/null | wc -l | tr -d ' '
}

# Count arc scopes from taxonomy.yaml
count_arc_scopes() {
  grep -E "^  - key: (intra_realm|cross_realm)$" packages/core/models/taxonomy.yaml 2>/dev/null | wc -l | tr -d ' '
}

# Count cardinalities from taxonomy.yaml
count_cardinalities() {
  grep -E "^  - key: (zero_to_one|one_to_one|one_to_many|many_to_many)$" packages/core/models/taxonomy.yaml 2>/dev/null | wc -l | tr -d ' '
}

# Validate YAML syntax for a file using Python (fallback to yq if available)
validate_yaml_syntax() {
  local file="$1"

  if command -v python3 &> /dev/null; then
    python3 -c "import yaml; yaml.safe_load(open('$file'))" 2>&1
    return $?
  elif command -v yq &> /dev/null; then
    yq eval '.' "$file" > /dev/null 2>&1
    return $?
  else
    # No YAML validator available, skip
    return 0
  fi
}

# Validate all YAML files in a directory
validate_yaml_directory() {
  local dir="$1"
  local label="${2:-YAML files}"
  local yaml_errors=0

  if [ ! -d "$dir" ]; then
    return 0
  fi

  for file in $(find "$dir" -name "*.yaml" 2>/dev/null); do
    local result
    result=$(validate_yaml_syntax "$file" 2>&1)
    if [ $? -ne 0 ]; then
      echo -e "${RED}ERROR${NC}: Invalid YAML syntax in $file"
      echo "  $result" | head -3
      ((yaml_errors++))
      ((ISSUES++))
    fi
  done

  if [ $yaml_errors -eq 0 ]; then
    local count
    count=$(find "$dir" -name "*.yaml" 2>/dev/null | wc -l | tr -d ' ')
    echo -e "${GREEN}OK${NC}: $label - $count files validated"
  else
    echo -e "${RED}FAIL${NC}: $label - $yaml_errors syntax errors"
  fi

  return $yaml_errors
}

# Print taxonomy summary
print_taxonomy_summary() {
  local node_count=$(count_node_kinds)
  local arc_count=$(count_arc_kinds)
  local realm_count=$(count_realms)
  local layer_count=$(count_layers)
  local trait_count=$(count_traits)
  local arc_family_count=$(count_arc_families)
  local arc_scope_count=$(count_arc_scopes)
  local cardinality_count=$(count_cardinalities)

  # Apply defaults if parsing failed (v10.6: 43 nodes, 63 arcs, 2 realms, 9 layers)
  [ "$node_count" -eq 0 ] 2>/dev/null || node_count="${node_count:-43}"
  [ "$arc_count" -eq 0 ] 2>/dev/null || arc_count="${arc_count:-63}"
  [ "$realm_count" -eq 0 ] 2>/dev/null || realm_count="${realm_count:-2}"
  [ "$layer_count" -eq 0 ] 2>/dev/null || layer_count="${layer_count:-9}"
  [ "$trait_count" -eq 0 ] 2>/dev/null || trait_count="${trait_count:-5}"
  [ "$arc_family_count" -eq 0 ] 2>/dev/null || arc_family_count="${arc_family_count:-5}"
  [ "$arc_scope_count" -eq 0 ] 2>/dev/null || arc_scope_count="${arc_scope_count:-2}"
  [ "$cardinality_count" -eq 0 ] 2>/dev/null || cardinality_count="${cardinality_count:-4}"

  echo -e "${CYAN}Taxonomy Summary (from YAML):${NC}"
  echo "  Nodes:        $node_count NodeKinds"
  echo "  Arcs:         $arc_count ArcKinds"
  echo "  Realms:       $realm_count (global, tenant)"
  echo "  Layers:       $layer_count"
  echo "  Traits:       $trait_count (invariant, localized, knowledge, derived, job)"
  echo "  ArcFamilies:  $arc_family_count (ownership, localization, semantic, generation, mining)"
  echo "  ArcScopes:    $arc_scope_count (intra_realm, cross_realm)"
  echo "  Cardinalities: $cardinality_count (0..1, 1:1, 1:N, N:M)"
  echo ""

  # Export for use by other scripts
  export NODE_COUNT="$node_count"
  export ARC_COUNT="$arc_count"
  export REALM_COUNT="$realm_count"
  export LAYER_COUNT="$layer_count"
  export TRAIT_COUNT="$trait_count"
  export ARC_FAMILY_COUNT="$arc_family_count"
  export ARC_SCOPE_COUNT="$arc_scope_count"
  export CARDINALITY_COUNT="$cardinality_count"
}

# Print final audit result
print_audit_result() {
  local script_name="${1:-Audit}"

  echo ""
  echo "========================================"
  if [ $ISSUES -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}$script_name: All checks passed!${NC}"
    return 0
  elif [ $ISSUES -eq 0 ]; then
    echo -e "${YELLOW}$script_name: $WARNINGS warning(s)${NC}"
    return 0
  else
    echo -e "${RED}$script_name: $ISSUES error(s), $WARNINGS warning(s)${NC}"
    return 1
  fi
}
