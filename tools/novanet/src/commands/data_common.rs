//! Shared types and utilities for data management commands.
//!
//! Extracted from `data_backup` and `data_status` to eliminate duplication.
//! Both commands share constants, types, and Neo4j query logic.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// =============================================================================
// CONSTANTS
// =============================================================================

/// Default exportable classes (org realm data nodes).
pub const DEFAULT_EXPORT_CLASSES: &[&str] = &[
    "Entity",
    "EntityNative",
    "Page",
    "PageNative",
    "Block",
    "BlockNative",
];

/// Standard properties present on all nodes.
pub const STANDARD_FIELDS: &[&str] = &[
    "key",
    "display_name",
    "content",
    "llm_context",
    "node_class",
];

/// Timestamp fields (handled separately for ISO formatting).
pub const TIMESTAMP_FIELDS: &[&str] = &["created_at", "updated_at"];

/// Regex pattern for valid Neo4j labels (PascalCase).
pub const LABEL_PATTERN: &str = r"^[A-Z][A-Za-z0-9]*$";

/// JSON fields that need parsing from Neo4j string representation.
pub const JSON_FIELDS: &[&str] = &["denomination_forms", "provenance"];

// =============================================================================
// TYPES
// =============================================================================

/// A single exported node with ordered properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedNode(pub IndexMap<String, serde_json::Value>);

/// YAML document written per class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDocument {
    /// ISO 8601 timestamp of the backup.
    pub exported_at: String,
    /// Node class name.
    pub class: String,
    /// Optional project filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Optional locale filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Exported nodes.
    pub nodes: Vec<ExportedNode>,
}

// =============================================================================
// SHARED HELPERS
// =============================================================================

/// Additional fields to extract for specific classes.
pub fn extra_fields(class: &str) -> &'static [&'static str] {
    match class {
        "EntityNative" => &["entity_key", "locale", "denomination_forms", "provenance"],
        "PageNative" => &["page_key", "locale", "slug", "meta_title", "meta_description"],
        "BlockNative" => &["block_key", "locale", "block_type"],
        "Entity" => &["project_key"],
        "Page" => &["project_key"],
        "Block" => &["page_key", "block_type", "sort_order"],
        _ => &[],
    }
}

/// Collect all field names for a class (standard + timestamp + class-specific).
pub fn all_fields(class: &str) -> Vec<&'static str> {
    STANDARD_FIELDS
        .iter()
        .chain(TIMESTAMP_FIELDS.iter())
        .chain(extra_fields(class).iter())
        .copied()
        .collect()
}

/// Build the RETURN clause for a Cypher query from field names.
pub fn return_clause(fields: &[&str]) -> String {
    fields
        .iter()
        .map(|f| format!("n.{f} AS {f}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Validate class names against injection (PascalCase only).
pub fn validate_class_names(classes: &[String]) -> crate::Result<()> {
    let re = regex::Regex::new(LABEL_PATTERN).expect("valid regex");
    for class in classes {
        if !re.is_match(class) {
            return Err(crate::NovaNetError::Validation(format!(
                "Invalid class name '{class}': must be PascalCase (e.g. Entity, EntityNative)"
            )));
        }
    }
    Ok(())
}

/// Transform a Neo4j row into an ExportedNode, parsing JSON fields as structured data.
pub fn row_to_exported_node(
    row: &neo4rs::Row,
    fields: &[&str],
) -> ExportedNode {
    let mut map = IndexMap::new();

    for field in fields {
        // Try string first (most common)
        if let Ok(val) = row.get::<String>(field) {
            if !val.is_empty() {
                // Parse JSON fields (denomination_forms, provenance)
                if JSON_FIELDS.contains(field)
                    && (val.starts_with('{') || val.starts_with('['))
                {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&val) {
                        map.insert(field.to_string(), json);
                        continue;
                    }
                }
                map.insert(
                    field.to_string(),
                    serde_json::Value::String(val),
                );
            }
            continue;
        }

        // Try integer
        if let Ok(val) = row.get::<i64>(field) {
            map.insert(
                field.to_string(),
                serde_json::Value::Number(val.into()),
            );
            continue;
        }

        // Try boolean
        if let Ok(val) = row.get::<bool>(field) {
            map.insert(field.to_string(), serde_json::Value::Bool(val));
        }

        // Skip if not found (field doesn't exist on this node)
    }

    ExportedNode(map)
}

/// Execute a Cypher query and collect rows into ExportedNode structs.
///
/// This is the unified query function used by both backup and status commands.
/// It builds the MATCH/WHERE/RETURN query from class + optional filters, then
/// transforms rows using `row_to_exported_node`.
pub async fn query_nodes(
    db: &crate::db::Db,
    class: &str,
    project: Option<&str>,
    locale: Option<&str>,
    since: Option<&chrono::DateTime<chrono::Utc>>,
) -> crate::Result<Vec<ExportedNode>> {
    let fields = all_fields(class);
    let ret = return_clause(&fields);

    // Build WHERE conditions (parameterized where possible)
    let mut conditions = Vec::new();
    if project.is_some() {
        conditions.push("n.key STARTS WITH $project".to_string());
    }
    if locale.is_some() {
        conditions.push("n.key ENDS WITH $locale".to_string());
    }
    if since.is_some() {
        conditions.push("n.updated_at > datetime($since)".to_string());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Note: class name is validated via regex, safe for interpolation.
    // Neo4j doesn't support parameterized labels.
    let cypher = format!("MATCH (n:{class}) {where_clause} RETURN {ret} ORDER BY n.key");

    // Build params for neo4rs
    let mut q = neo4rs::query(&cypher);
    if let Some(p) = project {
        q = q.param("project", p);
    }
    if let Some(l) = locale {
        let locale_suffix = format!("@{l}");
        q = q.param("locale", locale_suffix.as_str());
    }
    if let Some(s) = since {
        q = q.param("since", s.to_rfc3339().as_str());
    }

    // Execute
    let mut result = db
        .graph()
        .execute(q)
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?;

    let mut nodes = Vec::new();
    while let Some(row) = result
        .next()
        .await
        .map_err(|e| crate::NovaNetError::Query {
            query: cypher.clone(),
            source: e,
        })?
    {
        nodes.push(row_to_exported_node(&row, &fields));
    }

    Ok(nodes)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extra_fields_entity_native() {
        let fields = extra_fields("EntityNative");
        assert!(fields.contains(&"entity_key"));
        assert!(fields.contains(&"locale"));
        assert!(fields.contains(&"denomination_forms"));
    }

    #[test]
    fn extra_fields_unknown_class() {
        let fields = extra_fields("CustomClass");
        assert!(fields.is_empty());
    }

    #[test]
    fn all_fields_includes_standard_and_extra() {
        let fields = all_fields("Entity");
        assert!(fields.contains(&"key"));
        assert!(fields.contains(&"display_name"));
        assert!(fields.contains(&"created_at"));
        assert!(fields.contains(&"project_key"));
    }

    #[test]
    fn return_clause_format() {
        let clause = return_clause(&["key", "display_name"]);
        assert_eq!(clause, "n.key AS key, n.display_name AS display_name");
    }

    #[test]
    fn validate_class_names_rejects_injection() {
        let names = vec!["Entity}DETACH DELETE n".to_string()];
        assert!(validate_class_names(&names).is_err());
    }

    #[test]
    fn validate_class_names_accepts_pascal_case() {
        let names = vec![
            "Entity".to_string(),
            "EntityNative".to_string(),
            "PageNative".to_string(),
        ];
        assert!(validate_class_names(&names).is_ok());
    }

    #[test]
    fn export_document_serializes_to_yaml() {
        let mut node = IndexMap::new();
        node.insert(
            "key".to_string(),
            serde_json::Value::String("qr-code".to_string()),
        );
        node.insert(
            "display_name".to_string(),
            serde_json::Value::String("QR Code".to_string()),
        );

        let doc = ExportDocument {
            exported_at: "2026-03-13T14:30:22Z".to_string(),
            class: "Entity".to_string(),
            project: Some("qrcode-ai".to_string()),
            locale: None,
            nodes: vec![ExportedNode(node)],
        };

        let yaml = serde_yaml::to_string(&doc).unwrap();
        assert!(yaml.contains("class: Entity"));
        assert!(yaml.contains("qr-code"));
        assert!(yaml.contains("QR Code"));
    }
}
