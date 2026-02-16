//! Stats view — raw numbers for CI/scripts.

use crate::blueprint::sources::BlueprintData;
use crate::blueprint::validation::ValidationResult;
use crate::output::OutputFormat;
use serde::{Deserialize, Serialize};

/// Stats data structure for JSON output.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlueprintStats {
    pub node_classes: usize,
    pub arc_classes: usize,
    pub realms: usize,
    pub layers: usize,
    pub traits: TraitStats,
    pub arc_families: ArcFamilyStats,
    pub validation: ValidationStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitStats {
    pub defined: usize,
    pub authored: usize,
    pub imported: usize,
    pub generated: usize,
    pub retrieved: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcFamilyStats {
    pub ownership: usize,
    pub localization: usize,
    pub semantic: usize,
    pub generation: usize,
    pub mining: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationStats {
    pub passed: bool,
    pub errors: usize,
    pub warnings: usize,
}

/// Render stats view.
pub fn render(data: &BlueprintData, format: OutputFormat) -> String {
    let stats = collect_stats(data);

    match format {
        OutputFormat::Json => serde_json::to_string_pretty(&stats).unwrap_or_default(),
        OutputFormat::Table | OutputFormat::Cypher => render_table(&stats),
    }
}

fn collect_stats(data: &BlueprintData) -> BlueprintStats {
    use crate::parsers::arcs::ArcFamily;
    use crate::parsers::yaml_node::NodeTrait;

    let by_trait = data.nodes_by_trait();
    let by_family = data.arcs_by_family();
    let validation = ValidationResult::validate(data);

    BlueprintStats {
        node_classes: data.node_class_count(),
        arc_classes: data.arc_count(),
        realms: data.realm_count(),
        layers: data.layer_count(),
        traits: TraitStats {
            defined: by_trait
                .get(&NodeTrait::Defined)
                .map(|v| v.len())
                .unwrap_or(0),
            authored: by_trait
                .get(&NodeTrait::Authored)
                .map(|v| v.len())
                .unwrap_or(0),
            imported: by_trait
                .get(&NodeTrait::Imported)
                .map(|v| v.len())
                .unwrap_or(0),
            generated: by_trait
                .get(&NodeTrait::Generated)
                .map(|v| v.len())
                .unwrap_or(0),
            retrieved: by_trait
                .get(&NodeTrait::Retrieved)
                .map(|v| v.len())
                .unwrap_or(0),
        },
        arc_families: ArcFamilyStats {
            ownership: by_family
                .get(&ArcFamily::Ownership)
                .map(|v| v.len())
                .unwrap_or(0),
            localization: by_family
                .get(&ArcFamily::Localization)
                .map(|v| v.len())
                .unwrap_or(0),
            semantic: by_family
                .get(&ArcFamily::Semantic)
                .map(|v| v.len())
                .unwrap_or(0),
            generation: by_family
                .get(&ArcFamily::Generation)
                .map(|v| v.len())
                .unwrap_or(0),
            mining: by_family
                .get(&ArcFamily::Mining)
                .map(|v| v.len())
                .unwrap_or(0),
        },
        validation: ValidationStats {
            passed: validation.is_valid(),
            errors: validation.error_count(),
            warnings: validation.warning_count(),
        },
    }
}

fn render_table(stats: &BlueprintStats) -> String {
    format!(
        "BLUEPRINT STATS\n\
         ───────────────────────────────────────\n\
         Classes:          {}\n\
         ArcClasses:       {}\n\
         Realms:           {}\n\
         Layers:           {}\n\
         \n\
         TRAITS (ADR-024)\n\
         ───────────────────────────────────────\n\
         defined:          {}\n\
         authored:         {}\n\
         imported:         {}\n\
         generated:        {}\n\
         retrieved:        {}\n\
         \n\
         ARC FAMILIES\n\
         ───────────────────────────────────────\n\
         ownership:        {}\n\
         localization:     {}\n\
         semantic:         {}\n\
         generation:       {}\n\
         mining:           {}\n\
         \n\
         VALIDATION\n\
         ───────────────────────────────────────\n\
         passed:           {}\n\
         errors:           {}\n\
         warnings:         {}\n",
        stats.node_classes,
        stats.arc_classes,
        stats.realms,
        stats.layers,
        stats.traits.defined,
        stats.traits.authored,
        stats.traits.imported,
        stats.traits.generated,
        stats.traits.retrieved,
        stats.arc_families.ownership,
        stats.arc_families.localization,
        stats.arc_families.semantic,
        stats.arc_families.generation,
        stats.arc_families.mining,
        stats.validation.passed,
        stats.validation.errors,
        stats.validation.warnings,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_json() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let output = render(&data, OutputFormat::Json);

        assert!(
            output.contains("\"node_classes\""),
            "Should have node_classes"
        );
        assert!(output.contains("\"validation\""), "Should have validation");

        // Verify it's valid JSON
        let parsed: Result<BlueprintStats, _> = serde_json::from_str(&output);
        assert!(parsed.is_ok(), "Should be valid JSON");
    }

    #[test]
    fn test_stats_table() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let output = render(&data, OutputFormat::Table);

        assert!(output.contains("BLUEPRINT STATS"), "Should have header");
        assert!(output.contains("Classes:"), "Should have Classes");
        assert!(
            output.contains("VALIDATION"),
            "Should have validation section"
        );
    }
}
