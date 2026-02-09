//! Atlas Mode state structures.

/// Which Atlas view is currently active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AtlasView {
    /// `a` Spreading Activation — Cognitive science math
    SpreadingActivation,
    /// `b` Knowledge Atoms — Selective loading vs blobs
    KnowledgeAtoms,
    /// `c` Generation Pipeline — Block generation flow
    GenerationPipeline,
    /// `v` View Traversal — Debug view definitions
    ViewTraversal,
    /// `e` Page Composition — Anatomy of a Page
    PageComposition,
    /// `r` Realm Map — Bird's-eye view
    #[default]
    RealmMap,
}

impl AtlasView {
    /// Get the shortcut key for this view.
    pub fn shortcut(&self) -> char {
        match self {
            AtlasView::SpreadingActivation => 'a',
            AtlasView::KnowledgeAtoms => 'b',
            AtlasView::GenerationPipeline => 'c',
            AtlasView::ViewTraversal => 'v',
            AtlasView::PageComposition => 'e',
            AtlasView::RealmMap => 'r',
        }
    }

    /// Get the display label for this view.
    pub fn label(&self) -> &'static str {
        match self {
            AtlasView::SpreadingActivation => "Activation",
            AtlasView::KnowledgeAtoms => "Atoms",
            AtlasView::GenerationPipeline => "Pipeline",
            AtlasView::ViewTraversal => "Traversal",
            AtlasView::PageComposition => "Page",
            AtlasView::RealmMap => "Realm",
        }
    }

    /// Get all views in order.
    pub fn all() -> &'static [AtlasView] {
        &[
            AtlasView::SpreadingActivation,
            AtlasView::KnowledgeAtoms,
            AtlasView::GenerationPipeline,
            AtlasView::ViewTraversal,
            AtlasView::PageComposition,
            AtlasView::RealmMap,
        ]
    }
}

/// Main Atlas mode state.
#[derive(Debug, Clone)]
pub struct AtlasState {
    /// Currently active view.
    pub current_view: AtlasView,

    /// Demo mode (static example data) vs Live mode (Neo4j data).
    pub demo_mode: bool,

    /// Selected locale for localized views.
    pub selected_locale: String,

    /// General scroll offset.
    pub scroll: usize,

    // === Spreading Activation state ===
    /// Current step in activation propagation.
    pub activation_step: usize,
    /// Selected entity key for activation root.
    pub activation_root: Option<String>,
    /// Task type for activation boosts.
    pub activation_task: ActivationTask,

    // === Page Composition state ===
    /// Current page index in project.
    pub page_index: usize,
    /// Total pages in project (set by Neo4j load).
    pub page_count: usize,
    /// Current page key.
    pub current_page_key: Option<String>,
    /// Pending page data load.
    pub pending_page_load: bool,
    /// Loaded page composition data.
    pub page_data: Option<PageCompositionData>,
    /// Pending pages list load.
    pub pending_pages_list_load: bool,
    /// List of available pages.
    pub pages_list: Vec<crate::tui::data::AtlasPageInfo>,

    // === Generation Pipeline state ===
    /// Current stage in pipeline (0-5).
    pub pipeline_stage: usize,
    /// Block key for pipeline view.
    pub pipeline_block_key: Option<String>,

    // === View Traversal state ===
    /// Selected view definition index.
    pub view_cursor: usize,
    /// Temperature threshold for filtering.
    pub traversal_temperature: f32,
    /// Max depth for traversal.
    pub traversal_depth: usize,

    // === Realm Map state ===
    /// Cursor position (0=global, 1+=layers).
    pub realm_cursor: usize,
    /// Whether zoomed into a layer.
    pub realm_zoomed: bool,
    /// Node counts per layer (loaded from Neo4j).
    pub layer_counts: Vec<(String, usize)>,
    /// Pending realm stats load.
    pub pending_realm_stats_load: bool,
    /// Loaded realm stats from Neo4j.
    pub realm_stats: Option<crate::tui::data::AtlasRealmStats>,
}

impl Default for AtlasState {
    fn default() -> Self {
        Self {
            current_view: AtlasView::RealmMap,
            demo_mode: false,
            selected_locale: "en-US".to_string(),
            scroll: 0,

            // Spreading Activation
            activation_step: 0,
            activation_root: None,
            activation_task: ActivationTask::default(),

            // Page Composition
            page_index: 0,
            page_count: 0,
            current_page_key: None,
            pending_page_load: false,
            page_data: None,
            pending_pages_list_load: true, // Load pages list on first render
            pages_list: Vec::new(),

            // Generation Pipeline
            pipeline_stage: 0,
            pipeline_block_key: None,

            // View Traversal
            view_cursor: 0,
            traversal_temperature: 0.5,
            traversal_depth: 2,

            // Realm Map
            realm_cursor: 0,
            realm_zoomed: false,
            layer_counts: Vec::new(),
            pending_realm_stats_load: true, // Load on first render
            realm_stats: None,
        }
    }
}

/// Task type for spreading activation boosts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActivationTask {
    #[default]
    CTA,
    FAQ,
    Hero,
    Pricing,
    Features,
}

impl ActivationTask {
    /// Get boost multiplier for this task.
    pub fn boost(&self, concept_type: &str) -> f32 {
        match (self, concept_type) {
            (ActivationTask::CTA, "urgency") => 1.3,
            (ActivationTask::FAQ, "definition") => 1.3,
            (ActivationTask::Hero, "benefit") => 1.2,
            (ActivationTask::Pricing, "value") => 1.2,
            (ActivationTask::Features, "capability") => 1.2,
            _ => 1.0,
        }
    }

    /// Get display label.
    pub fn label(&self) -> &'static str {
        match self {
            ActivationTask::CTA => "CTA",
            ActivationTask::FAQ => "FAQ",
            ActivationTask::Hero => "Hero",
            ActivationTask::Pricing => "Pricing",
            ActivationTask::Features => "Features",
        }
    }

    /// Cycle to next task.
    pub fn next(&self) -> Self {
        match self {
            ActivationTask::CTA => ActivationTask::FAQ,
            ActivationTask::FAQ => ActivationTask::Hero,
            ActivationTask::Hero => ActivationTask::Pricing,
            ActivationTask::Pricing => ActivationTask::Features,
            ActivationTask::Features => ActivationTask::CTA,
        }
    }
}

/// Page composition data loaded from Neo4j.
#[derive(Debug, Clone, Default)]
pub struct PageCompositionData {
    /// Page key.
    pub page_key: String,
    /// Page display name.
    pub page_display_name: String,
    /// Project key.
    pub project_key: String,
    /// Project display name.
    pub project_display_name: String,
    /// Page type.
    pub page_type: Option<String>,
    /// Page prompt text.
    pub page_prompt: Option<String>,
    /// Page generated content for selected locale (v10.9: renamed from page_l10n).
    pub page_generated: Option<PageGeneratedData>,
    /// Blocks in order.
    pub blocks: Vec<BlockData>,
    /// Entities used by blocks.
    pub entities: Vec<EntityData>,
    /// SEO keywords connected to entities.
    pub seo_keywords: Vec<SeoKeywordData>,
}

/// Block data within page composition.
#[derive(Debug, Clone)]
pub struct BlockData {
    pub key: String,
    pub display_name: String,
    pub order: i64,
    pub block_type: Option<String>,
    pub prompt: Option<String>,
    pub rules: Option<String>,
    /// Block generated content (v10.9: renamed from l10n).
    pub generated: Option<BlockGeneratedData>,
}

/// Block generated data (v10.9.0: renamed from BlockGeneratedData).
#[derive(Debug, Clone)]
pub struct BlockGeneratedData {
    pub locale: String,
    pub content_preview: String,
}

/// Page generated data (v10.9.0: renamed from PageGeneratedData).
#[derive(Debug, Clone)]
pub struct PageGeneratedData {
    pub locale: String,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub meta_description: Option<String>,
}

/// Entity data in page composition.
#[derive(Debug, Clone)]
pub struct EntityData {
    pub key: String,
    pub display_name: String,
    /// Entity content for locale (v10.9: renamed from l10n).
    pub content: Option<EntityContentData>,
    pub connected_blocks: Vec<String>,
}

/// Entity content data (v10.9.0: renamed from EntityContentData).
#[derive(Debug, Clone)]
pub struct EntityContentData {
    pub locale: String,
    pub name: Option<String>,
    pub description_preview: Option<String>,
}

/// SEO keyword data.
#[derive(Debug, Clone)]
pub struct SeoKeywordData {
    pub keyword: String,
    pub volume: Option<i64>,
    pub connected_entities: Vec<String>,
}
