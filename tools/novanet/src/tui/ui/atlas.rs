//! Atlas mode UI rendering.
//!
//! This module contains all rendering functions for Atlas mode views:
//! - Realm Map: Bird's-eye view of 2-realm architecture
//! - Knowledge Atoms: Selective loading vs monolithic blobs
//! - Generation Pipeline: Block generation flow (not translation)
//! - Page Composition: Complete anatomy of a Page
//! - Spreading Activation: Cognitive science math behind context assembly
//! - View Traversal: Debug the 12 view definitions

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use super::{STYLE_ACCENT, STYLE_DIM, STYLE_INFO, truncate_str};
use crate::tui::app::App;
use crate::tui::atlas::{ActivationTask, AtlasView};
use crate::tui::data::AtlasRealmStats;

/// Atlas mode: interactive architecture visualizations.
pub fn render_atlas(f: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    // View selector tabs
    let view_tabs: Vec<Line> = AtlasView::all()
        .iter()
        .map(|v| {
            let is_selected = v == &app.atlas.current_view;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                STYLE_DIM
            };
            Line::from(vec![
                Span::styled(format!("[{}] ", v.shortcut()), style),
                Span::styled(v.label(), style),
            ])
        })
        .collect();

    let tabs_text = view_tabs
        .into_iter()
        .map(|l| l.to_string())
        .collect::<Vec<_>>()
        .join("  ");

    let tabs_block = Block::default()
        .title(Span::styled(
            " Atlas Mode ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(STYLE_ACCENT);

    let tabs_paragraph = Paragraph::new(tabs_text).block(tabs_block);
    f.render_widget(tabs_paragraph, chunks[0]);

    // Main content area
    let content_block = Block::default()
        .title(Span::styled(
            format!(" {} ", app.atlas.current_view.label()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(STYLE_INFO);

    let content = match app.atlas.current_view {
        AtlasView::RealmMap => render_atlas_realm_map(app),
        AtlasView::SpreadingActivation => render_atlas_spreading_activation(app),
        AtlasView::KnowledgeAtoms => render_atlas_knowledge_atoms(app),
        AtlasView::GenerationPipeline => render_atlas_generation_pipeline(app),
        AtlasView::ViewTraversal => render_atlas_view_traversal(app),
        AtlasView::PageComposition => render_atlas_page_composition(app),
    };

    let content_paragraph = Paragraph::new(content)
        .block(content_block)
        .wrap(Wrap { trim: true });
    f.render_widget(content_paragraph, chunks[1]);
}

/// Render the Realm Map view in Atlas mode.
fn render_atlas_realm_map(app: &App) -> String {
    let mut lines = Vec::new();

    // Mode indicator
    let mode_label = if app.atlas.demo_mode {
        "[d] DEMO MODE — Press [d] for live data"
    } else {
        "[d] LIVE MODE — Press [d] for demo"
    };
    lines.push(format!("  {}  |  j/k: navigate  Enter: zoom", mode_label));
    lines.push(String::new());

    // Use live data if available, otherwise demo
    if let Some(ref stats) = app.atlas.realm_stats {
        render_realm_map_live(
            &mut lines,
            stats,
            app.atlas.realm_cursor,
            app.atlas.realm_zoomed,
        );
    } else if app.atlas.demo_mode {
        render_realm_map_demo(&mut lines, app.atlas.realm_cursor);
    } else {
        lines.push("Loading realm statistics from Neo4j...".to_string());
    }

    lines.join("\n")
}

/// Render Realm Map with live Neo4j data.
fn render_realm_map_live(
    lines: &mut Vec<String>,
    stats: &AtlasRealmStats,
    cursor: usize,
    zoomed: bool,
) {
    let total_items = stats
        .realms
        .iter()
        .map(|r| 1 + r.layers.len())
        .sum::<usize>();
    lines.push(
        "╔═══════════════════════════════════════════════════════════════════════════╗".to_string(),
    );
    lines.push(format!(
        "║  2-REALM ARCHITECTURE (v11.0)        {} NodeKinds total        ║",
        stats.total_kinds
    ));
    lines.push(
        "╠═══════════════════════════════════════════════════════════════════════════╣".to_string(),
    );

    let mut item_index = 0;
    for realm in &stats.realms {
        let is_realm_selected = cursor == item_index;
        let realm_prefix = if is_realm_selected { "▶" } else { " " };
        let realm_style = if realm.key == "global" {
            "READ-ONLY"
        } else {
            "per-tenant"
        };

        lines.push(
            "║                                                                           ║"
                .to_string(),
        );
        lines.push(format!(
            "║  {} ┌─ {} ({}) ─────────────────────── {} kinds ─────┐  ║",
            realm_prefix,
            realm.display_name.to_uppercase(),
            realm_style,
            realm.total_kinds
        ));

        item_index += 1;

        for layer in &realm.layers {
            let is_layer_selected = cursor == item_index;
            let layer_prefix = if is_layer_selected { "▶" } else { " " };
            let expanded = if zoomed && is_layer_selected {
                " [expanded]"
            } else {
                ""
            };

            // Pad layer name to align counts
            let padded_name = format!("{:<20}", layer.display_name);
            lines.push(format!(
                "║  {}  │  {} {:>3} kinds{}",
                layer_prefix, padded_name, layer.kind_count, expanded
            ));

            // If zoomed into this layer, show more detail
            if zoomed && is_layer_selected {
                lines.push("║     │    └─ (press Enter to see Kind list)".to_string());
            }

            item_index += 1;
        }

        lines.push(
            "║    └─────────────────────────────────────────────────────────────────┘  ║"
                .to_string(),
        );
    }

    // Arrow between realms
    if stats.realms.len() > 1 {
        lines.push(
            "║                              │                                          ║"
                .to_string(),
        );
        lines.push(
            "║                              ▼ cross_realm arcs                        ║"
                .to_string(),
        );
    }

    lines.push(
        "║                                                                           ║".to_string(),
    );
    lines.push(
        "╚═══════════════════════════════════════════════════════════════════════════╝".to_string(),
    );

    // Navigation hint
    lines.push(String::new());
    lines.push(format!(
        "  Cursor: {}/{} │ Press Enter to {} │ Press [d] to toggle demo mode",
        cursor + 1,
        total_items,
        if zoomed { "collapse" } else { "expand" }
    ));
}

/// Render Realm Map in demo mode (static example).
fn render_realm_map_demo(lines: &mut Vec<String>, cursor: usize) {
    lines.push(
        "╔═══════════════════════════════════════════════════════════════════════════╗".to_string(),
    );
    lines.push(
        "║  2-REALM ARCHITECTURE (v11.0)             DEMO DATA                       ║".to_string(),
    );
    lines.push(
        "╠═══════════════════════════════════════════════════════════════════════════╣".to_string(),
    );
    lines.push(
        "║                                                                           ║".to_string(),
    );

    let global_selected = cursor == 0;
    let g_prefix = if global_selected { "▶" } else { " " };
    lines.push(format!(
        "║  {} ┌─ GLOBAL (READ-ONLY) ───────────────────── 14 kinds ──────┐          ║",
        g_prefix
    ));
    lines.push(
        "║    │  config              2 kinds   (Taxonomy, VisualEncoding)│          ║".to_string(),
    );
    lines.push(
        "║    │  locale-knowledge   12 kinds   (Locale, TermSet, Term...)│          ║".to_string(),
    );
    lines.push(
        "║    └──────────────────────────────────────────────────────────┘          ║".to_string(),
    );
    lines.push(
        "║                              │                                          ║".to_string(),
    );
    lines.push(
        "║                              ▼                                          ║".to_string(),
    );

    let tenant_selected = cursor == 1;
    let t_prefix = if tenant_selected { "▶" } else { " " };
    lines.push(format!(
        "║  {} ┌─ TENANT ─────────────────────────────── 32 kinds ──────┐          ║",
        t_prefix
    ));
    lines.push(
        "║    │  config              2 kinds   (TenantConfig, Settings) │          ║".to_string(),
    );
    lines.push(
        "║    │  foundation          4 kinds   (Project, BrandIdentity...)│          ║".to_string(),
    );
    lines.push(
        "║    │  structure           3 kinds   (Page, Block, ContentSlot)│          ║".to_string(),
    );
    lines.push(
        "║    │  semantic            3 kinds   (Knowledge atom usage)   │          ║".to_string(),
    );
    lines.push(
        "║    │  instruction         4 kinds   (Prompt, GenerationJob)  │          ║".to_string(),
    );
    lines.push(
        "║    │  seo                 9 kinds   (SEOKeyword, GeoQuery...)│          ║".to_string(),
    );
    lines.push(
        "║    │  output              5 kinds   (PageGenerated, BlockGen.)│          ║".to_string(),
    );
    lines.push(
        "║    └──────────────────────────────────────────────────────────┘          ║".to_string(),
    );
    lines.push(
        "║                                                                           ║".to_string(),
    );
    lines.push(
        "╚═══════════════════════════════════════════════════════════════════════════╝".to_string(),
    );

    // Legend
    lines.push(String::new());
    lines.push("  DEMO: Static example data │ Press [d] to load live data from Neo4j".to_string());
}

/// Render the Knowledge Atoms view in Atlas mode.
/// Shows selective loading vs monolithic blob approach.
fn render_atlas_knowledge_atoms(app: &App) -> String {
    let mut lines = Vec::new();

    // Header
    let mode = if app.atlas.demo_mode { "DEMO" } else { "LIVE" };
    lines.push(format!(
        "  [{}]  |  Knowledge Atoms: Selective vs Monolithic Loading",
        mode
    ));
    lines.push(String::new());

    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(
        "║  KNOWLEDGE ATOMS ARCHITECTURE                                              ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // Problem: Monolithic approach
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  ❌ MONOLITHIC APPROACH (Traditional)                                       ║"
            .to_string(),
    );
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Locale                                                                  ║"
            .to_string(),
    );
    lines.push(
        "║      └── knowledge_data: {                                                 ║"
            .to_string(),
    );
    lines.push(
        "║            \"terms\": [... 20,000 entries ...],      ← 2MB JSON blob        ║"
            .to_string(),
    );
    lines.push(
        "║            \"expressions\": [... 5,000 entries ...],                        ║"
            .to_string(),
    );
    lines.push(
        "║            \"patterns\": [... 1,000 entries ...],                           ║"
            .to_string(),
    );
    lines.push(
        "║          }                                                                 ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Problems:                                                               ║"
            .to_string(),
    );
    lines.push(
        "║    • Load 2MB to use 50 terms                                              ║"
            .to_string(),
    );
    lines.push(
        "║    • Can't query: \"Which terms does this Block use?\"                       ║"
            .to_string(),
    );
    lines.push(
        "║    • Can't trace: \"Which Blocks use 'conversion' term?\"                    ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Solution: Knowledge Atoms
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  ✅ KNOWLEDGE ATOMS (NovaNet)                                               ║"
            .to_string(),
    );
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Locale ──[:HAS_TERMS]──> TermSet ──[:CONTAINS]──> Term                  ║"
            .to_string(),
    );
    lines.push(
        "║           ──[:HAS_EXPRESSIONS]──> ExpressionSet ──[:CONTAINS]──> Expression║"
            .to_string(),
    );
    lines.push(
        "║           ──[:HAS_PATTERNS]──> PatternSet ──[:CONTAINS]──> Pattern         ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Block ──[:USES_TERM]──> Term                                            ║"
            .to_string(),
    );
    lines.push(
        "║          ──[:USES_EXPRESSION]──> Expression                                ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Benefits:                                                               ║"
            .to_string(),
    );
    lines.push(
        "║    • Load 50 relevant terms, not 20K blob                                  ║"
            .to_string(),
    );
    lines.push(
        "║    • Query: MATCH (b:Block)-[:USES_TERM]->(t:Term) WHERE b.key = $key      ║"
            .to_string(),
    );
    lines.push(
        "║    • Trace: MATCH (t:Term)<-[:USES_TERM]-(b:Block) WHERE t.term = $term    ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Atom types
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  ATOM TYPES (6 Sets + 6 Atoms = 12 NodeKinds)                              ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  ┌─────────────────┬─────────────────┬──────────────────────────────────┐  ║"
            .to_string(),
    );
    lines.push(
        "║  │ Container (Set) │ Atom            │ Purpose                          │  ║"
            .to_string(),
    );
    lines.push(
        "║  ├─────────────────┼─────────────────┼──────────────────────────────────┤  ║"
            .to_string(),
    );
    lines.push(
        "║  │ TermSet         │ Term            │ Vocabulary, definitions          │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ ExpressionSet   │ Expression      │ Idioms, phrases, collocations    │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ PatternSet      │ Pattern         │ Sentence structures, templates   │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ CultureSet      │ CultureRef      │ Cultural references, symbols     │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ TabooSet        │ Taboo           │ Words/topics to avoid            │  ║"
            .to_string(),
    );
    lines.push(
        "║  │ AudienceSet     │ AudienceTrait   │ Reader characteristics           │  ║"
            .to_string(),
    );
    lines.push(
        "║  └─────────────────┴─────────────────┴──────────────────────────────────┘  ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  KEY PRINCIPLE: Containers are EMPTY — all data lives in atoms            ║".to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    lines.join("\n")
}

/// Render the Generation Pipeline view in Atlas mode.
/// Shows the 6-stage native content generation flow (NOT translation).
fn render_atlas_generation_pipeline(app: &App) -> String {
    let mut lines = Vec::new();
    let stage = app.atlas.pipeline_stage;

    // Navigation hint
    lines.push(format!(
        "  h/l: prev/next stage  |  Stage {}/5  |  Generation, NOT Translation",
        stage + 1
    ));
    lines.push(String::new());

    // Pipeline stages
    let stages = [
        (
            "ENTITY (invariant)",
            "The core concept that exists independently of locale",
        ),
        (
            "TASK (job)",
            "What kind of content to generate (Hero, FAQ, CTA...)",
        ),
        (
            "CONTEXT ASSEMBLY",
            "Spreading activation + selective knowledge atom loading",
        ),
        (
            "PROMPT ENGINEERING",
            "Rules, style guides, locale-specific patterns",
        ),
        (
            "GENERATION",
            "LLM call with assembled context → native content",
        ),
        ("OUTPUT", "Generated/Content nodes per locale"),
    ];

    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(
        "║  NATIVE CONTENT GENERATION PIPELINE                                       ║".to_string(),
    );
    lines.push(
        "║  ════════════════════════════════════════════════════════════════════════ ║".to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Show flow diagram
    for (i, (name, desc)) in stages.iter().enumerate() {
        let is_current = i == stage;
        let prefix = if is_current { "▶" } else { " " };
        let highlight = if is_current { "★" } else { " " };

        // Stage box
        if is_current {
            lines.push(format!(
                "║  {} ┌────────────────────────────────────────────────────────────────┐ {} ║",
                prefix, highlight
            ));
            lines.push(format!(
                "║    │  {}. {}                                                    │   ║",
                i + 1,
                name
            ));
            lines.push(
                "║    │                                                                  │   ║"
                    .to_string(),
            );
            lines.push(format!("║    │  {}  │   ║", truncate_str(desc, 60)));
            lines.push(
                "║    └────────────────────────────────────────────────────────────────┘   ║"
                    .to_string(),
            );
        } else {
            lines.push(format!(
                "║  {} [ {}. {} ]                                                       ║",
                prefix,
                i + 1,
                truncate_str(name, 50)
            ));
        }

        // Arrow between stages
        if i < stages.len() - 1 {
            lines.push(
                "║                              │                                          ║"
                    .to_string(),
            );
            lines.push(
                "║                              ▼                                          ║"
                    .to_string(),
            );
        }
    }

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // Stage-specific details
    lines.push(format!(
        "║  STAGE {} DETAILS                                                           ║",
        stage + 1
    ));
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    match stage {
        0 => {
            // Entity stage
            lines.push(
                "║  ENTITY: The invariant concept                                           ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  • Entity.key = \"qrcode-ai\" (universal identifier)                         ║"
                    .to_string(),
            );
            lines.push(
                "║  • Entity.display_name = \"QR Code AI\"                                      ║"
                    .to_string(),
            );
            lines.push(
                "║  • NO locale-specific content here                                         ║"
                    .to_string(),
            );
            lines.push(
                "║  • Links to EntityContent for each locale                                   ║"
                    .to_string(),
            );
        }
        1 => {
            // Task stage
            lines.push(
                "║  TASK: What type of content to generate                                   ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push("║  • GenerationTask.task_type = \"hero\" | \"faq\" | \"cta\" | ...                 ║".to_string());
            lines.push(
                "║  • Determines which knowledge atoms get boosted                           ║"
                    .to_string(),
            );
            lines.push(
                "║  • FAQ → boost definitions, Hero → boost benefits                         ║"
                    .to_string(),
            );
        }
        2 => {
            // Context Assembly stage
            lines.push(
                "║  CONTEXT ASSEMBLY: Build relevant context for LLM                         ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  1. Spreading Activation from Entity (decay over hops)                    ║"
                    .to_string(),
            );
            lines.push(
                "║  2. Task-specific boosts (urgency×1.3 for CTA, etc.)                      ║"
                    .to_string(),
            );
            lines.push(
                "║  3. Selective Knowledge Atom loading (50 Terms, not 20K blob)             ║"
                    .to_string(),
            );
            lines.push(
                "║  4. Temperature cutoff: only include atoms > threshold                    ║"
                    .to_string(),
            );
        }
        3 => {
            // Prompt Engineering stage
            lines.push(
                "║  PROMPT ENGINEERING: Structure the LLM request                            ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  • System prompt: role, constraints, output format                        ║"
                    .to_string(),
            );
            lines.push(
                "║  • Context: activated knowledge atoms + entity info                       ║"
                    .to_string(),
            );
            lines.push(
                "║  • Locale rules: fr-FR formal vous, ja-JP honorifics                      ║"
                    .to_string(),
            );
            lines.push(
                "║  • Style guide: brand voice, tone, terminology                            ║"
                    .to_string(),
            );
        }
        4 => {
            // Generation stage
            lines.push(
                "║  GENERATION: Native content creation (NOT translation!)                   ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  ┌─ WRONG ───────────────────────────────────────────────────────────┐    ║"
                    .to_string(),
            );
            lines.push(
                "║  │  Source (en-US) → Translate → Target (fr-FR)                      │    ║"
                    .to_string(),
            );
            lines.push(
                "║  └───────────────────────────────────────────────────────────────────┘    ║"
                    .to_string(),
            );
            lines.push(
                "║  ┌─ RIGHT ───────────────────────────────────────────────────────────┐    ║"
                    .to_string(),
            );
            lines.push(
                "║  │  Entity + Context → Generate natively → Content (locale-native)   │    ║"
                    .to_string(),
            );
            lines.push(
                "║  └───────────────────────────────────────────────────────────────────┘    ║"
                    .to_string(),
            );
        }
        5 => {
            // Output stage
            lines.push(
                "║  OUTPUT: Store localized content                                          ║"
                    .to_string(),
            );
            lines.push(
                "║                                                                            ║"
                    .to_string(),
            );
            lines.push(
                "║  • EntityContent.display_name = \"IA pour QR Code\" (fr-FR)                 ║"
                    .to_string(),
            );
            lines.push(
                "║  • EntityContent.description = \"Créez des QR codes...\" (native French)   ║"
                    .to_string(),
            );
            lines.push(
                "║  • BlockGenerated.content = native locale-specific content                ║"
                    .to_string(),
            );
            lines.push(
                "║  • Links: Entity -[:HAS_CONTENT]-> EntityContent -[:FOR_LOCALE]-> Locale  ║"
                    .to_string(),
            );
        }
        _ => {}
    }

    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    lines.join("\n")
}

/// Render the Page Composition view in Atlas mode.
fn render_atlas_page_composition(app: &App) -> String {
    let mut lines = Vec::new();

    // Mode and navigation indicator
    let mode_label = if app.atlas.demo_mode {
        "[d] DEMO"
    } else {
        "[d] LIVE"
    };
    lines.push(format!(
        "  {}  |  h/l: prev/next page  l: locale ({})  j/k: scroll",
        mode_label, app.atlas.selected_locale
    ));
    lines.push(String::new());

    // Check if we have data
    if app.atlas.pages_list.is_empty() {
        lines.push("Loading pages list from Neo4j...".to_string());
        return lines.join("\n");
    }

    // Page selector
    if app.atlas.page_index < app.atlas.pages_list.len() {
        let page_info = &app.atlas.pages_list[app.atlas.page_index];
        lines.push(format!(
            "  Page {}/{}: {} ({})",
            app.atlas.page_index + 1,
            app.atlas.pages_list.len(),
            page_info.display_name,
            page_info.project_name
        ));
        lines.push(String::new());
    }

    // Page composition data
    if let Some(ref data) = app.atlas.page_data {
        render_page_composition_data(&mut lines, data, &app.atlas.selected_locale);
    } else {
        lines.push("Loading page composition...".to_string());
    }

    lines.join("\n")
}

/// Render the page composition data.
fn render_page_composition_data(
    lines: &mut Vec<String>,
    data: &crate::tui::atlas::PageCompositionData,
    locale: &str,
) {
    // Header
    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(format!(
        "║  PAGE: {:<66}║",
        truncate_str(&data.page_display_name, 66)
    ));
    if let Some(ref page_type) = data.page_type {
        lines.push(format!("║  Type: {:<66}║", truncate_str(page_type, 66)));
    }
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // Generated content info (v10.9: renamed from L10n)
    if let Some(ref generated) = data.page_generated {
        lines.push(format!(
            "║  GENERATED [{}]:                                                          ║",
            locale
        ));
        if let Some(ref title) = generated.title {
            lines.push(format!("║    Title: {:<63}║", truncate_str(title, 63)));
        }
        if let Some(ref slug) = generated.slug {
            lines.push(format!("║    Slug:  /{:<62}║", truncate_str(slug, 62)));
        }
        lines.push(
            "╠════════════════════════════════════════════════════════════════════════════╣"
                .to_string(),
        );
    }

    // Blocks
    lines.push(format!(
        "║  BLOCKS ({})                                                               ║",
        data.blocks.len()
    ));
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );

    for (i, block) in data.blocks.iter().enumerate() {
        let block_type = block.block_type.as_deref().unwrap_or("generic");
        let marker = if i == data.blocks.len() - 1 {
            "└──"
        } else {
            "├──"
        };
        lines.push(format!(
            "║  {} #{} {} [{}]                                               ║",
            marker,
            block.order,
            truncate_str(&block.display_name, 35),
            truncate_str(block_type, 12)
        ));

        // Block generated preview (v10.9: renamed from l10n)
        if let Some(ref generated) = block.generated {
            let preview = truncate_str(&generated.content_preview, 55);
            lines.push(format!(
                "║       └─ \"{}...\"                                 ║",
                preview
            ));
        }
    }

    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // Entities
    lines.push(format!(
        "║  ENTITIES ({})                                                             ║",
        data.entities.len()
    ));
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );

    for entity in &data.entities {
        let blocks_str = entity.connected_blocks.join(", ");
        lines.push(format!(
            "║  • {} → [{}]                                      ║",
            truncate_str(&entity.display_name, 25),
            truncate_str(&blocks_str, 30)
        ));

        // Entity content (v10.9: renamed from l10n)
        if let Some(ref content) = entity.content {
            if let Some(ref name) = content.name {
                lines.push(format!(
                    "║      CONTENT: {}                                         ║",
                    truncate_str(name, 45)
                ));
            }
        }
    }

    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // SEO Keywords
    lines.push(format!(
        "║  SEO KEYWORDS ({})                                                        ║",
        data.seo_keywords.len()
    ));
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );

    for kw in data.seo_keywords.iter().take(10) {
        let vol = kw
            .volume
            .map(|v| format!("{}/mo", v))
            .unwrap_or_else(|| "?".to_string());
        lines.push(format!(
            "║  • \"{}\" ({}) → [{}]                               ║",
            truncate_str(&kw.keyword, 25),
            vol,
            truncate_str(&kw.connected_entities.join(", "), 15)
        ));
    }

    if data.seo_keywords.len() > 10 {
        lines.push(format!(
            "║    ... and {} more keywords                                             ║",
            data.seo_keywords.len() - 10
        ));
    }

    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    // Summary
    lines.push(String::new());
    lines.push(format!(
        "  Summary: {} blocks, {} entities, {} SEO keywords │ Locale: {}",
        data.blocks.len(),
        data.entities.len(),
        data.seo_keywords.len(),
        locale
    ));
}

/// Render the Spreading Activation view in Atlas mode.
/// Shows cognitive science math behind context assembly.
fn render_atlas_spreading_activation(app: &App) -> String {
    let mut lines = Vec::new();
    let step = app.atlas.activation_step;
    let task = &app.atlas.activation_task;

    // Header with navigation hints
    lines.push(format!(
        "  h/l: step activation  t: cycle task [{}]  Enter: reset  |  Step {}",
        task.label(),
        step
    ));
    lines.push(String::new());

    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(
        "║  SPREADING ACTIVATION — Context Assembly for LLM Prompts                   ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // Formula section
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  ACTIVATION FORMULA:                                                       ║"
            .to_string(),
    );
    lines.push(
        "║  ────────────────────────────────────────────────────────────────────────  ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    A(t) = A₀ × e^(-λt) × task_boost                                        ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Where:                                                                  ║"
            .to_string(),
    );
    lines.push(
        "║      A₀   = Initial activation (1.0 for root entity)                       ║"
            .to_string(),
    );
    lines.push(
        "║      λ    = Decay rate (0.3 per hop)                                       ║"
            .to_string(),
    );
    lines.push(
        "║      t    = Distance from root (hop count)                                 ║"
            .to_string(),
    );
    lines.push(
        "║      boost = Task-specific multiplier                                      ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Task boosts
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  TASK-SPECIFIC BOOSTS:                                                     ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    let tasks = [
        (ActivationTask::CTA, "urgency", 1.3),
        (ActivationTask::FAQ, "definition", 1.3),
        (ActivationTask::Hero, "benefit", 1.2),
        (ActivationTask::Pricing, "value", 1.2),
        (ActivationTask::Features, "capability", 1.2),
    ];

    for (t, concept, boost) in &tasks {
        let marker = if t == task { "►" } else { " " };
        let highlight = if t == task { " ◄─ ACTIVE" } else { "" };
        lines.push(format!(
            "║  {} {:12} boosts {:12} by ×{:.1}{}                      ║",
            marker,
            t.label(),
            concept,
            boost,
            highlight
        ));
    }

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Visualization of activation spreading
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  ACTIVATION PROPAGATION:                                                   ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Simulate activation values based on step
    let decay = 0.3_f32;
    let calc_activation = |hop: usize| -> f32 {
        if hop > step {
            0.0
        } else {
            1.0_f32 * (-decay * hop as f32).exp()
        }
    };

    // Visual network representation
    let a0 = calc_activation(0);
    let a1 = calc_activation(1);
    let a2 = calc_activation(2);
    let a3 = calc_activation(3);

    let bar = |a: f32| -> String {
        let filled = (a * 10.0) as usize;
        format!(
            "[{}{}] {:.2}",
            "█".repeat(filled),
            "░".repeat(10 - filled),
            a
        )
    };

    lines.push(
        "║                           ┌─────────────────┐                             ║".to_string(),
    );
    lines.push(
        "║                           │  ROOT ENTITY    │                             ║".to_string(),
    );
    lines.push(format!(
        "║                           │   A₀ = {}   │                             ║",
        bar(a0)
    ));
    lines.push(
        "║                           └────────┬────────┘                             ║".to_string(),
    );
    lines.push(
        "║                        ┌───────────┼───────────┐                          ║".to_string(),
    );
    lines.push(
        "║                        ▼           ▼           ▼                          ║".to_string(),
    );
    lines.push(
        "║              ┌───────────────┐ ┌───────────┐ ┌───────────────┐            ║".to_string(),
    );
    lines.push(
        "║              │   Concept A   │ │ Concept B │ │   Concept C   │            ║".to_string(),
    );
    lines.push(format!(
        "║              │ A₁ = {}│ │A₁ = {} │ │ A₁ = {}│            ║",
        bar(a1),
        bar(a1),
        bar(a1)
    ));
    lines.push(
        "║              └───────┬───────┘ └─────┬─────┘ └───────┬───────┘            ║".to_string(),
    );
    lines.push(
        "║                      ▼               ▼               ▼                    ║".to_string(),
    );
    lines.push(
        "║          ┌─────────────────┐ ┌─────────────┐ ┌─────────────────┐          ║".to_string(),
    );
    lines.push(
        "║          │    Sub-concept  │ │ Sub-concept │ │    Sub-concept  │          ║".to_string(),
    );
    lines.push(format!(
        "║          │  A₂ = {}  │ │A₂ = {} │ │  A₂ = {}  │          ║",
        bar(a2),
        bar(a2),
        bar(a2)
    ));
    lines.push(
        "║          └────────┬────────┘ └──────┬──────┘ └────────┬────────┘          ║".to_string(),
    );
    lines.push(
        "║                   ▼                 ▼                 ▼                   ║".to_string(),
    );
    lines.push(
        "║          ┌─────────────────────────────────────────────────────┐          ║".to_string(),
    );
    lines.push(
        "║          │                   Distant nodes                     │          ║".to_string(),
    );
    lines.push(format!(
        "║          │                A₃ = {}                    │          ║",
        bar(a3)
    ));
    lines.push(
        "║          └─────────────────────────────────────────────────────┘          ║".to_string(),
    );

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Threshold and selection
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  CONTEXT ASSEMBLY:                                                         ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    Threshold: 0.40                                                         ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Count nodes above threshold
    let above_threshold = [a0, a1, a1, a1, a2, a2, a2, a3]
        .iter()
        .filter(|&&a| a >= 0.40)
        .count();
    let total_nodes = 8;

    lines.push(format!(
        "║    Nodes above threshold: {}/{}                                             ║",
        above_threshold, total_nodes
    ));
    lines.push(
        "║    → These nodes become LLM context                                        ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(format!(
        "║    Step {} result: Activation has propagated {} hops from root              ║",
        step, step
    ));
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    lines.join("\n")
}

/// Render the View Traversal Debugger in Atlas mode.
/// Shows the 12 view definitions with traversal patterns.
fn render_atlas_view_traversal(app: &App) -> String {
    let mut lines = Vec::new();
    let cursor = app.atlas.view_cursor;

    // View definitions (static for now, matches _registry.yaml)
    let views = [
        (
            "complete-graph",
            "overview",
            "Full NovaNet graph (all nodes)",
        ),
        (
            "global-layer",
            "overview",
            "Locale config and knowledge (15 nodes)",
        ),
        (
            "seo-keywords",
            "overview",
            "SEO keywords and metrics (3 nodes)",
        ),
        ("project-layer", "overview", "Per-project nodes (14 nodes)"),
        (
            "page-generation-context",
            "generation",
            "Full context for page orchestrator",
        ),
        (
            "block-generation",
            "generation",
            "Context for block sub-agent",
        ),
        (
            "block-semantic-network",
            "generation",
            "Block with spreading activation",
        ),
        (
            "locale-full-knowledge",
            "knowledge",
            "Complete locale knowledge",
        ),
        ("entity-ecosystem", "knowledge", "Entity with Content and SEO"),
        (
            "project-context",
            "project",
            "Project with locales and pages",
        ),
        ("project-overview", "project", "Project dashboard"),
        ("seo-pipeline", "mining", "SEO keyword mining workflow"),
    ];

    // Header
    lines.push(format!(
        "  j/k: navigate  |  View {}/{}  |  Temperature: {:.1}  Depth: {}",
        cursor + 1,
        views.len(),
        app.atlas.traversal_temperature,
        app.atlas.traversal_depth
    ));
    lines.push(String::new());

    lines.push(
        "╔════════════════════════════════════════════════════════════════════════════╗"
            .to_string(),
    );
    lines.push(
        "║  VIEW TRAVERSAL DEBUGGER — 12 View Definitions                             ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );

    // View list (left side) + Detail (right side)
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║  VIEWS                           │ SELECTED VIEW DETAILS                   ║"
            .to_string(),
    );
    lines.push(
        "║  ────────────────────────────────┼─────────────────────────────────────────║"
            .to_string(),
    );

    let selected_idx = cursor.min(views.len() - 1);
    let (sel_id, sel_cat, sel_desc) = views[selected_idx];

    for (i, (id, cat, _desc)) in views.iter().enumerate() {
        let marker = if i == selected_idx { "►" } else { " " };
        let cat_icon = match *cat {
            "overview" => "▣",
            "generation" => "◇",
            "knowledge" => "▤",
            "project" => "▢",
            "mining" => "◆",
            _ => "•",
        };

        // Build the left side (view list)
        let left = format!("  {} {} {:<20}", marker, cat_icon, id);

        // Build the right side (details for selected view only at certain rows)
        let right = match i {
            0 => format!("ID: {}", sel_id),
            1 => format!("Category: {}", sel_cat),
            2 => "Description:".to_string(),
            3 => format!("  {}", truncate_str(sel_desc, 35)),
            5 => "Traversal Pattern:".to_string(),
            6 => "  root → relations → depth".to_string(),
            8 => "Filters:".to_string(),
            9 => "  locale: $locale".to_string(),
            10 => "  temperature: >= 0.3".to_string(),
            _ => String::new(),
        };

        lines.push(format!("║{:<34}│ {:<40}║", left, right));
    }

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Traversal pattern visualization
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║  TRAVERSAL ALGORITHM:                                                      ║"
            .to_string(),
    );
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "║    1. Start at ROOT node (e.g., Block, Page, Entity)                       ║"
            .to_string(),
    );
    lines.push(
        "║    2. Follow INCLUDE relations:                                            ║"
            .to_string(),
    );
    lines.push(
        "║       • direction: outgoing/incoming                                       ║"
            .to_string(),
    );
    lines.push(
        "║       • depth: max hops (default 1)                                        ║"
            .to_string(),
    );
    lines.push(
        "║       • nested includes for deeper traversal                               ║"
            .to_string(),
    );
    lines.push(
        "║    3. Apply FILTERS:                                                       ║"
            .to_string(),
    );
    lines.push(
        "║       • locale: $locale parameter                                          ║"
            .to_string(),
    );
    lines.push(
        "║       • temperature: >= threshold for spreading activation                 ║"
            .to_string(),
    );
    lines.push(
        "║    4. Return assembled context as LLM prompt input                         ║"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Example view structure
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(format!(
        "║  EXAMPLE: {} view structure:{}║",
        sel_id,
        " ".repeat(56 - sel_id.len())
    ));
    lines.push(
        "╠════════════════════════════════════════════════════════════════════════════╣"
            .to_string(),
    );
    lines.push(
        "║                                                                            ║"
            .to_string(),
    );

    // Show example traversal for selected view
    match sel_id {
        "block-generation" => {
            lines.push(
                "║    Block ─────┬─[:HAS_PROMPT]──────> BlockPrompt                         ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:OF_TYPE]──────────> BlockType ─[:HAS_RULES]> Rules     ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:HAS_INSTRUCTION]──> BlockInstruction                   ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:USES_ENTITY]──────> Entity ─[:HAS_CONTENT]> EntityContent║"
                    .to_string(),
            );
            lines.push(
                "║               │                              └─[:SEMANTIC_LINK]> Entity  ║"
                    .to_string(),
            );
            lines.push(
                "║               └─[:HAS_GENERATED]────> BlockGenerated                     ║"
                    .to_string(),
            );
        }
        "page-generation-context" => {
            lines.push(
                "║    Page ──────┬─[:HAS_PROMPT]──────> PagePrompt                          ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:HAS_BLOCK]────────> Block (ordered)                    ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:HAS_GENERATED]────> PageGenerated                      ║"
                    .to_string(),
            );
            lines.push(
                "║               └─[:BELONGS_TO]───────> Project                            ║"
                    .to_string(),
            );
        }
        "entity-ecosystem" => {
            lines.push(
                "║    Entity ────┬─[:HAS_CONTENT]──────> EntityContent                      ║"
                    .to_string(),
            );
            lines.push(
                "║               ├─[:SEMANTIC_LINK]───> Entity (related)                   ║"
                    .to_string(),
            );
            lines.push(
                "║               └─[:EXPRESSES]<──────── SEOKeyword                         ║"
                    .to_string(),
            );
        }
        _ => {
            lines.push(format!(
                "║    Root ──────┬─ [relations defined in {}.yaml]{}║",
                sel_id,
                " ".repeat(31 - sel_id.len().min(30))
            ));
            lines.push(
                "║               └─ See: packages/core/models/views/                        ║"
                    .to_string(),
            );
        }
    }

    lines.push(
        "║                                                                            ║"
            .to_string(),
    );
    lines.push(
        "╚════════════════════════════════════════════════════════════════════════════╝"
            .to_string(),
    );

    lines.join("\n")
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::app::App;
    use crate::tui::atlas::{ActivationTask, AtlasView};
    use crate::tui::data::{AtlasLayerInfo, AtlasPageInfo, AtlasRealmInfo, AtlasRealmStats};
    use crate::tui::data::TaxonomyTree;

    /// Create a minimal test App with default AtlasState.
    fn test_app() -> App {
        let tree = TaxonomyTree::mock_for_testing();
        App::new(tree, "/tmp/test".to_string())
    }

    /// Create test realm stats for live mode testing.
    fn test_realm_stats() -> AtlasRealmStats {
        AtlasRealmStats {
            realms: vec![
                AtlasRealmInfo {
                    key: "global".to_string(),
                    display_name: "Global".to_string(),
                    color: "#2aa198".to_string(),
                    layers: vec![
                        AtlasLayerInfo {
                            key: "config".to_string(),
                            display_name: "Config".to_string(),
                            color: "#6c71c4".to_string(),
                            kind_count: 2,
                        },
                        AtlasLayerInfo {
                            key: "locale-knowledge".to_string(),
                            display_name: "Locale Knowledge".to_string(),
                            color: "#268bd2".to_string(),
                            kind_count: 12,
                        },
                    ],
                    total_kinds: 14,
                },
                AtlasRealmInfo {
                    key: "tenant".to_string(),
                    display_name: "Tenant".to_string(),
                    color: "#d33682".to_string(),
                    layers: vec![
                        AtlasLayerInfo {
                            key: "foundation".to_string(),
                            display_name: "Foundation".to_string(),
                            color: "#cb4b16".to_string(),
                            kind_count: 5,
                        },
                    ],
                    total_kinds: 5,
                },
            ],
            total_kinds: 19,
        }
    }

    // =========================================================================
    // render_atlas_realm_map tests
    // =========================================================================

    #[test]
    fn test_render_atlas_realm_map_demo_mode() {
        let mut app = test_app();
        app.atlas.demo_mode = true;
        app.atlas.current_view = AtlasView::RealmMap;
        app.atlas.realm_cursor = 0;

        let output = render_atlas_realm_map(&app);

        // Should contain demo mode indicator
        assert!(output.contains("[d] DEMO MODE"), "Should show DEMO MODE label");

        // Should contain the 2-realm architecture header
        assert!(
            output.contains("2-REALM ARCHITECTURE"),
            "Should show architecture title"
        );

        // Should contain DEMO DATA indicator
        assert!(output.contains("DEMO DATA"), "Should indicate demo data");

        // Should contain realm names in demo format
        assert!(output.contains("GLOBAL"), "Should show GLOBAL realm");
        assert!(output.contains("TENANT"), "Should show TENANT realm");

        // Should have navigation hint
        assert!(
            output.contains("j/k: navigate"),
            "Should show navigation hint"
        );
    }

    #[test]
    fn test_render_atlas_realm_map_live_mode_no_data() {
        let mut app = test_app();
        app.atlas.demo_mode = false;
        app.atlas.realm_stats = None;

        let output = render_atlas_realm_map(&app);

        // Should indicate live mode
        assert!(
            output.contains("[d] LIVE MODE"),
            "Should show LIVE MODE label"
        );

        // Should show loading message when no data
        assert!(
            output.contains("Loading realm statistics"),
            "Should show loading message"
        );
    }

    #[test]
    fn test_render_atlas_realm_map_live_mode_with_data() {
        let mut app = test_app();
        app.atlas.demo_mode = false;
        app.atlas.realm_stats = Some(test_realm_stats());
        app.atlas.realm_cursor = 0;
        app.atlas.realm_zoomed = false;

        let output = render_atlas_realm_map(&app);

        // Should contain live mode indicator
        assert!(
            output.contains("[d] LIVE MODE"),
            "Should show LIVE MODE label"
        );

        // Should contain realm stats from data
        assert!(output.contains("19 NodeKinds"), "Should show total kinds");

        // Should contain realm names
        assert!(output.contains("GLOBAL"), "Should show Global realm");
        assert!(output.contains("TENANT"), "Should show Tenant realm");

        // Should contain layer names
        assert!(output.contains("Config"), "Should show Config layer");
        assert!(
            output.contains("Locale Knowledge"),
            "Should show Locale Knowledge layer"
        );
        assert!(output.contains("Foundation"), "Should show Foundation layer");

        // Should have cursor indicator on first item
        assert!(output.contains("▶"), "Should show cursor indicator");
    }

    #[test]
    fn test_render_atlas_realm_map_cursor_in_live_mode() {
        let mut app = test_app();
        app.atlas.demo_mode = false;
        app.atlas.realm_stats = Some(test_realm_stats());
        app.atlas.realm_cursor = 2; // Third item (first layer in global)
        app.atlas.realm_zoomed = false;

        let output = render_atlas_realm_map(&app);

        // Cursor position should be shown in live mode
        // Total items = realms (2) + layers (2 in global + 1 in tenant) = 5
        assert!(
            output.contains("Cursor: 3/5"),
            "Should show cursor position 3/5 in live mode"
        );
    }

    #[test]
    fn test_render_atlas_realm_map_zoomed_layer() {
        let mut app = test_app();
        app.atlas.demo_mode = false;
        app.atlas.realm_stats = Some(test_realm_stats());
        app.atlas.realm_cursor = 1; // First layer in global
        app.atlas.realm_zoomed = true;

        let output = render_atlas_realm_map(&app);

        // When zoomed, should show expanded indicator
        assert!(
            output.contains("[expanded]"),
            "Should show expanded indicator when zoomed"
        );

        // Should offer to see Kind list
        assert!(
            output.contains("press Enter to see Kind list"),
            "Should show zoom hint"
        );
    }

    // =========================================================================
    // render_atlas_spreading_activation tests
    // =========================================================================

    #[test]
    fn test_render_atlas_spreading_activation_formula() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::SpreadingActivation;
        app.atlas.activation_step = 0;
        app.atlas.activation_task = ActivationTask::CTA;

        let output = render_atlas_spreading_activation(&app);

        // Should contain the formula
        assert!(
            output.contains("A(t) = A\u{2080} × e^(-λt) × task_boost"),
            "Should show activation formula"
        );

        // Should explain formula components
        assert!(
            output.contains("A\u{2080}   = Initial activation"),
            "Should explain A_0"
        );
        assert!(output.contains("λ    = Decay rate"), "Should explain lambda");
        assert!(
            output.contains("t    = Distance from root"),
            "Should explain t (distance)"
        );
    }

    #[test]
    fn test_render_atlas_spreading_activation_task_boosts() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::SpreadingActivation;
        app.atlas.activation_task = ActivationTask::FAQ;

        let output = render_atlas_spreading_activation(&app);

        // Should show all task boosts
        assert!(output.contains("CTA"), "Should list CTA task");
        assert!(output.contains("FAQ"), "Should list FAQ task");
        assert!(output.contains("Hero"), "Should list Hero task");
        assert!(output.contains("Pricing"), "Should list Pricing task");
        assert!(output.contains("Features"), "Should list Features task");

        // FAQ should be marked as active
        assert!(
            output.contains("FAQ") && output.contains("◄─ ACTIVE"),
            "FAQ should be marked active"
        );

        // Should show boost values
        assert!(output.contains("×1.3"), "Should show 1.3x boost");
        assert!(output.contains("×1.2"), "Should show 1.2x boost");
    }

    #[test]
    fn test_render_atlas_spreading_activation_propagation_step0() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::SpreadingActivation;
        app.atlas.activation_step = 0;

        let output = render_atlas_spreading_activation(&app);

        // At step 0, only root has activation
        assert!(output.contains("Step 0"), "Should show Step 0");

        // Root entity should have full activation bar
        assert!(
            output.contains("ROOT ENTITY"),
            "Should show root entity label"
        );
    }

    #[test]
    fn test_render_atlas_spreading_activation_propagation_step3() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::SpreadingActivation;
        app.atlas.activation_step = 3;

        let output = render_atlas_spreading_activation(&app);

        // At step 3, activation has propagated 3 hops
        assert!(output.contains("Step 3"), "Should show Step 3");
        assert!(
            output.contains("propagated 3 hops"),
            "Should mention 3 hops propagation"
        );

        // Should show context assembly section
        assert!(
            output.contains("CONTEXT ASSEMBLY"),
            "Should show context assembly section"
        );
        assert!(
            output.contains("Threshold: 0.40"),
            "Should show threshold value"
        );
    }

    #[test]
    fn test_render_atlas_spreading_activation_navigation_hints() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::SpreadingActivation;
        app.atlas.activation_task = ActivationTask::Hero;

        let output = render_atlas_spreading_activation(&app);

        // Should show navigation hints
        assert!(
            output.contains("h/l: step activation"),
            "Should show h/l hint"
        );
        assert!(
            output.contains("t: cycle task [Hero]"),
            "Should show task cycle hint with current task"
        );
        assert!(output.contains("Enter: reset"), "Should show reset hint");
    }

    // =========================================================================
    // render_atlas_knowledge_atoms tests
    // =========================================================================

    #[test]
    fn test_render_atlas_knowledge_atoms_header() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::KnowledgeAtoms;
        app.atlas.demo_mode = false;

        let output = render_atlas_knowledge_atoms(&app);

        // Should show mode indicator
        assert!(output.contains("[LIVE]"), "Should show LIVE mode in header");

        // Should show main title
        assert!(
            output.contains("KNOWLEDGE ATOMS ARCHITECTURE"),
            "Should show architecture title"
        );
    }

    #[test]
    fn test_render_atlas_knowledge_atoms_monolithic_problem() {
        let app = test_app();
        let output = render_atlas_knowledge_atoms(&app);

        // Should explain the monolithic problem
        assert!(
            output.contains("MONOLITHIC APPROACH"),
            "Should describe monolithic approach"
        );
        assert!(
            output.contains("20,000 entries"),
            "Should mention large entry count"
        );
        assert!(output.contains("2MB JSON blob"), "Should mention blob size");

        // Should list problems with monolithic approach
        assert!(
            output.contains("Load 2MB to use 50 terms"),
            "Should list wasteful loading problem"
        );
    }

    #[test]
    fn test_render_atlas_knowledge_atoms_solution() {
        let app = test_app();
        let output = render_atlas_knowledge_atoms(&app);

        // Should show NovaNet solution
        assert!(
            output.contains("KNOWLEDGE ATOMS (NovaNet)"),
            "Should describe NovaNet solution"
        );

        // Should show graph relationships
        assert!(
            output.contains("[:HAS_TERMS]"),
            "Should show HAS_TERMS relationship"
        );
        assert!(
            output.contains("[:CONTAINS]"),
            "Should show CONTAINS relationship"
        );
        assert!(
            output.contains("[:USES_TERM]"),
            "Should show USES_TERM relationship"
        );

        // Should show benefits
        assert!(
            output.contains("Load 50 relevant terms"),
            "Should describe selective loading benefit"
        );
    }

    #[test]
    fn test_render_atlas_knowledge_atoms_atom_types_table() {
        let app = test_app();
        let output = render_atlas_knowledge_atoms(&app);

        // Should have atom types section
        assert!(output.contains("ATOM TYPES"), "Should have ATOM TYPES section");
        assert!(
            output.contains("6 Sets + 6 Atoms"),
            "Should mention 6 sets and 6 atoms"
        );

        // Should list all container/atom pairs
        let containers = [
            "TermSet",
            "ExpressionSet",
            "PatternSet",
            "CultureSet",
            "TabooSet",
            "AudienceSet",
        ];
        let atoms = [
            "Term",
            "Expression",
            "Pattern",
            "CultureRef",
            "Taboo",
            "AudienceTrait",
        ];

        for container in containers {
            assert!(
                output.contains(container),
                "Should list container: {}",
                container
            );
        }

        for atom in atoms {
            assert!(output.contains(atom), "Should list atom: {}", atom);
        }
    }

    #[test]
    fn test_render_atlas_knowledge_atoms_key_principle() {
        let app = test_app();
        let output = render_atlas_knowledge_atoms(&app);

        // Should emphasize the key principle
        assert!(
            output.contains("Containers are EMPTY"),
            "Should state containers are empty"
        );
        assert!(
            output.contains("all data lives in atoms"),
            "Should explain data lives in atoms"
        );
    }

    #[test]
    fn test_render_atlas_knowledge_atoms_demo_mode() {
        let mut app = test_app();
        app.atlas.demo_mode = true;

        let output = render_atlas_knowledge_atoms(&app);

        // Should show DEMO in header when demo mode enabled
        assert!(output.contains("[DEMO]"), "Should show DEMO mode in header");
    }

    // =========================================================================
    // render_atlas_generation_pipeline tests
    // =========================================================================

    #[test]
    fn test_render_atlas_generation_pipeline_all_stages() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::GenerationPipeline;

        // Test stage 0 through 5
        for stage in 0..=5 {
            app.atlas.pipeline_stage = stage;
            let output = render_atlas_generation_pipeline(&app);

            assert!(
                output.contains(&format!("Stage {}/5", stage + 1)),
                "Should show Stage {}/5",
                stage + 1
            );
        }
    }

    #[test]
    fn test_render_atlas_generation_pipeline_stage0_entity() {
        let mut app = test_app();
        app.atlas.pipeline_stage = 0;

        let output = render_atlas_generation_pipeline(&app);

        // Stage 0 is about Entity
        assert!(output.contains("ENTITY"), "Stage 0 should discuss ENTITY");
        assert!(
            output.contains("invariant concept"),
            "Should describe invariant concept"
        );
        assert!(
            output.contains("Entity.key"),
            "Should show Entity.key example"
        );
    }

    #[test]
    fn test_render_atlas_generation_pipeline_stage4_generation() {
        let mut app = test_app();
        app.atlas.pipeline_stage = 4;

        let output = render_atlas_generation_pipeline(&app);

        // Stage 4 shows WRONG vs RIGHT generation
        assert!(
            output.contains("NOT translation"),
            "Should emphasize not translation"
        );
        assert!(output.contains("WRONG"), "Should show WRONG approach");
        assert!(output.contains("RIGHT"), "Should show RIGHT approach");
        assert!(
            output.contains("Generate natively"),
            "Should describe native generation"
        );
    }

    // =========================================================================
    // render_atlas_view_traversal tests
    // =========================================================================

    #[test]
    fn test_render_atlas_view_traversal_12_views() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::ViewTraversal;

        let output = render_atlas_view_traversal(&app);

        // Should mention 12 view definitions
        assert!(
            output.contains("12 View Definitions"),
            "Should mention 12 views"
        );

        // Should list some view names
        let view_names = [
            "complete-graph",
            "global-layer",
            "block-generation",
            "entity-ecosystem",
        ];
        for name in view_names {
            assert!(output.contains(name), "Should list view: {}", name);
        }
    }

    #[test]
    fn test_render_atlas_view_traversal_algorithm() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::ViewTraversal;

        let output = render_atlas_view_traversal(&app);

        // Should describe traversal algorithm
        assert!(
            output.contains("TRAVERSAL ALGORITHM"),
            "Should show algorithm section"
        );
        assert!(
            output.contains("Start at ROOT node"),
            "Should mention root node"
        );
        assert!(
            output.contains("INCLUDE relations"),
            "Should mention include relations"
        );
        assert!(output.contains("Apply FILTERS"), "Should mention filters");
    }

    // =========================================================================
    // render_atlas_page_composition tests
    // =========================================================================

    #[test]
    fn test_render_atlas_page_composition_no_pages() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::PageComposition;
        app.atlas.pages_list = Vec::new();

        let output = render_atlas_page_composition(&app);

        // Should show loading message when no pages
        assert!(
            output.contains("Loading pages list"),
            "Should show loading message"
        );
    }

    #[test]
    fn test_render_atlas_page_composition_with_pages() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::PageComposition;
        app.atlas.pages_list = vec![
            AtlasPageInfo {
                key: "home".to_string(),
                display_name: "Home Page".to_string(),
                project_key: "qrcode-ai".to_string(),
                project_name: "QR Code AI".to_string(),
            },
            AtlasPageInfo {
                key: "pricing".to_string(),
                display_name: "Pricing".to_string(),
                project_key: "qrcode-ai".to_string(),
                project_name: "QR Code AI".to_string(),
            },
        ];
        app.atlas.page_index = 0;

        let output = render_atlas_page_composition(&app);

        // Should show page info
        assert!(output.contains("Page 1/2"), "Should show page number");
        assert!(
            output.contains("Home Page"),
            "Should show page display name"
        );
        assert!(output.contains("QR Code AI"), "Should show project name");

        // Should show navigation hints
        assert!(
            output.contains("h/l: prev/next page"),
            "Should show navigation hint"
        );
    }

    // =========================================================================
    // Edge cases and error handling
    // =========================================================================

    #[test]
    fn test_render_empty_realm_stats() {
        let mut app = test_app();
        app.atlas.demo_mode = false;
        app.atlas.realm_stats = Some(AtlasRealmStats {
            realms: Vec::new(),
            total_kinds: 0,
        });

        let output = render_atlas_realm_map(&app);

        // Should handle empty stats gracefully
        assert!(
            output.contains("0 NodeKinds"),
            "Should show 0 kinds for empty stats"
        );
    }

    #[test]
    fn test_render_spreading_activation_high_step() {
        let mut app = test_app();
        app.atlas.activation_step = 100;

        let output = render_atlas_spreading_activation(&app);

        // Should handle high step values gracefully
        assert!(output.contains("Step 100"), "Should display high step value");
        assert!(
            output.contains("propagated 100 hops"),
            "Should show hops for high step"
        );
    }

    #[test]
    fn test_all_activation_tasks_render() {
        let mut app = test_app();
        app.atlas.current_view = AtlasView::SpreadingActivation;

        let tasks = [
            ActivationTask::CTA,
            ActivationTask::FAQ,
            ActivationTask::Hero,
            ActivationTask::Pricing,
            ActivationTask::Features,
        ];

        for task in tasks {
            app.atlas.activation_task = task;
            let output = render_atlas_spreading_activation(&app);

            // Each task should show its label as active
            let label = task.label();
            assert!(
                output.contains(label),
                "Should show task {} in output",
                label
            );
        }
    }
}
