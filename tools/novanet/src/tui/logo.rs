//! NovaNet ASCII logo and branding.
//!
//! Saturn-graph animated logo representing NovaNet's context graph.
//! Three variants: full (boot/help), compact (status bar), inline (headers).

use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::tui::theme;

/// Full Saturn-graph logo for boot splash and help screen (15 lines).
pub const FULL_LOGO: &[&str] = &[
    r#"                     .    ✦    .                     "#,
    r#"              ✦                       ✦              "#,
    r#"          ·        ◉ ─ ─ ─ ◉        ·               "#,
    r#"        ◉─────── ╱           ╲ ───────◉             "#,
    r#"       ╱       ╱  ╭─────────╮  ╲       ╲            "#,
    r#" ◉────╱──────╱    │ N O V A │    ╲──────╲────◉      "#,
    r#"═══════════════╳  │  N E T  │  ╳═══════════════     "#,
    r#"═══════════════╳  │    ◉    │  ╳═══════════════     "#,
    r#" ◉────╲──────╲    │         │    ╱──────╱────◉      "#,
    r#"       ╲       ╲  ╰─────────╯  ╱       ╱            "#,
    r#"        ◉─────── ╲           ╱ ───────◉             "#,
    r#"          ·        ◉ ─ ─ ─ ◉        ·               "#,
    r#"              ✦                       ✦              "#,
    r#"                     .    ✦    .                     "#,
];

/// Compact 3-line logo for the status bar corner.
#[allow(dead_code)] // Used by tests; available for status bar rendering in Phase 7C
pub const COMPACT_LOGO: &[&str] = &[r#"◉─╮╭──╮╭─◉"#, r#"═══╳│◉N│╳═══"#, r#"◉─╯╰──╯╰─◉"#];

/// Single-line inline logo for headers and tab bars.
#[allow(dead_code)] // Used by tests; available for header rendering
pub const INLINE_LOGO: &str = "◉═╳◉╳═◉ NOVANET";

/// Render the full logo with Galaxy theme colors.
///
/// Returns styled `Line`s ready for ratatui rendering.
pub fn full_logo_lines() -> Vec<Line<'static>> {
    FULL_LOGO
        .iter()
        .map(|line| colorize_logo_line(line))
        .collect()
}

/// Render branding lines below the logo.
pub fn branding_lines() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled(
            "SuperNovae Studio",
            Style::default()
                .fg(theme::NOVA_WHITE)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "context graph engine v9.0",
            Style::default().fg(theme::STAR_DIM),
        )),
    ]
}

/// Render the inline logo with colors for headers.
#[allow(dead_code)] // Used by tests; available for header rendering
pub fn inline_logo() -> Line<'static> {
    Line::from(vec![
        Span::styled(
            "◉═╳◉╳═◉",
            Style::default()
                .fg(theme::CYBER_CYAN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            "NOVANET",
            Style::default()
                .fg(theme::NOVA_WHITE)
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

/// Colorize a single logo line based on character patterns.
///
/// - Stars (· ✦ .)  → star_dim
/// - Orbit nodes (◉) → plasma_pink (center) or nebula_purple
/// - Ring (═╳)       → cyber_cyan, bold
/// - Planet border   → nebula_purple
/// - Planet text     → nova_white, bold
/// - Edges (─ ╱ ╲)  → star_dim
fn colorize_logo_line(line: &str) -> Line<'static> {
    let chars: Vec<char> = line.chars().collect();
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        let (content, style) = match ch {
            // Stars and dots
            '✦' | '·' | '.' => (ch.to_string(), Style::default().fg(theme::STAR_DIM)),
            // Orbit nodes
            '◉' => (
                "◉".to_string(),
                Style::default()
                    .fg(theme::PLASMA_PINK)
                    .add_modifier(Modifier::BOLD),
            ),
            // Ring characters
            '═' | '╳' => (
                ch.to_string(),
                Style::default()
                    .fg(theme::CYBER_CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            // Planet border
            '╭' | '╮' | '╰' | '╯' | '│' => {
                (ch.to_string(), Style::default().fg(theme::NEBULA_PURPLE))
            }
            // Edge characters
            '─' | '╱' | '╲' => (ch.to_string(), Style::default().fg(theme::STAR_DIM)),
            // Text inside planet (uppercase letters and spaces between box borders)
            'N' | 'O' | 'V' | 'A' | 'E' | 'T' => (
                ch.to_string(),
                Style::default()
                    .fg(theme::NOVA_WHITE)
                    .add_modifier(Modifier::BOLD),
            ),
            // Whitespace
            ' ' => {
                // Accumulate consecutive spaces
                let start = i;
                while i < chars.len() && chars[i] == ' ' {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                spans.push(Span::raw(s));
                continue;
            }
            // Everything else
            _ => (ch.to_string(), Style::default().fg(theme::STAR_DIM)),
        };

        spans.push(Span::styled(content, style));
        i += 1;
    }

    Line::from(spans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_logo_has_14_lines() {
        assert_eq!(FULL_LOGO.len(), 14);
    }

    #[test]
    fn compact_logo_has_3_lines() {
        assert_eq!(COMPACT_LOGO.len(), 3);
    }

    #[test]
    fn inline_logo_contains_novanet() {
        assert!(INLINE_LOGO.contains("NOVANET"));
    }

    #[test]
    fn full_logo_lines_returns_styled() {
        let lines = full_logo_lines();
        assert_eq!(lines.len(), 14);
        // Each line should have spans
        for line in &lines {
            assert!(!line.spans.is_empty());
        }
    }

    #[test]
    fn branding_lines_contain_studio() {
        let lines = branding_lines();
        let text: String = lines
            .iter()
            .flat_map(|l| l.spans.iter().map(|s| s.content.to_string()))
            .collect();
        assert!(text.contains("SuperNovae Studio"));
    }

    #[test]
    fn inline_logo_styled_has_novanet() {
        let line = inline_logo();
        let text: String = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(text.contains("NOVANET"));
    }

    #[test]
    fn colorize_ring_chars_are_cyan() {
        let line = colorize_logo_line("═══╳");
        for span in &line.spans {
            let ch = span.content.chars().next().unwrap_or(' ');
            if ch == '═' || ch == '╳' {
                assert_eq!(
                    span.style.fg,
                    Some(ratatui::style::Color::Rgb(34, 211, 238))
                );
            }
        }
    }

    #[test]
    fn colorize_stars_are_dim() {
        let line = colorize_logo_line("  ✦  ·  ");
        for span in &line.spans {
            let ch = span.content.chars().next().unwrap_or(' ');
            if ch == '✦' || ch == '·' {
                assert_eq!(
                    span.style.fg,
                    Some(ratatui::style::Color::Rgb(100, 116, 139))
                );
            }
        }
    }

    #[test]
    fn colorize_nodes_are_pink() {
        let line = colorize_logo_line("◉───◉");
        for span in &line.spans {
            if span.content.as_ref() == "◉" {
                assert_eq!(
                    span.style.fg,
                    Some(ratatui::style::Color::Rgb(236, 72, 153))
                );
            }
        }
    }
}
