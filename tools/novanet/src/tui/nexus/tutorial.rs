//! Tutorial Tab - Guided learning journey for NovaNet.
//!
//! 5-step interactive tutorial with hands-on practice:
//! 1. Graph Fundamentals - Meta vs Data distinction
//! 2. Classification - Realm, Layer, Trait
//! 3. Arcs & Relationships - Family, Scope, Cardinality
//! 4. Generation Flow - NOT translation
//! 5. Unified Tree - v11.7 navigation

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use super::{NexusLocale, i18n};
use crate::tui::app::App;

// =============================================================================
// TUTORIAL DATA STRUCTURES
// =============================================================================

/// A tutorial step with objective, explanation, and practice tasks.
#[derive(Debug, Clone)]
pub struct TutorialStep {
    pub id: usize,
    pub title: &'static str,
    pub objective: &'static str,
    pub explanation: &'static [&'static str],
    pub tasks: &'static [TutorialTask],
}

/// A practice task within a tutorial step.
#[derive(Debug, Clone)]
pub struct TutorialTask {
    pub description: &'static str,
    pub hint: Option<&'static str>,
}

/// Total number of tutorial steps.
pub const TUTORIAL_STEPS: usize = 5;

// =============================================================================
// TUTORIAL CONTENT
// =============================================================================

pub static STEPS: [TutorialStep; 5] = [
    // Step 1: Graph Fundamentals
    TutorialStep {
        id: 1,
        title: "Graph Fundamentals",
        objective: "Understand that NovaNet has definitions (META) and actual data (DATA)",
        explanation: &[
            "NovaNet stores TWO kinds of information:",
            "",
            "META: \"What types exist?\"",
            "  - 60 total Kind definitions",
            "  - Example: Kind:Locale (the definition of what a locale IS)",
            "",
            "DATA: \"What instances exist?\"",
            "  - 200,000+ actual instances",
            "  - Example: Locale:fr-FR (an actual locale instance)",
            "",
            "The relationship between them is OF_KIND:",
            "  (:Locale {key: 'fr-FR'})-[:OF_KIND]->(:Kind {label: 'Locale'})",
        ],
        tasks: &[
            TutorialTask {
                description: "Find \"Locale\" in the tree (shared > config)",
                hint: Some("Press [1] to switch to Graph mode, then navigate to shared > config"),
            },
            TutorialTask {
                description: "Expand Locale to see instances: fr-FR, ja-JP, ar-AE...",
                hint: Some("Press Enter or 'l' to expand the Locale kind"),
            },
            TutorialTask {
                description: "Click on \"fr-FR\" to see its properties",
                hint: Some("Look for language_code: \"fr\", script: \"latin\""),
            },
        ],
    },
    // Step 2: Classification
    TutorialStep {
        id: 2,
        title: "Classification",
        objective: "Understand how nodes are classified with Realm, Layer, and Trait",
        explanation: &[
            "Every Kind has 3 classification axes:",
            "",
            "WHERE?  Realm",
            "  shared (39 nodes) = Universal, READ-ONLY",
            "  org (21 nodes) = Organization-specific",
            "",
            "WHAT?   Layer  (10 total)",
            "  shared: config, locale, geography, knowledge",
            "  org: config, foundation, structure, semantic, instruction, output",
            "",
            "HOW?    Trait  (5 behaviors)",
            "  invariant  = Same everywhere (Entity, Page)",
            "  localized  = Generated per locale (EntityContent)",
            "  knowledge  = Locale expertise (Term, Culture)",
            "  generated  = LLM output (PageGenerated)",
            "  aggregated = Computed metrics (SEOKeywordMetrics)",
        ],
        tasks: &[
            TutorialTask {
                description: "Find a node in shared/config layer",
                hint: Some("Locale and EntityCategory are in shared/config"),
            },
            TutorialTask {
                description: "Find a node with trait 'localized' (look for dashed border)",
                hint: Some("EntityContent in org/semantic has localized trait"),
            },
            TutorialTask {
                description: "Find a node in org/semantic layer",
                hint: Some("Entity and EntityContent are in org/semantic"),
            },
        ],
    },
    // Step 3: Arcs & Relationships
    TutorialStep {
        id: 3,
        title: "Arcs & Relationships",
        objective: "Understand how nodes connect with Arcs, Families, and Scope",
        explanation: &[
            "Nodes are connected by ARCS (directed relationships).",
            "",
            "The main pattern:",
            "  Entity (invariant) --HAS_CONTENT--> EntityContent (localized)",
            "",
            "ARC FAMILIES group relationships by function:",
            "  ownership     - Parent owns child (HAS_CONTENT, HAS_BLOCK)",
            "  localization  - Links to locale (FOR_LOCALE)",
            "  semantic      - Meaning connections (SEMANTIC_LINK)",
            "  generation    - LLM pipeline (HAS_GENERATED)",
            "  mining        - SEO/GEO intelligence (HAS_KEYWORD)",
            "",
            "ARC SCOPE indicates realm crossing:",
            "  intra_realm  - Both nodes in same realm",
            "  cross_realm  - Nodes in different realms (org -> shared)",
        ],
        tasks: &[
            TutorialTask {
                description: "Find Entity in org/semantic",
                hint: Some("Entity is invariant - solid border"),
            },
            TutorialTask {
                description: "Look at Entity's outgoing arcs",
                hint: Some("HAS_CONTENT goes to EntityContent"),
            },
            TutorialTask {
                description: "Find a cross_realm arc (org -> shared)",
                hint: Some("Entity BELONGS_TO EntityCategory (org -> shared/config)"),
            },
        ],
    },
    // Step 4: Generation Flow
    TutorialStep {
        id: 4,
        title: "Generation Flow",
        objective: "Understand that NovaNet GENERATES content, NOT translates",
        explanation: &[
            "WRONG approach (translation):",
            "  Source (English) -> Translate -> Target (French)",
            "  Problem: Loses cultural nuance, costs 200x",
            "",
            "RIGHT approach (native generation):",
            "  Entity (invariant, defined once)",
            "    + Knowledge (fr-FR: Terms, Culture, Style)",
            "    -> EntityContent@fr-FR (generated natively)",
            "",
            "The pipeline:",
            "  1. Knowledge - Load locale expertise (Terms, Culture)",
            "  2. Entity - Get invariant definition",
            "  3. Structure - Page/Block layout",
            "  4. Instructions - Prompts and constraints",
            "  5. Generation - LLM produces native content",
            "  6. Output - PageGenerated, BlockGenerated",
        ],
        tasks: &[
            TutorialTask {
                description: "Go to Pipeline tab [4] and watch the animation",
                hint: Some("Press Space to play/pause the animation"),
            },
            TutorialTask {
                description: "Identify the 6 stages of generation",
                hint: Some("Knowledge -> Entity -> Structure -> Instructions -> Generation -> Output"),
            },
            TutorialTask {
                description: "Understand why 'knowledge' comes FIRST",
                hint: Some("LLM needs locale expertise before generating"),
            },
        ],
    },
    // Step 5: Unified Tree Navigation
    TutorialStep {
        id: 5,
        title: "Unified Tree",
        objective: "Master the v11.7 unified tree navigation",
        explanation: &[
            "v11.7 introduced the Unified Tree principle:",
            "  \"If it's a node in Neo4j, it's a node everywhere\"",
            "",
            "The tree structure:",
            "  Nodes (60)",
            "    Realm:shared (clickable!)",
            "      Layer:config (clickable!)",
            "        Kind:Locale (clickable, expandable)",
            "          Locale:fr-FR (data instance)",
            "          Locale:ja-JP",
            "          ...",
            "",
            "  Arcs (114)",
            "    ArcFamily:ownership (clickable!)",
            "      ArcKind:HAS_CONTENT (clickable!)",
            "",
            "Navigation modes:",
            "  [1] Graph - Unified tree (you are here normally)",
            "  [2] Nexus - Learning hub (you are here now!)",
        ],
        tasks: &[
            TutorialTask {
                description: "Switch to Graph mode [1] and explore the full tree",
                hint: Some("j/k to navigate, Enter/l to expand, Esc/h to collapse"),
            },
            TutorialTask {
                description: "Click on a Realm node (shared or org) to see its properties",
                hint: Some("Realms are now clickable nodes, not just labels"),
            },
            TutorialTask {
                description: "Explore the Arcs section and click on an ArcFamily",
                hint: Some("ArcFamily:ownership shows all ownership arcs"),
            },
        ],
    },
];

// =============================================================================
// TUTORIAL STATE
// =============================================================================

/// State for the Tutorial tab.
#[derive(Debug, Clone)]
pub struct TutorialState {
    /// Current step (0-indexed).
    pub current_step: usize,
    /// Task completion status for each step.
    pub tasks_completed: Vec<Vec<bool>>,
    /// Whether the tutorial is complete.
    pub complete: bool,
}

impl Default for TutorialState {
    fn default() -> Self {
        Self::new()
    }
}

impl TutorialState {
    /// Create new TutorialState.
    pub fn new() -> Self {
        let tasks_completed = STEPS
            .iter()
            .map(|step| vec![false; step.tasks.len()])
            .collect();

        Self {
            current_step: 0,
            tasks_completed,
            complete: false,
        }
    }

    /// Get current step.
    pub fn current(&self) -> &TutorialStep {
        &STEPS[self.current_step.min(TUTORIAL_STEPS - 1)]
    }

    /// Get progress percentage (0-100).
    pub fn progress_percent(&self) -> usize {
        let total_tasks: usize = STEPS.iter().map(|s| s.tasks.len()).sum();
        let completed_tasks: usize = self
            .tasks_completed
            .iter()
            .flat_map(|t| t.iter())
            .filter(|&&b| b)
            .count();

        if total_tasks == 0 {
            0
        } else {
            (completed_tasks * 100) / total_tasks
        }
    }

    /// Get completed steps count.
    pub fn completed_steps(&self) -> usize {
        self.tasks_completed
            .iter()
            .filter(|tasks| tasks.iter().all(|&t| t))
            .count()
    }

    /// Check if current step is complete.
    pub fn is_step_complete(&self) -> bool {
        self.tasks_completed
            .get(self.current_step)
            .map(|tasks| tasks.iter().all(|&t| t))
            .unwrap_or(false)
    }

    /// Toggle task completion.
    pub fn toggle_task(&mut self, task_index: usize) {
        if let Some(tasks) = self.tasks_completed.get_mut(self.current_step) {
            if let Some(task) = tasks.get_mut(task_index) {
                *task = !*task;
            }
        }
        self.check_completion();
    }

    /// Mark all tasks in current step as complete.
    pub fn mark_step_complete(&mut self) {
        if let Some(tasks) = self.tasks_completed.get_mut(self.current_step) {
            for task in tasks.iter_mut() {
                *task = true;
            }
        }
        self.check_completion();
    }

    /// Go to next step.
    pub fn next_step(&mut self) {
        if self.current_step < TUTORIAL_STEPS - 1 {
            self.current_step += 1;
        }
    }

    /// Go to previous step.
    pub fn prev_step(&mut self) {
        if self.current_step > 0 {
            self.current_step -= 1;
        }
    }

    /// Reset tutorial progress.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Check if tutorial is complete.
    fn check_completion(&mut self) {
        self.complete = self.completed_steps() == TUTORIAL_STEPS;
    }

    /// Get step status indicator.
    pub fn step_status(&self, step_index: usize) -> &'static str {
        if step_index < self.current_step {
            "●" // Completed
        } else if step_index == self.current_step {
            "◉" // Current
        } else {
            "○" // Not started
        }
    }

    /// Navigate up (in task list within current step).
    pub fn navigate_up(&mut self) {
        // For now, navigate up goes to previous step
        self.prev_step();
    }

    /// Navigate down (in task list within current step).
    pub fn navigate_down(&mut self) {
        // For now, navigate down goes to next step
        self.next_step();
    }

    /// Get text to yank.
    pub fn get_yank_text(&self) -> Option<String> {
        Some(self.current().title.to_string())
    }
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Tutorial tab.
pub fn render_tutorial_tab(f: &mut Frame, app: &App, area: Rect) {
    let tutorial = &app.nexus.tutorial;
    let locale = app.nexus.locale;
    let _theme = &app.theme;

    // Split into progress bar, content, and navigation
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Progress bar
            Constraint::Min(1),    // Content
            Constraint::Length(2), // Navigation
        ])
        .split(area);

    render_progress_bar(f, tutorial, locale, chunks[0]);
    render_step_content(f, app, chunks[1]);
    render_navigation(f, tutorial, locale, chunks[2]);
}

/// Render the progress bar at the top.
fn render_progress_bar(f: &mut Frame, tutorial: &TutorialState, locale: NexusLocale, area: Rect) {
    let _progress = tutorial.progress_percent();
    let step = tutorial.current_step + 1;

    // i18n labels
    let journey_label = match locale {
        NexusLocale::En => "  YOUR JOURNEY  ",
        NexusLocale::Fr => "  VOTRE PARCOURS  ",
    };
    let step_label = match locale {
        NexusLocale::En => "Step",
        NexusLocale::Fr => "Étape",
    };

    // Build progress indicators
    let mut spans = vec![
        Span::styled(journey_label, Style::default().fg(Color::Magenta)),
    ];

    for i in 0..TUTORIAL_STEPS {
        let status = tutorial.step_status(i);
        let style = if i < tutorial.current_step {
            Style::default().fg(Color::Green) // Completed
        } else if i == tutorial.current_step {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD) // Current
        } else {
            Style::default().fg(Color::DarkGray) // Not started
        };

        spans.push(Span::styled(status, style));

        if i < TUTORIAL_STEPS - 1 {
            let connector_style = if i < tutorial.current_step {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            spans.push(Span::styled("━━━━━━━━━━", connector_style));
        }
    }

    spans.push(Span::styled(
        format!("    {step_label} {step}/{TUTORIAL_STEPS}"),
        Style::default().fg(Color::Cyan),
    ));

    let line = Line::from(spans);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let paragraph = Paragraph::new(vec![Line::from(""), line]).block(block);
    f.render_widget(paragraph, area);
}

/// Render the step content (objective, explanation, tasks).
#[allow(clippy::vec_init_then_push)] // Dynamic vector building with loops
fn render_step_content(f: &mut Frame, app: &App, area: Rect) {
    let tutorial = &app.nexus.tutorial;
    let locale = app.nexus.locale;
    let step_idx = tutorial.current_step;
    let _theme = &app.theme;

    // Get localized content
    let i18n_steps = i18n::tutorial_steps(locale);
    let i18n_step = i18n_steps.get(step_idx);

    // Get title and description from i18n
    let title = i18n_step.map(|s| s.title).unwrap_or_else(|| {
        STEPS.get(step_idx).map(|s| s.title).unwrap_or("Unknown")
    });
    let description = i18n_step.map(|s| s.description).unwrap_or_else(|| {
        STEPS.get(step_idx).map(|s| s.objective).unwrap_or("")
    });
    let i18n_tasks = i18n_step.map(|s| s.tasks).unwrap_or(&[]);

    // Labels based on locale
    let (goal_label, insight_label, try_label, hint_prefix, nav_hint) = match locale {
        NexusLocale::En => ("GOAL: ", "THE KEY INSIGHT", "TRY IT", "Hint: ", "Press [1-3] to toggle task completion, [Enter] to mark step complete"),
        NexusLocale::Fr => ("OBJECTIF : ", "L'IDÉE CLÉ", "ESSAYEZ", "Astuce : ", "Appuyez [1-3] pour cocher, [Entrée] pour terminer l'étape"),
    };

    let block = Block::default()
        .title(Span::styled(
            format!(" ÉTAPE {}: {} ", step_idx + 1, title.to_uppercase()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = Vec::new();

    // Objective/Description
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(
            goal_label,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(description, Style::default().fg(Color::White)),
    ]));
    lines.push(Line::from(""));

    // Explanation (only show for English, French has simplified content)
    if locale == NexusLocale::En {
        if let Some(step) = STEPS.get(step_idx) {
            lines.push(Line::from(Span::styled(
                format!("  {insight_label}"),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                "  ─────────────────────────────────────────────────────────────────────",
                Style::default().fg(Color::DarkGray),
            )));

            for explanation_line in step.explanation {
                if explanation_line.is_empty() {
                    lines.push(Line::from(""));
                } else {
                    lines.push(Line::from(vec![
                        Span::styled("  ", Style::default()),
                        Span::styled(*explanation_line, Style::default().fg(Color::White)),
                    ]));
                }
            }
            lines.push(Line::from(""));
        }
    }

    // Practice tasks
    lines.push(Line::from(Span::styled(
        format!("  {try_label}"),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "  ─────────────────────────────────────────────────────────────────────",
        Style::default().fg(Color::DarkGray),
    )));

    let task_completions = tutorial
        .tasks_completed
        .get(step_idx)
        .cloned()
        .unwrap_or_default();

    // Use i18n tasks if available, otherwise fall back to STEPS
    let task_count = i18n_tasks.len().max(
        STEPS.get(step_idx).map(|s| s.tasks.len()).unwrap_or(0)
    );

    for i in 0..task_count {
        let is_complete = task_completions.get(i).copied().unwrap_or(false);
        let checkbox = if is_complete { "☑" } else { "☐" };
        let style = if is_complete {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::White)
        };

        // Get task description from i18n or fallback
        let task_desc = i18n_tasks.get(i).copied().unwrap_or_else(|| {
            STEPS.get(step_idx)
                .and_then(|s| s.tasks.get(i))
                .map(|t| t.description)
                .unwrap_or("Task")
        });

        lines.push(Line::from(vec![
            Span::styled(format!("  {} {}. ", checkbox, i + 1), style),
            Span::styled(task_desc, style),
        ]));

        // Show hints only in English (detailed content)
        if locale == NexusLocale::En {
            if let Some(step) = STEPS.get(step_idx) {
                if let Some(task) = step.tasks.get(i) {
                    if let Some(hint) = task.hint {
                        lines.push(Line::from(vec![
                            Span::styled("       ", Style::default()),
                            Span::styled(
                                format!("{hint_prefix}{hint}"),
                                Style::default().fg(Color::DarkGray),
                            ),
                        ]));
                    }
                }
            }
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  {nav_hint}"),
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

/// Render navigation hint at the bottom.
fn render_navigation(f: &mut Frame, tutorial: &TutorialState, locale: NexusLocale, area: Rect) {
    // i18n labels
    let (prev_label, next_label, complete_label, finish_label, reset_label, graph_label) = match locale {
        NexusLocale::En => ("[p: previous]", "[n: next step]", "[Complete!]", "[Finish tasks to complete]", "[r: reset]", "[1: Graph mode]"),
        NexusLocale::Fr => ("[p: précédent]", "[n: suivant]", "[Terminé !]", "[Finir les tâches]", "[r: réinit.]", "[1: mode Graphe]"),
    };

    let prev = if tutorial.current_step > 0 {
        prev_label
    } else {
        ""
    };
    let next = if tutorial.current_step < TUTORIAL_STEPS - 1 {
        next_label
    } else if tutorial.complete {
        complete_label
    } else {
        finish_label
    };

    let progress_bar = format!(
        "[{}{}] {}%",
        "█".repeat(tutorial.progress_percent() / 10),
        "░".repeat(10 - tutorial.progress_percent() / 10),
        tutorial.progress_percent()
    );

    let line = Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(prev, Style::default().fg(Color::DarkGray)),
        Span::styled("  ", Style::default()),
        Span::styled(next, Style::default().fg(Color::Green)),
        Span::styled(format!("  {reset_label}  {graph_label}  "), Style::default().fg(Color::DarkGray)),
        Span::styled(progress_bar, Style::default().fg(Color::Cyan)),
    ]);

    let paragraph = Paragraph::new(line);
    f.render_widget(paragraph, area);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_steps_count() {
        assert_eq!(STEPS.len(), TUTORIAL_STEPS);
    }

    #[test]
    fn test_tutorial_state_new() {
        let state = TutorialState::new();
        assert_eq!(state.current_step, 0);
        assert!(!state.complete);
        assert_eq!(state.tasks_completed.len(), TUTORIAL_STEPS);
    }

    #[test]
    fn test_progress_percent_initial() {
        let state = TutorialState::new();
        assert_eq!(state.progress_percent(), 0);
    }

    #[test]
    fn test_toggle_task() {
        let mut state = TutorialState::new();
        assert!(!state.tasks_completed[0][0]);

        state.toggle_task(0);
        assert!(state.tasks_completed[0][0]);

        state.toggle_task(0);
        assert!(!state.tasks_completed[0][0]);
    }

    #[test]
    fn test_mark_step_complete() {
        let mut state = TutorialState::new();

        state.mark_step_complete();

        // All tasks in step 0 should be complete
        assert!(state.tasks_completed[0].iter().all(|&t| t));
    }

    #[test]
    fn test_next_prev_step() {
        let mut state = TutorialState::new();
        assert_eq!(state.current_step, 0);

        state.next_step();
        assert_eq!(state.current_step, 1);

        state.prev_step();
        assert_eq!(state.current_step, 0);

        // Should not go below 0
        state.prev_step();
        assert_eq!(state.current_step, 0);
    }

    #[test]
    fn test_next_step_max() {
        let mut state = TutorialState::new();
        state.current_step = TUTORIAL_STEPS - 1;

        state.next_step();
        assert_eq!(state.current_step, TUTORIAL_STEPS - 1); // Should not exceed max
    }

    #[test]
    fn test_reset() {
        let mut state = TutorialState::new();
        state.current_step = 3;
        state.tasks_completed[0][0] = true;
        state.tasks_completed[1][0] = true;

        state.reset();

        assert_eq!(state.current_step, 0);
        assert!(!state.tasks_completed[0][0]);
        assert!(!state.tasks_completed[1][0]);
    }

    #[test]
    fn test_step_status() {
        let mut state = TutorialState::new();
        state.current_step = 2;

        assert_eq!(state.step_status(0), "●"); // Past
        assert_eq!(state.step_status(1), "●"); // Past
        assert_eq!(state.step_status(2), "◉"); // Current
        assert_eq!(state.step_status(3), "○"); // Future
        assert_eq!(state.step_status(4), "○"); // Future
    }

    #[test]
    fn test_completed_steps() {
        let mut state = TutorialState::new();
        assert_eq!(state.completed_steps(), 0);

        // Complete all tasks in step 0
        for i in 0..state.tasks_completed[0].len() {
            state.tasks_completed[0][i] = true;
        }
        assert_eq!(state.completed_steps(), 1);
    }

    #[test]
    fn test_is_step_complete() {
        let mut state = TutorialState::new();
        assert!(!state.is_step_complete());

        state.mark_step_complete();
        assert!(state.is_step_complete());
    }

    #[test]
    fn test_each_step_has_tasks() {
        for step in &STEPS {
            assert!(!step.tasks.is_empty(), "Step {} should have tasks", step.id);
        }
    }

    #[test]
    fn test_tutorial_completion() {
        let mut state = TutorialState::new();

        // Complete all steps
        for step_idx in 0..TUTORIAL_STEPS {
            state.current_step = step_idx;
            state.mark_step_complete();
        }

        assert!(state.complete);
        assert_eq!(state.progress_percent(), 100);
    }
}
