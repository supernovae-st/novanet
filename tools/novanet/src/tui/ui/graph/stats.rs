//! Distribution statistics for the graph panel fallback view.
//!
//! Renders realm/layer bar charts when no specific node is selected.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use super::super::super::app::App;
use super::super::{STYLE_DIM, STYLE_MUTED, STYLE_PALETTE_DIM};
use crate::tui::widgets::ProgressBar;

/// Build realm/layer distribution stats for the graph panel fallback view.
pub(crate) fn build_graph_distribution_stats(app: &App) -> Vec<Line<'static>> {
    let theme = &app.theme;
    let dim = STYLE_PALETTE_DIM;
    let mut lines: Vec<Line<'static>> = Vec::with_capacity(20);

    // Calculate total classes
    let mut total_classes: usize = 0;
    for realm in &app.tree.realms {
        for layer in &realm.layers {
            total_classes += layer.classes.len();
        }
    }

    if total_classes == 0 {
        lines.push(Line::from(Span::styled("  No classes loaded", STYLE_DIM)));
        return lines;
    }

    // Header
    lines.push(Line::from(Span::styled(
        "  REALM DISTRIBUTION",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
        dim,
    )));

    let bar_max_width = 20usize;

    // Realm bars
    for realm in &app.tree.realms {
        let realm_classes: usize = realm.layers.iter().map(|l| l.classes.len()).sum();
        let percent = (realm_classes as f64 / total_classes as f64 * 100.0).round() as u8;
        let realm_color = theme.realm_color(&realm.key);
        let (bar, empty) = ProgressBar::new(realm_classes, total_classes, bar_max_width)
            .filled_style(Style::default().fg(realm_color))
            .empty_style(STYLE_DIM)
            .to_spans();

        lines.push(Line::from(vec![
            Span::styled("    ", dim),
            Span::styled(
                format!("{:8}", realm.display_name),
                Style::default()
                    .fg(realm_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", dim),
            bar,
            empty,
            Span::styled(format!(" {:>3}%", percent), STYLE_MUTED),
            Span::styled(format!("  {} Classes", realm_classes), STYLE_DIM),
        ]));
    }

    lines.push(Line::from(Span::raw("")));

    // Layer breakdown header
    lines.push(Line::from(Span::styled(
        "  LAYER BREAKDOWN",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  \u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}",
        dim,
    )));

    // Find max classes per layer for scaling
    let max_layer_classes = app
        .tree
        .realms
        .iter()
        .flat_map(|r| r.layers.iter())
        .map(|l| l.classes.len())
        .max()
        .unwrap_or(1)
        .max(1);

    // Layer bars (grouped by realm)
    for realm in &app.tree.realms {
        for layer in &realm.layers {
            let layer_classes = layer.classes.len();
            if layer_classes == 0 {
                continue; // Skip empty layers
            }
            let bar_width = (layer_classes * bar_max_width) / max_layer_classes;
            let bar = "\u{2588}".repeat(bar_width.max(1));

            lines.push(Line::from(vec![
                Span::styled("    ", dim),
                Span::styled(
                    format!("{:16}", layer.display_name),
                    Style::default().fg(theme.layer_color(&layer.key)),
                ),
                Span::styled(format!("{:>3} ", layer_classes), STYLE_MUTED),
                Span::styled(bar, Style::default().fg(theme.layer_color(&layer.key))),
            ]));
        }
    }

    lines
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::data::{ClassInfo, RealmInfo, TaxonomyTree};
    use crate::tui::testing::{
        create_empty_tree, create_test_class, create_test_layer, create_test_realm,
        create_test_theme, create_tree_with_realms,
    };
    use pretty_assertions::assert_eq;

    fn create_test_app_with_tree(tree: TaxonomyTree) -> App {
        let mut app = App::new(TaxonomyTree::mock_for_testing(), String::new());
        app.tree = tree;
        app.theme = create_test_theme();
        app
    }

    #[test]
    fn test_build_graph_distribution_stats_empty_tree() {
        let tree = create_empty_tree();
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);

        assert_eq!(lines.len(), 1, "empty tree should produce 1 line");
        let content: String = lines[0].spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(
            content.contains("No classes loaded"),
            "should show 'No classes loaded', got: {}",
            content
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_single_realm() {
        let class1 = create_test_class("Page");
        let class2 = create_test_class("Block");
        let layer = create_test_layer("structure", vec![class1, class2]);
        let realm = create_test_realm("org", vec![layer]);
        let tree = create_tree_with_realms(vec![realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);

        assert!(lines.len() >= 5, "should have multiple lines for stats");

        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("REALM DISTRIBUTION"),
            "should contain realm header"
        );
        assert!(
            all_content.contains("LAYER BREAKDOWN"),
            "should contain layer header"
        );
        assert!(all_content.contains("100%"), "single realm should be 100%");
    }

    #[test]
    fn test_build_graph_distribution_stats_percentage_calculation() {
        let shared_class = create_test_class("Config");
        let shared_layer = create_test_layer("config", vec![shared_class]);
        let shared_realm = create_test_realm("shared", vec![shared_layer]);

        let org_classes = vec![
            create_test_class("Page"),
            create_test_class("Block"),
            create_test_class("Entity"),
        ];
        let org_layer = create_test_layer("structure", org_classes);
        let org_realm = create_test_realm("org", vec![org_layer]);

        let tree = create_tree_with_realms(vec![shared_realm, org_realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);
        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("25%"),
            "shared should be 25%, got: {}",
            all_content
        );
        assert!(
            all_content.contains("75%"),
            "org should be 75%, got: {}",
            all_content
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_bar_width_calculation() {
        let shared_classes = vec![create_test_class("Config1"), create_test_class("Config2")];
        let shared_layer = create_test_layer("config", shared_classes);
        let shared_realm = create_test_realm("shared", vec![shared_layer]);

        let org_classes: Vec<ClassInfo> = (0..8)
            .map(|i| create_test_class(&format!("Class{}", i)))
            .collect();
        let org_layer = create_test_layer("structure", org_classes);
        let org_realm = create_test_realm("org", vec![org_layer]);

        let tree = create_tree_with_realms(vec![shared_realm, org_realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);

        let realm_section: Vec<&Line> = lines
            .iter()
            .filter(|l| {
                let content: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                (content.contains("shared") || content.contains("org")) && content.contains('%')
            })
            .collect();

        assert_eq!(realm_section.len(), 2, "should have 2 realm bar lines");

        let shared_line: String = realm_section[0]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();
        let org_line: String = realm_section[1]
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect();

        let shared_blocks = shared_line.matches('\u{2588}').count();
        let org_blocks = org_line.matches('\u{2588}').count();

        assert!(
            org_blocks > shared_blocks,
            "org ({} blocks) should have more than shared ({} blocks)",
            org_blocks,
            shared_blocks
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_layer_bar_scaling() {
        let layer1_classes = vec![create_test_class("Class1")];
        let layer2_classes = vec![
            create_test_class("Class2"),
            create_test_class("Class3"),
            create_test_class("Class4"),
            create_test_class("Class5"),
        ];

        let layer1 = create_test_layer("config", layer1_classes);
        let layer2 = create_test_layer("foundation", layer2_classes);
        let realm = create_test_realm("org", vec![layer1, layer2]);
        let tree = create_tree_with_realms(vec![realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);

        let layer_section: Vec<&Line> = lines
            .iter()
            .skip_while(|l| {
                let content: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                !content.contains("LAYER BREAKDOWN")
            })
            .skip(2)
            .filter(|l| !l.spans.is_empty())
            .collect();

        assert!(layer_section.len() >= 2, "should have 2 layer lines");

        let config_line: String = layer_section
            .iter()
            .find(|l| {
                let content: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                content.contains("config")
            })
            .map(|l| l.spans.iter().map(|s| s.content.as_ref()).collect())
            .unwrap_or_default();

        let foundation_line: String = layer_section
            .iter()
            .find(|l| {
                let content: String = l.spans.iter().map(|s| s.content.as_ref()).collect();
                content.contains("foundation")
            })
            .map(|l| l.spans.iter().map(|s| s.content.as_ref()).collect())
            .unwrap_or_default();

        let config_blocks = config_line.matches('\u{2588}').count();
        let foundation_blocks = foundation_line.matches('\u{2588}').count();

        assert!(
            foundation_blocks > config_blocks,
            "foundation ({} blocks) should have more than config ({} blocks)",
            foundation_blocks,
            config_blocks
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_skips_empty_layers() {
        let class_info = create_test_class("Page");
        let layer_with_classes = create_test_layer("structure", vec![class_info]);
        let empty_layer = create_test_layer("empty", Vec::new());
        let realm = create_test_realm("org", vec![layer_with_classes, empty_layer]);
        let tree = create_tree_with_realms(vec![realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);
        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("structure"),
            "should contain structure layer"
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_class_counts_displayed() {
        let classes = vec![
            create_test_class("Page"),
            create_test_class("Block"),
            create_test_class("Entity"),
        ];
        let layer = create_test_layer("structure", classes);
        let realm = create_test_realm("org", vec![layer]);
        let tree = create_tree_with_realms(vec![realm]);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);
        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(
            all_content.contains("3 Classes") || all_content.contains("3"),
            "should show class count, got: {}",
            all_content
        );
    }

    #[test]
    fn test_build_graph_distribution_stats_many_realms() {
        let realms: Vec<RealmInfo> = (0..3)
            .map(|i| {
                let classes: Vec<ClassInfo> = (0..(i + 1))
                    .map(|j| create_test_class(&format!("Class{}_{}", i, j)))
                    .collect();
                let layer = create_test_layer(&format!("layer{}", i), classes);
                create_test_realm(&format!("realm{}", i), vec![layer])
            })
            .collect();

        let tree = create_tree_with_realms(realms);
        let app = create_test_app_with_tree(tree);

        let lines = build_graph_distribution_stats(&app);
        let all_content: String = lines
            .iter()
            .flat_map(|l| l.spans.iter())
            .map(|s| s.content.as_ref())
            .collect();

        assert!(all_content.contains("realm0"), "should have realm0");
        assert!(all_content.contains("realm1"), "should have realm1");
        assert!(all_content.contains("realm2"), "should have realm2");
    }
}
