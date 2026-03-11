//! State types for TUI v2.
//!
//! This module contains all enums and state structs used by the App.
//! Extracted from app.rs for better organization.

use std::collections::BTreeMap;

use ratatui::layout::Rect;
use rustc_hash::FxHashMap;
use serde_json::Value as JsonValue;

use crate::tui::data::{ArcClassDetails, ClassArcsData, LayerDetails, RealmDetails};
use crate::tui::schema::{CoverageStats, MatchedProperty, ValidatedProperty, ValidationStats};

// =============================================================================
// NAVIGATION ENUMS
// =============================================================================

/// Navigation mode — 3 modes in v0.12.5.
/// Order: 1:Graph 2:Views 3:Nexus
/// Keys 1-3 switch modes GLOBALLY from anywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavMode {
    /// Graph mode: Unified tree view (Realm > Layer > Class hierarchy with instances)
    /// Replaces Meta/Data/Overlay modes from v11.6
    #[default]
    Graph,
    /// Views mode: Schema views explorer (Query-First architecture)
    Views,
    /// Nexus mode: Hub for Quiz, Stats, Help
    Nexus,
}

impl NavMode {
    pub fn label(&self) -> &'static str {
        match self {
            NavMode::Graph => "Graph",
            NavMode::Views => "Views",
            NavMode::Nexus => "Nexus",
        }
    }

    /// Get array index for mode_cursors (0-2).
    pub fn index(&self) -> usize {
        match self {
            NavMode::Graph => 0,
            NavMode::Views => 1,
            NavMode::Nexus => 2,
        }
    }

    /// Get all modes in order.
    pub fn all() -> &'static [NavMode] {
        &[NavMode::Graph, NavMode::Views, NavMode::Nexus]
    }
}

/// Which panel is currently focused for keyboard input.
/// v0.18.3: Extended to 5 panels for new layout.
///
/// Layout:
/// ```text
/// ┌──────────┬────────────────────────┬──────────────────┐
/// │ TREE [1] │ Identity [2]           │ Props [4]        │
/// │          ├────────────────────────┼──────────────────┤
/// │          │ Content [3]            │ Arcs [5]         │
/// └──────────┴────────────────────────┴──────────────────┘
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    #[default]
    Tree, // [1] Left panel - tree navigation
    Identity, // [2] Center top - identity & provenance (NEW in v0.18.3)
    Content,  // [3] Center bottom - data viewer
    Props,    // [4] Right top - properties
    Arcs,     // [5] Right bottom - relationships
}

impl Focus {
    /// Cycle to next focus panel (Tab or l).
    /// Cycle: Tree [1] → Identity [2] → Content [3] → Props [4] → Arcs [5] → Tree [1]
    pub fn next(self) -> Self {
        match self {
            Focus::Tree => Focus::Identity,
            Focus::Identity => Focus::Content,
            Focus::Content => Focus::Props,
            Focus::Props => Focus::Arcs,
            Focus::Arcs => Focus::Tree,
        }
    }

    /// Cycle to previous focus panel (Shift+Tab or h).
    pub fn prev(self) -> Self {
        match self {
            Focus::Tree => Focus::Arcs,
            Focus::Identity => Focus::Tree,
            Focus::Content => Focus::Identity,
            Focus::Props => Focus::Content,
            Focus::Arcs => Focus::Props,
        }
    }

    /// Get panel number for display [1-5].
    pub fn number(self) -> u8 {
        match self {
            Focus::Tree => 1,
            Focus::Identity => 2,
            Focus::Content => 3,
            Focus::Props => 4,
            Focus::Arcs => 5,
        }
    }

    /// Spatial navigation: move up.
    pub fn up(self) -> Self {
        match self {
            Focus::Content => Focus::Identity, // Center bottom → top
            Focus::Arcs => Focus::Props,       // Right bottom → top
            other => other,                    // No change
        }
    }

    /// Spatial navigation: move down.
    pub fn down(self) -> Self {
        match self {
            Focus::Identity => Focus::Content, // Center top → bottom
            Focus::Props => Focus::Arcs,       // Right top → bottom
            other => other,                    // No change
        }
    }

    /// Spatial navigation: move left.
    pub fn left(self) -> Self {
        match self {
            Focus::Identity | Focus::Content => Focus::Tree,
            Focus::Props => Focus::Identity,
            Focus::Arcs => Focus::Content,
            Focus::Tree => Focus::Tree, // Already leftmost
        }
    }

    /// Spatial navigation: move right.
    pub fn right(self) -> Self {
        match self {
            Focus::Tree => Focus::Identity, // Go to center top by default
            Focus::Identity => Focus::Props,
            Focus::Content => Focus::Arcs,
            Focus::Props | Focus::Arcs => self, // Already rightmost
        }
    }

    /// Display name for status bar.
    pub fn name(self) -> &'static str {
        match self {
            Focus::Tree => "TREE",
            Focus::Identity => "IDENTITY",
            Focus::Content => "CONTENT",
            Focus::Props => "PROPS",
            Focus::Arcs => "ARCS",
        }
    }
}

/// DEPRECATED: Use Focus enum instead.
/// v0.18.3: Scheduled for removal. Use Focus enum for panel navigation.
///
/// Which info box is selected for copy/scroll within the Graph mode.
/// Implements "Focusable Box" pattern from TUI Box Navigation design.
///
/// Layout (3 columns):
/// ```text
/// ┌─────────┬─────────────────┬───────────────┐
/// │  TREE   │ HEADER          │ SOURCE        │
/// │         │ PROPERTIES      │               │
/// │         │ ARCS            │ (empty)       │
/// └─────────┴─────────────────┴───────────────┘
/// ```
/// Tab cycles: Tree -> Header -> Properties -> Arcs -> Source -> Tree
/// v0.13.1: Diagram and Architecture removed (panel simplification)
#[deprecated(since = "0.18.3", note = "Use Focus enum instead")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InfoBox {
    #[default]
    Tree,
    Header,
    Properties,
    Arcs, // v0.13: Consolidated arc relationships panel (was Graph + Arcs)
    Source,
    // v0.13.1: Diagram and Architecture removed (panel simplification)
}

impl InfoBox {
    /// Cycle to next box (Tab or right arrow).
    /// 5-box cycle: Tree → Header → Properties → Arcs → Source → Tree
    /// v0.13.1: Diagram and Architecture removed
    pub fn next(self) -> Self {
        match self {
            Self::Tree => Self::Header,
            Self::Header => Self::Properties,
            Self::Properties => Self::Arcs,
            Self::Arcs => Self::Source,
            Self::Source => Self::Tree,
        }
    }

    /// Cycle to previous box (Shift+Tab or left arrow).
    pub fn prev(self) -> Self {
        match self {
            Self::Tree => Self::Source,
            Self::Header => Self::Tree,
            Self::Properties => Self::Header,
            Self::Arcs => Self::Properties,
            Self::Source => Self::Arcs,
        }
    }

    /// Display name for status bar.
    pub fn name(self) -> &'static str {
        match self {
            Self::Tree => "TREE",
            Self::Header => "HEADER",
            Self::Properties => "PROPERTIES",
            Self::Arcs => "ARCS",
            Self::Source => "SOURCE",
        }
    }

    /// Check if this box is in the right panel (YAML column).
    pub fn is_right_panel(self) -> bool {
        matches!(self, Self::Source)
    }
}

// =============================================================================
// CONTENT PANEL
// =============================================================================

/// Content panel mode - determines what the center panel shows.
/// v0.17.3: Replaces SourceTab - no toggle, context-aware content.
///
/// Computed on-demand via `App::content_panel_mode()` - not stored in App.
/// This avoids lifetime complexity while still providing context-aware rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentPanelMode {
    /// Show YAML schema definition (for Class, ArcClass).
    Schema {
        /// Path to the YAML file.
        path: String,
        /// Name of the class/arc.
        name: String,
    },
    /// Show instance data from Neo4j (symmetric with Schema for YAML).
    /// v0.17.3: Now shows actual properties instead of redirect message.
    InstanceInfo {
        /// Instance key (e.g., "barcode@en-US").
        instance_key: String,
        /// Parent class name.
        class_name: String,
        /// Realm of the class.
        realm: String,
        /// Layer of the class.
        layer: String,
        /// Instance properties from Neo4j.
        properties: BTreeMap<String, JsonValue>,
    },
    /// Show section info (for Realm, Layer, Section headers).
    SectionInfo {
        /// Section name.
        name: String,
        /// Description or stats.
        description: String,
    },
    /// No content available (empty tree or no selection).
    Empty,
}

/// Extracted data from a TreeItem for use in load_yaml_for_current().
/// This avoids borrow checker issues when we need to both read the tree and mutate App.
#[derive(Debug)]
pub enum TreeItemData {
    Class {
        yaml_path: String,
        key: String,
        properties: Vec<String>,
    },
    ArcClass {
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
    /// Instance with full metadata for content panel and YAML loading.
    /// v0.17.3: Extended with instance_key, class_name, realm, layer, properties for ContentPanelMode.
    Instance {
        /// Instance key (e.g., "barcode@en-US")
        instance_key: String,
        /// Class name (e.g., "EntityNative")
        class_name: String,
        /// Realm (e.g., "org")
        realm: String,
        /// Layer (e.g., "semantic")
        layer: String,
        /// Class's yaml_path (to show schema in YAML panel)
        class_yaml_path: String,
        /// Class properties for loading validated properties with types.
        class_properties: Vec<String>,
        /// Instance properties from Neo4j.
        properties: BTreeMap<String, JsonValue>,
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

/// All pending async load requests (extracted sub-state).
/// v0.14.0: Extracted from App struct for clarity.
#[derive(Debug, Default)]
pub struct PendingLoads {
    /// Pending instance load request (Class label to load).
    pub instance: Option<String>,
    /// Pending Class arcs load request (Class label to load from Neo4j).
    pub arcs: Option<String>,
    /// Pending instance arc loading (Class label + instance keys for background arc loading).
    pub instance_arcs: Option<(String, Vec<String>)>,
    /// Pending entity categories load (triggered when Entity Class expanded).
    pub entity_categories: bool,
    /// Pending category instances load (category key to load).
    pub category_instances: Option<String>,
    /// Pending EntityNative locale groups load (triggered when EntityNative Class expanded).
    pub entity_natives: bool,
    /// Pending ArcClass details load request (Arc key to load from Neo4j).
    pub arc_class: Option<String>,
    /// Pending Realm details load request (Realm key to load from Neo4j).
    pub realm: Option<String>,
    /// Pending Layer details load request (Layer key to load from Neo4j).
    pub layer: Option<String>,
}

impl PendingLoads {
    /// Check if any load is pending.
    pub fn has_pending(&self) -> bool {
        self.instance.is_some()
            || self.arcs.is_some()
            || self.instance_arcs.is_some()
            || self.entity_categories
            || self.category_instances.is_some()
            || self.entity_natives
            || self.arc_class.is_some()
            || self.realm.is_some()
            || self.layer.is_some()
    }

    /// Clear all pending loads.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// Neo4j details for current selection (extracted sub-state).
/// Loaded async when user selects Realm/Layer/Class/Arc.
#[derive(Debug, Default)]
pub struct LoadedDetails {
    /// Neo4j arc data for current Class.
    pub class_arcs: Option<ClassArcsData>,
    /// Neo4j arc class details (loaded async when ArcClass selected).
    pub arc_class: Option<ArcClassDetails>,
    /// Neo4j Realm details (loaded async when Realm selected).
    pub realm: Option<RealmDetails>,
    /// Neo4j Layer details (loaded async when Layer selected).
    pub layer: Option<LayerDetails>,
}

impl LoadedDetails {
    /// Clear all loaded details.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// Schema overlay state for Data mode (extracted sub-state).
/// Shows YAML schema properties alongside Neo4j instance data.
#[derive(Debug)]
pub struct SchemaOverlayState {
    /// Whether schema overlay is enabled (toggle with 's').
    pub enabled: bool,
    /// Matched properties for current instance.
    pub matched_properties: Option<Vec<MatchedProperty>>,
    /// Coverage stats for current instance.
    pub coverage_stats: Option<CoverageStats>,
    /// Validated properties for current Class (YAML schema vs Neo4j).
    pub validated_class_properties: Option<Vec<ValidatedProperty>>,
    /// Validation stats for current Class.
    pub validation_stats: Option<ValidationStats>,
}

impl Default for SchemaOverlayState {
    fn default() -> Self {
        Self {
            enabled: true, // Enabled by default
            matched_properties: None,
            coverage_stats: None,
            validated_class_properties: None,
            validation_stats: None,
        }
    }
}

impl SchemaOverlayState {
    /// Clear all schema overlay state (except enabled flag).
    pub fn clear_data(&mut self) {
        self.matched_properties = None;
        self.coverage_stats = None;
        self.validated_class_properties = None;
        self.validation_stats = None;
    }
}

/// YAML panel state (extracted sub-state).
/// Displays Class YAML or Instance data in the right panel.
#[derive(Debug, Default)]
pub struct YamlPreviewState {
    /// YAML content to display.
    pub content: String,
    /// Path to the YAML file.
    pub path: String,
    /// Scroll position in the YAML panel.
    pub scroll: usize,
    /// Cached line count (avoids per-scroll recomputation).
    pub line_count: usize,
}

impl YamlPreviewState {
    /// Clear all YAML preview state.
    pub fn clear(&mut self) {
        self.content.clear();
        self.path.clear();
        self.scroll = 0;
        self.line_count = 0;
    }
}

/// Overlay visibility state (extracted sub-state).
/// Overlays are modal panels that appear on top of the main UI.
#[derive(Debug, Default)]
pub struct OverlayState {
    /// Whether help overlay is active.
    pub help_active: bool,
    /// Whether legend overlay is active.
    pub legend_active: bool,
    /// Whether recent items overlay is active.
    pub recent_items_active: bool,
    /// Cursor position in recent items overlay.
    pub recent_items_cursor: usize,
}

impl OverlayState {
    /// Check if any overlay is active.
    pub fn is_active(&self) -> bool {
        self.help_active || self.legend_active || self.recent_items_active
    }

    /// Close all overlays.
    pub fn close_all(&mut self) {
        self.help_active = false;
        self.legend_active = false;
        self.recent_items_active = false;
    }
}

// =============================================================================
// PANEL RECTS (Mouse Scroll Support v0.17.3)
// =============================================================================

/// Panel identifiers for mouse hit-testing.
/// v0.17.3: Renamed Yaml → Content to reflect context-aware content.
/// v0.18.3: Added Identity panel for new 5-panel layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Tree,
    Identity, // v0.18.3: Center top - identity & provenance
    Content,  // was Yaml
    Props,
    Arcs,
}

impl Panel {
    /// Convert Panel to corresponding Focus for click-to-focus.
    pub const fn to_focus(self) -> Focus {
        match self {
            Panel::Tree => Focus::Tree,
            Panel::Identity => Focus::Identity,
            Panel::Content => Focus::Content,
            Panel::Props => Focus::Props,
            Panel::Arcs => Focus::Arcs,
        }
    }
}

/// Stores panel rectangles for mouse hit-testing.
/// Updated during each render pass with the actual panel areas.
/// v0.17.3: Renamed yaml → content to reflect context-aware content.
/// v0.18.3: Added identity panel for new 5-panel layout.
#[derive(Debug, Clone, Default)]
pub struct PanelRects {
    pub tree: Option<Rect>,
    pub identity: Option<Rect>, // v0.18.3: Center top
    pub content: Option<Rect>,  // was yaml
    pub props: Option<Rect>,
    pub arcs: Option<Rect>,
}

impl PanelRects {
    /// Hit-test: returns which panel contains the given (column, row) position.
    pub fn hit_test(&self, column: u16, row: u16) -> Option<Panel> {
        // Check panels in z-order (front to back)
        // Note: Rect::contains takes a Position, but we can check manually
        if let Some(rect) = &self.tree {
            if Self::contains(rect, column, row) {
                return Some(Panel::Tree);
            }
        }
        if let Some(rect) = &self.identity {
            if Self::contains(rect, column, row) {
                return Some(Panel::Identity);
            }
        }
        if let Some(rect) = &self.content {
            if Self::contains(rect, column, row) {
                return Some(Panel::Content);
            }
        }
        if let Some(rect) = &self.props {
            if Self::contains(rect, column, row) {
                return Some(Panel::Props);
            }
        }
        if let Some(rect) = &self.arcs {
            if Self::contains(rect, column, row) {
                return Some(Panel::Arcs);
            }
        }
        None
    }

    /// Check if a point (column, row) is within a Rect.
    #[inline]
    fn contains(rect: &Rect, column: u16, row: u16) -> bool {
        column >= rect.x
            && column < rect.x + rect.width
            && row >= rect.y
            && row < rect.y + rect.height
    }

    /// Clear all panel rects (called at start of render).
    pub fn clear(&mut self) {
        self.tree = None;
        self.identity = None;
        self.content = None;
        self.props = None;
        self.arcs = None;
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // NavMode Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_navmode_labels() {
        insta::assert_snapshot!(format!(
            "Graph: {}\nViews: {}\nNexus: {}",
            NavMode::Graph.label(),
            NavMode::Views.label(),
            NavMode::Nexus.label()
        ));
    }

    #[test]
    fn test_navmode_indices() {
        assert_eq!(NavMode::Graph.index(), 0);
        assert_eq!(NavMode::Views.index(), 1);
        assert_eq!(NavMode::Nexus.index(), 2);
    }

    #[test]
    fn test_navmode_all() {
        let all = NavMode::all();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0], NavMode::Graph);
        assert_eq!(all[1], NavMode::Views);
        assert_eq!(all[2], NavMode::Nexus);
    }

    // -------------------------------------------------------------------------
    // Focus Tests (v0.18.3: 5 panels)
    // -------------------------------------------------------------------------

    #[test]
    fn test_focus_cycle_next() {
        // v0.18.3: Tree → Identity → Content → Props → Arcs → Tree
        let focus = Focus::Tree;
        assert_eq!(focus.next(), Focus::Identity);
        assert_eq!(focus.next().next(), Focus::Content);
        assert_eq!(focus.next().next().next(), Focus::Props);
        assert_eq!(focus.next().next().next().next(), Focus::Arcs);
        assert_eq!(focus.next().next().next().next().next(), Focus::Tree);
    }

    #[test]
    fn test_focus_cycle_prev() {
        // v0.18.3: Tree ← Identity ← Content ← Props ← Arcs ← Tree
        let focus = Focus::Tree;
        assert_eq!(focus.prev(), Focus::Arcs);
        assert_eq!(focus.prev().prev(), Focus::Props);
        assert_eq!(focus.prev().prev().prev(), Focus::Content);
        assert_eq!(focus.prev().prev().prev().prev(), Focus::Identity);
        assert_eq!(focus.prev().prev().prev().prev().prev(), Focus::Tree);
    }

    #[test]
    fn test_focus_numbers() {
        // v0.18.3: 5 panels [1-5]
        insta::assert_snapshot!(format!(
            "Tree: [{}]\nIdentity: [{}]\nContent: [{}]\nProps: [{}]\nArcs: [{}]",
            Focus::Tree.number(),
            Focus::Identity.number(),
            Focus::Content.number(),
            Focus::Props.number(),
            Focus::Arcs.number()
        ));
    }

    #[test]
    fn test_focus_spatial_up() {
        assert_eq!(Focus::Content.up(), Focus::Identity);
        assert_eq!(Focus::Arcs.up(), Focus::Props);
        assert_eq!(Focus::Tree.up(), Focus::Tree); // No change
        assert_eq!(Focus::Identity.up(), Focus::Identity); // Already top
        assert_eq!(Focus::Props.up(), Focus::Props); // Already top
    }

    #[test]
    fn test_focus_spatial_down() {
        assert_eq!(Focus::Identity.down(), Focus::Content);
        assert_eq!(Focus::Props.down(), Focus::Arcs);
        assert_eq!(Focus::Tree.down(), Focus::Tree); // No change
        assert_eq!(Focus::Content.down(), Focus::Content); // Already bottom
        assert_eq!(Focus::Arcs.down(), Focus::Arcs); // Already bottom
    }

    #[test]
    fn test_focus_spatial_left() {
        assert_eq!(Focus::Identity.left(), Focus::Tree);
        assert_eq!(Focus::Content.left(), Focus::Tree);
        assert_eq!(Focus::Props.left(), Focus::Identity);
        assert_eq!(Focus::Arcs.left(), Focus::Content);
        assert_eq!(Focus::Tree.left(), Focus::Tree); // Already leftmost
    }

    #[test]
    fn test_focus_spatial_right() {
        assert_eq!(Focus::Tree.right(), Focus::Identity);
        assert_eq!(Focus::Identity.right(), Focus::Props);
        assert_eq!(Focus::Content.right(), Focus::Arcs);
        assert_eq!(Focus::Props.right(), Focus::Props); // Already rightmost
        assert_eq!(Focus::Arcs.right(), Focus::Arcs); // Already rightmost
    }

    #[test]
    fn test_focus_names() {
        assert_eq!(Focus::Tree.name(), "TREE");
        assert_eq!(Focus::Identity.name(), "IDENTITY");
        assert_eq!(Focus::Content.name(), "CONTENT");
        assert_eq!(Focus::Props.name(), "PROPS");
        assert_eq!(Focus::Arcs.name(), "ARCS");
    }

    // -------------------------------------------------------------------------
    // InfoBox Tests (DEPRECATED: v0.18.3 - will be removed)
    // -------------------------------------------------------------------------

    #[test]
    #[allow(deprecated)]
    fn test_infobox_cycle() {
        let box_ = InfoBox::Tree;
        assert_eq!(box_.next(), InfoBox::Header);
        assert_eq!(box_.next().next(), InfoBox::Properties);
        assert_eq!(box_.next().next().next(), InfoBox::Arcs);
        assert_eq!(box_.next().next().next().next(), InfoBox::Source);
        assert_eq!(box_.next().next().next().next().next(), InfoBox::Tree);
    }

    #[test]
    #[allow(deprecated)]
    fn test_infobox_names() {
        insta::assert_snapshot!(format!(
            "Tree: {}\nHeader: {}\nProperties: {}\nArcs: {}\nSource: {}",
            InfoBox::Tree.name(),
            InfoBox::Header.name(),
            InfoBox::Properties.name(),
            InfoBox::Arcs.name(),
            InfoBox::Source.name()
        ));
    }

    #[test]
    #[allow(deprecated)]
    fn test_infobox_right_panel() {
        assert!(!InfoBox::Tree.is_right_panel());
        assert!(!InfoBox::Header.is_right_panel());
        assert!(!InfoBox::Properties.is_right_panel());
        assert!(!InfoBox::Arcs.is_right_panel());
        assert!(InfoBox::Source.is_right_panel());
    }

    // -------------------------------------------------------------------------
    // SearchState Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_search_state_clear() {
        let mut state = SearchState {
            active: true,
            query: "test".to_string(),
            results: vec![1, 2, 3],
            scores: vec![100, 90, 80],
            matches: FxHashMap::default(),
            cursor: 2,
        };
        state.clear();
        assert!(!state.active);
        assert!(state.query.is_empty());
        assert!(state.results.is_empty());
        assert!(state.scores.is_empty());
        assert!(state.matches.is_empty());
        assert_eq!(state.cursor, 0);
    }

    // -------------------------------------------------------------------------
    // PendingLoads Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_pending_loads_has_pending() {
        let empty = PendingLoads::default();
        assert!(!empty.has_pending());

        let with_instance = PendingLoads {
            instance: Some("Entity".to_string()),
            ..Default::default()
        };
        assert!(with_instance.has_pending());

        let with_arcs = PendingLoads {
            arcs: Some("Page".to_string()),
            ..Default::default()
        };
        assert!(with_arcs.has_pending());
    }

    #[test]
    fn test_pending_loads_clear() {
        let mut pending = PendingLoads {
            instance: Some("Entity".to_string()),
            arcs: Some("Page".to_string()),
            entity_categories: true,
            ..Default::default()
        };
        pending.clear();
        assert!(!pending.has_pending());
    }

    // -------------------------------------------------------------------------
    // OverlayState Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_overlay_state_is_active() {
        let inactive = OverlayState::default();
        assert!(!inactive.is_active());

        let help = OverlayState {
            help_active: true,
            ..Default::default()
        };
        assert!(help.is_active());

        let legend = OverlayState {
            legend_active: true,
            ..Default::default()
        };
        assert!(legend.is_active());
    }

    #[test]
    fn test_overlay_state_close_all() {
        let mut state = OverlayState {
            help_active: true,
            legend_active: true,
            recent_items_active: true,
            recent_items_cursor: 5,
        };
        state.close_all();
        assert!(!state.help_active);
        assert!(!state.legend_active);
        assert!(!state.recent_items_active);
    }

    // -------------------------------------------------------------------------
    // PanelRects Tests (v0.18.3: 5 panels)
    // -------------------------------------------------------------------------

    #[test]
    fn test_panel_rects_hit_test() {
        // v0.18.3: 5-panel layout with identity
        let rects = PanelRects {
            tree: Some(Rect::new(0, 0, 30, 20)),
            identity: Some(Rect::new(30, 0, 40, 10)), // Center top
            content: Some(Rect::new(30, 10, 40, 10)), // Center bottom
            props: Some(Rect::new(70, 0, 30, 10)),
            arcs: Some(Rect::new(70, 10, 30, 10)),
        };

        // Hit tree panel
        assert_eq!(rects.hit_test(15, 10), Some(Panel::Tree));

        // Hit identity panel (center top)
        assert_eq!(rects.hit_test(50, 5), Some(Panel::Identity));

        // Hit content panel (center bottom)
        assert_eq!(rects.hit_test(50, 15), Some(Panel::Content));

        // Hit props panel
        assert_eq!(rects.hit_test(85, 5), Some(Panel::Props));

        // Hit arcs panel
        assert_eq!(rects.hit_test(85, 15), Some(Panel::Arcs));

        // Miss all panels
        assert_eq!(rects.hit_test(100, 100), None);
    }

    #[test]
    fn test_panel_to_focus() {
        // v0.18.3: 5 panels map to 5 Focus variants
        assert_eq!(Panel::Tree.to_focus(), Focus::Tree);
        assert_eq!(Panel::Identity.to_focus(), Focus::Identity);
        assert_eq!(Panel::Content.to_focus(), Focus::Content);
        assert_eq!(Panel::Props.to_focus(), Focus::Props);
        assert_eq!(Panel::Arcs.to_focus(), Focus::Arcs);
    }

    // -------------------------------------------------------------------------
    // ContentPanelMode Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_content_panel_mode_schema() {
        let mode = ContentPanelMode::Schema {
            path: "/path/to/entity.yaml".to_string(),
            name: "Entity".to_string(),
        };
        if let ContentPanelMode::Schema { path, name } = mode {
            assert_eq!(path, "/path/to/entity.yaml");
            assert_eq!(name, "Entity");
        } else {
            panic!("Expected Schema variant");
        }
    }

    #[test]
    fn test_content_panel_mode_instance_info() {
        let mode = ContentPanelMode::InstanceInfo {
            instance_key: "barcode@en-US".to_string(),
            class_name: "EntityNative".to_string(),
            realm: "org".to_string(),
            layer: "semantic".to_string(),
            properties: std::collections::BTreeMap::new(),
        };
        if let ContentPanelMode::InstanceInfo {
            instance_key,
            class_name,
            realm,
            layer,
            ..
        } = mode
        {
            assert_eq!(instance_key, "barcode@en-US");
            assert_eq!(class_name, "EntityNative");
            assert_eq!(realm, "org");
            assert_eq!(layer, "semantic");
        } else {
            panic!("Expected InstanceInfo variant");
        }
    }
}
