//! Flow diagram content for TUI Flow view.
//!
//! Two diagrams:
//! 1. Schema Architecture — Realm > Layer > Class hierarchy with key arcs
//! 2. Data Pipeline — Entity → EntityNative → Page → Block → BlockNative flow

/// A single line in a flow diagram, with optional highlighted segments.
#[derive(Debug, Clone)]
pub struct FlowLine {
    pub text: String,
    /// Highlighted segments: (start_col, end_col, node_index)
    /// node_index is used for selection navigation
    pub highlights: Vec<(usize, usize, usize)>,
}

/// A complete flow diagram with metadata.
#[derive(Debug, Clone)]
pub struct FlowDiagram {
    pub title: String,
    pub lines: Vec<FlowLine>,
    pub node_count: usize,
    /// Node labels for the info/description panel
    pub node_labels: Vec<String>,
    pub node_descriptions: Vec<String>,
}

/// Build the Schema Architecture diagram.
pub fn schema_architecture() -> FlowDiagram {
    let mut node_idx: usize = 0;
    let mut node_labels = Vec::new();
    let mut node_descriptions = Vec::new();
    let mut lines = Vec::new();

    // Helper to register a node
    let mut add_node = |label: &str, desc: &str| -> usize {
        let idx = node_idx;
        node_labels.push(label.to_string());
        node_descriptions.push(desc.to_string());
        node_idx += 1;
        idx
    };

    let shared = add_node("SHARED", "Read-only universal definitions (36 nodes)");
    let org = add_node("ORG", "Organization-specific content (23 nodes)");
    let config_s = add_node("config (shared)", "BCP-47 locale definitions, ScriptSystem, WritingDirection");
    let locale = add_node("locale", "LocaleVoice, LocaleCulture, LocaleFormatting, LocaleAdaptation, LocaleSlugification");
    let geo = add_node("geography", "Continent, Country, Region, City, PopulationCluster, EconomicRegion, CulturalRealm");
    let knowledge = add_node("knowledge", "ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet + atoms (21 nodes)");
    let config_o = add_node("config (org)", "OrgConfig");
    let foundation = add_node("foundation", "Project, Brand, BrandDesign, BrandPrinciples, PromptStyle + 3 more (8 nodes)");
    let structure = add_node("structure", "Page, Block, EntityCategory (3 nodes)");
    let semantic = add_node("semantic", "Entity, EntityNative (2 nodes)");
    let instruction = add_node("instruction", "BlockType, BlockInstruction, PageInstruction (3 nodes)");
    let output = add_node("output", "PageNative, BlockNative, ProjectNative + 3 more (6 nodes)");

    let plain = |text: &str| FlowLine { text: text.to_string(), highlights: vec![] };
    let with_hl = |text: &str, highlights: Vec<(usize, usize, usize)>| FlowLine { text: text.to_string(), highlights };

    lines.push(plain(""));
    lines.push(plain("  NovaNet v0.20.0 — 59 Nodes, 159 Arcs, 6 Families"));
    lines.push(plain("  ═══════════════════════════════════════════════════════════════════════"));
    lines.push(plain(""));
    lines.push(with_hl(
        "  ┌─── SHARED (36 nodes, READ-ONLY) ──────┐   ┌─── ORG (23 nodes) ──────────────┐",
        vec![(6, 40, shared), (46, 77, org)],
    ));
    lines.push(plain("  │                                        │   │                                  │"));
    lines.push(with_hl(
        "  │  config ─────── Locale, Script, Dir    │   │  config ──── OrgConfig            │",
        vec![(5, 11, config_s), (46, 52, config_o)],
    ));
    lines.push(plain("  │     │                                  │   │     │                              │"));
    lines.push(with_hl(
        "  │  locale ─────── Voice, Culture, Format │   │  foundation ── Project, Brand     │",
        vec![(5, 11, locale), (46, 56, foundation)],
    ));
    lines.push(plain("  │     │                                  │   │     │                              │"));
    lines.push(with_hl(
        "  │  geography ──── Continent..City (7)    │   │  structure ─── Page, Block        │",
        vec![(5, 14, geo), (46, 55, structure)],
    ));
    lines.push(plain("  │     │                                  │   │     │                              │"));
    lines.push(with_hl(
        "  │  knowledge ──── Expressions, Patterns  │   │  semantic ──── Entity, Native     │",
        vec![(5, 14, knowledge), (46, 54, semantic)],
    ));
    lines.push(plain("  │                  Taboos, Audience (21)  │   │     │                              │"));
    lines.push(with_hl(
        "  │                                        │   │  instruction ─ BlockType, Instr   │",
        vec![(46, 57, instruction)],
    ));
    lines.push(plain("  │                                        │   │     │                              │"));
    lines.push(with_hl(
        "  │                                        │   │  output ────── PageNative, Block  │",
        vec![(46, 52, output)],
    ));
    lines.push(plain("  │                                        │   │                Native, Project   │"));
    lines.push(plain("  └────────────────────────────────────────┘   └──────────────────────────────────┘"));
    lines.push(plain(""));
    lines.push(plain("  ── ARC FAMILIES (6) ──────────────────────────────────────────────────"));
    lines.push(plain(""));
    lines.push(plain("  ownership ─────── HAS_PAGE, HAS_BLOCK, HAS_ENTITY, HAS_BRAND, ..."));
    lines.push(plain("  localization ──── HAS_NATIVE, FOR_LOCALE, NATIVE_OF"));
    lines.push(plain("  semantic ──────── USES_ENTITY, ABOUT, SEMANTIC_LINK, REFERENCES_PAGE"));
    lines.push(plain("  generation ────── GENERATES, DERIVED_FROM"));
    lines.push(plain("  mining ────────── TARGETS, ANSWERS, CLUSTERS"));
    lines.push(plain("  schema ────────── OF_CLASS, FROM_CLASS, TO_CLASS"));
    lines.push(plain(""));

    FlowDiagram {
        title: "Schema Architecture".to_string(),
        lines,
        node_count: node_idx,
        node_labels,
        node_descriptions,
    }
}

/// Build the Data Pipeline diagram.
pub fn data_pipeline() -> FlowDiagram {
    let mut node_idx: usize = 0;
    let mut node_labels = Vec::new();
    let mut node_descriptions = Vec::new();
    let mut lines = Vec::new();

    let mut add_node = |label: &str, desc: &str| -> usize {
        let idx = node_idx;
        node_labels.push(label.to_string());
        node_descriptions.push(desc.to_string());
        node_idx += 1;
        idx
    };

    let project = add_node("Project", "Root container — owns Pages, Entities, Brand");
    let entity = add_node("Entity", "Semantic concept (defined). Key: kebab-case");
    let entity_native = add_node("EntityNative", "Locale-specific content (authored). Key: entity@locale");
    let page = add_node("Page", "URL structure (defined). Owns Blocks");
    let block = add_node("Block", "Content section (defined). Has BlockType + instruction");
    let block_native = add_node("BlockNative", "Generated content (LLM output). One per Block+Locale");
    let page_native = add_node("PageNative", "Assembled page (generated). Slug lives here (ADR-030)");
    let locale = add_node("Locale", "BCP-47 locale config. Voice + Culture + Knowledge");
    let brand = add_node("Brand", "Brand identity — Design, Principles, PromptStyle");
    let seo = add_node("SEOKeyword", "Imported from Ahrefs. Targets EntityNative");

    let plain = |text: &str| FlowLine { text: text.to_string(), highlights: vec![] };
    let with_hl = |text: &str, highlights: Vec<(usize, usize, usize)>| FlowLine { text: text.to_string(), highlights };

    lines.push(plain(""));
    lines.push(plain("  NovaNet Content Generation Pipeline"));
    lines.push(plain("  ═══════════════════════════════════════════════════════════════════════"));
    lines.push(plain(""));
    lines.push(plain("  CRITICAL: Generation, NOT Translation"));
    lines.push(plain("  Entity (defined) ──▶ Generate natively ──▶ EntityNative (authored)"));
    lines.push(plain(""));
    lines.push(with_hl(
        "                              Project",
        vec![(30, 37, project)],
    ));
    lines.push(plain("                           ┌────┼────────┐"));
    lines.push(plain("                 [:HAS_ENTITY] [:HAS_PAGE] [:HAS_BRAND]"));
    lines.push(plain("                           │    │          │"));
    lines.push(with_hl(
        "                        Entity  Page       Brand",
        vec![(24, 30, entity), (32, 36, page), (43, 48, brand)],
    ));
    lines.push(plain("                           │    │"));
    lines.push(plain("              [:HAS_NATIVE] │   │ [:HAS_BLOCK {order}]"));
    lines.push(plain("                           │    │"));
    lines.push(with_hl(
        "                    EntityNative  Block ──[:OF_TYPE]──▶ BlockType",
        vec![(20, 32, entity_native), (34, 39, block)],
    ));
    lines.push(plain("                           │    │"));
    lines.push(plain("                [:FOR_LOCALE]    │ [:HAS_INSTRUCTION]"));
    lines.push(plain("                           │    │"));
    lines.push(with_hl(
        "                        Locale  BlockInstruction",
        vec![(24, 30, locale)],
    ));
    lines.push(plain("                                 │"));
    lines.push(plain("                                 │ [:USES_ENTITY] ──▶ Entity"));
    lines.push(plain("                                 │"));
    lines.push(plain("  ── GENERATION PHASE ────────────────────────────────────────────────"));
    lines.push(plain(""));
    lines.push(plain("    novanet_context(mode=page) assembles:"));
    lines.push(plain("    ├── EntityNative.denomination_forms (ADR-033)"));
    lines.push(plain("    ├── Locale knowledge (Expressions, Patterns, Taboos)"));
    lines.push(plain("    ├── Brand voice (PromptStyle, BrandPrinciples)"));
    lines.push(plain("    └── Block instructions + cross-page anchors"));
    lines.push(plain(""));
    lines.push(plain("                    ┌──────────────────────────────┐"));
    lines.push(with_hl(
        "                    │      BlockNative (per block)   │",
        vec![(27, 39, block_native)],
    ));
    lines.push(plain("                    │  LLM-generated content       │"));
    lines.push(plain("                    │  with denomination_forms     │"));
    lines.push(plain("                    └──────────────────────────────┘"));
    lines.push(plain("                                 │"));
    lines.push(plain("                                 ▼ assembled"));
    lines.push(plain("                    ┌──────────────────────────────┐"));
    lines.push(with_hl(
        "                    │      PageNative               │",
        vec![(27, 37, page_native)],
    ));
    lines.push(plain("                    │  slug: /fr/code-qr (ADR-030) │"));
    lines.push(plain("                    │  meta_title, meta_description │"));
    lines.push(plain("                    └──────────────────────────────┘"));
    lines.push(plain(""));
    lines.push(plain("  ── SEO FEEDBACK LOOP ───────────────────────────────────────────────"));
    lines.push(plain(""));
    lines.push(with_hl(
        "    SEOKeyword ──[:TARGETS]──▶ EntityNative (url form write-back)",
        vec![(4, 14, seo)],
    ));
    lines.push(plain("    GEOQuery ──[:ANSWERS]──▶ GEOAnswer (mined from search)"));
    lines.push(plain(""));

    FlowDiagram {
        title: "Data Pipeline".to_string(),
        lines,
        node_count: node_idx,
        node_labels,
        node_descriptions,
    }
}
