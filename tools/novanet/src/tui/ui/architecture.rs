//! Architecture panel rendering for TUI.
//!
//! Displays contextual architecture diagrams for key classes (Page, Entity, Block, etc.)
//! based on ADR-028 and related architecture decisions.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use super::super::app::{App, InfoBox};
use super::super::data::{TreeItem, get_architecture_diagram};
use super::STYLE_DIM;

// =============================================================================
// BOX VISUAL STATES (matching yaml_panel.rs)
// =============================================================================

/// Unfocused: Nord Polar Night (dim)
const BOX_BORDER_UNFOCUSED: Color = Color::Rgb(59, 66, 82); // #3B4252

/// Focused: Nord Snow Storm (subtle highlight) - panel has focus but not this box
const BOX_BORDER_FOCUSED: Color = Color::Rgb(129, 161, 193); // #81A1C1

/// Selected: Solarized Cyan (bright, active) - this specific box is selected
const BOX_BORDER_SELECTED: Color = Color::Rgb(42, 161, 152); // #2AA198

// =============================================================================
// ARCHITECTURE PANEL
// =============================================================================

/// Architecture panel: Displays ER diagrams for key classes.
///
/// Shows architecture diagrams from ADR-028 when a supported class is selected:
/// - Page, Entity, Block, Brand, Locale, Project
///
/// For other selections, shows a hint message.
///
/// v0.13.0: Uses InfoBox::Architecture for independent selection state.
pub fn render_architecture_panel(f: &mut Frame, area: Rect, app: &App) {
    // Determine if this box is selected or just focused
    let selected = app.selected_box == InfoBox::Architecture;
    let panel_focused = app.selected_box.is_right_panel();

    let border_color = if selected {
        BOX_BORDER_SELECTED
    } else if panel_focused {
        BOX_BORDER_FOCUSED
    } else {
        BOX_BORDER_UNFOCUSED
    };

    // Get current class name if applicable
    let class_name = match app.current_item() {
        Some(TreeItem::Class(_, _, info)) => Some(info.key.as_str()),
        Some(TreeItem::Instance(_, _, class_info, _)) => Some(class_info.key.as_str()),
        _ => None,
    };

    // Try to get diagram for the current class
    let diagram = class_name.and_then(get_architecture_diagram);

    // Build enhanced title with selection indicator (matches SOURCE/DIAGRAM pattern)
    let adr_badge = diagram.as_ref().map(|d| d.adr_id.as_str()).unwrap_or("—");

    let title = if selected {
        Line::from(vec![
            Span::styled(
                " \u{25B6} ",  // ▶
                Style::default()
                    .fg(BOX_BORDER_SELECTED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "ARCHITECTURE ",
                Style::default()
                    .fg(BOX_BORDER_SELECTED)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("[{}] ", adr_badge),
                Style::default().fg(Color::Rgb(180, 142, 173)), // Nord Aurora Purple
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(" ARCHITECTURE ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("[{}] ", adr_badge),
                Style::default().fg(Color::DarkGray),
            ),
        ])
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line<'static>> = Vec::new();

    if let Some(diag) = diagram {
        // Render the diagram
        for diagram_line in &diag.diagram {
            let styled_span: Span<'static> = Span::styled(
                diagram_line.clone(),
                Style::default().fg(Color::Rgb(150, 180, 200)),
            );
            lines.push(Line::from(styled_span));
        }

        // Add ADR reference at bottom
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("[r] ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("Jump to {} in Nexus", diag.adr_id),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    } else if let Some(name) = class_name {
        // No diagram for this class
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("No architecture diagram for '{}'", name),
            STYLE_DIM,
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Diagrams available for:",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            "  Page, Entity, Block, Brand, Locale, Project",
            Style::default().fg(Color::Cyan),
        )));
    } else {
        // No class selected
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Select a Class to view architecture",
            STYLE_DIM,
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Key classes with diagrams:",
            Style::default().fg(Color::DarkGray),
        )));

        let classes = ["Page", "Entity", "Block", "Brand", "Locale", "Project"];
        for class in classes {
            lines.push(Line::from(vec![
                Span::styled("  ◆ ", Style::default().fg(Color::Cyan)),
                Span::styled(class, Style::default().fg(Color::White)),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use crate::tui::data::get_architecture_diagram;

    #[test]
    fn test_page_diagram_exists() {
        let diag = get_architecture_diagram("Page");
        assert!(diag.is_some());
        let d = diag.unwrap();
        assert_eq!(d.class_name, "Page");
        assert_eq!(d.adr_id, "ADR-028");
        assert!(!d.diagram.is_empty());
    }

    #[test]
    fn test_entity_diagram_exists() {
        let diag = get_architecture_diagram("Entity");
        assert!(diag.is_some());
    }

    #[test]
    fn test_block_diagram_exists() {
        let diag = get_architecture_diagram("Block");
        assert!(diag.is_some());
    }

    #[test]
    fn test_brand_diagram_exists() {
        let diag = get_architecture_diagram("Brand");
        assert!(diag.is_some());
    }

    #[test]
    fn test_locale_diagram_exists() {
        let diag = get_architecture_diagram("Locale");
        assert!(diag.is_some());
    }

    #[test]
    fn test_project_diagram_exists() {
        let diag = get_architecture_diagram("Project");
        assert!(diag.is_some());
    }

    #[test]
    fn test_unknown_class_no_diagram() {
        let diag = get_architecture_diagram("UnknownClass");
        assert!(diag.is_none());
    }
}
