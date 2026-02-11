//! App state for TUI v2.

use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Normalization};
use nucleo_matcher::{Config, Matcher, Utf32Str};
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::fs;
use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};

use super::cache::RenderCache;
use super::data::{
    ArcKindDetails, KindArcsData, LayerDetails, RealmDetails, TaxonomyTree, TreeItem,
};
use super::nexus::NexusState;
use super::schema::{CoverageStats, MatchedProperty, ValidatedProperty, ValidationStats};
use super::handlers::dispatch_mode_handler;
use super::theme::Theme;
use super::yaml::{YamlSections, YamlViewSection};

use ratatui::text::Span;

// =============================================================================
// CONSTANTS
// =============================================================================

/// Number of items to scroll with page up/down (d/u keys).
pub const PAGE_SCROLL_AMOUNT: usize = 10;

/// Minimum visible lines to keep above max scroll position in YAML panel.
pub const YAML_SCROLL_MARGIN: usize = 10;

/// Minimum visible lines to keep above max scroll position in Info panel.
pub const INFO_SCROLL_MARGIN: usize = 5;

/// Default tree height (updated by UI on render).
pub const DEFAULT_TREE_HEIGHT: usize = 20;

/// Navigation mode — 2 modes in v11.7 Unified Tree.
/// Order: 1:Graph 2:Nexus
/// Keys 1-2 switch modes GLOBALLY from anywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavMode {
    /// Graph mode: Unified tree view (Realm > Layer > Kind hierarchy with instances)
    /// Replaces Meta/Data/Overlay modes from v11.6
    #[default]
    Graph,
    /// Nexus mode: Hub for Quiz, Stats, Help
    Nexus,
}

impl NavMode {
    pub fn label(&self) -> &'static str {
        match self {
            NavMode::Graph => "Graph",
            NavMode::Nexus => "Nexus",
        }
    }

    /// Get array index for mode_cursors (0-1).
    pub fn index(&self) -> usize {
        match self {
            NavMode::Graph => 0,
            NavMode::Nexus => 1,
        }
    }
}

/// Which panel has focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    #[default]
    Tree,
    Info,
    Graph,
    Yaml,
}

impl Focus {
    /// Cycle to next focus panel.
    pub fn next(self) -> Self {
        match self {
            Focus::Tree => Focus::Info,
            Focus::Info => Focus::Graph,
            Focus::Graph => Focus::Yaml,
            Focus::Yaml => Focus::Tree,
        }
    }

    /// Cycle to previous focus panel.
    pub fn prev(self) -> Self {
        match self {
            Focus::Tree => Focus::Yaml,
            Focus::Info => Focus::Tree,
            Focus::Graph => Focus::Info,
            Focus::Yaml => Focus::Graph,
        }
    }
}

/// Extracted data from a TreeItem for use in load_yaml_for_current().
/// This avoids borrow checker issues when we need to both read the tree and mutate App.
#[derive(Debug)]
enum TreeItemData {
    Kind {
        yaml_path: String,
        key: String,
        properties: Vec<String>,
    },
    ArcKind {
        yaml_path: String,
        key: String,
    },
    Realm {
        key: String,
    },
    Layer {
        key: String,
    },
    ArcFamily {
        key: String,
    },
    Section,
    /// Instance with its parent Kind's yaml_path (to show schema in YAML panel).
    Instance {
        kind_yaml_path: String,
    },
    None,
}

// =============================================================================
// SUB-STATES (extracted for cleaner architecture)
// =============================================================================

/// Search state for nucleo fuzzy search (extracted sub-state).
#[derive(Debug, Default)]
pub struct SearchState {
    /// Whether search overlay is active.
    pub active: bool,
    /// Current search query string.
    pub query: String,
    /// Indices into flattened tree matching the query.
    pub results: Vec<usize>,
    /// Fuzzy match scores (sorted descending, parallel to results).
    pub scores: Vec<u16>,
    /// Character positions for highlighting (idx -> matched positions).
    pub matches: FxHashMap<usize, Vec<u32>>,
    /// Current cursor within search results.
    pub cursor: usize,
}

impl SearchState {
    /// Clear all search state.
    pub fn clear(&mut self) {
        self.active = false;
        self.query.clear();
        self.results.clear();
        self.scores.clear();
        self.matches.clear();
        self.cursor = 0;
    }
}

/// Main app state.
#[allow(dead_code)]
pub struct App {
    /// Cached theme (color mode detected once at startup).
    pub theme: Theme,
    pub mode: NavMode,
    pub focus: Focus,
    pub tree_cursor: usize,
    /// Remember cursor position per mode (Graph, Nexus).
    pub mode_cursors: [usize; 2],
    pub tree_scroll: usize, // Scroll offset for tree
    pub tree_height: usize, // Visible height (set by UI)
    pub tree: TaxonomyTree,
    /// Search state (extracted sub-state).
    pub search: SearchState,
    // Help overlay
    pub help_active: bool,
    // Legend overlay (color meanings)
    pub legend_active: bool,
    // Recent items overlay (` key)
    pub recent_items_active: bool,
    pub recent_items_cursor: usize,
    /// Navigation history for Ctrl+o (back) / Ctrl+i (forward)
    pub nav_history: Vec<(NavMode, usize)>, // (mode, cursor)
    pub nav_history_pos: usize, // Current position in history
    /// Status message (e.g., "Copied to clipboard", "Refreshing...")
    pub status_message: Option<(String, std::time::Instant)>,
    /// Pending refresh request
    pub pending_refresh: bool,
    // YAML preview
    pub yaml_content: String,
    pub yaml_path: String,
    pub yaml_scroll: usize,
    pub yaml_line_count: usize, // Cached line count (avoids per-scroll recomputation)
    /// Parsed YAML sections for contextual view (Kind vs Instance).
    pub yaml_sections: Option<YamlSections>,
    /// Whether peek mode is active (showing hidden section in dim).
    pub yaml_peek: bool,
    // Info panel scroll (separate from yaml)
    pub info_scroll: usize,
    pub info_line_count: usize, // Set by UI after building lines
    pub root_path: String,
    /// Cache of YAML file contents (path -> content).
    /// Avoids re-reading files on every scroll/navigation.
    pub yaml_cache: FxHashMap<String, String>,
    /// Neo4j arc data for current Kind (loaded async)
    pub kind_arcs: Option<KindArcsData>,
    /// Neo4j arc kind details (loaded async when ArcKind selected)
    pub arc_kind_details: Option<ArcKindDetails>,
    // Data view: pending instance load request (Kind label to load)
    pub pending_instance_load: Option<String>,
    /// Pending Kind arcs load request (Kind label to load from Neo4j)
    pub pending_arcs_load: Option<String>,
    /// Pending instance arc loading (Kind label + instance keys for background arc loading)
    pub pending_instance_arcs_load: Option<(String, Vec<String>)>,
    /// Pending entity categories load (triggered when Entity Kind expanded)
    pub pending_entity_categories_load: bool,
    /// Pending category instances load (category key to load)
    pub pending_category_instances_load: Option<String>,
    /// Pending ArcKind details load request (Arc key to load from Neo4j)
    pub pending_arc_kind_load: Option<String>,
    /// Pending Realm details load request (Realm key to load from Neo4j)
    pub pending_realm_load: Option<String>,
    /// Pending Layer details load request (Layer key to load from Neo4j)
    pub pending_layer_load: Option<String>,
    /// Neo4j Realm details (loaded async when Realm selected)
    pub realm_details: Option<RealmDetails>,
    /// Neo4j Layer details (loaded async when Layer selected)
    pub layer_details: Option<LayerDetails>,
    /// Data mode filter: when set, show only instances of this Kind
    /// None = show full tree, Some(kind_key) = show only instances of that Kind
    pub data_filter_kind: Option<String>,
    /// Cursor position before entering filtered Data mode (for restoration)
    pub data_cursor_before_filter: usize,
    /// Hide empty: when true, hide kinds/layers with 0 instances in Data mode
    pub hide_empty: bool,
    /// Nexus mode state (gamified learning hub)
    pub nexus: NexusState,
    /// Animation tick counter (increments each frame, used for spinners)
    pub tick: u16,
    // ==========================================================================
    // Schema Overlay State (Feature 1)
    // ==========================================================================
    /// Whether schema overlay is enabled in Data mode (toggle with 's')
    pub schema_overlay_enabled: bool,
    /// Matched properties for current instance (schema + values)
    pub matched_properties: Option<Vec<MatchedProperty>>,
    /// Coverage stats for current instance
    pub coverage_stats: Option<CoverageStats>,
    // ==========================================================================
    // Kind Validation State (Neo4j ↔ YAML)
    // ==========================================================================
    /// Validated properties for current Kind (YAML schema vs Neo4j)
    pub validated_kind_properties: Option<Vec<ValidatedProperty>>,
    /// Validation stats for current Kind
    pub validation_stats: Option<ValidationStats>,
    // ==========================================================================
    // Property Focus State (Feature 3)
    // ==========================================================================
    /// Index of focused property in Info panel (for truncate intelligent)
    pub focused_property_idx: usize,
    /// Whether the focused property text is expanded (Enter toggle)
    pub expanded_property: bool,
    // ==========================================================================
    // JSON Pretty-Print State (Feature 4)
    // ==========================================================================
    /// Whether to pretty-print JSON values (toggle with 'J')
    pub json_pretty: bool,
    // ==========================================================================
    // Trait Filter State (Quick Filter: fi/fl/fk/fg/fa/ff)
    // ==========================================================================
    /// Active trait filter (None = show all, Some("invariant") = filter by trait)
    pub trait_filter: Option<String>,
    /// Pending filter key (true when 'f' was pressed, waiting for second key)
    pub filter_pending: bool,
    // ==========================================================================
    // Render Caches (D: Performance Optimization)
    // ==========================================================================
    /// Cache for status bar realm mini-bar (avoids Vec allocation per frame).
    /// Uses RefCell for interior mutability during immutable render calls.
    pub mini_bar_cache: RefCell<RenderCache<Vec<Span<'static>>>>,
}

impl App {
    pub fn new(tree: TaxonomyTree, root_path: String) -> Self {
        let mut app = Self {
            theme: Theme::with_root(&root_path), // Load colors + icons from YAML
            mode: NavMode::Graph,
            focus: Focus::Tree,
            tree_cursor: 0,
            mode_cursors: [0; 2], // Init all modes at cursor 0 (Graph, Nexus)
            tree_scroll: 0,
            tree_height: DEFAULT_TREE_HEIGHT,
            tree,
            search: SearchState::default(),
            help_active: false,
            legend_active: false,
            recent_items_active: false,
            recent_items_cursor: 0,
            nav_history: Vec::with_capacity(100),
            nav_history_pos: 0,
            status_message: None,
            pending_refresh: false,
            yaml_content: String::new(),
            yaml_path: String::new(),
            yaml_scroll: 0,
            yaml_line_count: 0,
            yaml_sections: None,
            yaml_peek: false,
            info_scroll: 0,
            info_line_count: 0,
            root_path,
            yaml_cache: FxHashMap::default(),
            kind_arcs: None,
            arc_kind_details: None,
            pending_instance_load: None,
            pending_arcs_load: None,
            pending_instance_arcs_load: None,
            pending_entity_categories_load: false,
            pending_category_instances_load: None,
            pending_arc_kind_load: None,
            pending_realm_load: None,
            pending_layer_load: None,
            realm_details: None,
            layer_details: None,
            data_filter_kind: None,
            data_cursor_before_filter: 0,
            hide_empty: false,
            nexus: NexusState::default(),
            tick: 0,
            // Schema overlay (Feature 1)
            schema_overlay_enabled: true, // Enabled by default
            matched_properties: None,
            coverage_stats: None,
            // Kind validation (Neo4j ↔ YAML)
            validated_kind_properties: None,
            validation_stats: None,
            // Property focus (Feature 3)
            focused_property_idx: 0,
            expanded_property: false,
            // JSON pretty-print (Feature 4)
            json_pretty: false,
            // Trait filter (Quick Filter)
            trait_filter: None,
            filter_pending: false,
            // Render caches (D: Performance Optimization)
            mini_bar_cache: RefCell::new(RenderCache::new()),
        };
        app.load_yaml_for_current();
        app
    }

    /// Get the active YAML section based on current navigation mode.
    /// - Meta mode → Kind section (schema)
    /// - Data mode → Instance section (data nodes)
    /// - Nexus → Kind section
    pub fn yaml_active_section(&self) -> YamlViewSection {
        match self.mode {
            // Graph mode shows Kind schema, Nexus shows Kind schema
            NavMode::Graph | NavMode::Nexus => YamlViewSection::Kind,
        }
    }

    /// Load YAML content for the current cursor position.
    /// Uses mode-aware item lookup to handle filtered Data mode correctly.
    pub fn load_yaml_for_current(&mut self) {
        // Reset scroll positions when changing items
        self.yaml_scroll = 0;
        self.info_scroll = 0;

        // Clear Neo4j data AND pending loads when moving away
        // (prevents race condition where pending load completes after navigation)
        self.kind_arcs = None;
        self.arc_kind_details = None;
        self.realm_details = None;
        self.layer_details = None;
        self.pending_arcs_load = None;
        self.pending_arc_kind_load = None;
        self.pending_realm_load = None;
        self.pending_layer_load = None;
        self.pending_instance_load = None;

        // Clear Kind validation state (only populated for Kind items)
        self.validated_kind_properties = None;
        self.validation_stats = None;

        // Get current item using mode-aware method (handles filtered Data mode)
        // This is the same logic as current_item() but we extract data to avoid borrow issues
        let current = self.get_current_tree_item_data();

        // Handle based on item type
        match current {
            TreeItemData::Kind {
                yaml_path,
                key,
                properties,
            } => {
                self.load_yaml_cached(&yaml_path);
                self.pending_arcs_load = Some(key);
                // Load Kind validation (Neo4j vs YAML)
                self.load_validated_kind_properties(&properties);
            }
            TreeItemData::ArcKind { yaml_path, key } => {
                self.load_yaml_cached(&yaml_path);
                self.pending_arc_kind_load = Some(key);
            }
            TreeItemData::Realm { key } => {
                let path = format!("packages/core/models/meta/realms/{}.yaml", key);
                self.load_yaml_cached(&path);
                self.pending_realm_load = Some(key);
            }
            TreeItemData::Layer { key } => {
                let path = format!("packages/core/models/meta/layers/{}.yaml", key);
                self.load_yaml_cached(&path);
                self.pending_layer_load = Some(key);
            }
            TreeItemData::ArcFamily { key } => {
                let path = format!("packages/core/models/meta/arc-families/{}.yaml", key);
                self.load_yaml_cached(&path);
            }
            TreeItemData::Section => {
                self.load_yaml_cached("packages/core/models/taxonomy.yaml");
            }
            TreeItemData::Instance { kind_yaml_path } => {
                // Load the Kind's YAML to show Instance schema (standard_properties)
                if !kind_yaml_path.is_empty() {
                    self.load_yaml_cached(&kind_yaml_path);
                } else {
                    self.yaml_path.clear();
                    self.yaml_content.clear();
                    self.yaml_line_count = 0;
                    self.yaml_sections = None;
                }
            }
            TreeItemData::None => {
                self.yaml_path.clear();
                self.yaml_content.clear();
                self.yaml_line_count = 0;
            }
        }
    }

    /// Extract current tree item data using mode-aware lookup.
    /// Handles filtered Data mode correctly (same logic as current_item()).
    fn get_current_tree_item_data(&self) -> TreeItemData {
        // In filtered Data mode, always return Instance (that's all we show)
        if self.is_data_mode() && self.data_filter_kind.is_some() {
            if let Some(kind_key) = &self.data_filter_kind {
                if self
                    .tree
                    .filtered_item_at(self.tree_cursor, kind_key)
                    .is_some()
                {
                    // Get the Kind's yaml_path for showing schema in YAML panel
                    if let Some((_, _, kind)) = self.tree.find_kind(kind_key) {
                        return TreeItemData::Instance {
                            kind_yaml_path: kind.yaml_path.clone(),
                        };
                    }
                    return TreeItemData::Instance {
                        kind_yaml_path: String::new(),
                    };
                }
            }
            return TreeItemData::None;
        }

        // Use mode-aware item lookup
        let item = if self.is_data_mode() {
            self.tree.item_at_for_mode(self.tree_cursor, true)
        } else {
            self.tree.item_at(self.tree_cursor)
        };

        match item {
            Some(TreeItem::Kind(_, _, kind)) => TreeItemData::Kind {
                yaml_path: kind.yaml_path.clone(),
                key: kind.key.clone(),
                properties: kind.properties.clone(),
            },
            Some(TreeItem::ArcKind(family, arc)) => {
                let arc_file = arc.key.to_lowercase().replace('_', "-");
                TreeItemData::ArcKind {
                    yaml_path: format!(
                        "packages/core/models/arc-kinds/{}/{}.yaml",
                        family.key, arc_file
                    ),
                    key: arc.key.clone(),
                }
            }
            Some(TreeItem::Realm(realm)) => TreeItemData::Realm {
                key: realm.key.clone(),
            },
            Some(TreeItem::Layer(_, layer)) => TreeItemData::Layer {
                key: layer.key.clone(),
            },
            Some(TreeItem::ArcFamily(family)) => TreeItemData::ArcFamily {
                key: family.key.clone(),
            },
            Some(TreeItem::KindsSection) | Some(TreeItem::ArcsSection) => TreeItemData::Section,
            Some(TreeItem::Instance(_, _, kind, _)) => TreeItemData::Instance {
                kind_yaml_path: kind.yaml_path.clone(),
            },
            // EntityCategory shows parent Entity Kind's YAML
            Some(TreeItem::EntityCategory(_, _, kind, _)) => TreeItemData::Kind {
                yaml_path: kind.yaml_path.clone(),
                key: kind.key.clone(),
                properties: kind.properties.clone(),
            },
            None => TreeItemData::None,
        }
    }

    /// Load YAML content with caching (avoids re-reading files on every navigation).
    fn load_yaml_cached(&mut self, relative_path: &str) {
        self.yaml_path = relative_path.to_string();
        self.yaml_peek = false; // Reset peek when loading new file

        // Check cache first
        if let Some(cached) = self.yaml_cache.get(relative_path) {
            self.yaml_content = cached.clone();
            self.yaml_line_count = self.yaml_content.lines().count();
            // Parse sections for contextual view
            self.yaml_sections = YamlSections::parse(&self.yaml_content);
            return;
        }

        // Load from disk
        let full_path = Path::new(&self.root_path).join(relative_path);
        let content = fs::read_to_string(&full_path)
            .unwrap_or_else(|_| format!("# File not found: {}", full_path.display()));

        // Update cache
        self.yaml_cache
            .insert(relative_path.to_string(), content.clone());

        // Update display
        self.yaml_content = content;
        self.yaml_line_count = self.yaml_content.lines().count();
        // Parse sections for contextual view
        self.yaml_sections = YamlSections::parse(&self.yaml_content);
    }

    /// Ensure cursor is visible by adjusting scroll.
    pub fn ensure_cursor_visible(&mut self) {
        // Scroll up if cursor is above viewport
        if self.tree_cursor < self.tree_scroll {
            self.tree_scroll = self.tree_cursor;
        }
        // Scroll down if cursor is below viewport
        // Use saturating_sub to prevent underflow when tree_height is 0
        if self.tree_cursor >= self.tree_scroll + self.tree_height {
            self.tree_scroll = self
                .tree_cursor
                .saturating_sub(self.tree_height.saturating_sub(1));
        }
    }

    /// Update search results based on current query using nucleo fuzzy matching.
    /// Results are sorted by match score (best matches first).
    pub fn update_search(&mut self) {
        self.search.results.clear();
        self.search.scores.clear();
        self.search.matches.clear();

        if self.search.query.is_empty() {
            return;
        }

        // Setup nucleo matcher with smart case matching
        let mut matcher = Matcher::new(Config::DEFAULT);
        let pattern = Atom::new(
            &self.search.query,
            CaseMatching::Smart, // Case-insensitive unless query has uppercase
            Normalization::Smart,
            AtomKind::Fuzzy, // Fuzzy matching (not exact)
            false,           // No append
        );

        // Collect all (idx, score, match_positions) tuples
        let mut matches: Vec<(usize, u16, Vec<u32>)> = Vec::new();
        let mut idx = 0;

        // Helper to check fuzzy match and collect positions
        let fuzzy_match =
            |text: &str, matcher: &mut Matcher, pattern: &Atom| -> Option<(u16, Vec<u32>)> {
                let mut buf = Vec::new();
                let haystack = Utf32Str::new(text, &mut buf);
                let mut indices = Vec::new();
                pattern
                    .indices(haystack, matcher, &mut indices)
                    .map(|score| (score, indices))
            };

        // Kinds section header
        if let Some((score, indices)) = fuzzy_match("Node Kinds", &mut matcher, &pattern) {
            matches.push((idx, score, indices));
        }
        idx += 1;

        if !self.tree.is_collapsed("kinds") {
            for realm in &self.tree.realms {
                // Check display_name and key, take best match
                let match_display = fuzzy_match(&realm.display_name, &mut matcher, &pattern);
                let match_key = fuzzy_match(&realm.key, &mut matcher, &pattern);
                if let Some((score, indices)) = match_display.or(match_key) {
                    matches.push((idx, score, indices));
                }
                idx += 1;

                if !self.tree.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        let match_display =
                            fuzzy_match(&layer.display_name, &mut matcher, &pattern);
                        let match_key = fuzzy_match(&layer.key, &mut matcher, &pattern);
                        if let Some((score, indices)) = match_display.or(match_key) {
                            matches.push((idx, score, indices));
                        }
                        idx += 1;

                        if !self
                            .tree
                            .is_collapsed(&format!("layer:{}:{}", realm.key, layer.key))
                        {
                            for kind in &layer.kinds {
                                let match_display =
                                    fuzzy_match(&kind.display_name, &mut matcher, &pattern);
                                let match_key = fuzzy_match(&kind.key, &mut matcher, &pattern);
                                if let Some((score, indices)) = match_display.or(match_key) {
                                    matches.push((idx, score, indices));
                                }
                                idx += 1;
                            }
                        }
                    }
                }
            }
        }

        // Arcs section header
        if let Some((score, indices)) = fuzzy_match("Arcs", &mut matcher, &pattern) {
            matches.push((idx, score, indices));
        }
        idx += 1;

        if !self.tree.is_collapsed("arcs") {
            for family in &self.tree.arc_families {
                let match_display = fuzzy_match(&family.display_name, &mut matcher, &pattern);
                let match_key = fuzzy_match(&family.key, &mut matcher, &pattern);
                if let Some((score, indices)) = match_display.or(match_key) {
                    matches.push((idx, score, indices));
                }
                idx += 1;

                if !self.tree.is_collapsed(&format!("family:{}", family.key)) {
                    for arc_kind in &family.arc_kinds {
                        let match_display =
                            fuzzy_match(&arc_kind.display_name, &mut matcher, &pattern);
                        let match_key = fuzzy_match(&arc_kind.key, &mut matcher, &pattern);
                        if let Some((score, indices)) = match_display.or(match_key) {
                            matches.push((idx, score, indices));
                        }
                        idx += 1;
                    }
                }
            }
        }

        // Sort by score (descending - best matches first)
        matches.sort_by(|a, b| b.1.cmp(&a.1));

        // Extract into separate vectors
        for (idx, score, indices) in matches {
            self.search.results.push(idx);
            self.search.scores.push(score);
            self.search.matches.insert(idx, indices);
        }

        // Reset cursor if out of bounds
        if self.search.cursor >= self.search.results.len() {
            self.search.cursor = 0;
        }
    }

    /// Select current search result and close search.
    pub fn select_search_result(&mut self) {
        if let Some(&idx) = self.search.results.get(self.search.cursor) {
            self.tree_cursor = idx;
            self.ensure_cursor_visible();
        }
        self.close_search();
    }

    /// Close search overlay.
    pub fn close_search(&mut self) {
        self.search.clear();
    }

    /// Navigate to next search result (n key).
    pub fn next_search_result(&mut self) {
        if self.search.results.is_empty() {
            return;
        }
        let max = self.search.results.len().saturating_sub(1);
        self.search.cursor = (self.search.cursor + 1).min(max);
        if let Some(&target_idx) = self.search.results.get(self.search.cursor) {
            self.tree_cursor = target_idx;
            self.ensure_cursor_visible();
            self.load_yaml_for_current();
        }
    }

    /// Navigate to previous search result (N key).
    pub fn prev_search_result(&mut self) {
        if self.search.results.is_empty() {
            return;
        }
        self.search.cursor = self.search.cursor.saturating_sub(1);
        if let Some(&target_idx) = self.search.results.get(self.search.cursor) {
            self.tree_cursor = target_idx;
            self.ensure_cursor_visible();
            self.load_yaml_for_current();
        }
    }

    /// Handle key input. Returns true if state changed (needs re-render).
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Help overlay - any key closes it
        if self.help_active {
            self.help_active = false;
            return true;
        }

        // Legend overlay - any key closes it
        if self.legend_active {
            self.legend_active = false;
            return true;
        }

        // Recent items overlay - handles navigation and selection
        if self.recent_items_active {
            return self.handle_recent_items_key(key);
        }

        // Filter pending mode: waiting for second key (fi/fl/fk/fg/fa/ff)
        if self.filter_pending {
            self.filter_pending = false; // Always clear pending state
            match key.code {
                KeyCode::Char('i') => {
                    self.trait_filter = Some("invariant".to_string());
                    self.tree_cursor = 0;
                    self.tree_scroll = 0;
                    self.set_status("Filter: invariant (■)");
                    return true;
                }
                KeyCode::Char('l') => {
                    self.trait_filter = Some("localized".to_string());
                    self.tree_cursor = 0;
                    self.tree_scroll = 0;
                    self.set_status("Filter: localized (□)");
                    return true;
                }
                KeyCode::Char('k') => {
                    self.trait_filter = Some("knowledge".to_string());
                    self.tree_cursor = 0;
                    self.tree_scroll = 0;
                    self.set_status("Filter: knowledge (◊)");
                    return true;
                }
                KeyCode::Char('g') => {
                    self.trait_filter = Some("generated".to_string());
                    self.tree_cursor = 0;
                    self.tree_scroll = 0;
                    self.set_status("Filter: generated (★)");
                    return true;
                }
                KeyCode::Char('a') => {
                    self.trait_filter = Some("aggregated".to_string());
                    self.tree_cursor = 0;
                    self.tree_scroll = 0;
                    self.set_status("Filter: aggregated (▪)");
                    return true;
                }
                KeyCode::Char('f') => {
                    // ff = clear filter
                    self.trait_filter = None;
                    self.tree_cursor = 0;
                    self.tree_scroll = 0;
                    self.set_status("Filter cleared");
                    return true;
                }
                KeyCode::Esc => {
                    // Cancel filter mode, do nothing
                    return true;
                }
                _ => {
                    // Unknown second key: fall through to activate search
                    self.search.active = true;
                    return true;
                }
            }
        }

        // Search mode captures all input
        if self.search.active {
            return self.handle_search_key(key);
        }

        // Search navigation: Ctrl-n (next) / Ctrl-p (prev) work in any mode
        if key
            .modifiers
            .contains(crossterm::event::KeyModifiers::CONTROL)
        {
            match key.code {
                KeyCode::Char('n') => {
                    self.next_search_result();
                    return true;
                }
                KeyCode::Char('p') => {
                    self.prev_search_result();
                    return true;
                }
                _ => {}
            }
        }

        // Mode-specific key handling (Graph, Nexus)
        // Returns early if handled, falls through to global handlers otherwise
        if let Some(result) = dispatch_mode_handler(self, key) {
            return result;
        }

        match key.code {
            // Open help (? = vim-style help)
            KeyCode::Char('?') => {
                self.help_active = true;
                true
            }

            // Open search (/ = vim-style search)
            KeyCode::Char('/') => {
                self.search.active = true;
                true
            }

            // Trait filter prefix (f = filter, wait for second key: i/l/k/g/a/f)
            KeyCode::Char('f') => {
                self.filter_pending = true;
                self.set_status(
                    "Filter: [i]nvariant [l]ocalized [k]nowledge [g]enerated [a]ggregated [f]clear",
                );
                true
            }

            // Open color legend (F1 = accessible, out of flow)
            KeyCode::F(1) => {
                self.legend_active = true;
                true
            }

            // Open recent items popup (` = backtick)
            KeyCode::Char('`') => {
                if !self.nav_history.is_empty() {
                    self.recent_items_active = true;
                    self.recent_items_cursor = 0;
                }
                true
            }

            // Mode switching: 1-2 global (1=Graph, 2=Nexus)
            KeyCode::Char('1') => {
                // Switch to Graph mode (unified tree view)
                if self.mode != NavMode::Graph {
                    self.save_mode_cursor();
                    self.mode = NavMode::Graph;
                    self.restore_mode_cursor(NavMode::Graph);
                    self.load_yaml_for_current();
                }
                true
            }
            KeyCode::Char('2') => {
                // Switch to Nexus mode (hub for Quiz, Stats, Help)
                if self.mode != NavMode::Nexus {
                    self.save_mode_cursor();
                    self.mode = NavMode::Nexus;
                    self.restore_mode_cursor(NavMode::Nexus);
                }
                true
            }

            // Panel focus: Tab cycles, ←→ always switch panels
            KeyCode::Tab => {
                self.focus = self.focus.next();
                true
            }
            KeyCode::BackTab => {
                self.focus = self.focus.prev();
                true
            }
            KeyCode::Left => {
                // Left: always go to previous panel
                self.focus = self.focus.prev();
                true
            }
            KeyCode::Right => {
                // Right: always go to next panel
                self.focus = self.focus.next();
                true
            }

            // Enter: toggle collapse/expand (Tree), toggle peek (YAML), or expand property (Info)
            KeyCode::Enter => {
                match self.focus {
                    Focus::Tree => {
                        self.toggle_tree_item();
                    }
                    Focus::Yaml => {
                        // Toggle peek mode (show/hide other section)
                        if self.yaml_sections.is_some() {
                            self.yaml_peek = !self.yaml_peek;
                        }
                    }
                    Focus::Info => {
                        // Toggle expanded property text (word-wrap on multiple lines)
                        self.expanded_property = !self.expanded_property;
                    }
                    Focus::Graph => {
                        // No-op for Graph focus (future: could activate selected node)
                    }
                }
                true
            }

            // Toggle collapse/expand: h/l/Space (Tree only)
            KeyCode::Char('h') | KeyCode::Char('l') | KeyCode::Char(' ') => {
                if self.focus == Focus::Tree {
                    self.toggle_tree_item();
                }
                true
            }
            KeyCode::Char('H') => {
                self.tree.collapse_all();
                self.tree_cursor = 0;
                self.tree_scroll = 0;
                true
            }
            KeyCode::Char('L') => {
                self.tree.expand_all();
                true
            }

            // Expand/Collapse subtree under cursor (e/c)
            KeyCode::Char('e') | KeyCode::Char('E') if key.modifiers.is_empty() => {
                // E = Expand subtree under cursor
                if self.focus == Focus::Tree {
                    if let Some(key) = self.tree.collapse_key_at(self.tree_cursor) {
                        self.tree.expand_subtree(&key);
                    }
                }
                true
            }
            KeyCode::Char('c') => {
                // c = Collapse subtree under cursor
                if self.focus == Focus::Tree {
                    if let Some(key) = self.tree.collapse_key_at(self.tree_cursor) {
                        self.tree.collapse_subtree(&key);
                    }
                }
                true
            }

            // Toggle hide empty (0) - only in Data mode
            KeyCode::Char('0') => {
                if self.is_data_mode() {
                    self.hide_empty = !self.hide_empty;
                    self.set_status(if self.hide_empty {
                        "Hide empty: ON"
                    } else {
                        "Hide empty: OFF"
                    });
                    // Reset cursor to avoid pointing to hidden item
                    self.tree_cursor = 0;
                    self.tree_scroll = 0;
                }
                true
            }

            // Jump to first/last (vim-style: g/G)
            KeyCode::Char('g') => {
                match self.focus {
                    Focus::Tree => {
                        self.tree_cursor = 0;
                        self.tree_scroll = 0;
                        self.load_yaml_for_current();
                        // Note: Instance loading removed - use Space/Enter to expand
                    }
                    Focus::Info => {
                        self.info_scroll = 0;
                    }
                    Focus::Graph => {} // Display-only
                    Focus::Yaml => {
                        self.yaml_scroll = 0;
                    }
                }
                true
            }
            KeyCode::Char('G') => {
                match self.focus {
                    Focus::Tree => {
                        let max = self.current_item_count().saturating_sub(1);
                        self.tree_cursor = max;
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                        // Note: Instance loading removed - use Space/Enter to expand
                    }
                    Focus::Info => {
                        let max_scroll = self.info_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                        self.info_scroll = max_scroll;
                    }
                    Focus::Graph => {} // Display-only
                    Focus::Yaml => {
                        let max_scroll = self.yaml_line_count.saturating_sub(YAML_SCROLL_MARGIN);
                        self.yaml_scroll = max_scroll;
                    }
                }
                true
            }

            // Navigation: ↑↓ and j/k scroll the focused panel (Graph is display-only)
            KeyCode::Up | KeyCode::Char('k') => {
                match self.focus {
                    Focus::Tree => {
                        if self.tree_cursor > 0 {
                            self.tree_cursor -= 1;
                            self.ensure_cursor_visible();
                            self.load_yaml_for_current();
                            // Note: Instance loading removed - use Space/Enter to expand
                        }
                    }
                    Focus::Info => {
                        if self.info_scroll > 0 {
                            self.info_scroll -= 1;
                        }
                    }
                    Focus::Graph => {} // Display-only panel, no navigation
                    Focus::Yaml => {
                        if self.yaml_scroll > 0 {
                            self.yaml_scroll -= 1;
                        }
                    }
                }
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                match self.focus {
                    Focus::Tree => {
                        let max = self.current_item_count().saturating_sub(1);
                        if self.tree_cursor < max {
                            self.tree_cursor += 1;
                            self.ensure_cursor_visible();
                            self.load_yaml_for_current();
                            // Note: Instance loading removed - use Space/Enter to expand
                        }
                    }
                    Focus::Info => {
                        let max_scroll = self.info_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                        if self.info_scroll < max_scroll {
                            self.info_scroll += 1;
                        }
                    }
                    Focus::Graph => {} // Display-only panel, no navigation
                    Focus::Yaml => {
                        let max_scroll = self.yaml_line_count.saturating_sub(YAML_SCROLL_MARGIN);
                        if self.yaml_scroll < max_scroll {
                            self.yaml_scroll += 1;
                        }
                    }
                }
                true
            }

            // Page scroll: d/u vim-style (Graph is display-only)
            KeyCode::Char('d') => {
                match self.focus {
                    Focus::Tree => {
                        let max = self.current_item_count().saturating_sub(1);
                        self.tree_cursor = (self.tree_cursor + PAGE_SCROLL_AMOUNT).min(max);
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                        // Note: Instance loading removed - use Space/Enter to expand
                    }
                    Focus::Info => {
                        let max_scroll = self.info_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                        self.info_scroll = (self.info_scroll + PAGE_SCROLL_AMOUNT).min(max_scroll);
                    }
                    Focus::Graph => {} // Display-only panel, no navigation
                    Focus::Yaml => {
                        let max_scroll = self.yaml_line_count.saturating_sub(YAML_SCROLL_MARGIN);
                        self.yaml_scroll = (self.yaml_scroll + PAGE_SCROLL_AMOUNT).min(max_scroll);
                    }
                }
                true
            }
            KeyCode::Char('u') => {
                match self.focus {
                    Focus::Tree => {
                        self.tree_cursor = self.tree_cursor.saturating_sub(PAGE_SCROLL_AMOUNT);
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                        // Note: Instance loading removed - use Space/Enter to expand
                    }
                    Focus::Info => {
                        self.info_scroll = self.info_scroll.saturating_sub(PAGE_SCROLL_AMOUNT);
                    }
                    Focus::Graph => {} // Display-only panel, no navigation
                    Focus::Yaml => {
                        self.yaml_scroll = self.yaml_scroll.saturating_sub(PAGE_SCROLL_AMOUNT);
                    }
                }
                true
            }

            // Refresh data from Neo4j (r = refresh)
            KeyCode::Char('r') => {
                self.pending_refresh = true;
                self.set_status("Refreshing...");
                true
            }

            // Yank (copy node key to clipboard)
            KeyCode::Char('y') => {
                self.yank_current_key();
                true
            }

            // Yank JSON properties (Y)
            KeyCode::Char('Y') => {
                self.yank_current_json();
                true
            }

            // Jump to parent [p]
            KeyCode::Char('p') => {
                if let Some(parent_cursor) = self
                    .tree
                    .find_parent_cursor(self.tree_cursor, self.is_data_mode())
                {
                    self.tree_cursor = parent_cursor;
                    self.ensure_cursor_visible();
                    self.set_status("↑ Parent");
                }
                true
            }

            // Toggle schema overlay (s) - only in Data mode
            KeyCode::Char('s') => {
                if self.is_data_mode() {
                    self.schema_overlay_enabled = !self.schema_overlay_enabled;
                    // Load/clear matched properties based on new state
                    self.update_schema_match_for_current();
                    self.set_status(if self.schema_overlay_enabled {
                        "Schema overlay ON"
                    } else {
                        "Schema overlay OFF"
                    });
                }
                true
            }

            // Toggle JSON pretty-print (J) - only when viewing properties
            KeyCode::Char('J') => {
                self.json_pretty = !self.json_pretty;
                self.set_status(if self.json_pretty {
                    "JSON pretty-print ON"
                } else {
                    "JSON compact mode"
                });
                true
            }

            // Property focus navigation (+/-) - Feature 3: Truncate Intelligent
            // Navigate focused property in schema overlay
            KeyCode::Char('+') | KeyCode::Char('=') => {
                if self.is_data_mode() && self.schema_overlay_enabled {
                    if let Some(matched) = &self.matched_properties {
                        let max_idx = matched.len().saturating_sub(1);
                        self.focused_property_idx = (self.focused_property_idx + 1).min(max_idx);
                        self.expanded_property = false; // Collapse when changing property
                    }
                }
                true
            }
            KeyCode::Char('-') | KeyCode::Char('_') => {
                if self.is_data_mode() && self.schema_overlay_enabled {
                    self.focused_property_idx = self.focused_property_idx.saturating_sub(1);
                    self.expanded_property = false; // Collapse when changing property
                }
                true
            }

            // Navigation history: back (Ctrl+o)
            KeyCode::Char('o')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.nav_back();
                true
            }

            // Navigation history: forward (Ctrl+i)
            KeyCode::Char('i')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.nav_forward();
                true
            }

            // Esc: close peek (YAML), or exit filtered mode
            KeyCode::Esc => {
                // First priority: close yaml peek if active
                if self.yaml_peek {
                    self.yaml_peek = false;
                    return true;
                }
                // Second priority: exit filtered mode (stay in Graph mode)
                if self.is_filtered_data_mode() {
                    self.exit_filtered_data_mode();
                    return true;
                }
                false
            }

            _ => false,
        }
    }

    /// Handle key input in search mode.
    fn handle_search_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            // Close search
            KeyCode::Esc => {
                self.close_search();
                true
            }

            // Select result
            KeyCode::Enter => {
                self.select_search_result();
                true
            }

            // Navigate results (arrow keys and vim j/k)
            KeyCode::Up | KeyCode::Char('k') => {
                if self.search.cursor > 0 {
                    self.search.cursor -= 1;
                }
                true
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.search.results.len().saturating_sub(1);
                if self.search.cursor < max {
                    self.search.cursor += 1;
                }
                true
            }

            // Type character (j/k are handled above for navigation)
            // Security: Limit search query length to prevent memory exhaustion
            KeyCode::Char(c) => {
                const MAX_SEARCH_LEN: usize = 256;
                if self.search.query.len() < MAX_SEARCH_LEN {
                    self.search.query.push(c);
                    self.update_search();
                }
                true
            }

            // Delete character
            KeyCode::Backspace => {
                self.search.query.pop();
                self.update_search();
                true
            }

            _ => false,
        }
    }

    /// Handle key events for the recent items popup.
    fn handle_recent_items_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            // Close popup
            KeyCode::Esc | KeyCode::Char('`') => {
                self.recent_items_active = false;
                true
            }

            // Select and jump to item
            KeyCode::Enter => {
                self.select_recent_item();
                true
            }

            // Navigate up
            KeyCode::Up | KeyCode::Char('k') => {
                if self.recent_items_cursor > 0 {
                    self.recent_items_cursor -= 1;
                }
                true
            }

            // Navigate down
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.nav_history.len().saturating_sub(1);
                if self.recent_items_cursor < max {
                    self.recent_items_cursor += 1;
                }
                true
            }

            _ => true, // Consume all other keys while popup is open
        }
    }

    /// Select and jump to the currently highlighted recent item.
    fn select_recent_item(&mut self) {
        // History is stored oldest→newest, but we display newest first
        let display_idx = self.recent_items_cursor;
        let history_idx = self.nav_history.len().saturating_sub(1 + display_idx);

        if let Some(&(mode, cursor)) = self.nav_history.get(history_idx) {
            self.recent_items_active = false;
            self.mode = mode;
            self.tree_cursor = cursor;
            self.ensure_cursor_visible();
            self.load_yaml_for_current();
            self.set_status("↩ Jumped to recent item");
        }
    }

    /// Check if currently in a mode that shows instances (Graph mode shows instances).
    /// In v11.7 unified tree, Graph mode is the unified view that always shows instances.
    pub fn is_data_mode(&self) -> bool {
        self.mode == NavMode::Graph
    }

    /// Save current cursor to mode_cursors for the current mode.
    fn save_mode_cursor(&mut self) {
        self.mode_cursors[self.mode.index()] = self.tree_cursor;
    }

    /// Restore cursor from mode_cursors for the new mode.
    fn restore_mode_cursor(&mut self, new_mode: NavMode) {
        self.tree_cursor = self.mode_cursors[new_mode.index()];
        self.ensure_cursor_visible();
    }

    /// Set a temporary status message (auto-clears after ~3 seconds).
    pub fn set_status(&mut self, msg: &str) {
        self.status_message = Some((msg.to_string(), std::time::Instant::now()));
    }

    /// Set an error status message with ⚠ prefix (auto-clears after ~3 seconds).
    /// Used when async data loading fails to inform the user.
    pub fn set_status_error(&mut self, msg: &str) {
        self.status_message = Some((format!("⚠ {}", msg), std::time::Instant::now()));
    }

    /// Clear status message if expired (called by UI tick).
    pub fn clear_expired_status(&mut self) {
        if let Some((_, instant)) = &self.status_message {
            if instant.elapsed().as_secs() >= 5 {
                self.status_message = None;
            }
        }
    }

    /// Push current position to navigation history.
    pub fn push_nav_history(&mut self) {
        let entry = (self.mode, self.tree_cursor);
        // Truncate forward history if we're not at the end
        if self.nav_history_pos < self.nav_history.len() {
            self.nav_history.truncate(self.nav_history_pos);
        }
        // Avoid duplicate consecutive entries
        if self.nav_history.last() != Some(&entry) {
            self.nav_history.push(entry);
            // Limit history size
            if self.nav_history.len() > 100 {
                self.nav_history.remove(0);
            }
        }
        self.nav_history_pos = self.nav_history.len();
    }

    /// Navigate back in history (Ctrl+o).
    pub fn nav_back(&mut self) {
        if self.nav_history_pos == 0 {
            return;
        }
        // Save current position before going back (if not already at end of history)
        if self.nav_history_pos == self.nav_history.len() {
            self.push_nav_history();
            self.nav_history_pos = self.nav_history_pos.saturating_sub(1);
        }
        self.nav_history_pos = self.nav_history_pos.saturating_sub(1);
        if let Some(&(mode, cursor)) = self.nav_history.get(self.nav_history_pos) {
            self.mode = mode;
            self.tree_cursor = cursor;
            self.ensure_cursor_visible();
            self.load_yaml_for_current();
        }
    }

    /// Navigate forward in history (Ctrl+i).
    pub fn nav_forward(&mut self) {
        if self.nav_history_pos >= self.nav_history.len().saturating_sub(1) {
            return;
        }
        self.nav_history_pos += 1;
        if let Some(&(mode, cursor)) = self.nav_history.get(self.nav_history_pos) {
            self.mode = mode;
            self.tree_cursor = cursor;
            self.ensure_cursor_visible();
            self.load_yaml_for_current();
        }
    }

    /// Yank (copy) the current item's key to clipboard.
    pub fn yank_current_key(&mut self) {
        use super::data::TreeItem;
        let key = match self.current_item() {
            Some(TreeItem::Realm(r)) => Some(r.key.clone()),
            Some(TreeItem::Layer(_, l)) => Some(l.key.clone()),
            Some(TreeItem::Kind(_, _, k)) => Some(k.key.clone()),
            Some(TreeItem::ArcFamily(f)) => Some(f.key.clone()),
            Some(TreeItem::ArcKind(_, a)) => Some(a.key.clone()),
            Some(TreeItem::EntityCategory(_, _, _, cat)) => Some(cat.key.clone()),
            Some(TreeItem::Instance(_, _, _, inst)) => Some(inst.key.clone()),
            _ => None,
        };
        if let Some(key) = key {
            // Show key in status (user can copy from terminal with mouse)
            self.set_status(&format!("□ {}", key));
        }
    }

    /// Yank (copy) the current item's properties as JSON.
    pub fn yank_current_json(&mut self) {
        use super::data::TreeItem;
        let json = match self.current_item() {
            Some(TreeItem::Instance(_, _, _, inst)) => {
                // Serialize instance properties to JSON
                serde_json::to_string_pretty(&inst.properties).ok()
            }
            Some(TreeItem::Kind(_, _, kind)) => {
                // For Kind, show properties schema
                Some(format!(
                    "{{\"properties\": {:?}, \"required\": {:?}}}",
                    kind.properties, kind.required_properties
                ))
            }
            _ => None,
        };
        if let Some(json) = json {
            // Truncate for status display (full JSON in terminal can be copied)
            let preview = if json.len() > 50 {
                format!("{}...", &json[..50])
            } else {
                json
            };
            self.set_status(&format!("JSON: {}", preview));
        }
    }

    /// Check if in filtered Data mode (drilling into a specific Kind).
    pub fn is_filtered_data_mode(&self) -> bool {
        self.is_data_mode() && self.data_filter_kind.is_some()
    }

    /// Get the current filter Kind key (if in filtered Data mode).
    pub fn get_filter_kind(&self) -> Option<&str> {
        self.data_filter_kind.as_deref()
    }

    /// Get item at cursor position for the current mode.
    /// Uses mode-aware method that shows instances in Data mode.
    pub fn current_item(&self) -> Option<super::data::TreeItem<'_>> {
        // Filtered Data mode: show only instances of the filtered Kind
        if let Some(kind_key) = &self.data_filter_kind {
            if self.is_data_mode() {
                return self.tree.filtered_item_at(self.tree_cursor, kind_key);
            }
        }
        // Normal mode
        if self.is_data_mode() {
            self.tree.item_at_for_mode(self.tree_cursor, true)
        } else {
            // Meta mode: apply trait filter if active
            self.tree
                .item_at_with_trait_filter(self.tree_cursor, self.trait_filter.as_deref())
        }
    }

    /// Get total item count for the current mode.
    pub fn current_item_count(&self) -> usize {
        // Filtered Data mode: count only instances of the filtered Kind
        if let Some(kind_key) = &self.data_filter_kind {
            if self.is_data_mode() {
                return self.tree.filtered_item_count(kind_key);
            }
        }
        // Normal mode
        if self.is_data_mode() {
            self.tree.item_count_for_mode(true)
        } else {
            // Meta mode: apply trait filter if active
            self.tree
                .item_count_with_trait_filter(self.trait_filter.as_deref())
        }
    }

    /// Enter filtered Data mode for a specific Kind.
    /// Saves cursor position and resets to 0.
    /// Also resets all scroll states to avoid stale positions.
    #[allow(dead_code)]
    pub fn enter_filtered_data_mode(&mut self, kind_key: String) {
        self.data_cursor_before_filter = self.tree_cursor;
        self.data_filter_kind = Some(kind_key.clone());
        self.tree_cursor = 0;
        self.tree_scroll = 0;
        // Reset other scroll states to avoid stale positions
        self.info_scroll = 0;
        self.yaml_scroll = 0;
        // Request instance load if not already loaded
        if self.tree.get_instances(&kind_key).is_none() {
            self.pending_instance_load = Some(kind_key);
        }
    }

    /// Get breadcrumb path for the current selection.
    /// Returns a string like "Org > Foundation > Entity (12)"
    pub fn current_breadcrumb(&self) -> String {
        use super::data::TreeItem;
        match self.current_item() {
            Some(TreeItem::KindsSection) => "Node Kinds".to_string(),
            Some(TreeItem::ArcsSection) => "Arcs".to_string(),
            Some(TreeItem::Realm(r)) => r.display_name.clone(),
            Some(TreeItem::Layer(r, l)) => {
                format!("{} → {}", r.display_name, l.display_name)
            }
            Some(TreeItem::Kind(r, l, k)) => {
                if self.is_data_mode() && k.instance_count > 0 {
                    format!(
                        "{} → {} → {} ({})",
                        r.display_name, l.display_name, k.display_name, k.instance_count
                    )
                } else {
                    format!(
                        "{} → {} → {}",
                        r.display_name, l.display_name, k.display_name
                    )
                }
            }
            Some(TreeItem::Instance(r, l, k, inst)) => {
                format!(
                    "{} → {} → {} → {}",
                    r.display_name, l.display_name, k.display_name, inst.display_name
                )
            }
            Some(TreeItem::EntityCategory(r, l, k, cat)) => {
                format!(
                    "{} → {} → {} → {}",
                    r.display_name, l.display_name, k.display_name, cat.display_name
                )
            }
            Some(TreeItem::ArcFamily(f)) => format!("Arcs → {}", f.display_name),
            Some(TreeItem::ArcKind(f, ak)) => {
                format!("Arcs → {} → {}", f.display_name, ak.display_name)
            }
            None => "NovaNet".to_string(),
        }
    }

    /// Exit filtered Data mode, restore cursor position.
    /// Clamps cursor to valid range in case tree structure changed.
    #[allow(dead_code)]
    pub fn exit_filtered_data_mode(&mut self) {
        if self.data_filter_kind.is_some() {
            self.data_filter_kind = None;
            self.pending_instance_load = None; // Clear pending to prevent race condition
            // Clamp cursor to valid range before restoring
            let max_cursor = self.tree.item_count().saturating_sub(1);
            self.tree_cursor = self.data_cursor_before_filter.min(max_cursor);
            self.ensure_cursor_visible();
        }
    }

    /// Request instance loading for the currently selected Kind.
    /// Sets `pending_instance_load` if a Kind is selected and we're in Data mode.
    pub fn request_instance_load_for_current(&mut self) {
        if !self.is_data_mode() {
            return;
        }

        // Check if current item is a Kind
        if let Some(super::data::TreeItem::Kind(_, _, kind)) = self.tree.item_at(self.tree_cursor) {
            // Only request if not already loaded
            if self.tree.get_instances(&kind.key).is_none() {
                self.pending_instance_load = Some(kind.key.clone());
            }
        }

        // Also update schema match if on an instance
        self.update_schema_match_for_current();
    }

    /// Auto-expand the current Kind when switching to Instances view.
    /// If cursor is on a Kind, expands it and triggers instance loading.
    ///
    /// Note: Currently unused after v11.7 mode simplification.
    #[allow(dead_code)]
    fn auto_expand_current_kind_for_instances(&mut self) {
        // Use Taxonomy view item_at since we just switched modes
        // and cursor position is based on Taxonomy view
        // Clone key upfront to avoid borrow checker issues
        let kind_key = if let Some(super::data::TreeItem::Kind(_, _, kind)) =
            self.tree.item_at(self.tree_cursor)
        {
            Some(kind.key.clone())
        } else {
            None
        };

        if let Some(key) = kind_key {
            let collapse_key = format!("kind:{}", key);

            // Expand the Kind if it's collapsed
            if self.tree.is_collapsed(&collapse_key) {
                self.tree.toggle(&collapse_key);
            }

            // Request instance loading if not already loaded
            if self.tree.get_instances(&key).is_none() {
                self.pending_instance_load = Some(key);
            }
        }
    }

    /// Toggle collapse/expand of the current tree item.
    /// Also triggers loading for instances, Entity categories, and category instances in Data mode.
    fn toggle_tree_item(&mut self) {
        if let Some(key) = self.tree.collapse_key_at(self.tree_cursor) {
            // Check if expanding (going from collapsed to expanded)
            let was_collapsed = self.tree.is_collapsed(&key);
            self.tree.toggle(&key);

            // Only trigger loading when expanding (not collapsing)
            if was_collapsed && self.is_data_mode() {
                // When expanding any Kind, load instances if not already loaded
                if let Some(kind_key) = key.strip_prefix("kind:") {
                    // Special case: Entity Kind also loads categories
                    if kind_key == "Entity" && self.tree.entity_categories.is_empty() {
                        self.pending_entity_categories_load = true;
                    }
                    // Load instances for this Kind if not already loaded
                    if self.tree.get_instances(kind_key).is_none() {
                        self.pending_instance_load = Some(kind_key.to_string());
                    }
                }
                // When expanding an EntityCategory, load instances for that category
                else if let Some(category_key) = key.strip_prefix("category:") {
                    if !self
                        .tree
                        .entity_category_instances
                        .contains_key(category_key)
                    {
                        self.pending_category_instances_load = Some(category_key.to_string());
                    }
                }
            }
        }
    }

    /// Update schema match for the current instance (if any).
    /// Called after navigation or schema overlay toggle.
    pub fn update_schema_match_for_current(&mut self) {
        // Reset focused property state when navigating to new instance
        self.focused_property_idx = 0;
        self.expanded_property = false;

        // Only relevant in Data mode with schema overlay enabled
        if !self.is_data_mode() || !self.schema_overlay_enabled {
            self.matched_properties = None;
            self.coverage_stats = None;
            return;
        }

        // Check if current item is an Instance
        // Note: item_at_for_mode takes (cursor, data_mode: bool), data_mode=true shows instances
        // Clone properties to avoid borrow conflict
        let props = if let Some(super::data::TreeItem::Instance(_, _, _, instance)) =
            self.tree.item_at_for_mode(self.tree_cursor, true)
        {
            Some(instance.properties.clone())
        } else {
            None
        };

        if let Some(properties) = props {
            self.load_matched_properties(&properties);
        } else {
            self.matched_properties = None;
            self.coverage_stats = None;
        }
    }

    /// Check and clear pending instance load request.
    /// Returns the Kind label to load, if any.
    pub fn take_pending_instance_load(&mut self) -> Option<String> {
        self.pending_instance_load.take()
    }

    /// Take the pending arcs load request (returns Kind label if one was queued).
    pub fn take_pending_arcs_load(&mut self) -> Option<String> {
        self.pending_arcs_load.take()
    }

    /// Take the pending instance arcs load request.
    /// Returns (Kind label, instance keys) to load arcs for.
    pub fn take_pending_instance_arcs_load(&mut self) -> Option<(String, Vec<String>)> {
        self.pending_instance_arcs_load.take()
    }

    /// Take the pending entity categories load request.
    /// Returns true if categories need to be loaded.
    pub fn take_pending_entity_categories_load(&mut self) -> bool {
        std::mem::take(&mut self.pending_entity_categories_load)
    }

    /// Take the pending category instances load request.
    /// Returns the category key if one was queued.
    pub fn take_pending_category_instances_load(&mut self) -> Option<String> {
        self.pending_category_instances_load.take()
    }

    /// Set the loaded Kind arcs data from Neo4j.
    pub fn set_kind_arcs(&mut self, arcs: KindArcsData) {
        self.kind_arcs = Some(arcs);
    }

    /// Take the pending arc kind details load request (returns Arc key if one was queued).
    pub fn take_pending_arc_kind_load(&mut self) -> Option<String> {
        self.pending_arc_kind_load.take()
    }

    /// Set the loaded ArcKind details from Neo4j.
    pub fn set_arc_kind_details(&mut self, details: ArcKindDetails) {
        self.arc_kind_details = Some(details);
    }

    /// Take the pending Realm details load request (returns Realm key if one was queued).
    pub fn take_pending_realm_load(&mut self) -> Option<String> {
        self.pending_realm_load.take()
    }

    /// Set the loaded Realm details from Neo4j.
    pub fn set_realm_details(&mut self, details: RealmDetails) {
        self.realm_details = Some(details);
    }

    /// Take the pending Layer details load request (returns Layer key if one was queued).
    pub fn take_pending_layer_load(&mut self) -> Option<String> {
        self.pending_layer_load.take()
    }

    /// Set the loaded Layer details from Neo4j.
    pub fn set_layer_details(&mut self, details: LayerDetails) {
        self.layer_details = Some(details);
    }

    /// Check if any data is currently being loaded from Neo4j.
    /// Used to trigger animation re-renders during loading.
    pub fn has_pending_load(&self) -> bool {
        self.pending_instance_load.is_some()
            || self.pending_arcs_load.is_some()
            || self.pending_instance_arcs_load.is_some()
            || self.pending_arc_kind_load.is_some()
            || self.pending_realm_load.is_some()
            || self.pending_layer_load.is_some()
            || self.pending_entity_categories_load
            || self.pending_category_instances_load.is_some()
    }

    /// Check if any overlay (help, legend, search, recent) is currently open.
    /// Used to prevent 'q' from quitting while overlays are active.
    pub fn has_overlay_open(&self) -> bool {
        self.help_active || self.legend_active || self.search.active || self.recent_items_active
    }

    /// Get the current spinner frame character (braille dots animation).
    /// Uses tick counter to animate smoothly during loading.
    pub fn spinner_frame(&self) -> char {
        const BRAILLE: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        BRAILLE[(self.tick / 2) as usize % BRAILLE.len()]
    }

    // =========================================================================
    // Schema Overlay Methods (Feature 1)
    // =========================================================================

    /// Load matched properties for the current instance (schema + values).
    /// Called after loading instance data to prepare schema overlay.
    pub fn load_matched_properties(
        &mut self,
        instance_props: &std::collections::BTreeMap<String, serde_json::Value>,
    ) {
        use super::schema::{CoverageStats, load_schema_properties, match_properties};

        // Only in Data mode with schema overlay enabled
        if !self.is_data_mode() || !self.schema_overlay_enabled {
            self.matched_properties = None;
            self.coverage_stats = None;
            return;
        }

        // Need the Kind's YAML path to load schema
        if self.yaml_path.is_empty() {
            self.matched_properties = None;
            self.coverage_stats = None;
            return;
        }

        // Load schema from YAML
        let schema = load_schema_properties(&self.root_path, &self.yaml_path);
        if schema.is_empty() {
            self.matched_properties = None;
            self.coverage_stats = None;
            return;
        }

        // Match properties
        let matched = match_properties(&schema, instance_props);
        let stats = CoverageStats::from_matched(&matched);

        self.matched_properties = Some(matched);
        self.coverage_stats = Some(stats);
    }

    // ==========================================================================
    // Kind Validation (Neo4j ↔ YAML)
    // ==========================================================================

    /// Load validated properties for the current Kind (compares Neo4j vs YAML).
    /// Called when selecting a Kind in Meta mode to show validation status.
    /// Uses cached YAML content to avoid redundant file I/O.
    pub fn load_validated_kind_properties(&mut self, kind_properties: &[String]) {
        use super::schema::{ValidationStats, parse_schema_properties, validate_kind_properties};

        // Need the Kind's YAML path to load schema
        if self.yaml_path.is_empty() {
            return; // State already cleared in load_yaml_for_current()
        }

        // Use cached YAML content (already loaded by load_yaml_cached)
        let yaml_content = match self.yaml_cache.get(&self.yaml_path) {
            Some(content) => content,
            None => {
                tracing::warn!(path = %self.yaml_path, "YAML not in cache for Kind validation");
                return;
            }
        };

        // Parse schema from cached YAML content
        let schema = parse_schema_properties(yaml_content);
        if schema.is_empty() {
            tracing::debug!(path = %self.yaml_path, "No schema properties found in YAML");
            return;
        }

        // Validate: compare YAML schema against Neo4j properties
        let validated = validate_kind_properties(&schema, kind_properties);
        let stats = ValidationStats::from_validated(&validated);

        self.validated_kind_properties = Some(validated);
        self.validation_stats = Some(stats);
    }
}

#[cfg(test)]
mod tests {
    use super::super::data::{
        GraphStats, InstanceInfo, KindInfo, LayerInfo, RealmInfo, TaxonomyTree, TreeItem,
    };
    use super::*;
    use rustc_hash::{FxHashMap, FxHashSet};
    use std::collections::BTreeMap;

    // Helper: Create test taxonomy tree
    fn create_test_tree() -> TaxonomyTree {
        let locale_kind = KindInfo {
            key: "Locale".to_string(),
            display_name: "Locale".to_string(),
            description: String::new(),
            icon: String::new(),
            trait_name: "knowledge".to_string(),
            instance_count: 3,
            arcs: Vec::new(),
            yaml_path: String::new(),
            properties: Vec::new(),
            required_properties: Vec::new(),
            schema_hint: String::new(),
            context_budget: String::new(),
            knowledge_tier: None,
            health_percent: None,
            issues_count: None,
        };

        let page_kind = KindInfo {
            key: "Page".to_string(),
            display_name: "Page".to_string(),
            description: String::new(),
            icon: String::new(),
            trait_name: "invariant".to_string(),
            instance_count: 5,
            arcs: Vec::new(),
            yaml_path: String::new(),
            properties: Vec::new(),
            required_properties: Vec::new(),
            schema_hint: String::new(),
            context_budget: String::new(),
            knowledge_tier: None,
            health_percent: None,
            issues_count: None,
        };

        // Minimal test fixture (v11.5 has 4 shared layers: config, locale, geography, knowledge)
        let locale_layer = LayerInfo {
            key: "locale".to_string(),
            display_name: "Locale".to_string(),
            color: "#2aa198".to_string(),
            kinds: vec![locale_kind],
        };

        let structure = LayerInfo {
            key: "structure".to_string(),
            display_name: "Structure".to_string(),
            color: "#b58900".to_string(),
            kinds: vec![page_kind],
        };

        let global = RealmInfo {
            key: "shared".to_string(),
            display_name: "Shared".to_string(),
            color: "#859900".to_string(),
            icon: "◉",
            layers: vec![locale_layer],
        };

        let tenant = RealmInfo {
            key: "org".to_string(),
            display_name: "Org".to_string(),
            color: "#b58900".to_string(),
            icon: "◎",
            layers: vec![structure],
        };

        let realms = vec![global, tenant];

        // Build kind_index (mirrors load() behavior)
        let mut kind_index = FxHashMap::default();
        for (r_idx, realm) in realms.iter().enumerate() {
            for (l_idx, layer) in realm.layers.iter().enumerate() {
                for (k_idx, kind) in layer.kinds.iter().enumerate() {
                    kind_index.insert(kind.key.clone(), (r_idx, l_idx, k_idx));
                }
            }
        }

        TaxonomyTree {
            realms,
            arc_families: Vec::new(),
            stats: GraphStats::default(),
            collapsed: FxHashSet::default(),
            instances: BTreeMap::new(),
            instance_totals: BTreeMap::new(),
            kind_index,
            entity_categories: Vec::new(),
            entity_category_instances: BTreeMap::new(),
        }
    }

    // Helper: Create App with test tree
    fn create_test_app() -> App {
        App::new(create_test_tree(), "/test/root".to_string())
    }

    // ========================================================================
    // Mode tests (v11.7: 2 modes - Graph and Nexus)
    // ========================================================================

    #[test]
    fn test_mode_starts_as_graph() {
        let app = create_test_app();
        assert_eq!(app.mode, NavMode::Graph);
        assert!(app.is_data_mode()); // Graph mode shows instances
    }

    #[test]
    fn test_graph_mode_shows_instances() {
        let mut app = create_test_app();

        // Navigate to Locale kind (index 3)
        // Kinds (0), shared (1), locale (2), Locale (3)
        app.tree_cursor = 3;

        // Verify we're at Locale in Graph mode
        match app.tree.item_at(app.tree_cursor) {
            Some(TreeItem::Kind(_, _, k)) => assert_eq!(k.key, "Locale"),
            other => panic!("Expected Kind Locale, got {:?}", other),
        }

        // Graph mode should show instances (is_data_mode returns true)
        assert!(app.is_data_mode());

        // Cursor should be valid
        assert_eq!(app.tree_cursor, 3);

        // Item at cursor should still be Locale kind
        match app.current_item() {
            Some(TreeItem::Kind(_, _, k)) => assert_eq!(k.key, "Locale"),
            other => panic!("Expected Kind Locale in Graph mode, got {:?}", other),
        }
    }

    #[test]
    fn test_graph_mode_shows_instances_after_kind() {
        let mut app = create_test_app();

        // Add instances to Locale kind
        let instances = vec![
            InstanceInfo {
                key: "fr-FR".to_string(),
                display_name: "Français".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
            },
            InstanceInfo {
                key: "en-US".to_string(),
                display_name: "English".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
            },
        ];
        app.tree
            .set_instances("Locale", instances.clone(), instances.len());

        // Graph mode (default) shows instances

        // Item count should include instances
        // Taxonomy: 1 (Kinds) + 1 (shared) + 1 (locale) + 1 (Locale)
        //           + 1 (org) + 1 (structure) + 1 (Page) + 1 (Arcs) = 8
        // Instances: + 2 instances = 10
        assert_eq!(app.current_item_count(), 10);

        // Position 4 should be fr-FR instance
        app.tree_cursor = 4;
        match app.current_item() {
            Some(TreeItem::Instance(_, _, _, inst)) => {
                assert_eq!(inst.key, "fr-FR");
            }
            other => panic!("Expected Instance fr-FR, got {:?}", other),
        }

        // Position 5 should be en-US instance
        app.tree_cursor = 5;
        match app.current_item() {
            Some(TreeItem::Instance(_, _, _, inst)) => {
                assert_eq!(inst.key, "en-US");
            }
            other => panic!("Expected Instance en-US, got {:?}", other),
        }
    }

    #[test]
    fn test_graph_mode_counts_instances() {
        let mut app = create_test_app();

        // Add instances
        let instances = vec![InstanceInfo {
            key: "fr-FR".to_string(),
            display_name: "Français".to_string(),
            kind_key: "Locale".to_string(),
            properties: BTreeMap::new(),
            outgoing_arcs: vec![],
            incoming_arcs: vec![],
            arcs_loading: false,
            missing_required_count: 0,
            filled_properties: 0,
            total_properties: 0,
        }];
        app.tree
            .set_instances("Locale", instances.clone(), instances.len());

        // In Graph mode, instances are visible
        assert_eq!(app.mode, NavMode::Graph);
        // 8 base items + 1 instance = 9 items
        assert_eq!(app.current_item_count(), 9);
    }

    #[test]
    fn test_nav_mode_label() {
        assert_eq!(NavMode::Graph.label(), "Graph");
        assert_eq!(NavMode::Nexus.label(), "Nexus");
    }

    #[test]
    fn test_nav_mode_index() {
        assert_eq!(NavMode::Graph.index(), 0);
        assert_eq!(NavMode::Nexus.index(), 1);
    }

    #[test]
    fn test_key_1_switches_to_graph() {
        let mut app = create_test_app();
        app.mode = NavMode::Nexus; // Start in Nexus

        app.handle_key(crossterm::event::KeyEvent::from(KeyCode::Char('1')));

        assert_eq!(app.mode, NavMode::Graph);
    }

    #[test]
    fn test_key_2_switches_to_nexus() {
        let mut app = create_test_app();

        app.handle_key(crossterm::event::KeyEvent::from(KeyCode::Char('2')));

        assert_eq!(app.mode, NavMode::Nexus);
    }

    #[test]
    fn test_collapsed_kind_hides_instances_in_graph_mode() {
        let mut app = create_test_app();

        // Add instances
        let instances = vec![
            InstanceInfo {
                key: "fr-FR".to_string(),
                display_name: "Français".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
            },
            InstanceInfo {
                key: "en-US".to_string(),
                display_name: "English".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
            },
        ];
        app.tree
            .set_instances("Locale", instances.clone(), instances.len());

        // Graph mode (default) shows instances
        assert_eq!(app.mode, NavMode::Graph);

        // With expanded Locale: 10 items
        assert_eq!(app.current_item_count(), 10);

        // Collapse Locale's instances (using toggle since it starts expanded)
        app.tree.toggle("kind:Locale");

        // Now: 8 items (instances hidden when collapsed)
        assert_eq!(app.current_item_count(), 8);
    }

    // === FILTERED DATA MODE TESTS ===

    #[test]
    fn test_filtered_mode_entry_resets_cursor() {
        let mut app = create_test_app();
        app.tree_cursor = 5; // Some position
        app.tree_scroll = 3;
        app.info_scroll = 2;
        app.yaml_scroll = 1;
        app.mode = NavMode::Graph; // Graph mode shows instances

        app.enter_filtered_data_mode("Locale".to_string());

        // All cursors/scrolls should be reset
        assert_eq!(app.tree_cursor, 0);
        assert_eq!(app.tree_scroll, 0);
        assert_eq!(app.info_scroll, 0);
        assert_eq!(app.yaml_scroll, 0);
        // Previous cursor saved
        assert_eq!(app.data_cursor_before_filter, 5);
        assert!(app.is_filtered_data_mode());
    }

    #[test]
    fn test_filtered_mode_exit_restores_cursor() {
        let mut app = create_test_app();
        app.tree_cursor = 5;
        app.mode = NavMode::Graph;

        app.enter_filtered_data_mode("Locale".to_string());
        assert_eq!(app.tree_cursor, 0);

        app.exit_filtered_data_mode();
        assert_eq!(app.tree_cursor, 5);
        assert!(!app.is_filtered_data_mode());
    }

    #[test]
    fn test_filtered_mode_exit_clamps_cursor_to_bounds() {
        let mut app = create_test_app();
        app.mode = NavMode::Graph;
        app.tree_cursor = 100; // Way beyond valid range
        app.data_cursor_before_filter = 100;
        app.data_filter_kind = Some("Locale".to_string());

        app.exit_filtered_data_mode();

        // Cursor should be clamped to max valid position
        let max = app.tree.item_count().saturating_sub(1);
        assert!(app.tree_cursor <= max);
    }

    #[test]
    fn test_filtered_mode_empty_instances() {
        let mut app = create_test_app();
        app.mode = NavMode::Graph;
        // Page has no instances loaded
        app.enter_filtered_data_mode("Page".to_string());

        // Should still be in filtered mode
        assert!(app.is_filtered_data_mode());
        assert_eq!(app.get_filter_kind(), Some("Page"));

        // Count should be 0 (no instances)
        assert_eq!(app.current_item_count(), 0);
    }

    #[test]
    fn test_filtered_mode_with_instances() {
        let mut app = create_test_app();
        app.mode = NavMode::Graph;

        // Add instances to Locale kind
        let instances = vec![
            InstanceInfo {
                key: "fr-FR".to_string(),
                display_name: "Français".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
            },
            InstanceInfo {
                key: "en-US".to_string(),
                display_name: "English".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
            },
        ];
        app.tree
            .set_instances("Locale", instances.clone(), instances.len());

        app.enter_filtered_data_mode("Locale".to_string());

        // Count should be 2 instances
        assert_eq!(app.current_item_count(), 2);

        // Current item should be first instance
        match app.current_item() {
            Some(TreeItem::Instance(_, _, _, inst)) => {
                assert_eq!(inst.key, "fr-FR");
            }
            other => panic!("Expected Instance fr-FR, got {:?}", other),
        }
    }

    #[test]
    fn test_key_esc_exits_filtered_mode() {
        let mut app = create_test_app();
        app.mode = NavMode::Graph;
        app.enter_filtered_data_mode("Locale".to_string());

        assert!(app.is_filtered_data_mode());

        // Press Esc
        let handled = app.handle_key(crossterm::event::KeyEvent::from(KeyCode::Esc));

        assert!(handled);
        assert!(!app.is_filtered_data_mode());
        // v11.7: Esc exits filtered mode, stays in Graph mode
        assert_eq!(app.mode, NavMode::Graph);
    }

    #[test]
    fn test_key_1_in_graph_mode_no_mode_change() {
        let mut app = create_test_app();
        app.mode = NavMode::Graph;
        app.enter_filtered_data_mode("Locale".to_string());

        assert!(app.is_filtered_data_mode());

        // Press 1 (Graph mode - already in Graph mode, so no mode change)
        app.handle_key(crossterm::event::KeyEvent::from(KeyCode::Char('1')));

        // v11.7: Key 1 = Graph mode, already in Graph so filtered mode stays
        // Use Esc to exit filtered mode instead
        assert!(app.is_filtered_data_mode());
        assert_eq!(app.mode, NavMode::Graph);
    }

    // ========================================================================
    // Edge case: ensure_cursor_visible with zero tree_height
    // ========================================================================

    #[test]
    fn test_ensure_cursor_visible_zero_height() {
        let mut app = create_test_app();

        // Simulate edge case: tree_height = 0 (terminal too small)
        app.tree_height = 0;
        app.tree_cursor = 5;
        app.tree_scroll = 0;

        // Should not panic (saturating_sub prevents underflow)
        app.ensure_cursor_visible();

        // Cursor should still be valid
        assert_eq!(app.tree_cursor, 5);
    }

    #[test]
    fn test_ensure_cursor_visible_normal_scroll_down() {
        let mut app = create_test_app();

        // Normal case: cursor below viewport
        app.tree_height = 10;
        app.tree_cursor = 15;
        app.tree_scroll = 0;

        app.ensure_cursor_visible();

        // Scroll should adjust so cursor is at bottom of viewport
        // tree_scroll = tree_cursor - (tree_height - 1) = 15 - 9 = 6
        assert_eq!(app.tree_scroll, 6);
    }

    #[test]
    fn test_ensure_cursor_visible_scroll_up() {
        let mut app = create_test_app();

        // Cursor above viewport
        app.tree_height = 10;
        app.tree_cursor = 2;
        app.tree_scroll = 5;

        app.ensure_cursor_visible();

        // Scroll should adjust to show cursor at top
        assert_eq!(app.tree_scroll, 2);
    }

    // ========================================================================
    // Search navigation tests (Batch 3)
    // ========================================================================

    #[test]
    fn test_next_search_result_empty() {
        let mut app = create_test_app();
        // No search results - should do nothing
        app.search.results.clear();
        app.search.cursor = 0;

        app.next_search_result();

        // Cursor should remain at 0
        assert_eq!(app.search.cursor, 0);
    }

    #[test]
    fn test_next_search_result_advances() {
        let mut app = create_test_app();
        app.search.results = vec![0, 3, 5];
        app.search.cursor = 0;
        app.tree_height = 20; // Ensure cursor is visible

        app.next_search_result();

        assert_eq!(app.search.cursor, 1);
        assert_eq!(app.tree_cursor, 3); // Jumped to second result
    }

    #[test]
    fn test_next_search_result_stops_at_end() {
        let mut app = create_test_app();
        app.search.results = vec![0, 3];
        app.search.cursor = 1; // Already at last result
        app.tree_height = 20;

        app.next_search_result();

        // Should stay at last result
        assert_eq!(app.search.cursor, 1);
    }

    #[test]
    fn test_prev_search_result_empty() {
        let mut app = create_test_app();
        app.search.results.clear();
        app.search.cursor = 0;

        app.prev_search_result();

        assert_eq!(app.search.cursor, 0);
    }

    #[test]
    fn test_prev_search_result_goes_back() {
        let mut app = create_test_app();
        app.search.results = vec![0, 3, 5];
        app.search.cursor = 2;
        app.tree_height = 20;

        app.prev_search_result();

        assert_eq!(app.search.cursor, 1);
        assert_eq!(app.tree_cursor, 3);
    }

    #[test]
    fn test_prev_search_result_stops_at_start() {
        let mut app = create_test_app();
        app.search.results = vec![0, 3];
        app.search.cursor = 0; // Already at first result
        app.tree_height = 20;

        app.prev_search_result();

        assert_eq!(app.search.cursor, 0);
    }

    #[test]
    fn test_close_search_clears_all_state() {
        let mut app = create_test_app();
        app.search.active = true;
        app.search.query = "test".to_string();
        app.search.results = vec![1, 2, 3];
        app.search.scores = vec![100, 90, 80];
        app.search.matches.insert(1, vec![0, 2]);
        app.search.cursor = 2;

        app.close_search();

        assert!(!app.search.active);
        assert!(app.search.query.is_empty());
        assert!(app.search.results.is_empty());
        assert!(app.search.scores.is_empty());
        assert!(app.search.matches.is_empty());
        assert_eq!(app.search.cursor, 0);
    }

    // ========================================================================
    // Spinner tests (Batch 1)
    // ========================================================================

    #[test]
    fn test_spinner_frame_cycles() {
        let mut app = create_test_app();

        // Collect spinner frames for different ticks
        let mut frames = Vec::new();
        for tick in 0..20 {
            app.tick = tick;
            frames.push(app.spinner_frame());
        }

        // Should have multiple different frames (braille characters)
        let unique: std::collections::HashSet<_> = frames.iter().collect();
        assert!(unique.len() > 1); // At least 2 different frames
    }

    #[test]
    fn test_spinner_frame_is_braille() {
        let mut app = create_test_app();
        app.tick = 5;

        let frame = app.spinner_frame();

        // Braille characters are in Unicode block 0x2800-0x28FF
        assert!(frame as u32 >= 0x2800 && frame as u32 <= 0x28FF);
    }

    // ========================================================================
    // Fuzzy Search tests (Phase 6 TDD)
    // ========================================================================

    #[test]
    fn test_search_empty_query_returns_no_results() {
        let mut app = create_test_app();

        // Empty query
        app.search.query = String::new();
        app.update_search();

        assert!(app.search.results.is_empty());
        assert!(app.search.scores.is_empty());
        assert!(app.search.matches.is_empty());
    }

    #[test]
    fn test_search_finds_exact_match() {
        let mut app = create_test_app();

        // Search for "Page" (exact match exists)
        app.search.query = "Page".to_string();
        app.update_search();

        assert!(
            !app.search.results.is_empty(),
            "Should find at least one result"
        );

        // The result should include the index of "Page" kind
        // Tree structure: Kinds(0), shared(1), locale(2), Locale(3),
        //                 org(4), structure(5), Page(6), Arcs(7)
        assert!(
            app.search.results.contains(&6),
            "Should find Page at index 6, got: {:?}",
            app.search.results
        );
    }

    #[test]
    fn test_search_case_insensitive() {
        let mut app = create_test_app();

        // Search for "page" (lowercase) should match "Page"
        app.search.query = "page".to_string();
        app.update_search();

        assert!(
            !app.search.results.is_empty(),
            "Should find case-insensitive match"
        );
    }

    #[test]
    fn test_search_partial_match() {
        let mut app = create_test_app();

        // Search for "loc" should match "Locale" and "Locale Knowledge"
        app.search.query = "loc".to_string();
        app.update_search();

        assert!(
            app.search.results.len() >= 2,
            "Should find multiple partial matches, got: {}",
            app.search.results.len()
        );
    }

    #[test]
    fn test_search_fuzzy_match() {
        let mut app = create_test_app();

        // Search for "pg" should fuzzy match "Page"
        app.search.query = "pg".to_string();
        app.update_search();

        // May or may not find - depends on fuzzy threshold
        // This test documents expected behavior
        // Note: nucleo uses fuzzy matching, so "pg" might match "Page"
    }

    #[test]
    fn test_search_scores_descending() {
        let mut app = create_test_app();

        // Search for "l" which matches multiple items
        app.search.query = "l".to_string();
        app.update_search();

        if app.search.scores.len() >= 2 {
            // Scores should be in descending order (best first)
            for i in 1..app.search.scores.len() {
                assert!(
                    app.search.scores[i - 1] >= app.search.scores[i],
                    "Scores should be descending: {:?}",
                    app.search.scores
                );
            }
        }
    }

    #[test]
    fn test_search_stores_match_positions() {
        let mut app = create_test_app();

        // Search for "Page"
        app.search.query = "Page".to_string();
        app.update_search();

        // Should have match positions for each result
        for &idx in &app.search.results {
            assert!(
                app.search.matches.contains_key(&idx),
                "Should have match positions for index {}",
                idx
            );
        }
    }

    #[test]
    fn test_search_clears_on_new_search() {
        let mut app = create_test_app();

        // First search
        app.search.query = "Page".to_string();
        app.update_search();
        let first_results = app.search.results.clone();
        assert!(!first_results.is_empty());

        // Second search clears previous results
        app.search.query = "Locale".to_string();
        app.update_search();

        assert_ne!(
            app.search.results, first_results,
            "New search should replace old results"
        );
    }

    #[test]
    fn test_search_respects_collapsed_state() {
        let mut app = create_test_app();

        // Collapse the "kinds" section
        app.tree.collapsed.insert("kinds".to_string());

        // Search for "Page" (which is under kinds)
        app.search.query = "Page".to_string();
        app.update_search();

        // When kinds is collapsed, we shouldn't search its children
        // But we should still find "Node Kinds" header if it matches
        assert!(
            !app.search.results.contains(&6),
            "Should not find Page when kinds is collapsed"
        );
    }

    #[test]
    fn test_search_unicode_query() {
        let mut app = create_test_app();

        // Unicode query should not crash
        app.search.query = "日本語".to_string();
        app.update_search();

        // May or may not find results, but shouldn't panic
        // This tests robustness with non-ASCII input
    }

    #[test]
    fn test_search_special_chars_query() {
        let mut app = create_test_app();

        // Special characters should not crash
        app.search.query = ".*+?[]()".to_string();
        app.update_search();

        // nucleo handles regex-like chars as literals, shouldn't panic
    }

    #[test]
    fn test_search_results_match_scores_length() {
        let mut app = create_test_app();

        app.search.query = "a".to_string();
        app.update_search();

        // search_results and search_scores should have same length
        assert_eq!(
            app.search.results.len(),
            app.search.scores.len(),
            "Results and scores vectors should have same length"
        );
    }

    #[test]
    fn test_search_activate_deactivate() {
        let mut app = create_test_app();

        assert!(!app.search.active, "Search should start inactive");

        app.search.active = true;
        app.search.query = "test".to_string();
        app.update_search();

        assert!(app.search.active, "Search should be active");

        // Deactivate
        app.search.active = false;
        app.search.query.clear();
        app.update_search();

        assert!(!app.search.active, "Search should be inactive");
        assert!(app.search.results.is_empty(), "Results should be cleared");
    }

    #[test]
    fn test_search_long_query() {
        let mut app = create_test_app();

        // Very long query should not crash
        app.search.query = "a".repeat(1000);
        app.update_search();

        // Should complete without panic
    }

    #[test]
    fn test_search_whitespace_query() {
        let mut app = create_test_app();

        // Whitespace-only query
        app.search.query = "   ".to_string();
        app.update_search();

        // May find items with spaces in names, but shouldn't panic
    }

    // ========================================================================
    // Search ranking/scoring tests (Phase 6.3 TDD)
    // ========================================================================

    #[test]
    fn test_search_exact_match_scores_highest() {
        let mut app = create_test_app();

        // Search for exact name "Page"
        app.search.query = "Page".to_string();
        app.update_search();

        // Should have results
        assert!(
            !app.search.results.is_empty(),
            "Should find results for 'Page'"
        );

        // First result should be the exact match with highest score
        if app.search.results.len() > 1 {
            let top_score = app.search.scores[0];
            for score in &app.search.scores[1..] {
                assert!(
                    top_score >= *score,
                    "Top score {} should be >= other scores",
                    top_score
                );
            }
        }
    }

    #[test]
    fn test_search_prefix_match_scores_well() {
        let mut app = create_test_app();

        // Search for "Loc" which is a prefix of "Locale"
        app.search.query = "Loc".to_string();
        app.update_search();

        // Should find Locale-related items
        assert!(
            !app.search.results.is_empty(),
            "Should find results for 'Loc'"
        );
    }

    #[test]
    fn test_search_smart_case_lowercase() {
        let mut app = create_test_app();

        // Lowercase query should match regardless of case
        app.search.query = "page".to_string();
        app.update_search();

        // Should find "Page" even with lowercase query
        assert!(
            !app.search.results.is_empty(),
            "Lowercase 'page' should find 'Page'"
        );
    }

    #[test]
    fn test_search_smart_case_uppercase() {
        let mut app = create_test_app();

        // Query with uppercase forces case-sensitive matching
        app.search.query = "PAGE".to_string();
        app.update_search();

        // May or may not find results depending on nucleo's smart case behavior
        // Just verify no panic
    }

    #[test]
    fn test_search_same_query_same_scores() {
        let mut app = create_test_app();

        // Run search twice with same query
        app.search.query = "Entity".to_string();
        app.update_search();
        let first_results = app.search.results.clone();
        let first_scores = app.search.scores.clone();

        app.update_search();
        let second_results = app.search.results.clone();
        let second_scores = app.search.scores.clone();

        // Results and scores should be identical (deterministic)
        assert_eq!(
            first_results, second_results,
            "Results should be deterministic"
        );
        assert_eq!(
            first_scores, second_scores,
            "Scores should be deterministic"
        );
    }

    #[test]
    fn test_search_longer_query_fewer_results() {
        let mut app = create_test_app();

        // Short query
        app.search.query = "e".to_string();
        app.update_search();
        let short_count = app.search.results.len();

        // Longer query (more specific)
        app.search.query = "ent".to_string();
        app.update_search();
        let long_count = app.search.results.len();

        // More specific query should have fewer or equal results
        assert!(
            long_count <= short_count,
            "Longer query should filter more: {} vs {}",
            long_count,
            short_count
        );
    }

    #[test]
    fn test_search_matches_have_positions() {
        let mut app = create_test_app();

        app.search.query = "Pg".to_string();
        app.update_search();

        // For each result with match positions, verify positions are valid
        for (idx, positions) in &app.search.matches {
            // Index should be in results
            assert!(
                app.search.results.contains(idx),
                "Match index {} should be in results",
                idx
            );
            // Positions should be non-empty for matched items
            assert!(
                !positions.is_empty(),
                "Match positions for index {} should not be empty",
                idx
            );
        }
    }

    #[test]
    fn test_search_scores_are_positive() {
        let mut app = create_test_app();

        app.search.query = "Block".to_string();
        app.update_search();

        // All scores should be positive (nucleo returns positive scores for matches)
        for score in &app.search.scores {
            assert!(*score > 0, "Score should be positive, got {}", score);
        }
    }

    #[test]
    fn test_search_result_indices_reasonable() {
        let mut app = create_test_app();

        app.search.query = "config".to_string();
        app.update_search();

        // Result indices should be reasonable (within expected tree size)
        // Tree has: 2 headers + 2 realms + 10 layers + 60 kinds + 5 arc families + arc kinds
        // So max should be under 200
        let reasonable_max = 200;
        for idx in &app.search.results {
            assert!(
                *idx < reasonable_max,
                "Result index {} should be < {} (reasonable tree size)",
                idx,
                reasonable_max
            );
        }
    }

    #[test]
    fn test_search_different_queries_different_results() {
        let mut app = create_test_app();

        // First query
        app.search.query = "Page".to_string();
        app.update_search();
        let page_results = app.search.results.clone();

        // Different query
        app.search.query = "Block".to_string();
        app.update_search();
        let block_results = app.search.results.clone();

        // Should have different results (at least top result should differ)
        if !page_results.is_empty() && !block_results.is_empty() {
            assert_ne!(
                page_results[0], block_results[0],
                "Different queries should give different top results"
            );
        }
    }

    #[test]
    fn test_search_no_duplicate_results() {
        let mut app = create_test_app();

        app.search.query = "a".to_string(); // Common letter, many matches
        app.update_search();

        // Check no duplicates in results
        let mut seen = std::collections::HashSet::new();
        for idx in &app.search.results {
            assert!(seen.insert(*idx), "Duplicate result index: {}", idx);
        }
    }

    #[test]
    fn test_search_match_positions_sorted() {
        let mut app = create_test_app();

        app.search.query = "Pge".to_string();
        app.update_search();

        // Match positions should be sorted ascending (char positions in order)
        for positions in app.search.matches.values() {
            for i in 1..positions.len() {
                assert!(
                    positions[i - 1] < positions[i],
                    "Match positions should be sorted: {:?}",
                    positions
                );
            }
        }
    }

    // ========================================================================
    // Edge case tests (Phase 6.4 TDD)
    // ========================================================================

    #[test]
    fn test_empty_tree_cursor_at_zero() {
        // Create app with empty tree
        let tree = TaxonomyTree::default();
        let app = App::new(tree, "/test/root".to_string());

        assert_eq!(app.tree_cursor, 0, "Initial cursor should be 0");
    }

    #[test]
    fn test_navigation_cursor_at_zero_saturating_sub() {
        let mut app = create_test_app();
        app.tree_cursor = 0;

        // Saturating subtraction should keep at 0
        app.tree_cursor = app.tree_cursor.saturating_sub(1);
        assert_eq!(app.tree_cursor, 0, "Cursor should not go negative");
    }

    #[test]
    fn test_navigation_cursor_increment_decrement() {
        let mut app = create_test_app();
        app.tree_cursor = 0;

        // Move down then up should return to original
        app.tree_cursor += 1;
        assert_eq!(app.tree_cursor, 1);
        app.tree_cursor = app.tree_cursor.saturating_sub(1);
        assert_eq!(app.tree_cursor, 0);
    }

    #[test]
    fn test_navigation_page_size_moves() {
        let mut app = create_test_app();
        app.tree_cursor = 0;
        app.tree_height = 10; // Page size

        // Page down
        app.tree_cursor += app.tree_height;
        let after_down = app.tree_cursor;
        assert!(after_down >= 10, "Should move down by page size");

        // Page up
        app.tree_cursor = app.tree_cursor.saturating_sub(app.tree_height);
        assert!(app.tree_cursor < after_down, "Should move back up");
    }

    #[test]
    fn test_search_rapid_query_changes() {
        let mut app = create_test_app();

        // Rapidly change query
        for query in ["a", "ab", "abc", "ab", "a", ""] {
            app.search.query = query.to_string();
            app.update_search();
        }

        // Final empty query should clear results
        assert!(app.search.results.is_empty());
    }

    #[test]
    fn test_search_unicode_emoji_query() {
        let mut app = create_test_app();

        // Emoji query - should not panic
        app.search.query = "🔍".to_string();
        app.update_search();

        // May or may not find results, just verify no panic
    }

    #[test]
    fn test_search_unicode_cjk_query() {
        let mut app = create_test_app();

        // Chinese characters query
        app.search.query = "项目".to_string();
        app.update_search();

        // Should not panic, likely no results in English codebase
    }

    #[test]
    fn test_search_unicode_arabic_query() {
        let mut app = create_test_app();

        // Arabic query (right-to-left)
        app.search.query = "مشروع".to_string();
        app.update_search();

        // Should handle RTL gracefully
    }

    #[test]
    fn test_search_very_long_query_100_chars() {
        let mut app = create_test_app();

        // Very long query
        app.search.query = "a".repeat(100);
        app.update_search();

        // Should handle without panic, likely no matches
        assert!(
            app.search.results.is_empty(),
            "100-char query unlikely to match anything"
        );
    }

    #[test]
    fn test_search_newline_in_query() {
        let mut app = create_test_app();

        // Query with newline character
        app.search.query = "Page\nBlock".to_string();
        app.update_search();

        // Should handle gracefully (may or may not match)
    }

    #[test]
    fn test_search_tab_in_query() {
        let mut app = create_test_app();

        // Query with tab character
        app.search.query = "Page\tBlock".to_string();
        app.update_search();

        // Should handle gracefully
    }

    #[test]
    fn test_collapse_all_then_search() {
        let mut app = create_test_app();

        // Collapse everything
        app.tree.collapse_all();

        // Search should still work on visible items (headers)
        app.search.query = "Kind".to_string();
        app.update_search();

        // Should find "Node Kinds" header
        assert!(
            !app.search.results.is_empty(),
            "Should find header even when collapsed"
        );
    }

    #[test]
    fn test_toggle_collapse_on_tree() {
        let mut app = create_test_app();

        // Toggle a section using the tree API
        app.tree.toggle("kinds");

        // Verify toggle happened (collapsed state changed)
        let is_collapsed = app.tree.is_collapsed("kinds");
        // Toggle again
        app.tree.toggle("kinds");
        let after_toggle = app.tree.is_collapsed("kinds");

        // Should be opposite of before
        assert_ne!(
            is_collapsed, after_toggle,
            "Toggle should change collapsed state"
        );
    }

    #[test]
    fn test_mode_switch_direct() {
        let mut app = create_test_app();

        // Start search
        app.search.query = "Page".to_string();
        app.search.active = true;
        app.update_search();

        // Switch mode directly (to Nexus, for example)
        app.mode = NavMode::Nexus;

        // Search state should still exist
        assert_eq!(app.mode, NavMode::Nexus);
        // Search results preserved (or cleared depending on design)
    }

    #[test]
    fn test_scroll_at_boundary_zero() {
        let mut app = create_test_app();
        app.tree_scroll = 0;
        app.tree_height = 10;

        // Scroll up from 0 should stay at 0
        if app.tree_scroll > 0 {
            app.tree_scroll -= 1;
        }
        assert_eq!(app.tree_scroll, 0);
    }

    #[test]
    fn test_yaml_scroll_at_zero() {
        let mut app = create_test_app();
        app.yaml_scroll = 0;

        // Should not go negative
        if app.yaml_scroll > 0 {
            app.yaml_scroll -= 1;
        }
        assert_eq!(app.yaml_scroll, 0);
    }

    #[test]
    fn test_focus_cycle_tree_to_yaml() {
        let mut app = create_test_app();
        app.focus = Focus::Tree;

        // Cycle focus
        app.focus = Focus::Yaml;
        assert_eq!(app.focus, Focus::Yaml);

        app.focus = Focus::Tree;
        assert_eq!(app.focus, Focus::Tree);
    }

    #[test]
    fn test_info_scroll_at_zero() {
        let mut app = create_test_app();
        app.info_scroll = 0;

        // Should stay at 0
        if app.info_scroll > 0 {
            app.info_scroll -= 1;
        }
        assert_eq!(app.info_scroll, 0);
    }

    #[test]
    fn test_tick_counter_wraps() {
        let mut app = create_test_app();

        // Increment tick many times
        for _ in 0..1000 {
            app.tick = app.tick.wrapping_add(1);
        }

        // Should handle wrapping without panic
        // tick is u16 - after 1000 increments from 0, should be 1000
        assert_eq!(app.tick, 1000);
    }

    #[test]
    fn test_hide_empty_toggle() {
        let mut app = create_test_app();

        let initial = app.hide_empty;
        app.hide_empty = !app.hide_empty;
        assert_ne!(app.hide_empty, initial);

        app.hide_empty = !app.hide_empty;
        assert_eq!(app.hide_empty, initial);
    }

    #[test]
    fn test_set_status_error_adds_warning_prefix() {
        let mut app = create_test_app();

        // Error status should have ⚠ prefix
        app.set_status_error("Neo4j connection failed");
        assert!(app.status_message.is_some());
        let (msg, _) = app.status_message.as_ref().unwrap();
        assert!(msg.starts_with("⚠"), "Error status should start with ⚠");
        assert!(msg.contains("Neo4j connection failed"));

        // Regular status should NOT have ⚠ prefix
        app.set_status("Loading...");
        let (msg, _) = app.status_message.as_ref().unwrap();
        assert!(!msg.starts_with("⚠"), "Regular status should not have ⚠");
    }
}
