# TDD Implementation Plan: Schema Tools Refactoring

**Date**: 2026-02-17
**Status**: Ready to Execute
**Design Document**: `2026-02-17-schema-tools-refactoring-design.md`
**Context**: Detailed RED-GREEN-REFACTOR cycles for 3 sprints

## Overview

This plan provides step-by-step TDD cycles for implementing the Schema Tools refactoring.
Each cycle follows strict RED-GREEN-REFACTOR discipline.

**Principles**:
1. **RED**: Write failing test FIRST
2. **GREEN**: Write minimal code to pass
3. **REFACTOR**: Improve without changing behavior
4. **NEVER** skip RED phase (no "I'll add tests later")
5. **NEVER** write production code without a failing test

## Sprint 1: Auto-Fix + Suggestions (2-3 days)

### Day 1: Setup + CompositeKeyFixer (RED-GREEN)

#### Cycle 1.1: Create AutoFix Trait (RED)

**Test** (`validation/autofix/mod.rs`):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct TestFixer;

    impl AutoFix for TestFixer {
        fn can_fix(&self, issue: &SchemaIssue) -> bool {
            issue.rule == "TEST_RULE"
        }

        fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue)
            -> Result<FixAction>
        {
            Ok(FixAction::Skipped {
                reason: "test".to_string()
            })
        }

        fn description(&self) -> &str {
            "Test fixer"
        }
    }

    #[test]
    fn test_autofixコtrait_compiles() {
        let fixer = TestFixer;
        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "TEST_RULE",
            message: "test".into(),
        };

        assert!(fixer.can_fix(&issue));
        assert_eq!(fixer.description(), "Test fixer");
    }
}
```

**Expected**: ❌ Compilation error (AutoFix trait doesn't exist)

**Implementation** (GREEN):
```rust
// validation/autofix/mod.rs
use crate::parsers::schema_rules::{SchemaIssue, IssueSeverity};
use crate::parsers::yaml_node::ParsedNode;
use crate::Result;

pub trait AutoFix: Send + Sync {
    fn can_fix(&self, issue: &SchemaIssue) -> bool;
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction>;
    fn description(&self) -> &str;
}

pub enum FixAction {
    Modified { changes: Vec<Change> },
    Skipped { reason: String },
}

pub struct Change {
    pub field: String,
    pub old_value: Option<serde_yaml::Value>,
    pub new_value: serde_yaml::Value,
}
```

**Verify**: ✅ Test passes

#### Cycle 1.2: CompositeKeyFixer - Missing Pattern (RED)

**Test** (`validation/autofix/composite_key.rs`):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::yaml_node::ParsedNode;
    use std::collections::BTreeMap;

    fn create_entity_native_without_pattern() -> ParsedNode {
        let mut node = ParsedNode::default();
        node.def.name = "EntityNative".into();

        let mut key_prop = PropertyDef {
            type_: "string".into(),
            required: true,
            extra: BTreeMap::new(),
        };
        // Missing pattern!

        let mut props = BTreeMap::new();
        props.insert("key".into(), key_prop);
        node.def.standard_properties = Some(props);

        node
    }

    #[test]
    fn test_adds_missing_pattern_for_entity_native() {
        let mut node = create_entity_native_without_pattern();

        let issue = SchemaIssue {
            node_name: "EntityNative".into(),
            severity: IssueSeverity::Warning,
            rule: "COMPOSITE_KEY_FORMAT",
            message: "Composite key node should have 'pattern' regex: ^entity:[^@]+@[a-z]{2}-[A-Z]{2}$".into(),
        };

        let fixer = CompositeKeyFixer;
        assert!(fixer.can_fix(&issue));

        let result = fixer.fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Modified { changes } => {
                assert_eq!(changes.len(), 1);
                assert_eq!(changes[0].field, "key.pattern");

                // Verify pattern was added
                let pattern = node.def.standard_properties.as_ref()
                    .unwrap()
                    .get("key")
                    .unwrap()
                    .extra
                    .get("pattern")
                    .unwrap()
                    .as_str()
                    .unwrap();

                assert_eq!(pattern, "^entity:[^@]+@[a-z]{2}-[A-Z]{2}$");
            }
            _ => panic!("Expected Modified, got {:?}", result),
        }
    }
}
```

**Expected**: ❌ Test fails (CompositeKeyFixer doesn't exist)

**Implementation** (GREEN):
```rust
// validation/autofix/composite_key.rs
use super::{AutoFix, FixAction, Change};
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::ParsedNode;
use crate::Result;
use serde_yaml::Value;

pub struct CompositeKeyFixer;

const COMPOSITE_PATTERNS: &[(&str, &str)] = &[
    ("EntityNative", "^entity:[^@]+@[a-z]{2}-[A-Z]{2}$"),
    ("PageNative", "^page:[^@]+@[a-z]{2}-[A-Z]{2}$"),
    ("BlockNative", "^block:[^@]+@[a-z]{2}-[A-Z]{2}$"),
];

impl AutoFix for CompositeKeyFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "COMPOSITE_KEY_FORMAT"
    }

    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction> {
        let pattern = COMPOSITE_PATTERNS
            .iter()
            .find(|(name, _)| *name == node.def.name)
            .map(|(_, pat)| *pat);

        let Some(pattern) = pattern else {
            return Ok(FixAction::Skipped {
                reason: format!("No pattern defined for {}", node.def.name),
            });
        };

        // Add pattern to key property
        if let Some(props) = node.def.standard_properties.as_mut() {
            if let Some(key_prop) = props.get_mut("key") {
                let old_value = key_prop.extra.get("pattern").cloned();

                key_prop.extra.insert(
                    "pattern".to_string(),
                    Value::String(pattern.to_string()),
                );

                return Ok(FixAction::Modified {
                    changes: vec![Change {
                        field: "key.pattern".into(),
                        old_value,
                        new_value: Value::String(pattern.to_string()),
                    }],
                });
            }
        }

        Ok(FixAction::Skipped {
            reason: "No key property found".into(),
        })
    }

    fn description(&self) -> &str {
        "Adds missing pattern regex for composite key nodes"
    }
}
```

**Verify**: ✅ Test passes

#### Cycle 1.3: CompositeKeyFixer - Invalid Examples (RED)

**Test**:
```rust
#[test]
fn test_fixes_invalid_composite_key_examples() {
    let mut node = create_entity_native_with_bad_examples();

    // Bad examples: "qr-code-instagram" (missing prefix and locale)
    let issue = SchemaIssue {
        node_name: "EntityNative".into(),
        severity: IssueSeverity::Error,
        rule: "COMPOSITE_KEY_FORMAT",
        message: "Example[0] 'qr-code-instagram' should start with 'entity:'".into(),
    };

    let fixer = CompositeKeyFixer;
    let result = fixer.fix(&mut node, &issue).unwrap();

    match result {
        FixAction::Modified { changes } => {
            let examples = node.def.standard_properties.as_ref()
                .unwrap()
                .get("key")
                .unwrap()
                .extra
                .get("examples")
                .unwrap()
                .as_sequence()
                .unwrap();

            // Should be: "entity:qr-code-instagram@fr-FR"
            assert_eq!(
                examples[0].as_str().unwrap(),
                "entity:qr-code-instagram@fr-FR"
            );
        }
        _ => panic!("Expected Modified"),
    }
}
```

**Expected**: ❌ Test fails (example fixing not implemented)

**Implementation** (GREEN):
```rust
impl AutoFix for CompositeKeyFixer {
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction> {
        // ... existing pattern fix ...

        // NEW: Fix examples
        if issue.message.contains("Example[") {
            if let Some(props) = node.def.standard_properties.as_mut() {
                if let Some(key_prop) = props.get_mut("key") {
                    if let Some(Value::Sequence(examples)) = key_prop.extra.get_mut("examples") {
                        let prefix = get_prefix(&node.def.name);

                        for example in examples.iter_mut() {
                            if let Some(s) = example.as_str() {
                                if !s.starts_with(&format!("{}:", prefix)) {
                                    // Add prefix + default locale
                                    *example = Value::String(
                                        format!("{}:{}@fr-FR", prefix, s)
                                    );
                                }
                            }
                        }

                        return Ok(FixAction::Modified {
                            changes: vec![Change {
                                field: "key.examples".into(),
                                old_value: None,
                                new_value: Value::Sequence(examples.clone()),
                            }],
                        });
                    }
                }
            }
        }

        // ... rest of implementation ...
    }
}

fn get_prefix(node_name: &str) -> &str {
    match node_name {
        "EntityNative" => "entity",
        "PageNative" => "page",
        "BlockNative" => "block",
        _ => "unknown",
    }
}
```

**Verify**: ✅ Test passes

### Day 2: PropertyOrderFixer + TimestampFixer (RED-GREEN)

#### Cycle 2.1: PropertyOrderFixer (RED)

**Test** (`validation/autofix/property_order.rs`):
```rust
#[test]
fn test_reorders_standard_properties() {
    let mut node = create_node_with_wrong_order();
    // Current order: description, key, display_name, updated_at, created_at
    // Expected: key, display_name, description, created_at, updated_at

    let issue = SchemaIssue {
        node_name: "TestNode".into(),
        severity: IssueSeverity::Warning,
        rule: "PROP_ORDER",
        message: "Standard properties out of order".into(),
    };

    let fixer = PropertyOrderFixer;
    let result = fixer.fix(&mut node, &issue).unwrap();

    match result {
        FixAction::Modified { .. } => {
            let props = node.def.standard_properties.as_ref().unwrap();
            let keys: Vec<&str> = props.keys().map(|k| k.as_str()).collect();

            assert_eq!(
                keys,
                vec!["key", "display_name", "description", "created_at", "updated_at"]
            );
        }
        _ => panic!("Expected Modified"),
    }
}
```

**Expected**: ❌ Test fails (PropertyOrderFixer doesn't exist)

**Implementation** (GREEN):
```rust
// validation/autofix/property_order.rs
use super::{AutoFix, FixAction, Change};
use crate::parsers::schema_rules::{SchemaIssue, STANDARD_PROPS_ORDER};
use crate::parsers::yaml_node::ParsedNode;
use crate::Result;
use std::collections::BTreeMap;

pub struct PropertyOrderFixer;

impl AutoFix for PropertyOrderFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "PROP_ORDER"
    }

    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction> {
        if let Some(props) = node.def.standard_properties.as_mut() {
            let old_order: Vec<String> = props.keys().cloned().collect();

            // Create new ordered map
            let mut new_props = BTreeMap::new();

            // First: add properties in STANDARD_PROPS_ORDER
            for &standard_key in STANDARD_PROPS_ORDER {
                if let Some(value) = props.remove(standard_key) {
                    new_props.insert(standard_key.to_string(), value);
                }
            }

            // Then: add remaining properties (preserve their order)
            for (key, value) in props.drain() {
                new_props.insert(key, value);
            }

            *props = new_props;

            return Ok(FixAction::Modified {
                changes: vec![Change {
                    field: "standard_properties".into(),
                    old_value: None, // Too complex to serialize
                    new_value: serde_yaml::Value::String(
                        format!("Reordered from {:?}", old_order)
                    ),
                }],
            });
        }

        Ok(FixAction::Skipped {
            reason: "No standard_properties found".into(),
        })
    }

    fn description(&self) -> &str {
        "Reorders standard properties to canonical order"
    }
}
```

**Verify**: ✅ Test passes

#### Cycle 2.2: TimestampFixer (RED)

**Test** (`validation/autofix/timestamps.rs`):
```rust
#[test]
fn test_adds_missing_timestamps() {
    let mut node = create_node_without_timestamps();

    let issue = SchemaIssue {
        node_name: "TestNode".into(),
        severity: IssueSeverity::Error,
        rule: "TIMESTAMP_REQUIRED",
        message: "Missing 'created_at' in standard_properties".into(),
    };

    let fixer = TimestampFixer;
    let result = fixer.fix(&mut node, &issue).unwrap();

    match result {
        FixAction::Modified { changes } => {
            assert!(changes.len() >= 1);

            let props = node.def.standard_properties.as_ref().unwrap();
            assert!(props.contains_key("created_at"));
            assert!(props.contains_key("updated_at"));

            let created_at = &props["created_at"];
            assert_eq!(created_at.type_, "datetime");
            assert_eq!(created_at.required, true);
        }
        _ => panic!("Expected Modified"),
    }
}
```

**Expected**: ❌ Test fails (TimestampFixer doesn't exist)

**Implementation** (GREEN):
```rust
// validation/autofix/timestamps.rs
use super::{AutoFix, FixAction, Change};
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::{ParsedNode, PropertyDef};
use crate::Result;
use std::collections::BTreeMap;
use serde_yaml::Value;

pub struct TimestampFixer;

impl AutoFix for TimestampFixer {
    fn can_fix(&self, issue: &SchemaIssue) -> bool {
        issue.rule == "TIMESTAMP_REQUIRED"
    }

    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction> {
        if let Some(props) = node.def.standard_properties.as_mut() {
            let mut changes = Vec::new();

            // Add created_at if missing
            if !props.contains_key("created_at") {
                props.insert("created_at".to_string(), PropertyDef {
                    type_: "datetime".into(),
                    required: true,
                    extra: BTreeMap::new(),
                });

                changes.push(Change {
                    field: "created_at".into(),
                    old_value: None,
                    new_value: Value::String("datetime (required)".into()),
                });
            }

            // Add updated_at if missing
            if !props.contains_key("updated_at") {
                props.insert("updated_at".to_string(), PropertyDef {
                    type_: "datetime".into(),
                    required: true,
                    extra: BTreeMap::new(),
                });

                changes.push(Change {
                    field: "updated_at".into(),
                    old_value: None,
                    new_value: Value::String("datetime (required)".into()),
                });
            }

            if !changes.is_empty() {
                return Ok(FixAction::Modified { changes });
            }
        }

        Ok(FixAction::Skipped {
            reason: "Timestamps already present".into(),
        })
    }

    fn description(&self) -> &str {
        "Adds missing created_at/updated_at timestamps"
    }
}
```

**Verify**: ✅ Test passes

### Day 3: Integration + CLI (REFACTOR)

#### Cycle 3.1: FixEngine Registry (RED)

**Test** (`validation/autofix/mod.rs`):
```rust
#[test]
fn test_fix_engine_applies_first_matching_fixer() {
    let mut engine = FixEngine::new();
    engine.register(Box::new(CompositeKeyFixer));
    engine.register(Box::new(PropertyOrderFixer));
    engine.register(Box::new(TimestampFixer));

    let mut node = create_entity_native_without_pattern();
    let issue = SchemaIssue {
        node_name: "EntityNative".into(),
        severity: IssueSeverity::Warning,
        rule: "COMPOSITE_KEY_FORMAT",
        message: "Missing pattern".into(),
    };

    let result = engine.apply_fix(&mut node, &issue).unwrap();

    assert!(matches!(result, FixAction::Modified { .. }));
}
```

**Expected**: ❌ Test fails (FixEngine doesn't exist)

**Implementation** (GREEN):
```rust
pub struct FixEngine {
    fixers: Vec<Box<dyn AutoFix>>,
}

impl FixEngine {
    pub fn new() -> Self {
        Self { fixers: Vec::new() }
    }

    pub fn register(&mut self, fixer: Box<dyn AutoFix>) {
        self.fixers.push(fixer);
    }

    pub fn apply_fix(
        &self,
        node: &mut ParsedNode,
        issue: &SchemaIssue,
    ) -> Result<FixAction> {
        for fixer in &self.fixers {
            if fixer.can_fix(issue) {
                return fixer.fix(node, issue);
            }
        }

        Ok(FixAction::Skipped {
            reason: "No fixer available for this issue".into(),
        })
    }
}

impl Default for FixEngine {
    fn default() -> Self {
        let mut engine = Self::new();
        engine.register(Box::new(CompositeKeyFixer));
        engine.register(Box::new(PropertyOrderFixer));
        engine.register(Box::new(TimestampFixer));
        engine
    }
}
```

**Verify**: ✅ Test passes

#### Cycle 3.2: CLI Integration (RED)

**Test** (`commands/schema.rs`):
```rust
#[test]
fn test_schema_validate_with_fix_flag() {
    let root = test_root().unwrap();

    // Create a test node with violations
    let test_node_path = root.join("packages/core/models/node-classes/test-node.yaml");
    fs::write(&test_node_path, "...yaml with violations...").unwrap();

    // Run validation with --fix
    let result = schema_validate_with_fix(&root, FixStrategy::Safe);

    assert!(result.is_ok());

    // Verify violations were fixed
    let content = fs::read_to_string(&test_node_path).unwrap();
    assert!(content.contains("pattern:"));

    // Cleanup
    fs::remove_file(&test_node_path).unwrap();
}
```

**Expected**: ❌ Test fails (schema_validate_with_fix doesn't exist)

**Implementation** (GREEN):
```rust
// commands/schema.rs
pub fn schema_validate_with_fix(
    root: &Path,
    strategy: FixStrategy,
) -> crate::Result<Vec<ValidationIssue>> {
    let mut nodes = crate::parsers::yaml_node::load_all_nodes(root)?;
    let mut issues = Vec::new();
    let mut fixes_applied = 0;

    let engine = FixEngine::default();

    for node in &mut nodes {
        let node_issues = crate::parsers::schema_rules::validate_node(node);

        for issue in node_issues {
            match strategy {
                FixStrategy::DryRun => {
                    // Just collect issues, don't fix
                    issues.push(issue);
                }
                FixStrategy::Safe | FixStrategy::Auto => {
                    match engine.apply_fix(node, &issue)? {
                        FixAction::Modified { changes } => {
                            // Write fixed node back to YAML
                            save_node_to_yaml(node, root)?;
                            fixes_applied += 1;

                            // Still report as fixed issue
                            issues.push(ValidationIssue {
                                severity: Severity::Warning,
                                message: format!(
                                    "{} (FIXED: {})",
                                    issue.message,
                                    changes.iter()
                                        .map(|c| &c.field)
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                ),
                            });
                        }
                        FixAction::Skipped { reason } => {
                            // Can't fix, report as-is
                            issues.push(ValidationIssue {
                                severity: match issue.severity {
                                    IssueSeverity::Error => Severity::Error,
                                    IssueSeverity::Warning => Severity::Warning,
                                },
                                message: format!("{} (Can't auto-fix: {})", issue.message, reason),
                            });
                        }
                    }
                }
            }
        }
    }

    if fixes_applied > 0 {
        eprintln!("✅ Applied {} automatic fixes", fixes_applied);
    }

    Ok(issues)
}
```

**Verify**: ✅ Test passes

#### REFACTOR Phase: Property-Based Testing

**Add proptest**:
```rust
// validation/autofix/composite_key.rs
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn composite_key_pattern_always_matches_generated_examples(
            entity_key in "[a-z-]{3,20}",
            locale in "(en|fr|es|de|ja)-(US|FR|ES|DE|JP)"
        ) {
            let key = format!("entity:{}@{}", entity_key, locale);

            let pattern = "^entity:[^@]+@[a-z]{2}-[A-Z]{2}$";
            let re = regex::Regex::new(pattern).unwrap();

            prop_assert!(re.is_match(&key));
        }

        #[test]
        fn fix_always_produces_valid_pattern(
            node_name in "(EntityNative|PageNative|BlockNative)"
        ) {
            let mut node = create_test_node(&node_name);
            let issue = SchemaIssue {
                node_name: node_name.clone(),
                severity: IssueSeverity::Warning,
                rule: "COMPOSITE_KEY_FORMAT",
                message: "Missing pattern".into(),
            };

            let fixer = CompositeKeyFixer;
            let result = fixer.fix(&mut node, &issue).unwrap();

            if let FixAction::Modified { .. } = result {
                // Verify pattern was added and is valid
                let pattern = node.def.standard_properties
                    .as_ref()
                    .unwrap()
                    .get("key")
                    .unwrap()
                    .extra
                    .get("pattern")
                    .unwrap()
                    .as_str()
                    .unwrap();

                // Pattern should be valid regex
                prop_assert!(regex::Regex::new(pattern).is_ok());
            }
        }
    }
}
```

**Verify**: Run 100+ random test cases → all pass

### Sprint 1 Acceptance

```bash
# Run all tests
cargo nextest run

# Verify coverage
cargo llvm-cov --html

# Check no regressions
git diff --stat

# Commit Sprint 1
git add .
git commit -m "feat: auto-fix engine with 3 fixers (composite key, property order, timestamps)"
```

**✅ Sprint 1 Complete** when:
- All tests pass (including proptest)
- Coverage ≥ 90% for new code
- `novanet schema validate --fix` works
- No performance regression

---

## Sprint 2: Hooks + Plugin Architecture (3-4 days)

### Day 4: Hook System (RED-GREEN)

#### Cycle 4.1: Hook Trait (RED)

**Test** (`validation/hooks.rs`):
```rust
#[test]
fn test_hook_lifecycle_methods_called() {
    struct TestHook {
        calls: Arc<Mutex<Vec<String>>>,
    }

    impl Hook for TestHook {
        fn name(&self) -> &str { "test" }

        fn on_validate_start(&self, _ctx: &ValidationContext) {
            self.calls.lock().unwrap().push("start".into());
        }

        fn on_validate_end(&self, _ctx: &ValidationContext, _issues: &[SchemaIssue]) {
            self.calls.lock().unwrap().push("end".into());
        }

        fn on_fix_applied(&self, _ctx: &ValidationContext, _fix: &FixAction) {
            self.calls.lock().unwrap().push("fix".into());
        }
    }

    let calls = Arc::new(Mutex::new(Vec::new()));
    let hook = TestHook { calls: calls.clone() };

    let ctx = ValidationContext {
        root: PathBuf::from("/test"),
        nodes_count: 10,
        strategy: FixStrategy::Safe,
    };

    hook.on_validate_start(&ctx);
    hook.on_fix_applied(&ctx, &FixAction::Skipped { reason: "test".into() });
    hook.on_validate_end(&ctx, &[]);

    assert_eq!(
        *calls.lock().unwrap(),
        vec!["start", "fix", "end"]
    );
}
```

**Expected**: ❌ Test fails (Hook trait doesn't exist)

**Implementation** (GREEN):
```rust
// validation/hooks.rs
use crate::parsers::schema_rules::SchemaIssue;
use crate::validation::autofix::{FixAction, FixStrategy};
use std::path::PathBuf;

pub trait Hook: Send + Sync {
    fn name(&self) -> &str;

    fn on_validate_start(&self, ctx: &ValidationContext) {
        let _ = ctx; // Default: no-op
    }

    fn on_validate_end(&self, ctx: &ValidationContext, issues: &[SchemaIssue]) {
        let _ = (ctx, issues); // Default: no-op
    }

    fn on_fix_applied(&self, ctx: &ValidationContext, fix: &FixAction) {
        let _ = (ctx, fix); // Default: no-op
    }
}

pub struct ValidationContext {
    pub root: PathBuf,
    pub nodes_count: usize,
    pub strategy: FixStrategy,
}
```

**Verify**: ✅ Test passes

#### Cycle 4.2: HookRegistry (RED)

**Test**:
```rust
#[test]
fn test_hook_registry_triggers_all_hooks_in_order() {
    let calls = Arc::new(Mutex::new(Vec::new()));

    let mut registry = HookRegistry::new();
    registry.register(Box::new(create_numbered_hook(1, calls.clone())));
    registry.register(Box::new(create_numbered_hook(2, calls.clone())));
    registry.register(Box::new(create_numbered_hook(3, calls.clone())));

    let ctx = ValidationContext { /* ... */ };
    registry.trigger_validate_start(&ctx);

    assert_eq!(
        *calls.lock().unwrap(),
        vec![1, 2, 3]
    );
}
```

**Expected**: ❌ Test fails (HookRegistry doesn't exist)

**Implementation** (GREEN):
```rust
pub struct HookRegistry {
    hooks: Vec<Box<dyn Hook>>,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    pub fn register(&mut self, hook: Box<dyn Hook>) {
        self.hooks.push(hook);
    }

    pub fn trigger_validate_start(&self, ctx: &ValidationContext) {
        for hook in &self.hooks {
            hook.on_validate_start(ctx);
        }
    }

    pub fn trigger_validate_end(&self, ctx: &ValidationContext, issues: &[SchemaIssue]) {
        for hook in &self.hooks {
            hook.on_validate_end(ctx, issues);
        }
    }

    pub fn trigger_fix_applied(&self, ctx: &ValidationContext, fix: &FixAction) {
        for hook in &self.hooks {
            hook.on_fix_applied(ctx, fix);
        }
    }
}
```

**Verify**: ✅ Test passes

### Day 5: Built-in Hooks (RED-GREEN)

#### Cycle 5.1: ProgressHook (RED)

**Test** (`validation/hooks/progress.rs`):
```rust
#[test]
fn test_progress_hook_updates_bar() {
    let hook = ProgressHook::new();

    let ctx = ValidationContext {
        root: PathBuf::from("/test"),
        nodes_count: 100,
        strategy: FixStrategy::Safe,
    };

    hook.on_validate_start(&ctx);
    // Should create progress bar with length 100

    for i in 0..100 {
        hook.on_fix_applied(&ctx, &FixAction::Skipped { reason: "test".into() });
        // Should increment progress
    }

    hook.on_validate_end(&ctx, &[]);
    // Should finish progress bar
}
```

**Implementation** (GREEN):
```rust
// validation/hooks/progress.rs
use super::{Hook, ValidationContext};
use crate::parsers::schema_rules::SchemaIssue;
use crate::validation::autofix::FixAction;
use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressHook {
    bar: ProgressBar,
}

impl ProgressHook {
    pub fn new() -> Self {
        Self {
            bar: ProgressBar::hidden(),
        }
    }
}

impl Hook for ProgressHook {
    fn name(&self) -> &str {
        "progress"
    }

    fn on_validate_start(&self, ctx: &ValidationContext) {
        self.bar.set_length(ctx.nodes_count as u64);
        self.bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bar:40}] {pos}/{len} {msg}")
                .unwrap()
        );
        self.bar.set_message("Validating schema...");
    }

    fn on_fix_applied(&self, _ctx: &ValidationContext, fix: &FixAction) {
        match fix {
            FixAction::Modified { .. } => {
                self.bar.inc(1);
                self.bar.set_message("Fixing...");
            }
            FixAction::Skipped { .. } => {
                self.bar.inc(1);
            }
        }
    }

    fn on_validate_end(&self, _ctx: &ValidationContext, issues: &[SchemaIssue]) {
        self.bar.finish_with_message(format!("Found {} issues", issues.len()));
    }
}
```

**Verify**: ✅ Test passes

#### Cycle 5.2: MetricsHook (Snapshot Test)

**Test**:
```rust
#[test]
fn test_metrics_hook_output() {
    let hook = MetricsHook::new();

    let ctx = ValidationContext { /* ... */ };
    let issues = vec![/* test issues */];

    let output = hook.generate_report(&ctx, &issues);

    insta::assert_snapshot!(output);
}
```

**Implementation**: (GREEN - implement MetricsHook)

**Verify**: Review snapshot with `cargo insta review`

### Days 6-7: Refactor schema_rules.rs → Plugin Architecture

#### Cycle 6.1: ValidationRule Trait (RED)

**Test** (`validation/rules/mod.rs`):
```rust
#[test]
fn test_validation_rule_trait() {
    struct TestRule;

    impl ValidationRule for TestRule {
        fn name(&self) -> &str { "TEST" }

        fn validate(&self, node: &ParsedNode) -> Vec<SchemaIssue> {
            vec![SchemaIssue {
                node_name: node.def.name.clone(),
                severity: IssueSeverity::Warning,
                rule: "TEST",
                message: "test".into(),
            }]
        }
    }

    let rule = TestRule;
    let node = create_test_node();
    let issues = rule.validate(&node);

    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].rule, "TEST");
}
```

**Implementation** (GREEN):
```rust
// validation/rules/mod.rs
pub trait ValidationRule: Send + Sync {
    fn name(&self) -> &str;
    fn validate(&self, node: &ParsedNode) -> Vec<SchemaIssue>;
}

pub struct RuleRegistry {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn register(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
    }

    pub fn validate_node(&self, node: &ParsedNode) -> Vec<SchemaIssue> {
        self.rules
            .iter()
            .flat_map(|rule| rule.validate(node))
            .collect()
    }
}
```

#### Cycle 6.2: Extract CompositeKeyRule (REFACTOR)

**Before** (schema_rules.rs lines 176-317):
```rust
// 145 lines of composite key validation in validate_node()
```

**After** (`validation/rules/composite_key.rs`):
```rust
pub struct CompositeKeyRule;

impl ValidationRule for CompositeKeyRule {
    fn name(&self) -> &str {
        "COMPOSITE_KEY_FORMAT"
    }

    fn validate(&self, node: &ParsedNode) -> Vec<SchemaIssue> {
        // Extract 145 lines from schema_rules.rs
        // Keep exact same logic, just isolated
    }
}
```

**Verify**: All existing tests still pass

### Sprint 2 Acceptance

```bash
cargo nextest run
cargo insta review  # Review snapshot tests
cargo clippy -- -D warnings

git commit -m "feat: hooks system + plugin architecture with 5 validation rules refactored"
```

**✅ Sprint 2 Complete** when:
- Hook system with 4+ hooks
- ValidationRule trait with 5 rules
- All existing tests pass
- Snapshot tests approved

---

## Sprint 3: Performance + Advanced Features (2-3 days)

### Day 8: Parallel Validation (RED-GREEN)

#### Cycle 8.1: Rayon Parallel Validation (RED)

**Benchmark Test**:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_validation(c: &mut Criterion) {
    let root = test_root().unwrap();
    let nodes = load_all_nodes(&root).unwrap();

    c.bench_function("sequential", |b| {
        b.iter(|| {
            validate_all_nodes_sequential(black_box(&nodes))
        })
    });

    c.bench_function("parallel", |b| {
        b.iter(|| {
            validate_all_nodes_parallel(black_box(&nodes))
        })
    });
}

criterion_group!(benches, bench_validation);
criterion_main!(benches);
```

**Expected**: ❌ Parallel version doesn't exist yet

**Implementation** (GREEN):
```rust
// validation/engine.rs
use rayon::prelude::*;

pub fn validate_all_nodes_parallel(
    nodes: &[ParsedNode],
    rules: &RuleRegistry,
) -> Vec<SchemaIssue> {
    nodes
        .par_iter()
        .flat_map(|node| rules.validate_node(node))
        .collect()
}
```

**Verify**: ✅ Benchmark shows 4x speedup

### Day 9: Quality Metrics (RED-GREEN)

#### Cycle 9.1: QualityMetric Trait

**Test** (`quality/metrics.rs`):
```rust
#[test]
fn test_validation_coverage_metric() {
    let nodes = load_test_nodes();
    let issues = vec![/* some issues */];

    let metric = ValidationCoverage;
    let value = metric.collect(&nodes, &issues);

    match value {
        MetricValue::Percentage(pct) => {
            assert!(pct >= 0.0 && pct <= 100.0);
        }
        _ => panic!("Expected Percentage"),
    }
}
```

**Implementation** (GREEN): Implement QualityMetric trait + 4 metrics

### Day 10: HTML Report + Mutation Testing

#### Cycle 10.1: HTML Reporter (REFACTOR)

**Snapshot Test**:
```rust
#[test]
fn test_html_report_generation() {
    let stats = create_test_stats();
    let html = HtmlReporter::generate(&stats);

    insta::assert_snapshot!(html);
}
```

**Implementation**: Template engine for HTML generation

#### Cycle 10.2: Mutation Testing

**Run cargo-mutants**:
```bash
# Find test gaps
cargo mutants

# Expected: 0 mutants survived
# If mutants survive → add tests to catch them
```

### Sprint 3 Acceptance

```bash
# Benchmarks
cargo bench

# Mutation testing
cargo mutants

# Coverage
cargo llvm-cov --html --open

git commit -m "feat: 4x parallel validation, quality metrics, HTML reports, mutation testing"
```

**✅ Sprint 3 Complete** when:
- 4x speedup via rayon
- Quality metrics dashboard
- HTML report generation
- 0 mutants survived

---

## Definition of Done

### All Sprints

- ✅ All tests pass (`cargo nextest run`)
- ✅ Zero clippy warnings (`cargo clippy -- -D warnings`)
- ✅ Formatted (`cargo fmt --check`)
- ✅ No dependency vulnerabilities (`cargo deny check && cargo audit`)
- ✅ Documentation updated
- ✅ Examples added
- ✅ CHANGELOG.md updated

### v0.14.0 Release Checklist

- ✅ All 3 sprints complete
- ✅ cargo-semver-checks: no breaking changes
- ✅ Mutation testing: 0 survivors
- ✅ Coverage ≥ 90%
- ✅ Benchmarks show 4x speedup
- ✅ User documentation complete
- ✅ Migration guide written
- ✅ Tag + release notes

---

## Troubleshooting

### Test Fails

1. Read error message carefully
2. Check if test is RED (expected) or regression (bug)
3. If RED: implement minimal code to pass
4. If regression: fix the bug, ensure test passes

### Can't Make Test Pass

- Check test assumptions (is it even possible?)
- Simplify the test (too ambitious?)
- Ask for help (pair programming)

### Refactor Breaks Tests

- Stop refactoring immediately
- Revert to last green state
- Make smaller refactoring steps
- Run tests after each tiny change

---

**Next Step**: Mark this plan approved → Execute Sprint 1 Cycle 1.1 (RED)
