//! Data loading for TUI — Neo4j queries for taxonomy tree, stats, and detail.

use crate::db::Db;
use rustc_hash::FxHashSet;
use std::collections::BTreeMap;
use tokio::join;

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
    pub emoji: &'static str,
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

/// Full taxonomy tree: Realm > Layer > Kind + ArcFamily > ArcKind.
#[derive(Debug, Clone, Default)]
pub struct TaxonomyTree {
    pub realms: Vec<RealmInfo>,
    pub arc_families: Vec<ArcFamilyInfo>,
    pub stats: GraphStats,
    /// Collapsed state: stores keys of collapsed nodes (e.g., "kinds", "arcs", "realm:global", "layer:structure")
    /// Uses FxHashSet for ~30% faster lookups on string keys.
    pub collapsed: FxHashSet<String>,
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
            let yaml_path = if yaml_path_raw.is_empty() {
                // Fallback: compute path
                format!(
                    "packages/core/models/node-kinds/{}/{}/{}.yaml",
                    realm_key,
                    layer_key,
                    to_kebab_case(&kind_key)
                )
            } else {
                // Neo4j stores relative path like "node-kinds/tenant/structure/block.yaml"
                // We need to prefix with "packages/core/models/"
                format!("packages/core/models/{}", yaml_path_raw)
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
                    emoji: realm_emoji(&realm_key),
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

        Ok(Self {
            realms,
            arc_families,
            stats,
            collapsed: FxHashSet::default(),
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

    /// Check if a node is collapsed.
    pub fn is_collapsed(&self, key: &str) -> bool {
        self.collapsed.contains(key)
    }

    /// Collapse a node.
    #[allow(dead_code)]
    pub fn collapse(&mut self, key: &str) {
        self.collapsed.insert(key.to_string());
    }

    /// Expand a node.
    #[allow(dead_code)]
    pub fn expand(&mut self, key: &str) {
        self.collapsed.remove(key);
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
                self.collapsed.insert(format!("layer:{}", layer.key));
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
                        if !self.is_collapsed(&format!("layer:{}", layer.key)) {
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

                        if !self.is_collapsed(&format!("layer:{}", layer.key)) {
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
            Some(TreeItem::Layer(_, l)) => Some(format!("layer:{}", l.key)),
            Some(TreeItem::ArcFamily(f)) => Some(format!("family:{}", f.key)),
            // Leaf nodes can't be collapsed
            Some(TreeItem::Kind(_, _, _)) | Some(TreeItem::ArcKind(_, _)) | None => None,
        }
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
}

/// Get emoji for realm (v10.6: 2 realms only - global + tenant).
fn realm_emoji(key: &str) -> &'static str {
    match key {
        "global" => "🌍",
        "tenant" => "🏢",
        _ => "📁",
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
