//! Data loading for TUI — Neo4j queries for taxonomy tree, stats, and detail.

use crate::db::Db;
use std::collections::{BTreeMap, HashSet};

/// Edge type for a Kind (from schema).
#[derive(Debug, Clone)]
pub struct EdgeInfo {
    pub rel_type: String,
    pub direction: EdgeDirection,
    pub target_kind: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirection {
    Outgoing, // →
    Incoming, // ←
}

/// An EdgeKind in the relations tree.
#[derive(Debug, Clone)]
pub struct EdgeKindInfo {
    pub key: String,
    pub display_name: String,
    pub from_kind: String,
    pub to_kind: String,
    pub cardinality: String,
    pub description: String,
}

/// EdgeFamily containing EdgeKinds.
#[derive(Debug, Clone)]
pub struct EdgeFamilyInfo {
    pub key: String,
    pub display_name: String,
    pub edge_kinds: Vec<EdgeKindInfo>,
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
    pub edges: Vec<EdgeInfo>,
    pub yaml_path: String,
    // Schema properties (from Neo4j Kind node)
    pub properties: Vec<String>,
    pub required_properties: Vec<String>,
    pub schema_hint: String,
    pub context_budget: String,
}

/// Layer containing Kinds.
#[derive(Debug, Clone)]
pub struct LayerInfo {
    pub key: String,
    pub display_name: String,
    pub kinds: Vec<KindInfo>,
}

/// Realm containing Layers.
#[derive(Debug, Clone)]
pub struct RealmInfo {
    pub key: String,
    pub display_name: String,
    pub emoji: &'static str,
    pub layers: Vec<LayerInfo>,
}

/// Stats for status bar.
#[derive(Debug, Clone, Default)]
pub struct GraphStats {
    pub node_count: i64,
    pub edge_count: i64,
    pub kind_count: i64,
    pub edge_kind_count: i64,
}

/// Full taxonomy tree: Realm > Layer > Kind + EdgeFamily > EdgeKind.
#[derive(Debug, Clone, Default)]
pub struct TaxonomyTree {
    pub realms: Vec<RealmInfo>,
    pub edge_families: Vec<EdgeFamilyInfo>,
    pub stats: GraphStats,
    /// Collapsed state: stores keys of collapsed nodes (e.g., "kinds", "relations", "realm:global", "layer:structure")
    pub collapsed: HashSet<String>,
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
    coalesce(l.key, 'unknown') AS layer_key,
    coalesce(l.display_name, l.key, 'Unknown') AS layer_display,
    instances,
    coalesce(k.yaml_path, '') AS yaml_path,
    coalesce(k.properties, []) AS properties,
    coalesce(k.required_properties, []) AS required_properties,
    coalesce(k.schema_hint, '') AS schema_hint,
    coalesce(k.context_budget, '') AS context_budget
ORDER BY realm_key, layer_key, kind_key
"#;

        let rows = db.execute(cypher).await?;

        // Group into tree structure: realm_key -> (realm_display, layer_key -> (layer_display, kinds))
        #[allow(clippy::type_complexity)]
        let mut realm_map: BTreeMap<String, (String, BTreeMap<String, (String, Vec<KindInfo>)>)> =
            BTreeMap::new();

        for row in rows {
            let kind_key: String = row.get("kind_key").unwrap_or_default();
            let kind_display: String = row.get("kind_display").unwrap_or_default();
            let kind_desc: String = row.get("kind_desc").unwrap_or_default();
            let kind_icon: String = row.get("kind_icon").unwrap_or_default();
            let trait_key: String = row.get("trait_key").unwrap_or_default();
            let realm_key: String = row.get("realm_key").unwrap_or_default();
            let realm_display: String = row.get("realm_display").unwrap_or_default();
            let layer_key: String = row.get("layer_key").unwrap_or_default();
            let layer_display: String = row.get("layer_display").unwrap_or_default();
            let instances: i64 = row.get("instances").unwrap_or(0);

            // Get YAML path from Neo4j (with fallback to computed path)
            let yaml_path_raw: String = row.get("yaml_path").unwrap_or_default();
            let yaml_path = if yaml_path_raw.is_empty() {
                // Fallback: compute path
                format!(
                    "packages/core/models/nodes/{}/{}/{}.yaml",
                    realm_key,
                    layer_key,
                    to_kebab_case(&kind_key)
                )
            } else {
                // Neo4j stores relative path like "nodes/project/structure/block.yaml"
                // We need to prefix with "packages/core/models/"
                format!("packages/core/models/{}", yaml_path_raw)
            };

            // Get schema properties from Neo4j
            let properties: Vec<String> = row.get("properties").unwrap_or_default();
            let required_properties: Vec<String> = row.get("required_properties").unwrap_or_default();
            let schema_hint: String = row.get("schema_hint").unwrap_or_default();
            let context_budget: String = row.get("context_budget").unwrap_or_default();

            let kind = KindInfo {
                key: kind_key,
                display_name: kind_display,
                description: kind_desc,
                icon: kind_icon,
                trait_name: trait_key,
                instance_count: instances,
                edges: Vec::new(), // Loaded separately
                yaml_path,
                properties,
                required_properties,
                schema_hint,
                context_budget,
            };

            realm_map
                .entry(realm_key)
                .or_insert_with(|| (realm_display, BTreeMap::new()))
                .1
                .entry(layer_key)
                .or_insert_with(|| (layer_display, Vec::new()))
                .1
                .push(kind);
        }

        // Convert to RealmInfo vec
        let realms: Vec<RealmInfo> = realm_map
            .into_iter()
            .map(|(realm_key, (realm_display, layers_map))| {
                let layers: Vec<LayerInfo> = layers_map
                    .into_iter()
                    .map(|(layer_key, (layer_display, kinds))| LayerInfo {
                        key: layer_key,
                        display_name: layer_display,
                        kinds,
                    })
                    .collect();

                RealmInfo {
                    emoji: realm_emoji(&realm_key),
                    key: realm_key,
                    display_name: realm_display,
                    layers,
                }
            })
            .collect();

        // Load stats
        let stats = Self::load_stats(db).await?;

        let mut tree = Self {
            realms,
            edge_families: Vec::new(),
            stats,
            collapsed: HashSet::new(),
        };

        // Load edges for all Kinds (optional - may fail if schema not seeded)
        let _ = tree.load_edges(db).await;

        // Load edge families (optional)
        let _ = tree.load_edge_families(db).await;

        Ok(tree)
    }

    /// Load EdgeKinds grouped by EdgeFamily.
    async fn load_edge_families(&mut self, db: &Db) -> crate::Result<()> {
        let cypher = r#"
MATCH (ek:EdgeKind:Meta)-[:IN_FAMILY]->(ef:EdgeFamily:Meta)
MATCH (ek)-[:FROM_KIND]->(fromKind:Kind:Meta)
MATCH (ek)-[:TO_KIND]->(toKind:Kind:Meta)
RETURN
    ef.key AS family_key,
    coalesce(ef.display_name, ef.key) AS family_display,
    ek.key AS edge_key,
    coalesce(ek.display_name, ek.key) AS edge_display,
    coalesce(ek.cardinality, '') AS cardinality,
    coalesce(ek.llm_context, '') AS edge_desc,
    fromKind.label AS from_kind,
    toKind.label AS to_kind
ORDER BY family_key, edge_key
"#;

        let rows = db.execute(cypher).await?;

        // Group by family
        let mut family_map: BTreeMap<String, (String, Vec<EdgeKindInfo>)> = BTreeMap::new();

        for row in rows {
            let family_key: String = row.get("family_key").unwrap_or_default();
            let family_display: String = row.get("family_display").unwrap_or_default();
            let edge_key: String = row.get("edge_key").unwrap_or_default();
            let edge_display: String = row.get("edge_display").unwrap_or_default();
            let cardinality: String = row.get("cardinality").unwrap_or_default();
            let edge_desc: String = row.get("edge_desc").unwrap_or_default();
            let from_kind: String = row.get("from_kind").unwrap_or_default();
            let to_kind: String = row.get("to_kind").unwrap_or_default();

            if family_key.is_empty() || edge_key.is_empty() {
                continue;
            }

            let edge_kind = EdgeKindInfo {
                key: edge_key,
                display_name: edge_display,
                from_kind,
                to_kind,
                cardinality,
                description: edge_desc,
            };

            family_map
                .entry(family_key)
                .or_insert_with(|| (family_display, Vec::new()))
                .1
                .push(edge_kind);
        }

        self.edge_families = family_map
            .into_iter()
            .map(|(key, (display_name, edge_kinds))| EdgeFamilyInfo {
                key,
                display_name,
                edge_kinds,
            })
            .collect();

        Ok(())
    }

    /// Load edge information for all Kinds from EdgeKind schema.
    async fn load_edges(&mut self, db: &Db) -> crate::Result<()> {
        // Query EdgeKind schema: what relationships are defined between Kinds
        // Note: Kind uses 'label' property, not 'key'
        let cypher = r#"
MATCH (ek:EdgeKind:Meta)-[:FROM_KIND]->(fromKind:Kind:Meta)
MATCH (ek)-[:TO_KIND]->(toKind:Kind:Meta)
RETURN fromKind.label AS kind_key, ek.key AS rel_type, 'outgoing' AS direction, toKind.label AS target_kind
ORDER BY fromKind.label, ek.key

UNION

MATCH (ek:EdgeKind:Meta)-[:FROM_KIND]->(fromKind:Kind:Meta)
MATCH (ek)-[:TO_KIND]->(toKind:Kind:Meta)
RETURN toKind.label AS kind_key, ek.key AS rel_type, 'incoming' AS direction, fromKind.label AS target_kind
ORDER BY toKind.label, ek.key
"#;

        let rows = db.execute(cypher).await?;

        // Build a map of kind_key -> edges
        let mut edge_map: BTreeMap<String, Vec<EdgeInfo>> = BTreeMap::new();

        for row in rows {
            let kind_key: String = row.get("kind_key").unwrap_or_default();
            let rel_type: String = row.get("rel_type").unwrap_or_default();
            let direction_str: String = row.get("direction").unwrap_or_default();
            let target_kind: String = row.get("target_kind").unwrap_or_default();

            if kind_key.is_empty() || rel_type.is_empty() {
                continue;
            }

            let direction = if direction_str == "incoming" {
                EdgeDirection::Incoming
            } else {
                EdgeDirection::Outgoing
            };

            edge_map.entry(kind_key).or_default().push(EdgeInfo {
                rel_type,
                direction,
                target_kind,
            });
        }

        // Update Kinds with edges
        for realm in &mut self.realms {
            for layer in &mut realm.layers {
                for kind in &mut layer.kinds {
                    if let Some(edges) = edge_map.remove(&kind.key) {
                        kind.edges = edges;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load graph statistics.
    async fn load_stats(db: &Db) -> crate::Result<GraphStats> {
        let cypher = r#"
MATCH (n) WHERE NOT n:Meta
WITH count(n) AS nodes
MATCH ()-[r]->() WHERE NOT startNode(r):Meta AND NOT endNode(r):Meta
WITH nodes, count(r) AS edges
MATCH (k:Kind:Meta)
WITH nodes, edges, count(k) AS kinds
MATCH (ek:EdgeKind:Meta)
RETURN nodes, edges, kinds, count(ek) AS edge_kinds
"#;

        let rows = db.execute(cypher).await?;
        if let Some(row) = rows.into_iter().next() {
            Ok(GraphStats {
                node_count: row.get("nodes").unwrap_or(0),
                edge_count: row.get("edges").unwrap_or(0),
                kind_count: row.get("kinds").unwrap_or(0),
                edge_kind_count: row.get("edge_kinds").unwrap_or(0),
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
    pub fn collapse(&mut self, key: &str) {
        self.collapsed.insert(key.to_string());
    }

    /// Expand a node.
    pub fn expand(&mut self, key: &str) {
        self.collapsed.remove(key);
    }

    /// Collapse all collapsible nodes.
    pub fn collapse_all(&mut self) {
        self.collapsed.insert("kinds".to_string());
        self.collapsed.insert("relations".to_string());
        for realm in &self.realms {
            self.collapsed.insert(format!("realm:{}", realm.key));
            for layer in &realm.layers {
                self.collapsed.insert(format!("layer:{}", layer.key));
            }
        }
        for family in &self.edge_families {
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

        // Relations section
        count += 1; // "Relations" header
        if !self.is_collapsed("relations") {
            for family in &self.edge_families {
                count += 1; // family header
                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    count += family.edge_kinds.len();
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

        // Relations section header
        if idx == cursor {
            return Some(TreeItem::RelationsSection);
        }
        idx += 1;

        if !self.is_collapsed("relations") {
            for family in &self.edge_families {
                if idx == cursor {
                    return Some(TreeItem::EdgeFamily(family));
                }
                idx += 1;

                if !self.is_collapsed(&format!("family:{}", family.key)) {
                    for edge_kind in &family.edge_kinds {
                        if idx == cursor {
                            return Some(TreeItem::EdgeKind(family, edge_kind));
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
            Some(TreeItem::RelationsSection) => Some("relations".to_string()),
            Some(TreeItem::Realm(r)) => Some(format!("realm:{}", r.key)),
            Some(TreeItem::Layer(_, l)) => Some(format!("layer:{}", l.key)),
            Some(TreeItem::EdgeFamily(f)) => Some(format!("family:{}", f.key)),
            // Leaf nodes can't be collapsed
            Some(TreeItem::Kind(_, _, _)) | Some(TreeItem::EdgeKind(_, _)) | None => None,
        }
    }
}

/// Item type at a tree position.
#[derive(Debug, Clone)]
pub enum TreeItem<'a> {
    // Section headers
    KindsSection,
    RelationsSection,
    // Kinds hierarchy
    Realm(&'a RealmInfo),
    Layer(&'a RealmInfo, &'a LayerInfo),
    Kind(&'a RealmInfo, &'a LayerInfo, &'a KindInfo),
    // Relations hierarchy
    EdgeFamily(&'a EdgeFamilyInfo),
    EdgeKind(&'a EdgeFamilyInfo, &'a EdgeKindInfo),
}

/// Get emoji for realm.
fn realm_emoji(key: &str) -> &'static str {
    match key {
        "global" => "🌍",
        "project" => "📦",
        "shared" => "🔗",
        _ => "📁",
    }
}

/// Convert PascalCase to kebab-case (e.g., "BlockL10n" -> "block-l10n").
fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
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
