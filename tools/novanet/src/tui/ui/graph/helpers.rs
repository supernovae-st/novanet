//! Utility functions for graph panel rendering.
//!
//! Short name lookups, classification badges, and arc direction grouping.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use super::super::super::app::App;
use super::super::super::theme;
use super::super::{STYLE_DIM, STYLE_SUCCESS};
use super::ARC_SEPARATOR;

/// Render arcs grouped by direction (OUTGOING, INCOMING) with classification badges.
/// Format: -> ARC_NAME -> [realm/layer] TargetClass [fam]
pub(crate) fn render_arcs_by_direction(
    lines: &mut Vec<Line>,
    arcs: &super::super::super::data::ClassArcsData,
    app: &App,
    theme: &theme::Theme,
    dim: &Style,
) {
    if arcs.outgoing.is_empty() && arcs.incoming.is_empty() {
        lines.push(Line::from(Span::styled(
            "  No arc relationships defined for this Node Class",
            STYLE_DIM,
        )));
        return;
    }

    // === OUTGOING SECTION ===
    if !arcs.outgoing.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  ━▶ OUTGOING ({}) ", arcs.outgoing.len()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(ARC_SEPARATOR, *dim)));

        for arc in &arcs.outgoing {
            let family_color = theme.arc_family_color(&arc.family);
            let fam_short = family_short(&arc.family);

            // Look up target class info for classification badge
            let badge = class_badge(&arc.other_class, app, theme);

            lines.push(Line::from(vec![
                Span::styled(
                    "    → ",
                    Style::default()
                        .fg(family_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(arc.arc_key.clone(), Style::default().fg(family_color)),
                Span::styled(" → ", *dim),
                badge,
                Span::styled(arc.other_class.clone(), STYLE_SUCCESS),
                Span::styled(format!(" [{}]", fam_short), *dim),
            ]));
        }
        lines.push(Line::from(Span::raw("")));
    }

    // === INCOMING SECTION ===
    if !arcs.incoming.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  ◀━ INCOMING ({}) ", arcs.incoming.len()),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(ARC_SEPARATOR, *dim)));

        for arc in &arcs.incoming {
            let family_color = theme.arc_family_color(&arc.family);
            let fam_short = family_short(&arc.family);

            // Look up source class info for classification badge
            let badge = class_badge(&arc.other_class, app, theme);

            lines.push(Line::from(vec![
                Span::styled(
                    "    ← ",
                    Style::default()
                        .fg(family_color)
                        .add_modifier(Modifier::BOLD),
                ),
                badge,
                Span::styled(arc.other_class.clone(), STYLE_SUCCESS),
                Span::styled(" ← ", *dim),
                Span::styled(arc.arc_key.clone(), Style::default().fg(family_color)),
                Span::styled(format!(" [{}]", fam_short), *dim),
            ]));
        }
    }
}

/// Get short family name (3 chars).
pub(crate) fn family_short(family: &str) -> &'static str {
    match family {
        "ownership" => "own",
        "localization" => "loc",
        "semantic" => "sem",
        "generation" => "gen",
        "mining" => "min",
        _ => "???",
    }
}

/// Get short layer name (3 chars).
pub(crate) fn layer_short(layer: &str) -> &'static str {
    match layer {
        "config" => "cfg",
        "locale" => "loc",
        "geography" => "geo",
        "knowledge" => "kno",
        "foundation" => "fnd",
        "structure" => "str",
        "semantic" => "sem",
        "instruction" => "ins",
        "output" => "out",
        _ => "???",
    }
}

/// Get layer icon (Unicode symbol).
pub(crate) fn layer_icon(layer_key: &str) -> &'static str {
    match layer_key {
        "config" => "⚙",
        "locale" => "🌐",
        "geography" => "📍",
        "knowledge" => "📚",
        "foundation" => "🏛",
        "structure" => "🏗",
        "semantic" => "💎",
        "instruction" => "📝",
        "output" => "📤",
        _ => "○",
    }
}

/// Build classification badge Span: [realm/layer] layer_icon
/// Example: [org/fnd] ■
pub(crate) fn class_badge(class_key: &str, app: &App, theme: &theme::Theme) -> Span<'static> {
    if let Some((realm, layer, _class_info)) = app.tree.find_class(class_key) {
        let realm_short = if realm.key == "shared" { "shd" } else { "org" };
        let layer_s = layer_short(&layer.key);
        let icon = layer_icon(&layer.key);

        let badge = format!("[{}/{}] {} ", realm_short, layer_s, icon);
        Span::styled(badge, Style::default().fg(theme.layer_color(&layer.key)))
    } else {
        // Class not found in tree (external or unknown)
        Span::styled("[???] ○ ", Style::default().fg(Color::DarkGray))
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::super::super::STYLE_PALETTE_DIM;
    use super::*;
    use crate::tui::data::{ClassArcsData, ClassInfo, Neo4jArc, TaxonomyTree};
    use crate::tui::testing::{
        create_test_class, create_test_layer, create_test_realm, create_test_theme,
        create_tree_with_realms,
    };

    fn create_class_arcs_data(
        class_label: &str,
        incoming: Vec<Neo4jArc>,
        outgoing: Vec<Neo4jArc>,
    ) -> ClassArcsData {
        ClassArcsData {
            class_label: class_label.to_string(),
            realm: "org".to_string(),
            layer: "structure".to_string(),
            incoming,
            outgoing,
        }
    }

    fn create_neo4j_arc(arc_key: &str, other_class: &str, family: &str) -> Neo4jArc {
        Neo4jArc {
            arc_key: arc_key.to_string(),
            other_class: other_class.to_string(),
            family: family.to_string(),
        }
    }

    fn create_tree_for_arcs(class_names: &[&str]) -> TaxonomyTree {
        let classes: Vec<ClassInfo> = class_names
            .iter()
            .map(|name| create_test_class(name))
            .collect();
        let layer = create_test_layer("structure", classes);
        let realm = create_test_realm("org", vec![layer]);
        create_tree_with_realms(vec![realm])
    }

    fn create_test_app_with_tree(tree: TaxonomyTree) -> App {
        let mut app = App::new(TaxonomyTree::mock_for_testing(), String::new());
        app.tree = tree;
        app.theme = create_test_theme();
        app
    }

    #[test]
    fn test_render_arcs_by_direction_empty() {
        let tree = create_tree_for_arcs(&["Page"]);
        let app = create_test_app_with_tree(tree);
        let theme = create_test_theme();
        let dim = STYLE_PALETTE_DIM;
        let arcs = create_class_arcs_data("Page", Vec::new(), Vec::new());

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_direction(&mut lines, &arcs, &app, &theme, &dim);

        assert_eq!(lines.len(), 1, "empty arcs should produce 1 line");

        let line_content: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            line_content.contains("No arc relationships"),
            "should contain 'No arc relationships' message, got: {}",
            line_content
        );
    }

    #[test]
    fn test_render_arcs_by_direction_outgoing_only() {
        let tree = create_tree_for_arcs(&["Entity", "Block"]);
        let app = create_test_app_with_tree(tree);
        let theme = create_test_theme();
        let dim = STYLE_PALETTE_DIM;

        let outgoing = vec![
            create_neo4j_arc("USES_ENTITY", "Entity", "semantic"),
            create_neo4j_arc("USES_BLOCK", "Block", "semantic"),
        ];
        let arcs = create_class_arcs_data("Page", Vec::new(), outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_direction(&mut lines, &arcs, &app, &theme, &dim);

        assert!(lines.len() >= 4, "should have header + separator + arcs");

        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("OUTGOING"),
            "should have OUTGOING header"
        );
        assert!(
            all_content.contains("USES_ENTITY"),
            "should contain arc key"
        );
        assert!(
            all_content.contains("Entity"),
            "should contain target class"
        );
        assert!(
            all_content.contains("→"),
            "should contain direction indicator"
        );
        assert!(all_content.contains("[sem]"), "should contain family short");
    }

    #[test]
    fn test_render_arcs_by_direction_both_directions() {
        let tree = create_tree_for_arcs(&["Project", "Entity"]);
        let app = create_test_app_with_tree(tree);
        let theme = create_test_theme();
        let dim = STYLE_PALETTE_DIM;

        let incoming = vec![create_neo4j_arc("BELONGS_TO", "Project", "ownership")];
        let outgoing = vec![create_neo4j_arc("USES_ENTITY", "Entity", "semantic")];
        let arcs = create_class_arcs_data("Page", incoming, outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_direction(&mut lines, &arcs, &app, &theme, &dim);

        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("OUTGOING"),
            "should have OUTGOING header"
        );
        assert!(
            all_content.contains("INCOMING"),
            "should have INCOMING header"
        );
        assert!(
            all_content.contains("BELONGS_TO"),
            "should contain incoming arc"
        );
        assert!(
            all_content.contains("USES_ENTITY"),
            "should contain outgoing arc"
        );
    }

    #[test]
    fn test_render_arcs_by_direction_classification_badges() {
        let tree = create_tree_for_arcs(&["Entity", "Page", "Block"]);
        let app = create_test_app_with_tree(tree);
        let theme = create_test_theme();
        let dim = STYLE_PALETTE_DIM;

        let incoming = vec![
            create_neo4j_arc("USED_BY_PAGE", "Page", "semantic"),
            create_neo4j_arc("USED_BY_BLOCK", "Block", "semantic"),
        ];
        let outgoing = vec![create_neo4j_arc("USES_ENTITY", "Entity", "semantic")];
        let arcs = create_class_arcs_data("Class", incoming, outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_direction(&mut lines, &arcs, &app, &theme, &dim);

        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("[org/str]"),
            "should contain realm/layer badge"
        );
        assert!(
            all_content.contains("←"),
            "should contain incoming direction"
        );
        assert!(
            all_content.contains("→"),
            "should contain outgoing direction"
        );
    }

    #[test]
    fn test_render_arcs_by_direction_with_counts() {
        let tree = create_tree_for_arcs(&["Project", "Page"]);
        let app = create_test_app_with_tree(tree);
        let theme = create_test_theme();
        let dim = STYLE_PALETTE_DIM;

        let incoming = vec![create_neo4j_arc("BELONGS_TO", "Project", "ownership")];
        let outgoing = vec![create_neo4j_arc("HAS_PAGE", "Page", "ownership")];
        let arcs = create_class_arcs_data("Class", incoming, outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_direction(&mut lines, &arcs, &app, &theme, &dim);

        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("OUTGOING (1)"),
            "should show outgoing count"
        );
        assert!(
            all_content.contains("INCOMING (1)"),
            "should show incoming count"
        );
    }

    #[test]
    fn test_render_arcs_by_direction_all_five_families() {
        let theme = create_test_theme();
        let dim = STYLE_PALETTE_DIM;

        let classes = vec![
            create_test_class("Project"),
            create_test_class("EntityNative"),
            create_test_class("Entity"),
            create_test_class("Block"),
            create_test_class("Source"),
        ];
        let layer = create_test_layer("semantic", classes);
        let realm = create_test_realm("org", vec![layer]);
        let tree = create_tree_with_realms(vec![realm]);
        let app = create_test_app_with_tree(tree);

        let outgoing = vec![
            create_neo4j_arc("BELONGS_TO", "Project", "ownership"),
            create_neo4j_arc("LOCALIZES", "EntityNative", "localization"),
            create_neo4j_arc("USES_ENTITY", "Entity", "semantic"),
            create_neo4j_arc("GENERATES", "Block", "generation"),
            create_neo4j_arc("MINES_DATA", "Source", "mining"),
        ];
        let arcs = create_class_arcs_data("Class", Vec::new(), outgoing);

        let mut lines: Vec<Line> = Vec::new();
        render_arcs_by_direction(&mut lines, &arcs, &app, &theme, &dim);

        assert_eq!(
            lines.len(),
            8,
            "should have header + separator + 5 arcs + empty"
        );

        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("OUTGOING"),
            "should have OUTGOING header"
        );
        assert!(
            all_content.contains("BELONGS_TO"),
            "should have ownership arc"
        );
        assert!(
            all_content.contains("LOCALIZES"),
            "should have localization arc"
        );
        assert!(
            all_content.contains("USES_ENTITY"),
            "should have semantic arc"
        );
        assert!(
            all_content.contains("GENERATES"),
            "should have generation arc"
        );
        assert!(all_content.contains("MINES_DATA"), "should have mining arc");
    }
}
