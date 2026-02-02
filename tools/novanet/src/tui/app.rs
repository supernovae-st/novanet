//! App state machine for the TUI.
//!
//! Two top-level states: `Loading` (connecting to Neo4j, fetching meta-graph)
//! and `Ready` (interactive browsing). Navigation modes mirror the CLI:
//! Data (1), Meta (2), Overlay (3), Query (4).

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

/// Active panel in the layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Tree,
    Detail,
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
}
