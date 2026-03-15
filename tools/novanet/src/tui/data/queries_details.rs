//! Schema detail queries for the TUI.
//!
//! Loads detailed information for schema nodes: stats, class arcs,
//! arc class details, realm details, and layer details.

use crate::db::Db;

use super::types::*;
use super::TaxonomyTree;

impl TaxonomyTree {
    /// Load graph statistics.
    pub(super) async fn load_stats(db: &Db) -> crate::Result<GraphStats> {
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
                class_count: row.get("classes").unwrap_or(0),
                arc_class_count: row.get("arc_classes").unwrap_or(0),
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
    pub async fn load_arc_class_details(db: &Db, arc_key: &str) -> crate::Result<ArcClassDetails> {
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
       coalesce(ac.content, '') as description,
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

    /// Load Realm details from Neo4j (layers with class counts, total stats).
    pub async fn load_realm_details(db: &Db, realm_key: &str) -> crate::Result<RealmDetails> {
        // Query 1: Get realm info and totals
        let cypher_realm = r#"
MATCH (r:Realm {key: $realmKey})
OPTIONAL MATCH (r)-[:HAS_LAYER]->(l:Layer)<-[:IN_LAYER]-(c:Class)
OPTIONAL MATCH (c)<-[:OF_CLASS]-(n)
RETURN r.key as realm_key,
       coalesce(r.display_name, r.key) as display_name,
       coalesce(r.content, '') as description,
       count(DISTINCT c) as total_classes,
       count(DISTINCT n) as total_instances
"#;

        // Query 2: Get layers with their class counts (separate rows)
        let cypher_layers = r#"
MATCH (r:Realm {key: $realmKey})-[:HAS_LAYER]->(l:Layer)
OPTIONAL MATCH (l)<-[:IN_LAYER]-(c:Class)
WITH l, count(DISTINCT c) as class_count
ORDER BY l.order
RETURN l.key as layer_key,
       coalesce(l.display_name, l.key) as layer_display,
       class_count
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
                    class_count: lr.get::<i64>("class_count").unwrap_or(0) as usize,
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

    /// Load Layer details from Neo4j (classes, stats).
    /// Simplified - returns classes with instance counts.
    pub async fn load_layer_details(db: &Db, layer_key: &str) -> crate::Result<LayerDetails> {
        let cypher = r#"
MATCH (l:Layer {key: $layerKey})
OPTIONAL MATCH (r:Realm)-[:HAS_LAYER]->(l)
OPTIONAL MATCH (l)<-[:IN_LAYER]-(c:Class)
OPTIONAL MATCH (n) WHERE labels(n)[0] = c.label AND NOT n:Schema
WITH l, r, c, count(DISTINCT n) as inst_count
ORDER BY c.label
WITH l, r, collect(coalesce(c.display_name, c.label)) as class_names, count(c) as total_classes, sum(inst_count) as total_instances
RETURN l.key as layer_key,
       coalesce(l.display_name, l.key) as display_name,
       coalesce(l.content, '') as description,
       coalesce(r.key, '') as realm,
       class_names,
       total_classes,
       total_instances
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

            let class_names: Vec<String> = row.get("class_names").unwrap_or_default();

            Ok(LayerDetails {
                key,
                display_name,
                description,
                realm,
                class_names,
                total_classes: total_classes as usize,
                total_instances: total_instances as usize,
            })
        } else {
            Ok(LayerDetails::default())
        }
    }
}
