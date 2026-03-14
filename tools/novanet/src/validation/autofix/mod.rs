//! Auto-fix engine for schema validation violations.
//!
//! This module provides the AutoFix trait and implementations for automatically
//! correcting common validation issues.

use crate::Result;
use crate::parsers::schema_rules::SchemaIssue;
use crate::parsers::yaml_node::ParsedNode;
use serde_yaml::Value;

pub mod composite_key;
pub mod denormalized_key;
pub mod description;
pub mod example_data;
pub mod property_order;
pub mod timestamps;

/// Trait for auto-fixing validation issues.
pub trait AutoFix: Send + Sync {
    /// Check if this fixer can handle the issue.
    fn can_fix(&self, issue: &SchemaIssue) -> bool;

    /// Apply the fix to the node.
    fn fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction>;

    /// Human-readable description of what this fixer does.
    fn description(&self) -> &str;
}

/// Result of applying a fix.
#[derive(Debug, Clone)]
pub enum FixAction {
    /// Fix was applied successfully.
    Modified { changes: Vec<Change> },
    /// Fix was not applied (e.g., not applicable).
    Skipped { reason: String },
}

/// A single change made by a fixer.
#[derive(Debug, Clone)]
pub struct Change {
    /// Field that was changed (e.g., "key.pattern").
    pub field: String,
    /// Old value (if any).
    pub old_value: Option<Value>,
    /// New value.
    pub new_value: Value,
}

/// Strategy for applying fixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FixStrategy {
    /// Only apply fixes with 100% confidence (default).
    #[default]
    Safe,
    /// Apply all available fixes aggressively.
    Auto,
    /// Preview fixes without writing (dry-run).
    DryRun,
}

// ─────────────────────────────────────────────────────────────────────────────
// FixEngine (GREEN Phase)
// ─────────────────────────────────────────────────────────────────────────────

use composite_key::CompositeKeyFixer;
use denormalized_key::DenormalizedKeyFixer;
use description::DescriptionFixer;
use example_data::ExampleDataFixer;
use property_order::PropertyOrderFixer;
use timestamps::TimestampFixer;

/// Registry of auto-fixers.
///
/// The FixEngine maintains a collection of AutoFix implementations and applies
/// the first matching fixer for each validation issue.
pub struct FixEngine {
    fixers: Vec<Box<dyn AutoFix>>,
}

impl FixEngine {
    /// Create a new empty FixEngine.
    pub fn new() -> Self {
        Self { fixers: Vec::new() }
    }

    /// Register a new fixer.
    pub fn register(&mut self, fixer: Box<dyn AutoFix>) {
        self.fixers.push(fixer);
    }

    /// Apply the first matching fixer to the node.
    ///
    /// Iterates through registered fixers and applies the first one that can handle the issue.
    /// Returns `Skipped` if no fixer matches.
    pub fn apply_fix(&self, node: &mut ParsedNode, issue: &SchemaIssue) -> Result<FixAction> {
        for fixer in &self.fixers {
            if fixer.can_fix(issue) {
                return fixer.fix(node, issue);
            }
        }

        Ok(FixAction::Skipped {
            reason: "No fixer available for this issue".to_string(),
        })
    }

    /// Get the number of registered fixers.
    pub fn count(&self) -> usize {
        self.fixers.len()
    }
}

impl Default for FixEngine {
    /// Create a FixEngine with all standard fixers registered.
    fn default() -> Self {
        let mut engine = Self::new();
        engine.register(Box::new(CompositeKeyFixer));
        engine.register(Box::new(DenormalizedKeyFixer));
        engine.register(Box::new(DescriptionFixer));
        engine.register(Box::new(ExampleDataFixer));
        engine.register(Box::new(PropertyOrderFixer));
        engine.register(Box::new(TimestampFixer));
        engine
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests (RED Phase: These tests should FAIL initially)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::schema_rules::IssueSeverity;

    /// Test implementation of AutoFix trait.
    struct TestFixer;

    impl AutoFix for TestFixer {
        fn can_fix(&self, issue: &SchemaIssue) -> bool {
            issue.rule == "TEST_RULE"
        }

        fn fix(&self, _node: &mut ParsedNode, _issue: &SchemaIssue) -> Result<FixAction> {
            Ok(FixAction::Skipped {
                reason: "test".to_string(),
            })
        }

        fn description(&self) -> &str {
            "Test fixer"
        }
    }

    #[test]
    fn test_autofix_trait_compiles() {
        let fixer = TestFixer;
        let issue = SchemaIssue {
            node_name: "TestNode".into(),
            severity: IssueSeverity::Error,
            rule: "TEST_RULE",
            message: "test".into(),
        };

        // Verify trait methods work
        assert!(fixer.can_fix(&issue));
        assert_eq!(fixer.description(), "Test fixer");
    }

    #[test]
    fn test_fix_action_modified() {
        let changes = vec![Change {
            field: "test_field".into(),
            old_value: None,
            new_value: Value::String("new_value".into()),
        }];

        let action = FixAction::Modified {
            changes: changes.clone(),
        };

        match action {
            FixAction::Modified { changes: c } => {
                assert_eq!(c.len(), 1);
                assert_eq!(c[0].field, "test_field");
            },
            _ => panic!("Expected Modified variant"),
        }
    }

    #[test]
    fn test_fix_action_skipped() {
        let action = FixAction::Skipped {
            reason: "not applicable".into(),
        };

        match action {
            FixAction::Skipped { reason } => {
                assert_eq!(reason, "not applicable");
            },
            _ => panic!("Expected Skipped variant"),
        }
    }

    #[test]
    fn test_fix_strategy_default() {
        assert_eq!(FixStrategy::default(), FixStrategy::Safe);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // FixEngine Tests (RED Phase: FixEngine doesn't exist yet)
    // ─────────────────────────────────────────────────────────────────────────

    use crate::parsers::yaml_node::{NodeDef, PropertyDef};
    use indexmap::IndexMap;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Create an EntityNative node without pattern property for testing.
    fn create_entity_native_without_pattern() -> ParsedNode {
        let mut props = IndexMap::new();
        props.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: Some("Test key property".to_string()),
                extra: BTreeMap::new(), // ← No pattern!
            },
        );

        ParsedNode {
            def: NodeDef {
                name: "EntityNative".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                knowledge_tier: None,
                icon: None,
                description: "Test node".to_string(),
                standard_properties: Some(props),
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            source_path: PathBuf::from("/test/entity-native.yaml"),
        }
    }

    #[test]
    fn test_fix_engine_applies_first_matching_fixer() {
        use super::composite_key::CompositeKeyFixer;
        use super::property_order::PropertyOrderFixer;
        use super::timestamps::TimestampFixer;

        let mut engine = FixEngine::new(); // ← This doesn't exist yet (RED)
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

    #[test]
    fn test_fix_engine_default_includes_all_fixers() {
        let engine = FixEngine::default(); // ← This doesn't exist yet (RED)

        // Should have 6 registered fixers
        assert_eq!(engine.count(), 6);
    }

    #[test]
    fn test_fix_engine_skips_when_no_fixer_matches() {
        let engine = FixEngine::new();

        let mut node = create_entity_native_without_pattern();
        let issue = SchemaIssue {
            node_name: "EntityNative".into(),
            severity: IssueSeverity::Warning,
            rule: "UNKNOWN_RULE",
            message: "Unknown rule".into(),
        };

        let result = engine.apply_fix(&mut node, &issue).unwrap();

        match result {
            FixAction::Skipped { reason } => {
                assert!(reason.contains("No fixer available"));
            },
            _ => panic!("Expected Skipped when no fixer matches"),
        }
    }
}
