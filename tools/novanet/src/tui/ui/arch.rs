//! Architecture panel rendering for TUI.
//!
//! Displays contextual ASCII ER diagrams for key node classes.
//! v0.12.5: Part of the new 4-panel wide layout.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use super::super::app::{App, Focus};
use super::super::data::TreeItem;
use super::{COLOR_UNFOCUSED_BORDER, STYLE_DIM, STYLE_HINT, STYLE_INFO, STYLE_PRIMARY};

// =============================================================================
// ARCHITECTURE DIAGRAMS
// =============================================================================

/// Get architecture diagram for a class (if available).
/// Returns (diagram_lines, adr_id).
fn get_architecture_diagram(class_name: &str) -> Option<(Vec<&'static str>, &'static str)> {
    match class_name {
        "Page" => Some((PAGE_DIAGRAM.to_vec(), "ADR-028")),
        "Entity" => Some((ENTITY_DIAGRAM.to_vec(), "ADR-028")),
        "Block" => Some((BLOCK_DIAGRAM.to_vec(), "ADR-028")),
        "Brand" => Some((BRAND_DIAGRAM.to_vec(), "ADR-028")),
        "Locale" => Some((LOCALE_DIAGRAM.to_vec(), "ADR-020")),
        "Project" => Some((PROJECT_DIAGRAM.to_vec(), "ADR-028")),
        "EntityNative" => Some((ENTITY_NATIVE_DIAGRAM.to_vec(), "ADR-028")),
        "PageNative" => Some((PAGE_NATIVE_DIAGRAM.to_vec(), "ADR-028")),
        _ => None,
    }
}

// -----------------------------------------------------------------------------
// Diagram constants
// -----------------------------------------------------------------------------

const PAGE_DIAGRAM: &[&str] = &[
    "      Project",
    "        \u{2502}",
    "        \u{2502}[:HAS_PAGE]",
    "        \u{25bc}",
    "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Page \u{2550}\u{2550}[:REPRESENTS]\u{2550}\u{25b6} \u{2502}",
    "\u{2502}  \u{2502}     (1:1)     Entity \u{2502}",
    "\u{2502}  \u{2502}                \u{2502}   \u{2502}",
    "\u{2502}  \u{2502}[:HAS_BLOCK]    \u{2502}[:HAS_NATIVE]",
    "\u{2502}  \u{2502}  {order}       \u{2502}   \u{2502}",
    "\u{2502}  \u{25bc}                \u{25bc}   \u{2502}",
    "\u{2502} Block        EntityNative",
    "\u{2502}  \u{2502}                    \u{2502}",
    "\u{2502}  \u{2514}\u{2500}[:USES_ENTITY]\u{2500}\u{25b6}  \u{2502}",
    "\u{2502}             Entity     \u{2502}",
    "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
];

const ENTITY_DIAGRAM: &[&str] = &[
    "     Project",
    "        \u{2502}",
    "        \u{2502}[:HAS_ENTITY]",
    "        \u{25bc}",
    "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Entity \u{2500}[:BELONGS_TO]\u{2500}\u{25b6}",
    "\u{2502}   \u{2502}      EntityCategory \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}[:HAS_NATIVE]       \u{2502}",
    "\u{2502}   \u{25bc}                    \u{2502}",
    "\u{2502} EntityNative           \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}[:FOR_LOCALE]       \u{2502}",
    "\u{2502}   \u{25bc}                    \u{2502}",
    "\u{2502} Locale                 \u{2502}",
    "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
];

const BLOCK_DIAGRAM: &[&str] = &[
    "      Page",
    "        \u{2502}",
    "        \u{2502}[:HAS_BLOCK {order}]",
    "        \u{25bc}",
    "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Block \u{2500}[:OF_TYPE]\u{2500}\u{2500}\u{2500}\u{25b6} \u{2502}",
    "\u{2502}   \u{2502}         BlockType   \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}[:HAS_INSTRUCTION]  \u{2502}",
    "\u{2502}   \u{25bc}                    \u{2502}",
    "\u{2502} BlockInstruction       \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}[:USES_ENTITY]      \u{2502}",
    "\u{2502}   \u{25bc}                    \u{2502}",
    "\u{2502} Entity (via @ refs)    \u{2502}",
    "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
];

const BRAND_DIAGRAM: &[&str] = &[
    "     Project",
    "        \u{2502}",
    "        \u{2502}[:HAS_BRAND]",
    "        \u{25bc}",
    "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Brand \u{2500}[:HAS_DESIGN]\u{2500}\u{25b6} \u{2502}",
    "\u{2502}   \u{2502}       BrandDesign   \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}\u{2500}[:HAS_PRINCIPLES]\u{2500}\u{25b6}",
    "\u{2502}   \u{2502}      BrandPrinciples",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}\u{2500}[:HAS_PROMPT_STYLE]\u{25b6}",
    "\u{2502}   \u{2502}         PromptStyle \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2514}\u{2500}[:TARGETS_PERSONA]\u{2500}\u{25b6}",
    "\u{2502}        AudiencePersona  \u{2502}",
    "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
];

const LOCALE_DIAGRAM: &[&str] = &[
    "   EntityNative",
    "        \u{2502}",
    "        \u{2502}[:FOR_LOCALE]",
    "        \u{25bc}",
    "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Locale \u{2500}[:HAS_VOICE]\u{2500}\u{25b6} \u{2502}",
    "\u{2502}   \u{2502}       LocaleVoice   \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}\u{2500}[:HAS_CULTURE]\u{2500}\u{2500}\u{25b6}  \u{2502}",
    "\u{2502}   \u{2502}       CultureSet    \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}\u{2500}[:HAS_TERMS]\u{2500}\u{2500}\u{2500}\u{2500}\u{25b6}  \u{2502}",
    "\u{2502}   \u{2502}       TermSet       \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2514}\u{2500}[:HAS_EXPRESSIONS]\u{2500}\u{25b6}",
    "\u{2502}       ExpressionSet     \u{2502}",
    "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
];

const PROJECT_DIAGRAM: &[&str] = &[
    "     OrgConfig",
    "        \u{2502}",
    "        \u{2502}[:HAS_PROJECT]",
    "        \u{25bc}",
    "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Project \u{2500}[:HAS_PAGE]\u{2500}\u{25b6} \u{2502}",
    "\u{2502}   \u{2502}           Page      \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}\u{2500}[:HAS_ENTITY]\u{2500}\u{2500}\u{25b6}   \u{2502}",
    "\u{2502}   \u{2502}           Entity    \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}\u{2500}[:HAS_BRAND]\u{2500}\u{2500}\u{2500}\u{25b6}   \u{2502}",
    "\u{2502}   \u{2502}           Brand     \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2514}\u{2500}[:HAS_NATIVE]\u{2500}\u{2500}\u{2500}\u{25b6}   \u{2502}",
    "\u{2502}         ProjectNative   \u{2502}",
    "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
];

const ENTITY_NATIVE_DIAGRAM: &[&str] = &[
    "      Entity",
    "        \u{2502}",
    "        \u{2502}[:HAS_NATIVE]",
    "        \u{25bc}",
    "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
    "\u{2502}                        \u{2502}",
    "\u{2502} EntityNative           \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}[:FOR_LOCALE]       \u{2502}",
    "\u{2502}   \u{25bc}                    \u{2502}",
    "\u{2502} Locale                 \u{2502}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Key format:            \u{2502}",
    "\u{2502} entity:{key}@{locale}  \u{2502}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Example:               \u{2502}",
    "\u{2502} entity:qr-gen@fr-FR    \u{2502}",
    "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
];

const PAGE_NATIVE_DIAGRAM: &[&str] = &[
    "      Page",
    "        \u{2502}",
    "        \u{2502}[:HAS_NATIVE]",
    "        \u{25bc}",
    "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}",
    "\u{2502}                        \u{2502}",
    "\u{2502} PageNative             \u{2502}",
    "\u{2502}   \u{2502}                    \u{2502}",
    "\u{2502}   \u{2502}[:ASSEMBLES]        \u{2502}",
    "\u{2502}   \u{25bc}                    \u{2502}",
    "\u{2502} BlockNative[]          \u{2502}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Key format:            \u{2502}",
    "\u{2502} page:{key}@{locale}    \u{2502}",
    "\u{2502}                        \u{2502}",
    "\u{2502} Example:               \u{2502}",
    "\u{2502} page:homepage@ja-JP    \u{2502}",
    "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}",
];

const NO_DIAGRAM_MESSAGE: &[&str] = &[
    "",
    "  No architecture diagram",
    "  for this class.",
    "",
    "  Diagrams available for:",
    "  \u{2022} Page",
    "  \u{2022} Entity",
    "  \u{2022} Block",
    "  \u{2022} Brand",
    "  \u{2022} Locale",
    "  \u{2022} Project",
    "  \u{2022} EntityNative",
    "  \u{2022} PageNative",
];

// =============================================================================
// RENDER FUNCTION
// =============================================================================

/// Architecture panel: Displays ASCII ER diagrams for key classes.
///
/// Shows contextual architecture diagrams when a Class node is selected.
/// Includes a hint to jump to the related ADR in Nexus mode.
pub fn render_arch_panel(f: &mut Frame, area: Rect, app: &App) {
    let focused = app.focus == Focus::Graph; // Shares focus with graph panel
    let border_color = if focused {
        Color::Rgb(100, 140, 180)
    } else {
        COLOR_UNFOCUSED_BORDER
    };

    // Get current class name (if a Class is selected)
    let (class_name, diagram_data) = match app.current_item() {
        Some(TreeItem::Class(_, _, info)) => {
            let name = info.key.as_str();
            (Some(name), get_architecture_diagram(name))
        }
        _ => (None, None),
    };

    // Build title
    let title = if let Some(name) = class_name {
        format!(" Architecture: {} ", name)
    } else {
        " Architecture ".to_string()
    };

    let block = Block::default()
        .title(Span::styled(title, Style::default().fg(border_color)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    if let Some((diagram, adr_id)) = diagram_data {
        // Render the diagram
        for line in diagram {
            lines.push(Line::from(Span::styled(*line, STYLE_PRIMARY)));
        }

        // Add ADR hint at the bottom
        lines.push(Line::from(Span::raw("")));
        lines.push(Line::from(vec![
            Span::styled("[r] ", STYLE_HINT),
            Span::styled(format!("Jump to {}", adr_id), STYLE_INFO),
        ]));
    } else {
        // No diagram available - show helpful message
        for line in NO_DIAGRAM_MESSAGE {
            lines.push(Line::from(Span::styled(*line, STYLE_DIM)));
        }
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}
