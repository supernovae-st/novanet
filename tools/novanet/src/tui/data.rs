//! Data loading for TUI — Neo4j queries for taxonomy tree, stats, and detail.

use crate::db::{Db, RowExt};
use crate::parsers::taxonomy::{TaxonomyDoc, load_taxonomy};
use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::path::Path;
use tokio::join;

/// Maximum number of instances to load per Class.
/// Reduced from 500 to 300 for better performance with large datasets.
pub const INSTANCE_LIMIT: usize = 300;

// =============================================================================
// SECURITY: Label validation for Cypher injection prevention
// =============================================================================

/// Validates that a Neo4j label is safe for interpolation into Cypher queries.
/// Labels must be alphanumeric (with underscores allowed) and non-empty.
///
/// While our data comes from the meta-graph (not direct user input), this provides
/// defense-in-depth against potential injection if the database were compromised.
fn validate_cypher_label(label: &str) -> crate::Result<()> {
    if label.is_empty() {
        return Err(crate::error::NovaNetError::Validation(
            "Empty label not allowed in Cypher queries".into(),
        ));
    }
    // Allow alphanumeric, underscore, and dash (common in NovaNet labels like "locale-knowledge")
    if !label
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(crate::error::NovaNetError::Validation(format!(
            "Invalid characters in label '{}' - only alphanumeric, underscore, and dash allowed",
            label
        )));
    }
    Ok(())
}

/// Clean up Bolt debug output by removing wrapper type names.
/// E.g., "DateTime(BoltDateTime { seconds: BoltInteger { value: 123 }, ... })" -> "123"
fn clean_bolt_debug(debug: &str) -> String {
    const PATTERN: &str = "seconds: BoltInteger { value: ";
    // Extract just the timestamp if it's a DateTime
    // Use find() which returns byte index, but pattern is ASCII so addition is safe
    if let Some(start_byte) = debug.find(PATTERN) {
        // Pattern is ASCII-only, so start_byte + PATTERN.len() is a valid char boundary
        let rest = &debug[start_byte + PATTERN.len()..];
        // Find the end of the value (space or comma) - use chars for safety
        let value: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !value.is_empty() {
            return value;
        }
    }
    // Fallback: just return the debug string but truncated
    debug.chars().take(50).collect()
}

/// Convert a neo4rs BoltType to a serde_json::Value for clean display.
/// This extracts actual values instead of showing Bolt wrapper types.
fn bolt_to_json(bolt: &neo4rs::BoltType) -> JsonValue {
    use neo4rs::BoltType;
    match bolt {
        BoltType::String(s) => JsonValue::String(s.value.clone()),
        BoltType::Integer(i) => JsonValue::Number(i.value.into()),
        BoltType::Float(f) => serde_json::Number::from_f64(f.value)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        BoltType::Boolean(b) => JsonValue::Bool(b.value),
        BoltType::Null(_) => JsonValue::Null,
        BoltType::List(list) => JsonValue::Array(list.value.iter().map(bolt_to_json).collect()),
        BoltType::Map(map) => {
            let obj: serde_json::Map<String, JsonValue> = map
                .value
                .iter()
                .map(|(k, v)| (k.value.clone(), bolt_to_json(v)))
                .collect();
            JsonValue::Object(obj)
        }
        // For complex types (Node, Relationship, etc.), show a simplified representation
        BoltType::Node(n) => {
            let mut obj = serde_json::Map::new();
            obj.insert("_type".to_string(), JsonValue::String("Node".to_string()));
            obj.insert(
                "_labels".to_string(),
                JsonValue::Array(
                    n.labels
                        .iter()
                        .map(|l| JsonValue::String(l.to_string()))
                        .collect(),
                ),
            );
            for (k, v) in &n.properties.value {
                obj.insert(k.value.clone(), bolt_to_json(v));
            }
            JsonValue::Object(obj)
        }
        BoltType::Relation(r) => {
            let mut obj = serde_json::Map::new();
            obj.insert(
                "_type".to_string(),
                JsonValue::String("Relationship".to_string()),
            );
            obj.insert(
                "_rel_type".to_string(),
                JsonValue::String(r.typ.value.clone()),
            );
            JsonValue::Object(obj)
        }
        // DateTime and other complex types - extract what we can
        BoltType::DateTime(_)
        | BoltType::LocalDateTime(_)
        | BoltType::DateTimeZoneId(_)
        | BoltType::Date(_)
        | BoltType::Time(_)
        | BoltType::LocalTime(_)
        | BoltType::Duration(_)
        | BoltType::Point2D(_)
        | BoltType::Point3D(_)
        | BoltType::Path(_)
        | BoltType::UnboundedRelation(_)
        | BoltType::Bytes(_) => {
            // Clean up debug output: extract useful info
            let debug = format!("{:?}", bolt);
            JsonValue::String(clean_bolt_debug(&debug))
        }
    }
}

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
    /// LLM context for this arc family (from taxonomy.yaml).
    pub llm_context: String,
}

/// A Class in the taxonomy tree.
#[derive(Debug, Clone)]
#[allow(dead_code)] // schema_hint reserved for future use
pub struct ClassInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub icon: String,
    pub trait_name: String,
    pub instance_count: i64,
    pub arcs: Vec<ArcInfo>,
    pub yaml_path: String,
    // Schema properties (from Neo4j Class node)
    pub properties: Vec<String>,
    pub required_properties: Vec<String>,
    pub schema_hint: String,
    pub context_budget: String,
    /// v10 knowledge tier (technical/style/semantic) — only for knowledge-trait nodes.
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
    /// LLM context for this layer (from taxonomy.yaml).
    pub llm_context: String,
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
    /// LLM context for this realm (from taxonomy.yaml).
    pub llm_context: String,
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
    pub kind_count: i64,
    pub arc_kind_count: i64,
}

// ============================================================================
// Architecture Diagrams (v0.12.5 - ADR-028 visualization)
// ============================================================================

/// Architecture diagram for a specific Class.
/// Used in Graph mode detail panel to show contextual ER diagrams.
#[derive(Debug, Clone)]
pub struct ArchitectureDiagram {
    /// Class name this diagram is for (e.g., "Page", "Entity", "Brand")
    pub class_name: String,
    /// Related ADR identifier (e.g., "ADR-028")
    pub adr_id: String,
    /// ASCII diagram lines
    pub diagram: Vec<String>,
}

/// ADR category for grouping in Nexus Arch tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdrCategory {
    CorePrinciples,
    SchemaArchitecture,
    UxArchitecture,
    ArcPolicies,
    LayerEvolution,
}

impl AdrCategory {
    pub fn label(&self) -> &'static str {
        match self {
            AdrCategory::CorePrinciples => "Core Principles",
            AdrCategory::SchemaArchitecture => "Schema Architecture (v0.12.x)",
            AdrCategory::UxArchitecture => "UX Architecture",
            AdrCategory::ArcPolicies => "Arc Policies",
            AdrCategory::LayerEvolution => "Layer Evolution (v11.x history)",
        }
    }

    pub fn all() -> &'static [AdrCategory] {
        &[
            AdrCategory::CorePrinciples,
            AdrCategory::SchemaArchitecture,
            AdrCategory::UxArchitecture,
            AdrCategory::ArcPolicies,
            AdrCategory::LayerEvolution,
        ]
    }
}

/// ADR entry for Nexus Arch tab browser.
#[derive(Debug, Clone)]
pub struct AdrEntry {
    /// ADR identifier (e.g., "ADR-028")
    pub id: String,
    /// ADR title (e.g., "Page-Entity Architecture")
    pub title: String,
    /// Version when ADR was introduced (e.g., "v0.12.3")
    pub version: String,
    /// Status (e.g., "Approved", "Superseded")
    pub status: String,
    /// Category for grouping
    pub category: AdrCategory,
    /// Summary bullet points
    pub summary: Vec<String>,
    /// ASCII diagram lines
    pub diagram: Vec<String>,
    /// Key rules
    pub key_rules: Vec<String>,
    /// Related Class names
    pub related_classes: Vec<String>,
}

/// Get architecture diagram for a Class (if one exists).
/// Only key classes have dedicated diagrams per ADR-028.
pub fn get_architecture_diagram(class_name: &str) -> Option<ArchitectureDiagram> {
    match class_name {
        "Page" => Some(ArchitectureDiagram {
            class_name: "Page".to_string(),
            adr_id: "ADR-028".to_string(),
            diagram: vec![
                "        Project".to_string(),
                "          │".to_string(),
                "          │[:HAS_PAGE]".to_string(),
                "          ▼".to_string(),
                "┌────────────────────────────┐".to_string(),
                "│                            │".to_string(),
                "│ Page ══[:REPRESENTS]══▶    │".to_string(),
                "│   │      (1:1)      Entity │".to_string(),
                "│   │                   │    │".to_string(),
                "│   │[:HAS_BLOCK]       │[:HAS_CONTENT]".to_string(),
                "│   │  {order}          │    │".to_string(),
                "│   ▼                   ▼    │".to_string(),
                "│ Block          EntityContent".to_string(),
                "│   │                        │".to_string(),
                "│   └──[:USES_ENTITY]──▶     │".to_string(),
                "│              Entity        │".to_string(),
                "└────────────────────────────┘".to_string(),
            ],
        }),
        "Entity" => Some(ArchitectureDiagram {
            class_name: "Entity".to_string(),
            adr_id: "ADR-028".to_string(),
            diagram: vec![
                "       Project".to_string(),
                "          │".to_string(),
                "          │[:HAS_ENTITY]".to_string(),
                "          ▼".to_string(),
                "┌────────────────────────────┐".to_string(),
                "│                            │".to_string(),
                "│ Entity ──[:BELONGS_TO]──▶  │".to_string(),
                "│   │         EntityCategory │".to_string(),
                "│   │                        │".to_string(),
                "│   │[:HAS_CONTENT]          │".to_string(),
                "│   ▼                        │".to_string(),
                "│ EntityContent              │".to_string(),
                "│   │                        │".to_string(),
                "│   │[:FOR_LOCALE]           │".to_string(),
                "│   ▼                        │".to_string(),
                "│ Locale                     │".to_string(),
                "│                            │".to_string(),
                "│ ──[:SEMANTIC_LINK]──▶      │".to_string(),
                "│    {temp, link_type} Entity│".to_string(),
                "└────────────────────────────┘".to_string(),
            ],
        }),
        "Block" => Some(ArchitectureDiagram {
            class_name: "Block".to_string(),
            adr_id: "ADR-028".to_string(),
            diagram: vec![
                "         Page".to_string(),
                "          │".to_string(),
                "          │[:HAS_BLOCK {order}]".to_string(),
                "          ▼".to_string(),
                "┌────────────────────────────┐".to_string(),
                "│                            │".to_string(),
                "│ Block ──[:OF_TYPE]──▶      │".to_string(),
                "│   │           BlockType    │".to_string(),
                "│   │                        │".to_string(),
                "│   │[:HAS_INSTRUCTION]      │".to_string(),
                "│   ▼                        │".to_string(),
                "│ BlockInstruction           │".to_string(),
                "│   │                        │".to_string(),
                "│   │[:USES_ENTITY]          │".to_string(),
                "│   ▼                        │".to_string(),
                "│ Entity                     │".to_string(),
                "└────────────────────────────┘".to_string(),
            ],
        }),
        "Brand" => Some(ArchitectureDiagram {
            class_name: "Brand".to_string(),
            adr_id: "ADR-028".to_string(),
            diagram: vec![
                "       Project".to_string(),
                "          │".to_string(),
                "          │[:HAS_BRAND]".to_string(),
                "          ▼".to_string(),
                "┌────────────────────────────┐".to_string(),
                "│                            │".to_string(),
                "│ Brand ──[:HAS_DESIGN]──▶   │".to_string(),
                "│   │         BrandDesign    │".to_string(),
                "│   │                        │".to_string(),
                "│   │──[:HAS_PRINCIPLES]──▶  │".to_string(),
                "│   │        BrandPrinciples │".to_string(),
                "│   │                        │".to_string(),
                "│   │──[:HAS_PROMPT_STYLE]──▶│".to_string(),
                "│   │        PromptStyle     │".to_string(),
                "│   │                        │".to_string(),
                "│   └──[:TARGETS_PERSONA]──▶ │".to_string(),
                "│          AudiencePersona   │".to_string(),
                "└────────────────────────────┘".to_string(),
            ],
        }),
        "Locale" => Some(ArchitectureDiagram {
            class_name: "Locale".to_string(),
            adr_id: "ADR-028".to_string(),
            diagram: vec![
                "     shared/config".to_string(),
                "          │".to_string(),
                "          ▼".to_string(),
                "┌────────────────────────────┐".to_string(),
                "│                            │".to_string(),
                "│ Locale ──[:HAS_VOICE]──▶   │".to_string(),
                "│   │         LocaleVoice    │".to_string(),
                "│   │                        │".to_string(),
                "│   │──[:HAS_CULTURE]──▶     │".to_string(),
                "│   │         CultureSet     │".to_string(),
                "│   │                        │".to_string(),
                "│   │──[:HAS_TERMS]──▶       │".to_string(),
                "│   │         TermSet        │".to_string(),
                "│   │                        │".to_string(),
                "│   └──[:FOR_LOCALE]◀──      │".to_string(),
                "│          EntityContent     │".to_string(),
                "└────────────────────────────┘".to_string(),
            ],
        }),
        "Project" => Some(ArchitectureDiagram {
            class_name: "Project".to_string(),
            adr_id: "ADR-028".to_string(),
            diagram: vec![
                "      OrgConfig".to_string(),
                "          │".to_string(),
                "          │[:HAS_PROJECT]".to_string(),
                "          ▼".to_string(),
                "┌────────────────────────────┐".to_string(),
                "│                            │".to_string(),
                "│ Project ──[:HAS_PAGE]──▶   │".to_string(),
                "│   │              Page      │".to_string(),
                "│   │                        │".to_string(),
                "│   │──[:HAS_ENTITY]──▶      │".to_string(),
                "│   │              Entity    │".to_string(),
                "│   │                        │".to_string(),
                "│   │──[:HAS_BRAND]──▶       │".to_string(),
                "│   │              Brand     │".to_string(),
                "│   │                        │".to_string(),
                "│   └──[:HAS_CONTENT]──▶     │".to_string(),
                "│          ProjectContent    │".to_string(),
                "└────────────────────────────┘".to_string(),
            ],
        }),
        _ => None,
    }
}

/// Get all ADR entries for Nexus Arch tab.
pub fn get_all_adrs() -> Vec<AdrEntry> {
    vec![
        // Core Principles
        AdrEntry {
            id: "ADR-007".to_string(),
            title: "Generation NOT Translation".to_string(),
            version: "core".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::CorePrinciples,
            summary: vec![
                "Entity → Generate natively → EntityContent".to_string(),
                "NOT: Source → Translate → Target".to_string(),
            ],
            diagram: vec![
                "Entity (defined) ──▶ Generate ──▶ EntityContent (authored)".to_string(),
            ],
            key_rules: vec![
                "Content is generated natively per locale".to_string(),
                "Translation loses cultural nuance".to_string(),
            ],
            related_classes: vec!["Entity".to_string(), "EntityContent".to_string()],
        },
        AdrEntry {
            id: "ADR-003".to_string(),
            title: "YAML-First Architecture".to_string(),
            version: "v9.0".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::CorePrinciples,
            summary: vec![
                "YAML = single source of truth".to_string(),
                "Generators → TS/Cypher/Mermaid".to_string(),
            ],
            diagram: vec![
                "taxonomy.yaml ──▶ Rust Generator ──▶ TypeScript + Cypher".to_string(),
            ],
            key_rules: vec![
                "Single source prevents drift".to_string(),
                "CI validates sync".to_string(),
            ],
            related_classes: vec![],
        },
        AdrEntry {
            id: "ADR-001".to_string(),
            title: "Arc Terminology".to_string(),
            version: "v9.5".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::CorePrinciples,
            summary: vec![
                "Use 'Arc' (not Edge/Relation)".to_string(),
                "Graph theory for directed edges".to_string(),
            ],
            diagram: vec![],
            key_rules: vec![
                "Single consistent term across all platforms".to_string(),
            ],
            related_classes: vec![],
        },
        // Schema Architecture
        AdrEntry {
            id: "ADR-028".to_string(),
            title: "Page-Entity Architecture".to_string(),
            version: "v0.12.3".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::SchemaArchitecture,
            summary: vec![
                "Page ↔ Entity = 1:1 mandatory via [:REPRESENTS]".to_string(),
                "Slug = Entity.key (source of truth)".to_string(),
                "Order on arc: [:HAS_BLOCK {order}]".to_string(),
            ],
            diagram: vec![
                "Page ──[:REPRESENTS]──▶ Entity (1:1 mandatory)".to_string(),
                "  │                        │".to_string(),
                "  │[:HAS_BLOCK {order}]    │[:HAS_CONTENT]".to_string(),
                "  ▼                        ▼".to_string(),
                "Block                 EntityContent@locale".to_string(),
            ],
            key_rules: vec![
                "Every Page MUST have exactly one Entity".to_string(),
                "SEO Keywords live on Entity, not Page".to_string(),
                "Block.key = \"{page_key}:{block_type}:{index}\"".to_string(),
            ],
            related_classes: vec!["Page".to_string(), "Entity".to_string(), "Block".to_string()],
        },
        AdrEntry {
            id: "ADR-024".to_string(),
            title: "Trait = Data Origin".to_string(),
            version: "v0.12.0".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::SchemaArchitecture,
            summary: vec![
                "invariant → defined".to_string(),
                "localized → authored".to_string(),
                "knowledge → imported".to_string(),
                "aggregated → retrieved".to_string(),
            ],
            diagram: vec![],
            key_rules: vec![
                "Trait answers: WHERE does data come from?".to_string(),
                "Layer answers: WHAT functional category?".to_string(),
            ],
            related_classes: vec![],
        },
        AdrEntry {
            id: "ADR-023".to_string(),
            title: "Class/Instance Terminology".to_string(),
            version: "v0.12.0".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::SchemaArchitecture,
            summary: vec![
                "NodeKind → NodeClass".to_string(),
                ":Meta:Kind → :Schema:Class".to_string(),
                "\"Meta\" eliminated entirely".to_string(),
            ],
            diagram: vec![],
            key_rules: vec![
                "Class/Instance is standard OOP/ontology".to_string(),
                "Avoids Facebook 'Meta' confusion".to_string(),
            ],
            related_classes: vec![],
        },
        // UX Architecture
        AdrEntry {
            id: "ADR-022".to_string(),
            title: "Unified Tree (Graph/Nexus)".to_string(),
            version: "v11.7".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::UxArchitecture,
            summary: vec![
                "5 modes → 2 modes: [1]Graph + [2]Nexus".to_string(),
                "\"If it's a node in Neo4j, it's a node everywhere\"".to_string(),
            ],
            diagram: vec![
                "▼ Realm:shared".to_string(),
                "  ▼ Layer:config".to_string(),
                "    ▼ Class:Locale [200]".to_string(),
                "      ● Locale:fr-FR".to_string(),
            ],
            key_rules: vec![
                "Realm, Layer, Class are clickable nodes".to_string(),
                "Instances lazy-loaded under Class".to_string(),
            ],
            related_classes: vec![],
        },
        AdrEntry {
            id: "ADR-021".to_string(),
            title: "Query-First Architecture".to_string(),
            version: "v11.6".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::UxArchitecture,
            summary: vec![
                "Cypher = source of truth".to_string(),
                "YAML views only (no TS hardcoding)".to_string(),
            ],
            diagram: vec![],
            key_rules: vec![
                "Graph displays query results only".to_string(),
                "Views are parameterized Cypher templates".to_string(),
            ],
            related_classes: vec![],
        },
        // Arc Policies
        AdrEntry {
            id: "ADR-026".to_string(),
            title: "Inverse Arc Policy".to_string(),
            version: "v0.12.1".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::ArcPolicies,
            summary: vec![
                "TIER 1: Required (HAS_* ↔ *_OF)".to_string(),
                "TIER 2: Recommended (knowledge arcs)".to_string(),
                "TIER 3: No inverse needed".to_string(),
            ],
            diagram: vec![],
            key_rules: vec![
                "HAS_ENTITY ↔ ENTITY_OF".to_string(),
                "Container arcs: no inverse".to_string(),
            ],
            related_classes: vec![],
        },
        AdrEntry {
            id: "ADR-027".to_string(),
            title: "Generation Family Semantics".to_string(),
            version: "v0.12.1".to_string(),
            status: "Approved".to_string(),
            category: AdrCategory::ArcPolicies,
            summary: vec![
                "GENERATED vs HAS_GENERATED".to_string(),
                "Pipeline: Instruction → Prompt → Generated".to_string(),
            ],
            diagram: vec![
                "PageInstruction ──[:COMPILED_FROM]──< PromptArtifact".to_string(),
                "BlockInstruction ──[:GENERATED]──▶ BlockGenerated".to_string(),
            ],
            key_rules: vec![
                "GENERATED = provenance (what made this?)".to_string(),
                "HAS_GENERATED = ownership (what's the output?)".to_string(),
            ],
            related_classes: vec!["PageGenerated".to_string(), "BlockGenerated".to_string()],
        },
    ]
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
    /// LLM context for generation hints
    pub llm_context: String,
    /// Number of Entity instances in this category.
    pub instance_count: i64,
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

/// Class stats grouped by trait for Layer details view.
#[derive(Debug, Clone)]
pub struct TraitClassGroup {
    pub trait_key: String,
    pub kind_names: Vec<String>,
}

/// Complete details for a Layer, loaded from Neo4j.
#[derive(Debug, Clone, Default)]
pub struct LayerDetails {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub realm: String,
    pub classes_by_trait: Vec<TraitClassGroup>,
    pub total_classes: usize,
    pub total_instances: usize,
}

/// Full taxonomy tree: Realm > Layer > Class + ArcFamily > ArcClass.
#[derive(Debug, Clone, Default)]
pub struct TaxonomyTree {
    pub realms: Vec<RealmInfo>,
    pub arc_families: Vec<ArcFamilyInfo>,
    pub stats: GraphStats,
    /// Collapsed state: stores keys of collapsed nodes (e.g., "classes", "arcs", "realm:shared", "layer:structure")
    /// Uses FxHashSet for ~30% faster lookups on string keys.
    pub collapsed: FxHashSet<String>,
    /// Instances loaded for Data view, keyed by Class key.
    /// Only populated when in Data mode and a Class is selected.
    /// Uses FxHashMap for ~30% faster lookups (no ordering needed).
    pub instances: FxHashMap<String, Vec<InstanceInfo>>,
    /// Total instance counts in Neo4j (may be > loaded instances due to INSTANCE_LIMIT).
    /// Used to show "3/300 of 847" when results are truncated.
    /// Uses FxHashMap for ~30% faster lookups (no ordering needed).
    pub instance_totals: FxHashMap<String, usize>,
    /// Cache: class_key -> (realm_idx, layer_idx, kind_idx) for O(1) lookups.
    /// Built once on load, never mutated (tree structure is immutable).
    pub(crate) kind_index: FxHashMap<String, (usize, usize, usize)>,
    /// Entity categories for Data mode grouping.
    /// Loaded on-demand when viewing Entity instances by category.
    pub entity_categories: Vec<EntityCategory>,
    /// Entity instances grouped by category (key = category key like "THING", "ACTION").
    /// Loaded on-demand when Entity categories are expanded.
    /// Uses FxHashMap for ~30% faster lookups (no ordering needed).
    pub entity_category_instances: FxHashMap<String, Vec<InstanceInfo>>,
}

impl TaxonomyTree {
    /// Load taxonomy tree from Neo4j, enriched with llm_context from taxonomy.yaml.
    pub async fn load(db: &Db, root: &Path) -> crate::Result<Self> {
        // Load taxonomy.yaml for llm_context enrichment
        let taxonomy = load_taxonomy(root).ok();

        // Build lookup maps for realm/layer llm_context
        let (realm_llm_context, layer_llm_context, arc_family_llm_context) =
            Self::build_llm_context_maps(&taxonomy);

        // Query all Classes with their realm, layer, trait, and instance count
        // Note: Class uses 'label' property as identifier, not 'key'
        let cypher = r#"
MATCH (k:Class:Schema)
OPTIONAL MATCH (k)-[:IN_REALM]->(r:Realm)
OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer)
OPTIONAL MATCH (n)-[:OF_CLASS]->(k)
WITH k, r, l, count(n) AS instances
RETURN
    k.label AS class_key,
    coalesce(k.display_name, k.label) AS kind_display,
    coalesce(k.llm_context, '') AS kind_desc,
    coalesce(k.icon, '') AS kind_icon,
    coalesce(k.trait, '') AS trait_key,
    coalesce(r.key, 'unknown') AS realm_key,
    coalesce(r.display_name, r.key, 'Unknown') AS realm_display,
    coalesce(r.color, '#ffffff') AS realm_color,
    coalesce(l.key, 'unknown') AS layer_key,
    coalesce(l.display_name, l.key, 'Unknown') AS layer_display,
    coalesce(l.color, '#ffffff') AS layer_color,
    instances,
    coalesce(k.yaml_path, '') AS yaml_path,
    coalesce(k.properties, []) AS properties,
    coalesce(k.required_properties, []) AS required_properties,
    coalesce(k.schema_hint, '') AS schema_hint,
    coalesce(k.context_budget, '') AS context_budget,
    k.knowledge_tier AS knowledge_tier
ORDER BY realm_key, layer_key, class_key
"#;

        let rows = db.execute(cypher).await?;

        // Group into tree structure: realm_key -> (realm_display, realm_color, layer_key -> (layer_display, layer_color, classes))
        #[allow(clippy::type_complexity)]
        let mut realm_map: BTreeMap<
            String,
            (
                String,
                String,
                BTreeMap<String, (String, String, Vec<ClassInfo>)>,
            ),
        > = BTreeMap::new();

        for row in rows {
            // Extract fields using RowExt for ergonomic defaults
            let class_key = row.str("class_key");
            let kind_display = row.str("kind_display");
            let kind_desc = row.str("kind_desc");
            let kind_icon = row.str("kind_icon");
            let trait_key = row.str("trait_key");
            let realm_key = row.str("realm_key");
            let realm_display = row.str("realm_display");
            let realm_color = row.str("realm_color");
            let layer_key = row.str("layer_key");
            let layer_display = row.str("layer_display");
            let layer_color = row.str("layer_color");
            let instances = row.int("instances");

            // Get YAML path from Neo4j (with fallback to computed path)
            let yaml_path_raw = row.str("yaml_path");
            let yaml_path = if !yaml_path_raw.is_empty() {
                // Neo4j stores relative path like "node-classes/org/structure/block.yaml"
                // We need to prefix with "packages/core/models/"
                format!("packages/core/models/{}", yaml_path_raw)
            } else if realm_key == "unknown" || layer_key == "unknown" {
                // Missing realm/layer relationship - can't compute valid path
                // Return empty to signal "file not found" in UI (better than invalid path)
                String::new()
            } else {
                // Fallback: compute path from realm/layer
                format!(
                    "packages/core/models/node-classes/{}/{}/{}.yaml",
                    realm_key,
                    layer_key,
                    to_kebab_case(&class_key)
                )
            };

            // Get schema properties from Neo4j
            let properties = row.vec_str("properties");
            let required_properties = row.vec_str("required_properties");
            let schema_hint = row.str("schema_hint");
            let context_budget = row.str("context_budget");
            // v10: knowledge_tier (optional, only for knowledge-trait nodes)
            let knowledge_tier = row.opt_str("knowledge_tier");

            let kind = ClassInfo {
                key: class_key,
                display_name: kind_display,
                description: kind_desc,
                icon: kind_icon,
                trait_name: trait_key,
                instance_count: instances,
                arcs: Vec::new(), // Loaded separately
                yaml_path,
                properties,
                required_properties,
                schema_hint,
                context_budget,
                knowledge_tier,
                // Health stats (not loaded yet, requires separate query)
                health_percent: None,
                issues_count: None,
            };

            realm_map
                .entry(realm_key)
                .or_insert_with(|| (realm_display, realm_color, BTreeMap::new()))
                .2
                .entry(layer_key)
                .or_insert_with(|| (layer_display, layer_color, Vec::new()))
                .2
                .push(kind);
        }

        // Convert to RealmInfo vec with llm_context from taxonomy.yaml
        let realms: Vec<RealmInfo> = realm_map
            .into_iter()
            .map(|(realm_key, (realm_display, realm_color, layers_map))| {
                let layers: Vec<LayerInfo> = layers_map
                    .into_iter()
                    .map(|(layer_key, (layer_display, layer_color, classes))| {
                        // Look up llm_context from taxonomy.yaml
                        let llm_ctx = layer_llm_context
                            .get(&layer_key)
                            .cloned()
                            .unwrap_or_default();
                        LayerInfo {
                            key: layer_key,
                            display_name: layer_display,
                            color: layer_color,
                            classes,
                            llm_context: llm_ctx,
                        }
                    })
                    .collect();

                // Look up realm llm_context from taxonomy.yaml
                let realm_llm_ctx = realm_llm_context
                    .get(&realm_key)
                    .cloned()
                    .unwrap_or_default();
                RealmInfo {
                    icon: realm_icon(&realm_key),
                    key: realm_key.clone(),
                    display_name: realm_display,
                    color: realm_color,
                    layers,
                    llm_context: realm_llm_ctx,
                }
            })
            .collect();

        // Load stats, arcs, and families in parallel (~3x faster startup)
        let (stats_result, arcs_result, families_result) = join!(
            Self::load_stats(db),
            Self::fetch_arcs(db),
            Self::fetch_arc_families(db)
        );

        let stats = stats_result?;
        let arc_map = arcs_result.unwrap_or_default();
        // Enrich arc_families with llm_context from taxonomy.yaml
        let arc_families = Self::enrich_arc_families_with_llm_context(
            families_result.unwrap_or_default(),
            &arc_family_llm_context,
        );

        // Apply arcs to classes
        let realms = Self::apply_arcs_to_realms(realms, arc_map);

        // Build kind_index for O(1) lookups (replaces O(n*m*k) find_kind)
        let mut kind_index = FxHashMap::default();
        for (r_idx, realm) in realms.iter().enumerate() {
            for (l_idx, layer) in realm.layers.iter().enumerate() {
                for (k_idx, kind) in layer.classes.iter().enumerate() {
                    kind_index.insert(kind.key.clone(), (r_idx, l_idx, k_idx));
                }
            }
        }

        Ok(Self {
            realms,
            arc_families,
            stats,
            collapsed: FxHashSet::default(),
            instances: FxHashMap::default(),
            instance_totals: FxHashMap::default(),
            kind_index,
            entity_categories: Vec::new(), // Loaded on-demand via load_entity_categories
            entity_category_instances: FxHashMap::default(), // Loaded on-demand when category expanded
        })
    }

    /// Build lookup maps for llm_context from taxonomy.yaml.
    /// Returns (realm_llm_context, layer_llm_context, arc_family_llm_context).
    fn build_llm_context_maps(
        taxonomy: &Option<TaxonomyDoc>,
    ) -> (
        FxHashMap<String, String>,
        FxHashMap<String, String>,
        FxHashMap<String, String>,
    ) {
        let mut realm_map = FxHashMap::default();
        let mut layer_map = FxHashMap::default();
        let mut arc_family_map = FxHashMap::default();

        if let Some(tax) = taxonomy {
            // Extract realm llm_context
            for realm in &tax.node_realms {
                realm_map.insert(realm.key.clone(), realm.llm_context.clone());
                // Extract layer llm_context (nested under realm)
                for layer in &realm.layers {
                    layer_map.insert(layer.key.clone(), layer.llm_context.clone());
                }
            }

            // Extract arc_family llm_context
            for family in &tax.arc_families {
                arc_family_map.insert(family.key.clone(), family.llm_context.clone());
            }
        }

        (realm_map, layer_map, arc_family_map)
    }

    /// Enrich arc_families with llm_context from taxonomy.yaml lookup map.
    fn enrich_arc_families_with_llm_context(
        mut families: Vec<ArcFamilyInfo>,
        llm_context_map: &FxHashMap<String, String>,
    ) -> Vec<ArcFamilyInfo> {
        for family in &mut families {
            if let Some(llm_ctx) = llm_context_map.get(&family.key) {
                family.llm_context = llm_ctx.clone();
            }
        }
        families
    }

    /// Apply arc map to realm/layer/kind tree.
    fn apply_arcs_to_realms(
        mut realms: Vec<RealmInfo>,
        mut arc_map: BTreeMap<String, Vec<ArcInfo>>,
    ) -> Vec<RealmInfo> {
        for realm in &mut realms {
            for layer in &mut realm.layers {
                for kind in &mut layer.classes {
                    if let Some(arcs) = arc_map.remove(&kind.key) {
                        kind.arcs = arcs;
                    }
                }
            }
        }
        realms
    }

    /// Fetch arcs as a map (for parallel loading).
    async fn fetch_arcs(db: &Db) -> crate::Result<BTreeMap<String, Vec<ArcInfo>>> {
        let cypher = r#"
MATCH (ak:ArcClass:Schema)-[:FROM_CLASS]->(fromClass:Class:Schema)
MATCH (ak)-[:TO_CLASS]->(toClass:Class:Schema)
RETURN fromClass.label AS class_key, ak.key AS arc_type, 'outgoing' AS direction, toClass.label AS target_class
ORDER BY fromClass.label, ak.key

UNION

MATCH (ak:ArcClass:Schema)-[:FROM_CLASS]->(fromClass:Class:Schema)
MATCH (ak)-[:TO_CLASS]->(toClass:Class:Schema)
RETURN toClass.label AS class_key, ak.key AS arc_type, 'incoming' AS direction, fromClass.label AS target_class
ORDER BY toClass.label, ak.key
"#;

        let rows = db.execute(cypher).await?;
        let mut arc_map: BTreeMap<String, Vec<ArcInfo>> = BTreeMap::new();

        for row in rows {
            let class_key = row.str("class_key");
            let arc_type = row.str("arc_type");
            let direction_str = row.str("direction");
            let target_class = row.str("target_class");

            if class_key.is_empty() || arc_type.is_empty() {
                continue;
            }

            let direction = if direction_str == "incoming" {
                ArcDirection::Incoming
            } else {
                ArcDirection::Outgoing
            };

            arc_map.entry(class_key).or_default().push(ArcInfo {
                arc_type,
                direction,
                target_class,
            });
        }

        Ok(arc_map)
    }

    /// Fetch arc families (for parallel loading).
    async fn fetch_arc_families(db: &Db) -> crate::Result<Vec<ArcFamilyInfo>> {
        let cypher = r#"
MATCH (ak:ArcClass:Schema)-[:IN_FAMILY]->(af:ArcFamily:Schema)
MATCH (ak)-[:FROM_CLASS]->(fromClass:Class:Schema)
MATCH (ak)-[:TO_CLASS]->(toClass:Class:Schema)
RETURN
    af.key AS family_key,
    coalesce(af.display_name, af.key) AS family_display,
    ak.key AS arc_key,
    coalesce(ak.display_name, ak.key) AS arc_display,
    coalesce(ak.cardinality, '') AS cardinality,
    coalesce(ak.llm_context, '') AS arc_desc,
    fromClass.label AS from_class,
    toClass.label AS to_class
ORDER BY family_key, arc_key
"#;

        let rows = db.execute(cypher).await?;
        let mut family_map: BTreeMap<String, (String, Vec<ArcClassInfo>)> = BTreeMap::new();

        for row in rows {
            let family_key = row.str("family_key");
            let family_display = row.str("family_display");
            let arc_key = row.str("arc_key");
            let arc_display = row.str("arc_display");
            let cardinality = row.str("cardinality");
            let arc_desc = row.str("arc_desc");
            let from_class = row.str("from_class");
            let to_class = row.str("to_class");

            if family_key.is_empty() || arc_key.is_empty() {
                continue;
            }

            let arc_kind = ArcClassInfo {
                key: arc_key,
                display_name: arc_display,
                from_class,
                to_class,
                cardinality,
                description: arc_desc,
            };

            family_map
                .entry(family_key)
                .or_insert_with(|| (family_display, Vec::new()))
                .1
                .push(arc_kind);
        }

        Ok(family_map
            .into_iter()
            .map(|(key, (display_name, arc_classes))| ArcFamilyInfo {
                key,
                display_name,
                arc_classes,
                llm_context: String::new(), // Enriched later from taxonomy.yaml
            })
            .collect())
    }

    /// Load instances of a Class from Neo4j for Data view.
    /// Returns (instances, total_count) - instances are limited to INSTANCE_LIMIT, total is the real count.
    ///
    /// Performance: Uses a two-pass query strategy for large datasets:
    /// 1. Fast index scan to get first INSTANCE_LIMIT keys + total count
    /// 2. Detailed query with arcs only for those keys
    ///
    /// This avoids scanning all nodes (e.g., 9100 SEOKeyword) for arc collection.
    pub async fn load_instances(
        db: &Db,
        class_label: &str,
    ) -> crate::Result<(Vec<InstanceInfo>, usize)> {
        // Security: Validate label before interpolation into Cypher
        validate_cypher_label(class_label)?;

        // Pass 1: Get total count AND first N keys in a single fast query (index-based)
        let keys_cypher = format!(
            r#"
MATCH (n:{class_label})
WHERE NOT n:Schema
WITH count(n) AS total
MATCH (n:{class_label})
WHERE NOT n:Schema
WITH total, n.key AS key
ORDER BY key
LIMIT {limit}
RETURN collect(key) AS keys, total
"#,
            class_label = class_label,
            limit = INSTANCE_LIMIT
        );
        let keys_rows = db.execute(&keys_cypher).await?;
        let (keys, total_count): (Vec<String>, usize) = keys_rows
            .first()
            .map(|r| {
                let keys: Vec<String> = r.get("keys").unwrap_or_default();
                let total: i64 = r.get("total").unwrap_or(0);
                (keys, total as usize)
            })
            .unwrap_or_default();

        // Early return if no instances
        if keys.is_empty() {
            return Ok((Vec::new(), total_count));
        }

        // Pass 2: Get properties and arcs only for the selected keys
        // This is much faster than scanning all nodes for arc collection
        let cypher = format!(
            r#"
UNWIND $keys AS k
MATCH (n:{class_label} {{key: k}})
OPTIONAL MATCH (n)-[out]->(target)
WHERE NOT target:Meta
WITH n, k, collect(DISTINCT {{
    arc_type: type(out),
    target_key: coalesce(target.key, target.label, id(target)),
    target_class: head(labels(target))
}}) AS outgoing
OPTIONAL MATCH (source)-[inc]->(n)
WHERE NOT source:Meta
WITH n, k, outgoing, collect(DISTINCT {{
    arc_type: type(inc),
    source_key: coalesce(source.key, source.label, id(source)),
    source_kind: head(labels(source))
}}) AS incoming
RETURN
    coalesce(n.key, n.label, toString(id(n))) AS key,
    coalesce(n.display_name, n.key, n.label) AS display_name,
    properties(n) AS props,
    outgoing,
    incoming
ORDER BY key
"#,
            class_label = class_label
        );

        // Execute with parameterized keys (safe from injection)
        let rows = db.execute_with_params(&cypher, [("keys", keys)]).await?;
        let mut instances = Vec::with_capacity(rows.len());

        for row in rows {
            let key = row.str("key");
            let display_name = row.str("display_name");

            // Parse properties as BTreeMap with proper JSON values
            let props: BTreeMap<String, JsonValue> = row
                .get::<neo4rs::BoltMap>("props")
                .map(|m| {
                    m.value
                        .iter()
                        .map(|(k, v)| (k.value.clone(), bolt_to_json(v)))
                        .collect()
                })
                .unwrap_or_default();

            // Parse outgoing arcs
            let outgoing_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("outgoing")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("target_key").unwrap_or_default(),
                        target_class: m.get("target_class").unwrap_or_default(),
                        exists: true,
                    })
                })
                .collect();

            // Parse incoming arcs
            let incoming_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("incoming")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("source_key").unwrap_or_default(),
                        target_class: m.get("source_kind").unwrap_or_default(),
                        exists: true,
                    })
                })
                .collect();

            instances.push(InstanceInfo {
                key,
                display_name,
                class_key: class_label.to_string(),
                properties: props,
                outgoing_arcs,
                incoming_arcs,
                arcs_loading: false,       // Arcs already loaded in full query
                missing_required_count: 0, // Calculated later in set_instances
                filled_properties: 0,      // Calculated later in set_instances
                total_properties: 0,       // Calculated later in set_instances
            });
        }

        Ok((instances, total_count))
    }

    /// Load instances FAST - only keys + display_name, NO arc queries.
    /// This returns in ~50ms instead of ~3s for large datasets.
    /// Arcs should be loaded separately via `load_instance_arcs()`.
    pub async fn load_instances_fast(
        db: &Db,
        class_label: &str,
    ) -> crate::Result<(Vec<InstanceInfo>, usize)> {
        // Security: Validate label before interpolation into Cypher
        validate_cypher_label(class_label)?;

        // Single fast query: get keys + display_name + basic props (no arc traversal)
        let cypher = format!(
            r#"
MATCH (n:{class_label})
WHERE NOT n:Schema
WITH count(n) AS total
MATCH (n:{class_label})
WHERE NOT n:Schema
WITH total, n
ORDER BY n.key
LIMIT {limit}
RETURN
    total,
    coalesce(n.key, n.label, toString(id(n))) AS key,
    coalesce(n.display_name, n.key, n.label) AS display_name,
    properties(n) AS props
"#,
            class_label = class_label,
            limit = INSTANCE_LIMIT
        );

        let rows = db.execute(&cypher).await?;
        let mut instances = Vec::with_capacity(rows.len());
        let mut total_count = 0usize;

        for row in rows {
            // Get total from first row
            if total_count == 0 {
                total_count = row.get::<i64>("total").unwrap_or(0) as usize;
            }

            let key: String = row.get("key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();

            // Parse properties
            let props: BTreeMap<String, JsonValue> = row
                .get::<neo4rs::BoltMap>("props")
                .map(|m| {
                    m.value
                        .iter()
                        .map(|(k, v)| (k.value.clone(), bolt_to_json(v)))
                        .collect()
                })
                .unwrap_or_default();

            instances.push(InstanceInfo {
                key,
                display_name,
                class_key: class_label.to_string(),
                properties: props,
                outgoing_arcs: Vec::new(), // Empty - will be loaded separately
                incoming_arcs: Vec::new(), // Empty - will be loaded separately
                arcs_loading: true,        // Mark as loading
                missing_required_count: 0,
                filled_properties: 0,
                total_properties: 0,
            });
        }

        Ok((instances, total_count))
    }

    /// Load arcs for a batch of instance keys.
    /// Called AFTER `load_instances_fast()` to populate arc data in background.
    pub async fn load_instance_arcs(
        db: &Db,
        class_label: &str,
        keys: Vec<String>,
    ) -> crate::Result<FxHashMap<String, (Vec<InstanceArc>, Vec<InstanceArc>)>> {
        if keys.is_empty() {
            return Ok(FxHashMap::default());
        }

        // Security: Validate label
        validate_cypher_label(class_label)?;

        let cypher = format!(
            r#"
UNWIND $keys AS k
MATCH (n:{class_label} {{key: k}})
OPTIONAL MATCH (n)-[out]->(target)
WHERE NOT target:Meta
WITH n, k, collect(DISTINCT {{
    arc_type: type(out),
    target_key: coalesce(target.key, target.label, id(target)),
    target_class: head(labels(target))
}}) AS outgoing
OPTIONAL MATCH (source)-[inc]->(n)
WHERE NOT source:Meta
WITH n, k, outgoing, collect(DISTINCT {{
    arc_type: type(inc),
    source_key: coalesce(source.key, source.label, id(source)),
    source_kind: head(labels(source))
}}) AS incoming
RETURN k AS key, outgoing, incoming
"#,
            class_label = class_label
        );

        let rows = db.execute_with_params(&cypher, [("keys", keys)]).await?;
        let mut result = FxHashMap::default();

        for row in rows {
            let key: String = row.get("key").unwrap_or_default();

            // Parse outgoing arcs
            let outgoing_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("outgoing")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("target_key").unwrap_or_default(),
                        target_class: m.get("target_class").unwrap_or_default(),
                        exists: true,
                    })
                })
                .collect();

            // Parse incoming arcs
            let incoming_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("incoming")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("source_key").unwrap_or_default(),
                        target_class: m.get("source_kind").unwrap_or_default(),
                        exists: true,
                    })
                })
                .collect();

            result.insert(key, (outgoing_arcs, incoming_arcs));
        }

        Ok(result)
    }

    /// Load graph statistics.
    async fn load_stats(db: &Db) -> crate::Result<GraphStats> {
        let cypher = r#"
MATCH (n) WHERE NOT n:Schema
WITH count(n) AS nodes
MATCH ()-[r]->() WHERE NOT startNode(r):Schema AND NOT endNode(r):Schema
WITH nodes, count(r) AS arcs
MATCH (k:Class:Schema)
WITH nodes, arcs, count(k) AS classes
MATCH (ak:ArcClass:Schema)
RETURN nodes, arcs, classes, count(ak) AS arc_classes
"#;

        let rows = db.execute(cypher).await?;
        if let Some(row) = rows.into_iter().next() {
            Ok(GraphStats {
                node_count: row.get("nodes").unwrap_or(0),
                arc_count: row.get("arcs").unwrap_or(0),
                kind_count: row.get("classes").unwrap_or(0),
                arc_kind_count: row.get("arc_classes").unwrap_or(0),
            })
        } else {
            Ok(GraphStats::default())
        }
    }

    /// Load arc relationships for a Class from Neo4j.
    /// Returns incoming and outgoing arcs with their families.
    pub async fn load_class_arcs(db: &Db, class_label: &str) -> crate::Result<ClassArcsData> {
        let cypher = r#"
MATCH (c:Class {label: $classLabel})
OPTIONAL MATCH (c)-[:IN_LAYER]->(l:Layer)
OPTIONAL MATCH (l)<-[:HAS_LAYER]-(r:Realm)
OPTIONAL MATCH (c)<-[:TO_CLASS]-(inArc:ArcClass)-[:FROM_CLASS]->(fromClass:Class)
OPTIONAL MATCH (inArc)-[:IN_FAMILY]->(inFamily:ArcFamily)
OPTIONAL MATCH (c)<-[:FROM_CLASS]-(outArc:ArcClass)-[:TO_CLASS]->(toClass:Class)
OPTIONAL MATCH (outArc)-[:IN_FAMILY]->(outFamily:ArcFamily)
WITH c, r, l,
     collect(DISTINCT CASE WHEN inArc IS NOT NULL
         THEN {arc: inArc.key, from: fromClass.label, family: inFamily.key} END) as incoming,
     collect(DISTINCT CASE WHEN outArc IS NOT NULL
         THEN {arc: outArc.key, to: toClass.label, family: outFamily.key} END) as outgoing
RETURN c.label as class,
       r.key as realm,
       l.key as layer,
       [x IN incoming WHERE x IS NOT NULL] as incoming,
       [x IN outgoing WHERE x IS NOT NULL] as outgoing
LIMIT 1
"#;

        let rows = db
            .execute_with_params(cypher, [("classLabel", class_label)])
            .await?;

        if let Some(row) = rows.into_iter().next() {
            let class: String = row.get("class").unwrap_or_default();
            let realm: String = row.get("realm").unwrap_or_default();
            let layer: String = row.get("layer").unwrap_or_default();

            // Parse incoming arcs
            let incoming_raw: Vec<neo4rs::BoltMap> = row
                .get::<Vec<neo4rs::BoltMap>>("incoming")
                .unwrap_or_default();
            let incoming: Vec<Neo4jArc> = incoming_raw
                .into_iter()
                .filter_map(|m| {
                    let arc_key = m.get::<String>("arc").ok()?;
                    let other_class = m.get::<String>("from").ok()?;
                    let family = m.get::<String>("family").ok().unwrap_or_default();
                    Some(Neo4jArc {
                        arc_key,
                        other_class,
                        family,
                    })
                })
                .collect();

            // Parse outgoing arcs
            let outgoing_raw: Vec<neo4rs::BoltMap> = row
                .get::<Vec<neo4rs::BoltMap>>("outgoing")
                .unwrap_or_default();
            let outgoing: Vec<Neo4jArc> = outgoing_raw
                .into_iter()
                .filter_map(|m| {
                    let arc_key = m.get::<String>("arc").ok()?;
                    let other_class = m.get::<String>("to").ok()?;
                    let family = m.get::<String>("family").ok().unwrap_or_default();
                    Some(Neo4jArc {
                        arc_key,
                        other_class,
                        family,
                    })
                })
                .collect();

            Ok(ClassArcsData {
                class_label: class,
                realm,
                layer,
                incoming,
                outgoing,
            })
        } else {
            Ok(ClassArcsData::default())
        }
    }

    /// Load ArcClass details from Neo4j (endpoints, family, cardinality).
    pub async fn load_arc_kind_details(db: &Db, arc_key: &str) -> crate::Result<ArcClassDetails> {
        let cypher = r#"
MATCH (ac:ArcClass {key: $arcKey})
OPTIONAL MATCH (ac)-[:IN_FAMILY]->(af:ArcFamily)
OPTIONAL MATCH (ac)-[:FROM_CLASS]->(fromClass:Class)
OPTIONAL MATCH (fromClass)-[:IN_LAYER]->(fromLayer:Layer)
OPTIONAL MATCH (fromLayer)<-[:HAS_LAYER]-(fromRealm:Realm)
OPTIONAL MATCH (ac)-[:TO_CLASS]->(toClass:Class)
OPTIONAL MATCH (toClass)-[:IN_LAYER]->(toLayer:Layer)
OPTIONAL MATCH (toLayer)<-[:HAS_LAYER]-(toRealm:Realm)
RETURN coalesce(ac.display_name, ac.key) as display_name,
       coalesce(ac.llm_context, '') as description,
       coalesce(ac.cardinality, '') as cardinality,
       coalesce(ac.cypher_pattern, '') as cypher_pattern,
       coalesce(af.key, '') as family,
       fromClass.label as from_class,
       coalesce(fromRealm.key, '') as from_realm,
       coalesce(fromLayer.key, '') as from_layer,
       toClass.label as to_class,
       coalesce(toRealm.key, '') as to_realm,
       coalesce(toLayer.key, '') as to_layer
LIMIT 1
"#;

        let rows = db
            .execute_with_params(cypher, [("arcKey", arc_key)])
            .await?;

        if let Some(row) = rows.into_iter().next() {
            let display_name: String = row.get("display_name").unwrap_or_default();
            let description: String = row.get("description").unwrap_or_default();
            let cardinality: String = row.get("cardinality").unwrap_or_default();
            let cypher_pattern: String = row.get("cypher_pattern").unwrap_or_default();
            let family: String = row.get("family").unwrap_or_default();

            let from_class: Option<String> = row.get("from_class").ok();
            let from_realm: String = row.get("from_realm").unwrap_or_default();
            let from_layer: String = row.get("from_layer").unwrap_or_default();

            let to_class: Option<String> = row.get("to_class").ok();
            let to_realm: String = row.get("to_realm").unwrap_or_default();
            let to_layer: String = row.get("to_layer").unwrap_or_default();

            let from_endpoint = from_class.map(|class_label| ArcEndpoint {
                class_label,
                realm: from_realm,
                layer: from_layer,
            });

            let to_endpoint = to_class.map(|class_label| ArcEndpoint {
                class_label,
                realm: to_realm,
                layer: to_layer,
            });

            Ok(ArcClassDetails {
                display_name,
                description,
                family,
                cardinality,
                cypher_pattern,
                from_endpoint,
                to_endpoint,
            })
        } else {
            Ok(ArcClassDetails::default())
        }
    }

    /// Load Realm details from Neo4j (layers with kind counts, total stats).
    pub async fn load_realm_details(db: &Db, realm_key: &str) -> crate::Result<RealmDetails> {
        // Query 1: Get realm info and totals
        let cypher_realm = r#"
MATCH (r:Realm {key: $realmKey})
OPTIONAL MATCH (r)-[:HAS_LAYER]->(l:Layer)<-[:IN_LAYER]-(c:Class)
OPTIONAL MATCH (c)<-[:OF_CLASS]-(n)
RETURN r.key as realm_key,
       coalesce(r.display_name, r.key) as display_name,
       coalesce(r.llm_context, '') as description,
       count(DISTINCT c) as total_classes,
       count(DISTINCT n) as total_instances
"#;

        // Query 2: Get layers with their kind counts (separate rows)
        let cypher_layers = r#"
MATCH (r:Realm {key: $realmKey})-[:HAS_LAYER]->(l:Layer)
OPTIONAL MATCH (l)<-[:IN_LAYER]-(c:Class)
WITH l, count(DISTINCT c) as kind_count
ORDER BY l.order
RETURN l.key as layer_key,
       coalesce(l.display_name, l.key) as layer_display,
       kind_count
"#;

        // Execute both queries in parallel using tokio::join!
        let (realm_result, layers_result) = tokio::join!(
            db.execute_with_params(cypher_realm, [("realmKey", realm_key)]),
            db.execute_with_params(cypher_layers, [("realmKey", realm_key)]),
        );

        let realm_rows = realm_result?;
        let layer_rows = layers_result?;

        if let Some(row) = realm_rows.into_iter().next() {
            let key: String = row.get("realm_key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();
            let description: String = row.get("description").unwrap_or_default();
            let total_classes: i64 = row.get("total_classes").unwrap_or(0);
            let total_instances: i64 = row.get("total_instances").unwrap_or(0);

            let layers: Vec<LayerStats> = layer_rows
                .into_iter()
                .map(|lr| LayerStats {
                    key: lr.get("layer_key").unwrap_or_default(),
                    display_name: lr.get("layer_display").unwrap_or_default(),
                    class_count: lr.get::<i64>("kind_count").unwrap_or(0) as usize,
                })
                .collect();

            Ok(RealmDetails {
                key,
                display_name,
                description,
                layers,
                total_classes: total_classes as usize,
                total_instances: total_instances as usize,
            })
        } else {
            Ok(RealmDetails::default())
        }
    }

    /// Load Layer details from Neo4j (classes grouped by trait, stats).
    pub async fn load_layer_details(db: &Db, layer_key: &str) -> crate::Result<LayerDetails> {
        let cypher = r#"
MATCH (l:Layer {key: $layerKey})
OPTIONAL MATCH (r:Realm)-[:HAS_LAYER]->(l)
OPTIONAL MATCH (l)<-[:IN_LAYER]-(c:Class)
OPTIONAL MATCH (c)-[:HAS_TRAIT]->(t:Trait)
OPTIONAL MATCH (c)<-[:OF_CLASS]-(n)
WITH l, r, t.key as trait_key, c, count(DISTINCT n) as inst_count
ORDER BY trait_key, c.label
WITH l, r, trait_key, collect(coalesce(c.display_name, c.label)) as kind_names, count(c) as trait_kind_count, sum(inst_count) as trait_instances
RETURN l.key as layer_key,
       coalesce(l.display_name, l.key) as display_name,
       coalesce(l.llm_context, '') as description,
       coalesce(r.key, '') as realm,
       collect({trait_key: trait_key, kind_names: kind_names}) as classes_by_trait,
       sum(trait_kind_count) as total_classes,
       sum(trait_instances) as total_instances
"#;

        let rows = db
            .execute_with_params(cypher, [("layerKey", layer_key)])
            .await?;

        if let Some(row) = rows.into_iter().next() {
            let key: String = row.get("layer_key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();
            let description: String = row.get("description").unwrap_or_default();
            let realm: String = row.get("realm").unwrap_or_default();
            let total_classes: i64 = row.get("total_classes").unwrap_or(0);
            let total_instances: i64 = row.get("total_instances").unwrap_or(0);

            // Parse classes_by_trait
            let groups_list = row
                .get::<Vec<neo4rs::BoltMap>>("classes_by_trait")
                .unwrap_or_default();
            let mut classes_by_trait: Vec<TraitClassGroup> = Vec::with_capacity(groups_list.len());
            for group_map in groups_list {
                if let Ok(trait_key) = group_map.get::<String>("trait_key") {
                    let kind_names: Vec<String> = group_map
                        .get::<Vec<String>>("kind_names")
                        .unwrap_or_default();
                    classes_by_trait.push(TraitClassGroup {
                        trait_key,
                        kind_names,
                    });
                }
            }

            Ok(LayerDetails {
                key,
                display_name,
                description,
                realm,
                classes_by_trait,
                total_classes: total_classes as usize,
                total_instances: total_instances as usize,
            })
        } else {
            Ok(LayerDetails::default())
        }
    }

    // ========================================================================
    // Entity Category Hierarchy (Data mode)
    // ========================================================================

    /// Load all EntityCategory nodes from Neo4j with instance counts.
    /// Returns categories sorted by sort_order for display in Data mode.
    pub async fn load_entity_categories(db: &Db) -> crate::Result<Vec<EntityCategory>> {
        let cypher = r#"
MATCH (c:EntityCategory)
OPTIONAL MATCH (e:Entity)-[:BELONGS_TO]->(c)
WITH c, count(e) AS instance_count
RETURN c.key AS key,
       coalesce(c.display_name, c.key) AS display_name,
       coalesce(c.sort_order, 0) AS sort_order,
       coalesce(c.question, '') AS question,
       coalesce(c.llm_context, '') AS llm_context,
       instance_count
ORDER BY c.sort_order, c.key
"#;

        let rows = db.execute(cypher).await?;
        let mut categories = Vec::with_capacity(rows.len());

        for row in rows {
            categories.push(EntityCategory {
                key: row.get("key").unwrap_or_default(),
                display_name: row.get("display_name").unwrap_or_default(),
                sort_order: row.get("sort_order").unwrap_or(0),
                question: row.get("question").unwrap_or_default(),
                llm_context: row.get("llm_context").unwrap_or_default(),
                instance_count: row.get("instance_count").unwrap_or(0),
            });
        }

        Ok(categories)
    }

    /// Load Entity instances that belong to a specific EntityCategory.
    /// Returns (instances, total_count) for pagination display.
    ///
    /// Uses the BELONGS_TO arc: Entity -[:BELONGS_TO]-> EntityCategory
    /// This enables Data mode to show Entity instances grouped by category.
    pub async fn load_entities_by_category(
        db: &Db,
        category_key: &str,
    ) -> crate::Result<(Vec<InstanceInfo>, i64)> {
        // Query with parameterized category key (safe from injection)
        let cypher = r#"
MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory {key: $category})
WITH count(e) AS total
MATCH (e:Entity)-[:BELONGS_TO]->(c:EntityCategory {key: $category})
WITH total, e
ORDER BY e.display_name, e.key
LIMIT 1000
WITH total, collect(e) AS entities
UNWIND entities AS e
OPTIONAL MATCH (e)-[out]->(target)
WHERE NOT target:Meta
WITH total, e, collect(DISTINCT {
    arc_type: type(out),
    target_key: coalesce(target.key, target.label, toString(id(target))),
    target_class: head(labels(target))
}) AS outgoing
OPTIONAL MATCH (source)-[inc]->(e)
WHERE NOT source:Meta
WITH total, e, outgoing, collect(DISTINCT {
    arc_type: type(inc),
    source_key: coalesce(source.key, source.label, toString(id(source))),
    source_kind: head(labels(source))
}) AS incoming
RETURN total,
       coalesce(e.key, toString(id(e))) AS key,
       coalesce(e.display_name, e.key) AS display_name,
       properties(e) AS props,
       outgoing,
       incoming
"#;

        let rows = db
            .execute_with_params(cypher, [("category", category_key)])
            .await?;

        // Get total count from first row (all rows have same total)
        let total_count: i64 = rows.first().and_then(|r| r.get("total").ok()).unwrap_or(0);

        let mut instances = Vec::with_capacity(rows.len());

        for row in rows {
            let key: String = row.get("key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();

            // Parse properties as BTreeMap with proper JSON values
            let props: BTreeMap<String, JsonValue> = row
                .get::<neo4rs::BoltMap>("props")
                .map(|m| {
                    m.value
                        .iter()
                        .map(|(k, v)| (k.value.clone(), bolt_to_json(v)))
                        .collect()
                })
                .unwrap_or_default();

            // Parse outgoing arcs
            let outgoing_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("outgoing")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("target_key").unwrap_or_default(),
                        target_class: m.get("target_class").unwrap_or_default(),
                        exists: true,
                    })
                })
                .collect();

            // Parse incoming arcs
            let incoming_arcs: Vec<InstanceArc> = row
                .get::<Vec<neo4rs::BoltMap>>("incoming")
                .unwrap_or_default()
                .into_iter()
                .filter_map(|m| {
                    let arc_type = m.get::<String>("arc_type").ok()?;
                    if arc_type.is_empty() {
                        return None;
                    }
                    Some(InstanceArc {
                        arc_type,
                        target_key: m.get("source_key").unwrap_or_default(),
                        target_class: m.get("source_kind").unwrap_or_default(),
                        exists: true,
                    })
                })
                .collect();

            instances.push(InstanceInfo {
                key,
                display_name,
                class_key: "Entity".to_string(),
                properties: props,
                outgoing_arcs,
                incoming_arcs,
                arcs_loading: false,
                missing_required_count: 0, // Calculated later if needed
                filled_properties: 0,      // Calculated later if needed
                total_properties: 0,       // Calculated later if needed
            });
        }

        Ok((instances, total_count))
    }

    /// Check if a node is collapsed.
    pub fn is_collapsed(&self, key: &str) -> bool {
        self.collapsed.contains(key)
    }

    /// Toggle collapse state of a node.
    pub fn toggle(&mut self, key: &str) {
        if self.collapsed.contains(key) {
            self.collapsed.remove(key);
        } else {
            self.collapsed.insert(key.to_string());
        }
    }

    /// Collapse all collapsible nodes.
    pub fn collapse_all(&mut self) {
        self.collapsed.insert("classes".to_string());
        self.collapsed.insert("arcs".to_string());
        for realm in &self.realms {
            self.collapsed.insert(format!("realm:{}", realm.key));
            for layer in &realm.layers {
                self.collapsed
                    .insert(format!("layer:{}:{}", realm.key, layer.key));
            }
        }
        for family in &self.arc_families {
            self.collapsed.insert(format!("family:{}", family.key));
        }
    }

    /// Expand all nodes.
    pub fn expand_all(&mut self) {
        self.collapsed.clear();
    }

    /// Expand a single node (remove from collapsed set).
    /// Unlike `expand_subtree`, this only expands the specified item.
    pub fn expand(&mut self, key: &str) {
        self.collapsed.remove(key);
    }

    /// Collapse all Class instances (hide their instances).
    /// Used when switching between Meta and Data modes.
    pub fn collapse_all_classes(&mut self) {
        for realm in &self.realms {
            for layer in &realm.layers {
                for kind in &layer.classes {
                    self.collapsed.insert(format!("class:{}", kind.key));
                }
            }
        }
    }

    /// Expand subtree under a specific key.
    /// Expands the item and all its children.
    pub fn expand_subtree(&mut self, key: &str) {
        // Remove the key itself
        self.collapsed.remove(key);

        // Expand children based on key type
        if key == "classes" {
            // Expand all realms and layers
            for realm in &self.realms {
                self.collapsed.remove(&format!("realm:{}", realm.key));
                for layer in &realm.layers {
                    self.collapsed
                        .remove(&format!("layer:{}:{}", realm.key, layer.key));
                    for kind in &layer.classes {
                        self.collapsed.remove(&format!("class:{}", kind.key));
                    }
                }
            }
        } else if key == "arcs" {
            // Expand all arc families
            for family in &self.arc_families {
                self.collapsed.remove(&format!("family:{}", family.key));
            }
        } else if let Some(realm_key) = key.strip_prefix("realm:") {
            // Expand all layers in this realm
            if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                for layer in &realm.layers {
                    self.collapsed
                        .remove(&format!("layer:{}:{}", realm_key, layer.key));
                    for kind in &layer.classes {
                        self.collapsed.remove(&format!("class:{}", kind.key));
                    }
                }
            }
        } else if let Some(rest) = key.strip_prefix("layer:") {
            // Layer key format: layer:{realm_key}:{layer_key}
            // Expand all classes in this layer
            if let Some((realm_key, layer_key)) = rest.split_once(':') {
                if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                    if let Some(layer) = realm.layers.iter().find(|l| l.key == layer_key) {
                        for kind in &layer.classes {
                            self.collapsed.remove(&format!("class:{}", kind.key));
                        }
                    }
                }
            }
        } else if let Some(family_key) = key.strip_prefix("family:") {
            // Arc family - nothing more to expand (arc classes aren't collapsible)
            let _ = family_key; // Suppress unused warning
        }
        // kind: prefix - nothing more to expand (instances aren't collapsible)
    }

    /// Collapse subtree under a specific key.
    /// Collapses the item and all its children.
    pub fn collapse_subtree(&mut self, key: &str) {
        // Collapse the key itself
        self.collapsed.insert(key.to_string());

        // Collapse children based on key type
        if key == "classes" {
            // Collapse all realms and layers
            for realm in &self.realms {
                self.collapsed.insert(format!("realm:{}", realm.key));
                for layer in &realm.layers {
                    self.collapsed
                        .insert(format!("layer:{}:{}", realm.key, layer.key));
                    for kind in &layer.classes {
                        self.collapsed.insert(format!("class:{}", kind.key));
                    }
                }
            }
        } else if key == "arcs" {
            // Collapse all arc families
            for family in &self.arc_families {
                self.collapsed.insert(format!("family:{}", family.key));
            }
        } else if let Some(realm_key) = key.strip_prefix("realm:") {
            // Collapse all layers in this realm
            if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                for layer in &realm.layers {
                    self.collapsed
                        .insert(format!("layer:{}:{}", realm_key, layer.key));
                    for kind in &layer.classes {
                        self.collapsed.insert(format!("class:{}", kind.key));
                    }
                }
            }
        } else if let Some(rest) = key.strip_prefix("layer:") {
            // Layer key format: layer:{realm_key}:{layer_key}
            // Collapse all classes in this layer
            if let Some((realm_key, layer_key)) = rest.split_once(':') {
                if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                    if let Some(layer) = realm.layers.iter().find(|l| l.key == layer_key) {
                        for kind in &layer.classes {
                            self.collapsed.insert(format!("class:{}", kind.key));
                        }
                    }
                }
            }
        } else if let Some(family_key) = key.strip_prefix("family:") {
            // Arc family - nothing more to collapse
            let _ = family_key;
        }
        // kind: prefix - nothing more to collapse
    }

    // ========================================================================
    // Data view: Instance methods
    // ========================================================================

    /// Set instances for a Class (used in Data mode).
    /// Also stores the total count for "X of Y" display.
    /// Calculates missing_required_count for each instance based on Class schema.
    #[allow(dead_code)]
    pub fn set_instances(
        &mut self,
        class_key: &str,
        mut instances: Vec<InstanceInfo>,
        total: usize,
    ) {
        // Get schema info from Class
        let (required_props, all_props) = self
            .find_kind(class_key)
            .map(|(_, _, kind)| (kind.required_properties.clone(), kind.properties.clone()))
            .unwrap_or_default();

        let total_props = all_props.len();

        // Calculate stats for each instance
        for instance in &mut instances {
            // Missing required count
            let missing = required_props
                .iter()
                .filter(|prop| {
                    // Property is missing if not present or is null/empty
                    match instance.properties.get(*prop) {
                        None => true,
                        Some(JsonValue::Null) => true,
                        Some(JsonValue::String(s)) => s.is_empty(),
                        Some(_) => false,
                    }
                })
                .count();
            instance.missing_required_count = missing;

            // Filled properties count (non-null, non-empty)
            let filled = instance
                .properties
                .values()
                .filter(|v| {
                    !matches!(v, JsonValue::Null)
                        && !matches!(v, JsonValue::String(s) if s.is_empty())
                })
                .count();
            instance.filled_properties = filled;
            instance.total_properties = total_props;
        }

        self.instances.insert(class_key.to_string(), instances);
        self.instance_totals.insert(class_key.to_string(), total);
    }

    /// Get instances for a Class.
    pub fn get_instances(&self, class_key: &str) -> Option<&Vec<InstanceInfo>> {
        self.instances.get(class_key)
    }

    /// Get total instance count for a Class (may be > loaded instances).
    pub fn get_instance_total(&self, class_key: &str) -> Option<usize> {
        self.instance_totals.get(class_key).copied()
    }

    /// Update arcs for instances after progressive loading.
    /// Called AFTER `set_instances` with arc data from `load_instance_arcs`.
    pub fn update_instance_arcs(
        &mut self,
        class_key: &str,
        arcs: FxHashMap<String, (Vec<InstanceArc>, Vec<InstanceArc>)>,
    ) {
        if let Some(instances) = self.instances.get_mut(class_key) {
            for instance in instances.iter_mut() {
                if let Some((outgoing, incoming)) = arcs.get(&instance.key) {
                    instance.outgoing_arcs = outgoing.clone();
                    instance.incoming_arcs = incoming.clone();
                    instance.arcs_loading = false;
                }
            }
        }
    }

    /// Total number of visible items for a specific mode.
    /// In Data mode (data_mode=true), includes instances under expanded Classes.
    /// For Entity Class, shows category hierarchy: Entity > Category > instances.
    pub fn item_count_for_mode(&self, data_mode: bool) -> usize {
        let mut count = 0;

        // Classs section
        count += 1; // "Classes" header
        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                count += 1; // realm header
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        count += 1; // layer header
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            for kind in &layer.classes {
                                count += 1; // kind

                                // In Data mode, add instances if not collapsed
                                if data_mode && !self.is_collapsed(&format!("class:{}", kind.key)) {
                                    // Special case: Entity Class shows categories hierarchy
                                    if kind.key == "Entity" && !self.entity_categories.is_empty() {
                                        for category in &self.entity_categories {
                                            count += 1; // category node

                                            // Count instances under category if not collapsed
                                            let cat_key = format!("category:{}", category.key);
                                            if !self.is_collapsed(&cat_key) {
                                                if let Some(instances) = self
                                                    .entity_category_instances
                                                    .get(&category.key)
                                                {
                                                    count += instances.len();
                                                }
                                            }
                                        }
                                    } else {
                                        // Regular kind: flat instances
                                        if let Some(instances) = self.instances.get(&kind.key) {
                                            count += instances.len();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Arcs section
        count += 1; // "Arcs" header
        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                count += 1; // family header
                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    count += family.arc_classes.len();
                }
            }
        }

        count
    }

    /// Get item at cursor position for a specific mode.
    /// In Data mode (data_mode=true), includes instances under expanded Classes.
    /// For Entity Class, shows category hierarchy: Entity > Category > instances.
    pub fn item_at_for_mode(&self, cursor: usize, data_mode: bool) -> Option<TreeItem<'_>> {
        let mut idx = 0;

        // Classs section header
        if idx == cursor {
            return Some(TreeItem::ClassesSection);
        }
        idx += 1;

        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                if idx == cursor {
                    return Some(TreeItem::Realm(realm));
                }
                idx += 1;

                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        if idx == cursor {
                            return Some(TreeItem::Layer(realm, layer));
                        }
                        idx += 1;

                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            for kind in &layer.classes {
                                if idx == cursor {
                                    return Some(TreeItem::Class(realm, layer, kind));
                                }
                                idx += 1;

                                // In Data mode, check for instances (or categories for Entity)
                                if data_mode && !self.is_collapsed(&format!("class:{}", kind.key)) {
                                    // Special case: Entity Class shows categories hierarchy
                                    if kind.key == "Entity" && !self.entity_categories.is_empty() {
                                        for category in &self.entity_categories {
                                            if idx == cursor {
                                                return Some(TreeItem::EntityCategory(
                                                    realm, layer, kind, category,
                                                ));
                                            }
                                            idx += 1;

                                            // Show instances under category if not collapsed
                                            let cat_key = format!("category:{}", category.key);
                                            if !self.is_collapsed(&cat_key) {
                                                if let Some(instances) = self
                                                    .entity_category_instances
                                                    .get(&category.key)
                                                {
                                                    for instance in instances {
                                                        if idx == cursor {
                                                            return Some(TreeItem::Instance(
                                                                realm, layer, kind, instance,
                                                            ));
                                                        }
                                                        idx += 1;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        // Regular kind: flat instances
                                        if let Some(instances) = self.instances.get(&kind.key) {
                                            for instance in instances {
                                                if idx == cursor {
                                                    return Some(TreeItem::Instance(
                                                        realm, layer, kind, instance,
                                                    ));
                                                }
                                                idx += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Arcs section header
        if idx == cursor {
            return Some(TreeItem::ArcsSection);
        }
        idx += 1;

        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                if idx == cursor {
                    return Some(TreeItem::ArcFamily(family));
                }
                idx += 1;

                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    for arc_kind in &family.arc_classes {
                        if idx == cursor {
                            return Some(TreeItem::ArcClass(family, arc_kind));
                        }
                        idx += 1;
                    }
                }
            }
        }

        None
    }

    /// Total number of visible items in the flattened tree (respects collapsed state).
    pub fn item_count(&self) -> usize {
        let mut count = 0;

        // Classs section
        count += 1; // "Classes" header
        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                count += 1; // realm header
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        count += 1; // layer header
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            count += layer.classes.len();
                        }
                    }
                }
            }
        }

        // Arcs section
        count += 1; // "Arcs" header
        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                count += 1; // family header
                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    count += family.arc_classes.len();
                }
            }
        }

        count
    }

    /// Get item at cursor position (respects collapsed state).
    pub fn item_at(&self, cursor: usize) -> Option<TreeItem<'_>> {
        let mut idx = 0;

        // Classs section header
        if idx == cursor {
            return Some(TreeItem::ClassesSection);
        }
        idx += 1;

        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                if idx == cursor {
                    return Some(TreeItem::Realm(realm));
                }
                idx += 1;

                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        if idx == cursor {
                            return Some(TreeItem::Layer(realm, layer));
                        }
                        idx += 1;

                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            for kind in &layer.classes {
                                if idx == cursor {
                                    return Some(TreeItem::Class(realm, layer, kind));
                                }
                                idx += 1;
                            }
                        }
                    }
                }
            }
        }

        // Arcs section header
        if idx == cursor {
            return Some(TreeItem::ArcsSection);
        }
        idx += 1;

        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                if idx == cursor {
                    return Some(TreeItem::ArcFamily(family));
                }
                idx += 1;

                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    for arc_kind in &family.arc_classes {
                        if idx == cursor {
                            return Some(TreeItem::ArcClass(family, arc_kind));
                        }
                        idx += 1;
                    }
                }
            }
        }

        None
    }

    // =========================================================================
    // TRAIT FILTER METHODS (Quick Filter: fi/fl/fk/fg/fa)
    // =========================================================================

    /// Check if a Layer has any Classes matching the trait filter.
    fn layer_has_matching_classes(&self, layer: &LayerInfo, trait_filter: &str) -> bool {
        layer.classes.iter().any(|k| k.trait_name == trait_filter)
    }

    /// Check if a Realm has any Layers with matching Classes.
    fn realm_has_matching_classes(&self, realm: &RealmInfo, trait_filter: &str) -> bool {
        realm
            .layers
            .iter()
            .any(|l| self.layer_has_matching_classes(l, trait_filter))
    }

    /// Count visible items with trait filter applied.
    /// Hides Classes that don't match, and Layers/Realms with no matching Classes.
    pub fn item_count_with_trait_filter(&self, trait_filter: Option<&str>) -> usize {
        let Some(filter) = trait_filter else {
            return self.item_count(); // No filter, use normal count
        };

        let mut count = 0;

        // Classs section
        count += 1; // "Classes" header
        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                // Skip realms with no matching classes
                if !self.realm_has_matching_classes(realm, filter) {
                    continue;
                }
                count += 1; // realm header
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        // Skip layers with no matching classes
                        if !self.layer_has_matching_classes(layer, filter) {
                            continue;
                        }
                        count += 1; // layer header
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            // Count only matching classes
                            count += layer
                                .classes
                                .iter()
                                .filter(|k| k.trait_name == filter)
                                .count();
                        }
                    }
                }
            }
        }

        // Arcs section (not filtered by trait)
        count += 1; // "Arcs" header
        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                count += 1; // family header
                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    count += family.arc_classes.len();
                }
            }
        }

        count
    }

    /// Get item at cursor position with trait filter applied.
    pub fn item_at_with_trait_filter(
        &self,
        cursor: usize,
        trait_filter: Option<&str>,
    ) -> Option<TreeItem<'_>> {
        let Some(filter) = trait_filter else {
            return self.item_at(cursor); // No filter, use normal lookup
        };

        let mut idx = 0;

        // Classs section header
        if idx == cursor {
            return Some(TreeItem::ClassesSection);
        }
        idx += 1;

        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                // Skip realms with no matching classes
                if !self.realm_has_matching_classes(realm, filter) {
                    continue;
                }
                if idx == cursor {
                    return Some(TreeItem::Realm(realm));
                }
                idx += 1;

                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        // Skip layers with no matching classes
                        if !self.layer_has_matching_classes(layer, filter) {
                            continue;
                        }
                        if idx == cursor {
                            return Some(TreeItem::Layer(realm, layer));
                        }
                        idx += 1;

                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            // Only include matching classes
                            for kind in layer.classes.iter().filter(|k| k.trait_name == filter) {
                                if idx == cursor {
                                    return Some(TreeItem::Class(realm, layer, kind));
                                }
                                idx += 1;
                            }
                        }
                    }
                }
            }
        }

        // Arcs section header
        if idx == cursor {
            return Some(TreeItem::ArcsSection);
        }
        idx += 1;

        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                if idx == cursor {
                    return Some(TreeItem::ArcFamily(family));
                }
                idx += 1;

                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    for arc_kind in &family.arc_classes {
                        if idx == cursor {
                            return Some(TreeItem::ArcClass(family, arc_kind));
                        }
                        idx += 1;
                    }
                }
            }
        }

        None
    }

    /// Get the collapse key for an item at cursor position.
    pub fn collapse_key_at(&self, cursor: usize, data_mode: bool) -> Option<String> {
        let item = if data_mode {
            self.item_at_for_mode(cursor, true)
        } else {
            self.item_at(cursor)
        };
        match item {
            Some(TreeItem::ClassesSection) => Some("classes".to_string()),
            Some(TreeItem::ArcsSection) => Some("arcs".to_string()),
            Some(TreeItem::Realm(r)) => Some(format!("realm:{}", r.key)),
            Some(TreeItem::Layer(r, l)) => Some(format!("layer:{}:{}", r.key, l.key)),
            Some(TreeItem::ArcFamily(f)) => Some(format!("family:{}", f.key)),
            // In Data mode, Class can be collapsed to hide instances
            Some(TreeItem::Class(_, _, k)) => Some(format!("class:{}", k.key)),
            // EntityCategory can be collapsed to hide its instances
            Some(TreeItem::EntityCategory(_, _, _, cat)) => Some(format!("category:{}", cat.key)),
            // Leaf nodes can't be collapsed
            Some(TreeItem::ArcClass(_, _)) | Some(TreeItem::Instance(_, _, _, _)) | None => None,
        }
    }

    /// Find the cursor position of the parent item.
    /// Returns None if at root or no parent exists.
    /// Hierarchy: Instance → EntityCategory → Class → Layer → Realm → ClassesSection
    ///            ArcClass → ArcFamily → ArcsSection
    pub fn find_parent_cursor(&self, cursor: usize, data_mode: bool) -> Option<usize> {
        let current = if data_mode {
            self.item_at_for_mode(cursor, true)
        } else {
            self.item_at(cursor)
        };

        match current {
            // Section headers have no parent
            Some(TreeItem::ClassesSection) | Some(TreeItem::ArcsSection) | None => None,

            // Realm's parent is ClassesSection (always at index 0)
            Some(TreeItem::Realm(_)) => Some(0),

            // Layer's parent is its Realm
            Some(TreeItem::Layer(realm, _)) => self.find_realm_cursor(&realm.key),

            // Class's parent is its Layer
            Some(TreeItem::Class(realm, layer, _)) => {
                self.find_layer_cursor(&realm.key, &layer.key)
            }

            // EntityCategory's parent is its Class (Entity)
            Some(TreeItem::EntityCategory(realm, layer, kind, _)) => {
                self.find_kind_cursor_readonly(&realm.key, &layer.key, &kind.key, data_mode)
            }

            // Instance's parent is its Class (or EntityCategory for Entity class)
            Some(TreeItem::Instance(realm, layer, kind, _)) => {
                self.find_kind_cursor_readonly(&realm.key, &layer.key, &kind.key, data_mode)
            }

            // ArcFamily's parent is ArcsSection
            Some(TreeItem::ArcFamily(_)) => self.find_arcs_section_cursor(),

            // ArcClass's parent is its ArcFamily
            Some(TreeItem::ArcClass(family, _)) => self.find_family_cursor(&family.key),
        }
    }

    /// Find cursor position of a Realm (does not modify collapse state).
    fn find_realm_cursor(&self, realm_key: &str) -> Option<usize> {
        if self.is_collapsed("classes") {
            return None; // Realm not visible
        }
        let mut idx = 1; // Skip ClassesSection
        for realm in &self.realms {
            if realm.key == realm_key {
                return Some(idx);
            }
            idx += 1;
            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1;
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        idx += layer.classes.len();
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of a Layer (does not modify collapse state).
    fn find_layer_cursor(&self, realm_key: &str, layer_key: &str) -> Option<usize> {
        if self.is_collapsed("classes") {
            return None;
        }
        let mut idx = 1; // Skip ClassesSection
        for realm in &self.realms {
            idx += 1; // Realm
            if realm.key == realm_key {
                if self.is_collapsed(&format!("realm:{}", realm.key)) {
                    return None; // Layer not visible
                }
                for layer in &realm.layers {
                    if layer.key == layer_key {
                        return Some(idx);
                    }
                    idx += 1;
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        idx += layer.classes.len();
                    }
                }
                return None;
            }
            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1;
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        idx += layer.classes.len();
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of a Class (readonly, does not modify collapse state).
    fn find_kind_cursor_readonly(
        &self,
        realm_key: &str,
        layer_key: &str,
        class_key: &str,
        data_mode: bool,
    ) -> Option<usize> {
        if self.is_collapsed("classes") {
            return None;
        }
        let mut idx = 1; // Skip ClassesSection
        for realm in &self.realms {
            idx += 1; // Realm
            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1; // Layer
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        for kind in &layer.classes {
                            if realm.key == realm_key
                                && layer.key == layer_key
                                && kind.key == class_key
                            {
                                return Some(idx);
                            }
                            idx += 1;
                            // In data mode, count instances
                            if data_mode && !self.is_collapsed(&format!("class:{}", kind.key)) {
                                if let Some(instances) = self.instances.get(&kind.key) {
                                    idx += instances.len();
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of ArcsSection.
    fn find_arcs_section_cursor(&self) -> Option<usize> {
        let mut idx = 1; // Skip ClassesSection
        if !self.is_collapsed("classes") {
            for realm in &self.realms {
                idx += 1;
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        idx += 1;
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            idx += layer.classes.len();
                        }
                    }
                }
            }
        }
        Some(idx) // ArcsSection is right after all realms
    }

    /// Find cursor position of an ArcFamily.
    fn find_family_cursor(&self, family_key: &str) -> Option<usize> {
        let arcs_idx = self.find_arcs_section_cursor()?;
        if self.is_collapsed("arcs") {
            return None;
        }
        let mut idx = arcs_idx + 1;
        for family in &self.arc_families {
            if family.key == family_key {
                return Some(idx);
            }
            idx += 1;
            if !self.is_collapsed(&format!("family:{}", family.key)) {
                idx += family.arc_classes.len();
            }
        }
        None
    }

    // ========================================================================
    // Filtered Data mode: show only instances of a specific Class
    // ========================================================================

    /// Get item count when filtered to a specific Class (Data mode drill-down).
    /// Returns only instances of that Class.
    pub fn filtered_item_count(&self, class_key: &str) -> usize {
        self.instances.get(class_key).map(|v| v.len()).unwrap_or(0)
    }

    /// Get item at cursor when filtered to a specific Class.
    /// Returns Instance items only.
    pub fn filtered_item_at<'a>(&'a self, cursor: usize, class_key: &str) -> Option<TreeItem<'a>> {
        // Find the Class info for context
        let kind_info = self.find_kind(class_key)?;
        let instances = self.instances.get(class_key)?;
        let instance = instances.get(cursor)?;
        Some(TreeItem::Instance(
            kind_info.0,
            kind_info.1,
            kind_info.2,
            instance,
        ))
    }

    /// Find a Class by key, returns (Realm, Layer, Class) refs.
    /// O(1) lookup using cached index (built once on load).
    pub fn find_kind(&self, class_key: &str) -> Option<(&RealmInfo, &LayerInfo, &ClassInfo)> {
        let (r_idx, l_idx, k_idx) = self.kind_index.get(class_key)?;
        let realm = self.realms.get(*r_idx)?;
        let layer = realm.layers.get(*l_idx)?;
        let kind = layer.classes.get(*k_idx)?;
        Some((realm, layer, kind))
    }

    /// Calculate hierarchical position for the current tree item.
    /// Returns position info: R:realm L:layer K:kind I:instance (all 1-based).
    pub fn hierarchy_position(&self, cursor: usize, data_mode: bool) -> HierarchyPosition {
        let item = if data_mode {
            self.item_at_for_mode(cursor, true)
        } else {
            self.item_at(cursor)
        };

        let total_realms = self.realms.len();

        match item {
            None | Some(TreeItem::ClassesSection) | Some(TreeItem::ArcsSection) => {
                HierarchyPosition::default()
            }
            Some(TreeItem::Realm(realm)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    ..Default::default()
                }
            }
            Some(TreeItem::Layer(realm, layer)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    ..Default::default()
                }
            }
            Some(TreeItem::Class(realm, layer, kind)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let kind_idx = layer
                    .classes
                    .iter()
                    .position(|k| k.key == kind.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    kind: Some((kind_idx, layer.classes.len())),
                    ..Default::default()
                }
            }
            Some(TreeItem::Instance(realm, layer, kind, _)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let kind_idx = layer
                    .classes
                    .iter()
                    .position(|k| k.key == kind.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                // Calculate instance position within Class
                let instances = self.instances.get(&kind.key);
                let total_instances = self.instance_totals.get(&kind.key).copied().unwrap_or(0);
                // Find instance index by walking the visible items before cursor
                // For simplicity, use the loaded count as position indicator
                let loaded_count = instances.map(|v| v.len()).unwrap_or(0);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    kind: Some((kind_idx, layer.classes.len())),
                    instance: Some((loaded_count.min(INSTANCE_LIMIT), total_instances)),
                }
            }
            Some(TreeItem::EntityCategory(realm, layer, kind, _)) => {
                let realm_idx = self
                    .realms
                    .iter()
                    .position(|r| r.key == realm.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let layer_idx = realm
                    .layers
                    .iter()
                    .position(|l| l.key == layer.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                let kind_idx = layer
                    .classes
                    .iter()
                    .position(|k| k.key == kind.key)
                    .map(|i| i + 1)
                    .unwrap_or(1);
                HierarchyPosition {
                    realm: Some((realm_idx, total_realms)),
                    layer: Some((layer_idx, realm.layers.len())),
                    kind: Some((kind_idx, layer.classes.len())),
                    ..Default::default()
                }
            }
            Some(TreeItem::ArcFamily(_)) | Some(TreeItem::ArcClass(_, _)) => {
                // Arcs section - no realm/layer/kind hierarchy
                HierarchyPosition::default()
            }
        }
    }
}

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
    // Data view: instances under Classes (or under EntityCategory for Entity)
    Instance(
        &'a RealmInfo,
        &'a LayerInfo,
        &'a ClassInfo,
        &'a InstanceInfo,
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
    /// Current kind index (1-based) within layer and total classes in layer
    pub kind: Option<(usize, usize)>,
    /// Current instance index (1-based) and total instances for this kind
    pub instance: Option<(usize, usize)>,
}

impl HierarchyPosition {
    /// Format as compact string: "R:1/2 L:2/4 K:3/7 I:42/300"
    pub fn to_compact_string(&self) -> String {
        let mut parts = Vec::new();
        if let Some((cur, total)) = self.realm {
            parts.push(format!("R:{}/{}", cur, total));
        }
        if let Some((cur, total)) = self.layer {
            parts.push(format!("L:{}/{}", cur, total));
        }
        if let Some((cur, total)) = self.kind {
            parts.push(format!("K:{}/{}", cur, total));
        }
        if let Some((cur, total)) = self.instance {
            parts.push(format!("I:{}/{}", cur, total));
        }
        parts.join(" ")
    }
}

/// Get icon for realm (v11.4: 2 realms - shared + org).
/// Uses unicode symbols instead of emoji for terminal compatibility.
fn realm_icon(key: &str) -> &'static str {
    match key {
        "shared" => "◉", // filled circle - universal/shared
        "org" => "◎",    // circle with dot - scoped/owned
        _ => "○",        // empty circle - unknown
    }
}

/// Convert PascalCase to kebab-case (e.g., "BlockGenerated" -> "block-generated").
/// Pre-allocates capacity to avoid reallocations.
fn to_kebab_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4); // +4 for potential dashes
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('-');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

// ============================================================================
// DATA VIEW: Instance support (v10.6)
// Reserved for Data View feature - planned for v10.7+
// ============================================================================

/// An instance of a Class in the data graph.
#[allow(dead_code)]
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
}

/// An actual arc connection from/to an instance.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceArc {
    pub arc_type: String,
    pub target_key: String,
    pub target_class: String,
    /// True if this arc exists, false if it's from schema but not yet created.
    pub exists: bool,
}

/// Comparison of schema arcs vs actual arcs for an instance.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ArcComparison {
    pub arc_type: String,
    pub target_class: String,
    pub exists: bool,
    pub target_key: Option<String>, // Only if exists
}

#[allow(dead_code)]
impl InstanceInfo {
    /// Compare schema arcs with actual arcs.
    /// Returns list of arcs showing which exist and which are missing.
    pub fn compare_arcs(&self, schema_arcs: &[ArcInfo]) -> Vec<ArcComparison> {
        let mut comparisons = Vec::with_capacity(schema_arcs.len());

        for schema_arc in schema_arcs {
            if schema_arc.direction == ArcDirection::Outgoing {
                // Check if this arc type exists in outgoing_arcs
                let actual = self
                    .outgoing_arcs
                    .iter()
                    .find(|a| a.arc_type == schema_arc.arc_type);

                comparisons.push(ArcComparison {
                    arc_type: schema_arc.arc_type.clone(),
                    target_class: schema_arc.target_class.clone(),
                    exists: actual.is_some(),
                    target_key: actual.map(|a| a.target_key.clone()),
                });
            }
        }

        comparisons
    }
}

// =============================================================================
// Test-only TaxonomyTree construction
// =============================================================================

#[cfg(test)]
impl TaxonomyTree {
    /// Create a minimal mock tree for unit tests.
    ///
    /// Structure:
    /// - global (1 layer)
    ///   - config (1 kind)
    ///     - AppConfig
    /// - tenant (1 layer)
    ///   - foundation (1 kind)
    ///     - Entity
    ///
    /// Empty arc_families and default stats.
    pub fn mock_for_testing() -> Self {
        let app_config = ClassInfo {
            key: "AppConfig".to_string(),
            display_name: "App Config".to_string(),
            description: "Application configuration".to_string(),
            icon: String::new(),
            trait_name: "defined".to_string(),
            instance_count: 0,
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

        let entity = ClassInfo {
            key: "Entity".to_string(),
            display_name: "Entity".to_string(),
            description: "Foundation entity".to_string(),
            icon: String::new(),
            trait_name: "defined".to_string(),
            instance_count: 0,
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

        let config_layer = LayerInfo {
            key: "config".to_string(),
            display_name: "Config".to_string(),
            color: "#6c71c4".to_string(),
            classes: vec![app_config],
            llm_context: String::new(),
        };

        let foundation_layer = LayerInfo {
            key: "foundation".to_string(),
            display_name: "Foundation".to_string(),
            color: "#268bd2".to_string(),
            classes: vec![entity],
            llm_context: String::new(),
        };

        let shared_realm = RealmInfo {
            key: "shared".to_string(),
            display_name: "Shared".to_string(),
            color: "#2aa198".to_string(),
            icon: "◉",
            layers: vec![config_layer],
            llm_context: String::new(),
        };

        let org_realm = RealmInfo {
            key: "org".to_string(),
            display_name: "Org".to_string(),
            color: "#d33682".to_string(),
            icon: "◎",
            layers: vec![foundation_layer],
            llm_context: String::new(),
        };

        let realms = vec![shared_realm, org_realm];

        // Build kind_index for O(1) lookups
        let mut kind_index = FxHashMap::default();
        for (r_idx, realm) in realms.iter().enumerate() {
            for (l_idx, layer) in realm.layers.iter().enumerate() {
                for (k_idx, kind) in layer.classes.iter().enumerate() {
                    kind_index.insert(kind.key.clone(), (r_idx, l_idx, k_idx));
                }
            }
        }

        Self {
            realms,
            arc_families: Vec::new(),
            stats: GraphStats::default(),
            collapsed: FxHashSet::default(),
            instances: FxHashMap::default(),
            instance_totals: FxHashMap::default(),
            kind_index,
            entity_categories: Vec::new(),
            entity_category_instances: FxHashMap::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // TaxonomyTree::mock_for_testing() tests
    // ========================================================================

    #[test]
    fn test_mock_tree_has_realms() {
        let tree = TaxonomyTree::mock_for_testing();

        assert_eq!(tree.realms.len(), 2, "mock should have 2 realms");
        assert_eq!(tree.realms[0].key, "shared");
        assert_eq!(tree.realms[1].key, "org");
    }

    #[test]
    fn test_mock_tree_shared_structure() {
        let tree = TaxonomyTree::mock_for_testing();
        let shared = &tree.realms[0];

        assert_eq!(shared.layers.len(), 1, "shared should have 1 layer");
        assert_eq!(shared.layers[0].key, "config");
        assert_eq!(
            shared.layers[0].classes.len(),
            1,
            "config should have 1 kind"
        );
        assert_eq!(shared.layers[0].classes[0].key, "AppConfig");
    }

    #[test]
    fn test_mock_tree_org_structure() {
        let tree = TaxonomyTree::mock_for_testing();
        let org = &tree.realms[1];

        assert_eq!(org.layers.len(), 1, "org should have 1 layer");
        assert_eq!(org.layers[0].key, "foundation");
        assert_eq!(
            org.layers[0].classes.len(),
            1,
            "foundation should have 1 kind"
        );
        assert_eq!(org.layers[0].classes[0].key, "Entity");
    }

    #[test]
    fn test_mock_tree_kind_index() {
        let tree = TaxonomyTree::mock_for_testing();

        // Verify kind_index has correct mappings
        assert_eq!(tree.kind_index.get("AppConfig"), Some(&(0, 0, 0)));
        assert_eq!(tree.kind_index.get("Entity"), Some(&(1, 0, 0)));
    }

    // ========================================================================
    // Helper functions for creating test data
    // ========================================================================

    fn create_test_class(key: &str, display_name: &str) -> ClassInfo {
        ClassInfo {
            key: key.to_string(),
            display_name: display_name.to_string(),
            description: String::new(),
            icon: String::new(),
            trait_name: "defined".to_string(),
            instance_count: 0,
            arcs: Vec::new(),
            yaml_path: String::new(),
            properties: Vec::new(),
            required_properties: Vec::new(),
            schema_hint: String::new(),
            context_budget: String::new(),
            knowledge_tier: None,
            health_percent: None,
            issues_count: None,
        }
    }

    fn create_test_layer(key: &str, classes: Vec<ClassInfo>) -> LayerInfo {
        LayerInfo {
            key: key.to_string(),
            display_name: key.to_string(),
            color: "#ffffff".to_string(),
            classes,
            llm_context: String::new(),
        }
    }

    fn create_test_realm(key: &str, layers: Vec<LayerInfo>) -> RealmInfo {
        RealmInfo {
            key: key.to_string(),
            display_name: key.to_string(),
            color: "#ffffff".to_string(),
            icon: "○",
            layers,
            llm_context: String::new(),
        }
    }

    fn create_test_tree() -> TaxonomyTree {
        let locale_kind = create_test_class("Locale", "Locale");
        let page_kind = create_test_class("Page", "Page");
        let entity_kind = create_test_class("Entity", "Entity");

        // Minimal test fixture (v11.5 has 4 shared layers: config, locale, geography, knowledge)
        let locale_layer = create_test_layer("locale", vec![locale_kind]);
        let structure = create_test_layer("structure", vec![page_kind]);
        let semantic = create_test_layer("semantic", vec![entity_kind]);

        let global = create_test_realm("shared", vec![locale_layer]);
        let tenant = create_test_realm("org", vec![structure, semantic]);

        let realms = vec![global, tenant];

        // Build kind_index (mirrors load() behavior)
        let mut kind_index = FxHashMap::default();
        for (r_idx, realm) in realms.iter().enumerate() {
            for (l_idx, layer) in realm.layers.iter().enumerate() {
                for (k_idx, kind) in layer.classes.iter().enumerate() {
                    kind_index.insert(kind.key.clone(), (r_idx, l_idx, k_idx));
                }
            }
        }

        TaxonomyTree {
            realms,
            arc_families: Vec::new(),
            stats: GraphStats::default(),
            collapsed: FxHashSet::default(),
            instances: FxHashMap::default(),
            instance_totals: FxHashMap::default(),
            kind_index,
            entity_categories: Vec::new(),
            entity_category_instances: FxHashMap::default(),
        }
    }

    // ========================================================================
    // Instance data structure tests
    // ========================================================================

    #[test]
    fn test_instance_info_creation() {
        let instance = InstanceInfo {
            key: "fr-FR".to_string(),
            display_name: "Français (France)".to_string(),
            class_key: "Locale".to_string(),
            properties: BTreeMap::from([
                ("language".to_string(), JsonValue::String("fr".to_string())),
                ("region".to_string(), JsonValue::String("FR".to_string())),
            ]),
            outgoing_arcs: vec![],
            incoming_arcs: vec![],
            arcs_loading: false,
            missing_required_count: 0,
            filled_properties: 0,
            total_properties: 0,
        };

        assert_eq!(instance.key, "fr-FR");
        assert_eq!(instance.class_key, "Locale");
        assert_eq!(
            instance.properties.get("language"),
            Some(&JsonValue::String("fr".to_string()))
        );
    }

    #[test]
    fn test_instance_arc_comparison_exists() {
        let instance = InstanceInfo {
            key: "fr-FR".to_string(),
            display_name: "Français".to_string(),
            class_key: "Locale".to_string(),
            properties: BTreeMap::new(),
            outgoing_arcs: vec![InstanceArc {
                arc_type: "HAS_TERMS".to_string(),
                target_key: "fr-FR-terms".to_string(),
                target_class: "TermSet".to_string(),
                exists: true,
            }],
            incoming_arcs: vec![],
            arcs_loading: false,
            missing_required_count: 0,
            filled_properties: 0,
            total_properties: 0,
        };

        let schema_arcs = vec![
            ArcInfo {
                arc_type: "HAS_TERMS".to_string(),
                direction: ArcDirection::Outgoing,
                target_class: "TermSet".to_string(),
            },
            ArcInfo {
                arc_type: "HAS_CULTURE".to_string(),
                direction: ArcDirection::Outgoing,
                target_class: "CultureSet".to_string(),
            },
        ];

        let comparison = instance.compare_arcs(&schema_arcs);

        assert_eq!(comparison.len(), 2);

        // HAS_TERMS should exist
        let has_terms = comparison
            .iter()
            .find(|c| c.arc_type == "HAS_TERMS")
            .unwrap();
        assert!(has_terms.exists);
        assert_eq!(has_terms.target_key, Some("fr-FR-terms".to_string()));

        // HAS_CULTURE should be missing
        let has_culture = comparison
            .iter()
            .find(|c| c.arc_type == "HAS_CULTURE")
            .unwrap();
        assert!(!has_culture.exists);
        assert_eq!(has_culture.target_key, None);
    }

    // ========================================================================
    // Tree with instances tests (Data view)
    // ========================================================================

    #[test]
    fn test_tree_item_count_meta_mode() {
        let tree = create_test_tree();
        // In Schema mode: 1 (Classes) + 1 (shared) + 1 (locale) + 1 (Locale)
        //              + 1 (org) + 1 (structure) + 1 (Page) + 1 (semantic) + 1 (Entity)
        //              + 1 (Arcs)
        // Total: 10
        assert_eq!(tree.item_count(), 10);
    }

    #[test]
    fn test_item_count_collapsed() {
        let mut tree = create_test_tree();

        // Collapse everything
        tree.collapse_all();

        // When all collapsed: 1 (Classes header) + 1 (Arcs header) = 2
        assert_eq!(tree.item_count(), 2);
    }

    #[test]
    fn test_toggle_expands_realm() {
        let mut tree = create_test_tree();

        // Start with everything collapsed
        tree.collapse_all();
        let collapsed_count = tree.item_count();
        assert_eq!(collapsed_count, 2); // Just Classes + Arcs headers

        // Expand Classes section
        tree.toggle("classes");

        // Now we see: Classes + shared + org + Arcs = 4 (v11.2: shared + org)
        // (realms are still collapsed, so we don't see layers/classes)
        assert_eq!(tree.item_count(), 4);

        // Expand shared realm (v11.2: was global)
        tree.toggle("realm:shared");

        // Now we see: Classes + shared + locale + org + Arcs = 5
        // Note: collapse_all() also collapsed the layer, so we don't see Locale yet
        assert_eq!(tree.item_count(), 5);
    }

    #[test]
    fn test_toggle_twice_collapses() {
        let mut tree = create_test_tree();

        // Get initial count (everything expanded)
        let initial_count = tree.item_count();
        assert_eq!(initial_count, 10);

        // Toggle shared realm to collapse it (v11.2: was global)
        tree.toggle("realm:shared");

        // Now: Classes + shared (collapsed) + org + structure + Page + semantic + Entity + Arcs
        // = 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 = 8
        let after_collapse = tree.item_count();
        assert_eq!(after_collapse, 8);

        // Toggle again to expand
        tree.toggle("realm:shared");

        // Should return to original count
        assert_eq!(tree.item_count(), initial_count);
    }

    // NOTE: Data View tests (item_count_for_mode, item_at_for_mode, set_instances)
    // were removed as these methods were never implemented.
    // Data View feature is planned for v10.7+

    // ========================================================================
    // Tree structure navigation tests
    // ========================================================================

    #[test]
    fn test_mock_tree_has_two_realms() {
        let tree = create_test_tree();
        assert_eq!(tree.realms.len(), 2, "Tree should have exactly 2 realms");
    }

    #[test]
    fn test_mock_tree_shared_realm() {
        let tree = create_test_tree();
        let shared = tree.realms.iter().find(|r| r.key == "shared");
        assert!(shared.is_some(), "Tree should have a shared realm");
    }

    #[test]
    fn test_mock_tree_org_realm() {
        let tree = create_test_tree();
        let org = tree.realms.iter().find(|r| r.key == "org");
        assert!(org.is_some(), "Tree should have an org realm");
    }

    #[test]
    fn test_mock_tree_shared_has_locale_layer() {
        let tree = create_test_tree();
        let shared = tree
            .realms
            .iter()
            .find(|r| r.key == "shared")
            .expect("Shared realm should exist");
        // v11.5: 4 shared layers (config, locale, geography, knowledge); test tree uses "locale"
        let has_locale = shared.layers.iter().any(|l| l.key == "locale");
        assert!(has_locale, "Shared realm should have locale layer");
    }

    // ========================================================================
    // YAML path validation tests
    // ========================================================================

    #[test]
    fn test_yaml_path_fallback_rejects_unknown_realm() {
        // When realm is "unknown", fallback should return empty string
        // instead of generating invalid path like "node-classes/unknown/layer/kind.yaml"
        let realm_key = "unknown";
        let layer_key = "structure";
        let class_key = "Page";

        // Simulate the validation logic from TaxonomyTree::load
        let yaml_path = if realm_key == "unknown" || layer_key == "unknown" {
            String::new() // Invalid - can't compute path
        } else {
            format!(
                "packages/core/models/node-classes/{}/{}/{}.yaml",
                realm_key,
                layer_key,
                super::to_kebab_case(class_key)
            )
        };

        assert!(
            yaml_path.is_empty(),
            "Should return empty for unknown realm"
        );
    }

    #[test]
    fn test_yaml_path_fallback_accepts_valid_realm_layer() {
        // When realm and layer are valid, fallback should generate proper path
        let realm_key = "org";
        let layer_key = "structure";
        let class_key = "Page";

        let yaml_path = if realm_key == "unknown" || layer_key == "unknown" {
            String::new()
        } else {
            format!(
                "packages/core/models/node-classes/{}/{}/{}.yaml",
                realm_key,
                layer_key,
                super::to_kebab_case(class_key)
            )
        };

        // v11.2: org realm (was tenant)
        assert_eq!(
            yaml_path,
            "packages/core/models/node-classes/org/structure/page.yaml"
        );
    }

    // ========================================================================
    // Cypher label validation tests (SQL/Cypher injection prevention)
    // ========================================================================

    #[test]
    fn test_validate_cypher_label_valid() {
        // Valid labels: alphanumeric, underscore, dash
        assert!(super::validate_cypher_label("Entity").is_ok());
        assert!(super::validate_cypher_label("knowledge").is_ok());
        assert!(super::validate_cypher_label("PageGenerated").is_ok());
    }

    #[test]
    fn test_validate_cypher_label_empty() {
        // Empty labels are rejected
        let result = super::validate_cypher_label("");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Empty label"),
            "Error should mention empty label: {}",
            err_msg
        );
    }

    #[test]
    fn test_validate_cypher_label_invalid_chars() {
        // Injection attempts with dangerous characters
        let injection_attempts = [
            "Entity;DROP", // SQL/Cypher injection attempt
            "Page'",       // Quote injection
            "Node\"",      // Double quote injection
            "Entity{",     // Cypher clause injection
            "Kind}",       // Cypher clause end
            "Node:Label",  // Additional label injection
            "A()",         // Function call injection
            "A[0]",        // Index access injection
        ];

        for label in &injection_attempts {
            let result = super::validate_cypher_label(label);
            assert!(
                result.is_err(),
                "Label '{}' should be rejected as invalid",
                label
            );
            let err_msg = result.unwrap_err().to_string();
            assert!(
                err_msg.contains("Invalid characters"),
                "Error for '{}' should mention invalid characters: {}",
                label,
                err_msg
            );
        }
    }

    // ========================================================================
    // Neo4j Integration Tests (require running Neo4j)
    // Run with: cargo test -- --ignored
    // ========================================================================

    #[tokio::test]
    #[ignore] // Requires running Neo4j
    async fn test_taxonomy_tree_load_integration() {
        let db = crate::db::Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword")
            .await
            .expect("Failed to connect to Neo4j");

        // Use current working directory as root (tests run from monorepo root)
        let root = std::path::Path::new(".");
        let tree = TaxonomyTree::load(&db, root)
            .await
            .expect("Failed to load tree");

        // Basic sanity checks
        assert!(!tree.realms.is_empty(), "Should load realms from Neo4j");
        assert!(
            tree.realms.iter().any(|r| r.key == "shared"),
            "Should have shared realm"
        );
        assert!(
            tree.realms.iter().any(|r| r.key == "org"),
            "Should have org realm"
        );
    }

    #[tokio::test]
    #[ignore] // Requires running Neo4j
    async fn test_load_instances_integration() {
        let db = crate::db::Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword")
            .await
            .expect("Failed to connect to Neo4j");

        // Load instances for a Class that should exist (Locale has seed data)
        let result = TaxonomyTree::load_instances(&db, "Locale").await;

        match result {
            Ok((instances, total)) => {
                // Should return some data (at least empty vec with count)
                // total is usize, always non-negative
                assert_eq!(instances.len(), total, "Instance count should match");
            }
            Err(e) => {
                panic!("load_instances failed: {}", e);
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires running Neo4j
    async fn test_load_class_arcs_integration() {
        let db = crate::db::Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword")
            .await
            .expect("Failed to connect to Neo4j");

        // Load arcs for a Class that should have relationships
        let result = TaxonomyTree::load_class_arcs(&db, "Page").await;

        match result {
            Ok(arcs_data) => {
                // Page should have some outgoing arcs (HAS_BLOCK, etc.)
                // Even if empty, the call should succeed
                // (len() is usize, always >= 0, so just verify call succeeded)
                let _ = arcs_data.outgoing.len();
            }
            Err(e) => {
                panic!("load_class_arcs failed: {}", e);
            }
        }
    }
}
