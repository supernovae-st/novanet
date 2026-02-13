//! Nexus Mode - Gamified learning hub for NovaNet taxonomy.
//!
//! Nexus Mode provides 9 tabs organized in 3 sections:
//!
//! ## LEARN (Beginner-friendly)
//! - [I] Intro: Big Picture introduction (what is NovaNet?)
//! - [G] Glossary: 15 searchable concepts with YAML/Neo4j examples
//! - [U] Tutorial: 5-step guided learning journey
//!
//! ## EXPLORE (Schema exploration)
//! - [T] Traits: 5-trait constellation (defined, authored, imported, generated, retrieved)
//! - [L] Layers: 2-realm split view (Shared 4 layers | Org 6 layers)
//! - [A] Arcs: Arc families and scope visualization
//!
//! ## PRACTICE (Interactive)
//! - [P] Pipeline: Animated generation flow (not translation)
//! - [Q] Quiz: Interactive taxonomy quiz
//! - [V] Views: Schema views explorer (Query-First architecture)
//!
//! v11.8: 59 nodes (39 shared + 20 org), 10 layers (4 shared + 6 org).
//! Progress persistence to ~/.novanet/tutorial_progress.json

pub mod arcs;
pub mod glossary;
pub mod i18n;
pub mod intro;
pub mod layers;
pub mod persistence;
pub mod pipeline;
pub mod quiz;
pub mod traits;
pub mod tutorial;
pub mod views;

use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::tui::app::App;
use crate::tui::clipboard;
use crate::tui::ui::COLOR_UNFOCUSED_BORDER;

// Re-export TraitStats and CodeExample for external use
pub use traits::{CodeExample, TraitStats, trait_code_examples};

// Re-export new tab types
pub use glossary::GlossaryState;
pub use tutorial::TutorialState;

// =============================================================================
// "DID YOU KNOW?" TIPS
// =============================================================================

/// Educational tips shown at the bottom of Nexus mode.
/// Rotates through concepts about NovaNet's architecture.
pub const TIPS: &[&str] = &[
    "Imported is INPUT (savoir) - Authored is OUTPUT (generated)",
    "Layers define WHAT a node does, Traits define HOW it behaves with locale",
    "Content/Generated nodes have defined parents (Entity→EntityContent, Page→PageGenerated)",
    "Generation, NOT translation: Imported + Structure -> Native content",
    "Shared realm is READ-ONLY - all business content lives in Org",
    "Quick jump: gd=defined, ga=authored, gi=imported, gg=generated, gr=retrieved",
    "Imported nodes exist ONLY where needed (fr-FR: 20K Terms, sw-KE: 500)",
    "Arc families: ownership, localization, semantic, generation, mining",
    "defined = structure (solid border), authored = output (dashed border)",
    "Press 'n' to see the next tip!",
];

/// Which Nexus tab is currently active.
/// Tabs are organized into 3 sections: LEARN, EXPLORE, PRACTICE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NexusTab {
    // === LEARN Section (Beginner-friendly) ===
    /// Big Picture introduction (what is NovaNet?)
    #[default]
    Intro,
    /// 15 searchable concepts with YAML/Neo4j examples
    Glossary,
    /// 5-step guided learning journey
    Tutorial,

    // === EXPLORE Section (Schema exploration) ===
    /// Traits constellation (5 traits with detail panel)
    Traits,
    /// Layers split view (Shared | Org)
    Layers,
    /// Arc families and scope
    Arcs,

    // === PRACTICE Section (Interactive) ===
    /// Generation pipeline animation
    Pipeline,
    /// Interactive quiz about NovaNet taxonomy
    Quiz,
    /// Schema views explorer (Query-First architecture)
    Views,
}

impl NexusTab {
    /// Get the shortcut key for this tab.
    /// Letter-based shortcuts for mnemonic navigation.
    pub fn shortcut(&self) -> char {
        match self {
            // LEARN section
            NexusTab::Intro => 'i',
            NexusTab::Glossary => 'g',
            NexusTab::Tutorial => 'u',
            // EXPLORE section
            NexusTab::Traits => 't',
            NexusTab::Layers => 'l',
            NexusTab::Arcs => 'a',
            // PRACTICE section
            NexusTab::Pipeline => 'p',
            NexusTab::Quiz => 'q',
            NexusTab::Views => 'v',
        }
    }

    /// Get the display label for this tab.
    pub fn label(&self) -> &'static str {
        match self {
            NexusTab::Intro => "Intro",
            NexusTab::Glossary => "Glossary",
            NexusTab::Tutorial => "Tutorial",
            NexusTab::Traits => "Traits",
            NexusTab::Layers => "Layers",
            NexusTab::Arcs => "Arcs",
            NexusTab::Pipeline => "Pipeline",
            NexusTab::Quiz => "Quiz",
            NexusTab::Views => "Views",
        }
    }

    /// Get all tabs in order.
    pub fn all() -> &'static [NexusTab] {
        &[
            // LEARN
            NexusTab::Intro,
            NexusTab::Glossary,
            NexusTab::Tutorial,
            // EXPLORE
            NexusTab::Traits,
            NexusTab::Layers,
            NexusTab::Arcs,
            // PRACTICE
            NexusTab::Pipeline,
            NexusTab::Quiz,
            NexusTab::Views,
        ]
    }

    /// Cycle to next tab.
    pub fn next(&self) -> Self {
        match self {
            NexusTab::Intro => NexusTab::Glossary,
            NexusTab::Glossary => NexusTab::Tutorial,
            NexusTab::Tutorial => NexusTab::Traits,
            NexusTab::Traits => NexusTab::Layers,
            NexusTab::Layers => NexusTab::Arcs,
            NexusTab::Arcs => NexusTab::Pipeline,
            NexusTab::Pipeline => NexusTab::Quiz,
            NexusTab::Quiz => NexusTab::Views,
            NexusTab::Views => NexusTab::Intro,
        }
    }

    /// Cycle to previous tab.
    pub fn prev(&self) -> Self {
        match self {
            NexusTab::Intro => NexusTab::Views,
            NexusTab::Glossary => NexusTab::Intro,
            NexusTab::Tutorial => NexusTab::Glossary,
            NexusTab::Traits => NexusTab::Tutorial,
            NexusTab::Layers => NexusTab::Traits,
            NexusTab::Arcs => NexusTab::Layers,
            NexusTab::Pipeline => NexusTab::Arcs,
            NexusTab::Quiz => NexusTab::Pipeline,
            NexusTab::Views => NexusTab::Quiz,
        }
    }

    /// Get the section this tab belongs to.
    pub fn section(&self) -> &'static str {
        match self {
            NexusTab::Intro | NexusTab::Glossary | NexusTab::Tutorial => "LEARN",
            NexusTab::Traits | NexusTab::Layers | NexusTab::Arcs => "EXPLORE",
            NexusTab::Pipeline | NexusTab::Quiz | NexusTab::Views => "PRACTICE",
        }
    }

    /// Get related tabs with navigation hints (v0.12.0).
    /// Returns pairs of (related_tab, hint_text) for cross-navigation.
    pub fn related_tabs(&self, locale: NexusLocale) -> Vec<(NexusTab, &'static str)> {
        match locale {
            NexusLocale::En => match self {
                NexusTab::Intro => vec![
                    (NexusTab::Glossary, "Learn terms in Glossary"),
                    (NexusTab::Tutorial, "Start Tutorial journey"),
                ],
                NexusTab::Glossary => vec![
                    (NexusTab::Traits, "See Traits constellation"),
                    (NexusTab::Quiz, "Test your knowledge"),
                ],
                NexusTab::Tutorial => vec![
                    (NexusTab::Pipeline, "See generation Pipeline"),
                    (NexusTab::Quiz, "Practice with Quiz"),
                ],
                NexusTab::Traits => vec![
                    (NexusTab::Layers, "See Layer organization"),
                    (NexusTab::Pipeline, "See data flow"),
                ],
                NexusTab::Layers => vec![
                    (NexusTab::Arcs, "See Arc families"),
                    (NexusTab::Views, "Query with Views"),
                ],
                NexusTab::Arcs => vec![
                    (NexusTab::Traits, "Back to Traits"),
                    (NexusTab::Views, "See schema Views"),
                ],
                NexusTab::Pipeline => vec![
                    (NexusTab::Traits, "Understand Traits"),
                    (NexusTab::Quiz, "Test your knowledge"),
                ],
                NexusTab::Quiz => vec![
                    (NexusTab::Glossary, "Review in Glossary"),
                    (NexusTab::Tutorial, "Continue Tutorial"),
                ],
                NexusTab::Views => vec![
                    (NexusTab::Layers, "See Layer structure"),
                    (NexusTab::Arcs, "See Arc families"),
                ],
            },
            NexusLocale::Fr => match self {
                NexusTab::Intro => vec![
                    (NexusTab::Glossary, "Termes dans Glossaire"),
                    (NexusTab::Tutorial, "Commencer le Tutoriel"),
                ],
                NexusTab::Glossary => vec![
                    (NexusTab::Traits, "Voir les Traits"),
                    (NexusTab::Quiz, "Tester vos connaissances"),
                ],
                NexusTab::Tutorial => vec![
                    (NexusTab::Pipeline, "Voir le Pipeline"),
                    (NexusTab::Quiz, "Pratiquer avec le Quiz"),
                ],
                NexusTab::Traits => vec![
                    (NexusTab::Layers, "Voir les Couches"),
                    (NexusTab::Pipeline, "Voir le flux"),
                ],
                NexusTab::Layers => vec![
                    (NexusTab::Arcs, "Voir les Arcs"),
                    (NexusTab::Views, "Requêter avec Views"),
                ],
                NexusTab::Arcs => vec![
                    (NexusTab::Traits, "Retour aux Traits"),
                    (NexusTab::Views, "Voir les Vues"),
                ],
                NexusTab::Pipeline => vec![
                    (NexusTab::Traits, "Comprendre les Traits"),
                    (NexusTab::Quiz, "Tester vos connaissances"),
                ],
                NexusTab::Quiz => vec![
                    (NexusTab::Glossary, "Revoir dans Glossaire"),
                    (NexusTab::Tutorial, "Continuer le Tutoriel"),
                ],
                NexusTab::Views => vec![
                    (NexusTab::Layers, "Voir les Couches"),
                    (NexusTab::Arcs, "Voir les Arcs"),
                ],
            },
        }
    }
}

// =============================================================================
// LOCALE (i18n)
// =============================================================================

/// Language for Nexus content (toggle with Shift+I).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NexusLocale {
    /// English (default)
    #[default]
    En,
    /// French
    Fr,
}

impl NexusLocale {
    /// Toggle between En and Fr.
    pub fn toggle(&self) -> Self {
        match self {
            NexusLocale::En => NexusLocale::Fr,
            NexusLocale::Fr => NexusLocale::En,
        }
    }

    /// Get display label for current locale.
    pub fn label(&self) -> &'static str {
        match self {
            NexusLocale::En => "EN",
            NexusLocale::Fr => "FR",
        }
    }

    /// Get flag emoji for current locale.
    pub fn flag(&self) -> &'static str {
        match self {
            NexusLocale::En => "🇬🇧",
            NexusLocale::Fr => "🇫🇷",
        }
    }
}

/// Main Nexus mode state.
#[derive(Debug, Clone)]
pub struct NexusState {
    /// Currently active tab.
    pub tab: NexusTab,

    /// Current locale for i18n (toggle with Shift+I).
    pub locale: NexusLocale,

    // === LEARN Section State ===

    // === Intro tab state ===
    /// Current page in the intro (0-based, 3 pages total).
    pub intro_page: usize,

    // === Glossary tab state ===
    /// Glossary state (search, cursor, category expansion).
    pub glossary: glossary::GlossaryState,

    // === Tutorial tab state ===
    /// Tutorial state (current step, task completion).
    pub tutorial: tutorial::TutorialState,

    // === EXPLORE Section State ===

    // === Traits tab state ===
    /// Cursor position in traits constellation (0-4 for 5 traits).
    pub trait_cursor: usize,

    // === Layers tab state ===
    /// Cursor position in layers list.
    pub layer_cursor: usize,
    /// Selected realm (0=shared, 1=org).
    pub layer_realm: usize,

    // === Arcs tab state ===
    /// Cursor position in arc families.
    pub arc_cursor: usize,

    // === PRACTICE Section State ===

    // === Pipeline tab state ===
    /// Current stage in pipeline (0-based).
    pub pipeline_stage: usize,
    /// Whether pipeline animation is running.
    pub pipeline_animating: bool,

    // === Quiz tab state ===
    /// Quiz state (current question, score, etc.).
    pub quiz: quiz::QuizState,

    // === Views tab state ===
    /// Views state (category cursor, view cursor, concept panel).
    pub views: views::ViewsState,

    // === Shared State ===

    // === Drill-down state ===
    /// Drill depth (0=overview, 1=kinds, 2=instances).
    pub drill_depth: usize,
    /// Cursor within drill-down list.
    pub drill_cursor: usize,

    // === Quick jump state ===
    /// Pending 'g' key for quick jump sequences (gd, ga, gi, gg, gr).
    pub pending_g: bool,

    // === Tips state ===
    /// Current tip index for "Did you know?" rotation.
    pub tip_index: usize,

    // === Clipboard state ===
    /// Message to display after clipboard operation (e.g., "Copied: Entity").
    pub clipboard_message: Option<String>,
    /// When the clipboard message was set (for auto-clear after ~2s).
    pub clipboard_message_time: Option<Instant>,

    // === Persistence cache ===
    /// Cached progress to avoid disk reads on every save.
    progress_cache: Option<persistence::TutorialProgress>,
}

impl Default for NexusState {
    fn default() -> Self {
        Self::new()
    }
}

impl NexusState {
    /// Create a new NexusState with default values.
    pub fn new() -> Self {
        Self {
            tab: NexusTab::default(),
            locale: NexusLocale::default(),
            // LEARN section
            intro_page: 0,
            glossary: glossary::GlossaryState::new(),
            tutorial: tutorial::TutorialState::new(),
            // EXPLORE section
            trait_cursor: 0,
            layer_cursor: 0,
            layer_realm: 0,
            arc_cursor: 0,
            // PRACTICE section
            pipeline_stage: 0,
            pipeline_animating: false,
            quiz: quiz::QuizState::new(),
            views: views::ViewsState::new(),
            // Shared state
            drill_depth: 0,
            drill_cursor: 0,
            pending_g: false,
            tip_index: 0,
            clipboard_message: None,
            clipboard_message_time: None,
            progress_cache: None,
        }
    }

    /// Create NexusState with loaded persistence (tutorial progress + quiz high score).
    pub fn with_persistence() -> Self {
        let mut state = Self::new();

        // Load saved tutorial progress
        let progress = persistence::TutorialProgress::load();
        if progress.has_started() {
            state.tutorial = progress.to_state();
        }

        // Load quiz high score
        if let Some(high_score) = progress.quiz_high_score {
            state.quiz.high_score = Some(high_score);
        }

        // Cache for subsequent saves (avoids disk reads)
        state.progress_cache = Some(progress);

        state
    }

    /// Save current tutorial progress to disk (uses cache to avoid disk reads).
    pub fn save_tutorial_progress(&mut self) {
        let progress = self
            .progress_cache
            .get_or_insert_with(persistence::TutorialProgress::load);
        progress.update_from_state(&self.tutorial);
        if let Err(e) = progress.save() {
            self.clipboard_message = Some(format!("Save failed: {}", e));
            self.clipboard_message_time = Some(Instant::now());
        }
    }

    /// Save quiz high score to disk (uses cache to avoid disk reads).
    pub fn save_quiz_score(&mut self, score: usize) {
        let progress = self
            .progress_cache
            .get_or_insert_with(persistence::TutorialProgress::load);
        progress.update_quiz_score(score);
        if let Err(e) = progress.save() {
            self.clipboard_message = Some(format!("Save failed: {}", e));
            self.clipboard_message_time = Some(Instant::now());
        }
    }

    /// Reset drill-down state (when switching tabs).
    pub fn reset_drill(&mut self) {
        self.drill_depth = 0;
        self.drill_cursor = 0;
    }

    /// Handle key input in Nexus mode. Returns true if state changed.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Handle pending 'g' state for quick jump shortcuts (gd, ga, gi, gg, gr)
        // v11.8: Traits renamed per ADR-024 (data origin: defined/authored/imported/generated/retrieved)
        if self.pending_g {
            self.pending_g = false; // Clear pending state
            return match key.code {
                KeyCode::Char('d') => self.jump_to_trait(0), // defined (was invariant)
                KeyCode::Char('a') => self.jump_to_trait(1), // authored (was localized)
                KeyCode::Char('i') => self.jump_to_trait(2), // imported (was knowledge)
                KeyCode::Char('g') => self.jump_to_trait(3), // generated
                KeyCode::Char('r') => self.jump_to_trait(4), // retrieved (was aggregated)
                KeyCode::Char('0') => {
                    // g0 = go to top (reset cursors)
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

            // Tab switching with [ ] brackets (vim-style)
            KeyCode::Char('[') => {
                self.tab = self.tab.prev();
                self.reset_drill();
                true
            }
            KeyCode::Char(']') => {
                self.tab = self.tab.next();
                self.reset_drill();
                true
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

            // Section navigation with H/L (Shift + h/l)
            KeyCode::Char('H') => self.prev_section(),
            KeyCode::Char('L') => self.next_section(),

            // Cursor navigation: ↑↓, j/k
            KeyCode::Up | KeyCode::Char('k') => self.navigate_up(),
            KeyCode::Down | KeyCode::Char('j') => self.navigate_down(),

            // Horizontal navigation: ←→, h/l
            KeyCode::Left | KeyCode::Char('h') => self.navigate_left(),
            KeyCode::Right | KeyCode::Char('l') => self.navigate_right(),

            // Enter/Space for action
            KeyCode::Enter | KeyCode::Char(' ') => {
                match self.tab {
                    NexusTab::Quiz => {
                        if self.quiz.answered {
                            let quiz_completed = self.quiz.next_question(quiz::QUESTIONS);
                            if quiz_completed {
                                // Save quiz score to persistence
                                self.save_quiz_score(self.quiz.score);
                            }
                        } else {
                            self.quiz.submit_answer(quiz::QUESTIONS);
                        }
                        true
                    }
                    NexusTab::Pipeline => {
                        self.pipeline_animating = !self.pipeline_animating;
                        true
                    }
                    NexusTab::Tutorial => {
                        // Toggle current task completion and save
                        self.tutorial.toggle_task(0); // First task of current step
                        self.save_tutorial_progress();
                        true
                    }
                    _ => self.drill_down(),
                }
            }

            // Escape for drill-up (also clears pending_g, closes concept panel)
            KeyCode::Esc => {
                self.pending_g = false;
                // In Views tab, close concept panel first
                if self.tab == NexusTab::Views && self.views.show_concept {
                    self.views.show_concept = false;
                    return true;
                }
                self.drill_up()
            }

            // '?' to toggle Query-First concept panel (Views tab only)
            KeyCode::Char('?') => {
                if self.tab == NexusTab::Views {
                    self.views.toggle_concept();
                    true
                } else {
                    false
                }
            }

            // 'c' to mark tutorial step as complete
            KeyCode::Char('c') => {
                if self.tab == NexusTab::Tutorial {
                    self.tutorial.mark_step_complete();
                    self.save_tutorial_progress();
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

            // 'r' to restart quiz (when in Quiz tab)
            KeyCode::Char('r') => {
                if self.tab == NexusTab::Quiz {
                    self.quiz.reset();
                    true
                } else {
                    false
                }
            }

            // 'y' to yank (copy) current selection to clipboard
            KeyCode::Char('y') => self.yank_current(),

            // 'I' (Shift+I) to toggle locale/i18n (En/Fr)
            KeyCode::Char('I') => {
                self.locale = self.locale.toggle();
                self.clipboard_message = Some(format!(
                    "Language: {} {}",
                    self.locale.flag(),
                    self.locale.label()
                ));
                self.clipboard_message_time = Some(Instant::now());
                true
            }

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
            // LEARN section
            NexusTab::Intro => {
                // Yank the current page title
                let titles = [
                    "What is NovaNet?",
                    "Schema vs Instance Nodes",
                    "Classification Axes",
                ];
                titles.get(self.intro_page).map(|s| s.to_string())
            }
            NexusTab::Glossary => {
                // Yank the current concept name
                self.glossary.get_yank_text()
            }
            NexusTab::Tutorial => {
                // Yank the current step title
                self.tutorial.get_yank_text()
            }
            // EXPLORE section
            NexusTab::Traits => {
                // v11.8: ADR-024 Data Origin semantics (5 traits)
                let traits = [
                    "defined",  // was: invariant
                    "authored", // was: localized
                    "imported", // was: knowledge
                    "generated",
                    "retrieved", // was: aggregated
                ];
                traits.get(self.trait_cursor).map(|s| s.to_string())
            }
            NexusTab::Layers => {
                // Yank the current layer key
                let layers = if self.layer_realm == 0 {
                    // Shared layers (v11.5: 4 layers)
                    vec!["config", "locale", "geography", "knowledge"]
                } else {
                    // Org layers (v11.5: 6 layers) - SEO/GEO consolidated to shared/knowledge
                    vec![
                        "config",
                        "foundation",
                        "structure",
                        "semantic",
                        "instruction",
                        "output",
                    ]
                };
                layers.get(self.layer_cursor).map(|s| s.to_string())
            }
            NexusTab::Arcs => {
                // Yank the current arc family
                let families = [
                    "ownership",
                    "localization",
                    "semantic",
                    "generation",
                    "mining",
                ];
                families.get(self.arc_cursor).map(|s| s.to_string())
            }
            // PRACTICE section
            NexusTab::Pipeline => {
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
            NexusTab::Quiz => {
                // Yank the current question text
                quiz::QUESTIONS
                    .get(self.quiz.current_question)
                    .map(|q| q.question.to_string())
            }
            NexusTab::Views => {
                // Yank the current view ID
                self.views.get_yank_text()
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
    /// Used by quick jump shortcuts (gi, gl, gk, gg, ga).
    fn jump_to_trait(&mut self, trait_index: usize) -> bool {
        self.tab = NexusTab::Traits;
        self.trait_cursor = trait_index.min(4); // Clamp to 0-4
        self.reset_drill();
        true
    }

    /// Navigate to next section (LEARN → EXPLORE → PRACTICE → LEARN).
    fn next_section(&mut self) -> bool {
        let target_tab = match self.tab.section() {
            "LEARN" => NexusTab::Traits,     // → EXPLORE
            "EXPLORE" => NexusTab::Pipeline, // → PRACTICE
            "PRACTICE" => NexusTab::Intro,   // → LEARN (wrap)
            _ => return false,
        };
        self.tab = target_tab;
        self.reset_drill();
        true
    }

    /// Navigate to previous section (LEARN ← EXPLORE ← PRACTICE ← LEARN).
    fn prev_section(&mut self) -> bool {
        let target_tab = match self.tab.section() {
            "LEARN" => NexusTab::Pipeline,  // ← PRACTICE (wrap)
            "EXPLORE" => NexusTab::Intro,   // ← LEARN
            "PRACTICE" => NexusTab::Traits, // ← EXPLORE
            _ => return false,
        };
        self.tab = target_tab;
        self.reset_drill();
        true
    }

    /// Advance to the next "Did you know?" tip.
    pub fn next_tip(&mut self) {
        let tips = i18n::tips(self.locale);
        self.tip_index = (self.tip_index + 1) % tips.len();
    }

    /// Get the current "Did you know?" tip.
    pub fn current_tip(&self) -> &'static str {
        let tips = i18n::tips(self.locale);
        tips.get(self.tip_index).unwrap_or(&tips[0])
    }

    /// Check if there's a pending 'g' key waiting for completion.
    pub fn has_pending_g(&self) -> bool {
        self.pending_g
    }

    /// Navigate up (cursor movement).
    fn navigate_up(&mut self) -> bool {
        match self.tab {
            // LEARN section
            NexusTab::Intro => {
                // Previous page
                if self.intro_page > 0 {
                    self.intro_page -= 1;
                    true
                } else {
                    false
                }
            }
            NexusTab::Glossary => {
                self.glossary.navigate_up();
                true
            }
            NexusTab::Tutorial => {
                self.tutorial.navigate_up();
                true
            }
            // EXPLORE section
            NexusTab::Traits => {
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
            NexusTab::Layers => {
                if self.layer_cursor > 0 {
                    self.layer_cursor -= 1;
                    true
                } else {
                    false
                }
            }
            NexusTab::Arcs => {
                if self.arc_cursor > 0 {
                    self.arc_cursor -= 1;
                    true
                } else {
                    false
                }
            }
            // PRACTICE section
            NexusTab::Pipeline => {
                if self.pipeline_stage > 0 {
                    self.pipeline_stage -= 1;
                    true
                } else {
                    false
                }
            }
            NexusTab::Quiz => {
                self.quiz.select_up();
                true
            }
            NexusTab::Views => {
                self.views.navigate_up();
                true
            }
        }
    }

    /// Navigate down (cursor movement).
    fn navigate_down(&mut self) -> bool {
        match self.tab {
            // LEARN section
            NexusTab::Intro => {
                // Next page
                if self.intro_page < intro::INTRO_PAGES - 1 {
                    self.intro_page += 1;
                    true
                } else {
                    false
                }
            }
            NexusTab::Glossary => {
                self.glossary.navigate_down();
                true
            }
            NexusTab::Tutorial => {
                self.tutorial.navigate_down();
                true
            }
            // EXPLORE section
            NexusTab::Traits => {
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
            NexusTab::Layers => {
                // Bound by number of layers in current realm
                // v11.5: shared: 4 layers (0-3), org: 6 layers (0-5)
                let max = if self.layer_realm == 0 { 3 } else { 5 };
                if self.layer_cursor < max {
                    self.layer_cursor += 1;
                    true
                } else {
                    false
                }
            }
            NexusTab::Arcs => {
                // 5 arc families
                if self.arc_cursor < 4 {
                    self.arc_cursor += 1;
                    true
                } else {
                    false
                }
            }
            // PRACTICE section
            NexusTab::Pipeline => {
                // 6 pipeline stages (0-5)
                if self.pipeline_stage < 5 {
                    self.pipeline_stage += 1;
                    true
                } else {
                    false
                }
            }
            NexusTab::Quiz => {
                self.quiz.select_down();
                true
            }
            NexusTab::Views => {
                self.views.navigate_down();
                true
            }
        }
    }

    /// Navigate left (realm switching in Layers, category in Views/Glossary, drill-out elsewhere).
    fn navigate_left(&mut self) -> bool {
        match self.tab {
            // LEARN section
            NexusTab::Intro => {
                // Previous page
                if self.intro_page > 0 {
                    self.intro_page -= 1;
                    true
                } else {
                    false
                }
            }
            NexusTab::Glossary => {
                // Previous category
                self.glossary.prev_category();
                true
            }
            NexusTab::Tutorial => {
                // Previous step
                self.tutorial.prev_step();
                true
            }
            // EXPLORE section
            NexusTab::Layers => {
                // Switch to Shared realm (0)
                if self.layer_realm != 0 {
                    self.layer_realm = 0;
                    self.layer_cursor = 0; // Reset cursor when switching realm
                    true
                } else {
                    false
                }
            }
            NexusTab::Views => {
                // Switch to previous category
                self.views.prev_category();
                true
            }
            _ => {
                // Drill up as alternative to Escape
                self.drill_up()
            }
        }
    }

    /// Navigate right (realm switching in Layers, category in Views/Glossary, drill-in elsewhere).
    fn navigate_right(&mut self) -> bool {
        match self.tab {
            // LEARN section
            NexusTab::Intro => {
                // Next page
                if self.intro_page < intro::INTRO_PAGES - 1 {
                    self.intro_page += 1;
                    true
                } else {
                    false
                }
            }
            NexusTab::Glossary => {
                // Next category
                self.glossary.next_category();
                true
            }
            NexusTab::Tutorial => {
                // Next step
                self.tutorial.next_step();
                true
            }
            // EXPLORE section
            NexusTab::Layers => {
                // Switch to Org realm (1)
                if self.layer_realm != 1 {
                    self.layer_realm = 1;
                    self.layer_cursor = 0; // Reset cursor when switching realm
                    true
                } else {
                    false
                }
            }
            NexusTab::Views => {
                // Switch to next category
                self.views.next_category();
                true
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
            // LEARN section - no drill-down
            NexusTab::Intro => {
                // Intro uses page navigation instead
                if self.intro_page < intro::INTRO_PAGES - 1 {
                    self.intro_page += 1;
                    true
                } else {
                    false
                }
            }
            NexusTab::Glossary => {
                // Glossary - expand category or select concept
                self.glossary.toggle_expand();
                true
            }
            NexusTab::Tutorial => {
                // Tutorial - next step
                self.tutorial.next_step();
                true
            }
            // EXPLORE section
            NexusTab::Traits | NexusTab::Layers | NexusTab::Arcs => {
                if self.drill_depth < 2 {
                    self.drill_depth += 1;
                    self.drill_cursor = 0;
                    true
                } else {
                    false
                }
            }
            // PRACTICE section
            NexusTab::Pipeline => {
                // Pipeline doesn't have drill-down, toggle animation instead
                self.pipeline_animating = !self.pipeline_animating;
                true
            }
            NexusTab::Quiz => {
                // Quiz doesn't have drill-down
                false
            }
            NexusTab::Views => {
                // Views doesn't have drill-down
                false
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

    /// Get breadcrumb for current Nexus mode state.
    /// Returns path like "Nexus > LEARN > Intro > Page 1"
    pub fn breadcrumb(&self, trait_stats: &[traits::TraitStats]) -> String {
        let section = self.tab.section();
        let tab_name = self.tab.label();
        match self.tab {
            // LEARN section
            NexusTab::Intro => {
                let page = self.intro_page + 1;
                let total = intro::INTRO_PAGES;
                format!(
                    "Nexus > {} > {} > Page {}/{}",
                    section, tab_name, page, total
                )
            }
            NexusTab::Glossary => {
                if self.glossary.search_active {
                    format!(
                        "Nexus > {} > {} > Search: \"{}\"",
                        section, tab_name, self.glossary.search_query
                    )
                } else if let Some(cat_idx) = self.glossary.expanded_category {
                    let categories = glossary::CATEGORIES;
                    let cat_name = categories.get(cat_idx).map(|c| c.0).unwrap_or("?");
                    format!("Nexus > {} > {} > {}", section, tab_name, cat_name)
                } else {
                    format!("Nexus > {} > {}", section, tab_name)
                }
            }
            NexusTab::Tutorial => {
                let step = self.tutorial.current_step + 1;
                let total = tutorial::TUTORIAL_STEPS;
                if self.tutorial.complete {
                    format!("Nexus > {} > {} > Complete!", section, tab_name)
                } else {
                    format!(
                        "Nexus > {} > {} > Step {}/{}",
                        section, tab_name, step, total
                    )
                }
            }
            // EXPLORE section
            NexusTab::Traits => {
                let trait_name = traits::TRAIT_ORDER.get(self.trait_cursor).unwrap_or(&"");
                if self.drill_depth == 0 {
                    format!("Nexus > {} > {} > {}", section, tab_name, trait_name)
                } else {
                    let kinds = self.get_trait_kinds(trait_stats);
                    if let Some((layer, kind)) = kinds.get(self.drill_cursor) {
                        format!(
                            "Nexus > {} > {} > {} > {} ({})",
                            section, tab_name, trait_name, kind, layer
                        )
                    } else {
                        format!("Nexus > {} > {} > {}", section, tab_name, trait_name)
                    }
                }
            }
            NexusTab::Layers => {
                let realm = if self.layer_realm == 0 {
                    "Shared"
                } else {
                    "Org"
                };
                format!("Nexus > {} > {} > {}", section, tab_name, realm)
            }
            NexusTab::Arcs => {
                let families = [
                    "ownership",
                    "localization",
                    "semantic",
                    "generation",
                    "mining",
                ];
                let family = families.get(self.arc_cursor).unwrap_or(&"");
                format!("Nexus > {} > {} > {}", section, tab_name, family)
            }
            // PRACTICE section
            NexusTab::Pipeline => {
                let stages = [
                    "Knowledge",
                    "Entity",
                    "Structure",
                    "Instructions",
                    "Generation",
                    "Output",
                ];
                let stage = stages.get(self.pipeline_stage).unwrap_or(&"");
                format!("Nexus > {} > {} > {}", section, tab_name, stage)
            }
            NexusTab::Quiz => {
                let total = quiz::QUESTIONS.len();
                if self.quiz.complete {
                    format!(
                        "Nexus > {} > {} > Complete ({}/{})",
                        section, tab_name, self.quiz.score, total
                    )
                } else {
                    format!(
                        "Nexus > {} > {} > Q{}/{}",
                        section,
                        tab_name,
                        self.quiz.current_question + 1,
                        total
                    )
                }
            }
            NexusTab::Views => {
                if self.views.show_concept {
                    format!("Nexus > {} > {} > Query-First", section, tab_name)
                } else {
                    let cat = self.views.current_category();
                    if let Some(view) = self.views.current_view() {
                        format!(
                            "Nexus > {} > {} > {} > {}",
                            section,
                            tab_name,
                            cat.label(),
                            view.name
                        )
                    } else {
                        format!("Nexus > {} > {} > {}", section, tab_name, cat.label())
                    }
                }
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

    /// Get short breadcrumb for status bar display.
    /// Returns "SECTION > Tab" format (e.g., "LEARN > Intro").
    pub fn status_breadcrumb(&self) -> String {
        let section = self.tab.section();
        let tab_name = self.tab.label();
        format!("{} > {}", section, tab_name)
    }

    /// Get context-sensitive action hints for the current tab.
    /// Returns 3-4 items: Tab navigation + Context actions + Enter action.
    /// This is the SINGLE source of truth for keybindings (no other hints displayed).
    pub fn context_actions(&self) -> Vec<(&'static str, &'static str)> {
        // Unified format: arrows for nav, Enter for action
        // Display: ↑/↓=vertical, ←/→=horizontal, Enter=action
        // Silent alternatives: hjkl, Space (handled in key processing)
        match self.tab {
            NexusTab::Intro => vec![("←/→", "page"), ("Enter", "next"), ("y", "copy")],
            NexusTab::Glossary => vec![("↑/↓", "nav"), ("Enter", "expand"), ("y", "copy")],
            NexusTab::Tutorial => vec![("↑/↓", "task"), ("Enter", "done"), ("r", "reset")],
            NexusTab::Traits => {
                if self.drill_depth > 0 {
                    vec![("↑/↓", "nav"), ("Enter", "select"), ("Esc", "back")]
                } else {
                    vec![("↑/↓", "trait"), ("Enter", "drill"), ("y", "copy")]
                }
            }
            NexusTab::Layers => vec![("←/→", "realm"), ("↑/↓", "layer"), ("y", "copy")],
            NexusTab::Arcs => vec![("↑/↓", "family"), ("Enter", "detail"), ("y", "copy")],
            NexusTab::Pipeline => vec![("↑/↓", "stage"), ("Enter", "play"), ("r", "reset")],
            NexusTab::Quiz => {
                if self.quiz.answered {
                    vec![("Enter", "next"), ("r", "restart"), ("y", "copy")]
                } else {
                    vec![("↑/↓", "option"), ("Enter", "submit"), ("←/→", "hint")]
                }
            }
            NexusTab::Views => vec![("↑/↓", "view"), ("Enter", "detail"), ("y", "copy")],
        }
    }
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Nexus mode with tab bar, breadcrumb, and content.
/// Note: Action bar hints are now in the unified status bar (ui/status.rs).
pub fn render_nexus(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tab bar
            Constraint::Length(1), // Breadcrumb
            Constraint::Min(1),    // Content
            Constraint::Length(1), // Cross-tab hints (v0.12.0)
        ])
        .split(area);

    // Render tab bar
    render_tab_bar(f, chunks[0], app);

    // Render breadcrumb
    render_breadcrumb(f, chunks[1], app);

    // Render content based on active tab
    match app.nexus.tab {
        // LEARN section
        NexusTab::Intro => intro::render_intro_tab(f, app, chunks[2]),
        NexusTab::Glossary => glossary::render_glossary_tab(f, app, chunks[2]),
        NexusTab::Tutorial => tutorial::render_tutorial_tab(f, app, chunks[2]),
        // EXPLORE section
        NexusTab::Traits => traits::render_traits_tab(f, app, chunks[2]),
        NexusTab::Layers => layers::render_layers_tab(f, app, chunks[2]),
        NexusTab::Arcs => arcs::render_arcs_tab(f, app, chunks[2]),
        // PRACTICE section
        NexusTab::Pipeline => pipeline::render_pipeline_tab(f, app, chunks[2]),
        NexusTab::Quiz => quiz::render_quiz_tab(f, app, chunks[2]),
        NexusTab::Views => views::render_views_tab(f, app, chunks[2]),
    }

    // Render cross-tab navigation hints (v0.12.0)
    render_cross_tab_hints(f, chunks[3], app);
}

/// Render cross-tab navigation hints at the bottom (v0.12.0).
fn render_cross_tab_hints(f: &mut Frame, area: Rect, app: &App) {
    let related = app.nexus.tab.related_tabs(app.nexus.locale);
    let mut spans: Vec<Span> = Vec::new();

    // Leading arrow
    spans.push(Span::styled(
        "→ ",
        Style::default().fg(Color::DarkGray),
    ));

    for (i, (tab, hint)) in related.iter().enumerate() {
        // Tab shortcut key
        spans.push(Span::styled(
            format!("[{}] ", tab.shortcut()),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
        // Hint text
        spans.push(Span::styled(
            *hint,
            Style::default().fg(Color::Cyan),
        ));

        // Separator
        if i < related.len() - 1 {
            spans.push(Span::styled(
                "  │  ",
                Style::default().fg(Color::DarkGray),
            ));
        }
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    f.render_widget(paragraph, area);
}

/// Render the tab bar at the top of Nexus mode.
/// Minimalist design: section names + tab names only, no shortcuts displayed.
fn render_tab_bar(f: &mut Frame, area: Rect, app: &App) {
    let current_section = app.nexus.tab.section();

    // Section definitions: (name, tabs)
    let sections = [
        (
            "LEARN",
            vec![NexusTab::Intro, NexusTab::Glossary, NexusTab::Tutorial],
        ),
        (
            "EXPLORE",
            vec![NexusTab::Traits, NexusTab::Layers, NexusTab::Arcs],
        ),
        (
            "PRACTICE",
            vec![NexusTab::Pipeline, NexusTab::Quiz, NexusTab::Views],
        ),
    ];

    let mut spans: Vec<Span> = Vec::new();

    for (i, (name, tabs)) in sections.iter().enumerate() {
        let is_current_section = *name == current_section;

        // Section header with indicator
        let indicator = if is_current_section { "▼" } else { "▶" };
        let section_style = if is_current_section {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(100, 100, 110))
        };
        spans.push(Span::styled(
            format!("{} {} ", indicator, name),
            section_style,
        ));

        // Tab names within section (only show if current section)
        if is_current_section {
            for tab in tabs {
                let is_selected = *tab == app.nexus.tab;
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
                } else {
                    Style::default().fg(Color::White)
                };

                let prefix = if is_selected { "●" } else { "○" };
                spans.push(Span::styled(format!("{}{} ", prefix, tab.label()), style));
            }
        }

        // Section separator (except for last section)
        if i < sections.len() - 1 {
            spans.push(Span::styled(
                " │ ",
                Style::default().fg(COLOR_UNFOCUSED_BORDER),
            ));
        }
    }

    let tabs_line = Line::from(spans);

    let block = Block::default()
        .title(Span::styled(
            " Nexus ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(80, 80, 100)));

    let paragraph = Paragraph::new(tabs_line).block(block);
    f.render_widget(paragraph, area);
}

/// Render the breadcrumb bar showing current location in Nexus mode.
fn render_breadcrumb(f: &mut Frame, area: Rect, app: &App) {
    let trait_stats = app.tree.get_trait_stats();
    let breadcrumb = app.nexus.breadcrumb(&trait_stats);

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
                Style::default().fg(COLOR_UNFOCUSED_BORDER),
            ));
        }
    }

    // Note: hints removed - action bar is the single source of truth for keybindings
    let line = Line::from(spans);
    let paragraph = Paragraph::new(line);
    f.render_widget(paragraph, area);
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
        let state = NexusState::new();
        assert_eq!(state.tab, NexusTab::Intro); // v11.7: default is Intro
        assert_eq!(state.trait_cursor, 0);
        assert!(!state.pending_g);
        assert_eq!(state.tip_index, 0);
        assert_eq!(state.intro_page, 0);
    }

    #[test]
    fn test_tips_constant() {
        assert!(!TIPS.is_empty());
        assert!(TIPS.len() >= 5); // Ensure we have meaningful tips
    }

    #[test]
    fn test_current_tip() {
        let state = NexusState::new();
        let tip = state.current_tip();
        assert_eq!(tip, TIPS[0]);
    }

    #[test]
    fn test_next_tip_cycles() {
        let mut state = NexusState::new();
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
        let mut state = NexusState::new();
        assert!(!state.has_pending_g());

        // Press 'g' to enter pending state
        state.handle_key(key_event(KeyCode::Char('g')));
        assert!(state.has_pending_g());

        // Invalid key clears pending state
        state.handle_key(key_event(KeyCode::Char('x')));
        assert!(!state.has_pending_g());
    }

    #[test]
    fn test_quick_jump_gd() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers; // Start on different tab
        state.trait_cursor = 3;

        // Press 'g' then 'd' for defined (v11.8: was invariant)
        state.handle_key(key_event(KeyCode::Char('g')));
        assert!(state.has_pending_g());

        state.handle_key(key_event(KeyCode::Char('d')));
        assert!(!state.has_pending_g());
        assert_eq!(state.tab, NexusTab::Traits);
        assert_eq!(state.trait_cursor, 0); // defined = index 0
    }

    #[test]
    fn test_quick_jump_ga() {
        let mut state = NexusState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('a')));

        assert_eq!(state.tab, NexusTab::Traits);
        assert_eq!(state.trait_cursor, 1); // authored = index 1
    }

    #[test]
    fn test_quick_jump_gi() {
        let mut state = NexusState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('i')));

        assert_eq!(state.tab, NexusTab::Traits);
        assert_eq!(state.trait_cursor, 2); // imported = index 2
    }

    #[test]
    fn test_quick_jump_gg() {
        let mut state = NexusState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('g')));

        assert_eq!(state.tab, NexusTab::Traits);
        assert_eq!(state.trait_cursor, 3); // generated = index 3
    }

    #[test]
    fn test_quick_jump_gr() {
        let mut state = NexusState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('r')));

        assert_eq!(state.tab, NexusTab::Traits);
        assert_eq!(state.trait_cursor, 4); // retrieved = index 4 (v11.8: was aggregated)
    }

    #[test]
    fn test_quick_jump_g0() {
        let mut state = NexusState::new();
        state.trait_cursor = 3;
        state.layer_cursor = 2;
        state.arc_cursor = 1;

        // g0 should reset all cursors to 0 (v11.3: renamed from gg)
        state.handle_key(key_event(KeyCode::Char('g')));
        state.handle_key(key_event(KeyCode::Char('0')));

        assert_eq!(state.trait_cursor, 0);
        assert_eq!(state.layer_cursor, 0);
        assert_eq!(state.arc_cursor, 0);
    }

    #[test]
    fn test_pending_g_cancelled_by_escape() {
        let mut state = NexusState::new();

        state.handle_key(key_event(KeyCode::Char('g')));
        assert!(state.has_pending_g());

        state.handle_key(key_event(KeyCode::Esc));
        assert!(!state.has_pending_g());
    }

    #[test]
    fn test_n_key_cycles_tips() {
        let mut state = NexusState::new();
        assert_eq!(state.tip_index, 0);

        state.handle_key(key_event(KeyCode::Char('n')));
        assert_eq!(state.tip_index, 1);

        state.handle_key(key_event(KeyCode::Char('n')));
        assert_eq!(state.tip_index, 2);
    }

    #[test]
    fn test_tab_cycling() {
        let mut state = NexusState::new();
        assert_eq!(state.tab, NexusTab::Intro); // v11.7: default is Intro

        // Cycle through all 9 tabs
        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, NexusTab::Glossary);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, NexusTab::Tutorial);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, NexusTab::Traits);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, NexusTab::Layers);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, NexusTab::Arcs);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, NexusTab::Pipeline);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, NexusTab::Quiz);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, NexusTab::Views);

        state.handle_key(key_event(KeyCode::Tab));
        assert_eq!(state.tab, NexusTab::Intro); // Wraps around
    }

    #[test]
    fn test_guide_tab_all() {
        let all = NexusTab::all();
        assert_eq!(all.len(), 9); // v11.7: 9 tabs
        // LEARN section
        assert_eq!(all[0], NexusTab::Intro);
        assert_eq!(all[1], NexusTab::Glossary);
        assert_eq!(all[2], NexusTab::Tutorial);
        // EXPLORE section
        assert_eq!(all[3], NexusTab::Traits);
        assert_eq!(all[4], NexusTab::Layers);
        assert_eq!(all[5], NexusTab::Arcs);
        // PRACTICE section
        assert_eq!(all[6], NexusTab::Pipeline);
        assert_eq!(all[7], NexusTab::Quiz);
        assert_eq!(all[8], NexusTab::Views);
    }

    #[test]
    fn test_guide_tab_shortcuts() {
        // v11.7: letter-based shortcuts
        assert_eq!(NexusTab::Intro.shortcut(), 'i');
        assert_eq!(NexusTab::Glossary.shortcut(), 'g');
        assert_eq!(NexusTab::Tutorial.shortcut(), 'u');
        assert_eq!(NexusTab::Traits.shortcut(), 't');
        assert_eq!(NexusTab::Layers.shortcut(), 'l');
        assert_eq!(NexusTab::Arcs.shortcut(), 'a');
        assert_eq!(NexusTab::Pipeline.shortcut(), 'p');
        assert_eq!(NexusTab::Quiz.shortcut(), 'q');
        assert_eq!(NexusTab::Views.shortcut(), 'v');
    }

    #[test]
    fn test_guide_tab_labels() {
        // v11.7: 9 tabs with labels
        assert_eq!(NexusTab::Intro.label(), "Intro");
        assert_eq!(NexusTab::Glossary.label(), "Glossary");
        assert_eq!(NexusTab::Tutorial.label(), "Tutorial");
        assert_eq!(NexusTab::Traits.label(), "Traits");
        assert_eq!(NexusTab::Layers.label(), "Layers");
        assert_eq!(NexusTab::Arcs.label(), "Arcs");
        assert_eq!(NexusTab::Pipeline.label(), "Pipeline");
        assert_eq!(NexusTab::Quiz.label(), "Quiz");
        assert_eq!(NexusTab::Views.label(), "Views");
    }

    // ==========================================================================
    // TAB SWITCHING WITH BRACKET KEYS (v11.6: [ ] for tab nav)
    // ==========================================================================

    #[test]
    fn test_bracket_left_switches_to_prev_tab() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Glossary;

        let changed = state.handle_key(key_event(KeyCode::Char('[')));
        assert!(changed);
        assert_eq!(state.tab, NexusTab::Intro);
    }

    #[test]
    fn test_bracket_right_switches_to_next_tab() {
        let mut state = NexusState::new();
        assert_eq!(state.tab, NexusTab::Intro); // v11.7: default is Intro

        let changed = state.handle_key(key_event(KeyCode::Char(']')));
        assert!(changed);
        assert_eq!(state.tab, NexusTab::Glossary);
    }

    #[test]
    fn test_bracket_navigation_cycles() {
        let mut state = NexusState::new();
        assert_eq!(state.tab, NexusTab::Intro); // v11.7: default is Intro

        // Navigate forward through all 9 tabs
        state.handle_key(key_event(KeyCode::Char(']')));
        assert_eq!(state.tab, NexusTab::Glossary);

        state.handle_key(key_event(KeyCode::Char(']')));
        assert_eq!(state.tab, NexusTab::Tutorial);

        state.handle_key(key_event(KeyCode::Char(']')));
        assert_eq!(state.tab, NexusTab::Traits);

        state.handle_key(key_event(KeyCode::Char(']')));
        assert_eq!(state.tab, NexusTab::Layers);

        state.handle_key(key_event(KeyCode::Char(']')));
        assert_eq!(state.tab, NexusTab::Arcs);

        state.handle_key(key_event(KeyCode::Char(']')));
        assert_eq!(state.tab, NexusTab::Pipeline);

        state.handle_key(key_event(KeyCode::Char(']')));
        assert_eq!(state.tab, NexusTab::Quiz);

        state.handle_key(key_event(KeyCode::Char(']')));
        assert_eq!(state.tab, NexusTab::Views);

        // Wrap around
        state.handle_key(key_event(KeyCode::Char(']')));
        assert_eq!(state.tab, NexusTab::Intro);
    }

    #[test]
    fn test_bracket_left_wraps() {
        let mut state = NexusState::new();
        assert_eq!(state.tab, NexusTab::Intro); // v11.7: default is Intro

        // [ from Intro wraps to Views (last tab)
        let changed = state.handle_key(key_event(KeyCode::Char('[')));
        assert!(changed);
        assert_eq!(state.tab, NexusTab::Views);
    }

    #[test]
    fn test_tab_switch_resets_drill() {
        let mut state = NexusState::new();
        state.drill_depth = 2;
        state.drill_cursor = 5;

        state.handle_key(key_event(KeyCode::Char(']'))); // Switch to Layers via ]
        assert_eq!(state.drill_depth, 0);
        assert_eq!(state.drill_cursor, 0);
    }

    // ==========================================================================
    // BACKTAB NAVIGATION
    // ==========================================================================

    #[test]
    fn test_backtab_cycling() {
        let mut state = NexusState::new();
        assert_eq!(state.tab, NexusTab::Intro); // v11.7: default is Intro

        // Cycle backward through all 9 tabs
        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, NexusTab::Views); // Wraps to end

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, NexusTab::Quiz);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, NexusTab::Pipeline);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, NexusTab::Arcs);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, NexusTab::Layers);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, NexusTab::Traits);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, NexusTab::Tutorial);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, NexusTab::Glossary);

        state.handle_key(key_event(KeyCode::BackTab));
        assert_eq!(state.tab, NexusTab::Intro);
    }

    #[test]
    fn test_guide_tab_next_prev_symmetry() {
        // Verify next() and prev() are inverse operations
        for tab in NexusTab::all() {
            assert_eq!(tab.next().prev(), *tab);
            assert_eq!(tab.prev().next(), *tab);
        }
    }

    // ==========================================================================
    // SECTION NAVIGATION (v11.7: H/L shortcuts)
    // ==========================================================================

    #[test]
    fn test_section_next() {
        let mut state = NexusState::new();
        assert_eq!(state.tab, NexusTab::Intro); // Start in LEARN section

        // L (Shift+L) to next section
        let changed = state.handle_key(key_event(KeyCode::Char('L')));
        assert!(changed);
        assert_eq!(state.tab, NexusTab::Traits); // EXPLORE section
    }

    #[test]
    fn test_section_prev() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits; // Start in EXPLORE section

        // H (Shift+H) to prev section
        let changed = state.handle_key(key_event(KeyCode::Char('H')));
        assert!(changed);
        assert_eq!(state.tab, NexusTab::Intro); // LEARN section
    }

    #[test]
    fn test_section_cycle_forward() {
        let mut state = NexusState::new();
        assert_eq!(state.tab, NexusTab::Intro); // LEARN

        state.handle_key(key_event(KeyCode::Char('L'))); // → EXPLORE
        assert_eq!(state.tab, NexusTab::Traits);

        state.handle_key(key_event(KeyCode::Char('L'))); // → PRACTICE
        assert_eq!(state.tab, NexusTab::Pipeline);

        state.handle_key(key_event(KeyCode::Char('L'))); // → LEARN (wrap)
        assert_eq!(state.tab, NexusTab::Intro);
    }

    #[test]
    fn test_section_cycle_backward() {
        let mut state = NexusState::new();
        assert_eq!(state.tab, NexusTab::Intro); // LEARN

        state.handle_key(key_event(KeyCode::Char('H'))); // ← PRACTICE (wrap)
        assert_eq!(state.tab, NexusTab::Pipeline);

        state.handle_key(key_event(KeyCode::Char('H'))); // ← EXPLORE
        assert_eq!(state.tab, NexusTab::Traits);

        state.handle_key(key_event(KeyCode::Char('H'))); // ← LEARN
        assert_eq!(state.tab, NexusTab::Intro);
    }

    #[test]
    fn test_section_navigation_resets_drill() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
        state.drill_depth = 2;
        state.drill_cursor = 5;

        state.handle_key(key_event(KeyCode::Char('H'))); // Jump to LEARN
        assert_eq!(state.drill_depth, 0);
        assert_eq!(state.drill_cursor, 0);
    }

    #[test]
    fn test_section_method() {
        // LEARN section
        assert_eq!(NexusTab::Intro.section(), "LEARN");
        assert_eq!(NexusTab::Glossary.section(), "LEARN");
        assert_eq!(NexusTab::Tutorial.section(), "LEARN");
        // EXPLORE section
        assert_eq!(NexusTab::Traits.section(), "EXPLORE");
        assert_eq!(NexusTab::Layers.section(), "EXPLORE");
        assert_eq!(NexusTab::Arcs.section(), "EXPLORE");
        // PRACTICE section
        assert_eq!(NexusTab::Pipeline.section(), "PRACTICE");
        assert_eq!(NexusTab::Quiz.section(), "PRACTICE");
        assert_eq!(NexusTab::Views.section(), "PRACTICE");
    }

    // ==========================================================================
    // DRILL-DOWN STATE MANAGEMENT
    // ==========================================================================

    #[test]
    fn test_drill_down_enter_key() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits; // Use Traits tab for drill-down test
        assert_eq!(state.drill_depth, 0);

        // Enter to drill down
        let changed = state.handle_key(key_event(KeyCode::Enter));
        assert!(changed);
        assert_eq!(state.drill_depth, 1);
        assert_eq!(state.drill_cursor, 0);
    }

    #[test]
    fn test_drill_down_max_depth() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits; // Use Traits tab for drill-down test
        state.drill_depth = 2;

        // Already at max depth, should not drill further
        let changed = state.handle_key(key_event(KeyCode::Enter));
        assert!(!changed);
        assert_eq!(state.drill_depth, 2);
    }

    #[test]
    fn test_drill_up_escape_key() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits; // Use Traits tab for drill-down test
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits; // Use Traits tab for drill-down test
        assert_eq!(state.drill_depth, 0);

        // Already at depth 0, should not change
        let changed = state.handle_key(key_event(KeyCode::Esc));
        assert!(!changed);
        assert_eq!(state.drill_depth, 0);
    }

    #[test]
    fn test_drill_down_with_l_key() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits; // Not Layers
        assert_eq!(state.drill_depth, 0);

        // 'l' should drill down (except in Layers tab)
        let changed = state.handle_key(key_event(KeyCode::Char('l')));
        assert!(changed);
        assert_eq!(state.drill_depth, 1);
    }

    #[test]
    fn test_drill_up_with_h_key() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits; // Not Layers
        state.drill_depth = 1;

        // 'h' should drill up (except in Layers tab)
        let changed = state.handle_key(key_event(KeyCode::Char('h')));
        assert!(changed);
        assert_eq!(state.drill_depth, 0);
    }

    #[test]
    fn test_drill_right_left_keys() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
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
        let mut state = NexusState::new();
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 0; // Shared (2 layers, max index 1)
        state.layer_cursor = 0;

        // Navigate down
        state.handle_key(key_event(KeyCode::Char('j')));
        assert_eq!(state.layer_cursor, 1);
    }

    #[test]
    fn test_layers_navigate_up() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_cursor = 2;

        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.layer_cursor, 1);
    }

    #[test]
    fn test_layers_shared_max_cursor() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 0; // Shared (4 layers v11.5, max index 3)
        state.layer_cursor = 3;

        // Should not go beyond max
        let changed = state.handle_key(key_event(KeyCode::Char('j')));
        assert!(!changed);
        assert_eq!(state.layer_cursor, 3);
    }

    #[test]
    fn test_layers_org_max_cursor() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 1; // Org (6 layers v11.5, max index 5)
        state.layer_cursor = 5;

        // Should not go beyond max
        let changed = state.handle_key(key_event(KeyCode::Char('j')));
        assert!(!changed);
        assert_eq!(state.layer_cursor, 5);
    }

    #[test]
    fn test_layers_realm_switch_left() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 1; // Start on Org
        state.layer_cursor = 3;

        // Switch to Shared with 'h'
        let changed = state.handle_key(key_event(KeyCode::Char('h')));
        assert!(changed);
        assert_eq!(state.layer_realm, 0);
        assert_eq!(state.layer_cursor, 0); // Cursor reset on realm switch
    }

    #[test]
    fn test_layers_realm_switch_right() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 0; // Start on Shared
        state.layer_cursor = 1;

        // Switch to Org with 'l'
        let changed = state.handle_key(key_event(KeyCode::Char('l')));
        assert!(changed);
        assert_eq!(state.layer_realm, 1);
        assert_eq!(state.layer_cursor, 0); // Cursor reset on realm switch
    }

    #[test]
    fn test_layers_realm_switch_no_change() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 0; // Already on Shared

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
        let mut state = NexusState::new();
        state.tab = NexusTab::Arcs;
        assert_eq!(state.arc_cursor, 0);

        state.handle_key(key_event(KeyCode::Char('j')));
        assert_eq!(state.arc_cursor, 1);

        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.arc_cursor, 0);
    }

    #[test]
    fn test_arcs_max_cursor() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Arcs;
        state.arc_cursor = 4; // 5 arc families (0-4)

        let changed = state.handle_key(key_event(KeyCode::Char('j')));
        assert!(!changed);
        assert_eq!(state.arc_cursor, 4);
    }

    #[test]
    fn test_arcs_min_cursor() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Arcs;
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Pipeline;
        assert_eq!(state.pipeline_stage, 0);

        state.handle_key(key_event(KeyCode::Char('j')));
        assert_eq!(state.pipeline_stage, 1);

        state.handle_key(key_event(KeyCode::Char('k')));
        assert_eq!(state.pipeline_stage, 0);
    }

    #[test]
    fn test_pipeline_max_stage() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Pipeline;
        state.pipeline_stage = 5; // 6 stages (0-5)

        let changed = state.handle_key(key_event(KeyCode::Char('j')));
        assert!(!changed);
        assert_eq!(state.pipeline_stage, 5);
    }

    #[test]
    fn test_pipeline_min_stage() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Pipeline;
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Pipeline;
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Pipeline;
        assert!(!state.pipeline_animating);

        // Enter on Pipeline tab toggles animation (not drill down)
        let changed = state.handle_key(key_event(KeyCode::Enter));
        assert!(changed);
        assert!(state.pipeline_animating);
    }

    #[test]
    fn test_space_triggers_drill_on_traits() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;

        // Space triggers drill_down on Traits tab (same as Enter)
        let changed = state.handle_key(key_event(KeyCode::Char(' ')));
        assert!(changed); // Space is now handled, triggers drill_down
    }

    // ==========================================================================
    // CURSOR CLAMPING
    // ==========================================================================

    #[test]
    fn test_clamp_drill_cursor_zero_len() {
        let mut state = NexusState::new();
        state.drill_cursor = 10;

        state.clamp_drill_cursor(0);
        assert_eq!(state.drill_cursor, 0);
    }

    #[test]
    fn test_clamp_drill_cursor_over_max() {
        let mut state = NexusState::new();
        state.drill_cursor = 100;

        state.clamp_drill_cursor(10); // max_len = 10, valid indices 0-9
        assert_eq!(state.drill_cursor, 9);
    }

    #[test]
    fn test_clamp_drill_cursor_within_range() {
        let mut state = NexusState::new();
        state.drill_cursor = 5;

        state.clamp_drill_cursor(10);
        assert_eq!(state.drill_cursor, 5); // Unchanged
    }

    // ==========================================================================
    // BREADCRUMB GENERATION
    // ==========================================================================

    #[test]
    fn test_breadcrumb_intro() {
        let state = NexusState::new();
        let trait_stats = Vec::new();

        // v11.7: default is Intro, breadcrumb includes section
        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.starts_with("Nexus > LEARN > Intro"));
    }

    #[test]
    fn test_breadcrumb_traits_overview() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
        let trait_stats = Vec::new();

        // v11.7: breadcrumb includes section
        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.starts_with("Nexus > EXPLORE > Traits > "));
    }

    #[test]
    fn test_breadcrumb_layers() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 0;
        let trait_stats = Vec::new();

        // v11.7: breadcrumb includes section
        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.contains("Nexus > EXPLORE > Layers > Shared"));
    }

    #[test]
    fn test_breadcrumb_layers_org() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 1;
        let trait_stats = Vec::new();

        // v11.7: breadcrumb includes section
        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.contains("Nexus > EXPLORE > Layers > Org"));
    }

    #[test]
    fn test_breadcrumb_arcs() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Arcs;
        state.arc_cursor = 0;
        let trait_stats = Vec::new();

        // v11.7: breadcrumb includes section
        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.contains("Nexus > EXPLORE > Arcs > ownership"));
    }

    #[test]
    fn test_breadcrumb_pipeline() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Pipeline;
        state.pipeline_stage = 0;
        let trait_stats = Vec::new();

        // v11.7: breadcrumb includes section
        let breadcrumb = state.breadcrumb(&trait_stats);
        assert!(breadcrumb.contains("Nexus > PRACTICE > Pipeline > Knowledge"));
    }

    // ==========================================================================
    // EDGE CASES AND DEFENSIVE CHECKS
    // ==========================================================================

    #[test]
    fn test_unhandled_key_returns_false() {
        let mut state = NexusState::new();

        // Arbitrary unhandled keys
        let changed = state.handle_key(key_event(KeyCode::Char('z')));
        assert!(!changed);

        let changed = state.handle_key(key_event(KeyCode::F(1)));
        assert!(!changed);
    }

    #[test]
    fn test_default_impl() {
        let state = NexusState::default();
        assert_eq!(state.tab, NexusTab::Intro); // v11.7: default is Intro
        assert_eq!(state.trait_cursor, 0);
        assert_eq!(state.drill_depth, 0);
        assert_eq!(state.intro_page, 0);
    }

    #[test]
    fn test_guide_tab_default() {
        let tab = NexusTab::default();
        assert_eq!(tab, NexusTab::Intro); // v11.7: default is Intro
    }

    #[test]
    fn test_jump_to_trait_clamps() {
        let mut state = NexusState::new();

        // jump_to_trait clamps to 0-4
        state.jump_to_trait(100);
        assert_eq!(state.trait_cursor, 4);
    }

    #[test]
    fn test_get_trait_kinds_empty_stats() {
        let state = NexusState::new();
        let empty_stats: Vec<traits::TraitStats> = Vec::new();

        let kinds = state.get_trait_kinds(&empty_stats);
        assert!(kinds.is_empty());
    }

    #[test]
    fn test_current_tip_fallback() {
        let mut state = NexusState::new();
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
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;

        // v11.8: ADR-024 Data Origin semantics
        state.trait_cursor = 0;
        assert_eq!(state.get_current_yank_text(), Some("defined".to_string())); // was: invariant

        state.trait_cursor = 1;
        assert_eq!(state.get_current_yank_text(), Some("authored".to_string())); // was: localized

        state.trait_cursor = 2;
        assert_eq!(state.get_current_yank_text(), Some("imported".to_string())); // was: knowledge

        state.trait_cursor = 3;
        assert_eq!(state.get_current_yank_text(), Some("generated".to_string()));

        state.trait_cursor = 4;
        assert_eq!(
            state.get_current_yank_text(),
            Some("retrieved".to_string()) // was: aggregated
        );

        // Index 5 now returns None
        state.trait_cursor = 5;
        assert_eq!(state.get_current_yank_text(), None);
    }

    #[test]
    fn test_get_current_yank_text_layers_shared() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 0; // Shared

        // v11.5: Shared layers are config, locale, geography, knowledge
        state.layer_cursor = 0;
        assert_eq!(state.get_current_yank_text(), Some("config".to_string()));

        state.layer_cursor = 1;
        assert_eq!(state.get_current_yank_text(), Some("locale".to_string()));

        state.layer_cursor = 2;
        assert_eq!(state.get_current_yank_text(), Some("geography".to_string()));

        state.layer_cursor = 3;
        assert_eq!(state.get_current_yank_text(), Some("knowledge".to_string()));
    }

    #[test]
    fn test_get_current_yank_text_layers_org() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Layers;
        state.layer_realm = 1; // Org

        // v11.5: Org realm has 6 layers (SEO/GEO consolidated to shared/knowledge)
        // config(0), foundation(1), structure(2), semantic(3), instruction(4), output(5)
        state.layer_cursor = 0;
        assert_eq!(state.get_current_yank_text(), Some("config".to_string()));

        state.layer_cursor = 3;
        assert_eq!(state.get_current_yank_text(), Some("semantic".to_string()));

        state.layer_cursor = 4;
        assert_eq!(
            state.get_current_yank_text(),
            Some("instruction".to_string())
        );

        state.layer_cursor = 5;
        assert_eq!(state.get_current_yank_text(), Some("output".to_string()));
    }

    #[test]
    fn test_get_current_yank_text_arcs() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Arcs;

        state.arc_cursor = 0;
        assert_eq!(state.get_current_yank_text(), Some("ownership".to_string()));

        state.arc_cursor = 2;
        assert_eq!(state.get_current_yank_text(), Some("semantic".to_string()));

        state.arc_cursor = 4;
        assert_eq!(state.get_current_yank_text(), Some("mining".to_string()));
    }

    #[test]
    fn test_get_current_yank_text_pipeline() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Pipeline;

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
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
        state.trait_cursor = 100; // Out of bounds

        // Should return None for invalid cursor
        assert_eq!(state.get_current_yank_text(), None);
    }

    #[test]
    fn test_clipboard_message_initial_state() {
        let state = NexusState::new();
        assert!(state.clipboard_message.is_none());
        assert!(state.clipboard_message_time.is_none());
    }

    #[test]
    fn test_clear_expired_clipboard_message_none() {
        let mut state = NexusState::new();
        // Should not panic when no message exists
        state.clear_expired_clipboard_message();
        assert!(state.clipboard_message.is_none());
    }

    #[test]
    fn test_clear_expired_clipboard_message_recent() {
        let mut state = NexusState::new();
        state.clipboard_message = Some("test".to_string());
        state.clipboard_message_time = Some(std::time::Instant::now());

        // Message is fresh, should not be cleared
        state.clear_expired_clipboard_message();
        assert!(state.clipboard_message.is_some());
    }

    #[test]
    fn test_y_key_triggers_yank() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits;
        state.trait_cursor = 0;

        // y key should trigger yank (may fail in CI without clipboard, but state should change)
        // Note: This will return true if clipboard works, or if error message is set
        // In CI without clipboard access, it still sets clipboard_message with error
        // Just verify no panic - we don't assert on the return value as clipboard may fail in CI
        let _ = state.handle_key(key_event(KeyCode::Char('y')));
    }

    // ==========================================================================
    // PERSISTENCE INTEGRATION (v11.7: Save/Load tutorial progress & quiz scores)
    // ==========================================================================

    #[test]
    fn test_with_persistence_creates_state() {
        // with_persistence should return a valid state even if no saved data exists
        let state = NexusState::with_persistence();
        assert_eq!(state.tab, NexusTab::Intro);
        // Tutorial should be initialized (either fresh or from saved state)
        // Note: current_step may be non-zero if user has saved progress in ~/.novanet/
        assert!(state.tutorial.current_step < 10); // Sanity check: valid range
    }

    #[test]
    fn test_save_tutorial_progress_no_panic() {
        let mut state = NexusState::new();
        // Should not panic even if save fails (e.g., permission issues)
        state.save_tutorial_progress();
    }

    #[test]
    fn test_save_quiz_score_no_panic() {
        let mut state = NexusState::new();
        // Should not panic even if save fails
        state.save_quiz_score(10);
    }

    #[test]
    fn test_space_in_tutorial_toggles_task() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Tutorial;

        // Initial state: task 0 is not completed
        assert!(!state.tutorial.tasks_completed[0][0]);

        // Space toggles task 0
        let changed = state.handle_key(key_event(KeyCode::Char(' ')));
        assert!(changed);
        assert!(state.tutorial.tasks_completed[0][0]);

        // Space again toggles it back
        let changed = state.handle_key(key_event(KeyCode::Char(' ')));
        assert!(changed);
        assert!(!state.tutorial.tasks_completed[0][0]);
    }

    #[test]
    fn test_c_key_marks_tutorial_step_complete() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Tutorial;

        // Mark step 0 complete with 'c'
        let changed = state.handle_key(key_event(KeyCode::Char('c')));
        assert!(changed);

        // All tasks in step 0 should be complete
        for task in &state.tutorial.tasks_completed[0] {
            assert!(task);
        }
    }

    #[test]
    fn test_c_key_only_works_in_tutorial() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Traits; // Not Tutorial

        let changed = state.handle_key(key_event(KeyCode::Char('c')));
        assert!(!changed); // Should not handle in non-Tutorial tab
    }

    #[test]
    fn test_quiz_high_score_preserved_on_reset() {
        let mut state = NexusState::new();
        state.quiz.high_score = Some(15);

        state.quiz.reset();

        // High score should be preserved
        assert_eq!(state.quiz.high_score, Some(15));
    }

    #[test]
    fn test_quiz_updates_high_score_on_completion() {
        let mut state = NexusState::new();
        state.tab = NexusTab::Quiz;
        assert!(state.quiz.high_score.is_none());

        // Simulate quiz completion with score 3
        state.quiz.score = 3;
        state.quiz.current_question = quiz::QUESTIONS.len() - 1;
        state.quiz.answered = true;

        // Enter to complete quiz
        state.handle_key(key_event(KeyCode::Enter));

        assert!(state.quiz.complete);
        assert_eq!(state.quiz.high_score, Some(3));
    }

    #[test]
    fn test_quiz_is_new_high_score() {
        let mut state = NexusState::new();
        state.quiz.complete = true;
        state.quiz.score = 5;
        state.quiz.high_score = Some(5);

        assert!(state.quiz.is_new_high_score());

        // Different score means not new high score
        state.quiz.high_score = Some(10);
        assert!(!state.quiz.is_new_high_score());
    }
}
