//! Data loading for TUI — Neo4j queries for taxonomy tree, stats, and detail.

use crate::db::Db;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use tokio::join;

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

/// Arc type for a Kind (from schema).
#[derive(Debug, Clone)]
pub struct ArcInfo {
    pub rel_type: String,
    pub direction: ArcDirection,
    pub target_kind: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArcDirection {
    Outgoing, // →
    Incoming, // ←
}

/// An ArcKind in the arcs tree.
#[derive(Debug, Clone)]
pub struct ArcKindInfo {
    pub key: String,
    pub display_name: String,
    pub from_kind: String,
    pub to_kind: String,
    pub cardinality: String,
    pub description: String,
}

/// ArcFamily containing ArcKinds.
#[derive(Debug, Clone)]
pub struct ArcFamilyInfo {
    pub key: String,
    pub display_name: String,
    pub arc_kinds: Vec<ArcKindInfo>,
}

/// A Kind in the taxonomy tree.
#[derive(Debug, Clone)]
#[allow(dead_code)] // schema_hint reserved for future use
pub struct KindInfo {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub icon: String,
    pub trait_name: String,
    pub instance_count: i64,
    pub arcs: Vec<ArcInfo>,
    pub yaml_path: String,
    // Schema properties (from Neo4j Kind node)
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

/// Layer containing Kinds.
#[derive(Debug, Clone)]
pub struct LayerInfo {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub kinds: Vec<KindInfo>,
}

/// Realm containing Layers.
#[derive(Debug, Clone)]
pub struct RealmInfo {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub icon: &'static str,
    pub layers: Vec<LayerInfo>,
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
// Atlas Mode Data Types
// ============================================================================

/// Atlas Realm Map statistics.
#[derive(Debug, Clone, Default)]
pub struct AtlasRealmStats {
    pub realms: Vec<AtlasRealmInfo>,
    pub total_kinds: usize,
}

/// Atlas realm info for Realm Map view.
#[derive(Debug, Clone)]
pub struct AtlasRealmInfo {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub layers: Vec<AtlasLayerInfo>,
    pub total_kinds: usize,
}

/// Atlas layer info for Realm Map view.
#[derive(Debug, Clone)]
pub struct AtlasLayerInfo {
    pub key: String,
    pub display_name: String,
    pub color: String,
    pub kind_count: usize,
}

/// Atlas page info for Page Composition navigation.
#[derive(Debug, Clone)]
pub struct AtlasPageInfo {
    pub key: String,
    pub display_name: String,
    pub project_key: String,
    pub project_name: String,
}

// ============================================================================
// Neo4j Arc Data (live query)
// ============================================================================

/// A single arc relationship from Neo4j.
#[derive(Debug, Clone)]
pub struct Neo4jArc {
    pub arc_key: String,    // e.g., "FALLBACK_TO"
    pub other_kind: String, // The Kind on the other end
    pub family: String,     // e.g., "localization", "ownership"
}

/// Complete arc data for a Kind, loaded from Neo4j.
#[derive(Debug, Clone, Default)]
pub struct KindArcsData {
    pub kind_label: String,
    pub realm: String,
    pub layer: String,
    pub incoming: Vec<Neo4jArc>,
    pub outgoing: Vec<Neo4jArc>,
}

/// Endpoint info for an ArcKind (from/to Kind).
#[derive(Debug, Clone)]
pub struct ArcEndpoint {
    pub kind_label: String,
    pub realm: String,
    pub layer: String,
}

/// Complete details for an ArcKind, loaded from Neo4j.
#[derive(Debug, Clone, Default)]
pub struct ArcKindDetails {
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
    pub kind_count: usize,
}

/// Complete details for a Realm, loaded from Neo4j.
#[derive(Debug, Clone, Default)]
pub struct RealmDetails {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub layers: Vec<LayerStats>,
    pub total_kinds: usize,
    pub total_instances: usize,
}

/// Kind stats grouped by trait for Layer details view.
#[derive(Debug, Clone)]
pub struct TraitKindGroup {
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
    pub kinds_by_trait: Vec<TraitKindGroup>,
    pub total_kinds: usize,
    pub total_instances: usize,
}

/// Full taxonomy tree: Realm > Layer > Kind + ArcFamily > ArcKind.
#[derive(Debug, Clone, Default)]
pub struct TaxonomyTree {
    pub realms: Vec<RealmInfo>,
    pub arc_families: Vec<ArcFamilyInfo>,
    pub stats: GraphStats,
    /// Collapsed state: stores keys of collapsed nodes (e.g., "kinds", "arcs", "realm:global", "layer:structure")
    /// Uses FxHashSet for ~30% faster lookups on string keys.
    pub collapsed: FxHashSet<String>,
    /// Instances loaded for Data view, keyed by Kind key.
    /// Only populated when in Data mode and a Kind is selected.
    pub instances: BTreeMap<String, Vec<InstanceInfo>>,
    /// Total instance counts in Neo4j (may be > loaded instances due to LIMIT).
    /// Used to show "3/500 of 847" when results are truncated.
    pub instance_totals: BTreeMap<String, usize>,
    /// Cache: kind_key -> (realm_idx, layer_idx, kind_idx) for O(1) lookups.
    /// Built once on load, never mutated (tree structure is immutable).
    pub(crate) kind_index: FxHashMap<String, (usize, usize, usize)>,
}

impl TaxonomyTree {
    /// Load taxonomy tree from Neo4j.
    pub async fn load(db: &Db) -> crate::Result<Self> {
        // Query all Kinds with their realm, layer, trait, and instance count
        // Note: Kind uses 'label' property as identifier, not 'key'
        let cypher = r#"
MATCH (k:Kind:Meta)
OPTIONAL MATCH (k)-[:IN_REALM]->(r:Realm)
OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer)
OPTIONAL MATCH (k)-[:HAS_TRAIT]->(t:Trait)
OPTIONAL MATCH (n)-[:OF_KIND]->(k)
WITH k, r, l, t, count(n) AS instances
RETURN
    k.label AS kind_key,
    coalesce(k.display_name, k.label) AS kind_display,
    coalesce(k.llm_context, '') AS kind_desc,
    coalesce(k.icon, '') AS kind_icon,
    coalesce(t.key, '') AS trait_key,
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
ORDER BY realm_key, layer_key, kind_key
"#;

        let rows = db.execute(cypher).await?;

        // Group into tree structure: realm_key -> (realm_display, realm_color, layer_key -> (layer_display, layer_color, kinds))
        #[allow(clippy::type_complexity)]
        let mut realm_map: BTreeMap<
            String,
            (
                String,
                String,
                BTreeMap<String, (String, String, Vec<KindInfo>)>,
            ),
        > = BTreeMap::new();

        for row in rows {
            let kind_key: String = row.get("kind_key").unwrap_or_default();
            let kind_display: String = row.get("kind_display").unwrap_or_default();
            let kind_desc: String = row.get("kind_desc").unwrap_or_default();
            let kind_icon: String = row.get("kind_icon").unwrap_or_default();
            let trait_key: String = row.get("trait_key").unwrap_or_default();
            let realm_key: String = row.get("realm_key").unwrap_or_default();
            let realm_display: String = row.get("realm_display").unwrap_or_default();
            let realm_color: String = row.get("realm_color").unwrap_or_default();
            let layer_key: String = row.get("layer_key").unwrap_or_default();
            let layer_display: String = row.get("layer_display").unwrap_or_default();
            let layer_color: String = row.get("layer_color").unwrap_or_default();
            let instances: i64 = row.get("instances").unwrap_or(0);

            // Get YAML path from Neo4j (with fallback to computed path)
            let yaml_path_raw: String = row.get("yaml_path").unwrap_or_default();
            let yaml_path = if !yaml_path_raw.is_empty() {
                // Neo4j stores relative path like "node-kinds/tenant/structure/block.yaml"
                // We need to prefix with "packages/core/models/"
                format!("packages/core/models/{}", yaml_path_raw)
            } else if realm_key == "unknown" || layer_key == "unknown" {
                // Missing realm/layer relationship - can't compute valid path
                // Return empty to signal "file not found" in UI (better than invalid path)
                String::new()
            } else {
                // Fallback: compute path from realm/layer
                format!(
                    "packages/core/models/node-kinds/{}/{}/{}.yaml",
                    realm_key,
                    layer_key,
                    to_kebab_case(&kind_key)
                )
            };

            // Get schema properties from Neo4j
            let properties: Vec<String> = row.get("properties").unwrap_or_default();
            let required_properties: Vec<String> =
                row.get("required_properties").unwrap_or_default();
            let schema_hint: String = row.get("schema_hint").unwrap_or_default();
            let context_budget: String = row.get("context_budget").unwrap_or_default();
            // v10: knowledge_tier (optional, only for knowledge-trait nodes)
            let knowledge_tier: Option<String> = row.get("knowledge_tier").ok();

            let kind = KindInfo {
                key: kind_key,
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

        // Convert to RealmInfo vec
        let realms: Vec<RealmInfo> = realm_map
            .into_iter()
            .map(|(realm_key, (realm_display, realm_color, layers_map))| {
                let layers: Vec<LayerInfo> = layers_map
                    .into_iter()
                    .map(
                        |(layer_key, (layer_display, layer_color, kinds))| LayerInfo {
                            key: layer_key,
                            display_name: layer_display,
                            color: layer_color,
                            kinds,
                        },
                    )
                    .collect();

                RealmInfo {
                    icon: realm_icon(&realm_key),
                    key: realm_key,
                    display_name: realm_display,
                    color: realm_color,
                    layers,
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
        let arc_families = families_result.unwrap_or_default();

        // Apply arcs to kinds
        let realms = Self::apply_arcs_to_realms(realms, arc_map);

        // Build kind_index for O(1) lookups (replaces O(n*m*k) find_kind)
        let mut kind_index = FxHashMap::default();
        for (r_idx, realm) in realms.iter().enumerate() {
            for (l_idx, layer) in realm.layers.iter().enumerate() {
                for (k_idx, kind) in layer.kinds.iter().enumerate() {
                    kind_index.insert(kind.key.clone(), (r_idx, l_idx, k_idx));
                }
            }
        }

        Ok(Self {
            realms,
            arc_families,
            stats,
            collapsed: FxHashSet::default(),
            instances: BTreeMap::new(),
            instance_totals: BTreeMap::new(),
            kind_index,
        })
    }

    /// Apply arc map to realm/layer/kind tree.
    fn apply_arcs_to_realms(
        mut realms: Vec<RealmInfo>,
        mut arc_map: BTreeMap<String, Vec<ArcInfo>>,
    ) -> Vec<RealmInfo> {
        for realm in &mut realms {
            for layer in &mut realm.layers {
                for kind in &mut layer.kinds {
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
MATCH (ak:ArcKind:Meta)-[:FROM_KIND]->(fromKind:Kind:Meta)
MATCH (ak)-[:TO_KIND]->(toKind:Kind:Meta)
RETURN fromKind.label AS kind_key, ak.key AS rel_type, 'outgoing' AS direction, toKind.label AS target_kind
ORDER BY fromKind.label, ak.key

UNION

MATCH (ak:ArcKind:Meta)-[:FROM_KIND]->(fromKind:Kind:Meta)
MATCH (ak)-[:TO_KIND]->(toKind:Kind:Meta)
RETURN toKind.label AS kind_key, ak.key AS rel_type, 'incoming' AS direction, fromKind.label AS target_kind
ORDER BY toKind.label, ak.key
"#;

        let rows = db.execute(cypher).await?;
        let mut arc_map: BTreeMap<String, Vec<ArcInfo>> = BTreeMap::new();

        for row in rows {
            let kind_key: String = row.get("kind_key").unwrap_or_default();
            let rel_type: String = row.get("rel_type").unwrap_or_default();
            let direction_str: String = row.get("direction").unwrap_or_default();
            let target_kind: String = row.get("target_kind").unwrap_or_default();

            if kind_key.is_empty() || rel_type.is_empty() {
                continue;
            }

            let direction = if direction_str == "incoming" {
                ArcDirection::Incoming
            } else {
                ArcDirection::Outgoing
            };

            arc_map.entry(kind_key).or_default().push(ArcInfo {
                rel_type,
                direction,
                target_kind,
            });
        }

        Ok(arc_map)
    }

    /// Fetch arc families (for parallel loading).
    async fn fetch_arc_families(db: &Db) -> crate::Result<Vec<ArcFamilyInfo>> {
        let cypher = r#"
MATCH (ak:ArcKind:Meta)-[:IN_FAMILY]->(af:ArcFamily:Meta)
MATCH (ak)-[:FROM_KIND]->(fromKind:Kind:Meta)
MATCH (ak)-[:TO_KIND]->(toKind:Kind:Meta)
RETURN
    af.key AS family_key,
    coalesce(af.display_name, af.key) AS family_display,
    ak.key AS arc_key,
    coalesce(ak.display_name, ak.key) AS arc_display,
    coalesce(ak.cardinality, '') AS cardinality,
    coalesce(ak.llm_context, '') AS arc_desc,
    fromKind.label AS from_kind,
    toKind.label AS to_kind
ORDER BY family_key, arc_key
"#;

        let rows = db.execute(cypher).await?;
        let mut family_map: BTreeMap<String, (String, Vec<ArcKindInfo>)> = BTreeMap::new();

        for row in rows {
            let family_key: String = row.get("family_key").unwrap_or_default();
            let family_display: String = row.get("family_display").unwrap_or_default();
            let arc_key: String = row.get("arc_key").unwrap_or_default();
            let arc_display: String = row.get("arc_display").unwrap_or_default();
            let cardinality: String = row.get("cardinality").unwrap_or_default();
            let arc_desc: String = row.get("arc_desc").unwrap_or_default();
            let from_kind: String = row.get("from_kind").unwrap_or_default();
            let to_kind: String = row.get("to_kind").unwrap_or_default();

            if family_key.is_empty() || arc_key.is_empty() {
                continue;
            }

            let arc_kind = ArcKindInfo {
                key: arc_key,
                display_name: arc_display,
                from_kind,
                to_kind,
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
            .map(|(key, (display_name, arc_kinds))| ArcFamilyInfo {
                key,
                display_name,
                arc_kinds,
            })
            .collect())
    }

    /// Load instances of a Kind from Neo4j for Data view.
    /// Returns (instances, total_count) - instances are limited to 500, total is the real count.
    pub async fn load_instances(
        db: &Db,
        kind_label: &str,
    ) -> crate::Result<(Vec<InstanceInfo>, usize)> {
        // Security: Validate label before interpolation into Cypher
        validate_cypher_label(kind_label)?;

        // First, get the total count (fast query with index)
        let count_cypher = format!(
            "MATCH (n:{kind_label}) WHERE NOT n:Meta RETURN count(n) AS total",
            kind_label = kind_label
        );
        let count_rows = db.execute(&count_cypher).await?;
        let total_count: usize = count_rows
            .first()
            .and_then(|r| r.get::<i64>("total").ok())
            .unwrap_or(0) as usize;

        // Query instances of this Kind with their properties and arcs (limited to 500)
        let cypher = format!(
            r#"
MATCH (n:{kind_label})
WHERE NOT n:Meta
OPTIONAL MATCH (n)-[out]->(target)
WHERE NOT target:Meta
WITH n, collect(DISTINCT {{
    arc_type: type(out),
    target_key: coalesce(target.key, target.label, id(target)),
    target_kind: head(labels(target))
}}) AS outgoing
OPTIONAL MATCH (source)-[inc]->(n)
WHERE NOT source:Meta
WITH n, outgoing, collect(DISTINCT {{
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
LIMIT 500
"#,
            kind_label = kind_label
        );

        let rows = db.execute(&cypher).await?;
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
                        target_kind: m.get("target_kind").unwrap_or_default(),
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
                        target_kind: m.get("source_kind").unwrap_or_default(),
                        exists: true,
                    })
                })
                .collect();

            instances.push(InstanceInfo {
                key,
                display_name,
                kind_key: kind_label.to_string(),
                properties: props,
                outgoing_arcs,
                incoming_arcs,
                missing_required_count: 0, // Calculated later in set_instances
                filled_properties: 0,      // Calculated later in set_instances
                total_properties: 0,       // Calculated later in set_instances
            });
        }

        Ok((instances, total_count))
    }

    /// Load graph statistics.
    async fn load_stats(db: &Db) -> crate::Result<GraphStats> {
        let cypher = r#"
MATCH (n) WHERE NOT n:Meta
WITH count(n) AS nodes
MATCH ()-[r]->() WHERE NOT startNode(r):Meta AND NOT endNode(r):Meta
WITH nodes, count(r) AS arcs
MATCH (k:Kind:Meta)
WITH nodes, arcs, count(k) AS kinds
MATCH (ak:ArcKind:Meta)
RETURN nodes, arcs, kinds, count(ak) AS arc_kinds
"#;

        let rows = db.execute(cypher).await?;
        if let Some(row) = rows.into_iter().next() {
            Ok(GraphStats {
                node_count: row.get("nodes").unwrap_or(0),
                arc_count: row.get("arcs").unwrap_or(0),
                kind_count: row.get("kinds").unwrap_or(0),
                arc_kind_count: row.get("arc_kinds").unwrap_or(0),
            })
        } else {
            Ok(GraphStats::default())
        }
    }

    /// Load arc relationships for a Kind from Neo4j.
    /// Returns incoming and outgoing arcs with their families.
    pub async fn load_kind_arcs(db: &Db, kind_label: &str) -> crate::Result<KindArcsData> {
        let cypher = r#"
MATCH (k:Kind {label: $kindLabel})
OPTIONAL MATCH (k)-[:IN_LAYER]->(l:Layer)
OPTIONAL MATCH (l)<-[:HAS_LAYER]-(r:Realm)
OPTIONAL MATCH (k)<-[:TO_KIND]-(inArc:ArcKind)-[:FROM_KIND]->(fromKind:Kind)
OPTIONAL MATCH (inArc)-[:IN_FAMILY]->(inFamily:ArcFamily)
OPTIONAL MATCH (k)<-[:FROM_KIND]-(outArc:ArcKind)-[:TO_KIND]->(toKind:Kind)
OPTIONAL MATCH (outArc)-[:IN_FAMILY]->(outFamily:ArcFamily)
WITH k, r, l,
     collect(DISTINCT CASE WHEN inArc IS NOT NULL
         THEN {arc: inArc.key, from: fromKind.label, family: inFamily.key} END) as incoming,
     collect(DISTINCT CASE WHEN outArc IS NOT NULL
         THEN {arc: outArc.key, to: toKind.label, family: outFamily.key} END) as outgoing
RETURN k.label as kind,
       r.key as realm,
       l.key as layer,
       [x IN incoming WHERE x IS NOT NULL] as incoming,
       [x IN outgoing WHERE x IS NOT NULL] as outgoing
LIMIT 1
"#;

        let rows = db
            .execute_with_params(cypher, [("kindLabel", kind_label)])
            .await?;

        if let Some(row) = rows.into_iter().next() {
            let kind: String = row.get("kind").unwrap_or_default();
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
                    let other_kind = m.get::<String>("from").ok()?;
                    let family = m.get::<String>("family").ok().unwrap_or_default();
                    Some(Neo4jArc {
                        arc_key,
                        other_kind,
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
                    let other_kind = m.get::<String>("to").ok()?;
                    let family = m.get::<String>("family").ok().unwrap_or_default();
                    Some(Neo4jArc {
                        arc_key,
                        other_kind,
                        family,
                    })
                })
                .collect();

            Ok(KindArcsData {
                kind_label: kind,
                realm,
                layer,
                incoming,
                outgoing,
            })
        } else {
            Ok(KindArcsData::default())
        }
    }

    /// Load ArcKind details from Neo4j (endpoints, family, cardinality).
    pub async fn load_arc_kind_details(db: &Db, arc_key: &str) -> crate::Result<ArcKindDetails> {
        let cypher = r#"
MATCH (ak:ArcKind {key: $arcKey})
OPTIONAL MATCH (ak)-[:IN_FAMILY]->(af:ArcFamily)
OPTIONAL MATCH (ak)-[:FROM_KIND]->(fromKind:Kind)
OPTIONAL MATCH (fromKind)-[:IN_LAYER]->(fromLayer:Layer)
OPTIONAL MATCH (fromLayer)<-[:HAS_LAYER]-(fromRealm:Realm)
OPTIONAL MATCH (ak)-[:TO_KIND]->(toKind:Kind)
OPTIONAL MATCH (toKind)-[:IN_LAYER]->(toLayer:Layer)
OPTIONAL MATCH (toLayer)<-[:HAS_LAYER]-(toRealm:Realm)
RETURN coalesce(ak.display_name, ak.key) as display_name,
       coalesce(ak.llm_context, '') as description,
       coalesce(ak.cardinality, '') as cardinality,
       coalesce(ak.cypher_pattern, '') as cypher_pattern,
       coalesce(af.key, '') as family,
       fromKind.label as from_kind,
       coalesce(fromRealm.key, '') as from_realm,
       coalesce(fromLayer.key, '') as from_layer,
       toKind.label as to_kind,
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

            let from_kind: Option<String> = row.get("from_kind").ok();
            let from_realm: String = row.get("from_realm").unwrap_or_default();
            let from_layer: String = row.get("from_layer").unwrap_or_default();

            let to_kind: Option<String> = row.get("to_kind").ok();
            let to_realm: String = row.get("to_realm").unwrap_or_default();
            let to_layer: String = row.get("to_layer").unwrap_or_default();

            let from_endpoint = from_kind.map(|kind_label| ArcEndpoint {
                kind_label,
                realm: from_realm,
                layer: from_layer,
            });

            let to_endpoint = to_kind.map(|kind_label| ArcEndpoint {
                kind_label,
                realm: to_realm,
                layer: to_layer,
            });

            Ok(ArcKindDetails {
                display_name,
                description,
                family,
                cardinality,
                cypher_pattern,
                from_endpoint,
                to_endpoint,
            })
        } else {
            Ok(ArcKindDetails::default())
        }
    }

    /// Load Realm details from Neo4j (layers with kind counts, total stats).
    pub async fn load_realm_details(db: &Db, realm_key: &str) -> crate::Result<RealmDetails> {
        // Query 1: Get realm info and totals
        let cypher_realm = r#"
MATCH (r:Realm {key: $realmKey})
OPTIONAL MATCH (r)-[:HAS_LAYER]->(l:Layer)<-[:IN_LAYER]-(k:Kind)
OPTIONAL MATCH (k)<-[:OF_KIND]-(n)
RETURN r.key as realm_key,
       coalesce(r.display_name, r.key) as display_name,
       coalesce(r.llm_context, '') as description,
       count(DISTINCT k) as total_kinds,
       count(DISTINCT n) as total_instances
"#;

        // Query 2: Get layers with their kind counts (separate rows)
        let cypher_layers = r#"
MATCH (r:Realm {key: $realmKey})-[:HAS_LAYER]->(l:Layer)
OPTIONAL MATCH (l)<-[:IN_LAYER]-(k:Kind)
WITH l, count(DISTINCT k) as kind_count
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
            let total_kinds: i64 = row.get("total_kinds").unwrap_or(0);
            let total_instances: i64 = row.get("total_instances").unwrap_or(0);

            let layers: Vec<LayerStats> = layer_rows
                .into_iter()
                .map(|lr| LayerStats {
                    key: lr.get("layer_key").unwrap_or_default(),
                    display_name: lr.get("layer_display").unwrap_or_default(),
                    kind_count: lr.get::<i64>("kind_count").unwrap_or(0) as usize,
                })
                .collect();

            Ok(RealmDetails {
                key,
                display_name,
                description,
                layers,
                total_kinds: total_kinds as usize,
                total_instances: total_instances as usize,
            })
        } else {
            Ok(RealmDetails::default())
        }
    }

    /// Load Layer details from Neo4j (kinds grouped by trait, stats).
    pub async fn load_layer_details(db: &Db, layer_key: &str) -> crate::Result<LayerDetails> {
        let cypher = r#"
MATCH (l:Layer {key: $layerKey})
OPTIONAL MATCH (r:Realm)-[:HAS_LAYER]->(l)
OPTIONAL MATCH (l)<-[:IN_LAYER]-(k:Kind)
OPTIONAL MATCH (k)-[:HAS_TRAIT]->(t:Trait)
OPTIONAL MATCH (k)<-[:OF_KIND]-(n)
WITH l, r, t.key as trait_key, k, count(DISTINCT n) as inst_count
ORDER BY trait_key, k.label
WITH l, r, trait_key, collect(coalesce(k.display_name, k.label)) as kind_names, count(k) as trait_kind_count, sum(inst_count) as trait_instances
RETURN l.key as layer_key,
       coalesce(l.display_name, l.key) as display_name,
       coalesce(l.llm_context, '') as description,
       coalesce(r.key, '') as realm,
       collect({trait_key: trait_key, kind_names: kind_names}) as kinds_by_trait,
       sum(trait_kind_count) as total_kinds,
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
            let total_kinds: i64 = row.get("total_kinds").unwrap_or(0);
            let total_instances: i64 = row.get("total_instances").unwrap_or(0);

            // Parse kinds_by_trait
            let groups_list = row
                .get::<Vec<neo4rs::BoltMap>>("kinds_by_trait")
                .unwrap_or_default();
            let mut kinds_by_trait: Vec<TraitKindGroup> = Vec::with_capacity(groups_list.len());
            for group_map in groups_list {
                if let Ok(trait_key) = group_map.get::<String>("trait_key") {
                    let kind_names: Vec<String> = group_map
                        .get::<Vec<String>>("kind_names")
                        .unwrap_or_default();
                    kinds_by_trait.push(TraitKindGroup {
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
                kinds_by_trait,
                total_kinds: total_kinds as usize,
                total_instances: total_instances as usize,
            })
        } else {
            Ok(LayerDetails::default())
        }
    }

    /// Load Atlas Realm Map statistics from Neo4j.
    /// Returns layer counts organized by realm for the interactive Realm Map view.
    pub async fn load_atlas_realm_stats(db: &Db) -> crate::Result<AtlasRealmStats> {
        let cypher = r#"
MATCH (r:Realm)-[:HAS_LAYER]->(l:Layer)
OPTIONAL MATCH (k:Kind)-[:IN_LAYER]->(l)
WITH r, l, count(k) as kind_count
ORDER BY
    CASE r.key WHEN 'global' THEN 0 ELSE 1 END,
    CASE l.key
        WHEN 'config' THEN 0
        WHEN 'locale-knowledge' THEN 1
        WHEN 'seo' THEN 2
        WHEN 'foundation' THEN 3
        WHEN 'structure' THEN 4
        WHEN 'semantic' THEN 5
        WHEN 'instruction' THEN 6
        WHEN 'output' THEN 7
        ELSE 8
    END
RETURN r.key as realm_key,
       coalesce(r.display_name, r.key) as realm_name,
       coalesce(r.color, '#888888') as realm_color,
       collect({
           layer_key: l.key,
           layer_name: coalesce(l.display_name, l.key),
           layer_color: coalesce(l.color, '#888888'),
           kind_count: kind_count
       }) as layers
"#;

        let rows = db.execute(cypher).await?;

        let mut realms: Vec<AtlasRealmInfo> = Vec::with_capacity(rows.len());
        let mut total_kinds = 0;

        for row in rows {
            let realm_key: String = row.get("realm_key").unwrap_or_default();
            let realm_name: String = row.get("realm_name").unwrap_or_default();
            let realm_color: String = row.get("realm_color").unwrap_or_default();

            let layer_list = row
                .get::<Vec<neo4rs::BoltMap>>("layers")
                .unwrap_or_default();
            let mut layers: Vec<AtlasLayerInfo> = Vec::with_capacity(layer_list.len());
            let mut realm_kind_count = 0;

            for layer_map in layer_list {
                let layer_key: String = layer_map.get("layer_key").unwrap_or_default();
                let layer_name: String = layer_map.get("layer_name").unwrap_or_default();
                let layer_color: String = layer_map.get("layer_color").unwrap_or_default();
                let kind_count: i64 = layer_map.get("kind_count").unwrap_or(0);

                realm_kind_count += kind_count;
                layers.push(AtlasLayerInfo {
                    key: layer_key,
                    display_name: layer_name,
                    color: layer_color,
                    kind_count: kind_count as usize,
                });
            }

            total_kinds += realm_kind_count;
            realms.push(AtlasRealmInfo {
                key: realm_key,
                display_name: realm_name,
                color: realm_color,
                layers,
                total_kinds: realm_kind_count as usize,
            });
        }

        Ok(AtlasRealmStats {
            realms,
            total_kinds: total_kinds as usize,
        })
    }

    /// Load list of Pages for Atlas Page Composition navigation.
    pub async fn load_atlas_pages_list(db: &Db) -> crate::Result<Vec<AtlasPageInfo>> {
        let cypher = r#"
MATCH (p:Page)
OPTIONAL MATCH (p)<-[:HAS_PAGE]-(proj:Project)
WITH p, proj
ORDER BY proj.key, p.key
RETURN p.key as page_key,
       coalesce(p.display_name, p.key) as display_name,
       coalesce(proj.key, 'unknown') as project_key,
       coalesce(proj.display_name, proj.key, 'Unknown') as project_name
"#;

        let rows = db.execute(cypher).await?;
        let mut pages = Vec::with_capacity(rows.len());

        for row in rows {
            pages.push(AtlasPageInfo {
                key: row.get("page_key").unwrap_or_default(),
                display_name: row.get("display_name").unwrap_or_default(),
                project_key: row.get("project_key").unwrap_or_default(),
                project_name: row.get("project_name").unwrap_or_default(),
            });
        }

        Ok(pages)
    }

    /// Load Page Composition data for Atlas mode.
    /// Returns full anatomy: Page → Blocks → Entities → SEO Keywords with L10n.
    pub async fn load_atlas_page_composition(
        db: &Db,
        page_key: &str,
        locale: &str,
    ) -> crate::Result<super::atlas::PageCompositionData> {
        use super::atlas::{
            BlockData, BlockGeneratedData, EntityData, EntityContentData, PageCompositionData,
            PageGeneratedData, SeoKeywordData,
        };

        // Query 1: Page info and generated output (v10.9.0)
        let page_cypher = r#"
MATCH (p:Page {key: $pageKey})
OPTIONAL MATCH (p)<-[:HAS_PAGE]-(proj:Project)
OPTIONAL MATCH (p)-[:HAS_GENERATED]->(pg:PageGenerated)-[:FOR_LOCALE]->(loc:Locale {bcp47: $locale})
RETURN p.key as page_key,
       coalesce(p.display_name, p.key) as page_name,
       p.page_type as page_type,
       p.prompt as prompt,
       coalesce(proj.key, 'unknown') as project_key,
       coalesce(proj.display_name, proj.key, 'Unknown') as project_name,
       pg.title as l10n_title,
       pg.slug as l10n_slug,
       pg.meta_description as l10n_meta
"#;

        // Query 2: Blocks with generated output (v10.9.0)
        let blocks_cypher = r#"
MATCH (p:Page {key: $pageKey})-[:HAS_BLOCK]->(b:Block)
OPTIONAL MATCH (b)-[:HAS_GENERATED]->(bg:BlockGenerated)-[:FOR_LOCALE]->(loc:Locale {bcp47: $locale})
WITH b, bg
ORDER BY b.order
RETURN b.key as block_key,
       coalesce(b.display_name, b.key) as block_name,
       coalesce(b.order, 0) as block_order,
       b.block_type as block_type,
       b.prompt as prompt,
       b.rules as rules,
       substring(coalesce(bg.content, ''), 0, 100) as content_preview
"#;

        // Query 3: Entities used by blocks with content (v10.9.0)
        let entities_cypher = r#"
MATCH (p:Page {key: $pageKey})-[:HAS_BLOCK]->(b:Block)-[:USES_ENTITY]->(e:Entity)
OPTIONAL MATCH (e)-[:HAS_CONTENT]->(ec:EntityContent)-[:FOR_LOCALE]->(loc:Locale {bcp47: $locale})
WITH e, ec, collect(b.key) as connected_blocks
RETURN e.key as entity_key,
       coalesce(e.display_name, e.key) as entity_name,
       ec.display_name as l10n_name,
       substring(coalesce(ec.description, ''), 0, 80) as l10n_desc,
       connected_blocks
"#;

        // Query 4: SEO Keywords connected to entities
        let seo_cypher = r#"
MATCH (p:Page {key: $pageKey})-[:HAS_BLOCK]->(b:Block)-[:USES_ENTITY]->(e:Entity)<-[:EXPRESSES]-(kw:SEOKeyword)
WITH kw, collect(DISTINCT e.key) as entity_keys
OPTIONAL MATCH (kw)-[:HAS_METRICS]->(m:SEOKeywordMetrics)
RETURN kw.keyword as keyword,
       m.monthly_volume as volume,
       entity_keys
"#;

        // Execute all 4 queries in parallel using tokio::join!
        let (page_result, block_result, entity_result, seo_result) = tokio::join!(
            db.execute_with_params(page_cypher, [("pageKey", page_key), ("locale", locale)]),
            db.execute_with_params(blocks_cypher, [("pageKey", page_key), ("locale", locale)]),
            db.execute_with_params(entities_cypher, [("pageKey", page_key), ("locale", locale)]),
            db.execute_with_params(seo_cypher, [("pageKey", page_key), ("locale", locale)]),
        );

        let page_rows = page_result?;
        let block_rows = block_result?;
        let entity_rows = entity_result?;
        let seo_rows = seo_result?;

        let Some(page_row) = page_rows.into_iter().next() else {
            return Ok(PageCompositionData::default());
        };

        let page_l10n = if page_row
            .get::<Option<String>>("l10n_title")
            .ok()
            .flatten()
            .is_some()
        {
            Some(PageGeneratedData {
                locale: locale.to_string(),
                title: page_row.get("l10n_title").ok(),
                slug: page_row.get("l10n_slug").ok(),
                meta_description: page_row.get("l10n_meta").ok(),
            })
        } else {
            None
        };

        let mut blocks = Vec::with_capacity(block_rows.len());
        for row in block_rows {
            let content_preview: String = row.get("content_preview").unwrap_or_default();
            let l10n = if !content_preview.is_empty() {
                Some(BlockGeneratedData {
                    locale: locale.to_string(),
                    content_preview,
                })
            } else {
                None
            };

            blocks.push(BlockData {
                key: row.get("block_key").unwrap_or_default(),
                display_name: row.get("block_name").unwrap_or_default(),
                order: row.get("block_order").unwrap_or(0),
                block_type: row.get("block_type").ok(),
                prompt: row.get("prompt").ok(),
                rules: row.get("rules").ok(),
                l10n,
            });
        }

        let mut entities = Vec::with_capacity(entity_rows.len());
        for row in entity_rows {
            let l10n_name: Option<String> = row.get("l10n_name").ok();
            let l10n_desc: String = row.get("l10n_desc").unwrap_or_default();
            let l10n = if l10n_name.is_some() || !l10n_desc.is_empty() {
                Some(EntityContentData {
                    locale: locale.to_string(),
                    name: l10n_name,
                    description_preview: if l10n_desc.is_empty() {
                        None
                    } else {
                        Some(l10n_desc)
                    },
                })
            } else {
                None
            };

            let connected_blocks: Vec<String> = row
                .get::<Vec<String>>("connected_blocks")
                .unwrap_or_default();

            entities.push(EntityData {
                key: row.get("entity_key").unwrap_or_default(),
                display_name: row.get("entity_name").unwrap_or_default(),
                l10n,
                connected_blocks,
            });
        }

        let mut seo_keywords = Vec::with_capacity(seo_rows.len());
        for row in seo_rows {
            seo_keywords.push(SeoKeywordData {
                keyword: row.get("keyword").unwrap_or_default(),
                volume: row.get("volume").ok(),
                connected_entities: row.get("entity_keys").unwrap_or_default(),
            });
        }

        Ok(PageCompositionData {
            page_key: page_row.get("page_key").unwrap_or_default(),
            page_display_name: page_row.get("page_name").unwrap_or_default(),
            project_key: page_row.get("project_key").unwrap_or_default(),
            project_display_name: page_row.get("project_name").unwrap_or_default(),
            page_type: page_row.get("page_type").ok(),
            page_prompt: page_row.get("prompt").ok(),
            page_l10n,
            blocks,
            entities,
            seo_keywords,
        })
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
        self.collapsed.insert("kinds".to_string());
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

    /// Collapse all Kind instances (hide their instances).
    /// Used when switching between Meta and Data modes.
    pub fn collapse_all_kinds(&mut self) {
        for realm in &self.realms {
            for layer in &realm.layers {
                for kind in &layer.kinds {
                    self.collapsed.insert(format!("kind:{}", kind.key));
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
        if key == "kinds" {
            // Expand all realms and layers
            for realm in &self.realms {
                self.collapsed.remove(&format!("realm:{}", realm.key));
                for layer in &realm.layers {
                    self.collapsed
                        .remove(&format!("layer:{}:{}", realm.key, layer.key));
                    for kind in &layer.kinds {
                        self.collapsed.remove(&format!("kind:{}", kind.key));
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
                    for kind in &layer.kinds {
                        self.collapsed.remove(&format!("kind:{}", kind.key));
                    }
                }
            }
        } else if let Some(rest) = key.strip_prefix("layer:") {
            // Layer key format: layer:{realm_key}:{layer_key}
            // Expand all kinds in this layer
            if let Some((realm_key, layer_key)) = rest.split_once(':') {
                if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                    if let Some(layer) = realm.layers.iter().find(|l| l.key == layer_key) {
                        for kind in &layer.kinds {
                            self.collapsed.remove(&format!("kind:{}", kind.key));
                        }
                    }
                }
            }
        } else if let Some(family_key) = key.strip_prefix("family:") {
            // Arc family - nothing more to expand (arc kinds aren't collapsible)
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
        if key == "kinds" {
            // Collapse all realms and layers
            for realm in &self.realms {
                self.collapsed.insert(format!("realm:{}", realm.key));
                for layer in &realm.layers {
                    self.collapsed
                        .insert(format!("layer:{}:{}", realm.key, layer.key));
                    for kind in &layer.kinds {
                        self.collapsed.insert(format!("kind:{}", kind.key));
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
                    for kind in &layer.kinds {
                        self.collapsed.insert(format!("kind:{}", kind.key));
                    }
                }
            }
        } else if let Some(rest) = key.strip_prefix("layer:") {
            // Layer key format: layer:{realm_key}:{layer_key}
            // Collapse all kinds in this layer
            if let Some((realm_key, layer_key)) = rest.split_once(':') {
                if let Some(realm) = self.realms.iter().find(|r| r.key == realm_key) {
                    if let Some(layer) = realm.layers.iter().find(|l| l.key == layer_key) {
                        for kind in &layer.kinds {
                            self.collapsed.insert(format!("kind:{}", kind.key));
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

    /// Set instances for a Kind (used in Data mode).
    /// Also stores the total count for "X of Y" display.
    /// Calculates missing_required_count for each instance based on Kind schema.
    #[allow(dead_code)]
    pub fn set_instances(
        &mut self,
        kind_key: &str,
        mut instances: Vec<InstanceInfo>,
        total: usize,
    ) {
        // Get schema info from Kind
        let (required_props, all_props) = self
            .find_kind(kind_key)
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

        self.instances.insert(kind_key.to_string(), instances);
        self.instance_totals.insert(kind_key.to_string(), total);
    }

    /// Get instances for a Kind.
    pub fn get_instances(&self, kind_key: &str) -> Option<&Vec<InstanceInfo>> {
        self.instances.get(kind_key)
    }

    /// Get total instance count for a Kind (may be > loaded instances).
    pub fn get_instance_total(&self, kind_key: &str) -> Option<usize> {
        self.instance_totals.get(kind_key).copied()
    }

    /// Total number of visible items for a specific mode.
    /// In Data mode (data_mode=true), includes instances under expanded Kinds.
    pub fn item_count_for_mode(&self, data_mode: bool) -> usize {
        let mut count = 0;

        // Kinds section
        count += 1; // "Kinds" header
        if !self.is_collapsed("kinds") {
            for realm in &self.realms {
                count += 1; // realm header
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        count += 1; // layer header
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            for kind in &layer.kinds {
                                count += 1; // kind

                                // In Data mode, add instances if not collapsed
                                if data_mode && !self.is_collapsed(&format!("kind:{}", kind.key)) {
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

        // Arcs section
        count += 1; // "Arcs" header
        if !self.is_collapsed("arcs") {
            for family in &self.arc_families {
                count += 1; // family header
                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    count += family.arc_kinds.len();
                }
            }
        }

        count
    }

    /// Get item at cursor position for a specific mode.
    /// In Data mode (data_mode=true), includes instances under expanded Kinds.
    pub fn item_at_for_mode(&self, cursor: usize, data_mode: bool) -> Option<TreeItem<'_>> {
        let mut idx = 0;

        // Kinds section header
        if idx == cursor {
            return Some(TreeItem::KindsSection);
        }
        idx += 1;

        if !self.is_collapsed("kinds") {
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
                            for kind in &layer.kinds {
                                if idx == cursor {
                                    return Some(TreeItem::Kind(realm, layer, kind));
                                }
                                idx += 1;

                                // In Data mode, check for instances
                                if data_mode && !self.is_collapsed(&format!("kind:{}", kind.key)) {
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
                    for arc_kind in &family.arc_kinds {
                        if idx == cursor {
                            return Some(TreeItem::ArcKind(family, arc_kind));
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

        // Kinds section
        count += 1; // "Kinds" header
        if !self.is_collapsed("kinds") {
            for realm in &self.realms {
                count += 1; // realm header
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        count += 1; // layer header
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            count += layer.kinds.len();
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
                    count += family.arc_kinds.len();
                }
            }
        }

        count
    }

    /// Get item at cursor position (respects collapsed state).
    pub fn item_at(&self, cursor: usize) -> Option<TreeItem<'_>> {
        let mut idx = 0;

        // Kinds section header
        if idx == cursor {
            return Some(TreeItem::KindsSection);
        }
        idx += 1;

        if !self.is_collapsed("kinds") {
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
                            for kind in &layer.kinds {
                                if idx == cursor {
                                    return Some(TreeItem::Kind(realm, layer, kind));
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
                    for arc_kind in &family.arc_kinds {
                        if idx == cursor {
                            return Some(TreeItem::ArcKind(family, arc_kind));
                        }
                        idx += 1;
                    }
                }
            }
        }

        None
    }

    /// Get the collapse key for an item at cursor position.
    pub fn collapse_key_at(&self, cursor: usize) -> Option<String> {
        match self.item_at(cursor) {
            Some(TreeItem::KindsSection) => Some("kinds".to_string()),
            Some(TreeItem::ArcsSection) => Some("arcs".to_string()),
            Some(TreeItem::Realm(r)) => Some(format!("realm:{}", r.key)),
            Some(TreeItem::Layer(r, l)) => Some(format!("layer:{}:{}", r.key, l.key)),
            Some(TreeItem::ArcFamily(f)) => Some(format!("family:{}", f.key)),
            // In Data mode, Kind can be collapsed to hide instances
            Some(TreeItem::Kind(_, _, k)) => Some(format!("kind:{}", k.key)),
            // Leaf nodes can't be collapsed
            Some(TreeItem::ArcKind(_, _)) | Some(TreeItem::Instance(_, _, _, _)) | None => None,
        }
    }

    /// Find the cursor position of the parent item.
    /// Returns None if at root or no parent exists.
    /// Hierarchy: Instance → Kind → Layer → Realm → KindsSection
    ///            ArcKind → ArcFamily → ArcsSection
    pub fn find_parent_cursor(&self, cursor: usize, data_mode: bool) -> Option<usize> {
        let current = if data_mode {
            self.item_at_for_mode(cursor, true)
        } else {
            self.item_at(cursor)
        };

        match current {
            // Section headers have no parent
            Some(TreeItem::KindsSection) | Some(TreeItem::ArcsSection) | None => None,

            // Realm's parent is KindsSection (always at index 0)
            Some(TreeItem::Realm(_)) => Some(0),

            // Layer's parent is its Realm
            Some(TreeItem::Layer(realm, _)) => self.find_realm_cursor(&realm.key),

            // Kind's parent is its Layer
            Some(TreeItem::Kind(realm, layer, _)) => self.find_layer_cursor(&realm.key, &layer.key),

            // Instance's parent is its Kind
            Some(TreeItem::Instance(realm, layer, kind, _)) => {
                self.find_kind_cursor_readonly(&realm.key, &layer.key, &kind.key, data_mode)
            }

            // ArcFamily's parent is ArcsSection
            Some(TreeItem::ArcFamily(_)) => self.find_arcs_section_cursor(),

            // ArcKind's parent is its ArcFamily
            Some(TreeItem::ArcKind(family, _)) => self.find_family_cursor(&family.key),
        }
    }

    /// Find cursor position of a Realm (does not modify collapse state).
    fn find_realm_cursor(&self, realm_key: &str) -> Option<usize> {
        if self.is_collapsed("kinds") {
            return None; // Realm not visible
        }
        let mut idx = 1; // Skip KindsSection
        for realm in &self.realms {
            if realm.key == realm_key {
                return Some(idx);
            }
            idx += 1;
            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1;
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        idx += layer.kinds.len();
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of a Layer (does not modify collapse state).
    fn find_layer_cursor(&self, realm_key: &str, layer_key: &str) -> Option<usize> {
        if self.is_collapsed("kinds") {
            return None;
        }
        let mut idx = 1; // Skip KindsSection
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
                        idx += layer.kinds.len();
                    }
                }
                return None;
            }
            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1;
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        idx += layer.kinds.len();
                    }
                }
            }
        }
        None
    }

    /// Find cursor position of a Kind (readonly, does not modify collapse state).
    fn find_kind_cursor_readonly(
        &self,
        realm_key: &str,
        layer_key: &str,
        kind_key: &str,
        data_mode: bool,
    ) -> Option<usize> {
        if self.is_collapsed("kinds") {
            return None;
        }
        let mut idx = 1; // Skip KindsSection
        for realm in &self.realms {
            idx += 1; // Realm
            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1; // Layer
                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        for kind in &layer.kinds {
                            if realm.key == realm_key
                                && layer.key == layer_key
                                && kind.key == kind_key
                            {
                                return Some(idx);
                            }
                            idx += 1;
                            // In data mode, count instances
                            if data_mode && !self.is_collapsed(&format!("kind:{}", kind.key)) {
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
        let mut idx = 1; // Skip KindsSection
        if !self.is_collapsed("kinds") {
            for realm in &self.realms {
                idx += 1;
                if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                    for layer in &realm.layers {
                        idx += 1;
                        if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                            idx += layer.kinds.len();
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
                idx += family.arc_kinds.len();
            }
        }
        None
    }

    // ========================================================================
    // Filtered Data mode: show only instances of a specific Kind
    // ========================================================================

    /// Get item count when filtered to a specific Kind (Data mode drill-down).
    /// Returns only instances of that Kind.
    pub fn filtered_item_count(&self, kind_key: &str) -> usize {
        self.instances.get(kind_key).map(|v| v.len()).unwrap_or(0)
    }

    /// Get item at cursor when filtered to a specific Kind.
    /// Returns Instance items only.
    pub fn filtered_item_at<'a>(&'a self, cursor: usize, kind_key: &str) -> Option<TreeItem<'a>> {
        // Find the Kind info for context
        let kind_info = self.find_kind(kind_key)?;
        let instances = self.instances.get(kind_key)?;
        let instance = instances.get(cursor)?;
        Some(TreeItem::Instance(
            kind_info.0,
            kind_info.1,
            kind_info.2,
            instance,
        ))
    }

    /// Find a Kind by key, returns (Realm, Layer, Kind) refs.
    /// O(1) lookup using cached index (built once on load).
    pub fn find_kind(&self, kind_key: &str) -> Option<(&RealmInfo, &LayerInfo, &KindInfo)> {
        let (r_idx, l_idx, k_idx) = self.kind_index.get(kind_key)?;
        let realm = self.realms.get(*r_idx)?;
        let layer = realm.layers.get(*l_idx)?;
        let kind = layer.kinds.get(*k_idx)?;
        Some((realm, layer, kind))
    }

    /// Find cursor position for a Kind in Meta mode tree view.
    /// Expands necessary parents (realm, layer) to make the Kind visible.
    /// Returns the cursor position if found.
    #[allow(dead_code)] // Prepared for search/navigation features
    pub fn find_kind_cursor(&mut self, kind_key: &str) -> Option<usize> {
        // First, find the Kind and its parents
        let mut target_realm_key = None;
        let mut target_layer_key = None;

        for realm in &self.realms {
            for layer in &realm.layers {
                for kind in &layer.kinds {
                    if kind.key == kind_key {
                        target_realm_key = Some(realm.key.clone());
                        target_layer_key = Some(layer.key.clone());
                        break;
                    }
                }
                if target_layer_key.is_some() {
                    break;
                }
            }
            if target_realm_key.is_some() {
                break;
            }
        }

        let (realm_key, layer_key) = match (target_realm_key, target_layer_key) {
            (Some(r), Some(l)) => (r, l),
            _ => return None,
        };

        // Expand parents to make Kind visible
        self.collapsed.remove("kinds");
        self.collapsed.remove(&format!("realm:{}", realm_key));
        self.collapsed
            .remove(&format!("layer:{}:{}", realm_key, layer_key));

        // Now count to find the cursor position
        let mut idx = 0;

        // Kinds section header
        idx += 1;

        for realm in &self.realms {
            idx += 1; // Realm

            if !self.is_collapsed(&format!("realm:{}", realm.key)) {
                for layer in &realm.layers {
                    idx += 1; // Layer

                    if !self.is_collapsed(&format!("layer:{}:{}", realm.key, layer.key)) {
                        for kind in &layer.kinds {
                            if kind.key == kind_key {
                                return Some(idx);
                            }
                            idx += 1;
                        }
                    }
                }
            }
        }

        None
    }
}

/// Item type at a tree position.
#[derive(Debug, Clone)]
pub enum TreeItem<'a> {
    // Section headers
    KindsSection,
    ArcsSection,
    // Kinds hierarchy
    Realm(&'a RealmInfo),
    Layer(&'a RealmInfo, &'a LayerInfo),
    Kind(&'a RealmInfo, &'a LayerInfo, &'a KindInfo),
    // Arcs hierarchy
    ArcFamily(&'a ArcFamilyInfo),
    ArcKind(&'a ArcFamilyInfo, &'a ArcKindInfo),
    // Data view: instances under Kinds
    Instance(&'a RealmInfo, &'a LayerInfo, &'a KindInfo, &'a InstanceInfo),
}

/// Get icon for realm (v10.6: 2 realms only - global + tenant).
/// Uses unicode symbols instead of emoji for terminal compatibility.
fn realm_icon(key: &str) -> &'static str {
    match key {
        "global" => "◉", // filled circle - universal/shared
        "tenant" => "◎", // circle with dot - scoped/owned
        _ => "○",        // empty circle - unknown
    }
}

/// Convert PascalCase to kebab-case (e.g., "BlockL10n" -> "block-l10n").
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

/// An instance of a Kind in the data graph.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceInfo {
    pub key: String,
    pub display_name: String,
    pub kind_key: String,
    /// Properties as JSON values (properly typed, not debug strings).
    pub properties: BTreeMap<String, JsonValue>,
    /// Outgoing arcs from this instance.
    pub outgoing_arcs: Vec<InstanceArc>,
    /// Incoming arcs to this instance.
    pub incoming_arcs: Vec<InstanceArc>,
    /// Count of missing required properties (for tree badge).
    pub missing_required_count: usize,
    /// Count of filled (non-null/non-empty) properties.
    pub filled_properties: usize,
    /// Total properties in schema for this Kind.
    pub total_properties: usize,
}

/// An actual arc connection from/to an instance.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InstanceArc {
    pub arc_type: String,
    pub target_key: String,
    pub target_kind: String,
    /// True if this arc exists, false if it's from schema but not yet created.
    pub exists: bool,
}

/// Comparison of schema arcs vs actual arcs for an instance.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ArcComparison {
    pub arc_type: String,
    pub target_kind: String,
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
                    .find(|a| a.arc_type == schema_arc.rel_type);

                comparisons.push(ArcComparison {
                    arc_type: schema_arc.rel_type.clone(),
                    target_kind: schema_arc.target_kind.clone(),
                    exists: actual.is_some(),
                    target_key: actual.map(|a| a.target_key.clone()),
                });
            }
        }

        comparisons
    }

    /// Convert instance properties to colorized JSON lines for display.
    /// Colors: keys=cyan, strings=green, numbers/bools=yellow, null=gray
    pub fn to_colored_json(&self) -> Vec<Line<'static>> {
        let mut lines: Vec<Line<'static>> = Vec::with_capacity(self.properties.len() + 2);

        // Opening brace
        lines.push(Line::from(Span::styled(
            "{".to_string(),
            Style::default().fg(Color::White),
        )));

        let prop_count = self.properties.len();
        for (i, (key, value)) in self.properties.iter().enumerate() {
            let comma = if i < prop_count - 1 { "," } else { "" };

            // Detect value type and colorize accordingly
            let (value_str, value_color) = Self::colorize_value(value);

            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(format!("\"{}\"", key), Style::default().fg(Color::Cyan)),
                Span::styled(": ".to_string(), Style::default().fg(Color::White)),
                Span::styled(value_str, Style::default().fg(value_color)),
                Span::styled(comma.to_string(), Style::default().fg(Color::White)),
            ]));
        }

        // Closing brace
        lines.push(Line::from(Span::styled(
            "}".to_string(),
            Style::default().fg(Color::White),
        )));

        lines
    }

    /// Determine color based on JSON value type.
    fn colorize_value(value: &JsonValue) -> (String, Color) {
        match value {
            JsonValue::Null => ("null".to_string(), Color::DarkGray),
            JsonValue::Bool(b) => (b.to_string(), Color::Yellow),
            JsonValue::Number(n) => (n.to_string(), Color::Yellow),
            JsonValue::String(s) => {
                // Check if it looks like a date/timestamp
                if s.len() > 10
                    && s.chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false)
                    && (s.contains('T') || s.chars().filter(|&c| c == '-').count() >= 2)
                {
                    (format!("\"{}\"", s), Color::Magenta)
                } else {
                    (format!("\"{}\"", s), Color::Green)
                }
            }
            JsonValue::Array(arr) => {
                // Format arrays compactly
                let items: Vec<String> = arr.iter().map(|v| Self::colorize_value(v).0).collect();
                (format!("[{}]", items.join(", ")), Color::Cyan)
            }
            JsonValue::Object(obj) => {
                // Format objects compactly on one line
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, Self::colorize_value(v).0))
                    .collect();
                (format!("{{{}}}", items.join(", ")), Color::White)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Helper functions for creating test data
    // ========================================================================

    fn create_test_kind(key: &str, display_name: &str) -> KindInfo {
        KindInfo {
            key: key.to_string(),
            display_name: display_name.to_string(),
            description: String::new(),
            icon: String::new(),
            trait_name: "invariant".to_string(),
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

    fn create_test_layer(key: &str, kinds: Vec<KindInfo>) -> LayerInfo {
        LayerInfo {
            key: key.to_string(),
            display_name: key.to_string(),
            color: "#ffffff".to_string(),
            kinds,
        }
    }

    fn create_test_realm(key: &str, layers: Vec<LayerInfo>) -> RealmInfo {
        RealmInfo {
            key: key.to_string(),
            display_name: key.to_string(),
            color: "#ffffff".to_string(),
            icon: "○",
            layers,
        }
    }

    fn create_test_tree() -> TaxonomyTree {
        let locale_kind = create_test_kind("Locale", "Locale");
        let page_kind = create_test_kind("Page", "Page");
        let entity_kind = create_test_kind("Entity", "Entity");

        let locale_knowledge = create_test_layer("locale-knowledge", vec![locale_kind]);
        let structure = create_test_layer("structure", vec![page_kind]);
        let semantic = create_test_layer("semantic", vec![entity_kind]);

        let global = create_test_realm("global", vec![locale_knowledge]);
        let tenant = create_test_realm("tenant", vec![structure, semantic]);

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
            kind_key: "Locale".to_string(),
            properties: BTreeMap::from([
                ("language".to_string(), JsonValue::String("fr".to_string())),
                ("region".to_string(), JsonValue::String("FR".to_string())),
            ]),
            outgoing_arcs: vec![],
            incoming_arcs: vec![],
            missing_required_count: 0,
            filled_properties: 0,
            total_properties: 0,
        };

        assert_eq!(instance.key, "fr-FR");
        assert_eq!(instance.kind_key, "Locale");
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
            kind_key: "Locale".to_string(),
            properties: BTreeMap::new(),
            outgoing_arcs: vec![InstanceArc {
                arc_type: "HAS_TERMS".to_string(),
                target_key: "fr-FR-terms".to_string(),
                target_kind: "TermSet".to_string(),
                exists: true,
            }],
            incoming_arcs: vec![],
            missing_required_count: 0,
            filled_properties: 0,
            total_properties: 0,
        };

        let schema_arcs = vec![
            ArcInfo {
                rel_type: "HAS_TERMS".to_string(),
                direction: ArcDirection::Outgoing,
                target_kind: "TermSet".to_string(),
            },
            ArcInfo {
                rel_type: "HAS_CULTURE".to_string(),
                direction: ArcDirection::Outgoing,
                target_kind: "CultureSet".to_string(),
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
        // In Meta mode: 1 (Kinds) + 1 (global) + 1 (locale-knowledge) + 1 (Locale)
        //              + 1 (tenant) + 1 (structure) + 1 (Page) + 1 (semantic) + 1 (Entity)
        //              + 1 (Arcs)
        // Total: 10
        assert_eq!(tree.item_count(), 10);
    }

    // NOTE: Data View tests (item_count_for_mode, item_at_for_mode, set_instances)
    // were removed as these methods were never implemented.
    // Data View feature is planned for v10.7+

    // ========================================================================
    // YAML path validation tests
    // ========================================================================

    #[test]
    fn test_yaml_path_fallback_rejects_unknown_realm() {
        // When realm is "unknown", fallback should return empty string
        // instead of generating invalid path like "node-kinds/unknown/layer/kind.yaml"
        let realm_key = "unknown";
        let layer_key = "structure";
        let kind_key = "Page";

        // Simulate the validation logic from TaxonomyTree::load
        let yaml_path = if realm_key == "unknown" || layer_key == "unknown" {
            String::new() // Invalid - can't compute path
        } else {
            format!(
                "packages/core/models/node-kinds/{}/{}/{}.yaml",
                realm_key,
                layer_key,
                super::to_kebab_case(kind_key)
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
        let realm_key = "tenant";
        let layer_key = "structure";
        let kind_key = "Page";

        let yaml_path = if realm_key == "unknown" || layer_key == "unknown" {
            String::new()
        } else {
            format!(
                "packages/core/models/node-kinds/{}/{}/{}.yaml",
                realm_key,
                layer_key,
                super::to_kebab_case(kind_key)
            )
        };

        assert_eq!(
            yaml_path,
            "packages/core/models/node-kinds/tenant/structure/page.yaml"
        );
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

        let tree = TaxonomyTree::load(&db).await.expect("Failed to load tree");

        // Basic sanity checks
        assert!(!tree.realms.is_empty(), "Should load realms from Neo4j");
        assert!(
            tree.realms.iter().any(|r| r.key == "global"),
            "Should have global realm"
        );
        assert!(
            tree.realms.iter().any(|r| r.key == "tenant"),
            "Should have tenant realm"
        );
    }

    #[tokio::test]
    #[ignore] // Requires running Neo4j
    async fn test_load_instances_integration() {
        let db = crate::db::Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword")
            .await
            .expect("Failed to connect to Neo4j");

        // Load instances for a Kind that should exist (Locale has seed data)
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
    async fn test_load_kind_arcs_integration() {
        let db = crate::db::Db::connect("bolt://localhost:7687", "neo4j", "novanetpassword")
            .await
            .expect("Failed to connect to Neo4j");

        // Load arcs for a Kind that should have relationships
        let result = TaxonomyTree::load_kind_arcs(&db, "Page").await;

        match result {
            Ok(arcs_data) => {
                // Page should have some outgoing arcs (HAS_BLOCK, etc.)
                // Even if empty, the call should succeed
                // (len() is usize, always >= 0, so just verify call succeeded)
                let _ = arcs_data.outgoing.len();
            }
            Err(e) => {
                panic!("load_kind_arcs failed: {}", e);
            }
        }
    }
}
