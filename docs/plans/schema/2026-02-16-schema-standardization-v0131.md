# Schema Standardization v0.13.1 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Standardize all 61 node-class YAML files with consistent property ordering, fix missing denormalized properties, create Rust validation tests, and regenerate seeds.

**Architecture:** Define a canonical 6-BLOC YAML structure, implement Rust validation that fails CI if schemas deviate, fix PageNative/BlockNative missing properties, and migrate 24 legacy-format files to v0.13.1 LLM-First format.

**Tech Stack:** Rust (serde, indexmap), YAML, Cypher, cargo test, cargo clippy

---

## Phase 0: Define Canonical Schema Standard

### Task 0.1: Create Schema Standard Document

**Files:**
- Create: `.claude/rules/schema-standard.md`

**Step 1: Write the schema standard document**

```markdown
# NovaNet Schema Standard v0.13.1

## Canonical YAML Structure

Every node-class YAML file MUST follow this exact structure:

### BLOC Order (6 BLOCs)

```yaml
node:
  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 1: IDENTITY (required, order: name → realm → layer → trait)
  # ═══════════════════════════════════════════════════════════════════════════
  name: NodeName
  realm: shared | org
  layer: config | locale | geography | knowledge | foundation | structure | semantic | instruction | output
  trait: defined | authored | imported | generated | retrieved

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 2: SEMANTIC (required)
  # ═══════════════════════════════════════════════════════════════════════════
  description: "One-line description"

  llm_context: |
    USE: when [primary use case].
    TRIGGERS: "keyword1", "keyword2".
    NOT: for [disambiguation] (use [alternative] instead).
    RELATES: [Source] (role), [Target] (role).

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 3: VISUAL (required)
  # ═══════════════════════════════════════════════════════════════════════════
  icon:
    web: lucide-icon-name
    terminal: "◆"

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 4: DATA (required)
  # ═══════════════════════════════════════════════════════════════════════════
  standard_properties:
    # Order: key → *_key (denormalized) → display_name → description → llm_context → created_at → updated_at
    key:
      type: string
      required: true
      # ... pattern, examples, etc.

    display_name:
      type: string
      required: true

    description:
      type: string
      required: true

    created_at:
      type: datetime
      required: true

    updated_at:
      type: datetime
      required: true

  properties:
    # Node-specific properties in logical groupings

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 5: GRAPH (optional but recommended)
  # ═══════════════════════════════════════════════════════════════════════════
  relations:
    ARC_NAME:
      to: TargetNode
      cardinality: "1:N"
      description: "..."

  incoming_relations:
    ARC_NAME:
      from: SourceNode
      cardinality: "N:1"
      description: "..."

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 6: REFERENCE (optional but recommended)
  # ═══════════════════════════════════════════════════════════════════════════
  example:
    data:
      key: "example-key"
      # ...
    cypher: |
      // Example query
```

## Standard Properties Order

For ALL nodes:
1. `key` (if node has identity)
2. `*_key` denormalized properties (for composite keys: entity_key, page_key, block_key, locale_key)
3. `display_name`
4. `description`
5. `created_at`
6. `updated_at`

**Note:** `llm_context` is at BLOC 2 level (schema metadata), NOT in standard_properties.

## Composite Key Nodes

Nodes with composite keys (`entity:{key}@{locale}`, `page:{key}@{locale}`, `block:{key}@{locale}`) MUST have:
- `key`: Full composite key
- `{parent}_key`: Denormalized parent key (entity_key, page_key, block_key)
- `locale_key`: Denormalized locale key

## Nodes Without Key (Satellites)

These 8 nodes are intentionally without `key` — identified by relation chain:
- `project-native` (Project→HAS_NATIVE→ProjectNative→FOR_LOCALE→Locale)
- `block-rules` (BlockType→HAS_RULES→BlockRules)
- `*-set` containers (Locale→HAS_*→Set + domain property)
```

**Step 2: Commit the standard**

```bash
git add .claude/rules/schema-standard.md
git commit -m "docs: add schema standard v0.13.1"
```

---

## Phase 1: Fix Critical Missing Properties

### Task 1.1: Add denormalized properties to PageNative

**Files:**
- Modify: `packages/core/models/node-classes/org/output/page-native.yaml`

**Step 1: Read current file to understand structure**

Run: `cat packages/core/models/node-classes/org/output/page-native.yaml | head -80`

**Step 2: Add page_key and locale_key after key property**

Add these properties to `standard_properties` section, after `key`:

```yaml
  standard_properties:
    key:
      type: string
      required: true
      pattern: "^page:[a-z][a-z0-9-]*@[a-z]{2}-[A-Z]{2}$"
      description: "Composite key: page:{page_key}@{locale_key}"
      examples:
        - "page:pricing@fr-FR"
        - "page:features@ja-JP"

    # ADD THESE TWO PROPERTIES:
    page_key:
      type: string
      required: true
      pattern: "^[a-z][a-z0-9-]*$"
      indexed: true
      description: "Parent Page key (denormalized for fast lookup)"
      examples:
        - "pricing"
        - "features"

    locale_key:
      type: string
      required: true
      pattern: "^[a-z]{2}-[A-Z]{2}$"
      indexed: true
      description: "Target Locale key (denormalized). BCP 47 format."
      examples:
        - "fr-FR"
        - "en-US"
        - "ja-JP"

    display_name:
      # ... rest unchanged
```

**Step 3: Update example section to include new properties**

```yaml
  example:
    key: "page:pricing@fr-FR"
    page_key: "pricing"           # ADD
    locale_key: "fr-FR"           # ADD
    display_name: "Page Tarifs (fr-FR)"
    # ... rest unchanged
```

**Step 4: Add Neo4j indexes for denormalized properties**

```yaml
  neo4j:
    indexes:
      - "CREATE INDEX pagenative_status IF NOT EXISTS FOR (pn:PageNative) ON (pn.status)"
      - "CREATE INDEX pagenative_date IF NOT EXISTS FOR (pn:PageNative) ON (pn.assembled_at)"
      - "CREATE INDEX pagenative_page_key IF NOT EXISTS FOR (pn:PageNative) ON (pn.page_key)"      # ADD
      - "CREATE INDEX pagenative_locale_key IF NOT EXISTS FOR (pn:PageNative) ON (pn.locale_key)"  # ADD
```

**Step 5: Run schema validate to verify**

Run: `cd tools/novanet && cargo run -- schema validate`
Expected: No errors related to PageNative

**Step 6: Commit**

```bash
git add packages/core/models/node-classes/org/output/page-native.yaml
git commit -m "fix(schema): add page_key and locale_key to PageNative"
```

---

### Task 1.2: Add denormalized properties to BlockNative

**Files:**
- Modify: `packages/core/models/node-classes/org/output/block-native.yaml`

**Step 1: Add block_key and locale_key after key property**

Add these properties to `standard_properties` section, after `key`:

```yaml
  standard_properties:
    key:
      type: string
      required: true
      pattern: "^block:[a-z][a-z0-9-]*@[a-z]{2}-[A-Z]{2}$"
      description: "Composite key: block:{block_key}@{locale_key}"
      examples:
        - "block:pricing-hero@fr-FR"
        - "block:feature-list@ja-JP"

    # ADD THESE TWO PROPERTIES:
    block_key:
      type: string
      required: true
      pattern: "^[a-z][a-z0-9-]*$"
      indexed: true
      description: "Parent Block key (denormalized for fast lookup)"
      examples:
        - "pricing-hero"
        - "feature-list"

    locale_key:
      type: string
      required: true
      pattern: "^[a-z]{2}-[A-Z]{2}$"
      indexed: true
      description: "Target Locale key (denormalized). BCP 47 format."
      examples:
        - "fr-FR"
        - "en-US"
        - "ja-JP"

    display_name:
      # ... rest unchanged
```

**Step 2: Update example section**

```yaml
  example:
    key: "block:pricing-hero@fr-FR"
    block_key: "pricing-hero"     # ADD
    locale_key: "fr-FR"           # ADD
    display_name: "Pricing Hero (fr-FR)"
    # ... rest unchanged
```

**Step 3: Add Neo4j indexes**

```yaml
  neo4j:
    indexes:
      - "CREATE INDEX blocknative_status IF NOT EXISTS FOR (bn:BlockNative) ON (bn.status)"
      - "CREATE INDEX blocknative_date IF NOT EXISTS FOR (bn:BlockNative) ON (bn.generated_at)"
      - "CREATE INDEX blocknative_block_key IF NOT EXISTS FOR (bn:BlockNative) ON (bn.block_key)"    # ADD
      - "CREATE INDEX blocknative_locale_key IF NOT EXISTS FOR (bn:BlockNative) ON (bn.locale_key)"  # ADD
```

**Step 4: Run schema validate**

Run: `cd tools/novanet && cargo run -- schema validate`
Expected: No errors related to BlockNative

**Step 5: Commit**

```bash
git add packages/core/models/node-classes/org/output/block-native.yaml
git commit -m "fix(schema): add block_key and locale_key to BlockNative"
```

---

## Phase 2: Rust Validation Infrastructure

### Task 2.1: Create schema validation module

**Files:**
- Create: `tools/novanet/src/parsers/schema_rules.rs`
- Modify: `tools/novanet/src/parsers/mod.rs`

**Step 1: Write the failing test first**

Create test in `tools/novanet/src/parsers/schema_rules.rs`:

```rust
//! Schema validation rules for v0.13.1 standardization.
//!
//! Validates:
//! - BLOC structure (6 BLOCs in order)
//! - Standard properties order
//! - Composite key denormalization
//! - llm_context presence

use crate::parsers::yaml_node::ParsedNode;
use std::collections::HashSet;

/// Validation issue for schema standardization.
#[derive(Debug, Clone)]
pub struct SchemaIssue {
    pub node_name: String,
    pub severity: IssueSeverity,
    pub rule: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    Error,
    Warning,
}

/// Nodes that intentionally don't have a `key` property.
const KEYLESS_NODES: &[&str] = &[
    "ProjectNative",
    "BlockRules",
    "TermSet",
    "ExpressionSet",
    "PatternSet",
    "CultureSet",
    "TabooSet",
    "AudienceSet",
];

/// Nodes with composite keys that MUST have denormalized properties.
const COMPOSITE_KEY_NODES: &[(&str, &[&str])] = &[
    ("EntityNative", &["entity_key", "locale_key"]),
    ("PageNative", &["page_key", "locale_key"]),
    ("BlockNative", &["block_key", "locale_key"]),
];

/// Expected order for standard properties.
const STANDARD_PROPS_ORDER: &[&str] = &[
    "key",
    "entity_key",
    "page_key",
    "block_key",
    "locale_key",
    "display_name",
    "description",
    "created_at",
    "updated_at",
];

/// Validate a single node against schema rules.
pub fn validate_node(node: &ParsedNode) -> Vec<SchemaIssue> {
    let mut issues = Vec::new();

    // Rule 1: Check for key property (unless in KEYLESS_NODES)
    if !KEYLESS_NODES.contains(&node.def.name.as_str()) {
        if let Some(ref sp) = node.def.standard_properties {
            if !sp.contains_key("key") {
                issues.push(SchemaIssue {
                    node_name: node.def.name.clone(),
                    severity: IssueSeverity::Error,
                    rule: "KEY_REQUIRED",
                    message: "Missing 'key' property in standard_properties".into(),
                });
            }
        } else {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "STANDARD_PROPS_REQUIRED",
                message: "Missing standard_properties section".into(),
            });
        }
    }

    // Rule 2: Composite key nodes must have denormalized properties
    for (composite_node, required_props) in COMPOSITE_KEY_NODES {
        if node.def.name == *composite_node {
            if let Some(ref sp) = node.def.standard_properties {
                for prop in *required_props {
                    if !sp.contains_key(*prop) {
                        issues.push(SchemaIssue {
                            node_name: node.def.name.clone(),
                            severity: IssueSeverity::Error,
                            rule: "DENORM_REQUIRED",
                            message: format!(
                                "Composite key node missing denormalized property: {}",
                                prop
                            ),
                        });
                    }
                }
            }
        }
    }

    // Rule 3: Check timestamps
    if let Some(ref sp) = node.def.standard_properties {
        if !sp.contains_key("created_at") {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "TIMESTAMP_REQUIRED",
                message: "Missing 'created_at' in standard_properties".into(),
            });
        }
        if !sp.contains_key("updated_at") {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Error,
                rule: "TIMESTAMP_REQUIRED",
                message: "Missing 'updated_at' in standard_properties".into(),
            });
        }
    }

    // Rule 4: Check property order
    if let Some(ref sp) = node.def.standard_properties {
        let actual_order: Vec<&str> = sp.keys().map(|k| k.as_str()).collect();
        let expected_in_actual: Vec<&str> = STANDARD_PROPS_ORDER
            .iter()
            .filter(|p| actual_order.contains(p))
            .copied()
            .collect();

        let actual_filtered: Vec<&str> = actual_order
            .iter()
            .filter(|p| STANDARD_PROPS_ORDER.contains(p))
            .copied()
            .collect();

        if expected_in_actual != actual_filtered {
            issues.push(SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Warning,
                rule: "PROP_ORDER",
                message: format!(
                    "Standard properties out of order. Expected: {:?}, Got: {:?}",
                    expected_in_actual, actual_filtered
                ),
            });
        }
    }

    issues
}

/// Validate all nodes against schema rules.
pub fn validate_all_nodes(nodes: &[ParsedNode]) -> Vec<SchemaIssue> {
    nodes.iter().flat_map(validate_node).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_root() -> Option<std::path::PathBuf> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());
        let root = root?;
        if !root.join("pnpm-workspace.yaml").exists() {
            return None;
        }
        Some(root.to_path_buf())
    }

    #[test]
    fn all_nodes_pass_schema_rules() {
        let Some(root) = test_root() else { return };

        let nodes = crate::parsers::yaml_node::load_all_nodes(&root)
            .expect("should load all nodes");

        let issues = validate_all_nodes(&nodes);

        // Filter to errors only
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| i.severity == IssueSeverity::Error)
            .collect();

        if !errors.is_empty() {
            for err in &errors {
                eprintln!("ERROR [{}] {}: {}", err.rule, err.node_name, err.message);
            }
            panic!("Found {} schema errors", errors.len());
        }
    }

    #[test]
    fn composite_key_nodes_have_denormalized_props() {
        let Some(root) = test_root() else { return };

        let nodes = crate::parsers::yaml_node::load_all_nodes(&root)
            .expect("should load all nodes");

        for (node_name, required_props) in COMPOSITE_KEY_NODES {
            let node = nodes.iter().find(|n| n.def.name == *node_name);
            assert!(node.is_some(), "Node {} not found", node_name);

            let node = node.unwrap();
            let sp = node.def.standard_properties.as_ref()
                .expect(&format!("{} should have standard_properties", node_name));

            for prop in *required_props {
                assert!(
                    sp.contains_key(*prop),
                    "{} missing denormalized property: {}",
                    node_name, prop
                );
            }
        }
    }
}
```

**Step 2: Add module to mod.rs**

Add to `tools/novanet/src/parsers/mod.rs`:

```rust
pub mod schema_rules;
```

**Step 3: Run test to verify it fails (before fixing PageNative/BlockNative)**

Run: `cd tools/novanet && cargo test schema_rules --no-fail-fast`
Expected: FAIL with "PageNative missing denormalized property: page_key"

**Step 4: After fixing Phase 1, run test to verify it passes**

Run: `cd tools/novanet && cargo test schema_rules`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet/src/parsers/schema_rules.rs tools/novanet/src/parsers/mod.rs
git commit -m "feat(rust): add schema validation rules v0.13.1"
```

---

### Task 2.2: Integrate schema rules into `schema validate` command

**Files:**
- Modify: `tools/novanet/src/commands/schema.rs`

**Step 1: Import and use schema_rules in validate function**

Add to `schema_validate` function after existing validations:

```rust
// At top of file:
use crate::parsers::schema_rules;

// In schema_validate function, after existing checks:

    // 7. Validate schema standardization rules (v0.13.1)
    let schema_issues = schema_rules::validate_all_nodes(&nodes);
    for issue in schema_issues {
        let severity = match issue.severity {
            schema_rules::IssueSeverity::Error => Severity::Error,
            schema_rules::IssueSeverity::Warning => Severity::Warning,
        };
        issues.push(ValidationIssue {
            severity,
            message: format!("[{}] {}: {}", issue.rule, issue.node_name, issue.message),
        });
    }

    Ok(issues)
```

**Step 2: Run schema validate**

Run: `cd tools/novanet && cargo run -- schema validate`
Expected: No errors (after Phase 1 fixes)

**Step 3: Commit**

```bash
git add tools/novanet/src/commands/schema.rs
git commit -m "feat(cli): integrate schema rules into validate command"
```

---

## Phase 3: Regenerate Seeds

### Task 3.1: Regenerate all artifacts

**Files:**
- Generated: `packages/db/seed/01-classes.cypher`
- Generated: `packages/db/seed/02-arc-classes.cypher`
- Generated: Multiple TypeScript files

**Step 1: Run schema generate**

Run: `cd tools/novanet && cargo run -- schema generate`
Expected: All 12 generators succeed

**Step 2: Verify generated Cypher includes new indexes**

Run: `grep -A2 "pagenative_page_key\|blocknative_block_key" packages/db/seed/01-classes.cypher`
Expected: New index creation statements

**Step 3: Commit generated files**

```bash
git add packages/db/seed/ packages/core/src/graph/ apps/studio/src/design/ tools/novanet/src/tui/icons.rs tools/novanet/src/tui/colors.generated.rs
git commit -m "chore(seed): regenerate artifacts with PageNative/BlockNative indexes"
```

---

### Task 3.2: Run full test suite

**Step 1: Run Rust tests**

Run: `cd tools/novanet && cargo test`
Expected: All 1031+ tests pass

**Step 2: Run clippy**

Run: `cd tools/novanet && cargo clippy -- -D warnings`
Expected: No warnings

**Step 3: Run TypeScript checks**

Run: `pnpm type-check && pnpm lint`
Expected: No errors

**Step 4: Commit if any fixes needed**

```bash
git add -A
git commit -m "fix: address test/lint issues from schema standardization"
```

---

## Phase 4: Format Migration (24 files)

### Task 4.1: Create migration script

**Files:**
- Create: `tools/scripts/migrate-schema-format.sh`

**Step 1: Write migration helper script**

```bash
#!/usr/bin/env bash
# migrate-schema-format.sh — List files needing v0.13.1 format migration

set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

echo "=== Files needing v0.13.1 BLOC format migration ==="
echo ""

for f in $(find packages/core/models/node-classes -name "*.yaml" | sort); do
  if ! grep -q "# BLOC 1:" "$f" 2>/dev/null; then
    echo "  $(basename "$f" .yaml)"
  fi
done

echo ""
echo "Total: $(find packages/core/models/node-classes -name '*.yaml' -exec grep -L '# BLOC 1:' {} \; | wc -l | tr -d ' ') files"
```

**Step 2: Make executable and run**

Run: `chmod +x tools/scripts/migrate-schema-format.sh && ./tools/scripts/migrate-schema-format.sh`
Expected: List of 24 files needing migration

**Step 3: Commit script**

```bash
git add tools/scripts/migrate-schema-format.sh
git commit -m "chore(scripts): add schema format migration helper"
```

---

### Task 4.2: Migrate entity.yaml to v0.13.1 format (example)

**Files:**
- Modify: `packages/core/models/node-classes/org/semantic/entity.yaml`

**Step 1: Add BLOC comments**

Wrap existing sections with BLOC comments:

```yaml
node:
  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 1: IDENTITY (name, realm, layer, trait)
  # ═══════════════════════════════════════════════════════════════════════════
  name: Entity
  realm: org
  layer: semantic
  trait: defined

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 2: SEMANTIC (description, llm_context)
  # ═══════════════════════════════════════════════════════════════════════════
  description: "Semantic unit representing products, features, concepts, actions, and tools"

  llm_context: |
    USE: when loading semantic entity definitions for content generation or knowledge graph queries.
    TRIGGERS: "entity", "semantic", "concept", "product", "feature".
    NOT: for locale-specific content (use EntityNative), for pages (use Page).
    RELATES: Project (owner via HAS_ENTITY), EntityNative (content via HAS_NATIVE), EntityCategory (type via BELONGS_TO), Page (canonical via REPRESENTED_BY).

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 3: VISUAL (icon)
  # ═══════════════════════════════════════════════════════════════════════════
  icon:
    web: "diamond"
    terminal: "◆"

  # ═══════════════════════════════════════════════════════════════════════════
  # BLOC 4: DATA (standard_properties, properties)
  # ═══════════════════════════════════════════════════════════════════════════
  standard_properties:
    # ... (move llm_context OUT of here to BLOC 2)
```

**Step 2: Move llm_context from standard_properties to BLOC 2**

Remove `llm_context` from `standard_properties` section (it's now at node level in BLOC 2).

**Step 3: Verify parse**

Run: `cd tools/novanet && cargo run -- schema validate`
Expected: No errors for Entity

**Step 4: Commit**

```bash
git add packages/core/models/node-classes/org/semantic/entity.yaml
git commit -m "refactor(schema): migrate entity.yaml to v0.13.1 BLOC format"
```

---

### Task 4.3-4.25: Migrate remaining 23 files

Repeat Task 4.2 pattern for each of these files:

1. `entity-native.yaml`
2. `audience-persona.yaml`
3. `channel-surface.yaml`
4. `block-instruction.yaml`
5. `block-rules.yaml`
6. `block-type.yaml`
7. `prompt-artifact.yaml`
8. `block-native.yaml`
9. `output-artifact.yaml`
10. `page-native.yaml`
11. `audience-trait.yaml`
12. `culture-ref.yaml`
13. `expression.yaml`
14. `pattern.yaml`
15. `taboo.yaml`
16. `term.yaml`
17. `population-sub-cluster.yaml`
18. `seo-keyword-metrics.yaml`
19. `seo-keyword-set.yaml`
20. `seo-keyword.yaml`
21. `taboo-set.yaml`
22. `term-set.yaml`
23. `style.yaml`

**Batch commit after each 5 files:**

```bash
git add packages/core/models/node-classes/
git commit -m "refactor(schema): migrate batch N to v0.13.1 BLOC format"
```

---

## Phase 5: Final Validation

### Task 5.1: Run complete validation suite

**Step 1: Schema validate**

Run: `cd tools/novanet && cargo run -- schema validate --strict`
Expected: 0 errors, 0 warnings

**Step 2: Full test suite**

Run: `cd tools/novanet && cargo test`
Expected: All tests pass

**Step 3: Regenerate and verify no diff**

Run: `cd tools/novanet && cargo run -- schema generate && git diff --stat`
Expected: No changes (artifacts already up to date)

**Step 4: Migration script shows 0 files**

Run: `./tools/scripts/migrate-schema-format.sh`
Expected: "Total: 0 files"

---

### Task 5.2: Update CLAUDE.md with new version

**Files:**
- Modify: `CLAUDE.md`
- Modify: `tools/novanet/CLAUDE.md`

**Step 1: Update version references**

Change `v0.13.0` to `v0.13.1` where applicable, noting:
- Schema standardization complete
- All 61 nodes use v0.13.1 BLOC format
- PageNative/BlockNative have denormalized properties

**Step 2: Commit**

```bash
git add CLAUDE.md tools/novanet/CLAUDE.md
git commit -m "docs: update to v0.13.1 schema standardization"
```

---

## Verification Checklist

- [ ] All 61 YAML files use BLOC format
- [ ] PageNative has `page_key` and `locale_key`
- [ ] BlockNative has `block_key` and `locale_key`
- [ ] `cargo run -- schema validate --strict` passes
- [ ] All 1031+ Rust tests pass
- [ ] `pnpm type-check && pnpm lint` passes
- [ ] Seeds regenerated
- [ ] CLAUDE.md updated

---

**Total estimated tasks:** 30 bite-sized steps
**Commit frequency:** Every 1-2 tasks
