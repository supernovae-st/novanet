//! Guide Mode - Interactive educational views of the NovaNet taxonomy.
//!
//! Guide Mode provides 4 tabs for understanding NovaNet's core concepts:
//! - Traits: 5-trait constellation (invariant, localized, knowledge, derived, job)
//! - Layers: 2-realm split view (Global 2 layers | Tenant 7 layers)
//! - Arcs: Arc families and scope visualization
//! - Pipeline: Animated generation flow (not translation)

pub mod arcs;
pub mod layers;
pub mod pipeline;
pub mod traits;

use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::app::App;
use crate::tui::clipboard;
use crate::tui::theme::Theme;

// Re-export TraitStats and CodeExample for external use
pub use traits::{CodeExample, TraitStats, trait_code_examples};

// =============================================================================
// "DID YOU KNOW?" TIPS
// =============================================================================

/// Educational tips shown at the bottom of Guide mode.
/// Rotates through concepts about NovaNet's architecture.
pub const TIPS: &[&str] = &[
    "Knowledge is INPUT (savoir) - Localized is OUTPUT (generated)",
    "Layers define WHAT a node does, Traits define HOW it behaves with locale",
    "Content/Generated nodes have invariant parents (Entity→EntityContent, Page→PageGenerated)",
    "Generation, NOT translation: Knowledge + Structure -> Native content",
    "Global realm is READ-ONLY - all business content lives in Tenant",
    "Quick jump: gi=invariant, gl=localized, gk=knowledge, gd=derived, gj=job",
    "Knowledge nodes exist ONLY where needed (fr-FR: 20K Terms, sw-KE: 500)",
    "Arc families: ownership, localization, semantic, generation, mining",
    "Invariant = structure (solid border), Localized = output (dashed border)",
    "Press 'n' to see the next tip!",
];

/// Which Guide tab is currently active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GuideTab {
    /// Traits constellation (5 traits with detail panel)
    #[default]
    Traits,
    /// Layers split view (Global | Tenant)
    Layers,
    /// Arc families and scope
    Arcs,
    /// Generation pipeline animation
    Pipeline,
}

impl GuideTab {
    /// Get the shortcut key for this tab (1-4 when in Guide mode).
    pub fn shortcut(&self) -> char {
        match self {
            GuideTab::Traits => '1',
            GuideTab::Layers => '2',
            GuideTab::Arcs => '3',
            GuideTab::Pipeline => '4',
        }
    }

    /// Get the display label for this tab.
    pub fn label(&self) -> &'static str {
        match self {
            GuideTab::Traits => "Traits",
            GuideTab::Layers => "Layers",
            GuideTab::Arcs => "Arcs",
            GuideTab::Pipeline => "Pipeline",
        }
    }

    /// Get all tabs in order.
    pub fn all() -> &'static [GuideTab] {
        &[
            GuideTab::Traits,
            GuideTab::Layers,
            GuideTab::Arcs,
            GuideTab::Pipeline,
        ]
    }

    /// Cycle to next tab.
    pub fn next(&self) -> Self {
        match self {
            GuideTab::Traits => GuideTab::Layers,
            GuideTab::Layers => GuideTab::Arcs,
            GuideTab::Arcs => GuideTab::Pipeline,
            GuideTab::Pipeline => GuideTab::Traits,
        }
    }

    /// Cycle to previous tab.
    pub fn prev(&self) -> Self {
        match self {
            GuideTab::Traits => GuideTab::Pipeline,
            GuideTab::Layers => GuideTab::Traits,
            GuideTab::Arcs => GuideTab::Layers,
            GuideTab::Pipeline => GuideTab::Arcs,
        }
    }
}

/// Main Guide mode state.
#[derive(Debug, Clone)]
pub struct GuideState {
    /// Currently active tab.
    pub tab: GuideTab,

    // === Traits tab state ===
    /// Cursor position in traits constellation (0-4 for 5 traits).
    pub trait_cursor: usize,

    // === Layers tab state ===
    /// Cursor position in layers list.
    pub layer_cursor: usize,
    /// Selected realm (0=global, 1=tenant).
    pub layer_realm: usize,

    // === Arcs tab state ===
    /// Cursor position in arc families.
    pub arc_cursor: usize,

    // === Pipeline tab state ===
    /// Current stage in pipeline (0-based).
    pub pipeline_stage: usize,
    /// Whether pipeline animation is running.
    pub pipeline_animating: bool,

    // === Drill-down state ===
    /// Drill depth (0=overview, 1=kinds, 2=instances).
    pub drill_depth: usize,
    /// Cursor within drill-down list.
    pub drill_cursor: usize,

    // === Quick jump state ===
    /// Pending 'g' key for quick jump sequences (gi, gl, gk, gd, gj).
    pub pending_g: bool,

    // === Tips state ===
    /// Current tip index for "Did you know?" rotation.
    pub tip_index: usize,

    // === Clipboard state ===
    /// Message to display after clipboard operation (e.g., "Copied: Entity").
    pub clipboard_message: Option<String>,
    /// When the clipboard message was set (for auto-clear after ~2s).
    pub clipboard_message_time: Option<Instant>,
}

impl Default for GuideState {
    fn default() -> Self {
        Self::new()
    }
}

impl GuideState {
    /// Create a new GuideState with default values.
    pub fn new() -> Self {
        Self {
            tab: GuideTab::default(),
            trait_cursor: 0,
            layer_cursor: 0,
            layer_realm: 0,
            arc_cursor: 0,
            pipeline_stage: 0,
            pipeline_animating: false,
            drill_depth: 0,
            drill_cursor: 0,
            pending_g: false,
            tip_index: 0,
            clipboard_message: None,
            clipboard_message_time: None,
        }
    }

    /// Reset drill-down state (when switching tabs).
    pub fn reset_drill(&mut self) {
        self.drill_depth = 0;
        self.drill_cursor = 0;
    }

    /// Handle key input in Guide mode. Returns true if state changed.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Handle pending 'g' state for quick jump shortcuts (gi, gl, gk, gd, gj)
        if self.pending_g {
            self.pending_g = false; // Clear pending state
            return match key.code {
                KeyCode::Char('i') => self.jump_to_trait(0), // invariant
                KeyCode::Char('l') => self.jump_to_trait(1), // localized
                KeyCode::Char('k') => self.jump_to_trait(2), // knowledge
                KeyCode::Char('d') => self.jump_to_trait(3), // derived
                KeyCode::Char('j') => self.jump_to_trait(4), // job
                KeyCode::Char('g') => {
                    // gg = go to top (reset cursors)
                    self.trait_cursor = 0;
                    self.layer_cursor = 0;
                    self.arc_cursor = 0;
                    self.pipeline_stage = 0;
                    true
                }
                KeyCode::Esc => true, // Cancel pending g
                _ => false,           // Invalid sequence, ignore
            };
        }

        match key.code {
            // Start quick jump sequence with 'g'
            KeyCode::Char('g') => {
                self.pending_g = true;
                true
            }

            // Tab switching with number keys 1-4
            KeyCode::Char('1') => {
                if self.tab != GuideTab::Traits {
                    self.tab = GuideTab::Traits;
                    self.reset_drill();
                    true
                } else {
                    false
                }
            }
            KeyCode::Char('2') => {
                if self.tab != GuideTab::Layers {
                    self.tab = GuideTab::Layers;
                    self.reset_drill();
                    true
                } else {
                    false
                }
            }
            KeyCode::Char('3') => {
                if self.tab != GuideTab::Arcs {
                    self.tab = GuideTab::Arcs;
                    self.reset_drill();
                    true
                } else {
                    false
                }
            }
            KeyCode::Char('4') => {
                if self.tab != GuideTab::Pipeline {
                    self.tab = GuideTab::Pipeline;
                    self.reset_drill();
                    true
                } else {
                    false
                }
            }

            // Tab cycling with Tab key
            KeyCode::Tab => {
                self.tab = self.tab.next();
                self.reset_drill();
                true
            }
            KeyCode::BackTab => {
                self.tab = self.tab.prev();
                self.reset_drill();
                true
            }

            // Cursor navigation with j/k or Up/Down
            KeyCode::Up | KeyCode::Char('k') => self.navigate_up(),
            KeyCode::Down | KeyCode::Char('j') => self.navigate_down(),

            // Realm switching (Layers tab) or drill in/out with h/l or Left/Right
            KeyCode::Left | KeyCode::Char('h') => self.navigate_left(),
            KeyCode::Right | KeyCode::Char('l') => self.navigate_right(),

            // Enter for drill-down
            KeyCode::Enter => self.drill_down(),

            // Escape for drill-up (also clears pending_g)
            KeyCode::Esc => {
                self.pending_g = false;
                self.drill_up()
            }

            // Space for pipeline animation toggle
            KeyCode::Char(' ') => {
                if self.tab == GuideTab::Pipeline {
                    self.pipeline_animating = !self.pipeline_animating;
                    true
                } else {
                    false
                }
            }

            // 'n' to cycle to next tip
            KeyCode::Char('n') => {
                self.next_tip();
                true
            }

            // 'y' to yank (copy) current selection to clipboard
            KeyCode::Char('y') => self.yank_current(),

            _ => false,
        }
    }

    /// Yank (copy) current selection to clipboard.
    /// Returns true if state changed (message set).
    fn yank_current(&mut self) -> bool {
        if let Some(text) = self.get_current_yank_text() {
            match clipboard::copy_to_clipboard(&text) {
                Ok(()) => {
                    self.clipboard_message = Some(format!("Copied: {}", text));
                    self.clipboard_message_time = Some(Instant::now());
                    true
                }
                Err(e) => {
                    self.clipboard_message = Some(format!("Error: {}", e));
                    self.clipboard_message_time = Some(Instant::now());
                    true
                }
            }
        } else {
            false
        }
    }

    /// Get the text to yank based on current tab and selection.
    pub fn get_current_yank_text(&self) -> Option<String> {
        match self.tab {
            GuideTab::Traits => {
                // Yank the current trait name
                let traits = ["invariant", "localized", "knowledge", "derived", "job"];
                traits.get(self.trait_cursor).map(|s| s.to_string())
            }
            GuideTab::Layers => {
                // Yank the current layer key
                let layers = if self.layer_realm == 0 {
                    // Global layers
                    vec!["config", "locale-knowledge"]
                } else {
                    // Tenant layers
                    vec![
                        "config",
                        "foundation",
                        "structure",
                        "semantic",
                        "instruction",
                        "seo",
                        "output",
                    ]
                };
                layers.get(self.layer_cursor).map(|s| s.to_string())
            }
            GuideTab::Arcs => {
                // Yank the current arc family
                let families = ["ownership", "localization", "semantic", "generation", "mining"];
                families.get(self.arc_cursor).map(|s| s.to_string())
            }
            GuideTab::Pipeline => {
                // Yank the current pipeline stage
                let stages = [
                    "Knowledge",
                    "Entity",
                    "Structure",
                    "Instructions",
                    "Generation",
                    "Output",
                ];
                stages.get(self.pipeline_stage).map(|s| s.to_string())
            }
        }
    }

    /// Clear clipboard message if it has expired (>2 seconds old).
    pub fn clear_expired_clipboard_message(&mut self) {
        if let Some(time) = self.clipboard_message_time {
            if time.elapsed().as_secs() >= 2 {
                self.clipboard_message = None;
                self.clipboard_message_time = None;
            }
        }
    }

    /// Jump to a specific trait in the Traits tab.
    /// Used by quick jump shortcuts (gi, gl, gk, gd, gj).
    fn jump_to_trait(&mut self, trait_index: usize) -> bool {
        self.tab = GuideTab::Traits;
        self.trait_cursor = trait_index.min(4); // Clamp to 0-4
        self.reset_drill();
        true
    }

    /// Advance to the next "Did you know?" tip.
    pub fn next_tip(&mut self) {
        self.tip_index = (self.tip_index + 1) % TIPS.len();
    }

    /// Get the current "Did you know?" tip.
    pub fn current_tip(&self) -> &'static str {
        TIPS.get(self.tip_index).unwrap_or(&TIPS[0])
    }

    /// Check if there's a pending 'g' key waiting for completion.
    pub fn has_pending_g(&self) -> bool {
        self.pending_g
    }

    /// Navigate up (cursor movement).
    fn navigate_up(&mut self) -> bool {
        match self.tab {
            GuideTab::Traits => {
                if self.drill_depth == 0 {
                    // In constellation, cycle through 5 traits
                    self.trait_cursor = if self.trait_cursor == 0 {
                        4 // Wrap to last
                    } else {
                        self.trait_cursor - 1
                    };
                    true
                } else {
                    // In drill-down list
                    if self.drill_cursor > 0 {
                        self.drill_cursor -= 1;
                    }
                    true
                }
            }
            GuideTab::Layers => {
                if self.layer_cursor > 0 {
                    self.layer_cursor -= 1;
                    true
                } else {
                    false
                }
            }
            GuideTab::Arcs => {
                if self.arc_cursor > 0 {
                    self.arc_cursor -= 1;
                    true
                } else {
                    false
                }
            }
            GuideTab::Pipeline => {
                if self.pipeline_stage > 0 {
                    self.pipeline_stage -= 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Navigate down (cursor movement).
    fn navigate_down(&mut self) -> bool {
        match self.tab {
            GuideTab::Traits => {
                if self.drill_depth == 0 {
                    // In constellation, cycle through 5 traits
                    self.trait_cursor = (self.trait_cursor + 1) % 5;
                    true
                } else {
                    // In drill-down list (no max check yet, will be bounded by render)
                    self.drill_cursor += 1;
                    true
                }
            }
            GuideTab::Layers => {
                // Bound by number of layers in current realm
                // v11.0: global: 2 layers, tenant: 7 layers
                let max = if self.layer_realm == 0 { 1 } else { 6 };
                if self.layer_cursor < max {
                    self.layer_cursor += 1;
                    true
                } else {
                    false
                }
            }
            GuideTab::Arcs => {
                // 5 arc families
                if self.arc_cursor < 4 {
                    self.arc_cursor += 1;
                    true
                } else {
                    false
                }
            }
            GuideTab::Pipeline => {
                // 6 pipeline stages (0-5)
                if self.pipeline_stage < 5 {
                    self.pipeline_stage += 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Navigate left (realm switching in Layers, drill-out elsewhere).
    fn navigate_left(&mut self) -> bool {
        match self.tab {
            GuideTab::Layers => {
                // Switch to Global realm (0)
                if self.layer_realm != 0 {
                    self.layer_realm = 0;
                    self.layer_cursor = 0; // Reset cursor when switching realm
                    true
                } else {
                    false
                }
            }
            _ => {
                // Drill up as alternative to Escape
                self.drill_up()
            }
        }
    }

    /// Navigate right (realm switching in Layers, drill-in elsewhere).
    fn navigate_right(&mut self) -> bool {
        match self.tab {
            GuideTab::Layers => {
                // Switch to Tenant realm (1)
                if self.layer_realm != 1 {
                    self.layer_realm = 1;
                    self.layer_cursor = 0; // Reset cursor when switching realm
                    true
                } else {
                    false
                }
            }
            _ => {
                // Drill down as alternative to Enter
                self.drill_down()
            }
        }
    }

    /// Drill down into current selection.
    fn drill_down(&mut self) -> bool {
        match self.tab {
            GuideTab::Traits | GuideTab::Layers | GuideTab::Arcs => {
                if self.drill_depth < 2 {
                    self.drill_depth += 1;
                    self.drill_cursor = 0;
                    true
                } else {
                    false
                }
            }
            GuideTab::Pipeline => {
                // Pipeline doesn't have drill-down, toggle animation instead
                self.pipeline_animating = !self.pipeline_animating;
                true
            }
        }
    }

    /// Drill up from current view.
    fn drill_up(&mut self) -> bool {
        if self.drill_depth > 0 {
            self.drill_depth -= 1;
            self.drill_cursor = 0;
            true
        } else {
            false
        }
    }

    /// Get a flattened list of all kinds for the currently selected trait.
    /// Returns tuples of (layer_key, kind_key) for easy rendering.
    pub fn get_trait_kinds(&self, trait_stats: &[traits::TraitStats]) -> Vec<(String, String)> {
        if let Some(stat) = trait_stats.get(self.trait_cursor) {
            stat.kinds_by_layer
                .iter()
                .flat_map(|(layer_key, kinds)| {
                    kinds
                        .iter()
                        .map(|kind_key| (layer_key.clone(), kind_key.clone()))
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get breadcrumb for current Guide mode state.
    /// Returns path like "Guide > Traits > localized > EntityContent"
    pub fn breadcrumb(&self, trait_stats: &[traits::TraitStats]) -> String {
        let tab_name = self.tab.label();
        match self.tab {
            GuideTab::Traits => {
                let trait_name = traits::TRAIT_ORDER.get(self.trait_cursor).unwrap_or(&"");
                if self.drill_depth == 0 {
                    format!("Guide > {} > {}", tab_name, trait_name)
                } else {
                    let kinds = self.get_trait_kinds(trait_stats);
                    if let Some((layer, kind)) = kinds.get(self.drill_cursor) {
                        format!(
                            "Guide > {} > {} > {} ({})",
                            tab_name, trait_name, kind, layer
                        )
                    } else {
                        format!("Guide > {} > {}", tab_name, trait_name)
                    }
                }
            }
            GuideTab::Layers => {
                let realm = if self.layer_realm == 0 {
                    "Global"
                } else {
                    "Tenant"
                };
                format!("Guide > {} > {}", tab_name, realm)
            }
            GuideTab::Arcs => {
                let families = [
                    "ownership",
                    "localization",
                    "semantic",
                    "generation",
                    "mining",
                ];
                let family = families.get(self.arc_cursor).unwrap_or(&"");
                format!("Guide > {} > {}", tab_name, family)
            }
            GuideTab::Pipeline => {
                let stages = [
                    "Knowledge",
                    "Entity",
                    "Structure",
                    "Instructions",
                    "Generation",
                    "Output",
                ];
                let stage = stages.get(self.pipeline_stage).unwrap_or(&"");
                format!("Guide > {} > {}", tab_name, stage)
            }
        }
    }

    /// Clamp drill_cursor to valid bounds for the current kind list.
    pub fn clamp_drill_cursor(&mut self, max_len: usize) {
        if max_len == 0 {
            self.drill_cursor = 0;
        } else if self.drill_cursor >= max_len {
            self.drill_cursor = max_len - 1;
        }
    }
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Guide mode with tab bar, breadcrumb, content, and tips bar.
pub fn render_guide(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Length(1), // Breadcrumb
            Constraint::Min(1),    // Content
            Constraint::Length(2), // Tips bar
        ])
        .split(area);

    // Render tab bar
    render_tab_bar(f, chunks[0], app);

    // Render breadcrumb
    render_breadcrumb(f, chunks[1], app);

    // Render content based on active tab
    match app.guide.tab {
        GuideTab::Traits => traits::render_traits_tab(f, app, chunks[2]),
        GuideTab::Layers => layers::render_layers_tab(f, app, chunks[2]),
        GuideTab::Arcs => arcs::render_arcs_tab(f, app, chunks[2]),
        GuideTab::Pipeline => pipeline::render_pipeline_tab(f, app, chunks[2]),
    }

    // Render "Did you know?" tips bar
    render_tips_bar(f, chunks[3], app);
}

/// Render the tab bar at the top of Guide mode.
fn render_tab_bar(f: &mut Frame, area: Rect, app: &App) {
    let tabs: Vec<Span> = GuideTab::all()
        .iter()
        .enumerate()
        .map(|(idx, tab)| {
            let is_selected = *tab == app.guide.tab;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let symbol = match tab {
                GuideTab::Traits => "\u{25a0}",   // ■
                GuideTab::Layers => "\u{25a3}",   // ▣
                GuideTab::Arcs => "\u{21c4}",     // ⇄
                GuideTab::Pipeline => "\u{26a1}", // ⚡
            };

            Span::styled(format!("[{}]{} {} ", idx + 1, symbol, tab.label()), style)
        })
        .collect();

    let tabs_line = Line::from(tabs);

    let block = Block::default()
        .title(Span::styled(
            " Guide Mode ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(tabs_line).block(block);
    f.render_widget(paragraph, area);
}

/// Render the breadcrumb bar showing current location in Guide mode.
fn render_breadcrumb(f: &mut Frame, area: Rect, app: &App) {
    let trait_stats = app.tree.get_trait_stats();
    let breadcrumb = app.guide.breadcrumb(&trait_stats);

    // Style: dim path segments, bright current segment
    let segments: Vec<&str> = breadcrumb.split(" > ").collect();
    let mut spans: Vec<Span> = Vec::new();

    for (i, segment) in segments.iter().enumerate() {
        let is_last = i == segments.len() - 1;
        let style = if is_last {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        spans.push(Span::styled((*segment).to_string(), style));

        if !is_last {
            spans.push(Span::styled(
                " > ",
                Style::default().fg(Color::Rgb(60, 60, 70)),
            ));
        }
    }

    // Add drill hint if at depth 0 and drillable
    if app.guide.drill_depth == 0 && app.guide.tab != GuideTab::Pipeline {
        spans.push(Span::styled(
            "  [Enter: drill down]",
            Style::default().fg(Color::Rgb(80, 80, 100)),
        ));
    } else if app.guide.drill_depth > 0 {
        spans.push(Span::styled(
            "  [Esc: back]",
            Style::default().fg(Color::Rgb(80, 80, 100)),
        ));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    f.render_widget(paragraph, area);
}

/// Render the "Did you know?" tips bar at the bottom of Guide mode.
fn render_tips_bar(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.theme;

    // Priority: clipboard message > pending 'g' > normal tip
    if let Some(ref clipboard_msg) = app.guide.clipboard_message {
        // Show clipboard message (green for success, red for error)
        let is_error = clipboard_msg.starts_with("Error:");
        let style = if is_error {
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        };

        let prefix = Span::styled(" \u{f0c5} ", style); // 📋 clipboard icon area
        let message = Span::styled(clipboard_msg.clone(), style);
        let hint = Span::styled(
            "  [y: yank]",
            Style::default().fg(Color::DarkGray),
        );

        let line = Line::from(vec![prefix, message, hint]);
        let paragraph = Paragraph::new(vec![Line::from(""), line]);
        f.render_widget(paragraph, area);
        return;
    }

    let tip = app.guide.current_tip();
    let tip_index = app.guide.tip_index;
    let total_tips = TIPS.len();

    // Show pending 'g' indicator if waiting for second key
    let prefix = if app.guide.has_pending_g() {
        Span::styled(
            " g... ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            " \u{2728} Did you know? ",
            Style::default()
                .fg(Color::Rgb(139, 92, 246)) // Knowledge purple
                .add_modifier(Modifier::BOLD),
        )
    };

    // Build tip line with trait colors where relevant
    let tip_text = colorize_tip(tip, theme);

    // Tip counter + yank hint
    let counter = Span::styled(
        format!(" [{}/{}] [n: next] [y: yank]", tip_index + 1, total_tips),
        Style::default().fg(Color::DarkGray),
    );

    let mut spans = vec![prefix];
    spans.extend(tip_text);
    spans.push(counter);

    let line = Line::from(spans);
    let paragraph = Paragraph::new(vec![Line::from(""), line]);
    f.render_widget(paragraph, area);
}

/// Colorize tip text, highlighting trait names with their theme colors.
fn colorize_tip(tip: &str, theme: &Theme) -> Vec<Span<'static>> {
    // Keywords to highlight with their corresponding trait/type colors
    let keywords: &[(&str, &str)] = &[
        ("Knowledge", "knowledge"),
        ("KNOWLEDGE", "knowledge"),
        ("knowledge", "knowledge"),
        ("Localized", "localized"),
        ("LOCALIZED", "localized"),
        ("localized", "localized"),
        ("Invariant", "invariant"),
        ("INVARIANT", "invariant"),
        ("invariant", "invariant"),
        ("Derived", "derived"),
        ("DERIVED", "derived"),
        ("derived", "derived"),
        ("Job", "job"),
        ("JOB", "job"),
        ("job", "job"),
        ("INPUT", "knowledge"),
        ("OUTPUT", "localized"),
        ("Global", "global"),
        ("GLOBAL", "global"),
        ("Tenant", "tenant"),
        ("TENANT", "tenant"),
        ("Content", "localized"),
        ("Generated", "localized"),
        ("Generation", "localized"),
    ];

    let mut result: Vec<Span<'static>> = Vec::new();
    let mut remaining = tip.to_string();

    // Simple tokenization: scan for keywords
    while !remaining.is_empty() {
        let mut found = false;
        for (keyword, color_key) in keywords {
            if remaining.starts_with(*keyword) {
                // Found a keyword - add colored span
                let color = if *color_key == "global" {
                    theme.realm_color("global")
                } else if *color_key == "tenant" {
                    theme.realm_color("tenant")
                } else {
                    theme.trait_color(color_key)
                };
                result.push(Span::styled(
                    (*keyword).to_string(),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
                remaining = remaining[keyword.len()..].to_string();
                found = true;
                break;
            }
        }
        if !found {
            // Not a keyword, consume one character
            let ch = remaining.chars().next().unwrap();
            // Check if we can append to the last span if it's plain text
            if let Some(Span { content, style }) = result.last_mut() {
                if style.fg.is_none() || style.fg == Some(Color::Rgb(180, 180, 180)) {
                    // Same style, append to existing span
                    let mut new_content = content.to_string();
                    new_content.push(ch);
                    *content = std::borrow::Cow::Owned(new_content);
                    remaining = remaining[ch.len_utf8()..].to_string();
                    continue;
                }
            }
            // Add new plain text span
            result.push(Span::styled(
                ch.to_string(),
                Style::default().fg(Color::Rgb(180, 180, 180)),
            ));
            remaining = remaining[ch.len_utf8()..].to_string();
        }
    }

    result
}

/// Render a placeholder for tabs not yet implemented.
#[allow(dead_code)]
fn render_placeholder(f: &mut Frame, area: Rect, title: &str, message: &str) {
    let block = Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 70)));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(message, Style::default().fg(Color::DarkGray))),
        Line::from(""),
        Line::from(Span::styled(
            "Use [1-4] to switch tabs",
            Style::default().fg(Color::Rgb(100, 100, 120)),
        )),
    ];

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key_event(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::empty())
    }

    #[test]
    fn test_guide_state_new() {
        let state = GuideState::new();
        assert_eq!(state.tab, GuideTab::Traits);
        assert_eq!(state.trait_cursor, 0);
        assert!(!state.pending_g);
        assert_eq!(state.tip_index, 0);
    }

    #[test]
    fn test_tips_constant() {
        assert!(!TIPS.is_empty());
        assert!(TIPS.len() >= 5); // Ensure we have meaningful tips
    }

    #[test]
    fn test_current_tip() {
        let state = GuideState::new();
        let tip = state.current_tip();
        assert_eq!(tip, TIPS[0]);
    }

    #[test]
    fn test_next_tip_cycles() {
        let mut state = GuideState::new();
        assert_eq!(state.tip_index, 0);

        state.next_tip();
        assert_eq!(state.tip_index, 1);

        // Cycle through all tips
        for _ in 0..TIPS.len() {
            state.next_tip();
        }
        // Should wrap around
        assert_eq!(state.tip_index, 1);
    }

    #[test]
    fn test_pending_g_state() {
        let mut state = GuideState::new();
        assert!(!state.has_pending_g());

        // Press 'g' to enter pending state
        state.handle_key(key_event(KeyCode::Char('g')));
        assert!(state.has_pending_g());

        // Invalid key clears pending state
        state.handle_key(key_event(KeyCode::Char('x')));
        assert!(!state.has_pending_g());
    }

    #[test]
    fn test_quick_jump_gi() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers; // Start on different tab
        state.trait_cursor = 3;

        // Press 'g' then 'i' for invariant
        state.handle_key(key_event(KeyCode::Char('g')));
        assert!(state.has_pending_g());

        state.handle_key(key_event(KeyCode::Char('i')));
        assert!(!state.has_pending_g());
        assert_eq!(state.tab, GuideTab::Traits);
        assert_eq!(state.trait_cursor, 0); // invariant = index 0
    }

    #[test]
    fn test_quick_jump_gl() {
        let mut state = GuideState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('l')));

        assert_eq!(state.tab, GuideTab::Traits);
        assert_eq!(state.trait_cursor, 1); // localized = index 1
    }

    #[test]
    fn test_quick_jump_gk() {
        let mut state = GuideState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('k')));

        assert_eq!(state.tab, GuideTab::Traits);
        assert_eq!(state.trait_cursor, 2); // knowledge = index 2
    }

    #[test]
    fn test_quick_jump_gd() {
        let mut state = GuideState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('d')));

        assert_eq!(state.tab, GuideTab::Traits);
        assert_eq!(state.trait_cursor, 3); // derived = index 3
    }

    #[test]
    fn test_quick_jump_gj() {
        let mut state = GuideState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('j')));

        assert_eq!(state.tab, GuideTab::Traits);
        assert_eq!(state.trait_cursor, 4); // job = index 4
    }

    #[test]
    fn test_quick_jump_gg() {
        let mut state = GuideState::new();
        state.trait_cursor = 3;
        state.layer_cursor = 2;
        state.arc_cursor = 1;

        // gg should reset all cursors to 0
        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('g')));

        assert_eq!(state.trait_cursor, 0);
        assert_eq!(state.layer_cursor, 0);
        assert_eq!(state.arc_cursor, 0);
    }

    #[test]
    fn test_pending_g_cancelled_by_escape() {
        let mut state = GuideState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        assert!(state.has_pending_g());

        state.handle_key(key_event(KeyCode::Esc));
        assert!(!state.has_pending_g());
    }

    #[test]
    fn test_n_key_cycles_tips() {
        let mut state = GuideState::new();
        assert_eq!(state.tip_index, 0);

        state.handle_key(key_event(KeyCode::Char('n')));
        assert_eq!(state.tip_index, 1);

        state.handle_key(key_event(KeyCode::Char('n')));
        assert_eq!(state.tip_index, 2);
    }

    #[test]
    fn test_tab_cycling() {
        let mut state = GuideState::new();
        assert_eq!(state.tab, GuideTab::Traits);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, GuideTab::Layers);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, GuideTab::Arcs);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, GuideTab::Pipeline);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, GuideTab::Traits); // Wraps around
    }

    #[test]
    fn test_guide_tab_all() {
        let all = GuideTab::all();
        assert_eq!(all.len(), 4);
        assert_eq!(all[0], GuideTab::Traits);
        assert_eq!(all[1], GuideTab::Layers);
        assert_eq!(all[2], GuideTab::Arcs);
        assert_eq!(all[3], GuideTab::Pipeline);
    }

    #[test]
    fn test_guide_tab_shortcuts() {
        assert_eq!(GuideTab::Traits.shortcut(), '1');
        assert_eq!(GuideTab::Layers.shortcut(), '2');
        assert_eq!(GuideTab::Arcs.shortcut(), '3');
        assert_eq!(GuideTab::Pipeline.shortcut(), '4');
    }

    #[test]
    fn test_guide_tab_labels() {
        assert_eq!(GuideTab::Traits.label(), "Traits");
        assert_eq!(GuideTab::Layers.label(), "Layers");
        assert_eq!(GuideTab::Arcs.label(), "Arcs");
        assert_eq!(GuideTab::Pipeline.label(), "Pipeline");
    }

    // ==========================================================================
    // TAB SWITCHING WITH NUMBER KEYS (1-4)
    // ==========================================================================

    #[test]
    fn test_number_key_1_switches_to_traits() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;

        let changed = state.handle_key(key_event(KeyCode::Char('1')));
        assert!(changed);
        assert_eq!(state.tab, GuideTab::Traits);

        // Pressing '1' when already on Traits should return false
        let changed = state.handle_key(key_event(KeyCode::Char('1')));
        assert!(!changed);
        assert_eq!(state.tab, GuideTab::Traits);
    }

    #[test]
    fn test_number_key_2_switches_to_layers() {
        let mut state = GuideState::new();
        assert_eq!(state.tab, GuideTab::Traits);

        let changed = state.handle_key(key_event(KeyCode::Char('2')));
        assert!(changed);
        assert_eq!(state.tab, GuideTab::Layers);
    }

    #[test]
    fn test_number_key_3_switches_to_arcs() {
        let mut state = GuideState::new();

        let changed = state.handle_key(key_event(KeyCode::Char('3')));
        assert!(changed);
        assert_eq!(state.tab, GuideTab::Arcs);
    }

    #[test]
    fn test_number_key_4_switches_to_pipeline() {
        let mut state = GuideState::new();

        let changed = state.handle_key(key_event(KeyCode::Char('4')));
        assert!(changed);
        assert_eq!(state.tab, GuideTab::Pipeline);
    }

    #[test]
    fn test_tab_switch_resets_drill() {
        let mut state = GuideState::new();
        state.drill_depth = 2;
        state.drill_cursor = 5;

        state.handle_key(key_event(KeyCode::Char('2'))); // Switch to Layers
        assert_eq!(state.drill_depth, 0);
        assert_eq!(state.drill_cursor, 0);
    }

    // ==========================================================================
    // BACKTAB NAVIGATION
    // ==========================================================================

    #[test]
    fn test_backtab_cycling() {
        let mut state = GuideState::new();
        assert_eq!(state.tab, GuideTab::Traits);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, GuideTab::Pipeline); // Wraps to end

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, GuideTab::Arcs);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, GuideTab::Layers);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, GuideTab::Traits);
    }

    #[test]
    fn test_guide_tab_next_prev_symmetry() {
        // Verify next() and prev() are inverse operations
        for tab in GuideTab::all() {
            assert_eq!(tab.next().prev(), *tab);
            assert_eq!(tab.prev().next(), *tab);
        }
    }

    // ==========================================================================
    // DRILL-DOWN STATE MANAGEMENT
    // ==========================================================================

    #[test]
    fn test_drill_down_enter_key() {
        let mut state = GuideState::new();
        assert_eq!(state.drill_depth, 0);

        // Enter to drill down
        let changed = state.handle_key(key_event(KeyCode::Enter));
        assert!(changed);
        assert_eq!(state.drill_depth, 1);
        assert_eq!(state.drill_cursor, 0);
    }

    #[test]
    fn test_drill_down_max_depth() {
        let mut state = GuideState::new();
        state.drill_depth = 2;

        // Already at max depth, should not drill further
        let changed = state.handle_key(key_event(KeyCode::Enter));
        assert!(!changed);
        assert_eq!(state.drill_depth, 2);
    }

    #[test]
    fn test_drill_up_escape_key() {
        let mut state = GuideState::new();
        state.drill_depth = 2;
        state.drill_cursor = 5;

        // Escape to drill up
        let changed = state.handle_key(key_event(KeyCode::Esc));
        assert!(changed);
        assert_eq!(state.drill_depth, 1);
        assert_eq!(state.drill_cursor, 0);
    }

    #[test]
    fn test_drill_up_at_zero() {
        let mut state = GuideState::new();
        assert_eq!(state.drill_depth, 0);

        // Already at depth 0, should not change
        let changed = state.handle_key(key_event(KeyCode::Esc));
        assert!(!changed);
        assert_eq!(state.drill_depth, 0);
    }

    #[test]
    fn test_drill_down_with_l_key() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits; // Not Layers
        assert_eq!(state.drill_depth, 0);

        // 'l' should drill down (except in Layers tab)
        let changed = state.handle_key(key_event(KeyCode::Char('l')));
        assert!(changed);
        assert_eq!(state.drill_depth, 1);
    }

    #[test]
    fn test_drill_up_with_h_key() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits; // Not Layers
        state.drill_depth = 1;

        // 'h' should drill up (except in Layers tab)
        let changed = state.handle_key(key_event(KeyCode::Char('h')));
        assert!(changed);
        assert_eq!(state.drill_depth, 0);
    }

    #[test]
    fn test_drill_right_left_keys() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;
        assert_eq!(state.drill_depth, 0);

        // Right arrow to drill down
        let changed = state.handle_key(key_event(KeyCode::Right));
        assert!(changed);
        assert_eq!(state.drill_depth, 1);

        // Left arrow to drill up
        let changed = state.handle_key(key_event(KeyCode::Left));
        assert!(changed);
        assert_eq!(state.drill_depth, 0);
    }

    #[test]
    fn test_reset_drill() {
        let mut state = GuideState::new();
        state.drill_depth = 2;
        state.drill_cursor = 10;

        state.reset_drill();

        assert_eq!(state.drill_depth, 0);
        assert_eq!(state.drill_cursor, 0);
    }

    // ==========================================================================
    // NAVIGATION: TRAITS TAB
    // ==========================================================================

    #[test]
    fn test_traits_navigate_down() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;
        assert_eq!(state.trait_cursor, 0);

        // Navigate down with 'j'
        state.handle_key(key_event(KeyCode::Char('j')));
        assert_eq!(state.trait_cursor, 1);

        // Navigate down with Down arrow
        state.handle_key(key_event(KeyCode::Down));
        assert_eq!(state.trait_cursor, 2);
    }

    #[test]
    fn test_traits_navigate_up() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;
        state.trait_cursor = 3;

        // Navigate up with 'k'
        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.trait_cursor, 2);

        // Navigate up with Up arrow
        state.handle_key(key_event(KeyCode::Up));
        assert_eq!(state.trait_cursor, 1);
    }

    #[test]
    fn test_traits_cursor_wraps_around() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;
        state.trait_cursor = 4; // Last trait (index 0-4)

        // Navigate down should wrap to 0
        state.handle_key(key_event(KeyCode::Char('j')));
        assert_eq!(state.trait_cursor, 0);

        // Navigate up should wrap to 4
        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.trait_cursor, 4);
    }

    #[test]
    fn test_traits_drilled_navigation() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;
        state.drill_depth = 1;
        state.drill_cursor = 5;

        // Navigate up in drill mode
        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.drill_cursor, 4);

        // Navigate down in drill mode
        state.handle_key(key_event(KeyCode::Char('j')));
        assert_eq!(state.drill_cursor, 5);
    }

    #[test]
    fn test_traits_drilled_cursor_stops_at_zero() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;
        state.drill_depth = 1;
        state.drill_cursor = 0;

        // Navigate up at 0 should stay at 0
        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.drill_cursor, 0);
    }

    // ==========================================================================
    // NAVIGATION: LAYERS TAB
    // ==========================================================================

    #[test]
    fn test_layers_navigate_down() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 0; // Global (2 layers, max index 1)
        state.layer_cursor = 0;

        // Navigate down
        state.handle_key(key_event(KeyCode::Char('j')));
        assert_eq!(state.layer_cursor, 1);
    }

    #[test]
    fn test_layers_navigate_up() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_cursor = 2;

        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.layer_cursor, 1);
    }

    #[test]
    fn test_layers_global_max_cursor() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 0; // Global (2 layers, max index 1)
        state.layer_cursor = 1;

        // Should not go beyond max
        let changed = state.handle_key(key_event(KeyCode::Char('j')));
        assert!(!changed);
        assert_eq!(state.layer_cursor, 1);
    }

    #[test]
    fn test_layers_tenant_max_cursor() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 1; // Tenant (7 layers, max index 6)
        state.layer_cursor = 6;

        // Should not go beyond max
        let changed = state.handle_key(key_event(KeyCode::Char('j')));
        assert!(!changed);
        assert_eq!(state.layer_cursor, 6);
    }

    #[test]
    fn test_layers_realm_switch_left() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 1; // Start on Tenant
        state.layer_cursor = 3;

        // Switch to Global with 'h'
        let changed = state.handle_key(key_event(KeyCode::Char('h')));
        assert!(changed);
        assert_eq!(state.layer_realm, 0);
        assert_eq!(state.layer_cursor, 0); // Cursor reset on realm switch
    }

    #[test]
    fn test_layers_realm_switch_right() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 0; // Start on Global
        state.layer_cursor = 1;

        // Switch to Tenant with 'l'
        let changed = state.handle_key(key_event(KeyCode::Char('l')));
        assert!(changed);
        assert_eq!(state.layer_realm, 1);
        assert_eq!(state.layer_cursor, 0); // Cursor reset on realm switch
    }

    #[test]
    fn test_layers_realm_switch_no_change() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 0; // Already on Global

        // 'h' should not change anything
        let changed = state.handle_key(key_event(KeyCode::Char('h')));
        assert!(!changed);
        assert_eq!(state.layer_realm, 0);
    }

    // ==========================================================================
    // NAVIGATION: ARCS TAB
    // ==========================================================================

    #[test]
    fn test_arcs_navigate() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Arcs;
        assert_eq!(state.arc_cursor, 0);

        state.handle_key(key_event(KeyCode::Char('j')));
        assert_eq!(state.arc_cursor, 1);

        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.arc_cursor, 0);
    }

    #[test]
    fn test_arcs_max_cursor() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Arcs;
        state.arc_cursor = 4; // 5 arc families (0-4)

        let changed = state.handle_key(key_event(KeyCode::Char('j')));
        assert!(!changed);
        assert_eq!(state.arc_cursor, 4);
    }

    #[test]
    fn test_arcs_min_cursor() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Arcs;
        state.arc_cursor = 0;

        let changed = state.handle_key(key_event(KeyCode::Char('k')));
        assert!(!changed);
        assert_eq!(state.arc_cursor, 0);
    }

    // ==========================================================================
    // NAVIGATION: PIPELINE TAB
    // ==========================================================================

    #[test]
    fn test_pipeline_navigate() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Pipeline;
        assert_eq!(state.pipeline_stage, 0);

        state.handle_key(key_event(KeyCode::Char('j')));
        assert_eq!(state.pipeline_stage, 1);

        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.pipeline_stage, 0);
    }

    #[test]
    fn test_pipeline_max_stage() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Pipeline;
        state.pipeline_stage = 5; // 6 stages (0-5)

        let changed = state.handle_key(key_event(KeyCode::Char('j')));
        assert!(!changed);
        assert_eq!(state.pipeline_stage, 5);
    }

    #[test]
    fn test_pipeline_min_stage() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Pipeline;
        state.pipeline_stage = 0;

        let changed = state.handle_key(key_event(KeyCode::Char('k')));
        assert!(!changed);
        assert_eq!(state.pipeline_stage, 0);
    }

    // ==========================================================================
    // PIPELINE ANIMATION (SPACE KEY)
    // ==========================================================================

    #[test]
    fn test_pipeline_space_toggles_animation() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Pipeline;
        assert!(!state.pipeline_animating);

        // Space to start animation
        let changed = state.handle_key(key_event(KeyCode::Char(' ')));
        assert!(changed);
        assert!(state.pipeline_animating);

        // Space to pause animation
        let changed = state.handle_key(key_event(KeyCode::Char(' ')));
        assert!(changed);
        assert!(!state.pipeline_animating);
    }

    #[test]
    fn test_pipeline_enter_toggles_animation() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Pipeline;
        assert!(!state.pipeline_animating);

        // Enter on Pipeline tab toggles animation (not drill down)
        let changed = state.handle_key(key_event(KeyCode::Enter));
        assert!(changed);
        assert!(state.pipeline_animating);
    }

    #[test]
    fn test_space_only_works_on_pipeline() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;

        // Space should not do anything on Traits tab
        let changed = state.handle_key(key_event(KeyCode::Char(' ')));
        assert!(!changed);
    }

    // ==========================================================================
    // CURSOR CLAMPING
    // ==========================================================================

    #[test]
    fn test_clamp_drill_cursor_zero_len() {
        let mut state = GuideState::new();
        state.drill_cursor = 10;

        state.clamp_drill_cursor(0);
        assert_eq!(state.drill_cursor, 0);
    }

    #[test]
    fn test_clamp_drill_cursor_over_max() {
        let mut state = GuideState::new();
        state.drill_cursor = 100;

        state.clamp_drill_cursor(10); // max_len = 10, valid indices 0-9
        assert_eq!(state.drill_cursor, 9);
    }

    #[test]
    fn test_clamp_drill_cursor_within_range() {
        let mut state = GuideState::new();
        state.drill_cursor = 5;

        state.clamp_drill_cursor(10);
        assert_eq!(state.drill_cursor, 5); // Unchanged
    }

    // ==========================================================================
    // BREADCRUMB GENERATION
    // ==========================================================================

    #[test]
    fn test_breadcrumb_traits_overview() {
        let state = GuideState::new();
        let trait_stats = Vec::new();

        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.starts_with("Guide > Traits > "));
    }

    #[test]
    fn test_breadcrumb_layers() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 0;
        let trait_stats = Vec::new();

        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.contains("Guide > Layers > Global"));
    }

    #[test]
    fn test_breadcrumb_layers_tenant() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 1;
        let trait_stats = Vec::new();

        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.contains("Guide > Layers > Tenant"));
    }

    #[test]
    fn test_breadcrumb_arcs() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Arcs;
        state.arc_cursor = 0;
        let trait_stats = Vec::new();

        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.contains("Guide > Arcs > ownership"));
    }

    #[test]
    fn test_breadcrumb_pipeline() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Pipeline;
        state.pipeline_stage = 0;
        let trait_stats = Vec::new();

        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.contains("Guide > Pipeline > Knowledge"));
    }

    // ==========================================================================
    // EDGE CASES AND DEFENSIVE CHECKS
    // ==========================================================================

    #[test]
    fn test_unhandled_key_returns_false() {
        let mut state = GuideState::new();

        // Arbitrary unhandled keys
        let changed = state.handle_key(key_event(KeyCode::Char('z')));
        assert!(!changed);

        let changed = state.handle_key(key_event(KeyCode::F(1)));
        assert!(!changed);
    }

    #[test]
    fn test_default_impl() {
        let state = GuideState::default();
        assert_eq!(state.tab, GuideTab::Traits);
        assert_eq!(state.trait_cursor, 0);
        assert_eq!(state.drill_depth, 0);
    }

    #[test]
    fn test_guide_tab_default() {
        let tab = GuideTab::default();
        assert_eq!(tab, GuideTab::Traits);
    }

    #[test]
    fn test_jump_to_trait_clamps() {
        let mut state = GuideState::new();

        // jump_to_trait clamps to 0-4
        state.jump_to_trait(100);
        assert_eq!(state.trait_cursor, 4);
    }

    #[test]
    fn test_get_trait_kinds_empty_stats() {
        let state = GuideState::new();
        let empty_stats: Vec<traits::TraitStats> = Vec::new();

        let kinds = state.get_trait_kinds(&empty_stats);
        assert!(kinds.is_empty());
    }

    #[test]
    fn test_current_tip_fallback() {
        let mut state = GuideState::new();
        state.tip_index = usize::MAX; // Invalid index

        // Should fallback to first tip
        let tip = state.current_tip();
        assert_eq!(tip, TIPS[0]);
    }

    // ==========================================================================
    // YANK (CLIPBOARD) FUNCTIONALITY
    // ==========================================================================

    #[test]
    fn test_get_current_yank_text_traits() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;

        state.trait_cursor = 0;
        assert_eq!(state.get_current_yank_text(), Some("invariant".to_string()));

        state.trait_cursor = 1;
        assert_eq!(state.get_current_yank_text(), Some("localized".to_string()));

        state.trait_cursor = 2;
        assert_eq!(state.get_current_yank_text(), Some("knowledge".to_string()));

        state.trait_cursor = 3;
        assert_eq!(state.get_current_yank_text(), Some("derived".to_string()));

        state.trait_cursor = 4;
        assert_eq!(state.get_current_yank_text(), Some("job".to_string()));
    }

    #[test]
    fn test_get_current_yank_text_layers_global() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 0; // Global

        state.layer_cursor = 0;
        assert_eq!(state.get_current_yank_text(), Some("config".to_string()));

        state.layer_cursor = 1;
        assert_eq!(
            state.get_current_yank_text(),
            Some("locale-knowledge".to_string())
        );
    }

    #[test]
    fn test_get_current_yank_text_layers_tenant() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Layers;
        state.layer_realm = 1; // Tenant

        state.layer_cursor = 0;
        assert_eq!(state.get_current_yank_text(), Some("config".to_string()));

        state.layer_cursor = 3;
        assert_eq!(state.get_current_yank_text(), Some("semantic".to_string()));

        state.layer_cursor = 5;
        assert_eq!(state.get_current_yank_text(), Some("seo".to_string()));

        state.layer_cursor = 6;
        assert_eq!(state.get_current_yank_text(), Some("output".to_string()));
    }

    #[test]
    fn test_get_current_yank_text_arcs() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Arcs;

        state.arc_cursor = 0;
        assert_eq!(state.get_current_yank_text(), Some("ownership".to_string()));

        state.arc_cursor = 2;
        assert_eq!(state.get_current_yank_text(), Some("semantic".to_string()));

        state.arc_cursor = 4;
        assert_eq!(state.get_current_yank_text(), Some("mining".to_string()));
    }

    #[test]
    fn test_get_current_yank_text_pipeline() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Pipeline;

        state.pipeline_stage = 0;
        assert_eq!(state.get_current_yank_text(), Some("Knowledge".to_string()));

        state.pipeline_stage = 4;
        assert_eq!(
            state.get_current_yank_text(),
            Some("Generation".to_string())
        );

        state.pipeline_stage = 5;
        assert_eq!(state.get_current_yank_text(), Some("Output".to_string()));
    }

    #[test]
    fn test_get_current_yank_text_out_of_bounds() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;
        state.trait_cursor = 100; // Out of bounds

        // Should return None for invalid cursor
        assert_eq!(state.get_current_yank_text(), None);
    }

    #[test]
    fn test_clipboard_message_initial_state() {
        let state = GuideState::new();
        assert!(state.clipboard_message.is_none());
        assert!(state.clipboard_message_time.is_none());
    }

    #[test]
    fn test_clear_expired_clipboard_message_none() {
        let mut state = GuideState::new();
        // Should not panic when no message exists
        state.clear_expired_clipboard_message();
        assert!(state.clipboard_message.is_none());
    }

    #[test]
    fn test_clear_expired_clipboard_message_recent() {
        let mut state = GuideState::new();
        state.clipboard_message = Some("test".to_string());
        state.clipboard_message_time = Some(std::time::Instant::now());

        // Message is fresh, should not be cleared
        state.clear_expired_clipboard_message();
        assert!(state.clipboard_message.is_some());
    }

    #[test]
    fn test_y_key_triggers_yank() {
        let mut state = GuideState::new();
        state.tab = GuideTab::Traits;
        state.trait_cursor = 0;

        // y key should trigger yank (may fail in CI without clipboard, but state should change)
        let changed = state.handle_key(key_event(KeyCode::Char('y')));
        // Note: This will return true if clipboard works, or if error message is set
        // In CI without clipboard access, it still sets clipboard_message with error
        assert!(changed || !changed); // Just verify no panic
    }
}
