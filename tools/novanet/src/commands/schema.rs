//! `novanet schema generate` and `novanet schema validate` commands.
//!
//! - generate: Orchestrates all 12 generators in order
//! - validate: YAML-only validation (no Neo4j in Phase 2)

use crate::generators::Generator;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tracing::instrument;

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
        // ViewsGenerator removed in v0.12.5 — views.yaml is now loaded dynamically
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

    // 2. Parse arc definitions (v10.7+: from arc-classes/ directory)
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
    for arc in &rels_doc.arcs {
        for label in arc.source.labels() {
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

    // 7. Validate schema standardization rules (v0.13.1)
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
    by_trait: HashMap<String, usize>,
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
    let mut by_trait: HashMap<String, usize> = HashMap::new();

    for node in &nodes {
        *by_realm.entry(node.def.realm.clone()).or_insert(0) += 1;
        *by_layer.entry(node.def.layer.clone()).or_insert(0) += 1;
        *by_trait.entry(node.def.node_trait.to_string()).or_insert(0) += 1;
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
            by_trait,
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
        }
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
            println!("\nBy Trait:");
            let mut traits: Vec<_> = stats.nodes.by_trait.iter().collect();
            traits.sort_by_key(|(name, _)| *name);
            for (trait_name, count) in traits {
                println!("  {:<15} {}", trait_name, count);
            }

            println!("\nARC STATISTICS");
            println!("─────────────────────────────────────────────────────");
            println!("Total: {} arcs", stats.arcs.total);
            println!("\nBy Family:");
            let mut families: Vec<_> = stats.arcs.by_family.iter().collect();
            families.sort_by(|(_, a), (_, b)| b.cmp(a)); // Sort by count descending
            for (family, count) in families {
                println!("  {:<15} {}", family, count);
            }
        }
        _ => {
            return Err(crate::NovaNetError::Validation(
                "Unsupported format for stats (use json or table)".into(),
            ));
        }
    }

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

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
    fn schema_generate_dry_run_integration() {
        let Some(root) = test_root() else { return };

        let results = schema_generate(&root, true).expect("should generate all artifacts");

        // All 12 generators should succeed (views removed in v0.12.5)
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
    fn schema_validate_integration() {
        let Some(root) = test_root() else { return };

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
}
