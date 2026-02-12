//! Glossary Tab - Searchable concept dictionary for NovaNet.
//!
//! Provides 15 core concepts organized into 5 categories:
//! - Graph Basics (4): Meta Node, Data Node, Entity, EntityContent
//! - Classification (3): Realm, Layer, Trait
//! - Locale System (4): Locale, Term, Expression, Culture
//! - Relationships (3): Arc, Family, Scope
//! - Architecture (1): Native Generation

use std::borrow::Cow;
use std::sync::LazyLock;

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use crate::tui::app::App;
use crate::tui::theme::Theme;

/// Pre-computed flattened concept list (computed once at first access).
static ALL_CONCEPTS: LazyLock<Vec<(GlossaryCategory, &'static GlossaryConcept)>> =
    LazyLock::new(|| {
        let mut result = Vec::with_capacity(15); // Known size
        for category in GlossaryCategory::all() {
            for concept in category.concepts() {
                result.push((*category, concept));
            }
        }
        result
    });

// =============================================================================
// GLOSSARY DATA STRUCTURES
// =============================================================================

/// A concept category in the glossary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlossaryCategory {
    GraphBasics,
    Classification,
    LocaleSystem,
    Relationships,
    Architecture,
}

/// Categories for breadcrumb display.
/// (label, concept_count)
pub const CATEGORIES: &[(&str, usize)] = &[
    ("Graph Basics", 4),
    ("Classification", 3),
    ("Locale System", 4),
    ("Relationships", 3),
    ("Architecture", 1),
];

impl GlossaryCategory {
    /// Get all categories.
    pub fn all() -> &'static [GlossaryCategory] {
        &[
            GlossaryCategory::GraphBasics,
            GlossaryCategory::Classification,
            GlossaryCategory::LocaleSystem,
            GlossaryCategory::Relationships,
            GlossaryCategory::Architecture,
        ]
    }

    /// Get category label.
    pub fn label(&self) -> &'static str {
        match self {
            GlossaryCategory::GraphBasics => "Graph Basics",
            GlossaryCategory::Classification => "Classification",
            GlossaryCategory::LocaleSystem => "Locale System",
            GlossaryCategory::Relationships => "Relationships",
            GlossaryCategory::Architecture => "Architecture",
        }
    }

    /// Get concepts in this category.
    pub fn concepts(&self) -> &'static [GlossaryConcept] {
        match self {
            GlossaryCategory::GraphBasics => &GRAPH_BASICS_CONCEPTS,
            GlossaryCategory::Classification => &CLASSIFICATION_CONCEPTS,
            GlossaryCategory::LocaleSystem => &LOCALE_SYSTEM_CONCEPTS,
            GlossaryCategory::Relationships => &RELATIONSHIPS_CONCEPTS,
            GlossaryCategory::Architecture => &ARCHITECTURE_CONCEPTS,
        }
    }
}

/// A concept definition in the glossary.
#[derive(Debug, Clone)]
pub struct GlossaryConcept {
    pub name: &'static str,
    pub short_desc: &'static str,
    pub full_desc: &'static str,
    pub classification: Option<&'static str>,
    pub example_yaml: Option<&'static str>,
    pub example_neo4j: Option<&'static str>,
    pub see_also: &'static [&'static str],
}

// =============================================================================
// CONCEPT DEFINITIONS
// =============================================================================

static GRAPH_BASICS_CONCEPTS: [GlossaryConcept; 4] = [
    GlossaryConcept {
        name: "Meta Node",
        short_desc: "Schema definitions (:Meta:Kind)",
        full_desc: "A node that defines the SCHEMA (type system). Meta nodes have the :Meta label \
                    in Neo4j. There are 60 Meta nodes defining all types (Kinds) in NovaNet.",
        classification: None,
        example_yaml: None,
        example_neo4j: Some("(:Meta:Kind {label: 'Locale', realm: 'shared', layer: 'config'})"),
        see_also: &["Data Node", "Kind"],
    },
    GlossaryConcept {
        name: "Data Node",
        short_desc: "Actual instances (:Locale, :Entity)",
        full_desc: "Actual content instances. Data nodes are what your application uses - \
                    the real locales, entities, pages, etc. There are 200,000+ data nodes.",
        classification: None,
        example_yaml: None,
        example_neo4j: Some("(:Locale {key: 'fr-FR', display_name: 'French (France)', script: 'latin'})"),
        see_also: &["Meta Node", "Instance"],
    },
    GlossaryConcept {
        name: "Entity",
        short_desc: "Semantic unit (qr-code, wifi-qr)",
        full_desc: "A semantic unit representing products, features, concepts, actions, or tools. \
                    Entities are INVARIANT - defined once, then localized content is generated for each locale.",
        classification: Some("Realm: org | Layer: semantic | Trait: invariant"),
        example_yaml: Some("node:\n  name: Entity\n  realm: org\n  layer: semantic\n  trait: invariant"),
        example_neo4j: Some("(:Entity {key: 'qr-code', display_name: 'QR Code', is_pillar: true})"),
        see_also: &["EntityContent", "Realm", "Trait"],
    },
    GlossaryConcept {
        name: "EntityContent",
        short_desc: "Localized content (entity:qr-code@fr-FR)",
        full_desc: "Locale-native content for an Entity. NOT translated - GENERATED natively \
                    using locale knowledge (Terms, Culture, Style). One EntityContent per locale.",
        classification: Some("Realm: org | Layer: semantic | Trait: localized"),
        example_yaml: None,
        example_neo4j: Some("(:EntityContent {\n  key: 'entity:qr-code@fr-FR',\n  display_name: 'QR Code',\n  description: 'Code-barres 2D...'\n})"),
        see_also: &["Entity", "Locale", "Term"],
    },
];

static CLASSIFICATION_CONCEPTS: [GlossaryConcept; 3] = [
    GlossaryConcept {
        name: "Realm",
        short_desc: "WHERE: shared vs org",
        full_desc: "WHERE a node lives. Two realms exist:\n\
                    - SHARED (39 nodes): Universal knowledge, READ-ONLY\n\
                    - ORG (21 nodes): Organization-specific business content",
        classification: None,
        example_yaml: Some("node:\n  realm: shared  # Universal, cannot be modified by org\n  # OR\n  realm: org     # Organization-specific"),
        example_neo4j: None,
        see_also: &["Layer", "Trait"],
    },
    GlossaryConcept {
        name: "Layer",
        short_desc: "WHAT: functional category",
        full_desc: "WHAT is the node's function. 10 layers total:\n\
                    Shared: config, locale, geography, knowledge\n\
                    Org: config, foundation, structure, semantic, instruction, output",
        classification: None,
        example_yaml: Some("# Shared layers (4):\nconfig, locale, geography, knowledge\n\n# Org layers (6):\nconfig, foundation, structure, semantic, instruction, output"),
        example_neo4j: None,
        see_also: &["Realm", "Trait"],
    },
    GlossaryConcept {
        name: "Trait",
        short_desc: "HOW: locale behavior",
        full_desc: "HOW the node behaves with locales. 5 traits:\n\
                    - invariant: Same everywhere (Entity, Page)\n\
                    - localized: Generated per locale (EntityContent)\n\
                    - knowledge: Locale expertise (Term, Culture)\n\
                    - generated: LLM output (PageGenerated)\n\
                    - aggregated: Computed metrics (SEOKeywordMetrics)",
        classification: None,
        example_yaml: Some("# Visual encoding:\n  invariant  = solid border\n  localized  = dashed border\n  knowledge  = double border\n  generated  = dotted border\n  aggregated = thin dotted border"),
        example_neo4j: None,
        see_also: &["Realm", "Layer"],
    },
];

static LOCALE_SYSTEM_CONCEPTS: [GlossaryConcept; 4] = [
    GlossaryConcept {
        name: "Locale",
        short_desc: "BCP-47 identifier (fr-FR, ja-JP)",
        full_desc: "A BCP-47 locale identifier with properties like language_code, country_code, \
                    script, and text_direction. NovaNet supports 200 locales.",
        classification: Some("Realm: shared | Layer: config | Trait: invariant"),
        example_yaml: None,
        example_neo4j: Some("(:Locale {\n  key: 'fr-FR',\n  language_code: 'fr',\n  country_code: 'FR',\n  script: 'latin',\n  text_direction: 'ltr'\n})"),
        see_also: &["Term", "Culture"],
    },
    GlossaryConcept {
        name: "Term",
        short_desc: "Native vocabulary (\"abonnement mensuel\")",
        full_desc: "A vocabulary word or phrase specific to a locale. Part of the Knowledge \
                    that makes native generation possible. Terms have domain, register, and synonyms.",
        classification: Some("Realm: shared | Layer: knowledge | Trait: knowledge"),
        example_yaml: None,
        example_neo4j: Some("(:Term {\n  key: 'subscription_monthly',\n  value: 'abonnement mensuel',\n  domain: 'pricing',\n  register: 'formal',\n  synonyms: ['formule mensuelle']\n})"),
        see_also: &["Locale", "Expression", "Culture"],
    },
    GlossaryConcept {
        name: "Expression",
        short_desc: "Idioms (\"C'est du gateau\")",
        full_desc: "Idiomatic expressions and phrases native to a locale. Unlike Terms (vocabulary), \
                    Expressions capture cultural idioms that cannot be directly translated.",
        classification: Some("Realm: shared | Layer: knowledge | Trait: knowledge"),
        example_yaml: None,
        example_neo4j: Some("(:Expression {\n  key: 'easy_task_fr',\n  value: \"C'est du gateau\",\n  meaning: 'It\\'s easy',\n  formality: 'informal'\n})"),
        see_also: &["Term", "Culture"],
    },
    GlossaryConcept {
        name: "Culture",
        short_desc: "Social rules (formality, taboos)",
        full_desc: "Cultural rules and social conventions for a locale. Includes formality levels, \
                    taboos, humor style, and other cultural considerations for content generation.",
        classification: Some("Realm: shared | Layer: knowledge | Trait: knowledge"),
        example_yaml: None,
        example_neo4j: Some("(:Culture {\n  key: 'fr_formality',\n  aspect: 'formality',\n  value: 'Use vous for business',\n  strength: 'strong'\n})"),
        see_also: &["Term", "Locale"],
    },
];

static RELATIONSHIPS_CONCEPTS: [GlossaryConcept; 3] = [
    GlossaryConcept {
        name: "Arc",
        short_desc: "Connection between nodes",
        full_desc: "A directed relationship between two nodes. NovaNet uses 'Arc' instead of \
                    'Edge' or 'Relation' to align with graph theory terminology for directed graphs.",
        classification: None,
        example_yaml: None,
        example_neo4j: Some("(:Entity {key: 'qr-code'})-[:HAS_CONTENT]->(:EntityContent {key: 'entity:qr-code@fr-FR'})"),
        see_also: &["Family", "Scope"],
    },
    GlossaryConcept {
        name: "Family",
        short_desc: "Arc category (ownership, semantic)",
        full_desc: "Arc families group relationships by function:\n\
                    - ownership: Parent owns child (HAS_CONTENT)\n\
                    - localization: Links to locale (FOR_LOCALE)\n\
                    - semantic: Meaning connections (SEMANTIC_LINK)\n\
                    - generation: LLM pipeline (HAS_GENERATED)\n\
                    - mining: SEO/GEO intelligence (HAS_KEYWORD)",
        classification: None,
        example_yaml: Some("arc:\n  name: HAS_CONTENT\n  family: ownership\n  source: Entity\n  target: EntityContent"),
        example_neo4j: None,
        see_also: &["Arc", "Scope"],
    },
    GlossaryConcept {
        name: "Scope",
        short_desc: "intra_realm vs cross_realm",
        full_desc: "Arc scope indicates whether a relationship stays within one realm or crosses realms:\n\
                    - intra_realm: Both nodes in same realm (org Entity -> org EntityContent)\n\
                    - cross_realm: Nodes in different realms (org Entity -> shared EntityCategory)",
        classification: None,
        example_yaml: Some("arc:\n  scope: cross_realm  # org -> shared\n  # OR\n  scope: intra_realm  # same realm"),
        example_neo4j: None,
        see_also: &["Arc", "Family", "Realm"],
    },
];

static ARCHITECTURE_CONCEPTS: [GlossaryConcept; 1] = [
    GlossaryConcept {
        name: "Native Generation",
        short_desc: "Generate, NOT translate",
        full_desc: "NovaNet's core philosophy: content is GENERATED natively using locale \
                    knowledge, NOT translated from a source language. This preserves cultural \
                    nuance and produces content that sounds natural to native speakers.",
        classification: None,
        example_yaml: Some("# WRONG:\nSource -> Translate -> Target\n\n# RIGHT:\nEntity (invariant)\n  + Knowledge (fr-FR: Terms, Culture)\n  -> EntityContent@fr-FR (native)"),
        example_neo4j: None,
        see_also: &["Entity", "EntityContent", "Term"],
    },
];

// =============================================================================
// GLOSSARY STATE
// =============================================================================

/// State for the Glossary tab.
#[derive(Debug, Clone, Default)]
pub struct GlossaryState {
    /// Currently expanded category index (None = all collapsed).
    pub expanded_category: Option<usize>,
    /// Cursor position in concept list.
    pub concept_cursor: usize,
    /// Search query (empty = no filter).
    pub search_query: String,
    /// Whether search mode is active.
    pub search_active: bool,
}

impl GlossaryState {
    /// Create new GlossaryState.
    pub fn new() -> Self {
        Self {
            expanded_category: Some(0), // Start with first category expanded
            concept_cursor: 0,
            search_query: String::new(),
            search_active: false,
        }
    }

    /// Get all concepts (flattened) - returns reference to static data.
    pub fn all_concepts() -> &'static [(GlossaryCategory, &'static GlossaryConcept)] {
        &ALL_CONCEPTS
    }

    /// Get filtered concepts based on search query.
    /// Returns Cow to avoid allocation when no filter is active.
    pub fn filtered_concepts(&self) -> Cow<'static, [(GlossaryCategory, &'static GlossaryConcept)]> {
        if self.search_query.is_empty() {
            return Cow::Borrowed(Self::all_concepts());
        }

        let query = self.search_query.to_lowercase();
        Cow::Owned(
            Self::all_concepts()
                .iter()
                .filter(|(_, concept)| {
                    concept.name.to_lowercase().contains(&query)
                        || concept.short_desc.to_lowercase().contains(&query)
                        || concept.full_desc.to_lowercase().contains(&query)
                })
                .copied()
                .collect(),
        )
    }

    /// Get currently selected concept.
    pub fn current_concept(&self) -> Option<(GlossaryCategory, &'static GlossaryConcept)> {
        let concepts = self.filtered_concepts();
        concepts.get(self.concept_cursor).copied()
    }

    /// Navigate up.
    pub fn navigate_up(&mut self) {
        if self.concept_cursor > 0 {
            self.concept_cursor -= 1;
        }
    }

    /// Navigate down.
    pub fn navigate_down(&mut self) {
        let max = self.filtered_concepts().len().saturating_sub(1);
        if self.concept_cursor < max {
            self.concept_cursor += 1;
        }
    }

    /// Toggle category expansion.
    pub fn toggle_category(&mut self, index: usize) {
        if self.expanded_category == Some(index) {
            self.expanded_category = None;
        } else {
            self.expanded_category = Some(index);
        }
    }

    /// Start search mode.
    pub fn start_search(&mut self) {
        self.search_active = true;
    }

    /// End search mode.
    pub fn end_search(&mut self) {
        self.search_active = false;
    }

    /// Clear search.
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.concept_cursor = 0;
    }

    /// Add character to search.
    pub fn search_push(&mut self, c: char) {
        self.search_query.push(c);
        self.concept_cursor = 0; // Reset cursor on search change
    }

    /// Remove character from search.
    pub fn search_pop(&mut self) {
        self.search_query.pop();
        self.concept_cursor = 0;
    }

    /// Get text to yank.
    pub fn get_yank_text(&self) -> Option<String> {
        self.current_concept().map(|(_, c)| c.name.to_string())
    }

    /// Go to previous category.
    pub fn prev_category(&mut self) {
        if let Some(idx) = self.expanded_category {
            if idx > 0 {
                self.expanded_category = Some(idx - 1);
                self.concept_cursor = 0;
            }
        } else {
            // No category expanded, expand the last one
            self.expanded_category = Some(GlossaryCategory::all().len() - 1);
            self.concept_cursor = 0;
        }
    }

    /// Go to next category.
    pub fn next_category(&mut self) {
        if let Some(idx) = self.expanded_category {
            let max = GlossaryCategory::all().len() - 1;
            if idx < max {
                self.expanded_category = Some(idx + 1);
                self.concept_cursor = 0;
            }
        } else {
            // No category expanded, expand the first one
            self.expanded_category = Some(0);
            self.concept_cursor = 0;
        }
    }

    /// Toggle expand/collapse current category.
    pub fn toggle_expand(&mut self) {
        if self.expanded_category.is_some() {
            // Already expanded, collapse
            self.expanded_category = None;
        } else {
            // Collapsed, expand first category
            self.expanded_category = Some(0);
        }
    }
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Glossary tab.
pub fn render_glossary_tab(f: &mut Frame, app: &App, area: Rect) {
    // Split into concept list and definition panel
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);

    render_concept_list(f, app, chunks[0]);
    render_definition_panel(f, app, chunks[1]);
}

/// Render the concept list (left panel).
fn render_concept_list(f: &mut Frame, app: &App, area: Rect) {
    let glossary = &app.nexus.glossary;
    let concepts = glossary.filtered_concepts();

    // Build list items
    let mut items: Vec<ListItem> = Vec::new();
    let mut current_category: Option<GlossaryCategory> = None;

    for (i, (category, concept)) in concepts.iter().enumerate() {
        // Add category header if changed
        if current_category != Some(*category) {
            if current_category.is_some() {
                items.push(ListItem::new(Line::from(""))); // Spacing
            }
            items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    format!(" {} ", category.label()),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("({})", category.concepts().len()),
                    Style::default().fg(Color::DarkGray),
                ),
            ])));
            current_category = Some(*category);
        }

        // Add concept
        let is_selected = i == glossary.concept_cursor;
        let style = if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let marker = if is_selected { "●" } else { "○" };
        items.push(ListItem::new(Line::from(vec![
            Span::styled(format!("   {} ", marker), style),
            Span::styled(concept.name, style),
        ])));
    }

    // Search indicator
    let title = if glossary.search_active {
        format!(" [/] Search: {}_ ", glossary.search_query)
    } else if !glossary.search_query.is_empty() {
        format!(" [/] Filter: {} ", glossary.search_query)
    } else {
        format!(" CONCEPTS ({}) ", concepts.len())
    };

    let title_style = if glossary.search_active {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    };

    let block = Block::default()
        .title(Span::styled(title, title_style))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let list = List::new(items).block(block);
    f.render_widget(list, area);
}

/// Render the definition panel (right panel).
fn render_definition_panel(f: &mut Frame, app: &App, area: Rect) {
    let glossary = &app.nexus.glossary;
    let theme = &app.theme;

    let block = Block::default()
        .title(Span::styled(
            " DEFINITION ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some((category, concept)) = glossary.current_concept() {
        render_concept_definition(f, theme, inner, category, concept);
    } else {
        let empty = Paragraph::new("No concept selected").style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, inner);
    }
}

/// Render a single concept definition.
fn render_concept_definition(
    f: &mut Frame,
    _theme: &Theme,
    area: Rect,
    category: GlossaryCategory,
    concept: &GlossaryConcept,
) {
    let mut lines = Vec::new();

    // Title
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            format!("  {} ", concept.name.to_uppercase()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("({})", category.label()),
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    lines.push(Line::from(Span::styled(
        "  ════════════════════════════════════════════════════════════",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));

    // Short description
    lines.push(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(concept.short_desc, Style::default().fg(Color::Yellow)),
    ]));
    lines.push(Line::from(""));

    // Full description
    for line in concept.full_desc.lines() {
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(line, Style::default().fg(Color::White)),
        ]));
    }
    lines.push(Line::from(""));

    // Classification (if any)
    if let Some(classification) = concept.classification {
        lines.push(Line::from(Span::styled(
            "  CLASSIFICATION",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(classification, Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(""));
    }

    // YAML example (if any)
    if let Some(yaml) = concept.example_yaml {
        lines.push(Line::from(Span::styled(
            "  YAML EXAMPLE",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )));
        for line in yaml.lines() {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(line, Style::default().fg(Color::Green)),
            ]));
        }
        lines.push(Line::from(""));
    }

    // Neo4j example (if any)
    if let Some(neo4j) = concept.example_neo4j {
        lines.push(Line::from(Span::styled(
            "  NEO4J EXAMPLE",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )));
        for line in neo4j.lines() {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(line, Style::default().fg(Color::Cyan)),
            ]));
        }
        lines.push(Line::from(""));
    }

    // See also
    if !concept.see_also.is_empty() {
        lines.push(Line::from(Span::styled(
            "  SEE ALSO",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(
                concept.see_also.join(", "),
                Style::default().fg(Color::Cyan),
            ),
        ]));
    }

    // Navigation hint
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  [j/k: navigate]  [/: search]  [y: copy]  [Esc: clear]",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glossary_categories_count() {
        assert_eq!(GlossaryCategory::all().len(), 5);
    }

    #[test]
    fn test_total_concepts_count() {
        let total: usize = GlossaryCategory::all()
            .iter()
            .map(|c| c.concepts().len())
            .sum();
        assert_eq!(total, 15);
    }

    #[test]
    fn test_glossary_state_new() {
        let state = GlossaryState::new();
        assert_eq!(state.expanded_category, Some(0));
        assert_eq!(state.concept_cursor, 0);
        assert!(state.search_query.is_empty());
        assert!(!state.search_active);
    }

    #[test]
    fn test_all_concepts_flattened() {
        let concepts = GlossaryState::all_concepts();
        assert_eq!(concepts.len(), 15);
    }

    #[test]
    fn test_navigate_up_down() {
        let mut state = GlossaryState::new();
        state.concept_cursor = 5;

        state.navigate_up();
        assert_eq!(state.concept_cursor, 4);

        state.navigate_down();
        assert_eq!(state.concept_cursor, 5);
    }

    #[test]
    fn test_navigate_up_at_zero() {
        let mut state = GlossaryState::new();
        state.concept_cursor = 0;

        state.navigate_up();
        assert_eq!(state.concept_cursor, 0); // Should stay at 0
    }

    #[test]
    fn test_search_filter() {
        let mut state = GlossaryState::new();
        state.search_query = "entity".to_string();

        let filtered = state.filtered_concepts();
        // Should find Entity and EntityContent
        assert!(filtered.len() >= 2);
        assert!(filtered.iter().all(|(_, c)| c.name.to_lowercase().contains("entity")
            || c.short_desc.to_lowercase().contains("entity")
            || c.full_desc.to_lowercase().contains("entity")));
    }

    #[test]
    fn test_search_push_pop() {
        let mut state = GlossaryState::new();

        state.search_push('a');
        state.search_push('b');
        assert_eq!(state.search_query, "ab");

        state.search_pop();
        assert_eq!(state.search_query, "a");
    }

    #[test]
    fn test_clear_search() {
        let mut state = GlossaryState::new();
        state.search_query = "test".to_string();
        state.concept_cursor = 10;

        state.clear_search();
        assert!(state.search_query.is_empty());
        assert_eq!(state.concept_cursor, 0);
    }

    #[test]
    fn test_get_yank_text() {
        let state = GlossaryState::new();
        let yank = state.get_yank_text();
        assert!(yank.is_some());
        assert_eq!(yank.unwrap(), "Meta Node");
    }

    #[test]
    fn test_category_labels() {
        assert_eq!(GlossaryCategory::GraphBasics.label(), "Graph Basics");
        assert_eq!(GlossaryCategory::Classification.label(), "Classification");
        assert_eq!(GlossaryCategory::LocaleSystem.label(), "Locale System");
        assert_eq!(GlossaryCategory::Relationships.label(), "Relationships");
        assert_eq!(GlossaryCategory::Architecture.label(), "Architecture");
    }

    #[test]
    fn test_concepts_have_see_also() {
        for category in GlossaryCategory::all() {
            for concept in category.concepts() {
                // Most concepts should have see_also references
                // (Architecture only has 1 concept, so it may have fewer)
                if *category != GlossaryCategory::Architecture {
                    assert!(
                        !concept.see_also.is_empty(),
                        "Concept {} should have see_also",
                        concept.name
                    );
                }
            }
        }
    }
}
