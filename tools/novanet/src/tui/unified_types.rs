//! Unified Tree Types for v11.8
//!
//! This module defines the core data structures for the unified tree architecture
//! where Realm, Layer, Class, Instance, ArcFamily, and ArcClass are all represented
//! as clickable nodes with detail panels.

use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use std::collections::HashMap;

// ============================================================================
// Node Identification
// ============================================================================

/// Unique identifier for any node in the unified tree.
/// Enables O(1) lookups and consistent state management.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeId {
    /// Top-level section ("Nodes" or "Arcs")
    Section(SectionKind),
    /// Realm node (e.g., "shared", "org")
    Realm(String),
    /// Layer node within a realm (realm_key, layer_key)
    Layer { realm: String, layer: String },
    /// Class node (e.g., "Locale", "Entity")
    Class(String),
    /// Data instance (class_label, instance_key)
    Instance { class: String, key: String },
    /// Arc family (e.g., "ownership", "semantic")
    ArcFamily(String),
    /// Arc class (e.g., "HAS_PAGE", "USES_ENTITY")
    ArcClass(String),
}

/// Top-level section types in the tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionKind {
    /// "Nodes (60)" section
    Nodes,
    /// "Arcs (114)" section
    Arcs,
}

impl NodeId {
    /// Generate a stable string key for collapse state tracking.
    pub fn collapse_key(&self) -> String {
        match self {
            Self::Section(s) => format!("section:{:?}", s),
            Self::Realm(r) => format!("realm:{}", r),
            Self::Layer { realm, layer } => format!("layer:{}:{}", realm, layer),
            Self::Class(c) => format!("class:{}", c),
            Self::Instance { class, key } => format!("instance:{}:{}", class, key),
            Self::ArcFamily(f) => format!("arcfamily:{}", f),
            Self::ArcClass(a) => format!("arcclass:{}", a),
        }
    }

    /// Parse a collapse key back into a NodeId.
    pub fn from_collapse_key(key: &str) -> Option<Self> {
        let parts: Vec<&str> = key.splitn(2, ':').collect();
        if parts.len() != 2 {
            return None;
        }
        match parts[0] {
            "section" => match parts[1] {
                "Nodes" => Some(Self::Section(SectionKind::Nodes)),
                "Arcs" => Some(Self::Section(SectionKind::Arcs)),
                _ => None,
            },
            "realm" => Some(Self::Realm(parts[1].to_string())),
            "layer" => {
                let sub: Vec<&str> = parts[1].splitn(2, ':').collect();
                if sub.len() == 2 {
                    Some(Self::Layer {
                        realm: sub[0].to_string(),
                        layer: sub[1].to_string(),
                    })
                } else {
                    None
                }
            }
            "class" => Some(Self::Class(parts[1].to_string())),
            "instance" => {
                let sub: Vec<&str> = parts[1].splitn(2, ':').collect();
                if sub.len() == 2 {
                    Some(Self::Instance {
                        class: sub[0].to_string(),
                        key: sub[1].to_string(),
                    })
                } else {
                    None
                }
            }
            "arcfamily" => Some(Self::ArcFamily(parts[1].to_string())),
            "arcclass" => Some(Self::ArcClass(parts[1].to_string())),
            _ => None,
        }
    }
}

// ============================================================================
// Unified Node Structure
// ============================================================================

/// A unified tree node that can represent any type of graph element.
///
/// v11.7 Principle: "If it's a node in Neo4j, it's a node everywhere"
#[derive(Debug, Clone)]
pub struct UnifiedNode {
    /// Unique identifier for this node
    pub id: NodeId,
    /// Depth in tree (0=section, 1=realm/family, 2=layer, 3=class, 4=instance)
    pub depth: u8,
    /// Display properties (icon, label, badge, etc.)
    pub display: NodeDisplay,
    /// Child loading state
    pub children: LazyChildren,
    /// Neo4j labels for this node (e.g., [":Meta", ":Realm"])
    pub neo4j_labels: SmallVec<[String; 2]>,
}

/// Display properties for rendering a tree node.
#[derive(Debug, Clone)]
pub struct NodeDisplay {
    /// Unicode icon character (e.g., '◉', '⚙', '◆')
    pub icon: char,
    /// Display label
    pub label: String,
    /// Optional right-side badge
    pub badge: Option<Badge>,
    /// Child/instance count (shown in parentheses)
    pub count: Option<usize>,
    /// Arc and property statistics
    pub stats: Option<NodeStats>,
    /// Trait abbreviation for Class nodes (e.g., "def", "aut", "gen")
    pub trait_abbrev: Option<&'static str>,
}

/// Badge displayed on the right side of a tree line.
#[derive(Debug, Clone, Copy)]
pub struct Badge {
    /// Icon character
    pub icon: char,
    /// Short abbreviation (e.g., "fam", "arc", "shd", "org")
    pub abbrev: &'static str,
    /// Color lookup key (e.g., "realm.shared", "layer.config")
    pub color_key: &'static str,
}

/// Statistics for a node (arcs, properties).
#[derive(Debug, Clone, Default)]
pub struct NodeStats {
    /// Number of outgoing arcs
    pub outgoing_arcs: u16,
    /// Number of incoming arcs
    pub incoming_arcs: u16,
    /// Required properties count
    pub required_props: u8,
    /// Total properties count
    pub total_props: u8,
}

// ============================================================================
// Lazy Loading
// ============================================================================

/// State of child nodes (supports lazy loading).
#[derive(Debug, Clone)]
pub enum LazyChildren {
    /// Children not yet loaded
    NotLoaded,
    /// Currently loading children
    Loading,
    /// Children loaded (may have more available)
    Loaded {
        /// Loaded child node IDs
        items: Vec<NodeId>,
        /// Total available (may be > items.len())
        total: usize,
        /// True if more pages available
        has_more: bool,
    },
    /// Leaf node (no children possible)
    Leaf,
}

impl LazyChildren {
    /// Check if this node has loadable children.
    pub fn can_expand(&self) -> bool {
        !matches!(self, Self::Leaf)
    }

    /// Check if children are currently loaded.
    pub fn is_loaded(&self) -> bool {
        matches!(self, Self::Loaded { .. })
    }

    /// Check if currently loading.
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    /// Get loaded items if available.
    pub fn items(&self) -> Option<&[NodeId]> {
        match self {
            Self::Loaded { items, .. } => Some(items),
            _ => None,
        }
    }
}

// ============================================================================
// Pagination Constants
// ============================================================================

/// Number of instances to load initially when expanding a Class.
pub const INITIAL_INSTANCE_BATCH: usize = 10;

/// Number of instances to load on "Load more".
pub const INSTANCE_PAGE_SIZE: usize = 50;

/// Maximum instances to load before showing "too many" warning.
pub const MAX_INSTANCE_DISPLAY: usize = 1000;

// ============================================================================
// Async Communication
// ============================================================================

/// Commands sent from TUI to async worker.
#[derive(Debug, Clone)]
pub enum AsyncCommand {
    /// Load instances for a Class
    LoadInstances {
        class: String,
        offset: usize,
        limit: usize,
    },
    /// Load details for a Realm node
    LoadRealmDetails(String),
    /// Load details for a Layer node
    LoadLayerDetails { realm: String, layer: String },
    /// Load details for an ArcFamily node
    LoadArcFamilyDetails(String),
    /// Load details for an ArcClass node
    LoadArcClassDetails(String),
    /// Refresh the entire tree
    RefreshTree,
    /// Shutdown the async worker
    Shutdown,
}

/// Events sent from async worker to TUI.
#[derive(Debug)]
pub enum AsyncEvent {
    /// Instances loaded for a Class
    InstancesLoaded(InstanceLoadResponse),
    /// Realm details loaded
    RealmDetailsLoaded(RealmDetails),
    /// Layer details loaded
    LayerDetailsLoaded(LayerDetails),
    /// ArcFamily details loaded
    ArcFamilyDetailsLoaded(ArcFamilyDetails),
    /// ArcClass details loaded
    ArcClassDetailsLoaded(ArcClassDetails),
    /// Tree data refreshed
    TreeRefreshed(Box<UnifiedTreeData>),
    /// Error occurred
    Error(String),
}

// ============================================================================
// Response Types
// ============================================================================

/// Response from loading instances.
#[derive(Debug)]
pub struct InstanceLoadResponse {
    pub kind: String,
    pub instances: Vec<InstanceInfo>,
    pub total: usize,
    pub offset: usize,
}

/// Information about a single instance.
#[derive(Debug, Clone)]
pub struct InstanceInfo {
    pub key: String,
    pub display_name: String,
    pub labels: Vec<String>,
}

/// Details for a Realm node panel.
#[derive(Debug)]
pub struct RealmDetails {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub description: String,
    pub layer_count: usize,
    pub kind_count: usize,
    pub instance_count: usize,
}

/// Details for a Layer node panel.
#[derive(Debug)]
pub struct LayerDetails {
    pub key: String,
    pub realm: String,
    pub display_name: String,
    pub color: String,
    pub description: String,
    pub kind_count: usize,
    pub instance_count: usize,
}

/// Details for an ArcFamily node panel.
#[derive(Debug)]
pub struct ArcFamilyDetails {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub description: String,
    pub arc_kind_count: usize,
    pub instance_count: usize,
}

/// Details for an ArcKind node panel.
#[derive(Debug)]
pub struct ArcClassDetails {
    pub name: String,
    pub family: String,
    pub source: String,
    pub target: String,
    pub cardinality: String,
    pub description: String,
    pub instance_count: usize,
}

/// Complete unified tree data.
#[derive(Debug)]
pub struct UnifiedTreeData {
    pub nodes: HashMap<NodeId, UnifiedNode>,
    pub root_order: Vec<NodeId>,
    pub collapsed: FxHashSet<String>,
}

// ============================================================================
// Badge Presets
// ============================================================================

/// Pre-defined badges for common node types.
pub mod badges {
    use super::Badge;

    // Realm badges
    pub const REALM_SHARED: Badge = Badge {
        icon: '◎',
        abbrev: "shd",
        color_key: "realm.shared",
    };
    pub const REALM_ORG: Badge = Badge {
        icon: '●',
        abbrev: "org",
        color_key: "realm.org",
    };

    // Meta-type badges
    pub const ARC_FAMILY: Badge = Badge {
        icon: '●',
        abbrev: "fam",
        color_key: "arc.family",
    };
    pub const ARC_KIND: Badge = Badge {
        icon: '●',
        abbrev: "arc",
        color_key: "arc.kind",
    };

    // Layer badges
    pub const LAYER_CONFIG: Badge = Badge {
        icon: '◎',
        abbrev: "cfg",
        color_key: "layer.config",
    };
    pub const LAYER_LOCALE: Badge = Badge {
        icon: '◎',
        abbrev: "loc",
        color_key: "layer.locale",
    };
    pub const LAYER_GEOGRAPHY: Badge = Badge {
        icon: '▧',
        abbrev: "geo",
        color_key: "layer.geography",
    };
    pub const LAYER_KNOWLEDGE: Badge = Badge {
        icon: '◇',
        abbrev: "kno",
        color_key: "layer.knowledge",
    };
    pub const LAYER_FOUNDATION: Badge = Badge {
        icon: '▤',
        abbrev: "fnd",
        color_key: "layer.foundation",
    };
    pub const LAYER_STRUCTURE: Badge = Badge {
        icon: '▣',
        abbrev: "str",
        color_key: "layer.structure",
    };
    pub const LAYER_SEMANTIC: Badge = Badge {
        icon: '◆',
        abbrev: "sem",
        color_key: "layer.semantic",
    };
    pub const LAYER_INSTRUCTION: Badge = Badge {
        icon: '▥',
        abbrev: "ins",
        color_key: "layer.instruction",
    };
    pub const LAYER_OUTPUT: Badge = Badge {
        icon: '●',
        abbrev: "out",
        color_key: "layer.output",
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_collapse_key_roundtrip() {
        let ids = vec![
            NodeId::Section(SectionKind::Nodes),
            NodeId::Realm("shared".to_string()),
            NodeId::Layer {
                realm: "shared".to_string(),
                layer: "config".to_string(),
            },
            NodeId::Class("Locale".to_string()),
            NodeId::Instance {
                kind: "Locale".to_string(),
                key: "fr-FR".to_string(),
            },
            NodeId::ArcFamily("ownership".to_string()),
            NodeId::ArcKind("HAS_PAGE".to_string()),
        ];

        for id in ids {
            let key = id.collapse_key();
            let parsed = NodeId::from_collapse_key(&key);
            assert_eq!(parsed, Some(id.clone()), "Failed roundtrip for {:?}", id);
        }
    }

    #[test]
    fn test_lazy_children_states() {
        assert!(LazyChildren::NotLoaded.can_expand());
        assert!(LazyChildren::Loading.is_loading());
        assert!(!LazyChildren::Leaf.can_expand());

        let loaded = LazyChildren::Loaded {
            items: vec![NodeId::Class("Test".to_string())],
            total: 10,
            has_more: true,
        };
        assert!(loaded.is_loaded());
        assert_eq!(loaded.items().map(|i| i.len()), Some(1));
    }
}
