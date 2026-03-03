//! Glossary Tab - Searchable concept dictionary for NovaNet.
//!
//! **3-Panel Layout**:
//! - Left: Category browser with visual badges
//! - Center: Concept list with category grouping
//! - Right: Definition panel with interactive cross-links
//!
//! **Navigation**:
//! - [1-5]: Jump to category
//! - [j/k]: Navigate concepts
//! - [Enter]: Jump to "See Also" concept
//! - `/`: Search
//!
//! Provides 15 core concepts organized into 5 categories:
//! - Graph Basics (4): Class, Instance, Entity, EntityNative
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

use super::NexusLocale;
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

    /// Get category icon (v0.12.0).
    pub fn icon(&self) -> &'static str {
        match self {
            GlossaryCategory::GraphBasics => "◆",    // Diamond for nodes
            GlossaryCategory::Classification => "◫", // Stacked squares for layers
            GlossaryCategory::LocaleSystem => "◉",   // Circle for locales
            GlossaryCategory::Relationships => "→",  // Arrow for arcs
            GlossaryCategory::Architecture => "⚙",   // Gear for architecture
        }
    }

    /// Get category color (v0.12.0).
    pub fn color(&self) -> Color {
        match self {
            GlossaryCategory::GraphBasics => Color::Cyan,
            GlossaryCategory::Classification => Color::Magenta,
            GlossaryCategory::LocaleSystem => Color::Green,
            GlossaryCategory::Relationships => Color::Yellow,
            GlossaryCategory::Architecture => Color::Blue,
        }
    }

    /// Get shortcut key (1-5) for this category.
    pub fn shortcut(&self) -> char {
        match self {
            GlossaryCategory::GraphBasics => '1',
            GlossaryCategory::Classification => '2',
            GlossaryCategory::LocaleSystem => '3',
            GlossaryCategory::Relationships => '4',
            GlossaryCategory::Architecture => '5',
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
        name: "Class",
        short_desc: "Schema definitions (:Schema:Class)",
        full_desc: "A node that defines the SCHEMA (type system). Class nodes have the :Schema:Class \
                    label in Neo4j. There are 58 Classes defining all types in NovaNet.",
        classification: None,
        example_yaml: None,
        example_neo4j: Some("(:Schema:Class {label: 'Locale', realm: 'shared', layer: 'config'})"),
        see_also: &["Instance", "Realm", "Layer"],
    },
    GlossaryConcept {
        name: "Instance",
        short_desc: "Actual data nodes (:Locale, :Entity)",
        full_desc: "Actual content instances. Instances are what your application uses - \
                    the real locales, entities, pages, etc. There are 200,000+ instances.",
        classification: None,
        example_yaml: None,
        example_neo4j: Some(
            "(:Locale {key: 'fr-FR', display_name: 'French (France)', script: 'latin'})",
        ),
        see_also: &["Class", "Entity"],
    },
    GlossaryConcept {
        name: "Entity",
        short_desc: "Semantic unit (qr-code, wifi-qr)",
        full_desc: "A semantic unit representing products, features, concepts, actions, or tools. \
                    Entities are DEFINED - created once by humans, then content is generated for each locale.",
        classification: Some("Realm: org | Layer: semantic | Trait: defined"),
        example_yaml: Some(
            "node:\n  name: Entity\n  realm: org\n  layer: semantic\n  trait: defined",
        ),
        example_neo4j: Some("(:Entity {key: 'qr-code', display_name: 'QR Code', is_pillar: true})"),
        see_also: &["EntityNative", "Realm", "Trait"],
    },
    GlossaryConcept {
        name: "EntityNative",
        short_desc: "Locale-native content (entity:qr-code@fr-FR)",
        full_desc: "Locale-native content for an Entity. NOT translated - GENERATED natively \
                    using locale knowledge (Terms, Culture, Style). One EntityNative per locale. \
                    (v0.13.0: renamed from EntityContent)",
        classification: Some("Realm: org | Layer: semantic | Trait: authored"),
        example_yaml: None,
        example_neo4j: Some(
            "(:EntityNative {\n  key: 'entity:qr-code@fr-FR',\n  display_name: 'QR Code',\n  description: 'Code-barres 2D...'\n})",
        ),
        see_also: &["Entity", "Locale", "Term"],
    },
];

static CLASSIFICATION_CONCEPTS: [GlossaryConcept; 3] = [
    GlossaryConcept {
        name: "Realm",
        short_desc: "WHERE: shared vs org",
        full_desc: "WHERE a node lives. Two realms exist:\n\
                    - SHARED (40 classes): Universal knowledge, READ-ONLY\n\
                    - ORG (21 classes): Organization-specific business content",
        classification: None,
        example_yaml: Some(
            "node:\n  realm: shared  # Universal, cannot be modified by org\n  # OR\n  realm: org     # Organization-specific",
        ),
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
        example_yaml: Some(
            "# Shared layers (4):\nconfig, locale, geography, knowledge\n\n# Org layers (6):\nconfig, foundation, structure, semantic, instruction, output",
        ),
        example_neo4j: None,
        see_also: &["Realm", "Trait"],
    },
    GlossaryConcept {
        name: "Trait",
        short_desc: "HOW: data origin",
        full_desc: "WHERE does the data come from? 5 traits:\n\
                    - defined: Human-created once (Entity, Page)\n\
                    - authored: Human-written per locale (EntityNative)\n\
                    - imported: External data brought in (Term, Culture)\n\
                    - generated: Produced by NovaNet LLM (PageNative)\n\
                    - retrieved: Fetched from external APIs (GEOAnswer)",
        classification: None,
        example_yaml: Some(
            "# Visual encoding:\n  defined   = solid border\n  authored  = dashed border\n  imported  = double border\n  generated = dotted border\n  retrieved = thin dotted border",
        ),
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
        classification: Some("Realm: shared | Layer: config | Trait: defined"),
        example_yaml: None,
        example_neo4j: Some(
            "(:Locale {\n  key: 'fr-FR',\n  language_code: 'fr',\n  country_code: 'FR',\n  script: 'latin',\n  text_direction: 'ltr'\n})",
        ),
        see_also: &["Term", "Culture"],
    },
    GlossaryConcept {
        name: "Term",
        short_desc: "Native vocabulary (\"abonnement mensuel\")",
        full_desc: "A vocabulary word or phrase specific to a locale. Part of the Knowledge \
                    that makes native generation possible. Terms have domain, register, and synonyms.",
        classification: Some("Realm: shared | Layer: knowledge | Trait: imported"),
        example_yaml: None,
        example_neo4j: Some(
            "(:Term {\n  key: 'subscription_monthly',\n  value: 'abonnement mensuel',\n  domain: 'pricing',\n  register: 'formal',\n  synonyms: ['formule mensuelle']\n})",
        ),
        see_also: &["Locale", "Expression", "Culture"],
    },
    GlossaryConcept {
        name: "Expression",
        short_desc: "Idioms (\"C'est du gateau\")",
        full_desc: "Idiomatic expressions and phrases native to a locale. Unlike Terms (vocabulary), \
                    Expressions capture cultural idioms that cannot be directly translated.",
        classification: Some("Realm: shared | Layer: knowledge | Trait: imported"),
        example_yaml: None,
        example_neo4j: Some(
            "(:Expression {\n  key: 'easy_task_fr',\n  value: \"C'est du gateau\",\n  meaning: 'It\\'s easy',\n  formality: 'informal'\n})",
        ),
        see_also: &["Term", "Culture"],
    },
    GlossaryConcept {
        name: "Culture",
        short_desc: "Social rules (formality, taboos)",
        full_desc: "Cultural rules and social conventions for a locale. Includes formality levels, \
                    taboos, humor style, and other cultural considerations for content generation.",
        classification: Some("Realm: shared | Layer: knowledge | Trait: imported"),
        example_yaml: None,
        example_neo4j: Some(
            "(:Culture {\n  key: 'fr_formality',\n  aspect: 'formality',\n  value: 'Use vous for business',\n  strength: 'strong'\n})",
        ),
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
        example_neo4j: Some(
            "(:Entity {key: 'qr-code'})-[:HAS_NATIVE]->(:EntityNative {key: 'entity:qr-code@fr-FR'})",
        ),
        see_also: &["Family", "Scope"],
    },
    GlossaryConcept {
        name: "Family",
        short_desc: "Arc category (ownership, semantic)",
        full_desc: "Arc families group relationships by function:\n\
                    - ownership: Parent owns child (HAS_NATIVE)\n\
                    - localization: Links to locale (FOR_LOCALE)\n\
                    - semantic: Meaning connections (SEMANTIC_LINK)\n\
                    - generation: LLM pipeline (HAS_NATIVE)\n\
                    - mining: SEO/GEO intelligence (TARGETS)",
        classification: None,
        example_yaml: Some(
            "arc:\n  name: HAS_NATIVE\n  family: ownership\n  source: Entity\n  target: EntityNative",
        ),
        example_neo4j: None,
        see_also: &["Arc", "Scope"],
    },
    GlossaryConcept {
        name: "Scope",
        short_desc: "intra_realm vs cross_realm",
        full_desc: "Arc scope indicates whether a relationship stays within one realm or crosses realms:\n\
                    - intra_realm: Both nodes in same realm (org Entity -> org EntityNative)\n\
                    - cross_realm: Nodes in different realms (org Entity -> shared EntityCategory)",
        classification: None,
        example_yaml: Some(
            "arc:\n  scope: cross_realm  # org -> shared\n  # OR\n  scope: intra_realm  # same realm",
        ),
        example_neo4j: None,
        see_also: &["Arc", "Family", "Realm"],
    },
];

static ARCHITECTURE_CONCEPTS: [GlossaryConcept; 1] = [GlossaryConcept {
    name: "Native Generation",
    short_desc: "Generate, NOT translate",
    full_desc: "NovaNet's core philosophy: content is GENERATED natively using locale \
                    knowledge, NOT translated from a source language. This preserves cultural \
                    nuance and produces content that sounds natural to native speakers. \
                    v0.13.0 unifies all locale-specific nodes with *Native suffix.",
    classification: None,
    example_yaml: Some(
        "# WRONG:\nSource -> Translate -> Target\n\n# RIGHT:\nEntity (defined)\n  + Imported (fr-FR: Terms, Culture)\n  -> EntityNative@fr-FR (authored natively)",
    ),
    example_neo4j: None,
    see_also: &["Entity", "EntityNative", "Term"],
}];

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
    /// Selected "See Also" index for cross-navigation (v0.12.0).
    pub see_also_cursor: usize,
    /// Whether focus is on the "See Also" section (v0.12.0).
    pub see_also_focused: bool,
}

impl GlossaryState {
    /// Create new GlossaryState.
    pub fn new() -> Self {
        Self {
            expanded_category: Some(0), // Start with first category expanded
            concept_cursor: 0,
            search_query: String::new(),
            search_active: false,
            see_also_cursor: 0,
            see_also_focused: false,
        }
    }

    /// Get all concepts (flattened) - returns reference to static data.
    pub fn all_concepts() -> &'static [(GlossaryCategory, &'static GlossaryConcept)] {
        &ALL_CONCEPTS
    }

    /// Get filtered concepts based on search query.
    /// Returns Cow to avoid allocation when no filter is active.
    pub fn filtered_concepts(
        &self,
    ) -> Cow<'static, [(GlossaryCategory, &'static GlossaryConcept)]> {
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

    // =========================================================================
    // CROSS-NAVIGATION (v0.12.0)
    // =========================================================================

    /// Jump to a specific category by index (0-4).
    pub fn jump_to_category(&mut self, index: usize) {
        if index < GlossaryCategory::all().len() {
            self.expanded_category = Some(index);
            // Calculate starting cursor for this category
            let mut cursor = 0;
            for (i, category) in GlossaryCategory::all().iter().enumerate() {
                if i == index {
                    break;
                }
                cursor += category.concepts().len();
            }
            self.concept_cursor = cursor;
            self.see_also_focused = false;
            self.see_also_cursor = 0;
        }
    }

    /// Toggle focus between concept list and "See Also" section.
    pub fn toggle_see_also_focus(&mut self) {
        if let Some((_, concept)) = self.current_concept() {
            if !concept.see_also.is_empty() {
                self.see_also_focused = !self.see_also_focused;
                if self.see_also_focused {
                    self.see_also_cursor = 0;
                }
            }
        }
    }

    /// Navigate up in "See Also" section.
    pub fn see_also_up(&mut self) {
        if self.see_also_cursor > 0 {
            self.see_also_cursor -= 1;
        }
    }

    /// Navigate down in "See Also" section.
    pub fn see_also_down(&mut self) {
        if let Some((_, concept)) = self.current_concept() {
            let max = concept.see_also.len().saturating_sub(1);
            if self.see_also_cursor < max {
                self.see_also_cursor += 1;
            }
        }
    }

    /// Jump to the currently selected "See Also" concept.
    /// Returns true if successfully jumped.
    pub fn jump_to_see_also(&mut self) -> bool {
        if !self.see_also_focused {
            return false;
        }

        if let Some((_, concept)) = self.current_concept() {
            if let Some(target_name) = concept.see_also.get(self.see_also_cursor) {
                // Find the concept in all_concepts
                for (i, (_, c)) in Self::all_concepts().iter().enumerate() {
                    if c.name == *target_name {
                        self.concept_cursor = i;
                        self.see_also_focused = false;
                        self.see_also_cursor = 0;
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Get the current "See Also" selection (if focused).
    pub fn current_see_also(&self) -> Option<&'static str> {
        if !self.see_also_focused {
            return None;
        }
        self.current_concept()
            .and_then(|(_, c)| c.see_also.get(self.see_also_cursor).copied())
    }
}

// =============================================================================
// RENDERING
// =============================================================================

/// Render the Glossary tab.
pub fn render_glossary_tab(f: &mut Frame, app: &App, area: Rect) {
    let locale = app.nexus.locale;

    // v0.12.0: 3-panel layout
    // Left (15%): Category browser with visual badges
    // Center (30%): Concept list
    // Right (55%): Definition panel with cross-links
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(30),
            Constraint::Percentage(55),
        ])
        .split(area);

    render_category_browser(f, app, locale, chunks[0]);
    render_concept_list(f, app, locale, chunks[1]);
    render_definition_panel(f, app, locale, chunks[2]);
}

/// Render the category browser (left panel, v0.12.0).
fn render_category_browser(f: &mut Frame, app: &App, locale: NexusLocale, area: Rect) {
    let glossary = &app.nexus.glossary;

    // i18n labels
    let categories_label = match locale {
        NexusLocale::En => " CATEGORIES ",
        NexusLocale::Fr => " CATÉGORIES ",
    };

    let block = Block::default()
        .title(Span::styled(
            categories_label,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Render categories with icons and badges
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from("")); // Spacing

    for (i, category) in GlossaryCategory::all().iter().enumerate() {
        let is_current = glossary.expanded_category == Some(i);
        let concept_count = category.concepts().len();

        // Progress indicator: how many concepts in this category have been viewed
        // (For now, show as selected/not selected)
        let marker = if is_current { "▶" } else { " " };
        let badge_style = if is_current {
            Style::default()
                .fg(category.color())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        lines.push(Line::from(vec![
            Span::styled(format!(" {} ", marker), badge_style),
            Span::styled(
                format!("[{}]", category.shortcut()),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                format!(" {} ", category.icon()),
                Style::default().fg(category.color()),
            ),
        ]));

        // Category name (truncated if needed)
        let name = if area.width > 18 {
            category.label().to_string()
        } else {
            category.label().chars().take(8).collect::<String>() + ".."
        };

        lines.push(Line::from(vec![
            Span::styled("   ", Style::default()),
            Span::styled(
                name,
                if is_current {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                },
            ),
        ]));

        // Concept count
        lines.push(Line::from(vec![
            Span::styled("   ", Style::default()),
            Span::styled(
                format!("({} concepts)", concept_count),
                Style::default().fg(Color::DarkGray),
            ),
        ]));

        lines.push(Line::from("")); // Spacing between categories
    }

    // Navigation hint at bottom
    let hint = match locale {
        NexusLocale::En => "[1-5]",
        NexusLocale::Fr => "[1-5]",
    };
    lines.push(Line::from(Span::styled(
        format!("  {} jump", hint),
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

/// Render the concept list (left panel).
fn render_concept_list(f: &mut Frame, app: &App, locale: NexusLocale, area: Rect) {
    let glossary = &app.nexus.glossary;
    let concepts = glossary.filtered_concepts();

    // i18n labels
    let (search_label, filter_label, concepts_label) = match locale {
        NexusLocale::En => ("Search", "Filter", "CONCEPTS"),
        NexusLocale::Fr => ("Recherche", "Filtre", "CONCEPTS"),
    };

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
        format!(" [/] {search_label}: {}_ ", glossary.search_query)
    } else if !glossary.search_query.is_empty() {
        format!(" [/] {filter_label}: {} ", glossary.search_query)
    } else {
        format!(" {concepts_label} ({}) ", concepts.len())
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
fn render_definition_panel(f: &mut Frame, app: &App, locale: NexusLocale, area: Rect) {
    let glossary = &app.nexus.glossary;
    let theme = &app.theme;

    // i18n labels
    let (definition_label, no_concept_msg) = match locale {
        NexusLocale::En => (" DEFINITION ", "No concept selected"),
        NexusLocale::Fr => (" DÉFINITION ", "Aucun concept sélectionné"),
    };

    let block = Block::default()
        .title(Span::styled(
            definition_label,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some((category, concept)) = glossary.current_concept() {
        render_concept_definition(f, theme, locale, inner, category, concept, glossary);
    } else {
        let empty = Paragraph::new(no_concept_msg).style(Style::default().fg(Color::DarkGray));
        f.render_widget(empty, inner);
    }
}

/// Render a single concept definition.
fn render_concept_definition(
    f: &mut Frame,
    _theme: &Theme,
    locale: NexusLocale,
    area: Rect,
    category: GlossaryCategory,
    concept: &GlossaryConcept,
    glossary: &GlossaryState, // v0.12.0: Added for cross-link state
) {
    // i18n labels
    let (classification_label, yaml_label, neo4j_label, see_also_label, nav_hint) = match locale {
        NexusLocale::En => (
            "  CLASSIFICATION",
            "  YAML EXAMPLE",
            "  NEO4J EXAMPLE",
            "  SEE ALSO ↵",
            "  [j/k] nav  [/] search  [Tab] see-also  [Enter] jump  [y] copy",
        ),
        NexusLocale::Fr => (
            "  CLASSIFICATION",
            "  EXEMPLE YAML",
            "  EXEMPLE NEO4J",
            "  VOIR AUSSI ↵",
            "  [j/k] nav  [/] rechercher  [Tab] voir-aussi  [Enter] sauter  [y] copier",
        ),
    };

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
            classification_label,
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
            yaml_label,
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
            neo4j_label,
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

    // See also - v0.12.0: Interactive cross-links
    if !concept.see_also.is_empty() {
        let see_also_style = if glossary.see_also_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        };

        lines.push(Line::from(Span::styled(see_also_label, see_also_style)));
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────────────────────────",
            Style::default().fg(Color::DarkGray),
        )));

        // Render each cross-link as a clickable item
        for (i, related) in concept.see_also.iter().enumerate() {
            let is_selected = glossary.see_also_focused && glossary.see_also_cursor == i;
            let marker = if is_selected { "▶" } else { "○" };
            let link_style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(Color::Cyan)
            };

            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {} ", marker),
                    if is_selected {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
                Span::styled(*related, link_style),
                if is_selected {
                    Span::styled(" ← Enter to jump", Style::default().fg(Color::DarkGray))
                } else {
                    Span::raw("")
                },
            ]));
        }
    }

    // Navigation hint
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        nav_hint,
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
        // Should find Entity and EntityNative
        assert!(filtered.len() >= 2);
        assert!(
            filtered
                .iter()
                .all(|(_, c)| c.name.to_lowercase().contains("entity")
                    || c.short_desc.to_lowercase().contains("entity")
                    || c.full_desc.to_lowercase().contains("entity"))
        );
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
        assert_eq!(yank.unwrap(), "Class");
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
