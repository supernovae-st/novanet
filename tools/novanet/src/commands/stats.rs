//! `novanet stats` — Schema statistics from YAML (offline, no Neo4j required).
//!
//! Shows graph statistics extracted from schema YAML files:
//! - Node class counts by realm, layer, and trait
//! - Arc class counts by family and scope
//!
//! Output formats: text (default), json, yaml

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::instrument;

// ─────────────────────────────────────────────────────────────────────────────
// Stats Structures
// ─────────────────────────────────────────────────────────────────────────────

/// Complete schema statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaStats {
    /// Node class statistics.
    pub nodes: NodeStats,
    /// Arc class statistics (optional, only when --include-arcs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arcs: Option<ArcStats>,
}

/// Node class statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStats {
    /// Total number of node classes.
    pub total: usize,
    /// Count by realm (shared, org).
    pub by_realm: HashMap<String, usize>,
    /// Count by layer (10 layers: 4 shared + 6 org).
    pub by_layer: HashMap<String, usize>,
    /// Count by trait (5 traits: defined, authored, imported, generated, retrieved).
    pub by_trait: HashMap<String, usize>,
}

/// Arc class statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcStats {
    /// Total number of arc classes.
    pub total: usize,
    /// Count by family (6 families: ownership, localization, semantic, generation, mining, schema).
    pub by_family: HashMap<String, usize>,
    /// Count by scope (intra_realm, cross_realm).
    pub by_scope: HashMap<String, usize>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Stats Format
// ─────────────────────────────────────────────────────────────────────────────

/// Output format for stats command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
pub enum StatsFormat {
    /// Human-readable table (default).
    #[default]
    Text,
    /// JSON for programmatic use.
    Json,
    /// YAML for documentation.
    Yaml,
}

// ─────────────────────────────────────────────────────────────────────────────
// Stats Computation
// ─────────────────────────────────────────────────────────────────────────────

/// Compute node class statistics from YAML files.
fn compute_node_stats(root: &Path) -> crate::Result<NodeStats> {
    let nodes = crate::parsers::yaml_node::load_all_nodes(root)?;

    let mut by_realm: HashMap<String, usize> = HashMap::new();
    let mut by_layer: HashMap<String, usize> = HashMap::new();
    let mut by_trait: HashMap<String, usize> = HashMap::new();

    for node in &nodes {
        *by_realm.entry(node.def.realm.clone()).or_insert(0) += 1;
        *by_layer.entry(node.def.layer.clone()).or_insert(0) += 1;
        *by_trait.entry(node.def.node_trait.to_string()).or_insert(0) += 1;
    }

    Ok(NodeStats {
        total: nodes.len(),
        by_realm,
        by_layer,
        by_trait,
    })
}

/// Compute arc class statistics from YAML files.
fn compute_arc_stats(root: &Path) -> crate::Result<ArcStats> {
    let arcs_doc = crate::parsers::arcs::load_arc_classes_from_files(root)?;

    let mut by_family: HashMap<String, usize> = HashMap::new();
    let mut by_scope: HashMap<String, usize> = HashMap::new();

    for arc in &arcs_doc.arcs {
        *by_family.entry(arc.family.to_string()).or_insert(0) += 1;

        // Scope: intra_realm or cross_realm (default to intra_realm if not specified)
        let scope = arc
            .scope
            .clone()
            .unwrap_or_else(|| "intra_realm".to_string());
        *by_scope.entry(scope).or_insert(0) += 1;
    }

    Ok(ArcStats {
        total: arcs_doc.arcs.len(),
        by_family,
        by_scope,
    })
}

/// Compute complete schema statistics.
pub fn compute_stats(root: &Path, include_arcs: bool) -> crate::Result<SchemaStats> {
    let nodes = compute_node_stats(root)?;
    let arcs = if include_arcs {
        Some(compute_arc_stats(root)?)
    } else {
        None
    };

    Ok(SchemaStats { nodes, arcs })
}

// ─────────────────────────────────────────────────────────────────────────────
// Output Formatting
// ─────────────────────────────────────────────────────────────────────────────

/// Format stats as JSON string.
fn format_json(stats: &SchemaStats) -> crate::Result<String> {
    serde_json::to_string_pretty(stats)
        .map_err(|e| crate::NovaNetError::Validation(format!("JSON serialization failed: {}", e)))
}

/// Format stats as YAML string.
fn format_yaml(stats: &SchemaStats) -> crate::Result<String> {
    serde_yaml::to_string(stats)
        .map_err(|e| crate::NovaNetError::Validation(format!("YAML serialization failed: {}", e)))
}

/// Format stats as human-readable text table.
fn format_text(stats: &SchemaStats, detailed: bool) -> String {
    let mut output = String::new();

    // Header
    output.push_str("\nNovaNet Schema Statistics\n");
    output.push_str("═══════════════════════════════════════════════════════════════\n\n");

    // Summary line
    if let Some(ref arcs) = stats.arcs {
        output.push_str(&format!(
            "Total: {} node classes, {} arc classes\n\n",
            stats.nodes.total, arcs.total
        ));
    } else {
        output.push_str(&format!("Total: {} node classes\n\n", stats.nodes.total));
    }

    // Node statistics
    output.push_str("NODE CLASSES\n");
    output.push_str("───────────────────────────────────────────────────────────────\n");

    // By Realm (always show)
    output.push_str("\n  By Realm:\n");
    let mut realms: Vec<_> = stats.nodes.by_realm.iter().collect();
    realms.sort_by_key(|(name, _)| *name);
    for (realm, count) in realms {
        let pct = (*count as f64 / stats.nodes.total as f64) * 100.0;
        output.push_str(&format!("    {:<12} {:>3}  ({:.0}%)\n", realm, count, pct));
    }

    if detailed {
        // By Layer (detailed only)
        output.push_str("\n  By Layer:\n");
        let mut layers: Vec<_> = stats.nodes.by_layer.iter().collect();
        layers.sort_by_key(|(name, _)| *name);
        for (layer, count) in layers {
            let pct = (*count as f64 / stats.nodes.total as f64) * 100.0;
            output.push_str(&format!("    {:<12} {:>3}  ({:.0}%)\n", layer, count, pct));
        }

        // By Trait (detailed only)
        output.push_str("\n  By Trait:\n");
        let mut traits: Vec<_> = stats.nodes.by_trait.iter().collect();
        traits.sort_by_key(|(name, _)| *name);
        for (trait_name, count) in traits {
            let pct = (*count as f64 / stats.nodes.total as f64) * 100.0;
            output.push_str(&format!(
                "    {:<12} {:>3}  ({:.0}%)\n",
                trait_name, count, pct
            ));
        }
    }

    // Arc statistics (if included)
    if let Some(ref arcs) = stats.arcs {
        output.push_str("\nARC CLASSES\n");
        output.push_str("───────────────────────────────────────────────────────────────\n");

        // By Family (always show for arcs)
        output.push_str("\n  By Family:\n");
        let mut families: Vec<_> = arcs.by_family.iter().collect();
        families.sort_by(|(_, a), (_, b)| b.cmp(a)); // Sort by count descending
        for (family, count) in families {
            let pct = (*count as f64 / arcs.total as f64) * 100.0;
            output.push_str(&format!("    {:<12} {:>3}  ({:.0}%)\n", family, count, pct));
        }

        if detailed {
            // By Scope (detailed only)
            output.push_str("\n  By Scope:\n");
            let mut scopes: Vec<_> = arcs.by_scope.iter().collect();
            scopes.sort_by_key(|(name, _)| *name);
            for (scope, count) in scopes {
                let pct = (*count as f64 / arcs.total as f64) * 100.0;
                output.push_str(&format!("    {:<12} {:>3}  ({:.0}%)\n", scope, count, pct));
            }
        }
    }

    output
}

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

/// Run the stats command.
///
/// # Arguments
/// * `root` - Monorepo root path
/// * `format` - Output format (text, json, yaml)
/// * `detailed` - Show breakdown by category (only affects text format)
/// * `include_arcs` - Include arc statistics
#[instrument(skip_all, fields(root = %root.display()))]
pub fn run_stats(
    root: &Path,
    format: StatsFormat,
    detailed: bool,
    include_arcs: bool,
) -> crate::Result<()> {
    let stats = compute_stats(root, include_arcs)?;

    let output = match format {
        StatsFormat::Text => format_text(&stats, detailed),
        StatsFormat::Json => format_json(&stats)?,
        StatsFormat::Yaml => format_yaml(&stats)?,
    };

    println!("{}", output);
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

    // ─────────────────────────────────────────────────────────────────────────
    // Unit Tests: Stats Computation
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    #[ignore = "requires monorepo with packages/core/models/ (run locally)"]
    fn compute_node_stats_integration() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let stats = compute_node_stats(&root).expect("should compute node stats");

        // v0.17.2: 57 node classes (36 shared + 21 org)
        assert!(
            stats.total >= 57,
            "expected at least 57 node classes, got {}",
            stats.total
        );

        // Should have exactly 2 realms
        assert_eq!(stats.by_realm.len(), 2, "should have 2 realms");
        assert!(
            stats.by_realm.contains_key("shared"),
            "should have shared realm"
        );
        assert!(stats.by_realm.contains_key("org"), "should have org realm");

        // Should have at least 9 layers (actual count from models/layers/)
        assert!(
            stats.by_layer.len() >= 9,
            "expected at least 9 layers, got {}",
            stats.by_layer.len()
        );

        // Should have 5 traits
        assert_eq!(stats.by_trait.len(), 5, "should have 5 traits");
        for trait_name in ["defined", "authored", "imported", "generated", "retrieved"] {
            assert!(
                stats.by_trait.contains_key(trait_name),
                "should have {} trait",
                trait_name
            );
        }

        // Total should match sum of realms
        let realm_sum: usize = stats.by_realm.values().sum();
        assert_eq!(
            realm_sum, stats.total,
            "realm sum should match total: {} vs {}",
            realm_sum, stats.total
        );

        // Total should match sum of layers
        let layer_sum: usize = stats.by_layer.values().sum();
        assert_eq!(
            layer_sum, stats.total,
            "layer sum should match total: {} vs {}",
            layer_sum, stats.total
        );

        // Total should match sum of traits
        let trait_sum: usize = stats.by_trait.values().sum();
        assert_eq!(
            trait_sum, stats.total,
            "trait sum should match total: {} vs {}",
            trait_sum, stats.total
        );
    }

    #[test]
    #[ignore = "requires monorepo with packages/core/models/ (run locally)"]
    fn compute_arc_stats_integration() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let stats = compute_arc_stats(&root).expect("should compute arc stats");

        // v0.18.0: 140 arc classes
        assert!(
            stats.total >= 140,
            "expected at least 140 arc classes, got {}",
            stats.total
        );

        // Should have 6 families
        assert_eq!(stats.by_family.len(), 6, "should have 6 arc families");
        for family in [
            "ownership",
            "localization",
            "semantic",
            "generation",
            "mining",
            "schema",
        ] {
            assert!(
                stats.by_family.contains_key(family),
                "should have {} family",
                family
            );
        }

        // Should have scopes
        assert!(!stats.by_scope.is_empty(), "should have at least one scope");

        // Total should match sum of families
        let family_sum: usize = stats.by_family.values().sum();
        assert_eq!(
            family_sum, stats.total,
            "family sum should match total: {} vs {}",
            family_sum, stats.total
        );

        // Total should match sum of scopes
        let scope_sum: usize = stats.by_scope.values().sum();
        assert_eq!(
            scope_sum, stats.total,
            "scope sum should match total: {} vs {}",
            scope_sum, stats.total
        );
    }

    #[test]
    #[ignore = "requires monorepo with packages/core/models/ (run locally)"]
    fn compute_stats_nodes_only() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let stats = compute_stats(&root, false).expect("should compute stats without arcs");

        assert!(stats.nodes.total > 0, "should have node stats");
        assert!(stats.arcs.is_none(), "should not have arc stats");
    }

    #[test]
    #[ignore = "requires monorepo with packages/core/models/ (run locally)"]
    fn compute_stats_with_arcs() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let stats = compute_stats(&root, true).expect("should compute stats with arcs");

        assert!(stats.nodes.total > 0, "should have node stats");
        assert!(stats.arcs.is_some(), "should have arc stats");
        assert!(
            stats.arcs.as_ref().unwrap().total > 0,
            "should have arc count"
        );
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Unit Tests: Output Formatting
    // ─────────────────────────────────────────────────────────────────────────

    fn sample_stats() -> SchemaStats {
        let mut by_realm = HashMap::new();
        by_realm.insert("shared".to_string(), 39); // v0.17: -Market
        by_realm.insert("org".to_string(), 19);

        let mut by_layer = HashMap::new();
        by_layer.insert("config".to_string(), 4);
        by_layer.insert("locale".to_string(), 5); // v0.17: -Market
        by_layer.insert("knowledge".to_string(), 24);
        by_layer.insert("foundation".to_string(), 6);

        let mut by_trait = HashMap::new();
        by_trait.insert("defined".to_string(), 20);
        by_trait.insert("authored".to_string(), 5);
        by_trait.insert("imported".to_string(), 25);
        by_trait.insert("generated".to_string(), 8);
        by_trait.insert("retrieved".to_string(), 3);

        let mut by_family = HashMap::new();
        by_family.insert("ownership".to_string(), 70);
        by_family.insert("localization".to_string(), 20);
        by_family.insert("semantic".to_string(), 50);
        by_family.insert("generation".to_string(), 12);
        by_family.insert("mining".to_string(), 6);
        by_family.insert("schema".to_string(), 24);

        let mut by_scope = HashMap::new();
        by_scope.insert("intra_realm".to_string(), 150);
        by_scope.insert("cross_realm".to_string(), 32);

        SchemaStats {
            nodes: NodeStats {
                total: 58, // v0.17: removed Market
                by_realm,
                by_layer,
                by_trait,
            },
            arcs: Some(ArcStats {
                total: 175, // v0.17: removed Market arcs
                by_family,
                by_scope,
            }),
        }
    }

    #[test]
    fn format_json_output() {
        let stats = sample_stats();
        let json = format_json(&stats).expect("should format as JSON");

        // Should be valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("should parse as JSON");

        // Should have expected structure
        assert!(parsed["nodes"]["total"].is_number());
        assert!(parsed["arcs"]["total"].is_number());
        assert!(parsed["nodes"]["by_realm"]["shared"].is_number());
    }

    #[test]
    fn format_yaml_output() {
        let stats = sample_stats();
        let yaml = format_yaml(&stats).expect("should format as YAML");

        // Should be valid YAML
        let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("should parse as YAML");

        // Should have expected structure
        assert!(parsed["nodes"]["total"].is_number());
        assert!(parsed["arcs"]["total"].is_number());
    }

    #[test]
    fn format_text_summary() {
        let stats = sample_stats();
        let text = format_text(&stats, false);

        // Should contain header
        assert!(text.contains("NovaNet Schema Statistics"));

        // Should contain totals from sample_stats() mock data
        assert!(text.contains("58 node classes"));
        assert!(text.contains("175 arc classes"));

        // Should contain realm breakdown (always shown)
        assert!(text.contains("By Realm:"));
        assert!(text.contains("shared"));
        assert!(text.contains("org"));

        // Should NOT contain layer/trait breakdown in non-detailed mode
        // (Realm is always shown, but layer/trait are detailed-only for nodes)
    }

    #[test]
    fn format_text_detailed() {
        let stats = sample_stats();
        let text = format_text(&stats, true);

        // Should contain all breakdowns
        assert!(text.contains("By Realm:"));
        assert!(text.contains("By Layer:"));
        assert!(text.contains("By Trait:"));
        assert!(text.contains("By Family:"));
        assert!(text.contains("By Scope:"));

        // Should contain specific values
        assert!(text.contains("defined"));
        assert!(text.contains("authored"));
        assert!(text.contains("ownership"));
        assert!(text.contains("intra_realm"));
    }

    #[test]
    fn format_text_nodes_only() {
        let stats = SchemaStats {
            nodes: sample_stats().nodes,
            arcs: None,
        };
        let text = format_text(&stats, true);

        // Should contain node stats
        assert!(text.contains("NODE CLASSES"));

        // Should NOT contain arc stats
        assert!(!text.contains("ARC CLASSES"));
        assert!(!text.contains("By Family:"));
    }

    #[test]
    fn json_skips_none_arcs() {
        let stats = SchemaStats {
            nodes: sample_stats().nodes,
            arcs: None,
        };
        let json = format_json(&stats).expect("should format as JSON");

        // Should NOT contain arcs key when None
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(
            parsed.get("arcs").is_none(),
            "arcs should be omitted when None"
        );
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Unit Tests: StatsFormat enum
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn stats_format_default_is_text() {
        assert_eq!(StatsFormat::default(), StatsFormat::Text);
    }
}
