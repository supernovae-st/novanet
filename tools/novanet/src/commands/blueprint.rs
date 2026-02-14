//! Blueprint command — comprehensive schema-graph visualization and validation.
//!
//! Replaces the basic `novanet meta` with rich ASCII output.

use crate::blueprint::sources::BlueprintData;
use crate::blueprint::views::{self, BlueprintView};
use crate::db::Db;
use crate::output::OutputFormat;
use std::path::Path;
use tracing::instrument;

/// Run the blueprint command.
///
/// # Arguments
/// * `db` - Optional Neo4j connection (required for some views like coverage)
/// * `root` - Monorepo root path
/// * `view` - Specific view to render (None for default overview)
/// * `format` - Output format (Table, Json, Cypher)
/// * `no_validate` - Skip validation for faster output
#[instrument(skip(db))]
pub async fn run_blueprint(
    db: Option<&Db>,
    root: &Path,
    view: Option<BlueprintView>,
    format: OutputFormat,
    no_validate: bool,
) -> crate::Result<()> {
    // Load data from YAML (and Neo4j if connected)
    let data = if let Some(db) = db {
        BlueprintData::from_all(root, db).await?
    } else {
        BlueprintData::from_yaml(root)?
    };

    // Render the appropriate view
    let output = match view {
        None => views::render_default(&data, !no_validate),
        Some(v) => views::render_view(&data, v, format),
    };

    println!("{}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::resolve_root;

    #[test]
    fn test_blueprint_yaml_only() {
        let root = resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load data");

        // Test default view
        let output = views::render_default(&data, true);
        assert!(output.contains("NOVANET BLUEPRINT"));

        // Test stats view
        let stats = views::render_view(&data, BlueprintView::Stats, OutputFormat::Table);
        assert!(stats.contains("Classes"));
    }

    #[test]
    fn test_all_views_render() {
        let root = resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load data");

        // Test each view doesn't panic
        let views = [
            BlueprintView::Tree,
            BlueprintView::Flow,
            BlueprintView::Arcs,
            BlueprintView::Stats,
            BlueprintView::Glossary,
            BlueprintView::Cardinality,
        ];

        for view in views {
            let output = views::render_view(&data, view, OutputFormat::Table);
            assert!(!output.is_empty(), "View {:?} should produce output", view);
        }
    }
}
