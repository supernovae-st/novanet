//! Stats tab - Matrix Control Tower dashboard.
//!
//! Displays NovaNet schema statistics with cyberpunk aesthetics:
//! - Hero panel: Big animated counters (NODES, ARCS, LAYERS, TRAITS, REALMS)
//! - Sparkline: System heartbeat pulse animation
//! - Bar charts: Realm, Layer, and Arc Family distributions
//!
//! v0.12.0: Complete redesign as "Matrix Control Tower" dashboard.

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Sparkline};

use crate::tui::app::App;

// =============================================================================
// NEON CYBERPUNK COLORS
// =============================================================================

/// Magenta (#d33682) - Hero accents, status indicators
const COLOR_MAGENTA: Color = Color::Rgb(211, 54, 130);

/// Violet (#6c71c4) - Big numbers, emphasis
const COLOR_VIOLET: Color = Color::Rgb(108, 113, 196);

/// Cyan (#2aa198) - Data bars, secondary info
const COLOR_CYAN: Color = Color::Rgb(42, 161, 152);

/// Green (#859900) - Success, nominal status
const COLOR_GREEN: Color = Color::Rgb(133, 153, 0);

/// Blue (#268bd2) - Layers
const COLOR_BLUE: Color = Color::Rgb(38, 139, 210);

/// Orange (#cb4b16) - Arc families accent
const COLOR_ORANGE: Color = Color::Rgb(203, 75, 22);

/// Base dimmed (#586e75) - Borders, labels
const COLOR_DIM: Color = Color::Rgb(88, 110, 117);

/// Base muted (#93a1a1) - Secondary text
const COLOR_MUTED: Color = Color::Rgb(147, 161, 161);

// =============================================================================
// CONSTANTS
// =============================================================================

/// Boot animation duration in frames (~40 frames at 50ms = ~2s boot).
const BOOT_FRAMES: usize = 40;

/// Number of data points for heartbeat sparkline.
const HEARTBEAT_LEN: usize = 30;

/// Heartbeat waveform pattern (scaled 0-100).
const HEARTBEAT_PATTERN: &[u64] = &[
    10, 10, 10, 15, 25, 85, 100, 70, 30, 15, 10, 10, 15, 40, 60, 45, 20, 10, 10, 10,
];

// =============================================================================
// STATS STATE
// =============================================================================

/// State for the Stats Control Tower dashboard.
#[derive(Debug, Clone)]
pub struct StatsState {
    /// Animation frame counter (0 to BOOT_FRAMES during boot, then continues).
    pub frame: usize,
    /// Whether boot animation has completed.
    pub boot_complete: bool,
    /// Heartbeat sparkline data (cyclic, always animated).
    pub heartbeat: Vec<u64>,
    /// Heartbeat position (cycles through HEARTBEAT_PATTERN).
    heartbeat_pos: usize,
    /// Quiz score history (percentage, 0-100).
    pub score_history: Vec<u64>,
    /// Category mastery percentages (v0.12.0).
    pub category_mastery: Vec<(String, f64)>,
}

impl Default for StatsState {
    fn default() -> Self {
        Self::new()
    }
}

impl StatsState {
    /// Create a new StatsState.
    pub fn new() -> Self {
        Self {
            frame: 0,
            boot_complete: false,
            heartbeat: vec![10; HEARTBEAT_LEN],
            heartbeat_pos: 0,
            score_history: Vec::new(),
            category_mastery: Vec::new(),
        }
    }

    /// Add a quiz score to history (v0.12.0).
    pub fn add_score(&mut self, score_pct: u64) {
        self.score_history.push(score_pct);
        // Keep last 20 scores
        if self.score_history.len() > 20 {
            self.score_history.remove(0);
        }
    }

    /// Update category mastery from quiz scores (v0.12.0).
    pub fn update_mastery(&mut self, cat_scores: &[(super::quiz::QuizCategory, usize, usize)]) {
        self.category_mastery = cat_scores
            .iter()
            .map(|(cat, correct, total)| {
                let pct = if *total > 0 {
                    (*correct as f64 / *total as f64) * 100.0
                } else {
                    0.0
                };
                (format!("{:?}", cat), pct)
            })
            .collect();
    }

    /// Advance animation frame. Called on each tick (~50ms).
    pub fn tick(&mut self) {
        self.frame = self.frame.saturating_add(1);

        // Mark boot complete after BOOT_FRAMES
        if self.frame >= BOOT_FRAMES && !self.boot_complete {
            self.boot_complete = true;
        }

        // Update heartbeat sparkline (always animating)
        self.heartbeat_pos = (self.heartbeat_pos + 1) % HEARTBEAT_PATTERN.len();
        self.heartbeat.remove(0);
        self.heartbeat.push(HEARTBEAT_PATTERN[self.heartbeat_pos]);
    }

    /// Get boot progress (0.0 to 1.0).
    fn boot_progress(&self) -> f64 {
        if self.boot_complete {
            1.0
        } else {
            (self.frame as f64 / BOOT_FRAMES as f64).min(1.0)
        }
    }

    /// Get animated value during boot (counts up from 0 to target).
    fn animated_value(&self, target: i64) -> i64 {
        if self.boot_complete {
            target
        } else {
            let progress = self.boot_progress();
            // Easing: ease-out-cubic for smooth deceleration
            let eased = 1.0 - (1.0 - progress).powi(3);
            (target as f64 * eased).round() as i64
        }
    }
}

// =============================================================================
// SCHEMA STATS (extracted from App)
// =============================================================================

/// Schema statistics extracted from the TaxonomyTree.
struct SchemaStats {
    /// Total node classes (61)
    kinds: i64,
    /// Total arc classes (146)
    arcs: i64,
    /// Number of layers (10)
    layers: usize,
    /// Number of traits (5)
    traits: usize,
    /// Number of realms (2)
    realms: usize,
    /// Classs per realm: (name, count, color)
    realm_distribution: Vec<(&'static str, usize, Color)>,
    /// Classs per layer: (name, count)
    layer_distribution: Vec<(String, usize)>,
    /// Arcs per family: (name, count)
    arc_family_distribution: Vec<(String, usize)>,
}

impl SchemaStats {
    /// Extract schema stats from the App's TaxonomyTree.
    fn from_app(app: &App) -> Self {
        let tree = &app.tree;

        // Realm distribution with colors
        let realm_distribution: Vec<_> = tree
            .realms
            .iter()
            .map(|r| {
                let name = if r.key == "shared" { "shared" } else { "org" };
                let count = r.total_classes();
                let color = if r.key == "shared" {
                    COLOR_CYAN
                } else {
                    COLOR_BLUE
                };
                (name, count, color)
            })
            .collect();

        // Layer distribution (flatten all layers across realms)
        let mut layer_distribution: Vec<(String, usize)> = Vec::new();
        for realm in &tree.realms {
            for layer in &realm.layers {
                layer_distribution.push((layer.display_name.clone(), layer.classes.len()));
            }
        }
        // Sort by count descending
        layer_distribution.sort_by(|a, b| b.1.cmp(&a.1));

        // Arc family distribution
        let arc_family_distribution: Vec<_> = tree
            .arc_families
            .iter()
            .map(|f| (f.display_name.clone(), f.arc_classes.len()))
            .collect();

        // Count unique layers
        let total_layers: usize = tree.realms.iter().map(|r| r.layers.len()).sum();

        Self {
            kinds: tree.stats.class_count,
            arcs: tree.stats.arc_class_count,
            layers: total_layers,
            traits: 5, // Fixed: defined, authored, imported, generated, retrieved
            realms: tree.realms.len(),
            realm_distribution,
            layer_distribution,
            arc_family_distribution,
        }
    }
}

// =============================================================================
// RENDER FUNCTIONS
// =============================================================================

/// Render the Stats Control Tower dashboard.
pub fn render_stats_tab(f: &mut Frame, app: &App, area: Rect) {
    let stats = SchemaStats::from_app(app);
    let state = &app.nexus.stats;

    // Main layout: hero panel on top, details below
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // Hero panel with big numbers
            Constraint::Min(12),   // Details: 3-column bar charts
        ])
        .split(area);

    // === Hero Panel ===
    render_hero_panel(f, chunks[0], &stats, state);

    // === Details Panel: 3 columns ===
    render_details_panel(f, chunks[1], &stats, state);
}

/// Render the hero panel with big numbers and heartbeat.
fn render_hero_panel(f: &mut Frame, area: Rect, stats: &SchemaStats, state: &StatsState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_MAGENTA))
        .title(Span::styled(
            " ◈ NOVANET CONTROL ",
            Style::default()
                .fg(COLOR_MAGENTA)
                .add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split inner: left (numbers), right (heartbeat + status)
    let hero_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(50), Constraint::Length(35)])
        .split(inner);

    // === Left side: Big numbers ===
    render_big_numbers(f, hero_chunks[0], stats, state);

    // === Right side: Heartbeat + status ===
    render_heartbeat(f, hero_chunks[1], state);
}

/// Render the big animated counter numbers.
fn render_big_numbers(f: &mut Frame, area: Rect, stats: &SchemaStats, state: &StatsState) {
    // Animated values
    let kinds_val = state.animated_value(stats.kinds);
    let arcs_val = state.animated_value(stats.arcs);
    let layers_val = state.animated_value(stats.layers as i64);
    let traits_val = state.animated_value(stats.traits as i64);
    let realms_val = state.animated_value(stats.realms as i64);

    // Number style (large, violet)
    let num_style = Style::default()
        .fg(COLOR_VIOLET)
        .add_modifier(Modifier::BOLD);

    // Label style (dim)
    let label_style = Style::default().fg(COLOR_DIM);

    // Build lines
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(format!("{:>4}", kinds_val), num_style),
            Span::raw("        "),
            Span::styled(format!("{:>4}", arcs_val), num_style),
            Span::raw("        "),
            Span::styled(format!("{:>3}", layers_val), num_style),
            Span::raw("         "),
            Span::styled(format!("{:>2}", traits_val), num_style),
            Span::raw("         "),
            Span::styled(format!("{:>2}", realms_val), num_style),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("╔════╗", label_style),
            Span::raw("   "),
            Span::styled("╔════╗", label_style),
            Span::raw("   "),
            Span::styled("╔════╗", label_style),
            Span::raw("   "),
            Span::styled("╔════╗", label_style),
            Span::raw("   "),
            Span::styled("╔════╗", label_style),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled("║NODE║", label_style),
            Span::raw("   "),
            Span::styled("║ ARC║", label_style),
            Span::raw("   "),
            Span::styled("║LAYR║", label_style),
            Span::raw("   "),
            Span::styled("║TRAT║", label_style),
            Span::raw("   "),
            Span::styled("║RELM║", label_style),
        ]),
    ];

    let para = Paragraph::new(lines).alignment(Alignment::Left);
    f.render_widget(para, area);
}

/// Render the heartbeat sparkline and status indicator.
fn render_heartbeat(f: &mut Frame, area: Rect, state: &StatsState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    // Status line
    let status = if state.boot_complete {
        Line::from(vec![
            Span::styled(
                "NOMINAL ",
                Style::default()
                    .fg(COLOR_GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("●", Style::default().fg(COLOR_GREEN)),
        ])
    } else {
        let dots = ".".repeat((state.frame % 4) + 1);
        Line::from(vec![Span::styled(
            format!("BOOTING{:<4}", dots),
            Style::default()
                .fg(COLOR_ORANGE)
                .add_modifier(Modifier::BOLD),
        )])
    };
    let status_widget = Paragraph::new(status).alignment(Alignment::Right);
    f.render_widget(status_widget, chunks[0]);

    // Heartbeat sparkline
    let sparkline = Sparkline::default()
        .data(&state.heartbeat)
        .max(100)
        .style(Style::default().fg(COLOR_CYAN));

    f.render_widget(sparkline, chunks[1]);
}

/// Render the details panel with 3-column bar charts.
fn render_details_panel(f: &mut Frame, area: Rect, stats: &SchemaStats, state: &StatsState) {
    // Split into 3 columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30), // Realms
            Constraint::Percentage(40), // Layers
            Constraint::Percentage(30), // Arc Families
        ])
        .split(area);

    // === Column 1: Realms ===
    render_realm_bars(f, columns[0], stats, state);

    // === Column 2: Layers ===
    render_layer_bars(f, columns[1], stats, state);

    // === Column 3: Arc Families ===
    render_arc_family_bars(f, columns[2], stats, state);
}

/// Render realm distribution bars.
fn render_realm_bars(f: &mut Frame, area: Rect, stats: &SchemaStats, state: &StatsState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_DIM))
        .title(Span::styled(" REALMS ", Style::default().fg(COLOR_MUTED)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let total: usize = stats.realm_distribution.iter().map(|(_, c, _)| c).sum();

    let mut lines = Vec::new();
    for (name, count, color) in &stats.realm_distribution {
        let animated_count = state.animated_value(*count as i64) as usize;
        let bar_width = if total > 0 && inner.width > 15 {
            ((animated_count as f64 / total as f64) * (inner.width as f64 - 15.0)) as usize
        } else {
            0
        };

        let icon = if *name == "shared" { "◉" } else { "◎" };
        let bar = "█".repeat(bar_width);
        let empty = "░".repeat((inner.width as usize).saturating_sub(15 + bar_width));

        lines.push(Line::from(vec![
            Span::styled(format!(" {} ", icon), Style::default().fg(*color)),
            Span::styled(format!("{:<8}", name), Style::default().fg(COLOR_MUTED)),
            Span::styled(bar, Style::default().fg(*color)),
            Span::styled(empty, Style::default().fg(COLOR_DIM)),
            Span::styled(
                format!(" {:>2}", animated_count),
                Style::default().fg(*color),
            ),
        ]));
        lines.push(Line::from(""));
    }

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

/// Render layer distribution bars.
fn render_layer_bars(f: &mut Frame, area: Rect, stats: &SchemaStats, state: &StatsState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_DIM))
        .title(Span::styled(" LAYERS ", Style::default().fg(COLOR_MUTED)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let max_count = stats
        .layer_distribution
        .iter()
        .map(|(_, c)| *c)
        .max()
        .unwrap_or(1);

    let mut lines = Vec::new();
    for (name, count) in stats.layer_distribution.iter().take(inner.height as usize) {
        let animated_count = state.animated_value(*count as i64) as usize;
        let bar_width = if max_count > 0 && inner.width > 20 {
            ((animated_count as f64 / max_count as f64) * (inner.width as f64 - 20.0)) as usize
        } else {
            0
        };

        let bar = "█".repeat(bar_width);
        let empty = "░".repeat((inner.width as usize).saturating_sub(20 + bar_width));

        // Truncate name to fit
        let display_name: String = name.chars().take(12).collect();

        lines.push(Line::from(vec![
            Span::styled(
                format!(" {:<12}", display_name),
                Style::default().fg(COLOR_MUTED),
            ),
            Span::styled(bar, Style::default().fg(COLOR_BLUE)),
            Span::styled(empty, Style::default().fg(COLOR_DIM)),
            Span::styled(
                format!(" {:>2}", animated_count),
                Style::default().fg(COLOR_BLUE),
            ),
        ]));
    }

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

/// Render arc family distribution bars.
fn render_arc_family_bars(f: &mut Frame, area: Rect, stats: &SchemaStats, state: &StatsState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(COLOR_DIM))
        .title(Span::styled(
            " ARC FAMILIES ",
            Style::default().fg(COLOR_MUTED),
        ));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let max_count = stats
        .arc_family_distribution
        .iter()
        .map(|(_, c)| *c)
        .max()
        .unwrap_or(1);

    // Arc family colors
    let family_colors = [
        COLOR_ORANGE,
        COLOR_CYAN,
        COLOR_VIOLET,
        COLOR_GREEN,
        COLOR_MAGENTA,
    ];

    let mut lines = Vec::new();
    for (i, (name, count)) in stats.arc_family_distribution.iter().enumerate() {
        let animated_count = state.animated_value(*count as i64) as usize;
        let bar_width = if max_count > 0 && inner.width > 18 {
            ((animated_count as f64 / max_count as f64) * (inner.width as f64 - 18.0)) as usize
        } else {
            0
        };

        let color = family_colors.get(i).copied().unwrap_or(COLOR_MUTED);
        let bar = "═".repeat(bar_width);
        let arrow = if bar_width > 0 { "►" } else { "" };

        // Truncate name
        let display_name: String = name.chars().take(10).collect();

        lines.push(Line::from(vec![
            Span::styled(" → ", Style::default().fg(color)),
            Span::styled(
                format!("{:<10}", display_name),
                Style::default().fg(COLOR_MUTED),
            ),
            Span::styled(bar, Style::default().fg(color)),
            Span::styled(arrow, Style::default().fg(color)),
            Span::styled(
                format!(" {:>3}", animated_count),
                Style::default().fg(color),
            ),
        ]));
    }

    let para = Paragraph::new(lines);
    f.render_widget(para, inner);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_state_new() {
        let state = StatsState::new();
        assert_eq!(state.frame, 0);
        assert!(!state.boot_complete);
        assert_eq!(state.heartbeat.len(), HEARTBEAT_LEN);
    }

    #[test]
    fn test_stats_tick_advances_frame() {
        let mut state = StatsState::new();
        state.tick();
        assert_eq!(state.frame, 1);
        state.tick();
        assert_eq!(state.frame, 2);
    }

    #[test]
    fn test_stats_boot_completes() {
        let mut state = StatsState::new();
        for _ in 0..BOOT_FRAMES {
            assert!(!state.boot_complete);
            state.tick();
        }
        assert!(state.boot_complete);
    }

    #[test]
    fn test_stats_heartbeat_cycles() {
        let mut state = StatsState::new();
        let initial = state.heartbeat.clone();

        // Tick several times
        for _ in 0..5 {
            state.tick();
        }

        // Heartbeat should have changed
        assert_ne!(state.heartbeat, initial);
        assert_eq!(state.heartbeat.len(), HEARTBEAT_LEN);
    }

    #[test]
    fn test_animated_value_during_boot() {
        let mut state = StatsState::new();
        let target = 100i64;

        // At frame 0, should be 0
        assert_eq!(state.animated_value(target), 0);

        // Tick halfway through boot
        for _ in 0..BOOT_FRAMES / 2 {
            state.tick();
        }

        // Should be somewhere between 0 and target
        let mid_val = state.animated_value(target);
        assert!(mid_val > 0);
        assert!(mid_val < target);

        // Complete boot
        for _ in 0..BOOT_FRAMES / 2 + 1 {
            state.tick();
        }

        // Should be at target
        assert_eq!(state.animated_value(target), target);
    }

    #[test]
    fn test_boot_progress() {
        let mut state = StatsState::new();

        assert_eq!(state.boot_progress(), 0.0);

        for _ in 0..BOOT_FRAMES / 2 {
            state.tick();
        }
        let mid_progress = state.boot_progress();
        assert!(mid_progress > 0.4 && mid_progress < 0.6);

        for _ in 0..BOOT_FRAMES {
            state.tick();
        }
        assert_eq!(state.boot_progress(), 1.0);
    }
}
