//! Intro Tab - "The Big Picture" introduction to NovaNet.
//!
//! This tab is the first stop for newcomers, explaining:
//! - WHY NovaNet exists (translation vs generation)
//! - WHAT it does (Schema vs Instance nodes)
//! - HOW it classifies (Realm, Layer, Trait)

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use super::NexusLocale;
use crate::tui::app::App;
use crate::tui::theme::Theme;

/// Total number of intro pages.
pub const INTRO_PAGES: usize = 3;

/// Render the Intro tab content.
pub fn render_intro_tab(f: &mut Frame, app: &App, area: Rect) {
    let page = app.nexus.intro_page.min(INTRO_PAGES - 1);
    let locale = app.nexus.locale;

    // Render the current page (action bar in nexus/mod.rs handles keybindings)
    match page {
        0 => render_page_1_what_is_novanet(f, &app.theme, locale, area),
        1 => render_page_2_two_types_of_nodes(f, &app.theme, locale, area),
        2 => render_page_3_classification(f, &app.theme, locale, area),
        _ => render_page_1_what_is_novanet(f, &app.theme, locale, area),
    }
}

/// Page 1: What is NovaNet?
fn render_page_1_what_is_novanet(f: &mut Frame, theme: &Theme, locale: NexusLocale, area: Rect) {
    // i18n title
    let title = match locale {
        NexusLocale::En => " WHAT IS NOVANET? ",
        NexusLocale::Fr => " QU'EST-CE QUE NOVANET ? ",
    };

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Content lines
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  NovaNet is a ", Style::default().fg(Color::White)),
            Span::styled(
                "KNOWLEDGE GRAPH",
                Style::default()
                    .fg(theme.trait_color("imported"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " that helps generate website content in 200+ languages.",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Instead of ", Style::default().fg(Color::White)),
            Span::styled(
                "translating",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::CROSSED_OUT),
            ),
            Span::styled(
                " content (which loses cultural nuance), NovaNet ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                "GENERATES",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::styled(
            "  content natively for each locale using local knowledge (vocabulary, culture, style).",
            Style::default().fg(Color::White),
        )]),
        Line::from(""),
        Line::from(Span::styled(
            "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  REAL EXAMPLE:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  You define ", Style::default().fg(Color::White)),
            Span::styled(
                "ONE",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Entity:", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "    Entity:                                                   ",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(vec![
            Span::styled("      key: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"qr-code\"", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("      display_name: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"QR Code\"", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("      description: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "\"Two-dimensional barcode...\"",
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  NovaNet generates ", Style::default().fg(Color::White)),
            Span::styled(
                "NATIVE",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " content for each locale:",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                "French (fr-FR)",
                Style::default()
                    .fg(theme.trait_color("authored"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("                  ", Style::default()),
            Span::styled(
                "Japanese (ja-JP)",
                Style::default()
                    .fg(theme.trait_color("authored"))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  display_name: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"QR Code\"", Style::default().fg(Color::Green)),
            Span::styled(
                "          display_name: ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("\"QRコード\"", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("  description: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"Code-barres 2D\"", Style::default().fg(Color::Green)),
            Span::styled("    description: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"2次元バーコード\"", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                "Arabic (ar-AE)",
                Style::default()
                    .fg(theme.trait_color("authored"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("                   ", Style::default()),
            Span::styled(
                "Afrikaans (af-ZA)",
                Style::default()
                    .fg(theme.trait_color("authored"))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  display_name: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"رمز QR\"", Style::default().fg(Color::Green)),
            Span::styled(
                "           display_name: ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("\"QR-kode\"", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("  text_direction: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"rtl\" ←", Style::default().fg(Color::Magenta)),
        ]),
    ];

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Page 2: The Two Types of Nodes
fn render_page_2_two_types_of_nodes(f: &mut Frame, theme: &Theme, locale: NexusLocale, area: Rect) {
    // i18n title
    let title = match locale {
        NexusLocale::En => " THE TWO TYPES OF NODES ",
        NexusLocale::Fr => " LES DEUX TYPES DE NOEUDS ",
    };

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  NovaNet has TWO types of nodes. Understanding this is the key to everything.",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                "SCHEMA CLASSES (61 total)",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("                ", Style::default()),
            Span::styled(
                "DATA NODES (200,000+)",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                "══════════════════════",
                Style::default().fg(Color::Magenta),
            ),
            Span::styled("                ", Style::default()),
            Span::styled("══════════════════════", Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Define WHAT types exist",
                Style::default().fg(Color::White),
            ),
            Span::styled("               ", Style::default()),
            Span::styled(
                "Actual content instances",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Think of it like:", Style::default().fg(Color::DarkGray)),
            Span::styled("                 ", Style::default()),
            Span::styled("Think of it like:", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(
                "  \"What IS a Locale?\"",
                Style::default().fg(Color::Magenta),
            ),
            Span::styled("                 ", Style::default()),
            Span::styled(
                "\"French (France) is a Locale\"",
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  ┌─────────────────────────────────────────────────────────────────────────┐",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Class: Locale",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("              ", Style::default()),
            Span::styled("\"fr-FR is a\"", Style::default().fg(Color::Yellow)),
            Span::styled("        ", Style::default()),
            Span::styled(
                "Locale: fr-FR",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("        │", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("────────────────", Style::default().fg(Color::Magenta)),
            Span::styled("              ", Style::default()),
            Span::styled("\"Locale\"", Style::default().fg(Color::Yellow)),
            Span::styled("             ", Style::default()),
            Span::styled("─────────────────", Style::default().fg(Color::Cyan)),
            Span::styled("   │", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("realm: ", Style::default().fg(Color::DarkGray)),
            Span::styled("shared", Style::default().fg(theme.realm_color("shared"))),
            Span::styled("         ", Style::default()),
            Span::styled(" ◄───────────── ", Style::default().fg(Color::Yellow)),
            Span::styled("display_name: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"French\"", Style::default().fg(Color::Green)),
            Span::styled("    │", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("layer: ", Style::default().fg(Color::DarkGray)),
            Span::styled("config", Style::default().fg(Color::Blue)),
            Span::styled("                          ", Style::default()),
            Span::styled("language_code: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"fr\"", Style::default().fg(Color::Green)),
            Span::styled("      │", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("trait: ", Style::default().fg(Color::DarkGray)),
            Span::styled("defined", Style::default().fg(theme.trait_color("defined"))),
            Span::styled("                       ", Style::default()),
            Span::styled("country_code: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"FR\"", Style::default().fg(Color::Green)),
            Span::styled("       │", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("(definition)", Style::default().fg(Color::DarkGray)),
            Span::styled("                          ", Style::default()),
            Span::styled("script: ", Style::default().fg(Color::DarkGray)),
            Span::styled("\"latin\"", Style::default().fg(Color::Green)),
            Span::styled("           │", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(Span::styled(
            "  └─────────────────────────────────────────────────────────────────────────┘",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  The relationship is called: ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                "OF_CLASS",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![Span::styled(
            "  (:Locale {key: 'fr-FR'})-[:OF_CLASS]->(:Class {label: 'Locale'})",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Page 3: Classification (Realm, Layer, Trait)
fn render_page_3_classification(f: &mut Frame, theme: &Theme, locale: NexusLocale, area: Rect) {
    // i18n title
    let title = match locale {
        NexusLocale::En => " HOW NODES ARE CLASSIFIED ",
        NexusLocale::Fr => " CLASSIFICATION DES NOEUDS ",
    };

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Every Class (type definition) has 3 classification properties:",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        // REALM
        Line::from(Span::styled(
            "  ┌─ REALM: Where does it live? ────────────────────────────────────────────┐",
            Style::default().fg(theme.realm_color("shared")),
        )),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(theme.realm_color("shared"))),
            Span::styled(
                "◉ shared",
                Style::default()
                    .fg(theme.realm_color("shared"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (40 nodes)", Style::default().fg(Color::DarkGray)),
            Span::styled("              ", Style::default()),
            Span::styled(
                "◎ org",
                Style::default()
                    .fg(theme.realm_color("org"))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" (21 nodes)", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "                   │",
                Style::default().fg(theme.realm_color("shared")),
            ),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(theme.realm_color("shared"))),
            Span::styled("Universal knowledge", Style::default().fg(Color::White)),
            Span::styled("               ", Style::default()),
            Span::styled("Organization-specific", Style::default().fg(Color::White)),
            Span::styled(
                "             │",
                Style::default().fg(theme.realm_color("shared")),
            ),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(theme.realm_color("shared"))),
            Span::styled("READ-ONLY", Style::default().fg(Color::Red)),
            Span::styled("                          ", Style::default()),
            Span::styled("Your business content", Style::default().fg(Color::White)),
            Span::styled(
                "             │",
                Style::default().fg(theme.realm_color("shared")),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  │  Examples: ",
                Style::default().fg(theme.realm_color("shared")),
            ),
            Span::styled(
                "Locale, Term, Culture",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("    ", Style::default()),
            Span::styled(
                "Examples: ",
                Style::default().fg(theme.realm_color("shared")),
            ),
            Span::styled("Entity, Page", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "            │",
                Style::default().fg(theme.realm_color("shared")),
            ),
        ]),
        Line::from(Span::styled(
            "  └──────────────────────────────────────────────────────────────────────────┘",
            Style::default().fg(theme.realm_color("shared")),
        )),
        Line::from(""),
        // LAYER
        Line::from(Span::styled(
            "  ┌─ LAYER: What is its function? ──────────────────────────────────────────┐",
            Style::default().fg(Color::Blue),
        )),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::Blue)),
            Span::styled(
                "shared/config",
                Style::default().fg(theme.realm_color("shared")),
            ),
            Span::styled(
                " → Locale, EntityCategory       ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("(definitions)", Style::default().fg(Color::DarkGray)),
            Span::styled("      │", Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::Blue)),
            Span::styled(
                "shared/knowledge",
                Style::default().fg(theme.realm_color("shared")),
            ),
            Span::styled(
                " → Term, Expression, Culture  ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("(locale expertise)", Style::default().fg(Color::DarkGray)),
            Span::styled(" │", Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::Blue)),
            Span::styled(
                "org/semantic",
                Style::default().fg(theme.realm_color("org")),
            ),
            Span::styled(
                " → Entity, EntityNative          ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("(your content)", Style::default().fg(Color::DarkGray)),
            Span::styled("     │", Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::Blue)),
            Span::styled(
                "org/structure",
                Style::default().fg(theme.realm_color("org")),
            ),
            Span::styled(
                " → Page, Block                    ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("(website)", Style::default().fg(Color::DarkGray)),
            Span::styled("          │", Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(Color::Blue)),
            Span::styled("org/output", Style::default().fg(theme.realm_color("org"))),
            Span::styled(
                " → PageNative, BlockNative       ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("(LLM output)", Style::default().fg(Color::DarkGray)),
            Span::styled("       │", Style::default().fg(Color::Blue)),
        ]),
        Line::from(Span::styled(
            "  └──────────────────────────────────────────────────────────────────────────┘",
            Style::default().fg(Color::Blue),
        )),
        Line::from(""),
        // TRAIT (ADR-024 Data Origin renames)
        Line::from(Span::styled(
            "  ┌─ TRAIT: How does it behave with locales? ───────────────────────────────┐",
            Style::default().fg(theme.trait_color("defined")),
        )),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(theme.trait_color("defined"))),
            Span::styled("■", Style::default().fg(theme.trait_color("defined"))),
            Span::styled(" defined     ", Style::default().fg(Color::White)),
            Span::styled(
                "Same everywhere      Entity, Page, Block",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                "              │",
                Style::default().fg(theme.trait_color("defined")),
            ),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(theme.trait_color("defined"))),
            Span::styled("□", Style::default().fg(theme.trait_color("authored"))),
            Span::styled(" authored    ", Style::default().fg(Color::White)),
            Span::styled(
                "Generated per locale EntityNative@fr-FR",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                "             │",
                Style::default().fg(theme.trait_color("defined")),
            ),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(theme.trait_color("defined"))),
            Span::styled("◇", Style::default().fg(theme.trait_color("imported"))),
            Span::styled(" imported    ", Style::default().fg(Color::White)),
            Span::styled(
                "Locale expertise     Term, Culture",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                "                    │",
                Style::default().fg(theme.trait_color("defined")),
            ),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(theme.trait_color("defined"))),
            Span::styled("★", Style::default().fg(theme.trait_color("generated"))),
            Span::styled(" generated   ", Style::default().fg(Color::White)),
            Span::styled(
                "LLM output           PageNative",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                "                        │",
                Style::default().fg(theme.trait_color("defined")),
            ),
        ]),
        Line::from(vec![
            Span::styled("  │  ", Style::default().fg(theme.trait_color("defined"))),
            Span::styled("▪", Style::default().fg(theme.trait_color("retrieved"))),
            Span::styled(" retrieved   ", Style::default().fg(Color::White)),
            Span::styled(
                "Computed metrics     SEOKeywordMetrics",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                "                 │",
                Style::default().fg(theme.trait_color("defined")),
            ),
        ]),
        Line::from(Span::styled(
            "  └──────────────────────────────────────────────────────────────────────────┘",
            Style::default().fg(theme.trait_color("defined")),
        )),
    ];

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intro_pages_constant() {
        assert_eq!(INTRO_PAGES, 3);
    }
}
