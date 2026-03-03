//! Diff command: `novanet diff`.
//!
//! Compares schema YAML definitions with Neo4j database state to detect drift.
//! Identifies differences in node classes, arc classes, and their properties.

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

use crate::db::{Db, RowExt};
use crate::parsers::arcs::load_arc_classes_from_files;
use crate::parsers::yaml_node::{load_all_nodes, NodeTrait};

// =============================================================================
// TYPES
// =============================================================================

/// Output format for diff results.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum DiffFormat {
    /// Human-readable diff output (default)
    #[default]
    Human,
    /// JSON structured output
    Json,
}

/// Arguments for the diff command.
#[derive(Debug, Clone, Parser)]
#[command(about = "Compare schema YAML with Neo4j database state")]
pub struct DiffArgs {
    /// Output format
    #[arg(short, long, default_value = "human")]
    pub format: DiffFormat,

    /// Exit with code 1 if differences found
    #[arg(long, default_value = "false")]
    pub exit_code: bool,

    /// Only compare node classes
    #[arg(long, default_value = "false")]
    pub nodes_only: bool,

    /// Only compare arc classes
    #[arg(long, default_value = "false")]
    pub arcs_only: bool,
}

/// Category of difference detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiffCategory {
    /// Present in YAML but not in Neo4j
    Added,
    /// Present in Neo4j but not in YAML
    Removed,
    /// Present in both but with differences
    Modified,
}

/// A single difference item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffItem {
    /// Category of the difference
    pub category: DiffCategory,
    /// Type of item (node_class or arc_class)
    pub item_type: String,
    /// Name of the item
    pub name: String,
    /// Description of the difference
    pub description: String,
    /// Details about the difference (property changes, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<String>>,
}

/// Complete diff result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    /// Node class differences
    pub node_classes: Vec<DiffItem>,
    /// Arc class differences
    pub arc_classes: Vec<DiffItem>,
    /// Summary statistics
    pub summary: DiffSummary,
}

/// Summary of diff results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffSummary {
    pub nodes_added: usize,
    pub nodes_removed: usize,
    pub nodes_modified: usize,
    pub arcs_added: usize,
    pub arcs_removed: usize,
    pub arcs_modified: usize,
    pub has_differences: bool,
}

/// Node class info from Neo4j.
#[derive(Debug, Clone)]
pub struct Neo4jNodeClass {
    pub name: String,
    pub realm: String,
    pub layer: String,
    pub node_trait: String,
    pub properties: Vec<String>,
}

/// Arc class info from Neo4j.
#[derive(Debug, Clone)]
pub struct Neo4jArcClass {
    pub name: String,
    pub family: String,
    pub source: Vec<String>,
    pub target: Vec<String>,
    pub properties: Vec<String>,
}

// =============================================================================
// TYPE ALIASES
// =============================================================================

/// YAML node class tuple: (name, realm, layer, trait, properties)
pub type YamlNodeClass = (String, String, String, NodeTrait, Vec<String>);

/// YAML arc class tuple: (name, family, source, target, properties)
pub type YamlArcClass = (String, String, Vec<String>, Vec<String>, Vec<String>);

// =============================================================================
// DIFF LOGIC
// =============================================================================

/// Compare YAML node classes with Neo4j Schema:Class nodes.
pub fn diff_node_classes(
    yaml_nodes: &[YamlNodeClass],
    neo4j_nodes: &[Neo4jNodeClass],
) -> Vec<DiffItem> {
    let mut diffs = Vec::new();

    // Build sets for comparison
    let yaml_names: BTreeSet<&str> = yaml_nodes.iter().map(|(n, _, _, _, _)| n.as_str()).collect();
    let neo4j_names: BTreeSet<&str> = neo4j_nodes.iter().map(|n| n.name.as_str()).collect();

    // Find added (in YAML, not in Neo4j)
    for name in yaml_names.difference(&neo4j_names) {
        diffs.push(DiffItem {
            category: DiffCategory::Added,
            item_type: "node_class".to_string(),
            name: name.to_string(),
            description: "Present in YAML but not in Neo4j".to_string(),
            details: None,
        });
    }

    // Find removed (in Neo4j, not in YAML)
    for name in neo4j_names.difference(&yaml_names) {
        diffs.push(DiffItem {
            category: DiffCategory::Removed,
            item_type: "node_class".to_string(),
            name: name.to_string(),
            description: "Present in Neo4j but not in YAML".to_string(),
            details: None,
        });
    }

    // Find modified (in both, but different)
    for name in yaml_names.intersection(&neo4j_names) {
        let yaml_node = yaml_nodes
            .iter()
            .find(|(n, _, _, _, _)| n == *name)
            .expect("name guaranteed to exist from intersection");
        let neo4j_node = neo4j_nodes
            .iter()
            .find(|n| n.name == *name)
            .expect("name guaranteed to exist from intersection");

        let mut differences = Vec::new();

        // Check realm
        if yaml_node.1 != neo4j_node.realm {
            differences.push(format!(
                "realm: YAML={}, Neo4j={}",
                yaml_node.1, neo4j_node.realm
            ));
        }

        // Check layer
        if yaml_node.2 != neo4j_node.layer {
            differences.push(format!(
                "layer: YAML={}, Neo4j={}",
                yaml_node.2, neo4j_node.layer
            ));
        }

        // Check trait
        let yaml_trait = yaml_node.3.to_string();
        if yaml_trait != neo4j_node.node_trait {
            differences.push(format!(
                "trait: YAML={}, Neo4j={}",
                yaml_trait, neo4j_node.node_trait
            ));
        }

        // Check properties (simplified - just check if sets differ)
        let yaml_props: BTreeSet<&str> = yaml_node.4.iter().map(|s| s.as_str()).collect();
        let neo4j_props: BTreeSet<&str> = neo4j_node.properties.iter().map(|s| s.as_str()).collect();
        if yaml_props != neo4j_props {
            let added: Vec<_> = yaml_props.difference(&neo4j_props).collect();
            let removed: Vec<_> = neo4j_props.difference(&yaml_props).collect();
            if !added.is_empty() {
                differences.push(format!("properties added in YAML: {:?}", added));
            }
            if !removed.is_empty() {
                differences.push(format!("properties removed from YAML: {:?}", removed));
            }
        }

        if !differences.is_empty() {
            diffs.push(DiffItem {
                category: DiffCategory::Modified,
                item_type: "node_class".to_string(),
                name: name.to_string(),
                description: "Properties differ between YAML and Neo4j".to_string(),
                details: Some(differences),
            });
        }
    }

    diffs
}

/// Compare YAML arc classes with Neo4j Schema:ArcClass nodes.
pub fn diff_arc_classes(
    yaml_arcs: &[YamlArcClass],
    neo4j_arcs: &[Neo4jArcClass],
) -> Vec<DiffItem> {
    let mut diffs = Vec::new();

    // Build sets for comparison
    let yaml_names: BTreeSet<&str> = yaml_arcs.iter().map(|(n, _, _, _, _)| n.as_str()).collect();
    let neo4j_names: BTreeSet<&str> = neo4j_arcs.iter().map(|a| a.name.as_str()).collect();

    // Find added (in YAML, not in Neo4j)
    for name in yaml_names.difference(&neo4j_names) {
        diffs.push(DiffItem {
            category: DiffCategory::Added,
            item_type: "arc_class".to_string(),
            name: name.to_string(),
            description: "Present in YAML but not in Neo4j".to_string(),
            details: None,
        });
    }

    // Find removed (in Neo4j, not in YAML)
    for name in neo4j_names.difference(&yaml_names) {
        diffs.push(DiffItem {
            category: DiffCategory::Removed,
            item_type: "arc_class".to_string(),
            name: name.to_string(),
            description: "Present in Neo4j but not in YAML".to_string(),
            details: None,
        });
    }

    // Find modified (in both, but different)
    for name in yaml_names.intersection(&neo4j_names) {
        let yaml_arc = yaml_arcs
            .iter()
            .find(|(n, _, _, _, _)| n == *name)
            .expect("name guaranteed to exist from intersection");
        let neo4j_arc = neo4j_arcs
            .iter()
            .find(|a| a.name == *name)
            .expect("name guaranteed to exist from intersection");

        let mut differences = Vec::new();

        // Check family
        if yaml_arc.1 != neo4j_arc.family {
            differences.push(format!(
                "family: YAML={}, Neo4j={}",
                yaml_arc.1, neo4j_arc.family
            ));
        }

        // Check source
        let yaml_source: BTreeSet<&str> = yaml_arc.2.iter().map(|s| s.as_str()).collect();
        let neo4j_source: BTreeSet<&str> = neo4j_arc.source.iter().map(|s| s.as_str()).collect();
        if yaml_source != neo4j_source {
            differences.push(format!(
                "source: YAML={:?}, Neo4j={:?}",
                yaml_arc.2, neo4j_arc.source
            ));
        }

        // Check target
        let yaml_target: BTreeSet<&str> = yaml_arc.3.iter().map(|s| s.as_str()).collect();
        let neo4j_target: BTreeSet<&str> = neo4j_arc.target.iter().map(|s| s.as_str()).collect();
        if yaml_target != neo4j_target {
            differences.push(format!(
                "target: YAML={:?}, Neo4j={:?}",
                yaml_arc.3, neo4j_arc.target
            ));
        }

        // Check properties
        let yaml_props: BTreeSet<&str> = yaml_arc.4.iter().map(|s| s.as_str()).collect();
        let neo4j_props: BTreeSet<&str> = neo4j_arc.properties.iter().map(|s| s.as_str()).collect();
        if yaml_props != neo4j_props {
            let added: Vec<_> = yaml_props.difference(&neo4j_props).collect();
            let removed: Vec<_> = neo4j_props.difference(&yaml_props).collect();
            if !added.is_empty() {
                differences.push(format!("properties added in YAML: {:?}", added));
            }
            if !removed.is_empty() {
                differences.push(format!("properties removed from YAML: {:?}", removed));
            }
        }

        if !differences.is_empty() {
            diffs.push(DiffItem {
                category: DiffCategory::Modified,
                item_type: "arc_class".to_string(),
                name: name.to_string(),
                description: "Properties differ between YAML and Neo4j".to_string(),
                details: Some(differences),
            });
        }
    }

    diffs
}

/// Build summary from diff results.
pub fn build_summary(node_diffs: &[DiffItem], arc_diffs: &[DiffItem]) -> DiffSummary {
    let nodes_added = node_diffs
        .iter()
        .filter(|d| d.category == DiffCategory::Added)
        .count();
    let nodes_removed = node_diffs
        .iter()
        .filter(|d| d.category == DiffCategory::Removed)
        .count();
    let nodes_modified = node_diffs
        .iter()
        .filter(|d| d.category == DiffCategory::Modified)
        .count();

    let arcs_added = arc_diffs
        .iter()
        .filter(|d| d.category == DiffCategory::Added)
        .count();
    let arcs_removed = arc_diffs
        .iter()
        .filter(|d| d.category == DiffCategory::Removed)
        .count();
    let arcs_modified = arc_diffs
        .iter()
        .filter(|d| d.category == DiffCategory::Modified)
        .count();

    let has_differences = nodes_added > 0
        || nodes_removed > 0
        || nodes_modified > 0
        || arcs_added > 0
        || arcs_removed > 0
        || arcs_modified > 0;

    DiffSummary {
        nodes_added,
        nodes_removed,
        nodes_modified,
        arcs_added,
        arcs_removed,
        arcs_modified,
        has_differences,
    }
}

// =============================================================================
// OUTPUT FORMATTING
// =============================================================================

/// Format diff result as human-readable output.
pub fn format_human(result: &DiffResult) -> String {
    let mut output = String::new();

    if !result.summary.has_differences {
        output.push_str("✓ Schema YAML and Neo4j are in sync\n");
        return output;
    }

    output.push_str("Schema Drift Detected\n");
    output.push_str("=====================\n\n");

    // Node classes section
    if !result.node_classes.is_empty() {
        output.push_str("Node Classes:\n");
        output.push_str("─────────────\n");
        for diff in &result.node_classes {
            let symbol = match diff.category {
                DiffCategory::Added => "+",
                DiffCategory::Removed => "-",
                DiffCategory::Modified => "~",
            };
            output.push_str(&format!("  {} {} ({})\n", symbol, diff.name, diff.description));
            if let Some(details) = &diff.details {
                for detail in details {
                    output.push_str(&format!("      {}\n", detail));
                }
            }
        }
        output.push('\n');
    }

    // Arc classes section
    if !result.arc_classes.is_empty() {
        output.push_str("Arc Classes:\n");
        output.push_str("────────────\n");
        for diff in &result.arc_classes {
            let symbol = match diff.category {
                DiffCategory::Added => "+",
                DiffCategory::Removed => "-",
                DiffCategory::Modified => "~",
            };
            output.push_str(&format!("  {} {} ({})\n", symbol, diff.name, diff.description));
            if let Some(details) = &diff.details {
                for detail in details {
                    output.push_str(&format!("      {}\n", detail));
                }
            }
        }
        output.push('\n');
    }

    // Summary
    output.push_str("Summary:\n");
    output.push_str("────────\n");
    output.push_str(&format!(
        "  Nodes: {} added, {} removed, {} modified\n",
        result.summary.nodes_added,
        result.summary.nodes_removed,
        result.summary.nodes_modified
    ));
    output.push_str(&format!(
        "  Arcs:  {} added, {} removed, {} modified\n",
        result.summary.arcs_added,
        result.summary.arcs_removed,
        result.summary.arcs_modified
    ));

    output
}

/// Format diff result as JSON.
pub fn format_json(result: &DiffResult) -> String {
    serde_json::to_string_pretty(result).unwrap_or_else(|_| "{}".to_string())
}

// =============================================================================
// NEO4J QUERIES
// =============================================================================

/// Query Cypher for Schema:Class nodes.
/// Note: Class nodes store their name in the `label` property, not `name`.
const QUERY_NODE_CLASSES: &str = r#"
MATCH (c:Schema:Class)
RETURN c.label AS name,
       c.realm AS realm,
       c.layer AS layer,
       c.trait AS trait,
       COALESCE(c.properties, []) AS properties
ORDER BY c.label
"#;

/// Query Cypher for Schema:ArcClass nodes.
/// Note: ArcClass nodes store their name in the `key` property, not `name`.
const QUERY_ARC_CLASSES: &str = r#"
MATCH (a:Schema:ArcClass)
RETURN a.key AS name,
       a.family AS family,
       COALESCE(a.source, []) AS source,
       COALESCE(a.target, []) AS target,
       COALESCE(a.arc_properties, []) AS properties
ORDER BY a.key
"#;

/// Fetch node classes from Neo4j.
pub async fn fetch_neo4j_node_classes(db: &Db) -> crate::Result<Vec<Neo4jNodeClass>> {
    let rows = db.execute(QUERY_NODE_CLASSES).await?;
    let mut classes = Vec::new();

    for row in rows {
        classes.push(Neo4jNodeClass {
            name: row.str("name"),
            realm: row.str("realm"),
            layer: row.str("layer"),
            node_trait: row.str("trait"),
            properties: row.vec_str("properties"),
        });
    }

    Ok(classes)
}

/// Fetch arc classes from Neo4j.
pub async fn fetch_neo4j_arc_classes(db: &Db) -> crate::Result<Vec<Neo4jArcClass>> {
    let rows = db.execute(QUERY_ARC_CLASSES).await?;
    let mut classes = Vec::new();

    for row in rows {
        classes.push(Neo4jArcClass {
            name: row.str("name"),
            family: row.str("family"),
            source: row.vec_str("source"),
            target: row.vec_str("target"),
            properties: row.vec_str("properties"),
        });
    }

    Ok(classes)
}

// =============================================================================
// MAIN COMMAND
// =============================================================================

/// Run the diff command.
pub async fn run_diff(db: &Db, root: &Path, args: &DiffArgs) -> crate::Result<bool> {
    // Load YAML definitions
    let yaml_nodes = if !args.arcs_only {
        let nodes = load_all_nodes(root)?;
        nodes
            .into_iter()
            .map(|n| {
                let props: Vec<String> = n
                    .def
                    .properties
                    .as_ref()
                    .map(|p| p.keys().cloned().collect())
                    .unwrap_or_default();
                (n.def.name, n.realm, n.layer, n.def.node_trait, props)
            })
            .collect()
    } else {
        Vec::new()
    };

    let yaml_arcs = if !args.nodes_only {
        let arcs_doc = load_arc_classes_from_files(root)?;
        arcs_doc
            .arcs
            .into_iter()
            .map(|a| {
                let source: Vec<String> = a.source.labels().iter().map(|s| s.to_string()).collect();
                let target: Vec<String> = a.target.labels().iter().map(|s| s.to_string()).collect();
                let props: Vec<String> = a.properties.unwrap_or_default();
                (a.arc_type, a.family.to_string(), source, target, props)
            })
            .collect()
    } else {
        Vec::new()
    };

    // Fetch from Neo4j
    let neo4j_nodes = if !args.arcs_only {
        fetch_neo4j_node_classes(db).await?
    } else {
        Vec::new()
    };

    let neo4j_arcs = if !args.nodes_only {
        fetch_neo4j_arc_classes(db).await?
    } else {
        Vec::new()
    };

    // Compute diffs
    let node_diffs = if !args.arcs_only {
        diff_node_classes(&yaml_nodes, &neo4j_nodes)
    } else {
        Vec::new()
    };

    let arc_diffs = if !args.nodes_only {
        diff_arc_classes(&yaml_arcs, &neo4j_arcs)
    } else {
        Vec::new()
    };

    let summary = build_summary(&node_diffs, &arc_diffs);

    let result = DiffResult {
        node_classes: node_diffs,
        arc_classes: arc_diffs,
        summary,
    };

    // Output
    match args.format {
        DiffFormat::Human => print!("{}", format_human(&result)),
        DiffFormat::Json => println!("{}", format_json(&result)),
    }

    // Return whether differences exist (for exit code)
    Ok(result.summary.has_differences)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // DiffArgs parsing tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_diff_format_default() {
        let format = DiffFormat::default();
        assert_eq!(format, DiffFormat::Human);
    }

    #[test]
    fn test_diff_args_default_values() {
        // Parse with no args
        let args = DiffArgs::try_parse_from(["diff"]).unwrap();
        assert_eq!(args.format, DiffFormat::Human);
        assert!(!args.exit_code);
        assert!(!args.nodes_only);
        assert!(!args.arcs_only);
    }

    #[test]
    fn test_diff_args_json_format() {
        let args = DiffArgs::try_parse_from(["diff", "--format", "json"]).unwrap();
        assert_eq!(args.format, DiffFormat::Json);
    }

    #[test]
    fn test_diff_args_exit_code() {
        let args = DiffArgs::try_parse_from(["diff", "--exit-code"]).unwrap();
        assert!(args.exit_code);
    }

    #[test]
    fn test_diff_args_nodes_only() {
        let args = DiffArgs::try_parse_from(["diff", "--nodes-only"]).unwrap();
        assert!(args.nodes_only);
        assert!(!args.arcs_only);
    }

    #[test]
    fn test_diff_args_arcs_only() {
        let args = DiffArgs::try_parse_from(["diff", "--arcs-only"]).unwrap();
        assert!(!args.nodes_only);
        assert!(args.arcs_only);
    }

    // -------------------------------------------------------------------------
    // Diff algorithm tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_diff_node_classes_no_differences() {
        let yaml_nodes = vec![
            ("Page".to_string(), "org".to_string(), "structure".to_string(), NodeTrait::Defined, vec!["key".to_string()]),
            ("Entity".to_string(), "org".to_string(), "semantic".to_string(), NodeTrait::Defined, vec!["key".to_string()]),
        ];

        let neo4j_nodes = vec![
            Neo4jNodeClass {
                name: "Page".to_string(),
                realm: "org".to_string(),
                layer: "structure".to_string(),
                node_trait: "defined".to_string(),
                properties: vec!["key".to_string()],
            },
            Neo4jNodeClass {
                name: "Entity".to_string(),
                realm: "org".to_string(),
                layer: "semantic".to_string(),
                node_trait: "defined".to_string(),
                properties: vec!["key".to_string()],
            },
        ];

        let diffs = diff_node_classes(&yaml_nodes, &neo4j_nodes);
        assert!(diffs.is_empty(), "Expected no differences");
    }

    #[test]
    fn test_diff_node_classes_added() {
        let yaml_nodes = vec![
            ("Page".to_string(), "org".to_string(), "structure".to_string(), NodeTrait::Defined, vec![]),
            ("NewNode".to_string(), "org".to_string(), "foundation".to_string(), NodeTrait::Defined, vec![]),
        ];

        let neo4j_nodes = vec![
            Neo4jNodeClass {
                name: "Page".to_string(),
                realm: "org".to_string(),
                layer: "structure".to_string(),
                node_trait: "defined".to_string(),
                properties: vec![],
            },
        ];

        let diffs = diff_node_classes(&yaml_nodes, &neo4j_nodes);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].category, DiffCategory::Added);
        assert_eq!(diffs[0].name, "NewNode");
    }

    #[test]
    fn test_diff_node_classes_removed() {
        let yaml_nodes = vec![
            ("Page".to_string(), "org".to_string(), "structure".to_string(), NodeTrait::Defined, vec![]),
        ];

        let neo4j_nodes = vec![
            Neo4jNodeClass {
                name: "Page".to_string(),
                realm: "org".to_string(),
                layer: "structure".to_string(),
                node_trait: "defined".to_string(),
                properties: vec![],
            },
            Neo4jNodeClass {
                name: "OldNode".to_string(),
                realm: "org".to_string(),
                layer: "foundation".to_string(),
                node_trait: "defined".to_string(),
                properties: vec![],
            },
        ];

        let diffs = diff_node_classes(&yaml_nodes, &neo4j_nodes);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].category, DiffCategory::Removed);
        assert_eq!(diffs[0].name, "OldNode");
    }

    #[test]
    fn test_diff_node_classes_modified_realm() {
        let yaml_nodes = vec![
            ("Page".to_string(), "shared".to_string(), "structure".to_string(), NodeTrait::Defined, vec![]),
        ];

        let neo4j_nodes = vec![
            Neo4jNodeClass {
                name: "Page".to_string(),
                realm: "org".to_string(),
                layer: "structure".to_string(),
                node_trait: "defined".to_string(),
                properties: vec![],
            },
        ];

        let diffs = diff_node_classes(&yaml_nodes, &neo4j_nodes);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].category, DiffCategory::Modified);
        assert!(diffs[0].details.as_ref().unwrap().iter().any(|d| d.contains("realm")));
    }

    #[test]
    fn test_diff_node_classes_modified_properties() {
        let yaml_nodes = vec![
            ("Page".to_string(), "org".to_string(), "structure".to_string(), NodeTrait::Defined, vec!["key".to_string(), "new_prop".to_string()]),
        ];

        let neo4j_nodes = vec![
            Neo4jNodeClass {
                name: "Page".to_string(),
                realm: "org".to_string(),
                layer: "structure".to_string(),
                node_trait: "defined".to_string(),
                properties: vec!["key".to_string()],
            },
        ];

        let diffs = diff_node_classes(&yaml_nodes, &neo4j_nodes);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].category, DiffCategory::Modified);
        assert!(diffs[0].details.as_ref().unwrap().iter().any(|d| d.contains("new_prop")));
    }

    #[test]
    fn test_diff_arc_classes_no_differences() {
        let yaml_arcs = vec![
            ("HAS_PAGE".to_string(), "ownership".to_string(), vec!["Project".to_string()], vec!["Page".to_string()], vec![]),
        ];

        let neo4j_arcs = vec![
            Neo4jArcClass {
                name: "HAS_PAGE".to_string(),
                family: "ownership".to_string(),
                source: vec!["Project".to_string()],
                target: vec!["Page".to_string()],
                properties: vec![],
            },
        ];

        let diffs = diff_arc_classes(&yaml_arcs, &neo4j_arcs);
        assert!(diffs.is_empty());
    }

    #[test]
    fn test_diff_arc_classes_added() {
        let yaml_arcs = vec![
            ("HAS_PAGE".to_string(), "ownership".to_string(), vec!["Project".to_string()], vec!["Page".to_string()], vec![]),
            ("NEW_ARC".to_string(), "semantic".to_string(), vec!["Entity".to_string()], vec!["Page".to_string()], vec![]),
        ];

        let neo4j_arcs = vec![
            Neo4jArcClass {
                name: "HAS_PAGE".to_string(),
                family: "ownership".to_string(),
                source: vec!["Project".to_string()],
                target: vec!["Page".to_string()],
                properties: vec![],
            },
        ];

        let diffs = diff_arc_classes(&yaml_arcs, &neo4j_arcs);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].category, DiffCategory::Added);
        assert_eq!(diffs[0].name, "NEW_ARC");
    }

    #[test]
    fn test_diff_arc_classes_modified_family() {
        let yaml_arcs = vec![
            ("HAS_PAGE".to_string(), "semantic".to_string(), vec!["Project".to_string()], vec!["Page".to_string()], vec![]),
        ];

        let neo4j_arcs = vec![
            Neo4jArcClass {
                name: "HAS_PAGE".to_string(),
                family: "ownership".to_string(),
                source: vec!["Project".to_string()],
                target: vec!["Page".to_string()],
                properties: vec![],
            },
        ];

        let diffs = diff_arc_classes(&yaml_arcs, &neo4j_arcs);
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].category, DiffCategory::Modified);
        assert!(diffs[0].details.as_ref().unwrap().iter().any(|d| d.contains("family")));
    }

    // -------------------------------------------------------------------------
    // Summary tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_build_summary_no_differences() {
        let node_diffs: Vec<DiffItem> = vec![];
        let arc_diffs: Vec<DiffItem> = vec![];

        let summary = build_summary(&node_diffs, &arc_diffs);
        assert!(!summary.has_differences);
        assert_eq!(summary.nodes_added, 0);
        assert_eq!(summary.nodes_removed, 0);
        assert_eq!(summary.nodes_modified, 0);
    }

    #[test]
    fn test_build_summary_with_differences() {
        let node_diffs = vec![
            DiffItem {
                category: DiffCategory::Added,
                item_type: "node_class".to_string(),
                name: "NewNode".to_string(),
                description: "".to_string(),
                details: None,
            },
            DiffItem {
                category: DiffCategory::Modified,
                item_type: "node_class".to_string(),
                name: "Page".to_string(),
                description: "".to_string(),
                details: None,
            },
        ];
        let arc_diffs = vec![
            DiffItem {
                category: DiffCategory::Removed,
                item_type: "arc_class".to_string(),
                name: "OLD_ARC".to_string(),
                description: "".to_string(),
                details: None,
            },
        ];

        let summary = build_summary(&node_diffs, &arc_diffs);
        assert!(summary.has_differences);
        assert_eq!(summary.nodes_added, 1);
        assert_eq!(summary.nodes_modified, 1);
        assert_eq!(summary.arcs_removed, 1);
    }

    // -------------------------------------------------------------------------
    // Output formatting tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_human_no_differences() {
        let result = DiffResult {
            node_classes: vec![],
            arc_classes: vec![],
            summary: DiffSummary {
                nodes_added: 0,
                nodes_removed: 0,
                nodes_modified: 0,
                arcs_added: 0,
                arcs_removed: 0,
                arcs_modified: 0,
                has_differences: false,
            },
        };

        let output = format_human(&result);
        assert!(output.contains("in sync"));
    }

    #[test]
    fn test_format_human_with_differences() {
        let result = DiffResult {
            node_classes: vec![
                DiffItem {
                    category: DiffCategory::Added,
                    item_type: "node_class".to_string(),
                    name: "NewNode".to_string(),
                    description: "Present in YAML but not in Neo4j".to_string(),
                    details: None,
                },
            ],
            arc_classes: vec![],
            summary: DiffSummary {
                nodes_added: 1,
                nodes_removed: 0,
                nodes_modified: 0,
                arcs_added: 0,
                arcs_removed: 0,
                arcs_modified: 0,
                has_differences: true,
            },
        };

        let output = format_human(&result);
        assert!(output.contains("Schema Drift"));
        assert!(output.contains("+ NewNode"));
    }

    #[test]
    fn test_format_json() {
        let result = DiffResult {
            node_classes: vec![
                DiffItem {
                    category: DiffCategory::Added,
                    item_type: "node_class".to_string(),
                    name: "NewNode".to_string(),
                    description: "test".to_string(),
                    details: None,
                },
            ],
            arc_classes: vec![],
            summary: DiffSummary {
                nodes_added: 1,
                nodes_removed: 0,
                nodes_modified: 0,
                arcs_added: 0,
                arcs_removed: 0,
                arcs_modified: 0,
                has_differences: true,
            },
        };

        let output = format_json(&result);
        assert!(output.contains("\"name\": \"NewNode\""));
        assert!(output.contains("\"category\": \"added\""));
    }

    // -------------------------------------------------------------------------
    // Integration test (requires monorepo)
    // -------------------------------------------------------------------------

    #[test]
    fn test_requires_monorepo() {
        // This test verifies we can parse YAML from the monorepo
        // Skip if not in monorepo environment
        let root = crate::config::resolve_root(None).ok();
        if root.is_none() {
            eprintln!("Skipping test: not in monorepo");
            return;
        }

        let root = root.unwrap();
        let nodes = load_all_nodes(&root);
        assert!(nodes.is_ok(), "Should be able to load nodes from monorepo");

        let arcs = load_arc_classes_from_files(&root);
        assert!(arcs.is_ok(), "Should be able to load arcs from monorepo");
    }
}
