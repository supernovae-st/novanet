//! App state for TUI v2.
//!
//! Refactored into submodules:
//! - `constants`: Scroll amounts, margins, UI defaults
//! - `content`: YAML loading, content panel mode, tree item data extraction
//! - `input`: Key/mouse event handlers
//! - `search`: Nucleo fuzzy search logic
//! - `state`: Navigation enums and state structs

mod constants;
mod content;
mod input;
mod search;
mod state;

// Re-export constants
pub use constants::*;

// Re-export state types
pub use state::{
    ContentPanelMode, FlowState, FlowTab, Focus, LoadedDetails, NavMode, OverlayState, PanelRects,
    PendingLoads, SchemaOverlayState, SearchState, YamlPreviewState,
};

use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::path::Path;

use super::cache::RenderCache;
use super::data::{
    ArcClassDetails, ClassArcsData, CollapseKey, LayerDetails, RealmDetails, TaxonomyTree, TreeItem,
};
use super::theme::Theme;

use ratatui::text::Span;

// =============================================================================
// APP STRUCT
// =============================================================================

/// Main app state.
/// 55 fields → 30 direct + 25 in sub-structs.
pub struct App {
    // ==========================================================================
    // Core State
    // ==========================================================================
    /// Cached theme (color mode detected once at startup).
    pub theme: Theme,
    pub mode: NavMode,
    pub focus: Focus,
    pub tree_cursor: usize,
    /// Remember cursor position per mode.
    pub mode_cursors: [usize; 2],
    pub tree_scroll: usize,
    pub tree_height: usize,
    pub tree: TaxonomyTree,
    pub root_path: String,

    // ==========================================================================
    // Extracted Sub-States
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
    /// Flow mode state (navigable architecture diagrams).
    pub flow: FlowState,

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
    // Separate scroll states for Props and Arcs panels
    /// Properties panel scroll position.
    pub props_scroll: usize,
    /// Properties panel total line count.
    pub props_line_count: usize,
    /// Arcs panel scroll position.
    pub arcs_scroll: usize,
    /// Arcs panel total line count.
    pub arcs_line_count: usize,
    /// Identity panel scroll position.
    pub identity_scroll: usize,
    /// Identity panel total line count.
    pub identity_line_count: usize,
    /// Cache of YAML file contents (path -> content).
    pub yaml_cache: FxHashMap<String, String>,
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
    // Instance Panel State
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
        let mut app = Self {
            // Core state
            theme: Theme::with_root(&root_path),
            mode: NavMode::Graph,
            focus: Focus::Tree,
            tree_cursor: 0,
            mode_cursors: [0; 2],
            tree_scroll: 0,
            tree_height: DEFAULT_TREE_HEIGHT,
            tree,
            root_path,

            // Extracted sub-states
            search: SearchState::default(),
            overlays: OverlayState::default(),
            yaml: YamlPreviewState::default(),
            pending: PendingLoads::default(),
            details: LoadedDetails::default(),
            schema_overlay: SchemaOverlayState::default(),
            flow: FlowState::default(),

            // Navigation & history
            nav_history: Vec::with_capacity(100),
            nav_history_pos: 0,
            navigation_generation: 0,

            // UI state
            status_message: None,
            pending_refresh: false,
            // Separate scroll for Props and Arcs panels
            props_scroll: 0,
            props_line_count: 0,
            arcs_scroll: 0,
            arcs_line_count: 0,
            identity_scroll: 0,
            identity_line_count: 0,
            yaml_cache: FxHashMap::default(),
            tick: 0,
            panel_rects: PanelRects::default(),

            // Filter state
            data_filter_class: None,
            data_cursor_before_filter: 0,
            hide_empty: false,

            // Property focus state
            focused_property_idx: 0,
            expanded_property: false,
            json_pretty: false,

            // Instance panel state
            instance_standard_collapsed: false, // Expanded by default
            instance_specific_collapsed: false, // Expanded by default

            // Render caches
            mini_bar_cache: RefCell::new(RenderCache::new()),
        };
        // Initialize with smart collapsed defaults for better UX
        // (Classes section open with realms visible, but layers/classes collapsed)
        app.tree.init_default_collapsed();
        app.load_yaml_for_current();
        app
    }

    /// Ensure cursor is visible by adjusting scroll.
    pub fn ensure_cursor_visible(&mut self) {
        // Debug assertion to catch cursor bounds bugs during development
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
        self.tree.expand(&CollapseKey::Classes);
        self.tree.expand(&CollapseKey::Realm(realm_key.clone()));
        self.tree.expand(&CollapseKey::Layer {
            realm: realm_key.clone(),
            layer: layer_key.clone(),
        });

        // Calculate the index (same logic as update_search)
        let mut idx = 0;

        // Classes section header
        idx += 1;

        for realm in &self.tree.realms {
            // Realm
            idx += 1;

            if !self.tree.is_collapsed(&CollapseKey::Realm(realm.key.clone())) {
                for layer in &realm.layers {
                    // Layer
                    idx += 1;

                    if !self
                        .tree
                        .is_collapsed(&CollapseKey::Layer {
                            realm: realm.key.clone(),
                            layer: layer.key.clone(),
                        })
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

    // Event Handlers: see input.rs (handle_key, handle_mouse, handle_search_key, etc.)

    // =========================================================================
    // Mode & Navigation Helpers
    // =========================================================================

    /// Check if currently in Graph mode (unified tree that shows instances).
    /// Renamed from is_graph_mode() for clarity — Graph mode IS the unified view.
    pub fn is_graph_mode(&self) -> bool {
        self.mode == NavMode::Graph
    }

    /// Save current cursor to mode_cursors for the current mode.
    pub fn save_mode_cursor(&mut self) {
        self.mode_cursors[self.mode.index()] = self.tree_cursor;
    }

    /// Set a temporary status message (auto-clears after ~3 seconds).
    pub fn set_status(&mut self, msg: &str) {
        self.status_message = Some((msg.to_string(), std::time::Instant::now()));
    }

    /// Set an error status message with prefix (auto-clears after ~3 seconds).
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
    /// Uses $EDITOR environment variable, falls back to 'code' then 'vim'.
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

    /// Yank (copy) content based on the focused panel (smart copy).
    pub fn yank_focused_content(&mut self) {
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
    /// Single property copy.
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
    pub fn is_filtered_graph_mode(&self) -> bool {
        self.is_graph_mode() && self.data_filter_class.is_some()
    }

    /// Get the current filter Class key (if in filtered Graph mode).
    pub fn get_filter_class(&self) -> Option<&str> {
        self.data_filter_class.as_deref()
    }

    /// Get item at cursor position for the current mode.
    /// Uses mode-aware method that shows instances in Data mode.
    pub fn current_item(&self) -> Option<TreeItem<'_>> {
        // Filtered Data mode: show only instances of the filtered Class
        if let Some(class_key) = &self.data_filter_class {
            if self.is_graph_mode() {
                return self.tree.filtered_item_at(self.tree_cursor, class_key);
            }
        }
        // Normal mode
        // Pass hide_empty to match render_tree filtering
        if self.is_graph_mode() {
            self.tree
                .item_at_for_mode(self.tree_cursor, true, self.hide_empty)
        } else {
            self.tree.item_at(self.tree_cursor)
        }
    }

    /// Get total item count for the current mode.
    /// Pass hide_empty to match render_tree and item_at_for_mode filtering.
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
            self.tree.item_count()
        }
    }

    /// Get breadcrumb path for the current selection.
    /// Returns a string like "Org → Foundation → Entity (12)"
    pub fn current_breadcrumb(&self) -> String {
        if self.mode == NavMode::Flow {
            return format!("Flow → {}", self.flow.tab.label());
        }

        match self.current_item() {
            Some(TreeItem::ClassesSection) => "Node Classes".to_string(),
            Some(TreeItem::ArcsSection) => "Arcs".to_string(),
            Some(TreeItem::Realm(r)) => r.display_name.clone(),
            Some(TreeItem::Layer(r, l)) => {
                format!("{} → {}", r.display_name, l.display_name)
            },
            Some(TreeItem::Class(r, l, k)) => {
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
        if let Some(TreeItem::Class(_, _, class_info)) =
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
    pub(crate) fn toggle_tree_item(&mut self) {
        use crate::tui::data::CollapseKey;

        let data_mode = self.is_graph_mode();

        if let Some(key) = self
            .tree
            .collapse_key_at(self.tree_cursor, data_mode, self.hide_empty)
        {
            match &key {
                // Handle Class toggle in Data mode
                CollapseKey::Class(class_key) => {
                    if data_mode {
                        // Use helpers for Entity/EntityNative dual storage pattern
                        let instances_loaded = if class_key == "Entity" {
                            self.tree.has_entity_instances()
                        } else if class_key == "EntityNative" {
                            !self.tree.entity_native_groups.is_empty()
                        } else {
                            self.tree.get_instances(class_key).is_some()
                        };

                        if !instances_loaded {
                            // First click on unloaded Class: load instances AND ensure expanded
                            if class_key == "Entity" {
                                self.pending.instance = Some("Entity".to_string());
                            } else if class_key == "EntityNative" {
                                if self.tree.entity_native_groups.is_empty() {
                                    self.pending.entity_natives = true;
                                }
                            } else {
                                self.pending.instance = Some(class_key.clone());
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
                CollapseKey::Category(category_key) => {
                    if data_mode {
                        let instances_loaded = self
                            .tree
                            .entity_category_instances
                            .contains_key(category_key.as_str());

                        if !instances_loaded {
                            self.pending.category_instances = Some(category_key.clone());
                            if self.tree.is_collapsed(&key) {
                                self.tree.toggle(&key);
                            }
                        } else {
                            self.tree.toggle(&key);
                        }
                    } else {
                        self.tree.toggle(&key);
                    }
                }
                // Other items (Realm, Layer, ArcFamily, EntityGroup, etc.): normal toggle
                _ => {
                    self.tree.toggle(&key);
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
        if !self.is_graph_mode() || !self.schema_overlay.enabled {
            self.schema_overlay.matched_properties = None;
            self.schema_overlay.coverage_stats = None;
            return;
        }

        // Check if current item is an Instance
        // Pass hide_empty to match render_tree filtering
        // Clone properties to avoid borrow conflict
        let props = if let Some(TreeItem::Instance(_, _, _, instance)) = self
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
            || self.pending.entity_natives
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
}
