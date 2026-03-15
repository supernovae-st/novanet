//! Type definitions for TUI data structures.
//!
//! All structs and enums used by the taxonomy tree, instances, and tree navigation.

use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

// ============================================================================
// Schema Tree Types
// ============================================================================

/// Arc type for a Class (from schema).
#[derive(Debug, Clone)]
pub struct ArcInfo {
    pub arc_type: String,
    pub direction: ArcDirection,
    pub target_class: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcDirection {
    Outgoing, // →
    Incoming, // ←
}

/// An ArcClass in the arcs tree.
#[derive(Debug, Clone)]
pub struct ArcClassInfo {
    pub key: String,
    pub display_name: String,
    pub from_class: String,
    pub to_class: String,
    pub cardinality: String,
    pub description: String,
}

/// ArcFamily containing ArcClasss.
#[derive(Debug, Clone)]
pub struct ArcFamilyInfo {
    pub key: String,
    pub display_name: String,
    pub arc_classes: Vec<ArcClassInfo>,
    /// Content description for this arc family (from arc-families/*.yaml).
    pub content: String,
}

/// A Class in the taxonomy tree.
#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub icon: String,
    pub instance_count: i64,
    pub arcs: Vec<ArcInfo>,
    pub yaml_path: String,
    // Schema properties (from Neo4j Class node)
    pub properties: Vec<String>,
    pub required_properties: Vec<String>,
    pub schema_hint: String,
    pub context_budget: String,
    /// v10 knowledge tier (technical/style/semantic) — only for knowledge-layer nodes.
    pub knowledge_tier: Option<String>,
    // Health stats for tree badges (Feature 2)
    /// Coverage percentage (0-100) — instances with all required fields filled.
    pub health_percent: Option<u8>,
    /// Number of instances with missing required fields.
    pub issues_count: Option<usize>,
}

/// Layer containing Classes.
#[derive(Debug, Clone)]
pub struct LayerInfo {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub classes: Vec<ClassInfo>,
    /// Content description for this layer (from layers/*.yaml).
    pub content: String,
}

impl LayerInfo {
    /// Calculate health rollup for this layer (average of all Classes with health data).
    /// Returns (average_percent, total_issues) or None if no health data available.
    pub fn health_rollup(&self) -> Option<(u8, usize)> {
        let classes_with_health: Vec<_> = self
            .classes
            .iter()
            .filter_map(|k| k.health_percent.map(|p| (p, k.issues_count.unwrap_or(0))))
            .collect();

        if classes_with_health.is_empty() {
            return None;
        }

        let total_percent: u32 = classes_with_health.iter().map(|(p, _)| *p as u32).sum();
        let total_issues: usize = classes_with_health.iter().map(|(_, i)| *i).sum();
        let avg_percent = (total_percent / classes_with_health.len() as u32) as u8;

        Some((avg_percent, total_issues))
    }
}

/// Realm containing Layers.
#[derive(Debug, Clone)]
pub struct RealmInfo {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub icon: &'static str,
    pub layers: Vec<LayerInfo>,
    /// Content description for this realm (from realms/*.yaml).
    pub content: String,
}

impl RealmInfo {
    /// Total number of classes across all layers in this realm.
    pub fn total_classes(&self) -> usize {
        self.layers.iter().map(|l| l.classes.len()).sum()
    }

    /// Calculate health rollup for this realm (average of all Classes across all Layers).
    /// Returns (average_percent, total_issues) or None if no health data available.
    pub fn health_rollup(&self) -> Option<(u8, usize)> {
        let classes_with_health: Vec<_> = self
            .layers
            .iter()
            .flat_map(|l| l.classes.iter())
            .filter_map(|k| k.health_percent.map(|p| (p, k.issues_count.unwrap_or(0))))
            .collect();

        if classes_with_health.is_empty() {
            return None;
        }

        let total_percent: u32 = classes_with_health.iter().map(|(p, _)| *p as u32).sum();
        let total_issues: usize = classes_with_health.iter().map(|(_, i)| *i).sum();
        let avg_percent = (total_percent / classes_with_health.len() as u32) as u8;

        Some((avg_percent, total_issues))
    }
}

/// Stats for status bar.
#[derive(Debug, Clone, Default)]
pub struct GraphStats {
    pub node_count: i64,
    pub arc_count: i64,
    pub class_count: i64,
    pub arc_class_count: i64,
}

// ============================================================================
// Entity Category Hierarchy (Data mode)
// ============================================================================

/// EntityCategory for grouping Entity instances by semantic type.
/// Used in Data mode to show Entity instances organized by category.
#[derive(Debug, Clone)]
pub struct EntityCategory {
    /// Category key in UPPER_SNAKE_CASE (e.g., "THING", "ACTION", "FEATURE")
    pub key: String,
    /// Human-readable category name
    pub display_name: String,
    /// Display order (lower = first)
    pub sort_order: i64,
    /// Category question (WHAT?, HOW?, WHY?, etc.)
    pub question: String,
    /// Content description for generation hints
    pub content: String,
    /// Number of Entity instances in this category.
    pub instance_count: i64,
}

/// EntityNative info with parent Entity reference.
/// Used for displaying natives grouped by locale with defined parent name.
#[derive(Debug, Clone)]
pub struct EntityNativeInfo {
    /// EntityNative key (e.g., "qr-code@fr-FR")
    pub key: String,
    /// Native display name (e.g., "Créer un QR Code")
    pub display_name: String,
    /// Parent Entity key (e.g., "qr-code")
    pub entity_key: String,
    /// Parent Entity display name (defined, e.g., "QR Code")
    pub entity_display_name: String,
    /// Locale code (e.g., "fr-FR")
    pub locale_code: String,
    /// URL slug from denomination_forms
    pub slug: Option<String>,
    /// Relationship power (0-100) based on completeness
    pub relationship_power: u8,
    /// All properties from Neo4j for display in INSTANCE panel.
    /// v0.17.3: Added to show full instance details like Entity instances.
    pub properties: BTreeMap<String, JsonValue>,
}

/// Group of EntityNatives by parent Entity
#[derive(Debug, Clone)]
pub struct EntityNativeGroup {
    /// Parent Entity key (e.g., "qr-code")
    pub entity_key: String,
    /// Parent Entity display name
    pub entity_display_name: String,
    /// Number of natives for this entity
    pub native_count: usize,
    /// Relationship power (based on native count)
    pub relationship_power: u8,
}

// ============================================================================
// Neo4j Arc Data (live query)
// ============================================================================

/// A single arc relationship from Neo4j.
#[derive(Debug, Clone)]
pub struct Neo4jArc {
    pub arc_key: String,     // e.g., "FALLBACK_TO"
    pub other_class: String, // The Class on the other end
    pub family: String,      // e.g., "localization", "ownership"
}

/// Complete arc data for a Class, loaded from Neo4j.
#[derive(Debug, Clone, Default)]
pub struct ClassArcsData {
    pub class_label: String,
    pub realm: String,
    pub layer: String,
    pub incoming: Vec<Neo4jArc>,
    pub outgoing: Vec<Neo4jArc>,
}

/// Endpoint info for an ArcClass (from/to Class).
#[derive(Debug, Clone)]
pub struct ArcEndpoint {
    pub class_label: String,
    pub realm: String,
    pub layer: String,
}

/// Complete details for an ArcClass, loaded from Neo4j.
#[derive(Debug, Clone, Default)]
pub struct ArcClassDetails {
    pub display_name: String,
    pub description: String,
    pub family: String,
    pub cardinality: String,
    pub cypher_pattern: String,
    pub from_endpoint: Option<ArcEndpoint>,
    pub to_endpoint: Option<ArcEndpoint>,
}

/// Layer stats for Realm details view.
#[derive(Debug, Clone)]
pub struct LayerStats {
    pub key: String,
    pub display_name: String,
    pub class_count: usize,
}

/// Complete details for a Realm, loaded from Neo4j.
#[derive(Debug, Clone, Default)]
pub struct RealmDetails {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub layers: Vec<LayerStats>,
    pub total_classes: usize,
    pub total_instances: usize,
}


/// Complete details for a Layer, loaded from Neo4j.
#[derive(Debug, Clone, Default)]
pub struct LayerDetails {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub realm: String,
    /// Flat list of class names
    pub class_names: Vec<String>,
    pub total_classes: usize,
    pub total_instances: usize,
}

// ============================================================================
// Tree Navigation Types
// ============================================================================

/// Item type at a tree position.
#[derive(Debug, Clone)]
pub enum TreeItem<'a> {
    // Section headers
    ClassesSection,
    ArcsSection,
    // Classes hierarchy
    Realm(&'a RealmInfo),
    Layer(&'a RealmInfo, &'a LayerInfo),
    Class(&'a RealmInfo, &'a LayerInfo, &'a ClassInfo),
    // Arcs hierarchy
    ArcFamily(&'a ArcFamilyInfo),
    ArcClass(&'a ArcFamilyInfo, &'a ArcClassInfo),
    // Data view: Entity categories (between Class and instances for Entity only)
    EntityCategory(
        &'a RealmInfo,
        &'a LayerInfo,
        &'a ClassInfo,
        &'a EntityCategory,
    ),
    // Data view: Entity groups (between Class and instances for EntityNative only)
    // v0.17.3: Groups EntityNatives by parent Entity
    EntityGroup(
        &'a RealmInfo,
        &'a LayerInfo,
        &'a ClassInfo,
        &'a EntityNativeGroup,
    ),
    // Data view: instances under Classes (or under EntityCategory for Entity)
    Instance(
        &'a RealmInfo,
        &'a LayerInfo,
        &'a ClassInfo,
        &'a InstanceInfo,
    ),
    /// Data view: EntityNative items (locale-grouped, v0.17.3)
    EntityNativeItem(
        &'a RealmInfo,
        &'a LayerInfo,
        &'a ClassInfo,
        &'a EntityNativeInfo,
    ),
}

/// Hierarchical position info for compact display in tree title.
/// Format: "R:1/2 L:2/4 K:3/7 I:42/300"
#[derive(Debug, Clone, Default)]
pub struct HierarchyPosition {
    /// Current realm index (1-based) and total realms
    pub realm: Option<(usize, usize)>,
    /// Current layer index (1-based) within realm and total layers in realm
    pub layer: Option<(usize, usize)>,
    /// Current class index (1-based) within layer and total classes in layer
    pub class: Option<(usize, usize)>,
    /// Current instance index (1-based) and total instances for this class
    pub instance: Option<(usize, usize)>,
}

impl HierarchyPosition {
    /// Format as compact string: "R:1/2 L:2/4 C:3/7 I:42/300"
    pub fn to_compact_string(&self) -> String {
        let mut parts = Vec::new();
        if let Some((cur, total)) = self.realm {
            parts.push(format!("R:{}/{}", cur, total));
        }
        if let Some((cur, total)) = self.layer {
            parts.push(format!("L:{}/{}", cur, total));
        }
        if let Some((cur, total)) = self.class {
            parts.push(format!("C:{}/{}", cur, total));
        }
        if let Some((cur, total)) = self.instance {
            parts.push(format!("I:{}/{}", cur, total));
        }
        parts.join(" ")
    }
}

// ============================================================================
// DATA VIEW: Instance support (v10.6)
// ============================================================================

/// An instance of a Class in the data graph.
#[derive(Debug, Clone)]
pub struct InstanceInfo {
    pub key: String,
    pub display_name: String,
    pub class_key: String,
    /// Properties as JSON values (properly typed, not debug strings).
    pub properties: BTreeMap<String, JsonValue>,
    /// Outgoing arcs from this instance.
    pub outgoing_arcs: Vec<InstanceArc>,
    /// Incoming arcs to this instance.
    pub incoming_arcs: Vec<InstanceArc>,
    /// Whether arcs are still being loaded (progressive loading).
    pub arcs_loading: bool,
    /// Count of missing required properties (for tree badge).
    pub missing_required_count: usize,
    /// Count of filled (non-null/non-empty) properties.
    pub filled_properties: usize,
    /// Total properties in schema for this Class.
    pub total_properties: usize,
    /// URL slug from denomination_forms (for Entity instances).
    pub entity_slug: Option<String>,
    /// Relationship power score (0-100) based on HAS_NATIVE arc count.
    /// Used for power bar visualization: ▰▰▰▰▰▰▰▰▱▱
    pub relationship_power: u8,
}

/// An actual arc connection from/to an instance.
#[derive(Debug, Clone)]
pub struct InstanceArc {
    pub arc_type: String,
    pub target_key: String,
    pub target_class: String,
    /// True if this arc exists, false if it's from schema but not yet created.
    pub exists: bool,
    /// Display name of target node (for HAS_NATIVE arcs, shows EntityNative name).
    pub target_display_name: Option<String>,
    /// URL slug from denomination_forms (for EntityNative nodes).
    pub target_slug: Option<String>,
}
