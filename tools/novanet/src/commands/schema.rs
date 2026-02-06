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
        arc_kind::ArcKindGenerator, autowire::AutowireGenerator, colors::ColorsGenerator,
        hierarchy::HierarchyGenerator, icons::IconsGenerator, layer::LayerGenerator,
        mermaid::MermaidGenerator, node_kind::NodeKindGenerator, organizing::OrganizingGenerator,
        tui_icons::TuiIconsGenerator, views::ViewsGenerator,
        visual_encoding::VisualEncodingGenerator,
    };

    vec![
        GeneratorEntry {
            generator: Box::new(OrganizingGenerator),
            output_path: "packages/db/seed/00.5-taxonomy.cypher",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(NodeKindGenerator),
            output_path: "packages/db/seed/01-kinds.cypher",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(ArcKindGenerator),
            output_path: "packages/db/seed/02-arc-kinds.cypher",
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
            output_path: "packages/db/seed/99-autowire-kinds.cypher",
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
        GeneratorEntry {
            generator: Box::new(ViewsGenerator),
            output_path: "packages/core/src/filters/views.generated.ts",
            post_process: None,
        },
        GeneratorEntry {
            generator: Box::new(TuiIconsGenerator),
            output_path: "tools/novanet/src/tui/icons.rs",
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
/// Generator execution order: Organizing → Kind → ArcSchema → Layer → Mermaid → Autowire → Hierarchy → Colors → Icons → VisualEncoding → Views → TuiIcons
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
/// - taxonomy.yaml parses (realms, layers, traits, arc_families)
/// - Every node's realm/layer exists in taxonomy
/// - Every relation's source/target labels match known node names
#[instrument(skip_all, fields(root = %root.display()))]
pub fn schema_validate(root: &Path) -> crate::Result<Vec<ValidationIssue>> {
    let mut issues = Vec::new();

    // 1. Parse nodes
    let nodes = crate::parsers::yaml_node::load_all_nodes(root)?;
    let node_names: std::collections::HashSet<String> =
        nodes.iter().map(|n| n.def.name.clone()).collect();

    // 2. Parse relations
    let rels_doc = crate::parsers::arcs::load_arcs(root)?;

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
                    "{}: realm '{}' not defined in taxonomy.yaml",
                    node.def.name, node.realm
                ),
            });
        }
        if !known_layers.contains(node.layer.as_str()) {
            issues.push(ValidationIssue {
                severity: Severity::Error,
                message: format!(
                    "{}: layer '{}' not defined in taxonomy.yaml",
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

    Ok(issues)
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

        // All 12 generators should succeed
        assert_eq!(results.len(), 12, "expected 12 generator results");

        // Verify generator names and order
        let names: Vec<&str> = results.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "taxonomy",
                "kinds",
                "arc_schema",
                "layers",
                "mermaid",
                "autowire",
                "hierarchy",
                "colors",
                "icons",
                "visual_encoding",
                "views",
                "tui_icons",
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
