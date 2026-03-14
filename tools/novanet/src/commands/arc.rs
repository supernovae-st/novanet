//! Arc CRUD commands: `novanet arc create/delete`.
//!
//! Creates and deletes arcs (relationships) between nodes in Neo4j. Validates
//! arc type against ArcClass schema nodes (v0.12.0: ArcKind→ArcClass, Meta→Schema).

use crate::db::Db;
use tracing::{info, warn};

/// Validate that a relationship type contains only safe characters for Cypher.
/// Relationship types must be UPPER_SNAKE_CASE (A-Z, 0-9, underscore).
fn validate_rel_type(rel_type: &str) -> crate::Result<()> {
    if rel_type.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "relationship type cannot be empty".to_string(),
        ));
    }
    if !rel_type
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(crate::NovaNetError::Validation(format!(
            "invalid relationship type '{rel_type}': must be UPPER_SNAKE_CASE"
        )));
    }
    Ok(())
}

/// Create a relationship between two nodes.
pub async fn run_create(
    db: &Db,
    from_key: &str,
    to_key: &str,
    rel_type: &str,
    props_json: &str,
) -> crate::Result<()> {
    validate_rel_type(rel_type)?;

    // Validate ArcClass exists in schema graph (v0.12.0: ArcKind→ArcClass)
    let ak_rows = db
        .execute_with_params(
            "MATCH (ak:ArcClass {key: $rel_type}) RETURN ak.key AS key",
            [("rel_type", rel_type)],
        )
        .await?;

    if ak_rows.is_empty() {
        return Err(crate::NovaNetError::Validation(format!(
            "ArcClass '{rel_type}' not found in schema. Use `novanet blueprint` to list available arc types."
        )));
    }

    // Parse optional relation properties
    let props: serde_json::Value = serde_json::from_str(props_json)
        .map_err(|e| crate::NovaNetError::Validation(format!("invalid JSON properties: {e}")))?;

    // Build SET fragment for relation properties
    let mut set_parts = Vec::new();
    let mut param_pairs: Vec<(String, String)> = Vec::new();

    if let Some(obj) = props.as_object() {
        for (key, value) in obj {
            let param_name = format!("rp_{key}");
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            set_parts.push(format!("r.`{key}` = ${param_name}"));
            param_pairs.push((param_name, value_str));
        }
    }

    let set_clause = if set_parts.is_empty() {
        String::new()
    } else {
        format!("\nSET {}", set_parts.join(", "))
    };

    // SAFETY: rel_type is validated by validate_rel_type() to contain only [A-Z0-9_].
    // Property keys use backtick-escaping; values are parameterized.
    let cypher = format!(
        "MATCH (from {{key: $from_key}}), (to {{key: $to_key}})\n\
         CREATE (from)-[r:{rel_type}]->(to){set_clause}\n\
         RETURN type(r) AS rel_type, from.key AS from_key, to.key AS to_key"
    );

    let mut q = neo4rs::query(&cypher);
    q = q.param("from_key", from_key);
    q = q.param("to_key", to_key);
    for (name, value) in &param_pairs {
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
        Some(_row) => {
            info!(from = %from_key, to = %to_key, rel_type = %rel_type, "created relationship");
        },
        None => {
            return Err(crate::NovaNetError::Validation(format!(
                "could not create relationship: one or both nodes not found \
                 (from='{from_key}', to='{to_key}')"
            )));
        },
    }

    Ok(())
}

/// Delete a specific relationship between two nodes.
pub async fn run_delete(
    db: &Db,
    from_key: &str,
    to_key: &str,
    rel_type: &str,
) -> crate::Result<()> {
    validate_rel_type(rel_type)?;

    // SAFETY: rel_type is validated by validate_rel_type() to contain only [A-Z0-9_].
    let cypher = format!(
        "MATCH (from {{key: $from_key}})-[r:{rel_type}]->(to {{key: $to_key}})\n\
         DELETE r\n\
         RETURN count(*) AS deleted"
    );

    let mut q = neo4rs::query(&cypher);
    q = q.param("from_key", from_key);
    q = q.param("to_key", to_key);

    let mut result = db
        .graph()
        .execute(q)
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?;

    let deleted: i64 = match result
        .next()
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })? {
        Some(row) => row.get("deleted").unwrap_or(0),
        None => 0,
    };

    if deleted == 0 {
        warn!(from = %from_key, to = %to_key, rel_type = %rel_type, "no relationship found to delete");
    } else {
        info!(from = %from_key, to = %to_key, rel_type = %rel_type, count = deleted, "deleted relationships");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rel_type_valid() {
        assert!(validate_rel_type("FOR_LOCALE").is_ok());
        assert!(validate_rel_type("HAS_BLOCK").is_ok());
        assert!(validate_rel_type("OF_CLASS").is_ok());
        assert!(validate_rel_type("SUPPORTS_LOCALE").is_ok());
    }

    #[test]
    fn validate_rel_type_invalid() {
        assert!(validate_rel_type("").is_err());
        assert!(validate_rel_type("for_locale").is_err()); // lowercase
        assert!(validate_rel_type("FOR LOCALE").is_err()); // space
        assert!(validate_rel_type("FOR;DROP").is_err()); // injection
    }
}
