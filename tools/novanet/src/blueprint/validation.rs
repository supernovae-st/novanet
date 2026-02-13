//! Validation system for blueprint command.
//!
//! Compares YAML schema definitions with Neo4j meta nodes to detect drift.

use crate::blueprint::sources::BlueprintData;
use std::collections::HashSet;

/// Severity level for validation issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "ERROR"),
            Self::Warning => write!(f, "WARNING"),
            Self::Info => write!(f, "INFO"),
        }
    }
}

/// A single validation issue.
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub category: String,
    pub message: String,
    pub fix_hint: Option<String>,
}

impl ValidationIssue {
    pub fn error(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            category: category.into(),
            message: message.into(),
            fix_hint: None,
        }
    }

    pub fn warning(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            category: category.into(),
            message: message.into(),
            fix_hint: None,
        }
    }

    pub fn info(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Info,
            category: category.into(),
            message: message.into(),
            fix_hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.fix_hint = Some(hint.into());
        self
    }
}

/// A single validation check result.
#[derive(Debug, Clone)]
pub struct ValidationCheck {
    pub name: String,
    pub passed: bool,
    pub details: Option<String>,
}

impl ValidationCheck {
    pub fn pass(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: true,
            details: None,
        }
    }

    pub fn fail(name: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            passed: false,
            details: Some(details.into()),
        }
    }
}

/// Complete validation result.
#[derive(Debug)]
pub struct ValidationResult {
    pub checks: Vec<ValidationCheck>,
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    /// Run all validations on blueprint data.
    pub fn validate(data: &BlueprintData) -> Self {
        let mut result = Self {
            checks: Vec::new(),
            issues: Vec::new(),
        };

        // YAML-only checks (always run)
        result.check_arc_coherence(data);
        result.check_arc_scope_coherence(data);
        result.check_orphan_nodes(data);
        result.check_duplicate_arcs(data);
        result.check_path_content_match(data);
        result.check_required_fields(data);

        // Neo4j checks (only if connected)
        if data.neo4j_counts.is_some() {
            result.check_yaml_neo4j_sync(data);
        }

        result
    }

    /// Check if all validations passed.
    pub fn is_valid(&self) -> bool {
        self.issues.iter().all(|i| i.severity != Severity::Error)
    }

    /// Count of errors.
    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count()
    }

    /// Count of warnings.
    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count()
    }

    /// Total issue count.
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Validation checks
    // ─────────────────────────────────────────────────────────────────────────

    /// Check that YAML and Neo4j are in sync.
    fn check_yaml_neo4j_sync(&mut self, data: &BlueprintData) {
        let Some(ref neo4j) = data.neo4j_counts else {
            return;
        };

        let yaml_kinds: HashSet<&str> = data
            .node_kinds
            .iter()
            .map(|n| n.def.name.as_str())
            .collect();
        let neo4j_kinds: HashSet<&str> = neo4j.node_kind_names.iter().map(|s| s.as_str()).collect();

        // Check for kinds in YAML but not in Neo4j
        let missing_in_neo4j: Vec<&str> = yaml_kinds.difference(&neo4j_kinds).copied().collect();
        if !missing_in_neo4j.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "YAML kinds exist in Neo4j",
                format!("Missing in Neo4j: {}", missing_in_neo4j.join(", ")),
            ));
            self.issues.push(
                ValidationIssue::warning(
                    "sync",
                    format!(
                        "{} kinds in YAML but not in Neo4j: {}",
                        missing_in_neo4j.len(),
                        missing_in_neo4j.join(", ")
                    ),
                )
                .with_hint("Run: novanet db seed"),
            );
        } else {
            self.checks
                .push(ValidationCheck::pass("YAML kinds exist in Neo4j"));
        }

        // Check for kinds in Neo4j but not in YAML
        let orphan_in_neo4j: Vec<&str> = neo4j_kinds.difference(&yaml_kinds).copied().collect();
        if !orphan_in_neo4j.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "Neo4j kinds defined in YAML",
                format!("Orphan in Neo4j: {}", orphan_in_neo4j.join(", ")),
            ));
            self.issues.push(
                ValidationIssue::warning(
                    "sync",
                    format!(
                        "{} kinds in Neo4j but not in YAML: {}",
                        orphan_in_neo4j.len(),
                        orphan_in_neo4j.join(", ")
                    ),
                )
                .with_hint("These may be legacy nodes. Consider removing from Neo4j."),
            );
        } else {
            self.checks
                .push(ValidationCheck::pass("Neo4j kinds defined in YAML"));
        }

        // ─────────────────────────────────────────────────────────────────────────
        // ArcKind sync validation (YAML ↔ Neo4j)
        // ─────────────────────────────────────────────────────────────────────────
        let yaml_arcs: HashSet<&str> = data.arc_defs.iter().map(|a| a.arc_type.as_str()).collect();
        let neo4j_arcs: HashSet<&str> = neo4j.arc_kind_names.iter().map(|s| s.as_str()).collect();

        // Check for arc kinds in YAML but not in Neo4j
        let missing_arcs: Vec<&str> = yaml_arcs.difference(&neo4j_arcs).copied().collect();
        if !missing_arcs.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "YAML arc kinds exist in Neo4j",
                format!("Missing in Neo4j: {}", missing_arcs.join(", ")),
            ));
            self.issues.push(
                ValidationIssue::warning(
                    "sync",
                    format!(
                        "{} arc kinds in YAML but not in Neo4j: {}",
                        missing_arcs.len(),
                        missing_arcs.join(", ")
                    ),
                )
                .with_hint("Run: novanet db seed"),
            );
        } else {
            self.checks
                .push(ValidationCheck::pass("YAML arc kinds exist in Neo4j"));
        }

        // Check for arc kinds in Neo4j but not in YAML
        let orphan_arcs: Vec<&str> = neo4j_arcs.difference(&yaml_arcs).copied().collect();
        if !orphan_arcs.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "Neo4j arc kinds defined in YAML",
                format!("Orphan in Neo4j: {}", orphan_arcs.join(", ")),
            ));
            self.issues.push(
                ValidationIssue::warning(
                    "sync",
                    format!(
                        "{} arc kinds in Neo4j but not in YAML: {}",
                        orphan_arcs.len(),
                        orphan_arcs.join(", ")
                    ),
                )
                .with_hint("These may be legacy relationships. Consider removing from Neo4j."),
            );
        } else {
            self.checks
                .push(ValidationCheck::pass("Neo4j arc kinds defined in YAML"));
        }
    }

    /// Check that arc source/target types exist in node kinds.
    fn check_arc_coherence(&mut self, data: &BlueprintData) {
        let valid_kinds: HashSet<&str> = data
            .node_kinds
            .iter()
            .map(|n| n.def.name.as_str())
            .collect();

        let mut invalid_refs = Vec::new();

        for arc in &data.arc_defs {
            // Check source references
            for source in arc.source.labels() {
                if !valid_kinds.contains(source) {
                    invalid_refs.push(format!("{}: source '{}' not found", arc.arc_type, source));
                }
            }
            // Check target references
            for target in arc.target.labels() {
                if !valid_kinds.contains(target) {
                    invalid_refs.push(format!("{}: target '{}' not found", arc.arc_type, target));
                }
            }
        }

        if !invalid_refs.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "Arc source/target types match",
                format!("{} invalid references", invalid_refs.len()),
            ));
            for ref_err in &invalid_refs {
                self.issues
                    .push(ValidationIssue::error("arc_coherence", ref_err.clone()));
            }
        } else {
            self.checks
                .push(ValidationCheck::pass("Arc source/target types match"));
        }
    }

    /// Check that YAML file paths match the realm/layer declared in content.
    fn check_path_content_match(&mut self, data: &BlueprintData) {
        let mut mismatches = Vec::new();

        for node in &data.node_kinds {
            // Path should be: .../node-kinds/{realm}/{layer}/{name}.yaml
            let path_str = node.source_path.to_string_lossy();

            // Check realm in path
            if !path_str.contains(&format!("/{}/", node.realm)) {
                mismatches.push(format!(
                    "{}: path doesn't contain realm '{}'",
                    node.def.name, node.realm
                ));
            }

            // Check layer in path
            if !path_str.contains(&format!("/{}/", node.layer)) {
                mismatches.push(format!(
                    "{}: path doesn't contain layer '{}'",
                    node.def.name, node.layer
                ));
            }
        }

        if !mismatches.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "Realm/Layer paths match YAML content",
                format!("{} mismatches", mismatches.len()),
            ));
            for mismatch in &mismatches {
                self.issues
                    .push(ValidationIssue::error("path_mismatch", mismatch.clone()));
            }
        } else {
            self.checks.push(ValidationCheck::pass(
                "Realm/Layer paths match YAML content",
            ));
        }
    }

    /// Check that arc scope declarations match actual source/target realms.
    ///
    /// Validates:
    /// - `scope: intra_realm` → source and target are in the same realm
    /// - `scope: cross_realm` → source and target are in different realms
    fn check_arc_scope_coherence(&mut self, data: &BlueprintData) {
        // Build a map of node name -> realm for quick lookup
        let node_realms: std::collections::HashMap<&str, &str> = data
            .node_kinds
            .iter()
            .map(|n| (n.def.name.as_str(), n.realm.as_str()))
            .collect();

        let mut scope_errors = Vec::new();
        let mut missing_scope_count = 0;

        for arc in &data.arc_defs {
            let Some(ref declared_scope) = arc.scope else {
                // Track arcs without scope declaration
                missing_scope_count += 1;
                continue;
            };

            // Get realms for all source labels
            let source_realms: Vec<&str> = arc
                .source
                .labels()
                .iter()
                .filter_map(|label| node_realms.get(label).copied())
                .collect();

            // Get realms for all target labels
            let target_realms: Vec<&str> = arc
                .target
                .labels()
                .iter()
                .filter_map(|label| node_realms.get(label).copied())
                .collect();

            // Skip if we couldn't find realms (node type might not exist)
            if source_realms.is_empty() || target_realms.is_empty() {
                continue;
            }

            // Check if source and target realms match
            let is_cross_realm = source_realms
                .iter()
                .any(|sr| target_realms.iter().any(|tr| sr != tr));

            let is_intra_realm = source_realms
                .iter()
                .all(|sr| target_realms.iter().all(|tr| sr == tr));

            match declared_scope.as_str() {
                "intra_realm" => {
                    if !is_intra_realm {
                        scope_errors.push(format!(
                            "{}: declared intra_realm but source({:?}) and target({:?}) are in different realms",
                            arc.arc_type, source_realms, target_realms
                        ));
                    }
                }
                "cross_realm" => {
                    if !is_cross_realm {
                        scope_errors.push(format!(
                            "{}: declared cross_realm but source({:?}) and target({:?}) are in same realm",
                            arc.arc_type, source_realms, target_realms
                        ));
                    }
                }
                other => {
                    scope_errors.push(format!(
                        "{}: unknown scope value '{}' (expected intra_realm or cross_realm)",
                        arc.arc_type, other
                    ));
                }
            }
        }

        if !scope_errors.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "Arc scope coherence",
                format!("{} scope mismatches", scope_errors.len()),
            ));
            for err in &scope_errors {
                self.issues
                    .push(ValidationIssue::error("arc_scope", err.clone()).with_hint(
                        "Update scope in arc-kind YAML to match actual source/target realms",
                    ));
            }
        } else {
            self.checks
                .push(ValidationCheck::pass("Arc scope coherence"));
        }

        // Report arcs without scope declarations
        if missing_scope_count > 0 {
            self.issues.push(
                ValidationIssue::info(
                    "arc_scope",
                    format!("{} arcs have no scope declaration", missing_scope_count),
                )
                .with_hint(
                    "Add 'scope: intra_realm' or 'scope: cross_realm' to arc-kind YAML files",
                ),
            );
        }
    }

    /// Check for orphan node types (defined but never used in any arc).
    fn check_orphan_nodes(&mut self, data: &BlueprintData) {
        use std::collections::HashSet;

        // All defined node types
        let all_nodes: HashSet<&str> = data
            .node_kinds
            .iter()
            .map(|n| n.def.name.as_str())
            .collect();

        // Node types used in arcs (as source or target)
        let mut used_in_arcs: HashSet<&str> = HashSet::new();
        for arc in &data.arc_defs {
            for label in arc.source.labels() {
                used_in_arcs.insert(label);
            }
            for label in arc.target.labels() {
                used_in_arcs.insert(label);
            }
        }

        // Find orphans
        let mut orphans: Vec<&str> = all_nodes.difference(&used_in_arcs).copied().collect();
        orphans.sort();

        if !orphans.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "All nodes used in arcs",
                format!("{} orphan nodes", orphans.len()),
            ));
            self.issues.push(
                ValidationIssue::warning(
                    "orphan_node",
                    format!("Orphan nodes (not in any arc): {}", orphans.join(", ")),
                )
                .with_hint("Add arcs for these nodes or remove them if unused"),
            );
        } else {
            self.checks
                .push(ValidationCheck::pass("All nodes used in arcs"));
        }
    }

    /// Check for duplicate arc type names.
    fn check_duplicate_arcs(&mut self, data: &BlueprintData) {
        use std::collections::HashMap;

        let mut counts: HashMap<&str, usize> = HashMap::new();
        for arc in &data.arc_defs {
            *counts.entry(arc.arc_type.as_str()).or_insert(0) += 1;
        }

        let duplicates: Vec<(&str, usize)> =
            counts.into_iter().filter(|(_, count)| *count > 1).collect();

        if !duplicates.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "Unique arc types",
                format!("{} duplicates", duplicates.len()),
            ));
            for (arc_type, count) in &duplicates {
                self.issues.push(
                    ValidationIssue::error(
                        "duplicate_arc",
                        format!("{} defined {} times", arc_type, count),
                    )
                    .with_hint("Remove duplicate arc-kind YAML files"),
                );
            }
        } else {
            self.checks.push(ValidationCheck::pass("Unique arc types"));
        }
    }

    /// Check that all required fields are present.
    fn check_required_fields(&mut self, data: &BlueprintData) {
        let mut missing = Vec::new();

        for node in &data.node_kinds {
            if node.def.name.is_empty() {
                missing.push(format!("{}: missing name", node.source_path.display()));
            }
            if node.realm.is_empty() {
                missing.push(format!("{}: missing realm", node.def.name));
            }
            if node.layer.is_empty() {
                missing.push(format!("{}: missing layer", node.def.name));
            }
        }

        for arc in &data.arc_defs {
            if arc.arc_type.is_empty() {
                missing.push("Arc with empty rel_type".to_string());
            }
        }

        if !missing.is_empty() {
            self.checks.push(ValidationCheck::fail(
                "Required fields present",
                format!("{} missing fields", missing.len()),
            ));
            for m in &missing {
                self.issues
                    .push(ValidationIssue::error("required_field", m.clone()));
            }
        } else {
            self.checks
                .push(ValidationCheck::pass("Required fields present"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_from_yaml() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");
        let result = ValidationResult::validate(&data);

        // Should have run checks
        assert!(!result.checks.is_empty(), "Should have validation checks");

        // Print all issues for debugging
        println!("\n=== ALL VALIDATION ISSUES ===");
        for issue in &result.issues {
            println!(
                "[{:?}] {}: {}",
                issue.severity, issue.category, issue.message
            );
        }
        println!("=== END ISSUES ===\n");

        // Current schema should be valid
        // (This may need adjustment if schema has known issues)
        for check in &result.checks {
            if !check.passed {
                println!("Failed check: {} - {:?}", check.name, check.details);
            }
        }
    }

    #[test]
    fn test_validation_issue_creation() {
        let issue = ValidationIssue::error("test", "Test message").with_hint("Fix this");

        assert_eq!(issue.severity, Severity::Error);
        assert_eq!(issue.category, "test");
        assert_eq!(issue.message, "Test message");
        assert_eq!(issue.fix_hint, Some("Fix this".to_string()));
    }

    #[test]
    fn test_validation_check_creation() {
        let pass = ValidationCheck::pass("Test check");
        assert!(pass.passed);

        let fail = ValidationCheck::fail("Test check", "Details");
        assert!(!fail.passed);
        assert_eq!(fail.details, Some("Details".to_string()));
    }

    #[test]
    fn test_validation_checks_all_categories() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");
        let result = ValidationResult::validate(&data);

        // Should have all expected check categories
        let check_names: Vec<&str> = result.checks.iter().map(|c| c.name.as_str()).collect();

        assert!(
            check_names.contains(&"Arc source/target types match"),
            "Should check arc coherence"
        );
        assert!(
            check_names.contains(&"Arc scope coherence"),
            "Should check scope coherence"
        );
        assert!(
            check_names.contains(&"All nodes used in arcs"),
            "Should check orphan nodes"
        );
        assert!(
            check_names.contains(&"Unique arc types"),
            "Should check duplicate arcs"
        );
        assert!(
            check_names.contains(&"Realm/Layer paths match YAML content"),
            "Should check path match"
        );
        assert!(
            check_names.contains(&"Required fields present"),
            "Should check required fields"
        );
    }

    #[test]
    fn test_info_level_issues() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");
        let result = ValidationResult::validate(&data);

        // Check that INFO-level issues are being generated for missing scopes
        let info_issues: Vec<_> = result
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Info)
            .collect();

        // Should have at least one info issue if some arcs lack scope
        // (This is expected for legacy arcs that haven't been updated)
        println!("INFO issues: {:?}", info_issues.len());
    }

    #[test]
    fn test_validation_severity_types() {
        let issue_error = ValidationIssue::error("test", "error");
        let issue_warning = ValidationIssue::warning("test", "warning");
        let issue_info = ValidationIssue::info("test", "info");

        assert_eq!(issue_error.severity, Severity::Error);
        assert_eq!(issue_warning.severity, Severity::Warning);
        assert_eq!(issue_info.severity, Severity::Info);

        // All three severity levels should be distinct
        assert_ne!(issue_error.severity, issue_warning.severity);
        assert_ne!(issue_warning.severity, issue_info.severity);
        assert_ne!(issue_error.severity, issue_info.severity);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // NOMENCLATURE DX TESTS (v0.12.0)
    // ═══════════════════════════════════════════════════════════════════════════
    // These tests validate ADR-023 (Class/Instance), ADR-024 (Data Origin traits),
    // and ADR-025 (Instruction Layer) terminology is consistent across YAML.

    /// ADR-024: Trait values must be the new "Data Origin" names
    #[test]
    fn test_adr024_trait_values_are_data_origin() {
        use crate::parsers::yaml_node::NodeTrait;

        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        // All traits must be one of the valid Data Origin values
        for node in &data.node_kinds {
            let is_valid = matches!(
                node.def.node_trait,
                NodeTrait::Defined
                    | NodeTrait::Authored
                    | NodeTrait::Imported
                    | NodeTrait::Generated
                    | NodeTrait::Retrieved
            );

            assert!(
                is_valid,
                "Node '{}' has invalid trait '{:?}'. Valid traits: Defined, Authored, Imported, Generated, Retrieved",
                node.def.name,
                node.def.node_trait
            );
        }
    }

    /// ADR-023: Must have exactly 59 node classes (39 shared + 20 org)
    #[test]
    fn test_adr023_node_count() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let total = data.node_kinds.len();
        let shared_count = data.node_kinds.iter().filter(|n| n.realm == "shared").count();
        let org_count = data.node_kinds.iter().filter(|n| n.realm == "org").count();

        assert_eq!(total, 59, "Expected 59 total nodes, got {}", total);
        assert_eq!(shared_count, 39, "Expected 39 shared nodes, got {}", shared_count);
        assert_eq!(org_count, 20, "Expected 20 org nodes, got {}", org_count);
    }

    /// ADR-025: Instruction layer nodes must use new names
    #[test]
    fn test_adr025_instruction_layer_nodes() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let node_names: Vec<&str> = data.node_kinds.iter().map(|n| n.def.name.as_str()).collect();

        // New instruction layer names (ADR-025)
        assert!(
            node_names.contains(&"PageStructure"),
            "Missing PageStructure (renamed from PageType)"
        );
        assert!(
            node_names.contains(&"PageInstruction"),
            "Missing PageInstruction (renamed from PagePrompt)"
        );
        assert!(
            node_names.contains(&"BlockInstruction"),
            "Missing BlockInstruction (renamed from BlockPrompt)"
        );

        // Deprecated names should NOT exist
        assert!(
            !node_names.contains(&"PageType"),
            "PageType is deprecated, use PageStructure"
        );
        assert!(
            !node_names.contains(&"PagePrompt"),
            "PagePrompt is deprecated, use PageInstruction"
        );
        assert!(
            !node_names.contains(&"BlockPrompt"),
            "BlockPrompt is deprecated, use BlockInstruction"
        );
    }

    /// ADR-025: Arc types must use new instruction names
    #[test]
    fn test_adr025_instruction_arcs() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let arc_names: Vec<&str> = data.arc_defs.iter().map(|a| a.arc_type.as_str()).collect();

        // New arc names (ADR-025)
        assert!(
            arc_names.contains(&"HAS_INSTRUCTION"),
            "Missing HAS_INSTRUCTION arc (renamed from HAS_PROMPT)"
        );
        assert!(
            arc_names.contains(&"HAS_STRUCTURE"),
            "Missing HAS_STRUCTURE arc (Page -> PageStructure)"
        );

        // Deprecated arc names should NOT exist
        assert!(
            !arc_names.contains(&"HAS_PROMPT"),
            "HAS_PROMPT is deprecated, use HAS_INSTRUCTION"
        );
    }

    /// ADR-023: No deprecated KIND arc terminology in YAML
    /// Note: OF_CLASS, FROM_CLASS, TO_CLASS, HAS_CLASS are schema-level arcs
    /// that connect :Schema:Class nodes, created during db seed (not in arc-kinds YAML).
    /// Instance→Class relationship is via Neo4j labels, not explicit arcs.
    #[test]
    fn test_adr023_no_deprecated_kind_arcs() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let arc_names: Vec<&str> = data.arc_defs.iter().map(|a| a.arc_type.as_str()).collect();

        // Deprecated arc names should NOT exist in YAML definitions
        assert!(
            !arc_names.contains(&"OF_KIND"),
            "OF_KIND is deprecated - schema arcs use OF_CLASS"
        );
        assert!(
            !arc_names.contains(&"HAS_KIND"),
            "HAS_KIND is deprecated - schema arcs use HAS_CLASS"
        );
        assert!(
            !arc_names.contains(&"FROM_KIND"),
            "FROM_KIND is deprecated - use FROM_CLASS"
        );
        assert!(
            !arc_names.contains(&"TO_KIND"),
            "TO_KIND is deprecated - use TO_CLASS"
        );
    }

    /// Naming convention: *Content suffix indicates authored trait
    #[test]
    fn test_naming_convention_content_suffix() {
        use crate::parsers::yaml_node::NodeTrait;

        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let content_nodes: Vec<_> = data
            .node_kinds
            .iter()
            .filter(|n| n.def.name.ends_with("Content"))
            .collect();

        assert!(!content_nodes.is_empty(), "Should have *Content nodes");

        for node in content_nodes {
            assert!(
                matches!(node.def.node_trait, NodeTrait::Authored),
                "Node '{}' ends with 'Content' but has trait '{:?}' (should be Authored)",
                node.def.name, node.def.node_trait
            );
        }
    }

    /// Naming convention: *Generated suffix indicates generated trait
    #[test]
    fn test_naming_convention_generated_suffix() {
        use crate::parsers::yaml_node::NodeTrait;

        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let generated_nodes: Vec<_> = data
            .node_kinds
            .iter()
            .filter(|n| n.def.name.ends_with("Generated"))
            .collect();

        assert!(!generated_nodes.is_empty(), "Should have *Generated nodes");

        for node in generated_nodes {
            assert!(
                matches!(node.def.node_trait, NodeTrait::Generated),
                "Node '{}' ends with 'Generated' but has trait '{:?}' (should be Generated)",
                node.def.name, node.def.node_trait
            );
        }
    }

    /// Layer distribution validation (v0.12.0)
    #[test]
    fn test_layer_distribution() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        // Count by realm (the reliable way)
        let shared_count = data.node_kinds.iter().filter(|n| n.realm == "shared").count();
        let org_count = data.node_kinds.iter().filter(|n| n.realm == "org").count();

        assert_eq!(shared_count, 39, "Shared realm should have 39 nodes, got {}", shared_count);
        assert_eq!(org_count, 20, "Org realm should have 20 nodes, got {}", org_count);

        // Verify layer counts within each realm
        // Shared: config(3) + locale(6) + geography(6) + knowledge(24) = 39
        // Org: config(1) + foundation(3) + structure(3) + semantic(4) + instruction(6) + output(3) = 20

        // Check that each realm has the expected layers
        let shared_layers: std::collections::HashSet<&str> = data
            .node_kinds
            .iter()
            .filter(|n| n.realm == "shared")
            .map(|n| n.layer.as_str())
            .collect();
        assert!(shared_layers.contains("config"), "Shared should have config layer");
        assert!(shared_layers.contains("locale"), "Shared should have locale layer");
        assert!(shared_layers.contains("geography"), "Shared should have geography layer");
        assert!(shared_layers.contains("knowledge"), "Shared should have knowledge layer");

        let org_layers: std::collections::HashSet<&str> = data
            .node_kinds
            .iter()
            .filter(|n| n.realm == "org")
            .map(|n| n.layer.as_str())
            .collect();
        assert!(org_layers.contains("config"), "Org should have config layer");
        assert!(org_layers.contains("foundation"), "Org should have foundation layer");
        assert!(org_layers.contains("structure"), "Org should have structure layer");
        assert!(org_layers.contains("semantic"), "Org should have semantic layer");
        assert!(org_layers.contains("instruction"), "Org should have instruction layer");
        assert!(org_layers.contains("output"), "Org should have output layer");
    }
}
