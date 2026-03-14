//! `novanet schema generate` and `novanet schema validate` commands.
//!
//! - generate: Orchestrates all 12 generators in order
//! - validate: YAML-only validation (no Neo4j in Phase 2)

use crate::generators::Generator;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tracing::{debug, instrument};

// ─────────────────────────────────────────────────────────────────────────────
// Output mappings
// ─────────────────────────────────────────────────────────────────────────────

/// (generator, relative output path from monorepo root)
struct GeneratorEntry {
    generator: Box<dyn Generator>,
    output_path: &'static str,
    /// Optional post-processor (e.g., Mermaid wraps in Markdown)
    post_process: Option<fn(&str) -> String>,
}

fn all_generators() -> Vec<GeneratorEntry> {
    use crate::generators::{
        arc_class::ArcClassGenerator, autowire::AutowireGenerator, colors::ColorsGenerator,
        hierarchy::HierarchyGenerator, icons::IconsGenerator, layer::LayerGenerator,
        mermaid::MermaidGenerator, node_class::NodeClassGenerator, organizing::OrganizingGenerator,
        tui_colors::TuiColorsGenerator, tui_icons::TuiIconsGenerator,
        visual_encoding::VisualEncodingGenerator,
    };

    vec![
        GeneratorEntry {
            generator: Box::new(OrganizingGenerator),
            output_path: "packages/db/seed/00.5-taxonomy.cypher",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(NodeClassGenerator),
            output_path: "packages/db/seed/01-classes.cypher",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(ArcClassGenerator),
            output_path: "packages/db/seed/02-arc-classes.cypher",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(LayerGenerator),
            output_path: "packages/core/src/graph/layers.ts",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(MermaidGenerator),
            output_path: "packages/core/models/docs/complete-graph.md",
            post_process: Some(crate::generators::mermaid::wrap_in_markdown),
        },
        GeneratorEntry {
            generator: Box::new(AutowireGenerator),
            output_path: "packages/db/seed/99-autowire-classes.cypher",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(HierarchyGenerator),
            output_path: "packages/core/src/graph/hierarchy.ts",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(ColorsGenerator),
            output_path: "apps/studio/src/design/colors/generated.ts",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(IconsGenerator),
            output_path: "apps/studio/src/design/icons/nodeIcons.generated.ts",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(VisualEncodingGenerator),
            output_path: "packages/core/src/graph/visual-encoding.ts",
            post_process: None,
        },
        // ViewsGenerator removed — views.yaml is now loaded dynamically
        // by ViewLoader.ts (Studio) and nexus/views.rs (TUI)
        GeneratorEntry {
            generator: Box::new(TuiIconsGenerator),
            output_path: "tools/novanet/src/tui/icons.rs",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(TuiColorsGenerator),
            output_path: "tools/novanet/src/tui/colors.generated.rs",
            post_process: None,
        },
    ]
}

// ─────────────────────────────────────────────────────────────────────────────
// Schema Generate
// ─────────────────────────────────────────────────────────────────────────────

/// Result of a single generator run.
pub struct GenerateResult {
    pub name: String,
    pub output_path: String,
    pub bytes: usize,
    pub duration_ms: u128,
}

/// Run all 12 generators and optionally write output files.
///
/// Generator execution order: Organizing → Class → ArcClass → Layer → Mermaid → Autowire → Hierarchy → Colors → Icons → VisualEncoding → TuiIcons → TuiColors
#[instrument(skip_all, fields(root = %root.display(), dry_run))]
pub fn schema_generate(root: &Path, dry_run: bool) -> crate::Result<Vec<GenerateResult>> {
    let entries = all_generators();
    let mut results = Vec::with_capacity(entries.len());

    for entry in entries {
        let start = Instant::now();
        let mut output = entry.generator.generate(root)?;
        let duration_ms = start.elapsed().as_millis();

        if let Some(post) = entry.post_process {
            output = post(&output);
        }

        let bytes = output.len();

        if !dry_run {
            let full_path = root.join(entry.output_path);
            // Ensure parent directory exists
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).map_err(crate::NovaNetError::Io)?;
            }
            fs::write(&full_path, &output).map_err(crate::NovaNetError::Io)?;
        }

        results.push(GenerateResult {
            name: entry.generator.name().to_string(),
            output_path: entry.output_path.to_string(),
            bytes,
            duration_ms,
        });
    }

    Ok(results)
}

// ─────────────────────────────────────────────────────────────────────────────
// Schema Validate
// ─────────────────────────────────────────────────────────────────────────────

/// Validation issue with severity.
#[derive(Debug)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub message: String,
}

/// Severity level for schema validation issues.
#[derive(Debug, PartialEq, Eq)]
pub enum Severity {
    /// Critical issue that must be fixed before proceeding.
    Error,
    /// Non-critical issue that should be addressed.
    Warning,
}

/// Validate YAML model coherence (Phase 2: YAML-only, no Neo4j).
///
/// Checks:
/// - All 42 node YAMLs parse with trait
/// - relations.yaml parses with family on every relation
/// - taxonomy parses (realms, layers, traits, arc_families from individual files)
/// - Every node's realm/layer exists in taxonomy
/// - Every relation's source/target labels match known node names
#[instrument(skip_all, fields(root = %root.display()))]
pub fn schema_validate(root: &Path) -> crate::Result<Vec<ValidationIssue>> {
    let mut issues = Vec::new();

    // 1. Parse nodes
    let nodes = crate::parsers::yaml_node::load_all_nodes(root)?;
    let node_names: std::collections::HashSet<String> =
        nodes.iter().map(|n| n.def.name.clone()).collect();

    // 2. Parse arc definitions (from arc-classes/ directory)
    let rels_doc = crate::parsers::arcs::load_arc_classes_from_files(root)?;

    // 3. Parse organizing principles
    let org_doc = crate::parsers::organizing::load_organizing(root)?;

    // 4. Build known realms and layers
    let known_realms: std::collections::HashSet<&str> =
        org_doc.realms.iter().map(|r| r.key.as_str()).collect();
    let known_layers: std::collections::HashSet<&str> = org_doc
        .realms
        .iter()
        .flat_map(|r| r.layers.iter().map(|l| l.key.as_str()))
        .collect();

    // 5. Validate each node's realm/layer is defined in taxonomy
    for node in &nodes {
        if !known_realms.contains(node.realm.as_str()) {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                message: format!(
                    "{}: realm '{}' not defined in taxonomy",
                    node.def.name, node.realm
                ),
            });
        }
        if !known_layers.contains(node.layer.as_str()) {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                message: format!(
                    "{}: layer '{}' not defined in taxonomy",
                    node.def.name, node.layer
                ),
            });
        }
    }

    // 6. Validate arc source/target labels match known node names
    // Exception: Schema family arcs reference meta-types (Class, ArcClass) that are not runtime node types
    use crate::parsers::arcs::ArcFamily;
    const SCHEMA_META_TYPES: &[&str] = &["Class", "ArcClass"];
    for arc in &rels_doc.arcs {
        let is_schema_arc = arc.family == ArcFamily::Schema;
        for label in arc.source.labels() {
            // Skip validation for schema arcs referencing meta-types
            if is_schema_arc && SCHEMA_META_TYPES.contains(&label) {
                continue;
            }
            if label != "*" && !node_names.contains(label) {
                issues.push(ValidationIssue {
                    severity: Severity::Warning,
                    message: format!(
                        "arc {}: source '{}' is not a known node type",
                        arc.arc_type, label
                    ),
                });
            }
        }
        for label in arc.target.labels() {
            // Skip validation for schema arcs referencing meta-types
            if is_schema_arc && SCHEMA_META_TYPES.contains(&label) {
                continue;
            }
            if label != "*" && !node_names.contains(label) {
                issues.push(ValidationIssue {
                    severity: Severity::Warning,
                    message: format!(
                        "arc {}: target '{}' is not a known node type",
                        arc.arc_type, label
                    ),
                });
            }
        }
    }

    // 7. Validate schema standardization rules
    let schema_issues = crate::parsers::schema_rules::validate_all_nodes(&nodes);
    for issue in schema_issues {
        let severity = match issue.severity {
            crate::parsers::schema_rules::IssueSeverity::Error => Severity::Error,
            crate::parsers::schema_rules::IssueSeverity::Warning => Severity::Warning,
        };
        issues.push(ValidationIssue {
            severity,
            message: format!("[{}] {}: {}", issue.rule, issue.node_name, issue.message),
        });
    }

    Ok(issues)
}

/// Convert a PropertyDef to a serde_yaml::Value::Mapping, preserving field order.
fn property_def_to_yaml(prop: &crate::parsers::yaml_node::PropertyDef) -> serde_yaml::Value {
    use serde_yaml::{Mapping, Value};

    let mut prop_map = Mapping::new();

    // type is always first
    prop_map.insert(
        Value::String("type".to_string()),
        Value::String(prop.prop_type.clone()),
    );

    // required
    if let Some(required) = prop.required {
        prop_map.insert(Value::String("required".to_string()), Value::Bool(required));
    }

    // description
    if let Some(ref desc) = prop.description {
        prop_map.insert(
            Value::String("description".to_string()),
            Value::String(desc.clone()),
        );
    }

    // extra fields (pattern, examples, etc.) - preserve their order
    for (key, val) in &prop.extra {
        if let Ok(yaml_val) = serde_yaml::to_value(val) {
            prop_map.insert(Value::String(key.clone()), yaml_val);
        }
    }

    Value::Mapping(prop_map)
}

/// Convert an IndexMap of properties to a serde_yaml::Mapping, preserving key order.
///
/// This is necessary because `serde_yaml::to_value()` on IndexMap may not preserve
/// insertion order when the resulting Value is serialized back to YAML.
fn properties_to_yaml_ordered(
    props: &indexmap::IndexMap<String, crate::parsers::yaml_node::PropertyDef>,
) -> serde_yaml::Value {
    use serde_yaml::{Mapping, Value};

    let mut props_map = Mapping::new();

    // Iterate in IndexMap order (which is insertion order, i.e., the fixed order from PropertyOrderFixer)
    for (key, prop_def) in props {
        props_map.insert(Value::String(key.clone()), property_def_to_yaml(prop_def));
    }

    Value::Mapping(props_map)
}

/// Convert a NodeDef back to YAML structure for writing.
///
/// Manually builds a serde_yaml::Value that matches the expected YAML structure.
fn node_def_to_yaml(node: &crate::parsers::yaml_node::ParsedNode) -> serde_yaml::Value {
    use serde_yaml::{Mapping, Value};

    let mut node_map = Mapping::new();

    // Basic fields
    node_map.insert(
        Value::String("name".to_string()),
        Value::String(node.def.name.clone()),
    );
    node_map.insert(
        Value::String("realm".to_string()),
        Value::String(node.def.realm.clone()),
    );
    node_map.insert(
        Value::String("layer".to_string()),
        Value::String(node.def.layer.clone()),
    );
    // v0.17.3 (ADR-036): trait removed from schema-level, provenance is per-instance

    // Optional knowledge_tier
    if let Some(tier) = node.def.knowledge_tier {
        node_map.insert(
            Value::String("knowledge_tier".to_string()),
            Value::String(tier.to_string()),
        );
    }

    // Description
    node_map.insert(
        Value::String("description".to_string()),
        Value::String(node.def.description.clone()),
    );

    // Icon
    if let Some(ref icon) = node.def.icon {
        let mut icon_map = Mapping::new();
        icon_map.insert(
            Value::String("web".to_string()),
            Value::String(icon.web.clone()),
        );
        icon_map.insert(
            Value::String("terminal".to_string()),
            Value::String(icon.terminal.clone()),
        );
        node_map.insert(Value::String("icon".to_string()), Value::Mapping(icon_map));
    }

    // standard_properties - use order-preserving conversion
    if let Some(ref props) = node.def.standard_properties {
        let props_value = properties_to_yaml_ordered(props);
        node_map.insert(
            Value::String("standard_properties".to_string()),
            props_value,
        );
    }

    // properties - use order-preserving conversion
    if let Some(ref props) = node.def.properties {
        let props_value = properties_to_yaml_ordered(props);
        node_map.insert(Value::String("properties".to_string()), props_value);
    }

    // neo4j
    if let Some(ref neo4j) = node.def.neo4j {
        let neo4j_value = serde_yaml::to_value(neo4j).unwrap_or(Value::Null);
        node_map.insert(Value::String("neo4j".to_string()), neo4j_value);
    }

    // example
    if let Some(ref example) = node.def.example {
        let example_value = serde_yaml::to_value(example).unwrap_or(Value::Null);
        node_map.insert(Value::String("example".to_string()), example_value);
    }

    // Wrap in { node: {...} }
    let mut root_map = Mapping::new();
    root_map.insert(Value::String("node".to_string()), Value::Mapping(node_map));

    Value::Mapping(root_map)
}

/// Validate schema YAML files with auto-fix capability.
///
/// Runs validation and optionally applies auto-fixes for violations.
/// Returns validation issues with fix status annotations.
#[instrument(skip_all)]
pub fn schema_validate_with_fix(
    root: &Path,
    strategy: crate::validation::FixStrategy,
) -> crate::Result<Vec<ValidationIssue>> {
    use crate::validation::{FixAction, FixEngine};

    // Load all nodes
    let mut nodes = crate::parsers::yaml_node::load_all_nodes(root)?;

    // Create fix engine with all registered fixers
    let engine = FixEngine::default();

    debug!(
        "Loaded {} nodes, fix engine has {} fixers",
        nodes.len(),
        engine.count()
    );

    // Validate all nodes and collect issues
    let schema_issues = crate::parsers::schema_rules::validate_all_nodes(&nodes);

    let mut result_issues = Vec::new();
    let mut modified_node_indices = Vec::new();

    // Group issues by node for efficient processing
    use std::collections::HashMap;
    let mut issues_by_node: HashMap<String, Vec<crate::parsers::schema_rules::SchemaIssue>> =
        HashMap::new();
    for issue in schema_issues {
        issues_by_node
            .entry(issue.node_name.clone())
            .or_default()
            .push(issue);
    }

    // Process each node's issues
    for (idx, node) in nodes.iter_mut().enumerate() {
        if let Some(node_issues) = issues_by_node.get(&node.def.name) {
            let mut node_modified = false;

            for issue in node_issues {
                // Try to apply fix
                let fix_result = engine.apply_fix(node, issue)?;

                let severity = match issue.severity {
                    crate::parsers::schema_rules::IssueSeverity::Error => Severity::Error,
                    crate::parsers::schema_rules::IssueSeverity::Warning => Severity::Warning,
                };

                match fix_result {
                    FixAction::Modified { changes } => {
                        node_modified = true;
                        let change_desc = changes
                            .iter()
                            .map(|c| c.field.clone())
                            .collect::<Vec<_>>()
                            .join(", ");

                        let status = match strategy {
                            crate::validation::FixStrategy::DryRun => "(Would fix)",
                            crate::validation::FixStrategy::Safe
                            | crate::validation::FixStrategy::Auto => "(FIXED)",
                        };

                        result_issues.push(ValidationIssue {
                            severity,
                            message: format!(
                                "[{}] {}: {} {} [{}]",
                                issue.rule, issue.node_name, issue.message, status, change_desc
                            ),
                        });
                    },
                    FixAction::Skipped { reason } => {
                        result_issues.push(ValidationIssue {
                            severity,
                            message: format!(
                                "[{}] {}: {} (Can't auto-fix: {})",
                                issue.rule, issue.node_name, issue.message, reason
                            ),
                        });
                    },
                }
            }

            // Mark node index for writing if modified and not DryRun
            if node_modified && strategy != crate::validation::FixStrategy::DryRun {
                modified_node_indices.push(idx);
            }
        }
    }

    // Write fixed nodes back to YAML
    if !modified_node_indices.is_empty() {
        debug!(
            "Writing {} fixed nodes to YAML",
            modified_node_indices.len()
        );

        for idx in modified_node_indices {
            let node = &nodes[idx];

            // Reconstruct YAML content using helper function
            let yaml_value = node_def_to_yaml(node);

            let yaml_content = serde_yaml::to_string(&yaml_value)
                .map_err(|e| crate::NovaNetError::Validation(e.to_string()))?;

            // Write to original file path
            std::fs::write(&node.source_path, yaml_content).map_err(|e| {
                crate::NovaNetError::Validation(format!(
                    "Failed to write fixed YAML to {:?}: {}",
                    node.source_path, e
                ))
            })?;

            debug!("Wrote fixed YAML to {:?}", node.source_path);
        }
    }

    Ok(result_issues)
}

// ─────────────────────────────────────────────────────────────────────────────
// Schema Stats
// ─────────────────────────────────────────────────────────────────────────────

use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct SchemaStats {
    nodes: NodeStats,
    arcs: ArcStats,
}

#[derive(Debug, Serialize)]
struct NodeStats {
    total: usize,
    by_realm: HashMap<String, usize>,
    by_layer: HashMap<String, usize>,
    // v0.17.3 (ADR-036): by_trait removed, provenance is per-instance
}

#[derive(Debug, Serialize)]
struct ArcStats {
    total: usize,
    by_family: HashMap<String, usize>,
}

/// Extract schema statistics (node/arc counts) from YAML files.
///
/// Outputs counts in JSON or table format for documentation synchronization.
#[instrument(skip_all)]
pub fn schema_stats(root: &Path, format: crate::output::OutputFormat) -> crate::Result<()> {
    // Load all nodes
    let nodes = crate::parsers::yaml_node::load_all_nodes(root)?;

    // Count nodes by realm, layer, trait
    let mut by_realm: HashMap<String, usize> = HashMap::new();
    let mut by_layer: HashMap<String, usize> = HashMap::new();
    // v0.17.3 (ADR-036): by_trait removed, provenance is per-instance

    for node in &nodes {
        *by_realm.entry(node.def.realm.clone()).or_insert(0) += 1;
        *by_layer.entry(node.def.layer.clone()).or_insert(0) += 1;
    }

    // Load all arcs
    let arcs_doc = crate::parsers::arcs::load_arc_classes_from_files(root)?;

    // Count arcs by family
    let mut by_family: HashMap<String, usize> = HashMap::new();
    for arc in &arcs_doc.arcs {
        *by_family.entry(arc.family.to_string()).or_insert(0) += 1;
    }

    let stats = SchemaStats {
        nodes: NodeStats {
            total: nodes.len(),
            by_realm,
            by_layer,
        },
        arcs: ArcStats {
            total: arcs_doc.arcs.len(),
            by_family,
        },
    };

    match format {
        crate::output::OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&stats)
                    .map_err(|e| crate::NovaNetError::Validation(e.to_string()))?
            );
        },
        crate::output::OutputFormat::Table => {
            // Table output for human readability
            println!("\nNODE STATISTICS");
            println!("─────────────────────────────────────────────────────");
            println!("Total: {} nodes", stats.nodes.total);
            println!("\nBy Realm:");
            let mut realms: Vec<_> = stats.nodes.by_realm.iter().collect();
            realms.sort_by_key(|(name, _)| *name);
            for (realm, count) in realms {
                println!("  {:<15} {}", realm, count);
            }
            println!("\nBy Layer:");
            let mut layers: Vec<_> = stats.nodes.by_layer.iter().collect();
            layers.sort_by_key(|(name, _)| *name);
            for (layer, count) in layers {
                println!("  {:<15} {}", layer, count);
            }
            // v0.17.3 (ADR-036): By Trait section removed, provenance is per-instance

            println!("\nARC STATISTICS");
            println!("─────────────────────────────────────────────────────");
            println!("Total: {} arcs", stats.arcs.total);
            println!("\nBy Family:");
            let mut families: Vec<_> = stats.arcs.by_family.iter().collect();
            families.sort_by(|(_, a), (_, b)| b.cmp(a)); // Sort by count descending
            for (family, count) in families {
                println!("  {:<15} {}", family, count);
            }
        },
        _ => {
            return Err(crate::NovaNetError::Validation(
                "Unsupported format for stats (use json or table)".into(),
            ));
        },
    }

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

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
    #[serial]
    fn schema_generate_dry_run_integration() {
        let Some(root) = test_root() else { return };

        let results = schema_generate(&root, true).expect("should generate all artifacts");

        // All 12 generators should succeed (views removed)
        assert_eq!(results.len(), 12, "expected 12 generator results");

        // Verify generator names and order
        let names: Vec<&str> = results.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "taxonomy",
                "classes",
                "arc_schema",
                "layers",
                "mermaid",
                "autowire",
                "hierarchy",
                "colors",
                "icons",
                "visual_encoding",
                "tui_icons",
                "tui_colors",
            ]
        );

        // Each generator should produce non-trivial output
        for result in &results {
            assert!(
                result.bytes > 100,
                "{}: expected >100 bytes, got {}",
                result.name,
                result.bytes
            );
        }

        // Spot check: mermaid output should be wrapped in markdown
        let mermaid = &results[4];
        assert_eq!(mermaid.name, "mermaid");
        assert!(mermaid.output_path.ends_with(".md"));
    }

    #[test]
    #[serial]
    fn schema_validate_integration() {
        // Acquire lock to serialize with other tests that create files in schema directory
        let _lock = SCHEMA_TEST_LOCK.lock().unwrap();

        let Some(root) = test_root() else { return };

        // Clean up any leftover test files from parallel test execution
        cleanup_test_files(&root);

        let issues = schema_validate(&root).expect("should validate schema");

        // Check for any hard errors (there should be none in a clean repo)
        let errors: Vec<&ValidationIssue> = issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .collect();

        assert!(
            errors.is_empty(),
            "unexpected validation errors: {:?}",
            errors.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Cycle 3.2: CLI Integration (RED Phase)
    // ─────────────────────────────────────────────────────────────────────────

    // Static mutex to serialize tests that create files in schema directory
    use std::sync::Mutex;
    static SCHEMA_TEST_LOCK: Mutex<()> = Mutex::new(());

    /// Helper to clean up any leftover test files from previous test runs.
    /// Scans the entire node-classes directory tree, not just one subdirectory.
    fn cleanup_test_files(root: &std::path::Path) {
        use walkdir::WalkDir;

        let node_classes_dir = crate::config::node_classes_dir(root);
        if !node_classes_dir.exists() {
            return;
        }

        // Remove all temporary test files (starting with "test-", "_tmp-", "__test__", or containing "-test")
        for entry in WalkDir::new(node_classes_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(name) = entry.path().file_name().and_then(|n| n.to_str()) {
                if name.starts_with("test-")
                    || name.starts_with("_tmp-")
                    || name.starts_with("__test__")
                    || name.contains("-test")
                {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }
    }

    #[test]
    fn test_schema_validate_with_fix_dry_run() {
        use crate::parsers::schema_rules::validate_node;
        // v0.17.3 (ADR-036): NodeTrait removed, provenance is per-instance
        use crate::parsers::yaml_node::{NodeDef, ParsedNode, PropertyDef};
        use crate::validation::FixEngine;
        use indexmap::IndexMap;
        use std::collections::BTreeMap;
        use std::path::PathBuf;

        // Create a test node with issues (wrong property order, missing timestamps)
        let mut standard_properties = IndexMap::new();
        // WRONG ORDER: content before key (content replaces description)
        standard_properties.insert(
            "content".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: None,
                extra: BTreeMap::new(),
            },
        );
        standard_properties.insert(
            "key".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: None,
                extra: BTreeMap::new(),
            },
        );
        standard_properties.insert(
            "display_name".to_string(),
            PropertyDef {
                prop_type: "string".to_string(),
                required: Some(true),
                description: None,
                extra: BTreeMap::new(),
            },
        );
        // Missing created_at and updated_at

        let mut node = ParsedNode {
            def: NodeDef {
                name: "TestFixNode".to_string(),
                realm: "shared".to_string(),
                layer: "config".to_string(),
                // v0.17.3 (ADR-036): node_trait removed, provenance is per-instance
                description: "Test node for auto-fix".to_string(),
                standard_properties: Some(standard_properties),
                knowledge_tier: None,
                icon: None,
                properties: None,
                neo4j: None,
                example: None,
            },
            realm: "shared".to_string(),
            layer: "config".to_string(),
            source_path: PathBuf::from("test.yaml"),
        };

        // Validate the node
        let issues = validate_node(&node);

        // Should have PROP_ORDER and TIMESTAMP_REQUIRED issues
        let prop_order_issues: Vec<_> = issues.iter().filter(|i| i.rule == "PROP_ORDER").collect();
        let timestamp_issues: Vec<_> = issues
            .iter()
            .filter(|i| i.rule == "TIMESTAMP_REQUIRED")
            .collect();

        assert!(
            !prop_order_issues.is_empty(),
            "should report property order violation"
        );
        assert!(
            !timestamp_issues.is_empty(),
            "should report missing timestamps"
        );

        // Test that FixEngine can process these issues (DryRun doesn't modify)
        let engine = FixEngine::default();
        for issue in &issues {
            let result = engine.apply_fix(&mut node, issue);
            assert!(result.is_ok(), "FixEngine should handle issue: {:?}", issue);
        }
    }

    #[test]
    fn test_schema_validate_with_fix_safe_strategy() {
        use crate::validation::FixStrategy;
        use std::time::SystemTime;

        // Acquire lock to serialize with other tests that create schema files
        let _lock = SCHEMA_TEST_LOCK.lock().unwrap();

        let Some(root) = test_root() else { return };

        // Create a test directory for temporary nodes
        let test_dir = root.join("packages/core/models/node-classes/shared/config");
        std::fs::create_dir_all(&test_dir).expect("should create test dir");

        // Clean up any leftover test files from previous runs
        cleanup_test_files(&root);

        // Use timestamp to create unique filename for parallel test execution
        // Note: Don't use patterns like "test-*", "__test__*", "_tmp-*" or "*-test*"
        // as they are filtered out by load_all_nodes. Use "autofix_*" instead.
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_node_path = test_dir.join(format!("autofix_{}.yaml", timestamp));
        let yaml_content = r#"node:
  name: TestAutoFix
  realm: shared
  layer: config
  trait: defined
  description: "Test node for auto-fix"
  icon:
    web: test-icon
    terminal: "T"
  standard_properties:
    # WRONG ORDER
    display_name:
      type: string
      required: true
    key:
      type: string
      required: true
    description:
      type: string
      required: true
"#;
        std::fs::write(&test_node_path, yaml_content).expect("should write test node");

        // Run validation with Safe strategy (should apply fixes)
        let result = schema_validate_with_fix(&root, FixStrategy::Safe);

        // Verify function succeeded
        assert!(result.is_ok(), "schema_validate_with_fix should succeed");

        // Verify YAML was actually fixed
        let fixed_content =
            std::fs::read_to_string(&test_node_path).expect("should read fixed file");

        // Check property order is now correct
        // Need to find properties within standard_properties section only
        let lines: Vec<&str> = fixed_content.lines().collect();

        // Find the standard_properties section
        let std_props_start = lines
            .iter()
            .position(|l| l.trim() == "standard_properties:")
            .expect("should have standard_properties");

        // Look for properties after standard_properties line
        // Properties are at 4-space indentation (standard_properties + 2)
        // But nested fields (like description: inside a property) are at 6 spaces
        let key_line = lines[std_props_start..]
            .iter()
            .position(|l| {
                l.starts_with("    ") && !l.starts_with("      ") && l.trim().starts_with("key:")
            })
            .map(|pos| std_props_start + pos)
            .expect("should have key property");
        let display_name_line = lines[std_props_start..]
            .iter()
            .position(|l| {
                l.starts_with("    ")
                    && !l.starts_with("      ")
                    && l.trim().starts_with("display_name:")
            })
            .map(|pos| std_props_start + pos)
            .expect("should have display_name property");
        let description_prop_line = lines[std_props_start..]
            .iter()
            .position(|l| {
                l.starts_with("    ")
                    && !l.starts_with("      ")
                    && l.trim().starts_with("description:")
            })
            .map(|pos| std_props_start + pos)
            .expect("should have description property");

        assert!(
            key_line < display_name_line,
            "key should come before display_name"
        );
        assert!(
            display_name_line < description_prop_line,
            "display_name should come before description property"
        );

        // Check timestamps were added
        assert!(
            fixed_content.contains("created_at:"),
            "should add created_at"
        );
        assert!(
            fixed_content.contains("updated_at:"),
            "should add updated_at"
        );

        // Clean up
        std::fs::remove_file(&test_node_path).ok();
    }
}
