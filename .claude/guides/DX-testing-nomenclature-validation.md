# DX Testing & Nomenclature Validation Guide

**For NovaNet v11.8 (Class/Instance/Data Origin Traits)**

This guide documents best practices for creating DX tests that validate terminology consistency, nomenclature adherence, and schema coherence across a large codebase.

---

## Core Principles

### 1. Consistency as a Feature

Terminology consistency is not just documentation—it's a testable property of your system.

**From Claude documentation (Agent Skills Best Practices)**:
> "Consistency helps Claude understand and follow instructions."

The same principle applies to codebases: consistent terminology reduces cognitive load and prevents errors during refactoring.

### 2. Reference Documents Over Rules

Rather than encoding rules in code, establish reference documents (CLAUDE.md, terminology.md, ADR files) and test against them.

**Pattern from Agent Skills documentation**:
```markdown
Content review process:
1. Draft content following STYLE_GUIDE.md
2. Review against terminology checklist
3. Verify examples follow standard format
4. Check terminology consistency
5. Only proceed when all requirements are met
```

This approach allows humans to understand the "why" while tests verify the "what."

### 3. Validation Before Execution

Always validate schema coherence BEFORE using data:
- YAML path must match content (realm/layer declared in file)
- Arc scope must match actual source/target realms
- Terminology must follow established conventions

**Pattern from NovaNet validation.rs**:
```rust
// Path validation: file location must match YAML content
if !path_str.contains(&format!("/{}/", node.realm)) {
    error!("{}: path doesn't contain realm '{}'", node.def.name, node.realm);
}

// Scope validation: declared scope must match actual realms
if is_cross_realm && scope == "intra_realm" {
    error!("{}: cross_realm mismatch", arc.arc_type);
}
```

---

## Testing Patterns for Terminology

### Pattern 1: Test Fixtures with Semantic Names

Use factory functions that make terminology visible in test code:

```rust
// ✅ GOOD: Trait is explicit, parameters are semantic
fn make_node(
    name: &str,
    realm: &str,      // "shared" or "org"
    layer: &str,      // "config", "semantic", etc.
    trait: NodeTrait, // Defined, Authored, Imported, Generated, Retrieved
) -> ParsedNode { ... }

// Usage makes terminology clear:
let node = make_node("Page", "org", "structure", NodeTrait::Defined);
```

**Benefits**:
- Test code documents correct terminology usage
- Failures show exactly which field is wrong
- Adding new traits/layers shows up in type system

### Pattern 2: Semantic Validation Issues

Structure validation issues to include both human-readable messages and fix hints:

```rust
pub struct ValidationIssue {
    pub severity: Severity,        // Error, Warning, Info
    pub category: String,          // "realm_mismatch", "trait_mismatch", etc.
    pub message: String,           // Human-readable explanation
    pub fix_hint: Option<String>,  // Actionable fix ("Update to realm: 'org'")
}

// Usage:
issue.error("realm_mismatch", "Page: path doesn't contain realm 'org'")
    .with_hint("Move file to packages/core/models/node-kinds/org/...")
```

**Benefits**:
- Users understand what went wrong and how to fix it
- Categories enable filtering (show only trait mismatches)
- Hints prevent frustration with cryptic errors

### Pattern 3: YAML-First Validation Tests

Validate YAML coherence WITHOUT Neo4j (faster, more reliable):

```rust
#[test]
fn test_realm_layer_consistency() {
    // Test 1: File path matches YAML content
    for node in all_node_kinds() {
        let expected_path = format!(
            "node-kinds/{}/{}/{}",
            node.realm, node.layer, node.name
        );
        assert!(node.source_path.to_string_lossy().contains(&expected_path));
    }
}

#[test]
fn test_trait_defined_values() {
    // Test 2: Only allowed trait values exist
    for node in all_node_kinds() {
        assert!(matches!(
            node.trait,
            NodeTrait::Defined | NodeTrait::Authored | NodeTrait::Imported
                | NodeTrait::Generated | NodeTrait::Retrieved
        ));
    }
}

#[test]
fn test_arc_scope_matches_realms() {
    // Test 3: Arc scope (intra/cross) matches source/target realms
    let node_realms = build_node_realm_map();
    for arc in all_arc_kinds() {
        let (source_realm, target_realm) = get_arc_realms(&arc, &node_realms);
        let is_cross = source_realm != target_realm;

        if arc.scope == "intra_realm" {
            assert!(!is_cross, "{}: intra declared but cross_realm", arc.name);
        } else if arc.scope == "cross_realm" {
            assert!(is_cross, "{}: cross declared but intra_realm", arc.name);
        }
    }
}
```

### Pattern 4: Snapshot Testing for Generated Code

Use snapshot tests (insta) to catch unintended terminology changes:

```rust
#[test]
fn test_node_kind_typescript_generation() {
    let node = make_node("Page", "org", "structure", NodeTrait::Defined);
    let ts_output = generate_typescript(&node);
    insta::assert_snapshot!(ts_output);
    // If trait renames (Invariant → Defined), snapshot fails with diff
}

#[test]
fn test_neo4j_labels_consistency() {
    let schema = load_schema();
    let labels = extract_neo4j_labels(&schema);
    insta::assert_snapshot!(labels);
    // Changes to label naming show in git diff before commit
}
```

---

## Checklist: DX Tests for v11.8 Nomenclature

Before releasing a terminology update, ensure these tests exist and pass:

### YAML Coherence Tests

- [ ] Path validation: `realm` and `layer` in YAML match file location
- [ ] Required fields: Every node has name, realm, layer, trait
- [ ] Trait values: Only allowed traits (defined/authored/imported/generated/retrieved)
- [ ] Arc scope: Arc scope (intra/cross) matches source/target realms
- [ ] No orphans: Every arc source/target references an existing node
- [ ] No duplicates: Arc types are unique within their family
- [ ] Icon existence: Every realm/layer/trait has icon entry

### Terminology Consistency Tests

- [ ] Deprecated term detection: No `Kind`, `Meta`, `invariant`, `localized`, etc. in new code
- [ ] Naming patterns: `*Content` for localized, `*Generated` for output, `*Structure`/`*Instruction`
- [ ] Comments use correct terms: Comments mention "Class" not "Kind", "Data Origin" for traits
- [ ] Error messages: Use canonical terminology (PageStructure, not PageType)
- [ ] Documentation: References to nodes use correct names (ADRs, CLAUDE.md)

### Schema Alignment Tests (Neo4j)

- [ ] Count sync: YAML node count matches `:Schema:Class` nodes in Neo4j
- [ ] Trait distribution: Neo4j node trait properties match YAML
- [ ] Arc family mapping: `:HAS_STRUCTURE`, `:HAS_INSTRUCTION` use correct arc types
- [ ] Label consistency: `:Schema:Class` labels match YAML structure

### Integration Tests

- [ ] Generator output: `schema generate` produces code with correct terminology
- [ ] Snapshot diffs: Changed terminology shows in snapshot diffs before commit
- [ ] CLI output: `novanet meta` lists traits as defined/authored/imported/generated/retrieved
- [ ] TUI display: Keybinding help shows `[1]Graph` and `[2]Nexus`, not old mode names

---

## Implementation: NovaNet Example

### Test File Structure

```
tools/novanet/src/
├── commands/
│   ├── schema.rs          # `schema generate` + `schema validate`
│   └── tests/
│       └── schema_tests.rs
└── blueprint/
    ├── validation.rs      # ValidationIssue + ValidationResult
    └── tests/
        └── validation_tests.rs
```

### Key Test: Realm/Layer Path Validation

From `blueprint/validation.rs`:

```rust
fn check_path_content_match(result: &mut ValidationResult, data: &BlueprintData) {
    /// Check that YAML file paths match the realm/layer declared in content.
    for node in &data.nodes {
        let path_str = node.source_path.to_string_lossy();

        // Check realm in path
        if !path_str.contains(&format!("/{}/", node.realm)) {
            result.issues.push(
                ValidationIssue::error(
                    "path_content_mismatch",
                    format!("{}: path doesn't contain realm '{}'",
                        node.def.name, node.realm)
                ).with_hint(format!(
                    "Move to: packages/core/models/node-kinds/{}/{}/{}.yaml",
                    node.realm, node.layer, node.def.name.to_lowercase()
                ))
            );
        }

        // Check layer in path
        if !path_str.contains(&format!("/{}/", node.layer)) {
            result.issues.push(
                ValidationIssue::error(
                    "path_content_mismatch",
                    format!("{}: path doesn't contain layer '{}'",
                        node.def.name, node.layer)
                ).with_hint(...)
            );
        }
    }
}
```

### Key Test: Trait Value Validation

```rust
fn check_required_fields(result: &mut ValidationResult, data: &BlueprintData) {
    for node in &data.nodes {
        // Validate trait is one of the 5 allowed values
        let valid_traits = ["defined", "authored", "imported", "generated", "retrieved"];
        if !valid_traits.contains(&node.def.node_trait.to_string().as_str()) {
            result.issues.push(
                ValidationIssue::error(
                    "invalid_trait",
                    format!("{}: trait '{}' not recognized",
                        node.def.name, node.def.node_trait)
                ).with_hint("Use: defined, authored, imported, generated, or retrieved")
            );
        }
    }
}
```

### Command: Schema Validation

```bash
# Runs all validation tests on YAML (no Neo4j required)
cargo run -- schema validate

# Output:
# ✓ Path validation: 60/60 passed
# ✗ Arc scope coherence: 2 errors
#   - HAS_CONTENT: cross declared but intra_realm
#     Fix: Update scope in arc-kinds/semantic/has-content.yaml
```

---

## Testing Best Practices from Claude Docs

### 1. Task-Specific Test Cases

Design tests that mirror real-world usage:

```rust
// ✅ Tests reflect actual NovaNet operations
#[test]
fn test_schema_generation_with_all_traits() {
    // Tests: Defined (config), Authored (content), Imported (knowledge),
    //        Generated (output), Retrieved (metrics)
    let nodes = vec![
        make_node("Page", "org", "structure", NodeTrait::Defined),
        make_node("PageGenerated", "org", "output", NodeTrait::Generated),
        make_node("EntityContent", "org", "semantic", NodeTrait::Authored),
    ];
    let output = generate_schema(&nodes);
    assert!(output.contains("trait: defined"));
    assert!(output.contains("trait: generated"));
    assert!(output.contains("trait: authored"));
}
```

### 2. Automated Grading with Precise Rubrics

Test both positive and negative cases:

```rust
#[test]
fn test_deprecated_terminology_detection() {
    // NEGATIVE: Should NOT contain deprecated terms
    let bad_names = vec!["NodeKind", "ArcKind", "KindInfo", "invariant", "derived"];
    for node_name in all_node_kinds().iter().map(|n| &n.def.name) {
        for bad_term in &bad_names {
            assert!(!node_name.contains(bad_term),
                "Node name contains deprecated term: {} in {}",
                bad_term, node_name);
        }
    }

    // POSITIVE: Should use new terms
    let should_exist = vec!["PageStructure", "PageInstruction", "EntityContent"];
    let all_names: Vec<_> = all_node_kinds()
        .iter()
        .map(|n| n.def.name.as_str())
        .collect();
    for new_term in should_exist {
        assert!(all_names.contains(&new_term),
            "Missing expected node: {}", new_term);
    }
}
```

### 3. Consistency Evaluation

Test semantic similarity (like the FAQ example in Claude docs):

```rust
#[test]
fn test_naming_pattern_consistency() {
    // All *Content nodes should be localized trait
    for node in all_node_kinds() {
        if node.def.name.ends_with("Content") {
            assert_eq!(
                node.def.node_trait,
                NodeTrait::Authored,
                "{} ends with Content but has trait {:?}",
                node.def.name, node.def.node_trait
            );
        }
    }

    // All *Generated nodes should be generated trait and output layer
    for node in all_node_kinds() {
        if node.def.name.ends_with("Generated") {
            assert_eq!(node.def.node_trait, NodeTrait::Generated,
                "{} ends with Generated but has trait {:?}",
                node.def.name, node.def.node_trait);
            assert_eq!(node.layer, "output",
                "{} ends with Generated but in layer {}",
                node.def.name, node.layer);
        }
    }
}
```

### 4. Human-Understandable Failure Messages

Include context that helps developers fix issues:

```rust
#[test]
fn test_realm_values_are_canonical() {
    let valid_realms = ["shared", "org"];

    for node in all_node_kinds() {
        assert!(
            valid_realms.contains(&node.realm.as_str()),
            "Invalid realm '{}' in {}. \nValid realms: {:?}\nv11.2 migration: 'global'→'shared', 'tenant'→'org'",
            node.realm, node.def.name, valid_realms
        );
    }
}
```

---

## Running DX Tests

### Local Development

```bash
# Run all tests (YAML-based, no Neo4j required)
cargo test --lib

# Run specific test category
cargo test terminology
cargo test validation
cargo test naming_pattern

# Run with output
cargo test -- --nocapture
```

### Pre-Commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

set -e

echo "Running DX tests..."
cd tools/novanet
cargo test --lib -- --test-threads=1

echo "Checking terminology..."
cargo clippy -- -D warnings

echo "All checks passed!"
```

### CI/CD Integration

```yaml
# .github/workflows/validate.yml
- name: Validate YAML coherence
  run: cargo run -- schema validate --strict

- name: Run terminology tests
  run: cargo test terminology::*

- name: Snapshot validation
  run: cargo test -- --ignored  # snapshot comparison tests
```

---

## Migrating to New Terminology

### Step 1: Add Tests for New Terms (Red)

```rust
#[test]
fn test_new_trait_values_exist() {
    // This test FAILS until all nodes use new trait names
    assert!(all_node_kinds()
        .iter()
        .all(|n| matches!(n.def.node_trait, NodeTrait::Defined | ...))
    );
}

#[test]
fn test_no_deprecated_traits_remain() {
    // Fails if any "invariant" or "localized" traits exist
    assert!(!any_nodes_with_trait("invariant"));
    assert!(!any_nodes_with_trait("localized"));
}
```

### Step 2: Update YAML (Green)

```yaml
# Before (v11.7)
node:
  name: Page
  trait: invariant

# After (v11.8)
node:
  name: Page
  trait: defined
```

### Step 3: Regenerate + Verify (Refactor)

```bash
cargo run -- schema generate
cargo test
cargo run -- schema validate
```

### Step 4: Update Documentation

Update `.claude/rules/novanet-terminology.md` and ADRs to reflect new terms.

---

## Common Pitfalls & Solutions

### Pitfall 1: Testing Code, Not Concepts

**❌ Bad**:
```rust
#[test]
fn test_node_creation() {
    let node = make_node("Page", "org", "structure", NodeTrait::Defined);
    assert!(node.def.name.len() > 0);  // Tests length, not terminology
}
```

**✅ Good**:
```rust
#[test]
fn test_node_uses_correct_layer() {
    let node = make_node("Page", "org", "structure", NodeTrait::Defined);
    assert_eq!(node.layer, "structure");
    assert_eq!(node.realm, "org");
    // Tests that structure and org are correct layers for Page
}
```

### Pitfall 2: Snapshot Tests Without Context

**❌ Bad**:
```rust
#[test]
fn test_generation() {
    let output = generate_schema();
    insta::assert_snapshot!(output);  // Brittle - any whitespace change fails
}
```

**✅ Good**:
```rust
#[test]
fn test_typescript_export_names_use_new_terminology() {
    let output = generate_typescript();

    // Test what matters: terminology is correct
    assert!(output.contains("export type NodeClass"));
    assert!(!output.contains("export type NodeKind"));

    // Snapshot: this is expected structure
    insta::assert_snapshot!("typescript_class_export", output);
}
```

### Pitfall 3: Ignoring Edge Cases

**❌ Bad**:
```rust
#[test]
fn test_all_nodes_have_traits() {
    assert!(all_node_kinds().iter().all(|n| !n.def.node_trait.is_empty()));
}
```

**✅ Good**:
```rust
#[test]
fn test_trait_distribution_is_healthy() {
    let counts = all_node_kinds()
        .iter()
        .fold(HashMap::new(), |mut acc, n| {
            *acc.entry(n.def.node_trait).or_insert(0) += 1;
            acc
        });

    // Edge case: no trait should be missing
    assert!(counts.get(&NodeTrait::Defined).unwrap_or(&0) > 0);
    assert!(counts.get(&NodeTrait::Generated).unwrap_or(&0) > 0);
    assert!(counts.get(&NodeTrait::Authored).unwrap_or(&0) > 0);
    // etc.
}
```

---

## Summary: Creating DX Tests for Nomenclature

1. **Use reference documents** (CLAUDE.md, ADRs) as test spec, not hardcoded rules
2. **Validate YAML coherence first** (path/content match, trait values)
3. **Test against Neo4j second** (only if data is critical)
4. **Use semantic fixtures** that make terminology visible in test code
5. **Provide actionable fix hints** in validation errors
6. **Use snapshot testing** to catch unintended terminology changes
7. **Test both positive and negative cases** (should exist + should not exist)
8. **Include context in assertions** for faster debugging
9. **Run pre-commit** to catch issues before they reach CI
10. **Document the migration path** when updating terminology (Red → Green → Refactor)

**Key insight**: Terminology is not just style—it's a testable property of system coherence.
