//! App state for TUI v2.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};

use super::data::{
    ArcKindDetails, KindArcsData, LayerDetails, RealmDetails, TaxonomyTree, TreeItem,
};

/// Navigation mode (matches Studio).
/// Order: 1:Meta 2:Data 3:Overlay 4:Query
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavMode {
    #[default]
    Meta,
    Data,
    Overlay,
    Query,
}

impl NavMode {
    pub fn label(&self) -> &'static str {
        match self {
            NavMode::Data => "Data",
            NavMode::Meta => "Meta",
            NavMode::Overlay => "Overlay",
            NavMode::Query => "Query",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            NavMode::Meta => NavMode::Data,
            NavMode::Data => NavMode::Overlay,
            NavMode::Overlay => NavMode::Query,
            NavMode::Query => NavMode::Meta,
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

/// Type of node in the graph visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphNodeType {
    Realm,       // Parent realm (hierarchy)
    Layer,       // Parent layer (hierarchy)
    Kind,        // Semantic neighbor (arc target)
    ArcEndpoint, // From/to endpoint of an ArcKind
}

/// Position hint for graph visualization layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphPosition {
    Top,    // Parent hierarchy (Realm, Layer)
    Bottom, // Child nodes or outgoing arcs
    Left,   // Incoming arcs
    Right,  // Outgoing arcs
}

/// A node in the graph visualization.
#[allow(dead_code)] // Fields reserved for future graph visualization enhancements
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub key: String,
    pub display_name: String,
    pub node_type: GraphNodeType,
    pub position: GraphPosition,
    pub arc_label: Option<String>, // Arc type label (e.g., "HAS_LAYER", "HAS_PAGE")
    pub color: Option<String>,     // Hex color from taxonomy
}

/// Main app state.
#[allow(dead_code)]
pub struct App {
    pub mode: NavMode,
    pub focus: Focus,
    pub tree_cursor: usize,
    pub tree_scroll: usize, // Scroll offset for tree
    pub tree_height: usize, // Visible height (set by UI)
    pub tree: TaxonomyTree,
    // Search state
    pub search_active: bool,
    pub search_query: String,
    pub search_results: Vec<usize>, // indices into flattened tree
    pub search_cursor: usize,
    // Help overlay
    pub help_active: bool,
    // YAML preview
    pub yaml_content: String,
    pub yaml_path: String,
    pub yaml_scroll: usize,
    pub yaml_line_count: usize, // Cached line count (avoids per-scroll recomputation)
    // Info panel scroll (separate from yaml)
    pub info_scroll: usize,
    pub info_line_count: usize, // Set by UI after building lines
    pub root_path: String,
    /// Cache of YAML file contents (path -> content).
    /// Avoids re-reading files on every scroll/navigation.
    pub yaml_cache: HashMap<String, String>,
    // Graph panel state (display-only, no internal navigation)
    pub graph_nodes: Vec<GraphNode>, // Neighbors of currently selected node (YAML-based, legacy)
    /// Neo4j arc data for current Kind (loaded async)
    pub kind_arcs: Option<KindArcsData>,
    /// Neo4j arc kind details (loaded async when ArcKind selected)
    pub arc_kind_details: Option<ArcKindDetails>,
    // Data view: pending instance load request (Kind label to load)
    pub pending_instance_load: Option<String>,
    /// Pending Kind arcs load request (Kind label to load from Neo4j)
    pub pending_arcs_load: Option<String>,
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
}

impl App {
    pub fn new(tree: TaxonomyTree, root_path: String) -> Self {
        let mut app = Self {
            mode: NavMode::Meta,
            focus: Focus::Tree,
            tree_cursor: 0,
            tree_scroll: 0,
            tree_height: 20, // Default, updated by UI
            tree,
            search_active: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_cursor: 0,
            help_active: false,
            yaml_content: String::new(),
            yaml_path: String::new(),
            yaml_scroll: 0,
            yaml_line_count: 0,
            info_scroll: 0,
            info_line_count: 0,
            root_path,
            yaml_cache: HashMap::new(),
            graph_nodes: Vec::new(),
            kind_arcs: None,
            arc_kind_details: None,
            pending_instance_load: None,
            pending_arcs_load: None,
            pending_arc_kind_load: None,
            pending_realm_load: None,
            pending_layer_load: None,
            realm_details: None,
            layer_details: None,
            data_filter_kind: None,
            data_cursor_before_filter: 0,
        };
        app.load_yaml_for_current();
        app
    }

    /// Load YAML content for the current cursor position.
    pub fn load_yaml_for_current(&mut self) {
        // Reset scroll positions when changing items
        self.yaml_scroll = 0;
        self.info_scroll = 0;

        // Build graph nodes for the current selection (legacy YAML-based)
        self.build_graph_nodes();

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

        // Extract data before mutable borrow (to avoid borrow checker issues)
        let kind_info = match self.tree.item_at(self.tree_cursor) {
            Some(TreeItem::Kind(_, _, kind)) => Some((kind.yaml_path.clone(), kind.key.clone())),
            _ => None,
        };

        let arc_kind_info = match self.tree.item_at(self.tree_cursor) {
            Some(TreeItem::ArcKind(family, arc)) => {
                let arc_file = arc.key.to_lowercase().replace('_', "-");
                let path = format!(
                    "packages/core/models/arc-kinds/{}/{}.yaml",
                    family.key, arc_file
                );
                Some((path, arc.key.clone()))
            }
            _ => None,
        };

        // Handle Kind with Neo4j arcs load
        if let Some((yaml_path, kind_key)) = kind_info {
            self.load_yaml_cached(&yaml_path);
            self.pending_arcs_load = Some(kind_key);
            return;
        }

        // Handle ArcKind with Neo4j details load
        if let Some((yaml_path, arc_key)) = arc_kind_info {
            self.load_yaml_cached(&yaml_path);
            self.pending_arc_kind_load = Some(arc_key);
            return;
        }

        match self.tree.item_at(self.tree_cursor) {
            // Kind and ArcKind are handled above with early return
            Some(TreeItem::Kind(_, _, _)) => unreachable!(),
            Some(TreeItem::ArcKind(_, _)) => unreachable!(),
            // Realm → meta/realms/{key}.yaml + Neo4j details
            Some(TreeItem::Realm(realm)) => {
                let realm_key = realm.key.clone();
                let path = format!("packages/core/models/meta/realms/{}.yaml", realm_key);
                self.load_yaml_cached(&path);
                self.pending_realm_load = Some(realm_key);
            }
            // Layer → meta/layers/{key}.yaml + Neo4j details
            Some(TreeItem::Layer(_, layer)) => {
                let layer_key = layer.key.clone();
                let path = format!("packages/core/models/meta/layers/{}.yaml", layer_key);
                self.load_yaml_cached(&path);
                self.pending_layer_load = Some(layer_key);
            }
            // ArcFamily → meta/arc-families/{key}.yaml
            Some(TreeItem::ArcFamily(family)) => {
                let path = format!("packages/core/models/meta/arc-families/{}.yaml", family.key);
                self.load_yaml_cached(&path);
            }
            // Section headers → taxonomy overview
            Some(TreeItem::KindsSection) | Some(TreeItem::ArcsSection) => {
                self.load_yaml_cached("packages/core/models/taxonomy.yaml");
            }
            // Instance → show JSON properties (handled separately in Data mode)
            Some(TreeItem::Instance(_, _, _, _instance)) => {
                // In Data mode, YAML panel shows JSON properties instead
                // This is handled in ui.rs based on NavMode
                self.yaml_path = "# Instance data".to_string();
                self.yaml_content = "# Instance properties shown as JSON in Data mode".to_string();
                self.yaml_line_count = 1;
            }
            None => {
                self.yaml_path.clear();
                self.yaml_content.clear();
                self.yaml_line_count = 0;
            }
        }
    }

    /// Load YAML content with caching (avoids re-reading files on every navigation).
    fn load_yaml_cached(&mut self, relative_path: &str) {
        self.yaml_path = relative_path.to_string();

        // Check cache first
        if let Some(cached) = self.yaml_cache.get(relative_path) {
            self.yaml_content = cached.clone();
            self.yaml_line_count = self.yaml_content.lines().count();
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
    }

    /// Clear YAML cache (useful after external file modifications).
    #[allow(dead_code)]
    pub fn clear_yaml_cache(&mut self) {
        self.yaml_cache.clear();
    }

    /// Build graph nodes for the currently selected item (display-only).
    /// Supports: Realm, Layer, Kind, ArcKind selections.
    fn build_graph_nodes(&mut self) {
        self.graph_nodes.clear();

        match self.tree.item_at(self.tree_cursor) {
            // Realm → show child Layers
            Some(TreeItem::Realm(realm)) => {
                for layer in &realm.layers {
                    self.graph_nodes.push(GraphNode {
                        key: layer.key.clone(),
                        display_name: layer.display_name.clone(),
                        node_type: GraphNodeType::Layer,
                        position: GraphPosition::Bottom,
                        arc_label: Some("HAS_LAYER".to_string()),
                        color: Some(layer.color.clone()),
                    });
                }
            }

            // Layer → show parent Realm + child Kinds (limited)
            Some(TreeItem::Layer(realm, layer)) => {
                // Parent Realm (top)
                self.graph_nodes.push(GraphNode {
                    key: realm.key.clone(),
                    display_name: realm.display_name.clone(),
                    node_type: GraphNodeType::Realm,
                    position: GraphPosition::Top,
                    arc_label: Some("HAS_LAYER".to_string()),
                    color: Some(realm.color.clone()),
                });

                // Child Kinds (bottom, limited to 8 to avoid clutter)
                for kind in layer.kinds.iter().take(8) {
                    self.graph_nodes.push(GraphNode {
                        key: kind.key.clone(),
                        display_name: kind.display_name.clone(),
                        node_type: GraphNodeType::Kind,
                        position: GraphPosition::Bottom,
                        arc_label: Some("HAS_KIND".to_string()),
                        color: None, // Use layer color
                    });
                }
            }

            // Kind → show hierarchy (Realm, Layer) + semantic arcs
            Some(TreeItem::Kind(realm, layer, kind)) => {
                // Grandparent Realm (top-left)
                self.graph_nodes.push(GraphNode {
                    key: realm.key.clone(),
                    display_name: realm.display_name.clone(),
                    node_type: GraphNodeType::Realm,
                    position: GraphPosition::Top,
                    arc_label: None, // Implicit hierarchy
                    color: Some(realm.color.clone()),
                });

                // Parent Layer (top-right)
                self.graph_nodes.push(GraphNode {
                    key: layer.key.clone(),
                    display_name: layer.display_name.clone(),
                    node_type: GraphNodeType::Layer,
                    position: GraphPosition::Top,
                    arc_label: Some("HAS_KIND".to_string()),
                    color: Some(layer.color.clone()),
                });

                // Semantic arcs (incoming = left, outgoing = right)
                for arc in &kind.arcs {
                    let is_incoming = arc.direction == super::data::ArcDirection::Incoming;
                    self.graph_nodes.push(GraphNode {
                        key: arc.target_kind.clone(),
                        display_name: arc.target_kind.clone(),
                        node_type: GraphNodeType::Kind,
                        position: if is_incoming {
                            GraphPosition::Left
                        } else {
                            GraphPosition::Right
                        },
                        arc_label: Some(arc.rel_type.clone()),
                        color: None,
                    });
                }
            }

            // ArcKind → show from and to endpoint Kinds
            Some(TreeItem::ArcKind(_, arc_kind)) => {
                // From node (left)
                self.graph_nodes.push(GraphNode {
                    key: arc_kind.from_kind.clone(),
                    display_name: arc_kind.from_kind.clone(),
                    node_type: GraphNodeType::ArcEndpoint,
                    position: GraphPosition::Left,
                    arc_label: Some(arc_kind.key.clone()),
                    color: None,
                });
                // To node (right)
                self.graph_nodes.push(GraphNode {
                    key: arc_kind.to_kind.clone(),
                    display_name: arc_kind.to_kind.clone(),
                    node_type: GraphNodeType::ArcEndpoint,
                    position: GraphPosition::Right,
                    arc_label: Some(arc_kind.key.clone()),
                    color: None,
                });
            }

            // Sections don't have graph neighbors
            _ => {}
        }
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

    /// Update search results based on current query (respects collapsed state).
    pub fn update_search(&mut self) {
        self.search_results.clear();
        if self.search_query.is_empty() {
            return;
        }

        let query = self.search_query.to_lowercase();
        let mut idx = 0;

        // Kinds section header
        if "kinds".contains(&query) {
            self.search_results.push(idx);
        }
        idx += 1;

        if !self.tree.is_collapsed("kinds") {
            for realm in &self.tree.realms {
                if realm.display_name.to_lowercase().contains(&query)
                    || realm.key.to_lowercase().contains(&query)
                {
                    self.search_results.push(idx);
                }
                idx += 1;

                if !self.tree.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        if layer.display_name.to_lowercase().contains(&query)
                            || layer.key.to_lowercase().contains(&query)
                        {
                            self.search_results.push(idx);
                        }
                        idx += 1;

                        if !self.tree.is_collapsed(&format!("layer:{}", layer.key)) {
                            for kind in &layer.kinds {
                                if kind.display_name.to_lowercase().contains(&query)
                                    || kind.key.to_lowercase().contains(&query)
                                {
                                    self.search_results.push(idx);
                                }
                                idx += 1;
                            }
                        }
                    }
                }
            }
        }

        // Arcs section header
        if "arcs".contains(&query) {
            self.search_results.push(idx);
        }
        idx += 1;

        if !self.tree.is_collapsed("arcs") {
            for family in &self.tree.arc_families {
                if family.display_name.to_lowercase().contains(&query)
                    || family.key.to_lowercase().contains(&query)
                {
                    self.search_results.push(idx);
                }
                idx += 1;

                if !self.tree.is_collapsed(&format!("family:{}", family.key)) {
                    for arc_kind in &family.arc_kinds {
                        if arc_kind.display_name.to_lowercase().contains(&query)
                            || arc_kind.key.to_lowercase().contains(&query)
                        {
                            self.search_results.push(idx);
                        }
                        idx += 1;
                    }
                }
            }
        }

        // Reset cursor if out of bounds
        if self.search_cursor >= self.search_results.len() {
            self.search_cursor = 0;
        }
    }

    /// Select current search result and close search.
    pub fn select_search_result(&mut self) {
        if let Some(&idx) = self.search_results.get(self.search_cursor) {
            self.tree_cursor = idx;
            self.ensure_cursor_visible();
        }
        self.close_search();
    }

    /// Close search overlay.
    pub fn close_search(&mut self) {
        self.search_active = false;
        self.search_query.clear();
        self.search_results.clear();
        self.search_cursor = 0;
    }

    /// Handle key input. Returns true if state changed (needs re-render).
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Help overlay - any key closes it
        if self.help_active {
            self.help_active = false;
            return true;
        }

        // Search mode captures all input
        if self.search_active {
            return self.handle_search_key(key);
        }

        match key.code {
            // Open help
            KeyCode::Char('/') => {
                self.help_active = true;
                true
            }

            // Open search (f = find)
            KeyCode::Char('f') => {
                self.search_active = true;
                true
            }

            // Mode switching: 1-4 direct (1=Meta, 2=Data, 3=Overlay, 4=Query)
            KeyCode::Char('1') => {
                // If on an Instance in Data mode, navigate to its Kind in Meta mode
                let kind_key_to_focus =
                    if let Some(super::data::TreeItem::Instance(_, _, kind, _)) =
                        self.current_item()
                    {
                        Some(kind.key.clone())
                    } else {
                        None
                    };

                self.exit_filtered_data_mode();
                self.mode = NavMode::Meta;

                // Focus on the Kind if we came from an Instance
                if let Some(kind_key) = kind_key_to_focus {
                    if let Some(cursor) = self.tree.find_kind_cursor(&kind_key) {
                        self.tree_cursor = cursor;
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                    }
                }
                true
            }
            KeyCode::Char('2') => {
                // If on a Kind in Meta mode, drill-down to its instances
                if self.mode == NavMode::Meta {
                    if let Some(super::data::TreeItem::Kind(_, _, kind)) =
                        self.tree.item_at(self.tree_cursor)
                    {
                        let kind_key = kind.key.clone();
                        self.mode = NavMode::Data;
                        self.enter_filtered_data_mode(kind_key);
                        return true;
                    }
                }
                self.exit_filtered_data_mode();
                self.mode = NavMode::Data;
                self.request_instance_load_for_current();
                true
            }
            KeyCode::Char('3') => {
                self.exit_filtered_data_mode();
                self.mode = NavMode::Overlay;
                true
            }
            KeyCode::Char('4') => {
                self.exit_filtered_data_mode();
                self.mode = NavMode::Query;
                true
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.exit_filtered_data_mode();
                self.mode = self.mode.cycle();
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

            // Enter: toggle collapse/expand (Tree only)
            KeyCode::Enter => {
                if self.focus == Focus::Tree {
                    if let Some(key) = self.tree.collapse_key_at(self.tree_cursor) {
                        self.tree.toggle(&key);
                    }
                }
                true
            }

            // Toggle collapse/expand: h/l/Space (Tree only)
            KeyCode::Char('h') | KeyCode::Char('l') | KeyCode::Char(' ') => {
                if self.focus == Focus::Tree {
                    if let Some(key) = self.tree.collapse_key_at(self.tree_cursor) {
                        self.tree.toggle(&key);
                    }
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

            // Navigation: ↑↓ and j/k scroll the focused panel (Graph is display-only)
            KeyCode::Up | KeyCode::Char('k') => {
                match self.focus {
                    Focus::Tree => {
                        if self.tree_cursor > 0 {
                            self.tree_cursor -= 1;
                            self.ensure_cursor_visible();
                            self.load_yaml_for_current();
                            self.request_instance_load_for_current();
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
                            self.request_instance_load_for_current();
                        }
                    }
                    Focus::Info => {
                        let max_scroll = self.info_line_count.saturating_sub(5);
                        if self.info_scroll < max_scroll {
                            self.info_scroll += 1;
                        }
                    }
                    Focus::Graph => {} // Display-only panel, no navigation
                    Focus::Yaml => {
                        let max_scroll = self.yaml_line_count.saturating_sub(10);
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
                        self.tree_cursor = (self.tree_cursor + 10).min(max);
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                        self.request_instance_load_for_current();
                    }
                    Focus::Info => {
                        let max_scroll = self.info_line_count.saturating_sub(5);
                        self.info_scroll = (self.info_scroll + 10).min(max_scroll);
                    }
                    Focus::Graph => {} // Display-only panel, no navigation
                    Focus::Yaml => {
                        let max_scroll = self.yaml_line_count.saturating_sub(10);
                        self.yaml_scroll = (self.yaml_scroll + 10).min(max_scroll);
                    }
                }
                true
            }
            KeyCode::Char('u') => {
                match self.focus {
                    Focus::Tree => {
                        self.tree_cursor = self.tree_cursor.saturating_sub(10);
                        self.ensure_cursor_visible();
                        self.load_yaml_for_current();
                        self.request_instance_load_for_current();
                    }
                    Focus::Info => {
                        self.info_scroll = self.info_scroll.saturating_sub(10);
                    }
                    Focus::Graph => {} // Display-only panel, no navigation
                    Focus::Yaml => {
                        self.yaml_scroll = self.yaml_scroll.saturating_sub(10);
                    }
                }
                true
            }

            // Esc: exit filtered Data mode (if active)
            KeyCode::Esc => {
                if self.is_filtered_data_mode() {
                    self.exit_filtered_data_mode();
                    self.mode = NavMode::Meta;
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

            // Navigate results
            KeyCode::Up => {
                if self.search_cursor > 0 {
                    self.search_cursor -= 1;
                }
                true
            }
            KeyCode::Down => {
                let max = self.search_results.len().saturating_sub(1);
                if self.search_cursor < max {
                    self.search_cursor += 1;
                }
                true
            }

            // Type character
            KeyCode::Char(c) => {
                self.search_query.push(c);
                self.update_search();
                true
            }

            // Delete character
            KeyCode::Backspace => {
                self.search_query.pop();
                self.update_search();
                true
            }

            _ => false,
        }
    }

    /// Check if currently in Data mode.
    pub fn is_data_mode(&self) -> bool {
        self.mode == NavMode::Data
    }

    /// Check if in filtered Data mode (drilling into a specific Kind).
    #[allow(dead_code)]
    pub fn is_filtered_data_mode(&self) -> bool {
        self.is_data_mode() && self.data_filter_kind.is_some()
    }

    /// Get the current filter Kind key (if in filtered Data mode).
    #[allow(dead_code)]
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
            self.tree.item_at(self.tree_cursor)
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
            self.tree.item_count()
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
}

#[cfg(test)]
mod tests {
    use super::super::data::{
        GraphStats, InstanceInfo, KindInfo, LayerInfo, RealmInfo, TaxonomyTree, TreeItem,
    };
    use super::*;
    use rustc_hash::FxHashSet;
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
        };

        let locale_knowledge = LayerInfo {
            key: "locale-knowledge".to_string(),
            display_name: "Locale Knowledge".to_string(),
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
            key: "global".to_string(),
            display_name: "Global".to_string(),
            color: "#859900".to_string(),
            emoji: "🌍",
            layers: vec![locale_knowledge],
        };

        let tenant = RealmInfo {
            key: "tenant".to_string(),
            display_name: "Tenant".to_string(),
            color: "#b58900".to_string(),
            emoji: "🏢",
            layers: vec![structure],
        };

        TaxonomyTree {
            realms: vec![global, tenant],
            arc_families: Vec::new(),
            stats: GraphStats::default(),
            collapsed: FxHashSet::default(),
            instances: BTreeMap::new(),
        }
    }

    // Helper: Create App with test tree
    fn create_test_app() -> App {
        App::new(create_test_tree(), "/test/root".to_string())
    }

    // ========================================================================
    // View toggle tests
    // ========================================================================

    #[test]
    fn test_mode_starts_as_meta() {
        let app = create_test_app();
        assert_eq!(app.mode, NavMode::Meta);
        assert!(!app.is_data_mode());
    }

    #[test]
    fn test_switch_to_data_mode_preserves_cursor() {
        let mut app = create_test_app();

        // Navigate to Locale kind (index 3)
        // Kinds (0), global (1), locale-knowledge (2), Locale (3)
        app.tree_cursor = 3;

        // Verify we're at Locale in Meta mode
        match app.tree.item_at(app.tree_cursor) {
            Some(TreeItem::Kind(_, _, k)) => assert_eq!(k.key, "Locale"),
            other => panic!("Expected Kind Locale, got {:?}", other),
        }

        // Switch to Data mode
        app.mode = NavMode::Data;

        // Cursor should still be at same position
        assert_eq!(app.tree_cursor, 3);

        // Item at cursor should still be Locale kind
        match app.current_item() {
            Some(TreeItem::Kind(_, _, k)) => assert_eq!(k.key, "Locale"),
            other => panic!("Expected Kind Locale in Data mode, got {:?}", other),
        }
    }

    #[test]
    fn test_data_mode_shows_instances_after_kind() {
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
            },
            InstanceInfo {
                key: "en-US".to_string(),
                display_name: "English".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
            },
        ];
        app.tree.set_instances("Locale", instances);

        // Switch to Data mode
        app.mode = NavMode::Data;

        // Item count should include instances
        // Meta: 1 (Kinds) + 1 (global) + 1 (locale-knowledge) + 1 (Locale)
        //       + 1 (tenant) + 1 (structure) + 1 (Page) + 1 (Arcs) = 8
        // Data: + 2 instances = 10
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
    fn test_meta_mode_hides_instances() {
        let mut app = create_test_app();

        // Add instances
        let instances = vec![InstanceInfo {
            key: "fr-FR".to_string(),
            display_name: "Français".to_string(),
            kind_key: "Locale".to_string(),
            properties: BTreeMap::new(),
            outgoing_arcs: vec![],
            incoming_arcs: vec![],
        }];
        app.tree.set_instances("Locale", instances);

        // In Meta mode, instances should not be counted
        assert_eq!(app.current_item_count(), 8); // No instances

        // Position 4 should be tenant (not an instance)
        app.tree_cursor = 4;
        match app.current_item() {
            Some(TreeItem::Realm(r)) => assert_eq!(r.key, "tenant"),
            other => panic!("Expected Realm tenant, got {:?}", other),
        }
    }

    #[test]
    fn test_mode_cycle() {
        let mut app = create_test_app();

        assert_eq!(app.mode, NavMode::Meta);

        app.mode = app.mode.cycle();
        assert_eq!(app.mode, NavMode::Data);

        app.mode = app.mode.cycle();
        assert_eq!(app.mode, NavMode::Overlay);

        app.mode = app.mode.cycle();
        assert_eq!(app.mode, NavMode::Query);

        app.mode = app.mode.cycle();
        assert_eq!(app.mode, NavMode::Meta); // Cycle back
    }

    #[test]
    fn test_key_1_switches_to_meta() {
        let mut app = create_test_app();
        app.mode = NavMode::Data;

        app.handle_key(crossterm::event::KeyEvent::from(KeyCode::Char('1')));

        assert_eq!(app.mode, NavMode::Meta);
    }

    #[test]
    fn test_key_2_switches_to_data() {
        let mut app = create_test_app();

        app.handle_key(crossterm::event::KeyEvent::from(KeyCode::Char('2')));

        assert_eq!(app.mode, NavMode::Data);
    }

    #[test]
    fn test_collapsed_kind_hides_instances_in_data_mode() {
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
            },
            InstanceInfo {
                key: "en-US".to_string(),
                display_name: "English".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
            },
        ];
        app.tree.set_instances("Locale", instances);

        // Switch to Data mode
        app.mode = NavMode::Data;

        // With expanded Locale: 10 items
        assert_eq!(app.current_item_count(), 10);

        // Collapse Locale's instances (using toggle since it starts expanded)
        app.tree.toggle("kind:Locale");

        // Now: 8 items (instances hidden even in Data mode)
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
        app.mode = NavMode::Data; // Must be in Data mode for filtered mode

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
        app.mode = NavMode::Data;

        app.enter_filtered_data_mode("Locale".to_string());
        assert_eq!(app.tree_cursor, 0);

        app.exit_filtered_data_mode();
        assert_eq!(app.tree_cursor, 5);
        assert!(!app.is_filtered_data_mode());
    }

    #[test]
    fn test_filtered_mode_exit_clamps_cursor_to_bounds() {
        let mut app = create_test_app();
        app.mode = NavMode::Data;
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
        app.mode = NavMode::Data;
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
        app.mode = NavMode::Data;

        // Add instances to Locale kind
        let instances = vec![
            InstanceInfo {
                key: "fr-FR".to_string(),
                display_name: "Français".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
            },
            InstanceInfo {
                key: "en-US".to_string(),
                display_name: "English".to_string(),
                kind_key: "Locale".to_string(),
                properties: BTreeMap::new(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
            },
        ];
        app.tree.set_instances("Locale", instances);

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
        app.mode = NavMode::Data;
        app.enter_filtered_data_mode("Locale".to_string());

        assert!(app.is_filtered_data_mode());

        // Press Esc
        let handled = app.handle_key(crossterm::event::KeyEvent::from(KeyCode::Esc));

        assert!(handled);
        assert!(!app.is_filtered_data_mode());
        assert_eq!(app.mode, NavMode::Meta); // Switched back to Meta
    }

    #[test]
    fn test_key_1_exits_filtered_mode() {
        let mut app = create_test_app();
        app.mode = NavMode::Data;
        app.enter_filtered_data_mode("Locale".to_string());

        assert!(app.is_filtered_data_mode());

        // Press 1 (switch to Meta)
        app.handle_key(crossterm::event::KeyEvent::from(KeyCode::Char('1')));

        assert!(!app.is_filtered_data_mode());
        assert_eq!(app.mode, NavMode::Meta);
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
}
