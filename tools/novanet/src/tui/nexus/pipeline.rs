//! Pipeline Tab — Animated generation flow visualization.
//!
//! Shows the core NovaNet principle: Generation, NOT Translation.
//!
//! v0.12.0: Enhanced controls:
//! - [Space] Play/Pause animation
//! - [l/→] Forward one stage
//! - [h/←] Backward one stage
//! - [r] Reset to beginning
//! - [f] Fast mode (2x speed)
//! - [s] Slow mode (0.5x speed)
//!
//! The pipeline has 6 stages (0-5):
//! 0. IMPORTED - Input locale knowledge (Terms, Expressions, Patterns)
//! 1. DEFINED - Structural templates (Page, Entity, Block)
//! 2. CONTEXT_LOAD - Knowledge atoms loaded into LLM context
//! 3. LLM_GENERATE - LLM generation process
//! 4. AUTHORED - Output generated content (PageNative, EntityNative, BlockNative)
//! 5. COMPLETE - Generation complete

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::App;
use crate::tui::theme::Theme;
use crate::tui::ui::COLOR_UNFOCUSED_BORDER;

// =============================================================================
// PIPELINE STAGES
// =============================================================================

/// Pipeline stage definitions.
/// v11.8: Renamed per ADR-024 Data Origin semantics
pub const PIPELINE_STAGES: [(&str, &str, &str); 6] = [
    ("IMPORTED", "\u{25ca}", "Load imported knowledge"), // ◊ (was: KNOWLEDGE)
    ("DEFINED", "\u{25a0}", "Load structural definitions"), // ■ (was: INVARIANT)
    ("CONTEXT", "\u{2193}", "Inject into LLM context"),  // ↓
    ("GENERATE", "\u{21d2}", "LLM native generation"),   // ⇒
    ("AUTHORED", "\u{25a1}", "Output authored content"), // □ (was: LOCALIZED)
    ("COMPLETE", "\u{2713}", "Generation complete"),     // ✓
];

/// Get stage color based on stage index.
/// v11.8: Renamed per ADR-024 Data Origin semantics
fn stage_color(stage: usize, theme: &Theme) -> Color {
    match stage {
        0 => theme.trait_color("imported"), // Imported = purple (was: knowledge)
        1 => theme.trait_color("defined"),  // Defined = blue (was: invariant)
        2 => Color::Rgb(249, 115, 22),      // Context = orange
        3 => Color::Rgb(234, 179, 8),       // Generate = yellow (LLM)
        4 => theme.trait_color("authored"), // Authored = green (was: localized)
        5 => Color::Rgb(34, 197, 94),       // Complete = bright green
        _ => Color::White,
    }
}

/// Get animation character for vertical flow based on tick.
fn vflow_char(tick: usize, offset: usize) -> &'static str {
    let chars = ["\u{2502}", "\u{2503}", "\u{2502}", "\u{2506}"]; // │ ┃ │ ┆
    chars[(tick + offset) % chars.len()]
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Pipeline tab with animated generation flow.
pub fn render_pipeline_tab(f: &mut Frame, app: &App, area: Rect) {
    let theme = &app.theme;
    let current_stage = app.nexus.pipeline_stage;
    let is_animating = app.nexus.pipeline_animating;
    let tick = app.tick as usize;

    // Split into main diagram (70%) and principle box (30%)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    render_pipeline_diagram(f, chunks[0], current_stage, is_animating, tick, theme);
    render_principle_box(f, chunks[1], is_animating, current_stage);
}

/// Render the main pipeline diagram.
fn render_pipeline_diagram(
    f: &mut Frame,
    area: Rect,
    current_stage: usize,
    is_animating: bool,
    tick: usize,
    theme: &Theme,
) {
    let title = if is_animating {
        " \u{26a1} PIPELINE \u{2014} Animating (Space to pause) " // ⚡ PIPELINE — Animating
    } else {
        " \u{26a1} PIPELINE \u{2014} Press Space to animate " // ⚡ PIPELINE — Press Space
    };

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = build_pipeline_lines(
        current_stage,
        is_animating,
        tick,
        theme,
        inner.width as usize,
    );

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Build the pipeline diagram lines.
fn build_pipeline_lines(
    current_stage: usize,
    is_animating: bool,
    tick: usize,
    theme: &Theme,
    width: usize,
) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    // Helper to create styled stage box
    let stage_box = |stage_idx: usize, name: &str, symbol: &str| -> Vec<Line<'static>> {
        let is_active = stage_idx == current_stage;
        let is_past = stage_idx < current_stage;
        let color = stage_color(stage_idx, theme);

        let (border_style, content_style) = if is_active {
            (
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED),
            )
        } else if is_past {
            (
                Style::default().fg(color),
                Style::default().fg(color).add_modifier(Modifier::DIM),
            )
        } else {
            (
                Style::default().fg(COLOR_UNFOCUSED_BORDER),
                Style::default().fg(Color::Rgb(80, 80, 90)),
            )
        };

        vec![
            Line::from(Span::styled(
                "\u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}", // ┌──────────────┐
                border_style,
            )),
            Line::from(vec![
                Span::styled("\u{2502} ", border_style),
                Span::styled(format!("{} {:<10}", symbol, name), content_style),
                Span::styled(" \u{2502}", border_style),
            ]),
            Line::from(Span::styled(
                "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}", // └──────────────┘
                border_style,
            )),
        ]
    };

    // Header
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "                    THE GENERATION PIPELINE",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "\u{2500}".repeat(width.saturating_sub(4).min(60)),
        Style::default().fg(COLOR_UNFOCUSED_BORDER),
    )));
    lines.push(Line::from(""));

    // Row 1: IMPORTED and DEFINED side by side
    let imported_box = stage_box(0, "IMPORTED", "\u{25ca}");
    let defined_box = stage_box(1, "DEFINED", "\u{25a0}");

    // Merge boxes horizontally (simplified for terminal)
    for i in 0..3 {
        let mut spans: Vec<Span<'static>> = vec![Span::raw("    ")];

        // Add imported box line
        if let Some(line) = imported_box.get(i) {
            for span in &line.spans {
                spans.push(span.clone());
            }
        }

        spans.push(Span::raw("          "));

        // Add defined box line
        if let Some(line) = defined_box.get(i) {
            for span in &line.spans {
                spans.push(span.clone());
            }
        }

        lines.push(Line::from(spans));
    }

    // Labels under boxes
    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled("  (INPUT)      ", Style::default().fg(Color::DarkGray)),
        Span::raw("          "),
        Span::styled(" (STRUCTURE)   ", Style::default().fg(Color::DarkGray)),
    ]));

    lines.push(Line::from(""));

    // Flow arrows down
    let vflow = if is_animating && (current_stage == 0 || current_stage == 1) {
        vflow_char(tick, 0)
    } else {
        "\u{2502}" // │
    };
    let vflow_color = if current_stage <= 1 {
        Color::Rgb(100, 100, 110)
    } else {
        Color::Rgb(50, 50, 60)
    };

    lines.push(Line::from(vec![
        Span::raw("         "),
        Span::styled(vflow, Style::default().fg(vflow_color)),
        Span::raw("                       "),
        Span::styled(vflow, Style::default().fg(vflow_color)),
    ]));

    // Row 2: Context loading
    lines.push(Line::from(vec![
        Span::raw("         "),
        Span::styled(
            "\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}", // └────────────┬────────────┘
            Style::default().fg(if current_stage >= 2 {
                Color::Rgb(100, 100, 110)
            } else {
                Color::Rgb(50, 50, 60)
            }),
        ),
    ]));

    // Arrow to LLM
    let context_active = current_stage == 2;
    let context_color = if context_active && is_animating {
        Color::Rgb(249, 115, 22) // Orange
    } else if current_stage >= 2 {
        Color::Rgb(100, 100, 110)
    } else {
        Color::Rgb(50, 50, 60)
    };

    lines.push(Line::from(vec![
        Span::raw("                       "),
        Span::styled("\u{2502}", Style::default().fg(context_color)),
        Span::styled(" CONTEXT", Style::default().fg(Color::DarkGray)),
    ]));

    lines.push(Line::from(vec![
        Span::raw("                       "),
        Span::styled("\u{25bc}", Style::default().fg(context_color)), // ▼
    ]));

    lines.push(Line::from(""));

    // Row 3: LLM Generation box (wide)
    let llm_active = current_stage == 3;
    let llm_color = if llm_active {
        Color::Rgb(234, 179, 8) // Yellow
    } else if current_stage > 3 {
        Color::Rgb(100, 100, 110)
    } else {
        Color::Rgb(50, 50, 60)
    };

    let llm_style = if llm_active {
        Style::default().fg(llm_color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(llm_color)
    };

    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled(
            "\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}", // ╔═════════════════════════════════════╗
            llm_style,
        ),
    ]));

    // Animated LLM progress bar
    let progress_char = if llm_active && is_animating {
        match tick % 4 {
            0 => "\u{25cf}\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}", // ●○○○○○○○
            1 => "\u{25cb}\u{25cb}\u{25cf}\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}", // ○○●○○○○○
            2 => "\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cf}\u{25cb}\u{25cb}\u{25cb}", // ○○○○●○○○
            _ => "\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cf}\u{25cb}", // ○○○○○○●○
        }
    } else if current_stage > 3 {
        "\u{25cf}\u{25cf}\u{25cf}\u{25cf}\u{25cf}\u{25cf}\u{25cf}\u{25cf}" // ●●●●●●●●
    } else {
        "\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}\u{25cb}" // ○○○○○○○○
    };

    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled("\u{2551}", llm_style),
        Span::styled("          LLM GENERATION          ", llm_style),
        Span::styled("\u{2551}", llm_style),
    ]));

    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled("\u{2551}", llm_style),
        Span::styled(
            format!("    {}    ", progress_char),
            if llm_active {
                Style::default().fg(Color::Rgb(234, 179, 8))
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
        Span::styled("           \u{2551}", llm_style),
    ]));

    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled("\u{2551}", llm_style),
        Span::styled(
            " Knowledge + Structure \u{2192} Native ",
            Style::default().fg(Color::Rgb(150, 150, 160)),
        ),
        Span::styled("\u{2551}", llm_style),
    ]));

    lines.push(Line::from(vec![
        Span::raw("    "),
        Span::styled(
            "\u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}", // ╚═════════════════════════════════════╝
            llm_style,
        ),
    ]));

    lines.push(Line::from(""));

    // Arrow to output
    let output_color = if current_stage >= 4 {
        Color::Rgb(100, 100, 110)
    } else {
        Color::Rgb(50, 50, 60)
    };

    lines.push(Line::from(vec![
        Span::raw("                       "),
        Span::styled("\u{2502}", Style::default().fg(output_color)),
    ]));

    lines.push(Line::from(vec![
        Span::raw("                       "),
        Span::styled("\u{25bc}", Style::default().fg(output_color)), // ▼
    ]));

    // Row 4: AUTHORED output box
    let authored_box = stage_box(4, "AUTHORED", "\u{25a1}");
    for line in authored_box {
        let mut spans: Vec<Span<'static>> = vec![Span::raw("             ")];
        for span in line.spans {
            spans.push(span);
        }
        lines.push(Line::from(spans));
    }

    lines.push(Line::from(vec![
        Span::raw("             "),
        Span::styled("   (OUTPUT)     ", Style::default().fg(Color::DarkGray)),
    ]));

    // Completion status
    lines.push(Line::from(""));
    if current_stage >= 5 {
        lines.push(Line::from(vec![
            Span::raw("                  "),
            Span::styled(
                "\u{2713} Generation Complete!",
                Style::default()
                    .fg(Color::Rgb(34, 197, 94))
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    lines
}

/// Render the principle box at the bottom with enhanced controls (v0.12.0).
fn render_principle_box(f: &mut Frame, area: Rect, is_animating: bool, current_stage: usize) {
    let block = Block::default()
        .title(Span::styled(
            " CORE PRINCIPLE ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_UNFOCUSED_BORDER));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Build stage timeline indicator (v0.12.0)
    let mut timeline_spans: Vec<Span<'static>> = vec![Span::raw("   ")];
    for (i, (name, symbol, _)) in PIPELINE_STAGES.iter().enumerate() {
        let is_current = i == current_stage;
        let is_past = i < current_stage;

        let style = if is_current {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else if is_past {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        timeline_spans.push(Span::styled(format!("{} {}", symbol, name), style));

        if i < PIPELINE_STAGES.len() - 1 {
            let arrow_style = if is_past {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            timeline_spans.push(Span::styled(" → ", arrow_style));
        }
    }

    let lines: Vec<Line<'static>> = vec![
        Line::from(""),
        // Stage timeline
        Line::from(timeline_spans),
        Line::from(""),
        // WRONG way
        Line::from(vec![
            Span::styled("   ✗ WRONG:  ", Style::default().fg(Color::Red)),
            Span::styled("Source ", Style::default().fg(Color::Rgb(150, 150, 160))),
            Span::styled("→", Style::default().fg(Color::Red)),
            Span::styled(
                " Translate ",
                Style::default().fg(Color::Rgb(150, 150, 160)),
            ),
            Span::styled("→", Style::default().fg(Color::Red)),
            Span::styled(" Target", Style::default().fg(Color::Rgb(150, 150, 160))),
        ]),
        // RIGHT way
        Line::from(vec![
            Span::styled("   ✓ RIGHT:  ", Style::default().fg(Color::Green)),
            Span::styled(
                "Knowledge + Structure ",
                Style::default().fg(Color::Rgb(150, 150, 160)),
            ),
            Span::styled("→", Style::default().fg(Color::Green)),
            Span::styled(" Generate ", Style::default().fg(Color::Rgb(234, 179, 8))),
            Span::styled("→", Style::default().fg(Color::Green)),
            Span::styled(
                " Native Content",
                Style::default().fg(Color::Rgb(34, 197, 94)),
            ),
        ]),
        Line::from(""),
        // v0.12.0: Enhanced controls
        if is_animating {
            Line::from(vec![
                Span::styled("   [Space] Pause  ", Style::default().fg(Color::Cyan)),
                Span::styled("[h/←] Back  ", Style::default().fg(Color::Yellow)),
                Span::styled("[l/→] Forward  ", Style::default().fg(Color::Yellow)),
                Span::styled("[r] Reset  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("[{}/5]", current_stage),
                    Style::default().fg(Color::Cyan),
                ),
            ])
        } else {
            Line::from(vec![
                Span::styled("   [Space] Play  ", Style::default().fg(Color::Cyan)),
                Span::styled("[h/←] Back  ", Style::default().fg(Color::Yellow)),
                Span::styled("[l/→] Forward  ", Style::default().fg(Color::Yellow)),
                Span::styled("[r] Reset  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("[{}/5]", current_stage),
                    Style::default().fg(Color::Cyan),
                ),
            ])
        },
    ];

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Go to previous stage (v0.12.0).
pub fn prev_stage(stage: &mut usize) {
    if *stage > 0 {
        *stage -= 1;
    }
}

/// Go to next stage (v0.12.0).
pub fn next_stage(stage: &mut usize) {
    if *stage < 5 {
        *stage += 1;
    }
}

/// Reset to first stage (v0.12.0).
pub fn reset_stage(stage: &mut usize) {
    *stage = 0;
}

// =============================================================================
// ANIMATION TICK
// =============================================================================

/// Advance pipeline animation by one tick.
/// Returns true if animation should continue, false if complete.
pub fn advance_animation(stage: &mut usize) -> bool {
    if *stage < 5 {
        *stage += 1;
        true
    } else {
        // Reset to beginning for looping
        *stage = 0;
        true
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_stages_count() {
        assert_eq!(PIPELINE_STAGES.len(), 6);
    }

    #[test]
    fn test_pipeline_stages_names() {
        // v11.8: Renamed per ADR-024 Data Origin semantics
        assert_eq!(PIPELINE_STAGES[0].0, "IMPORTED"); // was: KNOWLEDGE
        assert_eq!(PIPELINE_STAGES[1].0, "DEFINED"); // was: INVARIANT
        assert_eq!(PIPELINE_STAGES[2].0, "CONTEXT");
        assert_eq!(PIPELINE_STAGES[3].0, "GENERATE");
        assert_eq!(PIPELINE_STAGES[4].0, "AUTHORED"); // was: LOCALIZED
        assert_eq!(PIPELINE_STAGES[5].0, "COMPLETE");
    }

    #[test]
    fn test_pipeline_symbols() {
        assert_eq!(PIPELINE_STAGES[0].1, "\u{25ca}"); // ◊
        assert_eq!(PIPELINE_STAGES[1].1, "\u{25a0}"); // ■
        assert_eq!(PIPELINE_STAGES[4].1, "\u{25a1}"); // □
        assert_eq!(PIPELINE_STAGES[5].1, "\u{2713}"); // ✓
    }

    #[test]
    fn test_vflow_char_animation() {
        // Should cycle through 4 different characters
        let chars: Vec<&str> = (0..4).map(|t| vflow_char(t, 0)).collect();
        assert_eq!(chars.len(), 4);
        // Verify cycling
        assert_eq!(vflow_char(0, 0), vflow_char(4, 0));
        assert_eq!(vflow_char(1, 0), vflow_char(5, 0));
    }

    #[test]
    fn test_stage_color() {
        let theme = Theme::new();

        // v11.8: Renamed per ADR-024 Data Origin semantics
        // Imported should be purple-ish (trait color, was: knowledge)
        let imported_color = stage_color(0, &theme);
        assert!(matches!(
            imported_color,
            Color::Rgb(_, _, _) | Color::Indexed(_) | Color::Magenta
        ));

        // Defined should be blue-ish (trait color, was: invariant)
        let defined_color = stage_color(1, &theme);
        assert!(matches!(
            defined_color,
            Color::Rgb(_, _, _) | Color::Indexed(_) | Color::Blue
        ));

        // Authored should be green-ish (trait color, was: localized)
        let authored_color = stage_color(4, &theme);
        assert!(matches!(
            authored_color,
            Color::Rgb(_, _, _) | Color::Indexed(_) | Color::Green
        ));
    }

    #[test]
    fn test_advance_animation() {
        let mut stage = 0;

        // Should advance through all stages
        for expected in 1..=5 {
            let continued = advance_animation(&mut stage);
            assert!(continued);
            assert_eq!(stage, expected);
        }

        // Should loop back to 0
        let continued = advance_animation(&mut stage);
        assert!(continued);
        assert_eq!(stage, 0);
    }

    #[test]
    fn test_advance_animation_loop() {
        let mut stage = 5;

        // From stage 5, should reset to 0
        let continued = advance_animation(&mut stage);
        assert!(continued);
        assert_eq!(stage, 0);
    }
}
