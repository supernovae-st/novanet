//! Node CRUD commands: `novanet node create/edit/delete`.
//!
//! Creates, edits, and deletes data nodes in Neo4j. Node creation validates
//! the Class against the schema-graph and auto-wires the `OF_CLASS` relationship.
//! (v0.12.0 ADR-023: Kind→Class, OF_KIND→OF_CLASS)

use crate::db::Db;
use tracing::{info, warn};

/// Validate that a label contains only safe characters for Cypher interpolation.
/// Labels must be alphanumeric (A-Z, a-z, 0-9, underscore).
fn validate_label(label: &str) -> crate::Result<()> {
    if label.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "label cannot be empty".to_string(),
        ));
    }
    if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(crate::NovaNetError::Validation(format!(
            "invalid label '{label}': must be alphanumeric/underscore only"
        )));
    }
    Ok(())
}

/// Parse a JSON string into a flat map of string key-value pairs for SET clauses.
fn parse_props_json(json: &str) -> crate::Result<serde_json::Value> {
    serde_json::from_str(json)
        .map_err(|e| crate::NovaNetError::Validation(format!("invalid JSON properties: {e}")))
}

/// Build a Cypher SET clause from a JSON object's keys.
/// Returns (SET fragment, param pairs) for parameterized execution.
fn build_set_fragment(props: &serde_json::Value, prefix: &str) -> (String, Vec<(String, String)>) {
    let mut parts = Vec::new();
    let mut params = Vec::new();

    if let Some(obj) = props.as_object() {
        for (key, value) in obj {
            let param_name = format!("prop_{key}");
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            parts.push(format!("{prefix}.`{key}` = ${param_name}"));
            params.push((param_name, value_str));
        }
    }

    (parts.join(", "), params)
}

/// Create a new node with the given Class, key, and properties.
/// Validates Class exists in schema-graph and auto-wires OF_CLASS.
pub async fn run_create(db: &Db, kind: &str, key: &str, props_json: &str) -> crate::Result<()> {
    validate_label(kind)?;

    // Validate Class exists in schema-graph (v0.12.0 ADR-023)
    let kind_rows = db
        .execute_with_params(
            "MATCH (c:Class {label: $kind}) RETURN c.label AS label",
            [("kind", kind)],
        )
        .await?;

    if kind_rows.is_empty() {
        return Err(crate::NovaNetError::Validation(format!(
            "Class '{kind}' not found in schema-graph. Use `novanet meta` to list available Classes."
        )));
    }

    let props = parse_props_json(props_json)?;
    let (set_fragment, params) = build_set_fragment(&props, "n");

    // Build Cypher: CREATE node with dynamic label, SET props, wire OF_CLASS
    let mut cypher = format!(
        "CREATE (n:{kind} {{key: $key}})\n\
         SET n.created_at = datetime(), n.updated_at = datetime()"
    );

    if !set_fragment.is_empty() {
        cypher.push_str(&format!(", {set_fragment}"));
    }

    cypher.push_str(
        "\nWITH n\n\
         MATCH (c:Class {label: $kind})\n\
         CREATE (n)-[:OF_CLASS]->(c)\n\
         RETURN n.key AS key, labels(n) AS labels",
    );

    // Build param list
    let mut q = neo4rs::query(&cypher);
    q = q.param("key", key);
    q = q.param("kind", kind);
    for (name, value) in &params {
        q = q.param(name.as_str(), value.as_str());
    }

    let mut result = db
        .graph()
        .execute(q)
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?;

    if let Some(row) = result
        .next()
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?
    {
        let created_key: String = row.get("key").map_err(|_| {
            crate::NovaNetError::Validation("Neo4j query result missing 'key' field".to_string())
        })?;
        info!(key = %created_key, kind = %kind, "created node");
    }

    Ok(())
}

/// Edit an existing node by merging properties.
pub async fn run_edit(db: &Db, key: &str, set_json: &str) -> crate::Result<()> {
    let props = parse_props_json(set_json)?;
    let (set_fragment, params) = build_set_fragment(&props, "n");

    if set_fragment.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "no properties to update (empty JSON object)".to_string(),
        ));
    }

    let cypher = format!(
        "MATCH (n {{key: $key}})\n\
         SET {set_fragment}, n.updated_at = datetime()\n\
         RETURN n.key AS key, labels(n) AS labels"
    );

    let mut q = neo4rs::query(&cypher);
    q = q.param("key", key);
    for (name, value) in &params {
        q = q.param(name.as_str(), value.as_str());
    }

    let mut result = db
        .graph()
        .execute(q)
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?;

    match result
        .next()
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })? {
        Some(row) => {
            let edited_key: String = row.get("key").map_err(|_| {
                crate::NovaNetError::Validation(
                    "Neo4j query result missing 'key' field".to_string(),
                )
            })?;
            info!(key = %edited_key, "updated node");
        },
        None => {
            return Err(crate::NovaNetError::Validation(format!(
                "node with key '{key}' not found"
            )));
        },
    }

    Ok(())
}

/// Delete a node and all its relationships.
pub async fn run_delete(db: &Db, key: &str, confirm: bool) -> crate::Result<()> {
    if !confirm {
        return Err(crate::NovaNetError::Validation(
            "destructive operation requires --confirm flag".to_string(),
        ));
    }

    let rows = db
        .execute_with_params(
            "MATCH (n {key: $key}) DETACH DELETE n RETURN count(*) AS deleted",
            [("key", key)],
        )
        .await?;

    let deleted: i64 = rows
        .first()
        .and_then(|r| r.get("deleted").ok())
        .unwrap_or(0);

    if deleted == 0 {
        warn!(key = %key, "no node found to delete");
    } else {
        info!(key = %key, "deleted node and all relationships");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_label_valid() {
        assert!(validate_label("Page").is_ok());
        assert!(validate_label("ConceptL10n").is_ok());
        assert!(validate_label("GEO_Mining_Run").is_ok());
    }

    #[test]
    fn validate_label_invalid() {
        assert!(validate_label("").is_err());
        assert!(validate_label("DROP CONSTRAINT").is_err());
        assert!(validate_label("Page;DROP").is_err());
        assert!(validate_label("n})//").is_err());
    }

    #[test]
    fn parse_props_valid() {
        let v = parse_props_json(r#"{"name":"test","count":"5"}"#).unwrap();
        assert_eq!(v["name"], "test");
    }

    #[test]
    fn parse_props_invalid() {
        assert!(parse_props_json("not json").is_err());
    }

    #[test]
    fn build_set_empty_object() {
        let v = serde_json::json!({});
        let (frag, params) = build_set_fragment(&v, "n");
        assert!(frag.is_empty());
        assert!(params.is_empty());
    }

    #[test]
    fn build_set_with_props() {
        let v = serde_json::json!({"display_name": "Test"});
        let (frag, params) = build_set_fragment(&v, "n");
        assert!(frag.contains("n.`display_name` = $prop_display_name"));
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].1, "Test");
    }
}
