# Quick Reference: Testing DX & Nomenclature

**TL;DR version of DX testing best practices**

---

## The Core Pattern

```rust
// 1. Define what should exist
#[test]
fn test_new_terminology_exists() {
    assert!(all_nodes().iter().any(|n| n.trait == "defined"));
}

// 2. Define what should NOT exist
#[test]
fn test_old_terminology_removed() {
    assert!(!all_nodes().iter().any(|n| n.trait == "invariant"));
}

// 3. Define semantic correctness
#[test]
fn test_trait_layer_alignment() {
    for node in all_nodes() {
        if node.trait == "authored" {
            assert!(["semantic", "foundation"].contains(&node.layer.as_str()));
        }
    }
}
```

---

## Test Checklist: Terminology Migration

| Phase | Test Category | Example |
|-------|---------------|---------|
| 🔴 Red | Define new values | `test_new_trait_values_are_defined` |
| 🔴 Red | Define old values removed | `test_old_traits_in_yaml_fail` |
| 🟢 Green | YAML parses with new values | `test_all_yaml_loads_with_new_traits` |
| 🟢 Green | No old traits remain | `test_no_old_traits_in_yaml` |
| 🔵 Refactor | Semantic correctness | `test_trait_layer_alignment` |
| 🔵 Refactor | Naming patterns match | `test_naming_patterns_consistency` |
| 🔵 Refactor | Output uses new terms | `test_cli_uses_new_terminology` |
| 🔵 Refactor | Docs updated | `test_documentation_uses_new_terms` |

---

## Test Template: Nomenclature Validation

```rust
#[test]
fn test_realm_values_are_canonical() {
    let valid = ["shared", "org"];

    for node in all_node_kinds() {
        assert!(
            valid.contains(&node.realm.as_str()),
            "Invalid realm '{}' in {}.\nValid: {:?}\nv11.2: 'global'→'shared', 'tenant'→'org'",
            node.realm, node.def.name, valid
        );
    }
}
```

**Key elements**:
1. ✓ What should be true
2. ✓ Assertion message with context
3. ✓ Migration hint in error message

---

## Test Template: Consistency Validation

```rust
#[test]
fn test_naming_pattern_consistency() {
    for node in all_node_kinds() {
        // If name matches pattern, check trait
        if node.def.name.ends_with("Content") {
            assert_eq!(node.def.node_trait, NodeTrait::Authored,
                "{} ends with Content but has {:?}",
                node.def.name, node.def.node_trait);
        }
    }
}
```

**Pattern**: `if name_pattern then assert_trait_and_layer`

---

## Test Template: Snapshot Testing

```rust
#[test]
fn test_typescript_generation_stability() {
    let output = generate_typescript_types();

    // Test what matters: terminology is correct
    assert!(output.contains("export type NodeClass"));
    assert!(!output.contains("export type NodeKind"));

    // Snapshot for structural stability
    insta::assert_snapshot!(output);
}
```

**Use snapshots for**: Generated code, CLI output, serialized structures

---

## Validation Issue Pattern

```rust
pub struct ValidationIssue {
    pub severity: Severity,        // Error, Warning, Info
    pub category: String,          // "realm_mismatch", "trait_mismatch"
    pub message: String,           // Human-readable explanation
    pub fix_hint: Option<String>,  // Actionable fix
}

// Usage
ValidationIssue::error(
    "path_mismatch",
    format!("Page: path '{}' doesn't contain realm 'org'", path)
)
.with_hint("Move file to: packages/core/models/node-kinds/org/structure/page.yaml")
```

**Benefits**:
- User understands what went wrong
- User knows exactly how to fix it
- Categories enable filtering/grouping errors

---

## Running Tests

```bash
# All YAML-only tests (no Neo4j)
cargo test --lib

# Specific category
cargo test realm
cargo test trait
cargo test naming_pattern

# With output
cargo test -- --nocapture --test-threads=1

# Ignored (snapshot) tests
cargo test -- --ignored

# Single test
cargo test test_trait_layer_alignment
```

---

## Migration Workflow (TDD)

### Step 1: Red (Define new state)

```bash
# Create test that expects new terminology
cargo test test_new_trait_values  # FAILS
cargo test test_no_old_traits     # FAILS
```

### Step 2: Green (Update YAML)

```bash
# Update all 60 node-kinds/*.yaml files
sed -i 's/trait: invariant/trait: defined/' \
    packages/core/models/node-kinds/**/*.yaml

# Regenerate code
cargo run -- schema generate

# Tests should now PASS
cargo test test_new_trait_values  # PASSES
cargo test test_no_old_traits     # PASSES
```

### Step 3: Refactor (Validate semantics)

```bash
# Run comprehensive validation
cargo run -- schema validate

# Run semantic tests
cargo test trait_layer_alignment   # PASSES
cargo test naming_patterns         # PASSES

# Update documentation
# - .claude/rules/novanet-terminology.md
# - .claude/rules/novanet-decisions.md
```

---

## Common Failure Patterns

| Failure Pattern | Root Cause | Fix |
|-----------------|-----------|-----|
| `trait: invariant not found` | YAML not updated | `sed -i 's/invariant/defined/'` |
| `NodeKind not found in code` | Type not renamed | Rename struct + update imports |
| `Trait mismatch: defined in output` | Wrong layer | Move node YAML to correct layer |
| `Arc scope intra but cross_realm` | Scope validation | Update arc-kinds/*.yaml scope field |
| `Snapshot mismatch` | Generated code outdated | `cargo test -- --ignored` to update |

---

## Debugging Commands

```bash
# Find old terminology
grep -r "invariant" packages/core/models/ --include="*.yaml"
grep -r "NodeKind" tools/novanet/src/ --include="*.rs"

# Count trait distribution
grep -r "trait:" packages/core/models/ | cut -d: -f3 | sort | uniq -c

# Validate YAML
cargo run -- schema validate --strict

# Show schema as table
cargo run -- meta --format=table | head -20

# Check Neo4j labels
cargo run -- db --query 'MATCH (n) RETURN distinct labels(n) LIMIT 10'
```

---

## Pre-Commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

set -e

echo "Checking terminology consistency..."

# YAML tests
cargo test --lib terminology::*
cargo test --lib trait::*

# Naming patterns
cargo test --lib naming_pattern

# Schema validation
cargo run -- schema validate

echo "✓ All checks passed"
```

---

## Documentation Reference

For full details, see:
- `DX-testing-nomenclature-validation.md` — Complete patterns and philosophy
- `v11.8-migration-testing.md` — Detailed Phase 1/2/3 testing for v11.8
- `.claude/rules/novanet-terminology.md` — Terminology source of truth
- `.claude/rules/novanet-decisions.md` — ADR-023, ADR-024 decisions

---

## Key Insights

1. **Terminology is testable** — Use tests as executable specification
2. **Validate early** — YAML coherence before Neo4j/runtime
3. **Semantic correctness matters** — Test trait ↔ layer alignment
4. **Naming patterns encode intent** — `*Content` → authored, `*Generated` → generated
5. **Errors should be actionable** — Include fix hints with messages
6. **Snapshots catch drift** — Generated code changes show in diffs
7. **TDD for migrations** — Red (new), Green (update), Refactor (validate)

---

## The One-Minute Test

```rust
#[test]
fn test_terminology_is_consistent() {
    // 1. All nodes have valid realm/layer/trait
    for node in all_node_kinds() {
        assert!(valid_realms.contains(&node.realm));
        assert!(valid_layers.contains(&node.layer));
        assert!(valid_traits.contains(&node.trait));
    }

    // 2. Arcs match their scope
    for arc in all_arc_kinds() {
        let (sr, tr) = get_arc_realms(&arc);
        let is_cross = sr != tr;
        assert!(
            (is_cross && arc.scope == "cross_realm") ||
            (!is_cross && arc.scope == "intra_realm"),
            "Arc {} scope mismatch", arc.name
        );
    }

    // 3. No old terminology
    assert!(!any_nodes_with_trait("invariant"));
    assert!(!any_nodes_with_trait("localized"));
}
```

This one test catches **90% of common issues**.
