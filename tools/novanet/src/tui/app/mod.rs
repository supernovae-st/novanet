//! App state for TUI v2.
//!
//! v0.17.3: Refactored into submodules:
//! - `constants`: Scroll amounts, margins, UI defaults
//! - `state`: Navigation enums and state structs

mod constants;
mod state;

// Re-export constants
pub use constants::*;

// Re-export state types
pub use state::{
    ContentPanelMode, Focus, InfoBox, LoadedDetails, NavMode, OverlayState, Panel, PanelRects,
    PendingLoads, SchemaOverlayState, SearchState, TreeItemData, YamlPreviewState,
};

use nucleo_matcher::pattern::{Atom, AtomKind, CaseMatching, Normalization};
use nucleo_matcher::{Config, Matcher, Utf32Str};
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::fs;
use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

use super::cache::RenderCache;
use super::data::{
    ArcClassDetails, ClassArcsData, LayerDetails, RealmDetails, TaxonomyTree, TreeItem,
    get_all_adrs, get_architecture_diagram,
};
use super::handlers::dispatch_mode_handler;
use super::nexus::views::LoadedViews;
use super::nexus::{NexusState, NexusTab};
use super::theme::Theme;

use ratatui::text::Span;

// =============================================================================
// APP STRUCT
// =============================================================================

/// Main app state.
/// v0.14.0: Refactored with sub-structs for better organization.
/// v0.17.3: State types extracted to state.rs
/// 55 fields → 30 direct + 25 in sub-structs.
#[allow(dead_code)]
pub struct App {
    // ==========================================================================
    // Core State
    // ==========================================================================
    /// Cached theme (color mode detected once at startup).
    pub theme: Theme,
    pub mode: NavMode,
    pub focus: Focus,
    /// Currently selected info box for copy/scroll (Graph mode).
    pub selected_box: InfoBox,
    pub tree_cursor: usize,
    /// Remember cursor position per mode (Graph, Nexus, Views).
    pub mode_cursors: [usize; 3],
    pub tree_scroll: usize,
    pub tree_height: usize,
    pub tree: TaxonomyTree,
    pub root_path: String,

    // ==========================================================================
    // Extracted Sub-States (v0.14.0)
    // ==========================================================================
    /// Search state (extracted sub-state).
    pub search: SearchState,
    /// Overlay visibility state (help, legend, recent items).
    pub overlays: OverlayState,
    /// YAML panel state.
    pub yaml: YamlPreviewState,
    /// Pending async load requests.
    pub pending: PendingLoads,
    /// Neo4j details for current selection.
    pub details: LoadedDetails,
    /// Schema overlay state for Data mode.
    pub schema_overlay: SchemaOverlayState,
    /// Nexus mode state (gamified learning hub).
    pub nexus: NexusState,

    // ==========================================================================
    // Navigation & History
    // ==========================================================================
    /// Navigation history for Ctrl+o (back) / Ctrl+i (forward).
    pub nav_history: Vec<(NavMode, usize)>,
    pub nav_history_pos: usize,
    /// Navigation generation counter for detecting stale async results.
    pub navigation_generation: u64,

    // ==========================================================================
    // UI State
    // ==========================================================================
    /// Status message (e.g., "Copied to clipboard", "Refreshing...").
    pub status_message: Option<(String, std::time::Instant)>,
    /// Pending refresh request.
    pub pending_refresh: bool,
    // v0.16.3: Separate scroll states for Props and Arcs panels
    /// Properties panel scroll position.
    pub props_scroll: usize,
    /// Properties panel total line count.
    pub props_line_count: usize,
    /// Arcs panel scroll position.
    pub arcs_scroll: usize,
    /// Arcs panel total line count.
    pub arcs_line_count: usize,
    /// Cache of YAML file contents (path -> content).
    pub yaml_cache: FxHashMap<String, String>,
    /// Loaded views from views.yaml (single source of truth for TUI + Studio).
    pub loaded_views: LoadedViews,
    /// Animation tick counter (increments each frame, used for spinners).
    pub tick: u16,
    /// Panel rectangles for mouse hit-testing (updated each render).
    pub panel_rects: PanelRects,

    // ==========================================================================
    // Filter State
    // ==========================================================================
    /// Data mode filter: when set, show only instances of this Class.
    pub data_filter_class: Option<String>,
    /// Cursor position before entering filtered Data mode (for restoration).
    pub data_cursor_before_filter: usize,
    /// Hide empty: when true, hide classes/layers with 0 instances in Data mode.
    pub hide_empty: bool,
    // v0.17.3 (ADR-036): trait_filter and filter_pending removed - traits no longer in schema

    // ==========================================================================
    // Property Focus State (Feature 3)
    // ==========================================================================
    /// Index of focused property in Info panel (for truncate intelligent).
    pub focused_property_idx: usize,
    /// Whether the focused property text is expanded (Enter toggle).
    pub expanded_property: bool,
    /// Whether to pretty-print JSON values (toggle with 'J').
    pub json_pretty: bool,

    // ==========================================================================
    // Instance Panel State (v0.17.3)
    // ==========================================================================
    /// Whether STANDARD section is collapsed in instance panel (default: false = expanded).
    pub instance_standard_collapsed: bool,
    /// Whether SPECIFIC section is collapsed in instance panel (default: false = expanded).
    pub instance_specific_collapsed: bool,

    // ==========================================================================
    // Render Caches (Performance Optimization)
    // ==========================================================================
    /// Cache for status bar realm mini-bar (avoids Vec allocation per frame).
    /// Uses RefCell for interior mutability during immutable render calls.
    pub mini_bar_cache: RefCell<RenderCache<Vec<Span<'static>>>>,
}

impl App {
    pub fn new(tree: TaxonomyTree, root_path: String) -> Self {
        // Load views before root_path is moved
        let loaded_views = LoadedViews::load(&root_path);

        let mut app = Self {
            // Core state
            theme: Theme::with_root(&root_path),
            mode: NavMode::Graph,
            focus: Focus::Tree,
            selected_box: InfoBox::default(),
            tree_cursor: 0,
            mode_cursors: [0; 3],
            tree_scroll: 0,
            tree_height: DEFAULT_TREE_HEIGHT,
            tree,
            root_path,

            // Extracted sub-states (v0.14.0)
            search: SearchState::default(),
            overlays: OverlayState::default(),
            yaml: YamlPreviewState::default(),
            pending: PendingLoads::default(),
            details: LoadedDetails::default(),
            schema_overlay: SchemaOverlayState::default(),
            nexus: NexusState::with_persistence(),

            // Navigation & history
            nav_history: Vec::with_capacity(100),
            nav_history_pos: 0,
            navigation_generation: 0,

            // UI state
            status_message: None,
            pending_refresh: false,
            // v0.16.3: Separate scroll for Props and Arcs panels
            props_scroll: 0,
            props_line_count: 0,
            arcs_scroll: 0,
            arcs_line_count: 0,
            yaml_cache: FxHashMap::default(),
            loaded_views,
            tick: 0,
            panel_rects: PanelRects::default(),

            // Filter state
            data_filter_class: None,
            data_cursor_before_filter: 0,
            hide_empty: false,
            // v0.17.3 (ADR-036): trait_filter/filter_pending removed

            // Property focus state
            focused_property_idx: 0,
            expanded_property: false,
            json_pretty: false,

            // Instance panel state (v0.17.3)
            instance_standard_collapsed: false, // Expanded by default
            instance_specific_collapsed: false, // Expanded by default

            // Render caches
            mini_bar_cache: RefCell::new(RenderCache::new()),
        };
        // v0.17.3: Initialize with smart collapsed defaults for better UX
        // (Classes section open with realms visible, but layers/classes collapsed)
        app.tree.init_default_collapsed();
        app.load_yaml_for_current();
        app
    }

    // v0.13.1: yaml_active_section() removed (collapse/peek eliminated)

    /// Map selected_box to the appropriate Focus panel.
    /// v0.16.3: Updated for 4-panel layout (Tree/Yaml/Props/Arcs).
    /// v0.18.3: DEPRECATED - use Focus directly instead of InfoBox.
    #[deprecated(since = "0.18.3", note = "Use Focus enum directly")]
    #[allow(deprecated)]
    pub fn focus_for_selected_box(&self) -> Focus {
        match self.selected_box {
            InfoBox::Tree => Focus::Tree,
            InfoBox::Header | InfoBox::Properties => Focus::Props,
            InfoBox::Arcs => Focus::Arcs,
            InfoBox::Source => Focus::Content,
        }
    }

    /// Load YAML content for the current cursor position.
    /// Uses mode-aware item lookup to handle filtered Data mode correctly.
    pub fn load_yaml_for_current(&mut self) {
        // Increment generation to invalidate any in-flight async loads
        self.navigation_generation = self.navigation_generation.wrapping_add(1);

        // Reset scroll positions when changing items
        self.yaml.scroll = 0;
        self.props_scroll = 0;
        self.arcs_scroll = 0;
        // v0.16.3: Reset property focus when changing tree items
        self.focused_property_idx = 0;
        self.expanded_property = false;

        // Clear Neo4j data AND pending loads when moving away
        // (prevents race condition where pending load completes after navigation)
        self.details.class_arcs = None;
        self.details.arc_class = None;
        self.details.realm = None;
        self.details.layer = None;
        self.pending.arcs = None;
        self.pending.arc_class = None;
        self.pending.realm = None;
        self.pending.layer = None;
        self.pending.instance = None;

        // Clear Class validation state (only populated for Class items)
        self.schema_overlay.validated_class_properties = None;
        self.schema_overlay.validation_stats = None;

        // Get current item using mode-aware method (handles filtered Data mode)
        // This is the same logic as current_item() but we extract data to avoid borrow issues
        let current = self.get_current_tree_item_data();

        // v0.17.3: Content panel mode is determined by tree selection (no toggle)
        // Handle based on item type
        match current {
            TreeItemData::Class {
                yaml_path,
                key,
                properties,
            } => {
                self.load_yaml_cached(&yaml_path);
                self.pending.arcs = Some(key);
                // Load Class validation (Neo4j vs YAML)
                self.load_validated_class_properties(&properties);
            },
            TreeItemData::ArcClass { yaml_path, key } => {
                self.load_yaml_cached(&yaml_path);
                self.pending.arc_class = Some(key);
            },
            TreeItemData::Realm { key } => {
                let path = format!("packages/core/models/realms/{}.yaml", key);
                self.load_yaml_cached(&path);
                self.pending.realm = Some(key);
            },
            TreeItemData::Layer { key } => {
                let path = format!("packages/core/models/layers/{}.yaml", key);
                self.load_yaml_cached(&path);
                self.pending.layer = Some(key);
            },
            TreeItemData::ArcFamily { key } => {
                let path = format!("packages/core/models/arc-families/{}.yaml", key);
                self.load_yaml_cached(&path);
            },
            TreeItemData::Section => {
                // v0.12.5: Show _index.yaml (complete schema overview) instead of taxonomy.yaml
                self.load_yaml_cached("packages/core/models/_index.yaml");
            },
            TreeItemData::Instance {
                class_yaml_path,
                class_properties,
                ..
            } => {
                // Load the Class's YAML to show Instance schema (standard_properties)
                if !class_yaml_path.is_empty() {
                    self.load_yaml_cached(&class_yaml_path);
                    // Load validated properties with types (same as Class view)
                    self.load_validated_class_properties(&class_properties);
                } else {
                    self.yaml.path.clear();
                    self.yaml.content.clear();
                    self.yaml.line_count = 0;
                }
            },
            TreeItemData::None => {
                self.yaml.path.clear();
                self.yaml.content.clear();
                self.yaml.line_count = 0;
            },
        }
    }

    /// Extract current tree item data using mode-aware lookup.
    /// Handles filtered Data mode correctly (same logic as current_item()).
    fn get_current_tree_item_data(&self) -> TreeItemData {
        // In filtered Data mode, always return Instance (that's all we show)
        if self.is_graph_mode() && self.data_filter_class.is_some() {
            if let Some(class_key) = &self.data_filter_class {
                if let Some(TreeItem::Instance(realm, layer, class_info, instance)) =
                    self.tree.filtered_item_at(self.tree_cursor, class_key)
                {
                    return TreeItemData::Instance {
                        instance_key: instance.key.clone(),
                        class_name: class_info.key.clone(),
                        realm: realm.key.clone(),
                        layer: layer.key.clone(),
                        class_yaml_path: class_info.yaml_path.clone(),
                        class_properties: class_info.properties.clone(),
                        properties: instance.properties.clone(),
                    };
                }
            }
            return TreeItemData::None;
        }

        // Use mode-aware item lookup
        // v0.17.3: Pass hide_empty to match render_tree filtering
        let item = if self.is_graph_mode() {
            self.tree
                .item_at_for_mode(self.tree_cursor, true, self.hide_empty)
        } else {
            self.tree.item_at(self.tree_cursor)
        };

        match item {
            Some(TreeItem::Class(_, _, class_info)) => TreeItemData::Class {
                yaml_path: class_info.yaml_path.clone(),
                key: class_info.key.clone(),
                properties: class_info.properties.clone(),
            },
            Some(TreeItem::ArcClass(family, arc)) => {
                let arc_file = arc.key.to_lowercase().replace('_', "-");
                TreeItemData::ArcClass {
                    yaml_path: format!(
                        "packages/core/models/arc-classes/{}/{}.yaml",
                        family.key, arc_file
                    ),
                    key: arc.key.clone(),
                }
            },
            Some(TreeItem::Realm(realm)) => TreeItemData::Realm {
                key: realm.key.clone(),
            },
            Some(TreeItem::Layer(_, layer)) => TreeItemData::Layer {
                key: layer.key.clone(),
            },
            Some(TreeItem::ArcFamily(family)) => TreeItemData::ArcFamily {
                key: family.key.clone(),
            },
            Some(TreeItem::ClassesSection) | Some(TreeItem::ArcsSection) => TreeItemData::Section,
            Some(TreeItem::Instance(realm, layer, class_info, instance)) => {
                TreeItemData::Instance {
                    instance_key: instance.key.clone(),
                    class_name: class_info.key.clone(),
                    realm: realm.key.clone(),
                    layer: layer.key.clone(),
                    class_yaml_path: class_info.yaml_path.clone(),
                    class_properties: class_info.properties.clone(),
                    properties: instance.properties.clone(),
                }
            },
            // EntityCategory is a grouper (THING, ACTION, etc.) - show as Section
            // v0.17.3: Categories don't have YAML schema, they are navigational groupers
            Some(TreeItem::EntityCategory(_, _, _, _)) => TreeItemData::Section,
            // LocaleGroup is a grouper (locale code) - show as Section
            // Note: Legacy, kept for backwards compatibility
            Some(TreeItem::LocaleGroup(_, _, _, _)) => TreeItemData::Section,
            // v0.17.3: EntityGroup shows parent Entity as INSTANCE panel
            // Look up the Entity instance by key to show its properties
            Some(TreeItem::EntityGroup(_, _, _, group)) => {
                // Find Entity class info
                if let Some((entity_realm, entity_layer, entity_class_info)) =
                    self.tree.find_class("Entity")
                {
                    // Look up the Entity instance with matching key
                    if let Some(instances) = self.tree.instances.get("Entity") {
                        if let Some(entity_instance) =
                            instances.iter().find(|i| i.key == group.entity_key)
                        {
                            return TreeItemData::Instance {
                                instance_key: entity_instance.key.clone(),
                                class_name: entity_class_info.key.clone(),
                                realm: entity_realm.key.clone(),
                                layer: entity_layer.key.clone(),
                                class_yaml_path: entity_class_info.yaml_path.clone(),
                                class_properties: entity_class_info.properties.clone(),
                                properties: entity_instance.properties.clone(),
                            };
                        }
                    }
                }
                // Fallback: show helpful message if Entity lookup fails
                TreeItemData::None
            },
            // EntityNativeItem shows as Instance (same data structure)
            // v0.17.3: Now includes full properties for INSTANCE panel display
            Some(TreeItem::EntityNativeItem(realm, layer, class_info, native)) => {
                TreeItemData::Instance {
                    instance_key: native.key.clone(),
                    class_name: class_info.key.clone(),
                    realm: realm.key.clone(),
                    layer: layer.key.clone(),
                    class_yaml_path: class_info.yaml_path.clone(),
                    class_properties: class_info.properties.clone(),
                    properties: native.properties.clone(),
                }
            },
            None => TreeItemData::None,
        }
    }

    /// Determine the content panel mode based on current tree selection.
    /// v0.17.3: Phase 3 of source-panel-redesign.md
    ///
    /// Returns a `ContentPanelMode` indicating what the center panel should show:
    /// - `Schema`: YAML definition for Class/ArcClass nodes
    /// - `InstanceInfo`: Info message pointing to PROPERTIES for instances
    /// - `SectionInfo`: Section overview for Section headers
    /// - `Empty`: No content available
    pub fn content_panel_mode(&self) -> ContentPanelMode {
        match self.get_current_tree_item_data() {
            TreeItemData::Class { yaml_path, key, .. } => ContentPanelMode::Schema {
                path: yaml_path,
                name: key,
            },
            TreeItemData::ArcClass { yaml_path, key } => ContentPanelMode::Schema {
                path: yaml_path,
                name: key,
            },
            TreeItemData::Instance {
                instance_key,
                class_name,
                realm,
                layer,
                properties,
                ..
            } => ContentPanelMode::InstanceInfo {
                instance_key,
                class_name,
                realm,
                layer,
                properties,
            },
            TreeItemData::Realm { key } => ContentPanelMode::Schema {
                path: format!("packages/core/models/realms/{}.yaml", key),
                name: format!("Realm: {}", key),
            },
            TreeItemData::Layer { key } => ContentPanelMode::Schema {
                path: format!("packages/core/models/layers/{}.yaml", key),
                name: format!("Layer: {}", key),
            },
            TreeItemData::ArcFamily { key } => ContentPanelMode::Schema {
                path: format!("packages/core/models/arc-families/{}.yaml", key),
                name: format!("Arc Family: {}", key),
            },
            TreeItemData::Section => ContentPanelMode::SectionInfo {
                name: "Section".to_string(),
                description: "Navigate to view node or arc classes.".to_string(),
            },
            TreeItemData::None => ContentPanelMode::Empty,
        }
    }

    /// Load YAML content with caching (avoids re-reading files on every navigation).
    fn load_yaml_cached(&mut self, relative_path: &str) {
        self.yaml.path = relative_path.to_string();
        // v0.13.1: yaml_peek reset removed (collapse/peek eliminated)

        // Check cache first
        if let Some(cached) = self.yaml_cache.get(relative_path) {
            self.yaml.content = cached.clone();
            self.yaml.line_count = self.yaml.content.lines().count();
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
        self.yaml.content = content;
        self.yaml.line_count = self.yaml.content.lines().count();
    }

    /// Ensure cursor is visible by adjusting scroll.
    pub fn ensure_cursor_visible(&mut self) {
        // v0.17.3: Debug assertion to catch cursor bounds bugs during development
        #[cfg(debug_assertions)]
        {
            let max = self.current_item_count();
            debug_assert!(
                self.tree_cursor < max || max == 0,
                "tree_cursor ({}) >= item_count ({})",
                self.tree_cursor,
                max
            );
        }

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

    /// Navigate to a specific class by key.
    ///
    /// Expands the tree path to the class (classes header → realm → layer)
    /// and sets the cursor to the class position.
    ///
    /// Returns `true` if the class was found and navigated to, `false` otherwise.
    pub fn navigate_to_class(&mut self, class_key: &str) -> bool {
        // Find the realm and layer containing this class
        let mut found_realm: Option<&str> = None;
        let mut found_layer: Option<&str> = None;

        for realm in &self.tree.realms {
            for layer in &realm.layers {
                for class in &layer.classes {
                    if class.key == class_key {
                        found_realm = Some(&realm.key);
                        found_layer = Some(&layer.key);
                        break;
                    }
                }
                if found_layer.is_some() {
                    break;
                }
            }
            if found_realm.is_some() {
                break;
            }
        }

        let (realm_key, layer_key) = match (found_realm, found_layer) {
            (Some(r), Some(l)) => (r.to_string(), l.to_string()),
            _ => return false,
        };

        // Expand the path: classes header, realm, layer
        self.tree.expand("classes");
        self.tree.expand(&format!("realm:{}", realm_key));
        self.tree
            .expand(&format!("layer:{}:{}", realm_key, layer_key));

        // Calculate the index (same logic as update_search)
        let mut idx = 0;

        // Classes section header
        idx += 1;

        for realm in &self.tree.realms {
            // Realm
            idx += 1;

            if !self.tree.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    // Layer
                    idx += 1;

                    if !self
                        .tree
                        .is_collapsed(&format!("layer:{}:{}", realm.key, layer.key))
                    {
                        for class in &layer.classes {
                            if class.key == class_key {
                                // Found it!
                                self.tree_cursor = idx;
                                self.ensure_cursor_visible();
                                self.load_yaml_for_current();
                                return true;
                            }
                            idx += 1;
                        }
                    }
                }
            }
        }

        false
    }

    // =========================================================================
    // Search Methods
    // =========================================================================

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

        // Classes section header
        if let Some((score, indices)) = fuzzy_match("Node Classes", &mut matcher, &pattern) {
            matches.push((idx, score, indices));
        }
        idx += 1;

        if !self.tree.is_collapsed("classes") {
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
                            for class_info in &layer.classes {
                                let match_display =
                                    fuzzy_match(&class_info.display_name, &mut matcher, &pattern);
                                let match_key =
                                    fuzzy_match(&class_info.key, &mut matcher, &pattern);
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
                    for arc_class in &family.arc_classes {
                        let match_display =
                            fuzzy_match(&arc_class.display_name, &mut matcher, &pattern);
                        let match_key = fuzzy_match(&arc_class.key, &mut matcher, &pattern);
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

    // =========================================================================
    // Event Handlers
    // =========================================================================

    /// Handle mouse input. Returns true if state changed (needs re-render).
    /// Mouse scroll works on the panel under the cursor, regardless of focus.
    pub fn handle_mouse(&mut self, event: MouseEvent) -> bool {
        match event.kind {
            MouseEventKind::ScrollUp => {
                if let Some(panel) = self.panel_rects.hit_test(event.column, event.row) {
                    match panel {
                        Panel::Tree => {
                            // Scroll up in tree (move cursor up)
                            if self.tree_cursor > 0 {
                                self.tree_cursor -= 1;
                                self.ensure_cursor_visible();
                                self.load_yaml_for_current();
                                return true;
                            }
                        },
                        Panel::Identity => {
                            // v0.18.3: Identity panel - no scroll (static content)
                        },
                        Panel::Content => {
                            if self.yaml.scroll > 0 {
                                self.yaml.scroll =
                                    self.yaml.scroll.saturating_sub(MOUSE_SCROLL_LINES);
                                return true;
                            }
                        },
                        Panel::Props => {
                            if self.props_scroll > 0 {
                                self.props_scroll =
                                    self.props_scroll.saturating_sub(MOUSE_SCROLL_LINES);
                                return true;
                            }
                        },
                        Panel::Arcs => {
                            if self.arcs_scroll > 0 {
                                self.arcs_scroll =
                                    self.arcs_scroll.saturating_sub(MOUSE_SCROLL_LINES);
                                return true;
                            }
                        },
                    }
                }
            },
            MouseEventKind::ScrollDown => {
                if let Some(panel) = self.panel_rects.hit_test(event.column, event.row) {
                    match panel {
                        Panel::Tree => {
                            // Scroll down in tree (move cursor down)
                            let max = self.current_item_count().saturating_sub(1);
                            if self.tree_cursor < max {
                                self.tree_cursor += 1;
                                self.ensure_cursor_visible();
                                self.load_yaml_for_current();
                                return true;
                            }
                        },
                        Panel::Identity => {
                            // v0.18.3: Identity panel - no scroll (static content)
                        },
                        Panel::Content => {
                            let max_scroll =
                                self.yaml.line_count.saturating_sub(YAML_SCROLL_MARGIN);
                            if self.yaml.scroll < max_scroll {
                                self.yaml.scroll =
                                    (self.yaml.scroll + MOUSE_SCROLL_LINES).min(max_scroll);
                                return true;
                            }
                        },
                        Panel::Props => {
                            let max_scroll =
                                self.props_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                            if self.props_scroll < max_scroll {
                                self.props_scroll =
                                    (self.props_scroll + MOUSE_SCROLL_LINES).min(max_scroll);
                                return true;
                            }
                        },
                        Panel::Arcs => {
                            let max_scroll =
                                self.arcs_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                            if self.arcs_scroll < max_scroll {
                                self.arcs_scroll =
                                    (self.arcs_scroll + MOUSE_SCROLL_LINES).min(max_scroll);
                                return true;
                            }
                        },
                    }
                }
            },
            // Click to focus: change panel focus when clicking
            MouseEventKind::Down(MouseButton::Left) => {
                if let Some(panel) = self.panel_rects.hit_test(event.column, event.row) {
                    let new_focus = panel.to_focus();
                    if self.focus != new_focus {
                        self.focus = new_focus;
                        return true;
                    }
                }
            },
            // Ignore other mouse events (right-click, middle-click, drags, etc.)
            _ => {},
        }
        false
    }

    /// Handle key input. Returns true if state changed (needs re-render).
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Help overlay - any key closes it
        if self.overlays.help_active {
            self.overlays.help_active = false;
            return true;
        }

        // Legend overlay - any key closes it
        if self.overlays.legend_active {
            self.overlays.legend_active = false;
            return true;
        }

        // Recent items overlay - handles navigation and selection
        if self.overlays.recent_items_active {
            return self.handle_recent_items_key(key);
        }

        // v0.17.3 (ADR-036): filter_pending/trait_filter removed - traits no longer in schema

        // Search mode captures all input
        if self.search.active {
            return self.handle_search_key(key);
        }

        // Ctrl modifiers: search nav (n/p) + vertical panel nav (up/down)
        if key
            .modifiers
            .contains(crossterm::event::KeyModifiers::CONTROL)
        {
            match key.code {
                KeyCode::Char('n') => {
                    self.next_search_result();
                    return true;
                },
                KeyCode::Char('p') => {
                    self.prev_search_result();
                    return true;
                },
                // Ctrl+Up/Down: vertical panel switching (Identity↔Content, Props↔Arcs)
                KeyCode::Up => {
                    self.focus = self.focus.up();
                    self.set_status(self.focus.name());
                    return true;
                },
                KeyCode::Down => {
                    self.focus = self.focus.down();
                    self.set_status(self.focus.name());
                    return true;
                },
                _ => {},
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
                self.overlays.help_active = true;
                true
            },

            // Open search (/ = vim-style search)
            KeyCode::Char('/') => {
                self.search.active = true;
                true
            },

            // v0.17.3 (ADR-036): 'f' trait filter keybinding removed

            // Open color legend (F1 = accessible, out of flow)
            KeyCode::F(1) => {
                self.overlays.legend_active = true;
                true
            },

            // Open recent items popup (` = backtick)
            KeyCode::Char('`') => {
                if !self.nav_history.is_empty() {
                    self.overlays.recent_items_active = true;
                    self.overlays.recent_items_cursor = 0;
                }
                true
            },

            // Mode switching: 1-3 global (1=Graph, 2=Views, 3=Nexus)
            KeyCode::Char('1') => {
                // Switch to Graph mode (unified tree view)
                if self.mode != NavMode::Graph {
                    self.save_mode_cursor();
                    self.mode = NavMode::Graph;
                    self.restore_mode_cursor(NavMode::Graph);
                    self.load_yaml_for_current();
                }
                true
            },
            KeyCode::Char('2') => {
                // Switch to Views mode (Schema views explorer)
                if self.mode != NavMode::Views {
                    self.save_mode_cursor();
                    self.mode = NavMode::Views;
                    self.restore_mode_cursor(NavMode::Views);
                }
                true
            },
            KeyCode::Char('3') => {
                // Switch to Nexus mode (hub for Quiz, Stats, Help)
                if self.mode != NavMode::Nexus {
                    self.save_mode_cursor();
                    self.mode = NavMode::Nexus;
                    self.restore_mode_cursor(NavMode::Nexus);
                }
                true
            },

            // Panel navigation: Tab cycles through 5 panels (v0.18.3)
            // Tree [1] → Identity [2] → Content [3] → Props [4] → Arcs [5]
            KeyCode::Tab => {
                self.focus = self.focus.next();
                self.set_status(self.focus.name());
                true
            },
            KeyCode::BackTab => {
                self.focus = self.focus.prev();
                self.set_status(self.focus.name());
                true
            },
            KeyCode::Left => {
                // Left arrow: spatial navigation left (v0.18.3)
                self.focus = self.focus.left();
                self.set_status(self.focus.name());
                true
            },
            KeyCode::Right => {
                // Right arrow: spatial navigation right (v0.18.3)
                self.focus = self.focus.right();
                self.set_status(self.focus.name());
                true
            },
            // NOTE: Up/Down arrows handled below for in-panel navigation (cursor/scroll)
            // Left/Right = panel switching, Up/Down/j/k = in-panel (vim/lazygit pattern)

            // v0.17.3: 't' keybinding removed (SourceTab toggle removed)
            // Content panel now shows context-aware content based on tree selection

            // Enter: toggle collapse/expand (Tree), toggle sections (Content), or expand property (Info)
            KeyCode::Enter => {
                match self.focus {
                    Focus::Tree => {
                        self.toggle_tree_item();
                    },
                    Focus::Identity => {
                        // v0.18.3: Identity panel - no action on Enter yet
                        // Future: could toggle between expanded/collapsed view
                    },
                    Focus::Content => {
                        // v0.17.3: Toggle instance panel sections collapse/expand
                        // Cycles: all expanded → STANDARD collapsed → both collapsed → all expanded
                        if !self.instance_standard_collapsed && !self.instance_specific_collapsed {
                            // Both expanded → collapse STANDARD
                            self.instance_standard_collapsed = true;
                        } else if self.instance_standard_collapsed
                            && !self.instance_specific_collapsed
                        {
                            // STANDARD collapsed → collapse both
                            self.instance_specific_collapsed = true;
                        } else {
                            // Any other state → expand both
                            self.instance_standard_collapsed = false;
                            self.instance_specific_collapsed = false;
                        }
                    },
                    Focus::Props => {
                        // Toggle expanded property text (word-wrap on multiple lines)
                        self.expanded_property = !self.expanded_property;
                    },
                    Focus::Arcs => {
                        // No-op for Arcs focus (future: could navigate to selected arc)
                    },
                }
                true
            },

            // h/l: Panel navigation OR tree toggle (v0.18.3)
            // When in Tree: toggle collapse/expand (existing behavior)
            // When in other panels: linear panel navigation (h=prev, l=next)
            KeyCode::Char('h') => {
                if self.focus == Focus::Tree {
                    self.toggle_tree_item();
                } else {
                    self.focus = self.focus.prev();
                    self.set_status(self.focus.name());
                }
                true
            },
            KeyCode::Char('l') => {
                if self.focus == Focus::Tree {
                    self.toggle_tree_item();
                } else {
                    self.focus = self.focus.next();
                    self.set_status(self.focus.name());
                }
                true
            },
            // Space: toggle collapse/expand (Tree only)
            KeyCode::Char(' ') => {
                if self.focus == Focus::Tree {
                    self.toggle_tree_item();
                }
                true
            },
            KeyCode::Char('H') => {
                self.tree.collapse_all();
                self.tree_cursor = 0;
                self.tree_scroll = 0;
                true
            },
            KeyCode::Char('L') => {
                self.tree.expand_all();
                true
            },

            // Expand/Collapse subtree under cursor (e/c)
            KeyCode::Char('e') | KeyCode::Char('E') if key.modifiers.is_empty() => {
                // E = Expand subtree under cursor
                if self.focus == Focus::Tree {
                    let data_mode = self.is_graph_mode();
                    if let Some(key) =
                        self.tree
                            .collapse_key_at(self.tree_cursor, data_mode, self.hide_empty)
                    {
                        self.tree.expand_subtree(&key);
                    }
                }
                true
            },
            KeyCode::Char('c') => {
                // c = Collapse subtree under cursor (Tree) OR copy property value (Info/Properties)
                if self.focus == Focus::Tree {
                    let data_mode = self.is_graph_mode();
                    if let Some(key) =
                        self.tree
                            .collapse_key_at(self.tree_cursor, data_mode, self.hide_empty)
                    {
                        self.tree.collapse_subtree(&key);
                    }
                } else if self.focus == Focus::Props && self.selected_box == InfoBox::Properties {
                    // v0.13.1: Feature 3 - copy focused property value
                    self.copy_focused_property();
                }
                true
            },

            // Toggle hide empty (0) - only in Data mode
            KeyCode::Char('0') => {
                if self.is_graph_mode() {
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
            },

            // Jump to first/last (vim-style: g/G)
            KeyCode::Char('g') => {
                match self.focus {
                    Focus::Tree => {
                        self.tree_cursor = 0;
                        self.tree_scroll = 0;
                        self.load_yaml_for_current();
                        // Note: Instance loading removed - use Space/Enter to expand
                    },
                    Focus::Identity => {
                        // v0.18.3: Identity panel - no scroll (static content)
                    },
                    Focus::Content => {
                        self.yaml.scroll = 0;
                    },
                    Focus::Props => {
                        self.props_scroll = 0;
                    },
                    Focus::Arcs => {
                        self.arcs_scroll = 0;
                    },
                }
                true
            },
            KeyCode::Char('G') => {
                match self.focus {
                    Focus::Tree => {
                        let max = self.current_item_count().saturating_sub(1);
                        self.tree_cursor = max;
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                        // Note: Instance loading removed - use Space/Enter to expand
                    },
                    Focus::Identity => {
                        // v0.18.3: Identity panel - no scroll (static content)
                    },
                    Focus::Content => {
                        let max_scroll = self.yaml.line_count.saturating_sub(YAML_SCROLL_MARGIN);
                        self.yaml.scroll = max_scroll;
                    },
                    Focus::Props => {
                        let max_scroll = self.props_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                        self.props_scroll = max_scroll;
                    },
                    Focus::Arcs => {
                        let max_scroll = self.arcs_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                        self.arcs_scroll = max_scroll;
                    },
                }
                true
            },

            // Navigation: ↑/k scroll up, ↓/j scroll down (in focused panel)
            KeyCode::Up | KeyCode::Char('k') => {
                match self.focus {
                    Focus::Tree => {
                        if self.tree_cursor > 0 {
                            self.tree_cursor -= 1;
                            self.ensure_cursor_visible();
                            self.load_yaml_for_current();
                        }
                    },
                    Focus::Identity => {
                        // v0.18.3: Identity panel - no scroll (static content)
                    },
                    Focus::Content => {
                        if self.yaml.scroll > 0 {
                            self.yaml.scroll -= 1;
                        }
                    },
                    Focus::Props => {
                        // Navigate properties with j/k
                        if self.schema_overlay.matched_properties.is_some()
                            && self.focused_property_idx > 0
                        {
                            self.focused_property_idx -= 1;
                            self.expanded_property = false;
                        } else if self.props_scroll > 0 {
                            self.props_scroll -= 1;
                        }
                    },
                    Focus::Arcs => {
                        if self.arcs_scroll > 0 {
                            self.arcs_scroll -= 1;
                        }
                    },
                }
                true
            },
            KeyCode::Down | KeyCode::Char('j') => {
                match self.focus {
                    Focus::Tree => {
                        let max = self.current_item_count().saturating_sub(1);
                        if self.tree_cursor < max {
                            self.tree_cursor += 1;
                            self.ensure_cursor_visible();
                            self.load_yaml_for_current();
                        }
                    },
                    Focus::Identity => {
                        // v0.18.3: Identity panel - no scroll (static content)
                    },
                    Focus::Content => {
                        let max_scroll = self.yaml.line_count.saturating_sub(YAML_SCROLL_MARGIN);
                        if self.yaml.scroll < max_scroll {
                            self.yaml.scroll += 1;
                        }
                    },
                    Focus::Props => {
                        // Navigate properties with j/k
                        if let Some(matched) = &self.schema_overlay.matched_properties {
                            let max_idx = matched.len().saturating_sub(1);
                            if self.focused_property_idx < max_idx {
                                self.focused_property_idx += 1;
                                self.expanded_property = false;
                            }
                        } else {
                            let max_scroll =
                                self.props_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                            if self.props_scroll < max_scroll {
                                self.props_scroll += 1;
                            }
                        }
                    },
                    Focus::Arcs => {
                        let max_scroll = self.arcs_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                        if self.arcs_scroll < max_scroll {
                            self.arcs_scroll += 1;
                        }
                    },
                }
                true
            },

            // Page scroll: d/u vim-style
            KeyCode::Char('d') => {
                match self.focus {
                    Focus::Tree => {
                        let max = self.current_item_count().saturating_sub(1);
                        self.tree_cursor = (self.tree_cursor + PAGE_SCROLL_AMOUNT).min(max);
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                    },
                    Focus::Identity => {
                        // v0.18.3: Identity panel - no scroll (static content)
                    },
                    Focus::Content => {
                        let max_scroll = self.yaml.line_count.saturating_sub(YAML_SCROLL_MARGIN);
                        self.yaml.scroll = (self.yaml.scroll + PAGE_SCROLL_AMOUNT).min(max_scroll);
                    },
                    Focus::Props => {
                        let max_scroll = self.props_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                        self.props_scroll =
                            (self.props_scroll + PAGE_SCROLL_AMOUNT).min(max_scroll);
                    },
                    Focus::Arcs => {
                        let max_scroll = self.arcs_line_count.saturating_sub(INFO_SCROLL_MARGIN);
                        self.arcs_scroll = (self.arcs_scroll + PAGE_SCROLL_AMOUNT).min(max_scroll);
                    },
                }
                true
            },
            KeyCode::Char('u') => {
                match self.focus {
                    Focus::Tree => {
                        self.tree_cursor = self.tree_cursor.saturating_sub(PAGE_SCROLL_AMOUNT);
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                    },
                    Focus::Identity => {
                        // v0.18.3: Identity panel - no scroll (static content)
                    },
                    Focus::Content => {
                        self.yaml.scroll = self.yaml.scroll.saturating_sub(PAGE_SCROLL_AMOUNT);
                    },
                    Focus::Props => {
                        self.props_scroll = self.props_scroll.saturating_sub(PAGE_SCROLL_AMOUNT);
                    },
                    Focus::Arcs => {
                        self.arcs_scroll = self.arcs_scroll.saturating_sub(PAGE_SCROLL_AMOUNT);
                    },
                }
                true
            },

            // 'r' key: Jump to ADR if architecture diagram exists, else refresh
            KeyCode::Char('r') => {
                // Check if current item has an architecture diagram
                if let Some(adr_id) = self.get_current_adr_id() {
                    // Find the ADR index
                    let adrs = get_all_adrs();
                    if let Some(idx) = adrs.iter().position(|adr| adr.id == adr_id) {
                        // Switch to Nexus mode, Arch tab
                        self.save_mode_cursor();
                        self.mode = NavMode::Nexus;
                        self.nexus.tab = NexusTab::Arch;
                        self.nexus.arch_adr_index = idx;
                        self.set_status(&format!("Jumped to {}", adr_id));
                        return true;
                    }
                }
                // No diagram or ADR not found — fall back to refresh
                self.pending_refresh = true;
                self.set_status("Refreshing...");
                true
            },

            // Yank (smart copy based on selected box)
            KeyCode::Char('y') => {
                self.yank_selected_box();
                true
            },

            // Yank JSON properties (Y) - legacy, kept for compatibility
            KeyCode::Char('Y') => {
                self.yank_current_json();
                true
            },

            // Jump to parent [p]
            KeyCode::Char('p') => {
                if let Some(parent_cursor) = self.tree.find_parent_cursor(
                    self.tree_cursor,
                    self.is_graph_mode(),
                    self.hide_empty,
                ) {
                    self.tree_cursor = parent_cursor;
                    self.ensure_cursor_visible();
                    self.set_status("↑ Parent");
                }
                true
            },

            // Toggle schema overlay (s) - only in Data mode
            KeyCode::Char('s') => {
                if self.is_graph_mode() {
                    self.schema_overlay.enabled = !self.schema_overlay.enabled;
                    // Load/clear matched properties based on new state
                    self.update_schema_match_for_current();
                    self.set_status(if self.schema_overlay.enabled {
                        "Schema overlay ON"
                    } else {
                        "Schema overlay OFF"
                    });
                }
                true
            },

            // Toggle JSON pretty-print (J) - only when viewing properties
            KeyCode::Char('J') => {
                self.json_pretty = !self.json_pretty;
                self.set_status(if self.json_pretty {
                    "JSON pretty-print ON"
                } else {
                    "JSON compact mode"
                });
                true
            },

            // Property focus navigation (+/-) - Feature 3: Truncate Intelligent
            // Navigate focused property in schema overlay
            KeyCode::Char('+') | KeyCode::Char('=') => {
                if self.is_graph_mode() && self.schema_overlay.enabled {
                    if let Some(matched) = &self.schema_overlay.matched_properties {
                        let max_idx = matched.len().saturating_sub(1);
                        self.focused_property_idx = (self.focused_property_idx + 1).min(max_idx);
                        self.expanded_property = false; // Collapse when changing property
                    }
                }
                true
            },
            KeyCode::Char('-') | KeyCode::Char('_') => {
                if self.is_graph_mode() && self.schema_overlay.enabled {
                    self.focused_property_idx = self.focused_property_idx.saturating_sub(1);
                    self.expanded_property = false; // Collapse when changing property
                }
                true
            },

            // Navigation history: back (Ctrl+o)
            KeyCode::Char('o')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.nav_back();
                true
            },

            // Navigation history: forward (Ctrl+i)
            KeyCode::Char('i')
                if key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL) =>
            {
                self.nav_forward();
                true
            },

            // Esc: exit filtered mode
            KeyCode::Esc => {
                if self.is_filtered_graph_mode() {
                    self.exit_filtered_data_mode();
                    return true;
                }
                false
            },

            // Open YAML in external editor (O = shift+o)
            KeyCode::Char('O') => {
                self.open_yaml_in_editor();
                true
            },

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
            },

            // Select result
            KeyCode::Enter => {
                self.select_search_result();
                true
            },

            // Navigate results (arrow keys and vim j/k)
            KeyCode::Up | KeyCode::Char('k') => {
                if self.search.cursor > 0 {
                    self.search.cursor -= 1;
                }
                true
            },
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.search.results.len().saturating_sub(1);
                if self.search.cursor < max {
                    self.search.cursor += 1;
                }
                true
            },

            // Type character (j/k are handled above for navigation)
            // Security: Limit search query length to prevent memory exhaustion
            KeyCode::Char(c) => {
                const MAX_SEARCH_LEN: usize = 256;
                if self.search.query.len() < MAX_SEARCH_LEN {
                    self.search.query.push(c);
                    self.update_search();
                }
                true
            },

            // Delete character
            KeyCode::Backspace => {
                self.search.query.pop();
                self.update_search();
                true
            },

            _ => false,
        }
    }

    /// Handle key events for the recent items popup.
    fn handle_recent_items_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            // Close popup
            KeyCode::Esc | KeyCode::Char('`') => {
                self.overlays.recent_items_active = false;
                true
            },

            // Select and jump to item
            KeyCode::Enter => {
                self.select_recent_item();
                true
            },

            // Navigate up (arrows, vim j/k)
            KeyCode::Up | KeyCode::Char('k') => {
                if self.overlays.recent_items_cursor > 0 {
                    self.overlays.recent_items_cursor -= 1;
                }
                true
            },

            // Navigate down
            KeyCode::Down | KeyCode::Char('j') => {
                let max = self.nav_history.len().saturating_sub(1);
                if self.overlays.recent_items_cursor < max {
                    self.overlays.recent_items_cursor += 1;
                }
                true
            },

            _ => true, // Consume all other keys while popup is open
        }
    }

    /// Select and jump to the currently highlighted recent item.
    fn select_recent_item(&mut self) {
        // History is stored oldest→newest, but we display newest first
        let display_idx = self.overlays.recent_items_cursor;
        let history_idx = self.nav_history.len().saturating_sub(1 + display_idx);

        if let Some(&(mode, cursor)) = self.nav_history.get(history_idx) {
            self.overlays.recent_items_active = false;
            self.mode = mode;
            self.tree_cursor = cursor;
            self.ensure_cursor_visible();
            self.load_yaml_for_current();
            self.set_status("↩ Jumped to recent item");
        }
    }

    // =========================================================================
    // Mode & Navigation Helpers
    // =========================================================================

    /// Check if currently in Graph mode (unified tree that shows instances).
    /// v11.7: Renamed from is_graph_mode() for clarity — Graph mode IS the unified view.
    pub fn is_graph_mode(&self) -> bool {
        self.mode == NavMode::Graph
    }

    /// Save current cursor to mode_cursors for the current mode.
    pub fn save_mode_cursor(&mut self) {
        self.mode_cursors[self.mode.index()] = self.tree_cursor;
    }

    /// Restore cursor from mode_cursors for the new mode.
    fn restore_mode_cursor(&mut self, new_mode: NavMode) {
        // v0.16.5: Clamp restored cursor to valid bounds for the new mode
        let restored = self.mode_cursors[new_mode.index()];
        let max_cursor = self.current_item_count().saturating_sub(1);
        self.tree_cursor = restored.min(max_cursor);
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

    /// Open the current YAML file in external editor.
    /// v0.17.3: Uses $EDITOR environment variable, falls back to 'code' then 'vim'.
    /// Returns true if editor was launched, false if no YAML file is available.
    pub fn open_yaml_in_editor(&mut self) -> bool {
        if self.yaml.path.is_empty() {
            self.set_status("No YAML file to open");
            return false;
        }

        // Build full path
        let full_path = format!("{}/{}", self.root_path, self.yaml.path);

        // Check if file exists
        if !Path::new(&full_path).exists() {
            self.set_status(&format!("File not found: {}", self.yaml.path));
            return false;
        }

        // Helper to check if command exists in PATH
        fn command_exists(cmd: &str) -> bool {
            std::process::Command::new("which")
                .arg(cmd)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }

        // Get editor from environment, with fallbacks
        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| {
                // Try common editors in order of preference
                if command_exists("code") {
                    "code".to_string()
                } else if command_exists("vim") {
                    "vim".to_string()
                } else if command_exists("nano") {
                    "nano".to_string()
                } else {
                    "vi".to_string()
                }
            });

        // Launch editor in background
        match std::process::Command::new(&editor).arg(&full_path).spawn() {
            Ok(_) => {
                self.set_status(&format!("Opened in {}", editor));
                true
            },
            Err(e) => {
                self.set_status_error(&format!("Failed to open editor: {}", e));
                false
            },
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

    // =========================================================================
    // Clipboard Methods
    // =========================================================================

    /// Yank (copy) the current item's key to clipboard.
    pub fn yank_current_key(&mut self) {
        use super::data::TreeItem;
        let key = match self.current_item() {
            Some(TreeItem::Realm(r)) => Some(r.key.clone()),
            Some(TreeItem::Layer(_, l)) => Some(l.key.clone()),
            Some(TreeItem::Class(_, _, k)) => Some(k.key.clone()),
            Some(TreeItem::ArcFamily(f)) => Some(f.key.clone()),
            Some(TreeItem::ArcClass(_, a)) => Some(a.key.clone()),
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
            },
            Some(TreeItem::Class(_, _, class_info)) => {
                // For Class, show properties schema
                Some(format!(
                    "{{\"properties\": {:?}, \"required\": {:?}}}",
                    class_info.properties, class_info.required_properties
                ))
            },
            _ => None,
        };
        if let Some(json) = json {
            // Truncate for status display (UTF-8 safe using char boundaries)
            let preview = if json.chars().count() > 50 {
                let truncated: String = json.chars().take(50).collect();
                format!("{}...", truncated)
            } else {
                json
            };
            self.set_status(&format!("JSON: {}", preview));
        }
    }

    /// Yank (copy) content based on the selected box (smart copy).
    pub fn yank_selected_box(&mut self) {
        use super::clipboard::{copy_to_clipboard, get_box_content};
        if let Some((content, format_name)) = get_box_content(self) {
            match copy_to_clipboard(&content) {
                Ok(()) => {
                    // Show preview of copied content (UTF-8 safe)
                    let preview = if content.chars().count() > 40 {
                        let truncated: String = content.chars().take(40).collect();
                        format!("{}...", truncated)
                    } else {
                        content.clone()
                    };
                    self.set_status(&format!("✓ {} copied: {}", format_name, preview));
                },
                Err(e) => {
                    self.set_status(&format!("✗ Copy failed: {}", e));
                },
            }
        } else {
            self.set_status("Nothing to copy");
        }
    }

    /// Copy focused property value to clipboard (c key in Properties box).
    /// v0.13.1: Feature 3 - single property copy.
    pub fn copy_focused_property(&mut self) {
        use super::clipboard::{copy_to_clipboard, get_focused_property};
        if let Some((key, value)) = get_focused_property(self) {
            // Truncate preview for long values (UTF-8 safe)
            let preview = if value.chars().count() > 30 {
                let truncated: String = value.chars().take(30).collect();
                format!("{}...", truncated)
            } else {
                value.clone()
            };
            match copy_to_clipboard(&value) {
                Ok(()) => {
                    self.set_status(&format!("✓ Copied: {} = {}", key, preview));
                },
                Err(e) => {
                    self.set_status(&format!("✗ Copy failed: {}", e));
                },
            }
        } else {
            self.set_status("No property selected");
        }
    }

    // =========================================================================
    // Query Methods
    // =========================================================================

    /// Check if in filtered Graph mode (drilling into a specific Class).
    /// v11.7: Renamed from is_filtered_graph_mode() for consistency.
    pub fn is_filtered_graph_mode(&self) -> bool {
        self.is_graph_mode() && self.data_filter_class.is_some()
    }

    /// Get the current filter Class key (if in filtered Graph mode).
    pub fn get_filter_class(&self) -> Option<&str> {
        self.data_filter_class.as_deref()
    }

    /// Get item at cursor position for the current mode.
    /// Uses mode-aware method that shows instances in Data mode.
    pub fn current_item(&self) -> Option<super::data::TreeItem<'_>> {
        // Filtered Data mode: show only instances of the filtered Class
        if let Some(class_key) = &self.data_filter_class {
            if self.is_graph_mode() {
                return self.tree.filtered_item_at(self.tree_cursor, class_key);
            }
        }
        // Normal mode
        // v0.17.3: Pass hide_empty to match render_tree filtering
        if self.is_graph_mode() {
            self.tree
                .item_at_for_mode(self.tree_cursor, true, self.hide_empty)
        } else {
            // v0.17.3 (ADR-036): Meta mode - trait filtering removed
            self.tree.item_at(self.tree_cursor)
        }
    }

    /// Get total item count for the current mode.
    /// v0.17.3: Pass hide_empty to match render_tree and item_at_for_mode filtering.
    pub fn current_item_count(&self) -> usize {
        // Filtered Data mode: count only instances of the filtered Class
        if let Some(class_key) = &self.data_filter_class {
            if self.is_graph_mode() {
                return self.tree.filtered_item_count(class_key);
            }
        }
        // Normal mode
        if self.is_graph_mode() {
            self.tree.item_count_for_mode(true, self.hide_empty)
        } else {
            // v0.17.3 (ADR-036): Meta mode - trait filtering removed
            self.tree.item_count()
        }
    }

    /// Get the ADR ID for the current item's architecture diagram (if any).
    /// Returns None if the current item doesn't have an associated ADR diagram.
    /// Used by 'r' keybinding to jump from Graph mode to Nexus Arch tab.
    pub fn get_current_adr_id(&self) -> Option<String> {
        // Get class name from current tree item
        let class_key = match self.current_item() {
            Some(TreeItem::Class(_, _, info)) => Some(info.key.as_str()),
            Some(TreeItem::Instance(_, _, class_info, _)) => Some(class_info.key.as_str()),
            _ => None,
        }?;

        // Get architecture diagram for this class
        let diagram = get_architecture_diagram(class_key)?;
        Some(diagram.adr_id.clone())
    }

    /// Enter filtered Data mode for a specific Class.
    /// Saves cursor position and resets to 0.
    /// Also resets all scroll states to avoid stale positions.
    #[allow(dead_code)]
    pub fn enter_filtered_data_mode(&mut self, class_key: String) {
        self.data_cursor_before_filter = self.tree_cursor;
        self.data_filter_class = Some(class_key.clone());
        self.tree_cursor = 0;
        self.tree_scroll = 0;
        // Reset other scroll states to avoid stale positions
        self.props_scroll = 0;
        self.arcs_scroll = 0;
        self.yaml.scroll = 0;
        // Request instance load if not already loaded
        if self.tree.get_instances(&class_key).is_none() {
            self.pending.instance = Some(class_key);
        }
    }

    /// Get breadcrumb path for the current selection.
    /// Returns a string like "Org > Foundation > Entity (12)"
    pub fn current_breadcrumb(&self) -> String {
        use super::data::TreeItem;
        match self.current_item() {
            Some(TreeItem::ClassesSection) => "Node Classes".to_string(),
            Some(TreeItem::ArcsSection) => "Arcs".to_string(),
            Some(TreeItem::Realm(r)) => r.display_name.clone(),
            Some(TreeItem::Layer(r, l)) => {
                format!("{} → {}", r.display_name, l.display_name)
            },
            Some(TreeItem::Class(r, l, k)) => {
                // v0.17.3 (ADR-036): trait removed from breadcrumb display
                if self.is_graph_mode() && k.instance_count > 0 {
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
            },
            Some(TreeItem::Instance(r, l, k, inst)) => {
                format!(
                    "{} → {} → {} → {}",
                    r.display_name, l.display_name, k.display_name, inst.display_name
                )
            },
            Some(TreeItem::EntityCategory(r, l, k, cat)) => {
                format!(
                    "{} → {} → {} → {}",
                    r.display_name, l.display_name, k.display_name, cat.display_name
                )
            },
            Some(TreeItem::LocaleGroup(r, l, k, group)) => {
                format!(
                    "{} → {} → {} → {} {}",
                    r.display_name, l.display_name, k.display_name, group.flag, group.locale_name
                )
            },
            // v0.17.3: EntityGroup breadcrumb shows entity name
            Some(TreeItem::EntityGroup(r, l, k, group)) => {
                format!(
                    "{} → {} → {} → {}",
                    r.display_name, l.display_name, k.display_name, group.entity_display_name
                )
            },
            Some(TreeItem::ArcFamily(f)) => format!("Arcs → {}", f.display_name),
            Some(TreeItem::ArcClass(f, ak)) => {
                format!("Arcs → {} → {}", f.display_name, ak.display_name)
            },
            Some(TreeItem::EntityNativeItem(r, l, k, native)) => {
                format!(
                    "{} → {} → {} → {}",
                    r.display_name, l.display_name, k.display_name, native.display_name
                )
            },
            None => "NovaNet".to_string(),
        }
    }

    /// Exit filtered Data mode, restore cursor position.
    /// Clamps cursor to valid range in case tree structure changed.
    #[allow(dead_code)]
    pub fn exit_filtered_data_mode(&mut self) {
        if self.data_filter_class.is_some() {
            self.data_filter_class = None;
            self.pending.instance = None; // Clear pending to prevent race condition
            // Clamp cursor to valid range before restoring
            let max_cursor = self.tree.item_count().saturating_sub(1);
            self.tree_cursor = self.data_cursor_before_filter.min(max_cursor);
            self.ensure_cursor_visible();
        }
    }

    /// Request instance loading for the currently selected Class.
    /// Sets `pending_instance_load` if a Class is selected and we're in Data mode.
    pub fn request_instance_load_for_current(&mut self) {
        if !self.is_graph_mode() {
            return;
        }

        // Check if current item is a Class
        if let Some(super::data::TreeItem::Class(_, _, class_info)) =
            self.tree.item_at(self.tree_cursor)
        {
            // Only request if not already loaded
            if self.tree.get_instances(&class_info.key).is_none() {
                self.pending.instance = Some(class_info.key.clone());
            }
        }

        // Also update schema match if on an instance
        self.update_schema_match_for_current();
    }

    /// Toggle collapse/expand of the current tree item.
    /// Also triggers loading for instances, Entity categories, and category instances in Data mode.
    /// Single-click behavior: if instances not loaded, load them AND expand in one action.
    fn toggle_tree_item(&mut self) {
        let data_mode = self.is_graph_mode();

        if let Some(key) = self
            .tree
            .collapse_key_at(self.tree_cursor, data_mode, self.hide_empty)
        {
            // Handle Class toggle in Data mode
            if let Some(class_key) = key.strip_prefix("class:") {
                if data_mode {
                    // v0.17.3: Use helpers for Entity/EntityNative dual storage pattern
                    let instances_loaded = if class_key == "Entity" {
                        self.tree.has_entity_instances()
                    } else if class_key == "EntityNative" {
                        !self.tree.entity_native_groups.is_empty()
                    } else {
                        self.tree.get_instances(class_key).is_some()
                    };

                    if !instances_loaded {
                        // First click on unloaded Class: load instances AND ensure expanded
                        // v0.17.3: Entity uses flat instances (same as regular classes)
                        if class_key == "Entity" {
                            // Load flat Entity instances
                            self.pending.instance = Some("Entity".to_string());
                        } else if class_key == "EntityNative" {
                            if self.tree.entity_native_groups.is_empty() {
                                self.pending.entity_natives = true;
                            }
                            // EntityNative uses entity-grouped display, not flat instances
                        } else {
                            // Regular classes: use flat instance loading
                            self.pending.instance = Some(class_key.to_string());
                        }
                        // Ensure state is "expanded" so instances show when loaded
                        if self.tree.is_collapsed(&key) {
                            self.tree.toggle(&key);
                        }
                    } else {
                        // Instances loaded: normal toggle
                        self.tree.toggle(&key);
                    }
                } else {
                    // Meta mode: normal toggle (Classes don't expand in schema)
                    self.tree.toggle(&key);
                }
            }
            // Handle EntityCategory toggle
            else if let Some(category_key) = key.strip_prefix("category:") {
                if data_mode {
                    let instances_loaded = self
                        .tree
                        .entity_category_instances
                        .contains_key(category_key);

                    if !instances_loaded {
                        // First click: load category instances AND ensure expanded
                        self.pending.category_instances = Some(category_key.to_string());
                        if self.tree.is_collapsed(&key) {
                            self.tree.toggle(&key);
                        }
                    } else {
                        // Instances loaded: normal toggle
                        self.tree.toggle(&key);
                    }
                } else {
                    self.tree.toggle(&key);
                }
            }
            // Handle LocaleGroup toggle (EntityNative locale groups)
            else if key.starts_with("locale:") {
                // LocaleGroup items are already loaded when EntityNative class was expanded
                // Just toggle expand/collapse
                self.tree.toggle(&key);
            }
            // Other items (Realm, Layer, ArcFamily, etc.): normal toggle
            else {
                self.tree.toggle(&key);
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
        if !self.is_graph_mode() || !self.schema_overlay.enabled {
            self.schema_overlay.matched_properties = None;
            self.schema_overlay.coverage_stats = None;
            return;
        }

        // Check if current item is an Instance
        // v0.17.3: Pass hide_empty to match render_tree filtering
        // Clone properties to avoid borrow conflict
        let props = if let Some(super::data::TreeItem::Instance(_, _, _, instance)) = self
            .tree
            .item_at_for_mode(self.tree_cursor, true, self.hide_empty)
        {
            Some(instance.properties.clone())
        } else {
            None
        };

        if let Some(properties) = props {
            self.load_matched_properties(&properties);
        } else {
            self.schema_overlay.matched_properties = None;
            self.schema_overlay.coverage_stats = None;
        }
    }

    // =========================================================================
    // Async Load Helpers
    // =========================================================================

    /// Check and clear pending instance load request.
    /// Returns the Class label to load, if any.
    pub fn take_pending_instance_load(&mut self) -> Option<String> {
        self.pending.instance.take()
    }

    /// Take the pending arcs load request (returns Class label if one was queued).
    pub fn take_pending_arcs_load(&mut self) -> Option<String> {
        self.pending.arcs.take()
    }

    /// Take the pending instance arcs load request.
    /// Returns (Class label, instance keys) to load arcs for.
    pub fn take_pending_instance_arcs_load(&mut self) -> Option<(String, Vec<String>)> {
        self.pending.instance_arcs.take()
    }

    /// Take the pending entity categories load request.
    /// Returns true if categories need to be loaded.
    pub fn take_pending_entity_categories_load(&mut self) -> bool {
        std::mem::take(&mut self.pending.entity_categories)
    }

    /// Take the pending category instances load request.
    /// Returns the category key if one was queued.
    pub fn take_pending_category_instances_load(&mut self) -> Option<String> {
        self.pending.category_instances.take()
    }

    /// Take the pending entity natives load request.
    /// Returns true if EntityNative locale groups need to be loaded.
    pub fn take_pending_entity_natives_load(&mut self) -> bool {
        std::mem::take(&mut self.pending.entity_natives)
    }

    /// Set the loaded Class arcs data from Neo4j.
    pub fn set_class_arcs(&mut self, arcs: ClassArcsData) {
        self.details.class_arcs = Some(arcs);
    }

    /// Take the pending arc class details load request (returns Arc key if one was queued).
    pub fn take_pending_arc_class_load(&mut self) -> Option<String> {
        self.pending.arc_class.take()
    }

    /// Set the loaded ArcClass details from Neo4j.
    pub fn set_arc_class_details(&mut self, details: ArcClassDetails) {
        self.details.arc_class = Some(details);
    }

    /// Take the pending Realm details load request (returns Realm key if one was queued).
    pub fn take_pending_realm_load(&mut self) -> Option<String> {
        self.pending.realm.take()
    }

    /// Set the loaded Realm details from Neo4j.
    pub fn set_realm_details(&mut self, details: RealmDetails) {
        self.details.realm = Some(details);
    }

    /// Take the pending Layer details load request (returns Layer key if one was queued).
    pub fn take_pending_layer_load(&mut self) -> Option<String> {
        self.pending.layer.take()
    }

    /// Set the loaded Layer details from Neo4j.
    pub fn set_layer_details(&mut self, details: LayerDetails) {
        self.details.layer = Some(details);
    }

    /// Check if any data is currently being loaded from Neo4j.
    /// Used to trigger animation re-renders during loading.
    pub fn has_pending_load(&self) -> bool {
        self.pending.instance.is_some()
            || self.pending.arcs.is_some()
            || self.pending.instance_arcs.is_some()
            || self.pending.arc_class.is_some()
            || self.pending.realm.is_some()
            || self.pending.layer.is_some()
            || self.pending.entity_categories
            || self.pending.category_instances.is_some()
    }

    /// Check if any overlay (help, legend, search, recent) is currently open.
    /// Used to prevent 'q' from quitting while overlays are active.
    pub fn has_overlay_open(&self) -> bool {
        self.overlays.help_active
            || self.overlays.legend_active
            || self.search.active
            || self.overlays.recent_items_active
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
        if !self.is_graph_mode() || !self.schema_overlay.enabled {
            self.schema_overlay.matched_properties = None;
            self.schema_overlay.coverage_stats = None;
            return;
        }

        // Need the Class's YAML path to load schema
        if self.yaml.path.is_empty() {
            self.schema_overlay.matched_properties = None;
            self.schema_overlay.coverage_stats = None;
            return;
        }

        // Load schema from YAML
        let schema = load_schema_properties(&self.root_path, &self.yaml.path);
        if schema.is_empty() {
            self.schema_overlay.matched_properties = None;
            self.schema_overlay.coverage_stats = None;
            return;
        }

        // Match properties
        let matched = match_properties(&schema, instance_props);
        let stats = CoverageStats::from_matched(&matched);

        self.schema_overlay.matched_properties = Some(matched);
        self.schema_overlay.coverage_stats = Some(stats);
    }

    // ==========================================================================
    // Class Validation (Neo4j ↔ YAML)
    // ==========================================================================

    /// Load validated properties for the current Class (compares Neo4j vs YAML).
    /// Called when selecting a Class in Meta mode to show validation status.
    /// Uses cached YAML content to avoid redundant file I/O.
    pub fn load_validated_class_properties(&mut self, class_properties: &[String]) {
        use super::schema::{ValidationStats, parse_schema_properties, validate_class_properties};

        // Need the Class's YAML path to load schema
        if self.yaml.path.is_empty() {
            return; // State already cleared in load_yaml_for_current()
        }

        // Use cached YAML content (already loaded by load_yaml_cached)
        let yaml_content = match self.yaml_cache.get(&self.yaml.path) {
            Some(content) => content,
            None => {
                tracing::warn!(path = %self.yaml.path, "YAML not in cache for Class validation");
                return;
            },
        };

        // Parse schema from cached YAML content
        let schema = parse_schema_properties(yaml_content);
        if schema.is_empty() {
            tracing::debug!(path = %self.yaml.path, "No schema properties found in YAML");
            return;
        }

        // Validate: compare YAML schema against Neo4j properties
        let validated = validate_class_properties(&schema, class_properties);
        let stats = ValidationStats::from_validated(&validated);

        self.schema_overlay.validated_class_properties = Some(validated);
        self.schema_overlay.validation_stats = Some(stats);
    }
}
