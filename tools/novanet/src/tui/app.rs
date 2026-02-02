//! App state machine for the TUI.
//!
//! Two top-level states: `Loading` (connecting to Neo4j, fetching meta-graph)
//! and `Ready` (interactive browsing). Navigation modes mirror the CLI:
//! Data (1), Meta (2), Overlay (3), Query (4).

use crate::tui::dashboard::DashboardStats;
use crate::tui::detail::KindDetail;
use crate::tui::dialogs::DialogState;
use crate::tui::palette::PaletteState;
use crate::tui::search::SearchState;
use crate::tui::tree::TaxonomyTree;

/// Navigation mode — mirrors CLI modes 1-4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavMode {
    Data,
    Meta,
    Overlay,
    Query,
}

impl NavMode {
    pub fn label(self) -> &'static str {
        match self {
            NavMode::Data => "Data",
            NavMode::Meta => "Meta",
            NavMode::Overlay => "Overlay",
            NavMode::Query => "Query",
        }
    }

    #[allow(dead_code)] // Used by Phase 7B mode tab rendering
    pub fn index(self) -> usize {
        match self {
            NavMode::Data => 0,
            NavMode::Meta => 1,
            NavMode::Overlay => 2,
            NavMode::Query => 3,
        }
    }

    pub fn cycle(self) -> Self {
        match self {
            NavMode::Data => NavMode::Meta,
            NavMode::Meta => NavMode::Overlay,
            NavMode::Overlay => NavMode::Query,
            NavMode::Query => NavMode::Data,
        }
    }

    pub fn from_key(c: char) -> Option<Self> {
        match c {
            '1' => Some(NavMode::Data),
            '2' => Some(NavMode::Meta),
            '3' => Some(NavMode::Overlay),
            '4' => Some(NavMode::Query),
            _ => None,
        }
    }
}

/// Active panel in the mission control layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Tree,
    Detail,
    CypherPreview,
}

impl ActivePanel {
    /// Cycle to the next panel (Tree → Detail → CypherPreview → Tree).
    pub fn cycle_next(self) -> Self {
        match self {
            ActivePanel::Tree => ActivePanel::Detail,
            ActivePanel::Detail => ActivePanel::CypherPreview,
            ActivePanel::CypherPreview => ActivePanel::Tree,
        }
    }

    /// Cycle to the previous panel (Tree → CypherPreview → Detail → Tree).
    pub fn cycle_prev(self) -> Self {
        match self {
            ActivePanel::Tree => ActivePanel::CypherPreview,
            ActivePanel::Detail => ActivePanel::Tree,
            ActivePanel::CypherPreview => ActivePanel::Detail,
        }
    }
}

/// Facet filter state for Query mode.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)] // Fields used in Phase 7B interactive filter selection
pub struct FacetFilterState {
    pub realms: Vec<String>,
    pub layers: Vec<String>,
    pub traits: Vec<String>,
    pub show_popup: bool,
    pub popup_cursor: usize,
}

/// Top-level app state.
#[allow(clippy::large_enum_variant)] // Ready is the hot path; Loading is rare
pub enum AppState {
    Loading {
        message: String,
    },
    Ready {
        mode: NavMode,
        tree: TaxonomyTree,
        active_panel: ActivePanel,
        detail_lines: Vec<String>,
        status: String,
        facets: FacetFilterState,
        node_count: usize,
        cypher_preview: Vec<String>,
        kind_detail: Option<Box<KindDetail>>,
        search: Option<SearchState>,
        /// Edge explorer cursor. None = normal detail, Some(idx) = focused edge view.
        edge_explorer_idx: Option<usize>,
        /// CRUD dialog overlay. None = no dialog, Some = modal form.
        dialog: Option<DialogState>,
        /// Available EdgeKind keys for relation type dropdowns.
        edge_kind_keys: Vec<String>,
        /// Dashboard statistics from Neo4j.
        dashboard_stats: Option<DashboardStats>,
        /// Whether the dashboard panel is visible (toggle with 's').
        show_dashboard: bool,
        /// Command palette overlay (`:` key).
        palette: Option<PaletteState>,
        /// Help reference card overlay (`?` key).
        show_help: bool,
    },
}

impl AppState {
    pub fn loading(message: impl Into<String>) -> Self {
        AppState::Loading {
            message: message.into(),
        }
    }

    pub fn ready(tree: TaxonomyTree) -> Self {
        let node_count = tree.item_count();
        AppState::Ready {
            mode: NavMode::Meta,
            tree,
            active_panel: ActivePanel::Tree,
            detail_lines: vec!["Select a node to see details.".to_string()],
            status: format!("{node_count} node(s) loaded"),
            facets: FacetFilterState::default(),
            node_count,
            cypher_preview: Vec::new(),
            kind_detail: None,
            search: None,
            edge_explorer_idx: None,
            dialog: None,
            edge_kind_keys: Vec::new(),
            dashboard_stats: None,
            show_dashboard: true,
            palette: None,
            show_help: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_mode_cycle() {
        assert_eq!(NavMode::Data.cycle(), NavMode::Meta);
        assert_eq!(NavMode::Meta.cycle(), NavMode::Overlay);
        assert_eq!(NavMode::Overlay.cycle(), NavMode::Query);
        assert_eq!(NavMode::Query.cycle(), NavMode::Data);
    }

    #[test]
    fn nav_mode_from_key() {
        assert_eq!(NavMode::from_key('1'), Some(NavMode::Data));
        assert_eq!(NavMode::from_key('4'), Some(NavMode::Query));
        assert_eq!(NavMode::from_key('x'), None);
    }

    #[test]
    fn nav_mode_labels() {
        assert_eq!(NavMode::Data.label(), "Data");
        assert_eq!(NavMode::Query.label(), "Query");
    }

    #[test]
    fn nav_mode_index() {
        assert_eq!(NavMode::Data.index(), 0);
        assert_eq!(NavMode::Query.index(), 3);
    }

    #[test]
    fn panel_cycle_next() {
        assert_eq!(ActivePanel::Tree.cycle_next(), ActivePanel::Detail);
        assert_eq!(ActivePanel::Detail.cycle_next(), ActivePanel::CypherPreview);
        assert_eq!(ActivePanel::CypherPreview.cycle_next(), ActivePanel::Tree);
    }

    #[test]
    fn panel_cycle_prev() {
        assert_eq!(ActivePanel::Tree.cycle_prev(), ActivePanel::CypherPreview);
        assert_eq!(ActivePanel::Detail.cycle_prev(), ActivePanel::Tree);
        assert_eq!(ActivePanel::CypherPreview.cycle_prev(), ActivePanel::Detail);
    }
}
