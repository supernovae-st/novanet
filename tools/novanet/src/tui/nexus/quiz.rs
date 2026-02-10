//! Quiz mode for Nexus - Interactive learning about NovaNet taxonomy.
//!
//! Tests knowledge of realms, layers, traits, arcs, and NovaNet principles.
//! 15 questions with multiple choice answers, immediate feedback.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::tui::app::App;

/// A quiz question with 4 answer options.
#[derive(Debug, Clone)]
pub struct QuizQuestion {
    /// The question text.
    pub question: &'static str,
    /// The 4 answer options.
    pub options: [&'static str; 4],
    /// Index of the correct answer (0-3).
    pub correct: usize,
    /// Explanation shown after answering.
    pub explanation: &'static str,
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
}

impl QuizState {
    /// Create a new quiz state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the quiz to start over.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Move selection up.
    pub fn select_up(&mut self) {
        if !self.answered && self.selected_option > 0 {
            self.selected_option -= 1;
        }
    }

    /// Move selection down.
    pub fn select_down(&mut self) {
        if !self.answered && self.selected_option < 3 {
            self.selected_option += 1;
        }
    }

    /// Submit the current answer.
    pub fn submit_answer(&mut self, questions: &[QuizQuestion]) {
        if self.answered || self.complete {
            return;
        }
        if let Some(q) = questions.get(self.current_question) {
            if self.selected_option == q.correct {
                self.score += 1;
            }
            self.answered = true;
        }
    }

    /// Move to the next question or complete the quiz.
    pub fn next_question(&mut self, questions: &[QuizQuestion]) {
        if !self.answered {
            return;
        }
        if self.current_question + 1 >= questions.len() {
            self.complete = true;
        } else {
            self.current_question += 1;
            self.selected_option = 0;
            self.answered = false;
        }
    }
}

/// All quiz questions about NovaNet taxonomy.
pub const QUESTIONS: &[QuizQuestion] = &[
    QuizQuestion {
        question: "How many realms does NovaNet v11.5 have?",
        options: ["1", "2", "3", "4"],
        correct: 1,
        explanation: "NovaNet has 2 realms: Shared (universal, READ-ONLY) and Org (organization-specific).",
    },
    QuizQuestion {
        question: "What does NovaNet do with content?",
        options: ["Translation", "Transcription", "Generation", "Compilation"],
        correct: 2,
        explanation: "NovaNet GENERATES content natively per locale, not translation. Entity → Generate → EntityContent.",
    },
    QuizQuestion {
        question: "How many node traits exist in v11.2+?",
        options: ["3", "4", "5", "6"],
        correct: 2,
        explanation: "5 traits: invariant, localized, knowledge, generated, aggregated. (derived split into generated+aggregated)",
    },
    QuizQuestion {
        question: "Which trait indicates LLM-generated output?",
        options: ["localized", "knowledge", "generated", "aggregated"],
        correct: 2,
        explanation: "Generated trait indicates LLM-generated output (PageGenerated, BlockGenerated, OutputArtifact).",
    },
    QuizQuestion {
        question: "How many layers does the Shared realm have in v11.5?",
        options: ["3", "4", "5", "6"],
        correct: 1,
        explanation: "Shared has 4 layers: config, locale, geography, knowledge (39 nodes total).",
    },
    QuizQuestion {
        question: "How many layers does the Org realm have in v11.5?",
        options: ["4", "5", "6", "7"],
        correct: 2,
        explanation: "Org has 6 layers: config, foundation, structure, semantic, instruction, output (21 nodes).",
    },
    QuizQuestion {
        question: "What is the total node count in NovaNet v11.5?",
        options: ["50", "55", "60", "65"],
        correct: 2,
        explanation: "60 total nodes: 39 shared + 21 org. Schema was refined to consolidate SEO/GEO to shared.",
    },
    QuizQuestion {
        question: "What was EntityL10n renamed to in v10.9?",
        options: ["EntityContent", "EntityGenerated", "EntityOutput", "EntityData"],
        correct: 0,
        explanation: "EntityL10n → EntityContent (semantic layer, localized trait). The 'Content' suffix indicates locale-specific semantic content.",
    },
    QuizQuestion {
        question: "What was PageL10n renamed to in v10.9?",
        options: ["PageContent", "PageGenerated", "PageOutput", "PageLocal"],
        correct: 1,
        explanation: "PageL10n → PageGenerated (output layer, generated trait). The 'Generated' suffix indicates LLM-generated output.",
    },
    QuizQuestion {
        question: "How many arc families exist in NovaNet?",
        options: ["3", "4", "5", "6"],
        correct: 2,
        explanation: "5 arc families: ownership, localization, semantic, generation, mining.",
    },
    QuizQuestion {
        question: "What arc scope is used when crossing realm boundaries?",
        options: ["intra_realm", "cross_realm", "multi_realm", "global_scope"],
        correct: 1,
        explanation: "cross_realm scope for arcs that cross between Shared and Org realms.",
    },
    QuizQuestion {
        question: "Where does the Locale node live in v11.5?",
        options: ["shared/locale", "shared/config", "org/config", "shared/knowledge"],
        correct: 1,
        explanation: "Locale moved to shared/config in v11.5 because it's a DEFINITION (invariant), not settings.",
    },
    QuizQuestion {
        question: "What border style indicates an invariant node?",
        options: ["dashed", "dotted", "double", "solid"],
        correct: 3,
        explanation: "Invariant nodes have solid borders. Localized=dashed, knowledge=double, generated=dotted.",
    },
    QuizQuestion {
        question: "What quick jump key goes to the 'generated' trait?",
        options: ["gd", "gg", "gn", "go"],
        correct: 1,
        explanation: "gg jumps to Generated trait (v11.3+). gi=invariant, gl=localized, gk=knowledge, ga=aggregated.",
    },
    QuizQuestion {
        question: "Knowledge atoms (Terms, Expressions) are loaded how?",
        options: ["All at once", "Selectively per context", "Never loaded", "Cached globally"],
        correct: 1,
        explanation: "Selective LLM loading: Load 50 relevant Terms, not 20K JSON blob. Graph queries filter by context.",
    },
];

/// Render the Quiz tab content.
pub fn render_quiz_tab(f: &mut Frame, app: &App, area: Rect) {
    let quiz = &app.nexus.quiz;
    let questions = QUESTIONS;

    // Layout: question area + options + status
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Question
            Constraint::Length(8),  // Options
            Constraint::Min(1),     // Explanation/Result
        ])
        .margin(1)
        .split(area);

    // Main block
    let title = if quiz.complete {
        format!(" Quiz Complete - Score: {}/{} ", quiz.score, questions.len())
    } else {
        format!(" Question {}/{} ", quiz.current_question + 1, questions.len())
    };

    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    f.render_widget(block, area);

    if quiz.complete {
        render_quiz_complete(f, app, &chunks);
    } else if let Some(question) = questions.get(quiz.current_question) {
        render_question(f, app, question, &chunks);
    }
}

/// Render the current question and options.
fn render_question(f: &mut Frame, app: &App, question: &QuizQuestion, chunks: &[Rect]) {
    let quiz = &app.nexus.quiz;

    // Question text
    let question_text = Paragraph::new(question.question)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .wrap(Wrap { trim: true });
    f.render_widget(question_text, chunks[0]);

    // Options
    let mut option_lines: Vec<Line> = Vec::new();
    for (i, opt) in question.options.iter().enumerate() {
        let is_selected = i == quiz.selected_option;
        let is_correct = i == question.correct;

        let (prefix, style) = if quiz.answered {
            // After answering, show correct/incorrect
            if is_correct {
                ("✓ ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else if is_selected {
                ("✗ ", Style::default().fg(Color::Red))
            } else {
                ("  ", Style::default().fg(Color::DarkGray))
            }
        } else {
            // Before answering, show selection cursor
            if is_selected {
                ("▶ ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            } else {
                ("  ", Style::default().fg(Color::White))
            }
        };

        let letter = ['A', 'B', 'C', 'D'][i];
        option_lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{}) ", letter), Style::default().fg(Color::Yellow)),
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
                "📚 Explanation:",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                question.explanation,
                Style::default().fg(Color::Rgb(180, 180, 200)),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "[Enter: next question]",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        // Show score so far
        let current = quiz.current_question + 1;
        lines.push(Line::from(Span::styled(
            format!("Score: {}/{} answered", quiz.score, current),
            Style::default().fg(Color::Cyan),
        )));

        let para = Paragraph::new(lines).wrap(Wrap { trim: true });
        f.render_widget(para, explanation_area);
    } else {
        let hint = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "[j/k or ↑/↓: select] [Enter: submit]",
                Style::default().fg(Color::DarkGray),
            )),
        ]);
        f.render_widget(hint, explanation_area);
    }
}

/// Render the quiz completion screen.
fn render_quiz_complete(f: &mut Frame, app: &App, chunks: &[Rect]) {
    let quiz = &app.nexus.quiz;
    let total = QUESTIONS.len();
    let pct = (quiz.score as f64 / total as f64 * 100.0) as u8;

    let (grade, grade_color) = match pct {
        90..=100 => ("🏆 Expert!", Color::Green),
        75..=89 => ("🎯 Great job!", Color::Cyan),
        60..=74 => ("📚 Good effort!", Color::Yellow),
        _ => ("💪 Keep learning!", Color::Magenta),
    };

    let result_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("Final Score: {}/{} ({}%)", quiz.score, total, pct),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            grade,
            Style::default().fg(grade_color).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "[r: restart quiz] [Tab: other tabs]",
            Style::default().fg(Color::DarkGray),
        )),
    ];

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

        state.select_down();
        assert_eq!(state.selected_option, 1);

        state.select_down();
        assert_eq!(state.selected_option, 2);

        state.select_down();
        assert_eq!(state.selected_option, 3);

        // Can't go past 3
        state.select_down();
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
            assert!(
                q.correct < 4,
                "Question {} has invalid correct index: {}",
                i,
                q.correct
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
        state.select_down();
        assert_eq!(state.selected_option, initial);

        state.select_up();
        assert_eq!(state.selected_option, initial);
    }
}
