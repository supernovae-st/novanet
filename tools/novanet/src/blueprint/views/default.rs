//! Default blueprint view — rich overview of the meta-graph.

use crate::blueprint::ascii::{self, progress_bar_compact, trait_symbol, arc_family_arrow, realm_icon, truncate};
use crate::blueprint::sources::BlueprintData;
use crate::blueprint::validation::ValidationResult;
use crate::parsers::yaml_node::NodeTrait;
use std::fmt::Write;

/// Render the default overview.
pub fn render(data: &BlueprintData, validate: bool) -> String {
    let mut out = String::new();

    // Header
    out.push_str(&render_header(data));
    out.push('\n');

    // Stats
    out.push_str(&render_stats(data));
    out.push('\n');

    // Realms
    out.push_str(&render_realms(data));
    out.push('\n');

    // Layers
    out.push_str(&render_layers(data));
    out.push('\n');

    // Traits
    out.push_str(&render_traits(data));
    out.push('\n');

    // Core Flow
    out.push_str(&render_core_flow());
    out.push('\n');

    // Arc Families
    out.push_str(&render_arc_families(data));
    out.push('\n');

    // Validation (if enabled)
    if validate {
        out.push_str(&render_validation(data));
        out.push('\n');
    }

    // Views hint
    out.push_str(&render_views_hint());

    out
}

fn render_header(_data: &BlueprintData) -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(
        "╭──────────────────────────────────────────────────────────────────────────────╮\n\
         │  ◉ NOVANET BLUEPRINT                                              v{}   │\n\
         ╰──────────────────────────────────────────────────────────────────────────────╯",
        version
    )
}

fn render_stats(data: &BlueprintData) -> String {
    format!(
        "┌──────────────────────────────────────────────────────────────────────────────┐\n\
         │  STATS        {} NodeKinds │ {} ArcKinds │ {} Realms │ {} Layers             │\n\
         └──────────────────────────────────────────────────────────────────────────────┘",
        data.node_kind_count(),
        data.arc_count(),
        data.realm_count(),
        data.layer_count()
    )
}

fn render_realms(data: &BlueprintData) -> String {
    let mut out = String::new();
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  REALMS                                                                      │\n");

    let by_realm = data.nodes_by_realm();
    let total = data.node_kind_count();

    for realm_def in &data.taxonomy.node_realms {
        let count = by_realm.get(realm_def.key.as_str()).map(|v| v.len()).unwrap_or(0);
        let bar = progress_bar_compact(count, total, 16);
        let icon = realm_icon(&realm_def.key);
        let desc = match realm_def.key.as_str() {
            "global" => "(read-only, universal knowledge)",
            "tenant" => "(business-specific content)",
            _ => "",
        };
        let _ = writeln!(out,
            "│  {} {:<8} {} {:>2} kinds {}{}│",
            icon,
            realm_def.key,
            bar,
            count,
            desc,
            " ".repeat(80 - 42 - desc.len())
        );
    }

    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘");
    out
}

fn render_layers(data: &BlueprintData) -> String {
    let mut out = String::new();
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  LAYERS                                                                      │\n");

    let by_layer = data.nodes_by_layer();
    let max_count = by_layer.values().map(|v| v.len()).max().unwrap_or(1);

    for realm_def in &data.taxonomy.node_realms {
        for layer_def in &realm_def.layers {
            let count = by_layer.get(layer_def.key.as_str()).map(|v| v.len()).unwrap_or(0);
            let bar_width = (count * 10 / max_count).max(1);
            let bar = "█".repeat(bar_width);
            let _ = writeln!(out,
                "│  {} {:<18} {:<10} {:>2}   {}{}│",
                &layer_def.emoji,
                layer_def.key,
                bar,
                count,
                truncate(&layer_def.display_name, 25),
                " ".repeat(80 - 52 - layer_def.display_name.len().min(25))
            );
        }
    }

    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘");
    out
}

fn render_traits(data: &BlueprintData) -> String {
    let mut out = String::new();
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  TRAITS (how nodes behave with locales)                                      │\n");

    let by_trait = data.nodes_by_trait();

    let trait_info = [
        (NodeTrait::Invariant, "invariant", "Same across all locales", "Entity, Page, Block"),
        (NodeTrait::Localized, "localized", "Native content per locale", "EntityContent, ProjectContent"),
        (NodeTrait::Knowledge, "knowledge", "Locale-specific atoms", "Term, Expression, Taboo"),
        (NodeTrait::Derived, "derived", "Generated from invariants", "PageGenerated, BlockGenerated"),
        (NodeTrait::Job, "job", "Async processing tasks", "GenerationJob"),
    ];

    for (trait_enum, key, description, examples) in trait_info {
        let symbol = trait_symbol(key);
        let count = by_trait.get(&trait_enum).map(|v| v.len()).unwrap_or(0);
        let count_str = format!("({})", count);
        let padding = 80usize.saturating_sub(53).saturating_sub(examples.len());
        let _ = writeln!(out,
            "│  {} {:<10} {:<28} {:>4} │ {}{}│",
            symbol,
            key,
            description,
            count_str,
            examples,
            " ".repeat(padding)
        );
    }

    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘");
    out
}

fn render_core_flow() -> String {
    "┌──────────────────────────────────────────────────────────────────────────────┐\n\
     │  CORE FLOW                                                                   │\n\
     │                                                                              │\n\
     │  Entity ────[HAS_CONTENT]────► EntityContent ────[USES_TERM]────► Term       │\n\
     │     │                               │                                        │\n\
     │     │                               ▼                                        │\n\
     │     │                         (LLM generation)                               │\n\
     │     │                               │                                        │\n\
     │     └────[HAS_GENERATED]────► PageGenerated ◄────[GENERATED_FOR]──── Page    │\n\
     │                                                                              │\n\
     └──────────────────────────────────────────────────────────────────────────────┘".to_string()
}

fn render_arc_families(data: &BlueprintData) -> String {
    let mut out = String::new();
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");
    out.push_str("│  ARC FAMILIES                                                                │\n");

    let by_family = data.arcs_by_family();

    let family_info = [
        ("ownership", "Parent-child hierarchy (HAS_PAGE, HAS_BLOCK)"),
        ("localization", "Invariant↔localized links (HAS_CONTENT)"),
        ("semantic", "Meaning connections (USES_ENTITY)"),
        ("generation", "LLM pipeline (HAS_GENERATED)"),
        ("mining", "Knowledge extraction (EXTRACTS_TERM)"),
    ];

    for (key, description) in family_info {
        let arrow = arc_family_arrow(key);
        let family_enum = match key {
            "ownership" => crate::parsers::arcs::ArcFamily::Ownership,
            "localization" => crate::parsers::arcs::ArcFamily::Localization,
            "semantic" => crate::parsers::arcs::ArcFamily::Semantic,
            "generation" => crate::parsers::arcs::ArcFamily::Generation,
            "mining" => crate::parsers::arcs::ArcFamily::Mining,
            _ => continue,
        };
        let count = by_family.get(&family_enum).map(|v| v.len()).unwrap_or(0);
        let _ = writeln!(out,
            "│  {} {:<14} {:>3} arcs   {}{}│",
            arrow,
            key,
            count,
            description,
            " ".repeat(80 - 30 - description.len())
        );
    }

    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘");
    out
}

fn render_validation(data: &BlueprintData) -> String {
    let result = ValidationResult::validate(data);

    let mut out = String::new();
    out.push_str("┌──────────────────────────────────────────────────────────────────────────────┐\n");

    let status = if result.is_valid() {
        "✓ COHERENT"
    } else {
        "⚠ ISSUES"
    };

    let _ = writeln!(out, "│  VALIDATION                                                      {} │", status);

    for check in &result.checks {
        let icon = if check.passed { "✓" } else { "⚠" };
        let _ = writeln!(out, "│  {} {}{}│",
            icon,
            ascii::pad_right(&check.name, 68),
            " ".repeat(80 - 76)
        );
        if let Some(ref details) = check.details {
            let _ = writeln!(out, "│     └── {}{}│",
                truncate(details, 65),
                " ".repeat(80 - 75)
            );
        }
    }

    if !result.issues.is_empty() {
        out.push_str("│                                                                              │\n");
        let _ = writeln!(out, "│  💡 Run: novanet blueprint --view=audit for details{}│", " ".repeat(80 - 58));
    }

    out.push_str("└──────────────────────────────────────────────────────────────────────────────┘");
    out
}

fn render_views_hint() -> String {
    "╭──────────────────────────────────────────────────────────────────────────────╮\n\
     │  📖 Views: --view=tree|flow|content|arcs|cardinality|glossary|audit|deps|   │\n\
     │            coverage|stats                                                    │\n\
     ╰──────────────────────────────────────────────────────────────────────────────╯".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_default() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let output = render(&data, true);

        assert!(output.contains("NOVANET BLUEPRINT"), "Should have header");
        assert!(output.contains("STATS"), "Should have stats section");
        assert!(output.contains("REALMS"), "Should have realms section");
        assert!(output.contains("LAYERS"), "Should have layers section");
        assert!(output.contains("TRAITS"), "Should have traits section");
        assert!(output.contains("CORE FLOW"), "Should have flow section");
        assert!(output.contains("ARC FAMILIES"), "Should have arc families");
        assert!(output.contains("VALIDATION"), "Should have validation");
        assert!(output.contains("--view="), "Should have views hint");
    }
}
