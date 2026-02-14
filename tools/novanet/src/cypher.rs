//! Cypher query builder for the 4 navigation modes.
//!
//! Produces `CypherStatement` values containing parameterized Cypher and
//! typed parameters. The caller decides whether to execute (Table/Json)
//! or display (Cypher format) the statement.

use crate::facets::FacetFilter;

/// A parameterized Cypher query ready for execution or display.
#[derive(Debug, Clone)]
pub struct CypherStatement {
    pub cypher: String,
    pub params: Vec<(String, ParamValue)>,
}

/// Parameter values that map to neo4rs BoltType.
#[derive(Debug, Clone)]
pub enum ParamValue {
    StringList(Vec<String>),
    Int(i64),
}

impl CypherStatement {
    /// Render the query with parameters inlined for copy-paste into Neo4j Browser.
    #[must_use]
    pub fn render_inline(&self) -> String {
        let mut output = self.cypher.clone();
        for (name, value) in &self.params {
            let placeholder = format!("${name}");
            let replacement = match value {
                ParamValue::StringList(list) => {
                    let items: Vec<String> = list
                        .iter()
                        .map(|s| format!("'{}'", s.replace('\'', "\\'")))
                        .collect();
                    format!("[{}]", items.join(", "))
                }
                ParamValue::Int(n) => n.to_string(),
            };
            output = output.replace(&placeholder, &replacement);
        }
        output
    }

    /// Get param by name (for passing to neo4rs).
    #[must_use]
    pub fn get_param(&self, name: &str) -> Option<&ParamValue> {
        self.params.iter().find(|(n, _)| n == name).map(|(_, v)| v)
    }
}

// ---------------------------------------------------------------------------
// Mode 1: Data — real instances only (no Meta nodes)
// ---------------------------------------------------------------------------

#[must_use]
pub fn data_query(limit: i64) -> CypherStatement {
    CypherStatement {
        cypher: "\
MATCH (n)
WHERE NOT n:Meta
WITH n, [l IN labels(n) WHERE l <> 'Meta'][0] AS label
RETURN label,
       n.key AS key,
       coalesce(n.display_name, '') AS display_name,
       coalesce(n.description, '') AS description
ORDER BY label, n.key
LIMIT $limit"
            .to_string(),
        params: vec![("limit".to_string(), ParamValue::Int(limit))],
    }
}

// ---------------------------------------------------------------------------
// Overlay — data + meta combined
// ---------------------------------------------------------------------------

#[must_use]
pub fn overlay_query(limit: i64) -> CypherStatement {
    CypherStatement {
        cypher: "\
MATCH (n)
WITH n,
     [l IN labels(n) WHERE l <> 'Meta'][0] AS label,
     n:Meta AS is_meta
RETURN label,
       n.key AS key,
       coalesce(n.display_name, '') AS display_name,
       coalesce(n.description, '') AS description,
       is_meta
ORDER BY is_meta DESC, label, n.key
LIMIT $limit"
            .to_string(),
        params: vec![("limit".to_string(), ParamValue::Int(limit))],
    }
}

// ---------------------------------------------------------------------------
// Mode 4: Faceted query — resolve classes via schema-graph, then fetch data nodes
// ---------------------------------------------------------------------------

/// Build a faceted Cypher query from a `FacetFilter`.
///
/// Strategy (v0.12.0 ADR-023):
/// 1. MATCH Class nodes satisfying all active facets (AND across axes, OR within axis)
/// 2. Collect resolved Class labels
/// 3. MATCH data nodes linked to those Classes via OF_CLASS
///
/// If no facets are active, falls back to data_query (all non-Schema nodes).
#[must_use]
pub fn faceted_query(filter: &FacetFilter, limit: i64) -> CypherStatement {
    if filter.is_empty() {
        return data_query(limit);
    }

    let mut where_clauses: Vec<String> = Vec::new();
    let mut params: Vec<(String, ParamValue)> = Vec::new();

    if !filter.realms.is_empty() {
        where_clauses
            .push("EXISTS { MATCH (c)-[:IN_REALM]->(r:Realm) WHERE r.key IN $realms }".to_string());
        params.push((
            "realms".to_string(),
            ParamValue::StringList(filter.realms.clone()),
        ));
    }

    if !filter.layers.is_empty() {
        where_clauses
            .push("EXISTS { MATCH (c)-[:IN_LAYER]->(l:Layer) WHERE l.key IN $layers }".to_string());
        params.push((
            "layers".to_string(),
            ParamValue::StringList(filter.layers.clone()),
        ));
    }

    if !filter.trait_filters.is_empty() {
        where_clauses.push(
            "EXISTS { MATCH (c)-[:HAS_TRAIT]->(t:Trait) WHERE t.key IN $traits }".to_string(),
        );
        params.push((
            "traits".to_string(),
            ParamValue::StringList(filter.trait_filters.clone()),
        ));
    }

    if !filter.kinds.is_empty() {
        where_clauses.push("c.label IN $kinds".to_string());
        params.push((
            "kinds".to_string(),
            ParamValue::StringList(filter.kinds.clone()),
        ));
    }

    let where_clause = where_clauses.join("\n  AND ");

    // Arc families filter arcs, not nodes — handled separately in output
    // (included in params for display but not in the WHERE clause for kind resolution)

    params.push(("limit".to_string(), ParamValue::Int(limit)));

    let cypher = format!(
        "\
MATCH (c:Class)
WHERE {where_clause}
WITH collect(c.label) AS resolved_classes
MATCH (n)-[:OF_CLASS]->(c2:Class)
WHERE c2.label IN resolved_classes
RETURN c2.label AS label,
       n.key AS key,
       coalesce(n.display_name, '') AS display_name,
       coalesce(n.description, '') AS description
ORDER BY label, n.key
LIMIT $limit"
    );

    CypherStatement { cypher, params }
}

/// Build a Cypher query that resolves Kind labels from facets without fetching data nodes.
/// Used by `novanet filter build` to output the resolved Cypher for Studio.
#[must_use]
pub fn filter_build_query(filter: &FacetFilter) -> CypherStatement {
    if filter.is_empty() {
        return CypherStatement {
            cypher: "MATCH (n) WHERE NOT n:Meta RETURN n".to_string(),
            params: vec![],
        };
    }

    // Reuse faceted_query but without limit and with RETURN n for Studio
    let mut where_clauses: Vec<String> = Vec::new();
    let mut params: Vec<(String, ParamValue)> = Vec::new();

    if !filter.realms.is_empty() {
        where_clauses
            .push("EXISTS { MATCH (c)-[:IN_REALM]->(r:Realm) WHERE r.key IN $realms }".to_string());
        params.push((
            "realms".to_string(),
            ParamValue::StringList(filter.realms.clone()),
        ));
    }

    if !filter.layers.is_empty() {
        where_clauses
            .push("EXISTS { MATCH (c)-[:IN_LAYER]->(l:Layer) WHERE l.key IN $layers }".to_string());
        params.push((
            "layers".to_string(),
            ParamValue::StringList(filter.layers.clone()),
        ));
    }

    if !filter.trait_filters.is_empty() {
        where_clauses.push(
            "EXISTS { MATCH (c)-[:HAS_TRAIT]->(t:Trait) WHERE t.key IN $traits }".to_string(),
        );
        params.push((
            "traits".to_string(),
            ParamValue::StringList(filter.trait_filters.clone()),
        ));
    }

    if !filter.kinds.is_empty() {
        where_clauses.push("c.label IN $kinds".to_string());
        params.push((
            "kinds".to_string(),
            ParamValue::StringList(filter.kinds.clone()),
        ));
    }

    let where_clause = where_clauses.join("\n  AND ");

    let cypher = format!(
        "\
MATCH (c:Class)
WHERE {where_clause}
WITH collect(c.label) AS resolved_classes
MATCH (n)-[:OF_CLASS]->(c2:Class)
WHERE c2.label IN resolved_classes
RETURN n, c2.label AS class_label"
    );

    CypherStatement { cypher, params }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_query_has_limit() {
        let stmt = data_query(100);
        assert!(stmt.cypher.contains("LIMIT $limit"));
        assert!(stmt.cypher.contains("NOT n:Meta"));
        assert!(matches!(
            stmt.get_param("limit"),
            Some(ParamValue::Int(100))
        ));
    }

    #[test]
    fn overlay_query_includes_is_meta() {
        let stmt = overlay_query(500);
        assert!(stmt.cypher.contains("is_meta"));
        assert!(stmt.cypher.contains("LIMIT $limit"));
    }

    #[test]
    fn faceted_query_empty_filter_falls_back() {
        let filter = FacetFilter::default();
        let stmt = faceted_query(&filter, 100);
        // Empty filter falls back to data_query
        assert!(stmt.cypher.contains("NOT n:Meta"));
    }

    #[test]
    fn faceted_query_single_realm() {
        let filter = FacetFilter {
            realms: vec!["shared".to_string()],
            ..Default::default()
        };
        let stmt = faceted_query(&filter, 200);
        assert!(stmt.cypher.contains("IN_REALM"));
        assert!(stmt.cypher.contains("$realms"));
        assert!(stmt.cypher.contains("resolved_classes"));
        assert!(stmt.cypher.contains("OF_CLASS"));
    }

    #[test]
    fn faceted_query_multiple_facets() {
        let filter = FacetFilter {
            realms: vec!["shared".to_string()],
            layers: vec!["knowledge".to_string()],
            trait_filters: vec!["defined".to_string()],
            ..Default::default()
        };
        let stmt = faceted_query(&filter, 100);
        assert!(stmt.cypher.contains("IN_REALM"));
        assert!(stmt.cypher.contains("IN_LAYER"));
        assert!(stmt.cypher.contains("HAS_TRAIT"));
        assert!(stmt.cypher.contains("AND"));
        // 4 params: realms, layers, traits, limit
        assert_eq!(stmt.params.len(), 4);
    }

    #[test]
    fn faceted_query_kinds_only() {
        let filter = FacetFilter {
            kinds: vec!["Locale".to_string(), "Expression".to_string()],
            ..Default::default()
        };
        let stmt = faceted_query(&filter, 50);
        assert!(stmt.cypher.contains("c.label IN $kinds"));
        assert!(!stmt.cypher.contains("IN_REALM"));
    }

    #[test]
    fn render_inline_substitutes_params() {
        let stmt = CypherStatement {
            cypher: "MATCH (n) WHERE n.key IN $keys LIMIT $limit".to_string(),
            params: vec![
                (
                    "keys".to_string(),
                    ParamValue::StringList(vec!["a".to_string(), "b".to_string()]),
                ),
                ("limit".to_string(), ParamValue::Int(10)),
            ],
        };
        let rendered = stmt.render_inline();
        assert!(rendered.contains("['a', 'b']"));
        assert!(rendered.contains("LIMIT 10"));
        assert!(!rendered.contains("$keys"));
        assert!(!rendered.contains("$limit"));
    }

    #[test]
    fn render_inline_escapes_quotes() {
        let stmt = CypherStatement {
            cypher: "WHERE n.key IN $vals".to_string(),
            params: vec![(
                "vals".to_string(),
                ParamValue::StringList(vec!["it's".to_string()]),
            )],
        };
        let rendered = stmt.render_inline();
        assert!(rendered.contains("it\\'s"));
    }

    #[test]
    fn filter_build_query_empty() {
        let stmt = filter_build_query(&FacetFilter::default());
        assert!(stmt.cypher.contains("NOT n:Meta"));
        assert!(stmt.cypher.contains("RETURN n"));
    }

    #[test]
    fn filter_build_query_with_facets() {
        let filter = FacetFilter {
            realms: vec!["org".to_string()],
            layers: vec!["structure".to_string()],
            ..Default::default()
        };
        let stmt = filter_build_query(&filter);
        assert!(stmt.cypher.contains("RETURN n, c2.label AS class_label"));
        assert!(stmt.cypher.contains("IN_REALM"));
        assert!(stmt.cypher.contains("IN_LAYER"));
    }

    #[test]
    fn faceted_query_all_axes_active() {
        let filter = FacetFilter {
            realms: vec!["shared".to_string(), "org".to_string()],
            layers: vec!["knowledge".to_string(), "structure".to_string()],
            trait_filters: vec!["defined".to_string(), "authored".to_string()],
            kinds: vec!["Locale".to_string()],
            arc_families: vec!["taxonomy".to_string()],
        };
        let stmt = faceted_query(&filter, 100);
        assert!(stmt.cypher.contains("IN_REALM"));
        assert!(stmt.cypher.contains("IN_LAYER"));
        assert!(stmt.cypher.contains("HAS_TRAIT"));
        assert!(stmt.cypher.contains("c.label IN $kinds"));
        // 5 params: realms, layers, traits, kinds, limit
        assert_eq!(stmt.params.len(), 5);
    }

    #[test]
    fn render_inline_empty_string_list() {
        let stmt = CypherStatement {
            cypher: "WHERE n.key IN $keys".to_string(),
            params: vec![("keys".to_string(), ParamValue::StringList(vec![]))],
        };
        let rendered = stmt.render_inline();
        assert!(rendered.contains("[]"));
    }

    #[test]
    fn render_inline_no_params() {
        let stmt = CypherStatement {
            cypher: "MATCH (n) RETURN n".to_string(),
            params: vec![],
        };
        let rendered = stmt.render_inline();
        assert_eq!(rendered, "MATCH (n) RETURN n");
    }

    #[test]
    fn get_param_missing_key() {
        let stmt = CypherStatement {
            cypher: "test".to_string(),
            params: vec![("limit".to_string(), ParamValue::Int(10))],
        };
        assert!(stmt.get_param("nonexistent").is_none());
        assert!(stmt.get_param("limit").is_some());
    }

    #[test]
    fn data_query_limit_zero() {
        let stmt = data_query(0);
        assert!(matches!(stmt.get_param("limit"), Some(ParamValue::Int(0))));
    }

    #[test]
    fn overlay_query_sort_order() {
        let stmt = overlay_query(100);
        assert!(stmt.cypher.contains("ORDER BY is_meta DESC, label, n.key"));
    }
}
