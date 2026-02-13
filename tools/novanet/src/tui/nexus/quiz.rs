//! Quiz mode for Nexus - Interactive learning about NovaNet taxonomy.
//!
//! v0.12.0 Enhanced Quiz with:
//! - 5 question categories: Realms, Layers, Traits, Arcs, Generation
//! - Category badges with color-coded progress
//! - Visual category indicators in question display
//! - Category breakdown in completion screen with per-category scores
//! - 30+ questions with multiple choice AND True/False types
//! - Immediate feedback with explanations

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use super::NexusLocale;
use crate::tui::app::App;

/// Question categories aligned with NovaNet classification (v0.12.0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuizCategory {
    /// Questions about shared/org realms.
    Realms,
    /// Questions about the 10 layers (4 shared + 6 org).
    Layers,
    /// Questions about the 5 traits: defined, authored, imported, generated, retrieved.
    Traits,
    /// Questions about arcs, arc families, and relationships.
    Arcs,
    /// Questions about generation vs translation, knowledge atoms, pipeline.
    Generation,
}

impl QuizCategory {
    /// Get category icon (v0.12.0).
    pub fn icon(&self) -> &'static str {
        match self {
            QuizCategory::Realms => "◉",
            QuizCategory::Layers => "◫",
            QuizCategory::Traits => "◆",
            QuizCategory::Arcs => "→",
            QuizCategory::Generation => "⚙",
        }
    }

    /// Get category color (v0.12.0).
    pub fn color(&self) -> Color {
        match self {
            QuizCategory::Realms => Color::Cyan,
            QuizCategory::Layers => Color::Magenta,
            QuizCategory::Traits => Color::Yellow,
            QuizCategory::Arcs => Color::Green,
            QuizCategory::Generation => Color::Blue,
        }
    }

    /// Get category display name.
    pub fn name(&self, locale: NexusLocale) -> &'static str {
        match locale {
            NexusLocale::En => match self {
                QuizCategory::Realms => "Realms",
                QuizCategory::Layers => "Layers",
                QuizCategory::Traits => "Traits",
                QuizCategory::Arcs => "Arcs",
                QuizCategory::Generation => "Generation",
            },
            NexusLocale::Fr => match self {
                QuizCategory::Realms => "Royaumes",
                QuizCategory::Layers => "Couches",
                QuizCategory::Traits => "Traits",
                QuizCategory::Arcs => "Arcs",
                QuizCategory::Generation => "Génération",
            },
        }
    }

    /// All categories in display order.
    pub fn all() -> &'static [QuizCategory] {
        &[
            QuizCategory::Realms,
            QuizCategory::Layers,
            QuizCategory::Traits,
            QuizCategory::Arcs,
            QuizCategory::Generation,
        ]
    }
}

/// Question type: Multiple Choice (4 options) or True/False (2 options).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuizQuestionType {
    /// Standard 4-option multiple choice (A, B, C, D).
    #[default]
    MultipleChoice,
    /// True/False question (faster pacing, binary choice).
    TrueFalse,
}

impl QuizQuestionType {
    /// Maximum option index for this question type.
    pub fn max_option(&self) -> usize {
        match self {
            QuizQuestionType::MultipleChoice => 3, // 0-3 for A-D
            QuizQuestionType::TrueFalse => 1,      // 0-1 for True/False
        }
    }

    /// Option labels for this question type.
    pub fn labels(&self) -> &'static [&'static str] {
        match self {
            QuizQuestionType::MultipleChoice => &["A", "B", "C", "D"],
            QuizQuestionType::TrueFalse => &["T", "F"],
        }
    }

    /// Icon for question type indicator.
    pub fn icon(&self) -> &'static str {
        match self {
            QuizQuestionType::MultipleChoice => "▣",
            QuizQuestionType::TrueFalse => "◐",
        }
    }
}

/// A quiz question with multiple answer options.
#[derive(Debug, Clone)]
pub struct QuizQuestion {
    /// The question text.
    pub question: &'static str,
    /// The answer options (4 for MultipleChoice, 2 for TrueFalse).
    pub options: [&'static str; 4],
    /// Index of the correct answer.
    pub correct: usize,
    /// Explanation shown after answering.
    pub explanation: &'static str,
    /// Category for grouping and badges (v0.12.0).
    pub category: QuizCategory,
    /// Question type: MultipleChoice (default) or TrueFalse.
    pub question_type: QuizQuestionType,
}

impl QuizQuestion {
    /// Get the effective number of options for this question.
    pub fn option_count(&self) -> usize {
        match self.question_type {
            QuizQuestionType::MultipleChoice => 4,
            QuizQuestionType::TrueFalse => 2,
        }
    }
}

/// State of the quiz within Nexus mode.
#[derive(Debug, Clone, Default)]
pub struct QuizState {
    /// Current question index (0-based).
    pub current_question: usize,
    /// Currently selected option (0-3).
    pub selected_option: usize,
    /// Number of correct answers.
    pub score: usize,
    /// Whether the current question has been answered.
    pub answered: bool,
    /// Whether the quiz is complete.
    pub complete: bool,
    /// High score (persisted across sessions).
    pub high_score: Option<usize>,
    /// Track correct/incorrect per question for category breakdown (v0.12.0).
    /// Index matches QUESTIONS, true = correct, false = incorrect.
    pub answers: Vec<bool>,
}

impl QuizState {
    /// Create a new quiz state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the quiz to start over (preserves high score).
    pub fn reset(&mut self) {
        let high_score = self.high_score;
        *self = Self::default();
        self.high_score = high_score;
    }

    /// Move selection up.
    pub fn select_up(&mut self) {
        if !self.answered && self.selected_option > 0 {
            self.selected_option -= 1;
        }
    }

    /// Move selection down, respecting question type bounds.
    pub fn select_down(&mut self, question: Option<&QuizQuestion>) {
        if self.answered {
            return;
        }
        let max = question
            .map(|q| q.question_type.max_option())
            .unwrap_or(3);
        if self.selected_option < max {
            self.selected_option += 1;
        }
    }

    /// Submit the current answer.
    pub fn submit_answer(&mut self, questions: &[QuizQuestion]) {
        if self.answered || self.complete {
            return;
        }
        if let Some(q) = questions.get(self.current_question) {
            let is_correct = self.selected_option == q.correct;
            if is_correct {
                self.score += 1;
            }
            // Track answer for category breakdown (v0.12.0)
            self.answers.push(is_correct);
            self.answered = true;
        }
    }

    /// Calculate score per category (v0.12.0).
    /// Returns (correct, total) for each category.
    pub fn category_scores(&self, questions: &[QuizQuestion]) -> Vec<(QuizCategory, usize, usize)> {
        let mut scores: Vec<(QuizCategory, usize, usize)> = Vec::new();

        for cat in QuizCategory::all() {
            let mut correct = 0;
            let mut total = 0;

            for (i, q) in questions.iter().enumerate() {
                if q.category == *cat {
                    total += 1;
                    if let Some(&is_correct) = self.answers.get(i) {
                        if is_correct {
                            correct += 1;
                        }
                    }
                }
            }

            if total > 0 {
                scores.push((*cat, correct, total));
            }
        }

        scores
    }

    /// Get badge emoji for category performance (v0.12.0).
    pub fn category_badge(correct: usize, total: usize) -> &'static str {
        if total == 0 {
            return "○";
        }
        let pct = (correct as f64 / total as f64 * 100.0) as u8;
        match pct {
            100 => "★",      // Perfect
            75..=99 => "◆",  // Great
            50..=74 => "●",  // Good
            _ => "○",        // Keep learning
        }
    }

    /// Move to the next question or complete the quiz.
    /// Returns true if this completes the quiz (for persistence trigger).
    pub fn next_question(&mut self, questions: &[QuizQuestion]) -> bool {
        if !self.answered {
            return false;
        }
        if self.current_question + 1 >= questions.len() {
            self.complete = true;
            // Update high score if current score beats it
            if self.high_score.map(|h| self.score > h).unwrap_or(true) {
                self.high_score = Some(self.score);
            }
            true // Quiz just completed
        } else {
            self.current_question += 1;
            self.selected_option = 0;
            self.answered = false;
            false
        }
    }

    /// Check if current score is a new high score.
    pub fn is_new_high_score(&self) -> bool {
        self.complete && self.high_score == Some(self.score)
    }
}

/// All quiz questions about NovaNet taxonomy.
/// v0.12.0: 36 questions (30 multiple choice + 6 True/False) across 5 categories.
pub const QUESTIONS: &[QuizQuestion] = &[
    // ═══════════════════════════════════════════════════════════════════════════
    // REALMS (6 questions)
    // ═══════════════════════════════════════════════════════════════════════════
    QuizQuestion {
        question: "How many realms does NovaNet v11.5 have?",
        options: ["1", "2", "3", "4"],
        correct: 1,
        explanation: "NovaNet has 2 realms: Shared (universal, READ-ONLY) and Org (organization-specific).",
        category: QuizCategory::Realms,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "What is the total node count in NovaNet v11.5+?",
        options: ["50", "55", "59", "65"],
        correct: 2,
        explanation: "59 total nodes: 39 shared + 20 org. v0.12.0 refined SEO/GEO and removed obsolete nodes.",
        category: QuizCategory::Realms,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "The Shared realm is...",
        options: ["Read-Write", "Read-Only", "Write-Only", "Admin-Only"],
        correct: 1,
        explanation: "Shared realm is READ-ONLY universal knowledge. All business content lives in Org realm.",
        category: QuizCategory::Realms,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "How many nodes are in the Shared realm?",
        options: ["20", "30", "39", "45"],
        correct: 2,
        explanation: "Shared realm has 39 nodes across 4 layers: config, locale, geography, knowledge.",
        category: QuizCategory::Realms,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "How many nodes are in the Org realm?",
        options: ["15", "20", "25", "30"],
        correct: 1,
        explanation: "Org realm has 20 nodes across 6 layers: config, foundation, structure, semantic, instruction, output.",
        category: QuizCategory::Realms,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "What was 'global' realm renamed to?",
        options: ["public", "shared", "common", "universal"],
        correct: 1,
        explanation: "v11.2: 'global' → 'shared' (describes WHAT), 'tenant' → 'org' (describes WHO).",
        category: QuizCategory::Realms,
        question_type: QuizQuestionType::MultipleChoice,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // LAYERS (6 questions)
    // ═══════════════════════════════════════════════════════════════════════════
    QuizQuestion {
        question: "How many layers does the Shared realm have?",
        options: ["3", "4", "5", "6"],
        correct: 1,
        explanation: "Shared has 4 layers: config, locale, geography, knowledge (39 nodes total).",
        category: QuizCategory::Layers,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "How many layers does the Org realm have?",
        options: ["4", "5", "6", "7"],
        correct: 2,
        explanation: "Org has 6 layers: config, foundation, structure, semantic, instruction, output (20 nodes).",
        category: QuizCategory::Layers,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Where does the Locale node live?",
        options: ["shared/locale", "shared/config", "org/config", "shared/knowledge"],
        correct: 1,
        explanation: "Locale moved to shared/config in v11.5 because it's a DEFINITION ('defined' trait), not settings.",
        category: QuizCategory::Layers,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Which layer contains Entity and EntityContent?",
        options: ["structure", "semantic", "foundation", "output"],
        correct: 1,
        explanation: "Entity and EntityContent live in the semantic layer - they represent meaning and knowledge.",
        category: QuizCategory::Layers,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "PageGenerated and BlockGenerated live in which layer?",
        options: ["semantic", "structure", "instruction", "output"],
        correct: 3,
        explanation: "Generated nodes (PageGenerated, BlockGenerated, OutputArtifact) live in the output layer.",
        category: QuizCategory::Layers,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Which layer contains Page and Block nodes?",
        options: ["semantic", "structure", "foundation", "output"],
        correct: 1,
        explanation: "Page and Block live in the structure layer - they define content organization.",
        category: QuizCategory::Layers,
        question_type: QuizQuestionType::MultipleChoice,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // TRAITS (6 questions)
    // ═══════════════════════════════════════════════════════════════════════════
    QuizQuestion {
        question: "How many node traits exist in v0.12.0?",
        options: ["3", "4", "5", "6"],
        correct: 2,
        explanation: "5 traits: defined, authored, imported, generated, retrieved (ADR-024 Data Origin).",
        category: QuizCategory::Traits,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Which trait indicates LLM-generated output?",
        options: ["authored", "imported", "generated", "retrieved"],
        correct: 2,
        explanation: "Generated trait indicates LLM output (PageGenerated, BlockGenerated, OutputArtifact).",
        category: QuizCategory::Traits,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "What border style indicates a 'defined' node?",
        options: ["dashed", "dotted", "double", "solid"],
        correct: 3,
        explanation: "Defined=solid, authored=dashed, imported=double, generated=dotted, retrieved=dotted-thin.",
        category: QuizCategory::Traits,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "What quick jump key goes to 'generated' trait?",
        options: ["gd", "gg", "gn", "go"],
        correct: 1,
        explanation: "gg=generated. Also: gd=defined, ga=authored, gi=imported, gr=retrieved.",
        category: QuizCategory::Traits,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Which trait is for human-written locale content?",
        options: ["defined", "authored", "imported", "generated"],
        correct: 1,
        explanation: "Authored trait is for human-written per-locale content (EntityContent, ProjectContent).",
        category: QuizCategory::Traits,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Which trait is for external API data?",
        options: ["imported", "generated", "retrieved", "authored"],
        correct: 2,
        explanation: "Retrieved trait is for data fetched from external APIs (SEOKeywordMetrics, GEOMetrics).",
        category: QuizCategory::Traits,
        question_type: QuizQuestionType::MultipleChoice,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // ARCS (6 questions)
    // ═══════════════════════════════════════════════════════════════════════════
    QuizQuestion {
        question: "How many arc families exist in NovaNet?",
        options: ["3", "4", "5", "6"],
        correct: 2,
        explanation: "5 arc families: ownership, localization, semantic, generation, mining.",
        category: QuizCategory::Arcs,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "What arc scope crosses realm boundaries?",
        options: ["intra_realm", "cross_realm", "multi_realm", "global_scope"],
        correct: 1,
        explanation: "cross_realm scope for arcs that cross between Shared and Org realms.",
        category: QuizCategory::Arcs,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "What was EntityL10n renamed to?",
        options: ["EntityContent", "EntityGenerated", "EntityOutput", "EntityData"],
        correct: 0,
        explanation: "EntityL10n → EntityContent (semantic layer, 'authored' trait). Content = locale-specific.",
        category: QuizCategory::Arcs,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "HAS_CONTENT arc connects Entity to what?",
        options: ["Page", "Block", "EntityContent", "Locale"],
        correct: 2,
        explanation: "HAS_CONTENT: Entity → EntityContent (ownership family, localization purpose).",
        category: QuizCategory::Arcs,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Which arc family is for Page-Block relationships?",
        options: ["localization", "semantic", "ownership", "generation"],
        correct: 2,
        explanation: "Ownership family includes HAS_BLOCK, HAS_PAGE, HAS_CONTENT - parent-child relationships.",
        category: QuizCategory::Arcs,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "How many total arcs (ArcClass) in v0.12.0?",
        options: ["50", "85", "114", "150"],
        correct: 2,
        explanation: "114 arc types defined across 5 families, covering all node relationships.",
        category: QuizCategory::Arcs,
        question_type: QuizQuestionType::MultipleChoice,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // GENERATION (6 questions)
    // ═══════════════════════════════════════════════════════════════════════════
    QuizQuestion {
        question: "What does NovaNet do with content?",
        options: ["Translation", "Transcription", "Generation", "Compilation"],
        correct: 2,
        explanation: "NovaNet GENERATES content natively per locale, NOT translation. Native generation preserves cultural nuance.",
        category: QuizCategory::Generation,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "What was PageL10n renamed to?",
        options: ["PageContent", "PageGenerated", "PageOutput", "PageLocal"],
        correct: 1,
        explanation: "PageL10n → PageGenerated (output layer, generated trait). 'Generated' = LLM output.",
        category: QuizCategory::Generation,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Knowledge atoms are loaded how?",
        options: ["All at once", "Selectively per context", "Never loaded", "Cached globally"],
        correct: 1,
        explanation: "Selective LLM loading: Load 50 relevant Terms, not 20K JSON blob. Graph queries filter by context.",
        category: QuizCategory::Generation,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "What's the generation pipeline order?",
        options: [
            "Entity → Structure → Knowledge → Output",
            "Knowledge → Entity → Structure → Output",
            "Structure → Entity → Output → Knowledge",
            "Output → Knowledge → Entity → Structure",
        ],
        correct: 1,
        explanation: "Knowledge (imported) feeds Entity (defined) which structures Pages/Blocks for Output (generated).",
        category: QuizCategory::Generation,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Term, Expression, Pattern have which trait?",
        options: ["defined", "authored", "imported", "generated"],
        correct: 2,
        explanation: "Knowledge atoms (Term, Expression, Pattern) have 'imported' trait - external knowledge brought in.",
        category: QuizCategory::Generation,
        question_type: QuizQuestionType::MultipleChoice,
    },
    QuizQuestion {
        question: "Why native generation over translation?",
        options: [
            "Faster processing",
            "Cheaper API costs",
            "Preserves cultural nuance",
            "Smaller file sizes",
        ],
        correct: 2,
        explanation: "Native generation preserves cultural nuance (idioms, humor, formality) that translation loses.",
        category: QuizCategory::Generation,
        question_type: QuizQuestionType::MultipleChoice,
    },
    // ═══════════════════════════════════════════════════════════════════════════
    // TRUE/FALSE QUESTIONS (6 questions - faster pacing)
    // ═══════════════════════════════════════════════════════════════════════════
    QuizQuestion {
        question: "NovaNet uses translation to create localized content.",
        options: ["True", "False", "", ""],
        correct: 1, // False
        explanation: "NovaNet uses NATIVE GENERATION, not translation. Content is generated per locale from invariant entities.",
        category: QuizCategory::Generation,
        question_type: QuizQuestionType::TrueFalse,
    },
    QuizQuestion {
        question: "The Shared realm can be modified by organizations.",
        options: ["True", "False", "", ""],
        correct: 1, // False
        explanation: "Shared realm is READ-ONLY. It contains universal knowledge that all organizations can read but not modify.",
        category: QuizCategory::Realms,
        question_type: QuizQuestionType::TrueFalse,
    },
    QuizQuestion {
        question: "Entity nodes are invariant across all locales.",
        options: ["True", "False", "", ""],
        correct: 0, // True
        explanation: "Entity nodes have 'defined' trait - they're invariant. EntityContent holds the locale-specific authored content.",
        category: QuizCategory::Traits,
        question_type: QuizQuestionType::TrueFalse,
    },
    QuizQuestion {
        question: "NovaNet has exactly 10 layers (4 shared + 6 org).",
        options: ["True", "False", "", ""],
        correct: 0, // True
        explanation: "Shared: config, locale, geography, knowledge (4). Org: config, foundation, structure, semantic, instruction, output (6).",
        category: QuizCategory::Layers,
        question_type: QuizQuestionType::TrueFalse,
    },
    QuizQuestion {
        question: "HAS_CONTENT is part of the 'generation' arc family.",
        options: ["True", "False", "", ""],
        correct: 1, // False
        explanation: "HAS_CONTENT is in the 'localization' family. HAS_GENERATED is in the 'generation' family.",
        category: QuizCategory::Arcs,
        question_type: QuizQuestionType::TrueFalse,
    },
    QuizQuestion {
        question: "Knowledge atoms (Term, Expression, Pattern) are locale-specific.",
        options: ["True", "False", "", ""],
        correct: 0, // True
        explanation: "Unlike Entities (invariant + Content for ALL locales), atoms exist only where needed. fr-FR may have 20K Terms, sw-KE may have 500.",
        category: QuizCategory::Generation,
        question_type: QuizQuestionType::TrueFalse,
    },
];

/// Render the Quiz tab content (v0.12.0 enhanced).
pub fn render_quiz_tab(f: &mut Frame, app: &App, area: Rect) {
    let quiz = &app.nexus.quiz;
    let locale = app.nexus.locale;
    let questions = QUESTIONS;

    // Layout: category badges + question + options + status
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Category badges bar (v0.12.0)
            Constraint::Length(5), // Question
            Constraint::Length(8), // Options
            Constraint::Min(1),    // Explanation/Result
        ])
        .margin(1)
        .split(area);

    // i18n labels
    let (complete_label, score_label, question_label) = match locale {
        NexusLocale::En => ("Quiz Complete", "Score", "Question"),
        NexusLocale::Fr => ("Quiz Terminé", "Score", "Question"),
    };

    // Main block with category indicator
    let current_cat = questions
        .get(quiz.current_question)
        .map(|q| q.category)
        .unwrap_or(QuizCategory::Realms);

    let title = if quiz.complete {
        format!(
            " {} - {}: {}/{} ",
            complete_label,
            score_label,
            quiz.score,
            questions.len()
        )
    } else {
        format!(
            " {} {} {}/{} │ {} {} ",
            current_cat.icon(),
            question_label,
            quiz.current_question + 1,
            questions.len(),
            current_cat.name(locale),
            current_cat.icon(),
        )
    };

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(current_cat.color())
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    f.render_widget(block, area);

    // Render category badges bar (v0.12.0)
    render_category_badges(f, quiz, locale, chunks[0]);

    if quiz.complete {
        render_quiz_complete(f, app, locale, &chunks[1..]);
    } else if let Some(question) = questions.get(quiz.current_question) {
        render_question(f, app, locale, question, &chunks[1..]);
    }
}

/// Render category badges showing progress (v0.12.0).
fn render_category_badges(f: &mut Frame, quiz: &QuizState, locale: NexusLocale, area: Rect) {
    let mut spans: Vec<Span> = Vec::new();

    // Calculate progress for each category
    for cat in QuizCategory::all() {
        let mut answered = 0;
        let mut correct = 0;
        let mut total = 0;

        for (i, q) in QUESTIONS.iter().enumerate() {
            if q.category == *cat {
                total += 1;
                if let Some(&is_correct) = quiz.answers.get(i) {
                    answered += 1;
                    if is_correct {
                        correct += 1;
                    }
                }
            }
        }

        // Badge style: dim if not started, colored if in progress, styled if complete
        let badge = QuizState::category_badge(correct, answered);
        let (style, text) = if answered == 0 {
            (
                Style::default().fg(Color::DarkGray),
                format!(" {} {} ", cat.icon(), cat.name(locale)),
            )
        } else if answered == total {
            (
                Style::default()
                    .fg(cat.color())
                    .add_modifier(Modifier::BOLD),
                format!(" {} {} {}/{} {} ", cat.icon(), cat.name(locale), correct, total, badge),
            )
        } else {
            (
                Style::default().fg(cat.color()),
                format!(" {} {} {}/{} ", cat.icon(), cat.name(locale), correct, answered),
            )
        };

        spans.push(Span::styled(text, style));
        spans.push(Span::raw("│"));
    }

    // Remove trailing separator
    if !spans.is_empty() {
        spans.pop();
    }

    let badges = Paragraph::new(Line::from(spans));
    f.render_widget(badges, area);
}

/// Render the current question and options.
fn render_question(
    f: &mut Frame,
    app: &App,
    locale: NexusLocale,
    question: &QuizQuestion,
    chunks: &[Rect],
) {
    let quiz = &app.nexus.quiz;

    // i18n labels
    let (explanation_label, next_hint, score_label, answered_label, nav_hint) = match locale {
        NexusLocale::En => (
            "Explanation:",
            "[Enter: next question]",
            "Score",
            "answered",
            "[j/k or ↑/↓: select] [Enter: submit]",
        ),
        NexusLocale::Fr => (
            "Explication:",
            "[Entrée: question suivante]",
            "Score",
            "répondu",
            "[j/k ou ↑/↓: sélectionner] [Entrée: valider]",
        ),
    };

    // Question text
    let question_text = Paragraph::new(question.question)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(question_text, chunks[0]);

    // Options - respect question type (True/False vs Multiple Choice)
    let option_count = question.option_count();
    let labels = question.question_type.labels();
    let mut option_lines: Vec<Line> = Vec::new();

    for (i, opt) in question.options.iter().take(option_count).enumerate() {
        let is_selected = i == quiz.selected_option;
        let is_correct = i == question.correct;

        let (prefix, style) = if quiz.answered {
            // After answering, show correct/incorrect
            if is_correct {
                (
                    "✓ ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
            } else if is_selected {
                ("✗ ", Style::default().fg(Color::Red))
            } else {
                ("  ", Style::default().fg(Color::DarkGray))
            }
        } else {
            // Before answering, show selection cursor
            if is_selected {
                (
                    "▶ ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                ("  ", Style::default().fg(Color::White))
            }
        };

        let label = labels.get(i).unwrap_or(&"?");
        option_lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{}) ", label), Style::default().fg(Color::Yellow)),
            Span::styled(*opt, style),
        ]));
    }
    let options_para = Paragraph::new(option_lines);
    f.render_widget(options_para, chunks[1]);

    // Explanation or hint
    let explanation_area = chunks[2];
    if quiz.answered {
        let mut lines: Vec<Line> = vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("📚 {}", explanation_label),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                question.explanation,
                Style::default().fg(Color::Rgb(180, 180, 200)),
            )),
            Line::from(""),
            Line::from(Span::styled(
                next_hint,
                Style::default().fg(Color::DarkGray),
            )),
        ];

        // Show score so far
        let current = quiz.current_question + 1;
        lines.push(Line::from(Span::styled(
            format!(
                "{}: {}/{} {}",
                score_label, quiz.score, current, answered_label
            ),
            Style::default().fg(Color::Cyan),
        )));

        let para = Paragraph::new(lines).wrap(Wrap { trim: true });
        f.render_widget(para, explanation_area);
    } else {
        let hint = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(nav_hint, Style::default().fg(Color::DarkGray))),
        ]);
        f.render_widget(hint, explanation_area);
    }
}

/// Render the quiz completion screen (v0.12.0 enhanced with category breakdown and streak).
fn render_quiz_complete(f: &mut Frame, app: &App, locale: NexusLocale, chunks: &[Rect]) {
    let quiz = &app.nexus.quiz;
    let total = QUESTIONS.len();
    let pct = (quiz.score as f64 / total as f64 * 100.0) as u8;

    // Load streak from persistence (fast, file is tiny)
    let progress = super::persistence::TutorialProgress::load();
    let streak = progress.current_streak;
    let best_streak = progress.best_streak;

    // i18n labels
    let (
        final_score_label,
        restart_hint,
        expert,
        great,
        good,
        keep_learning,
        category_breakdown_label,
        streak_label,
    ) = match locale {
        NexusLocale::En => (
            "Final Score",
            "[r: restart quiz] [Tab: other tabs]",
            "★ Expert!",
            "◆ Great job!",
            "● Good effort!",
            "○ Keep learning!",
            "Category Breakdown",
            "Streak",
        ),
        NexusLocale::Fr => (
            "Score Final",
            "[r: recommencer] [Tab: autres onglets]",
            "★ Expert !",
            "◆ Excellent !",
            "● Bon travail !",
            "○ Continue d'apprendre !",
            "Détail par Catégorie",
            "Série",
        ),
    };

    let (grade, grade_color) = match pct {
        90..=100 => (expert, Color::Green),
        75..=89 => (great, Color::Cyan),
        60..=74 => (good, Color::Yellow),
        _ => (keep_learning, Color::Magenta),
    };

    // Build result lines with category breakdown
    let mut result_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("{}: {}/{} ({}%)", final_score_label, quiz.score, total, pct),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            grade,
            Style::default()
                .fg(grade_color)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("── {} ──", category_breakdown_label),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    // Category breakdown with badges (v0.12.0)
    let cat_scores = quiz.category_scores(QUESTIONS);
    for (cat, correct, cat_total) in cat_scores {
        let badge = QuizState::category_badge(correct, cat_total);
        let cat_pct = if cat_total > 0 {
            (correct as f64 / cat_total as f64 * 100.0) as u8
        } else {
            0
        };

        // Progress bar
        let bar_width = 10;
        let filled = (bar_width * correct / cat_total.max(1)).min(bar_width);
        let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);

        result_lines.push(Line::from(vec![
            Span::styled(
                format!(" {} ", cat.icon()),
                Style::default().fg(cat.color()),
            ),
            Span::styled(
                format!("{:<12}", cat.name(locale)),
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!(" {} ", bar),
                Style::default().fg(cat.color()),
            ),
            Span::styled(
                format!("{}/{} ", correct, cat_total),
                Style::default()
                    .fg(cat.color())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("({}%) ", cat_pct),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                badge,
                Style::default()
                    .fg(if badge == "★" { Color::Yellow } else { cat.color() })
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    // Streak display (v0.12.0)
    result_lines.push(Line::from(""));
    let streak_icon = if streak >= 7 {
        "🔥🔥"
    } else if streak >= 3 {
        "🔥"
    } else {
        "○"
    };
    let streak_color = if streak >= 7 {
        Color::Yellow
    } else if streak >= 3 {
        Color::Rgb(255, 165, 0) // Orange
    } else if streak >= 1 {
        Color::Cyan
    } else {
        Color::DarkGray
    };

    result_lines.push(Line::from(vec![
        Span::styled(
            format!(" {} {}: ", streak_icon, streak_label),
            Style::default()
                .fg(streak_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{} day{}", streak, if streak == 1 { "" } else { "s" }),
            Style::default().fg(streak_color),
        ),
        Span::styled(
            format!(" (best: {})", best_streak),
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    result_lines.push(Line::from(""));
    result_lines.push(Line::from(Span::styled(
        restart_hint,
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(result_lines);
    // Use entire area for results
    let full_area = Rect {
        x: chunks[0].x,
        y: chunks[0].y,
        width: chunks[0].width,
        height: chunks[0].height + chunks[1].height + chunks[2].height,
    };
    f.render_widget(para, full_area);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz_state_default() {
        let state = QuizState::new();
        assert_eq!(state.current_question, 0);
        assert_eq!(state.selected_option, 0);
        assert_eq!(state.score, 0);
        assert!(!state.answered);
        assert!(!state.complete);
    }

    #[test]
    fn test_quiz_navigation() {
        let mut state = QuizState::new();

        // Use first question (multiple choice with max 3)
        let q = QUESTIONS.first();
        state.select_down(q);
        assert_eq!(state.selected_option, 1);

        state.select_down(q);
        assert_eq!(state.selected_option, 2);

        state.select_down(q);
        assert_eq!(state.selected_option, 3);

        // Can't go past 3 for multiple choice
        state.select_down(q);
        assert_eq!(state.selected_option, 3);

        state.select_up();
        assert_eq!(state.selected_option, 2);

        state.select_up();
        state.select_up();
        state.select_up();
        // Can't go below 0
        assert_eq!(state.selected_option, 0);
    }

    #[test]
    fn test_quiz_correct_answer() {
        let mut state = QuizState::new();

        // First question correct answer is index 1 (2 realms)
        state.selected_option = 1;
        state.submit_answer(QUESTIONS);

        assert!(state.answered);
        assert_eq!(state.score, 1);
    }

    #[test]
    fn test_quiz_incorrect_answer() {
        let mut state = QuizState::new();

        // First question correct answer is index 1, select wrong
        state.selected_option = 0;
        state.submit_answer(QUESTIONS);

        assert!(state.answered);
        assert_eq!(state.score, 0);
    }

    #[test]
    fn test_quiz_progression() {
        let mut state = QuizState::new();

        // Answer question 1
        state.selected_option = 1;
        state.submit_answer(QUESTIONS);
        assert!(state.answered);
        assert!(!state.complete);

        // Move to question 2
        state.next_question(QUESTIONS);
        assert_eq!(state.current_question, 1);
        assert_eq!(state.selected_option, 0);
        assert!(!state.answered);
    }

    #[test]
    fn test_quiz_completion() {
        let mut state = QuizState::new();

        // Simulate answering all questions
        for i in 0..QUESTIONS.len() {
            state.current_question = i;
            state.answered = false;
            state.submit_answer(QUESTIONS);
            state.next_question(QUESTIONS);
        }

        assert!(state.complete);
    }

    #[test]
    fn test_quiz_reset() {
        let mut state = QuizState::new();
        state.current_question = 5;
        state.score = 3;
        state.answered = true;

        state.reset();

        assert_eq!(state.current_question, 0);
        assert_eq!(state.score, 0);
        assert!(!state.answered);
    }

    #[test]
    fn test_questions_have_valid_correct_index() {
        for (i, q) in QUESTIONS.iter().enumerate() {
            let max_valid = q.option_count();
            assert!(
                q.correct < max_valid,
                "Question {} has invalid correct index: {} (max for {:?} is {})",
                i,
                q.correct,
                q.question_type,
                max_valid - 1
            );
        }
    }

    #[test]
    fn test_questions_count() {
        assert!(QUESTIONS.len() >= 15, "Expected at least 15 questions");
    }

    #[test]
    fn test_cannot_navigate_after_answered() {
        let mut state = QuizState::new();
        state.answered = true;

        let initial = state.selected_option;
        state.select_down(None);
        assert_eq!(state.selected_option, initial);

        state.select_up();
        assert_eq!(state.selected_option, initial);
    }

    #[test]
    fn test_true_false_question_bounds() {
        let mut state = QuizState::new();

        // Create a True/False question
        let tf_question = QuizQuestion {
            question: "Test question",
            options: ["True", "False", "", ""],
            correct: 0,
            explanation: "Test",
            category: QuizCategory::Realms,
            question_type: QuizQuestionType::TrueFalse,
        };

        // Can move from 0 to 1
        state.select_down(Some(&tf_question));
        assert_eq!(state.selected_option, 1);

        // Can't go past 1 for True/False
        state.select_down(Some(&tf_question));
        assert_eq!(state.selected_option, 1);
    }
}
