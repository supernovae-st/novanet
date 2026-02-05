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

// ============================================================================
// Neo4j Arc Data (live query)
// ============================================================================

/// A single arc relationship from Neo4j.
#[derive(Debug, Clone)]
pub struct Neo4jArc {
    pub arc_key: String,      // e.g., "FALLBACK_TO"
    pub other_kind: String,   // The Kind on the other end
    pub family: String,       // e.g., "localization", "ownership"
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
            instances: BTreeMap::new(),
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
    /// Returns instances with their properties and arcs.
    pub async fn load_instances(db: &Db, kind_label: &str) -> crate::Result<Vec<InstanceInfo>> {
        // Query instances of this Kind with their properties and arcs
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
LIMIT 100
"#,
            kind_label = kind_label
        );

        let rows = db.execute(&cypher).await?;
        let mut instances = Vec::new();

        for row in rows {
            let key: String = row.get("key").unwrap_or_default();
            let display_name: String = row.get("display_name").unwrap_or_default();

            // Parse properties as BTreeMap
            let props: BTreeMap<String, String> = row
                .get::<neo4rs::BoltMap>("props")
                .map(|m| {
                    m.value
                        .iter()
                        .map(|(k, v)| (k.to_string(), format!("{:?}", v)))
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
            });
        }

        Ok(instances)
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

    // ========================================================================
    // Data view: Instance methods
    // ========================================================================

    /// Set instances for a Kind (used in Data mode).
    /// Will be used when integrating Neo4j instance loading.
    #[allow(dead_code)]
    pub fn set_instances(&mut self, kind_key: &str, instances: Vec<InstanceInfo>) {
        self.instances.insert(kind_key.to_string(), instances);
    }

    /// Get instances for a Kind.
    pub fn get_instances(&self, kind_key: &str) -> Option<&Vec<InstanceInfo>> {
        self.instances.get(kind_key)
    }

    /// Clear all instances (when switching back to Meta mode).
    #[allow(dead_code)]
    pub fn clear_instances(&mut self) {
        self.instances.clear();
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
                        if !self.is_collapsed(&format!("layer:{}", layer.key)) {
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

                        if !self.is_collapsed(&format!("layer:{}", layer.key)) {
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
            // In Data mode, Kind can be collapsed to hide instances
            Some(TreeItem::Kind(_, _, k)) => Some(format!("kind:{}", k.key)),
            // Leaf nodes can't be collapsed
            Some(TreeItem::ArcKind(_, _)) | Some(TreeItem::Instance(_, _, _, _)) | None => None,
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
    // Data view: instances under Kinds
    Instance(&'a RealmInfo, &'a LayerInfo, &'a KindInfo, &'a InstanceInfo),
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
    /// Properties as JSON-like map (key -> value as string).
    pub properties: BTreeMap<String, String>,
    /// Outgoing arcs from this instance.
    pub outgoing_arcs: Vec<InstanceArc>,
    /// Incoming arcs to this instance.
    pub incoming_arcs: Vec<InstanceArc>,
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
        let mut comparisons = Vec::new();

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
            emoji: "📁",
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

        TaxonomyTree {
            realms: vec![global, tenant],
            arc_families: Vec::new(),
            stats: GraphStats::default(),
            collapsed: FxHashSet::default(),
            instances: BTreeMap::new(),
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
                ("language".to_string(), "fr".to_string()),
                ("region".to_string(), "FR".to_string()),
            ]),
            outgoing_arcs: vec![],
            incoming_arcs: vec![],
        };

        assert_eq!(instance.key, "fr-FR");
        assert_eq!(instance.kind_key, "Locale");
        assert_eq!(instance.properties.get("language"), Some(&"fr".to_string()));
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
        let has_terms = comparison.iter().find(|c| c.arc_type == "HAS_TERMS").unwrap();
        assert!(has_terms.exists);
        assert_eq!(has_terms.target_key, Some("fr-FR-terms".to_string()));

        // HAS_CULTURE should be missing
        let has_culture = comparison.iter().find(|c| c.arc_type == "HAS_CULTURE").unwrap();
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
}
