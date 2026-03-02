//! Export command: `novanet export`.
//!
//! Export subgraph data to various formats (Cypher, JSON, GraphML, CSV).
//! Supports filtering by labels, relationship types, and custom queries.

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::db::Db;
use crate::output::OutputFormat;

// =============================================================================
// TYPES
// =============================================================================

/// Export output format.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum ExportFormat {
    /// Cypher CREATE statements
    #[default]
    Cypher,
    /// JSON graph format
    Json,
    /// GraphML XML format
    Graphml,
    /// CSV (nodes + relationships sections)
    Csv,
}

/// Arguments for the export command.
#[derive(Debug, Clone, Parser)]
#[command(about = "Export subgraph to various formats")]
pub struct ExportArgs {
    /// Output format
    #[arg(short, long, default_value = "cypher")]
    pub format: ExportFormat,

    /// Output file path (stdout if not specified)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Filter by node labels (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub labels: Option<Vec<String>>,

    /// Filter by relationship types (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub relationships: Option<Vec<String>>,

    /// Custom Cypher query to select subgraph
    #[arg(long)]
    pub query: Option<String>,

    /// Include schema (constraints, indexes)
    #[arg(long, default_value = "false")]
    pub include_schema: bool,

    /// Maximum nodes to export
    #[arg(long, default_value = "10000")]
    pub limit: usize,
}

/// Graph data for export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<NodeData>,
    pub relationships: Vec<RelData>,
}

/// Node data for export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeData {
    pub id: i64,
    pub labels: Vec<String>,
    pub properties: serde_json::Value,
}

/// Relationship data for export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelData {
    pub id: i64,
    pub start_id: i64,
    pub end_id: i64,
    pub rel_type: String,
    pub properties: serde_json::Value,
}

// =============================================================================
// QUERY BUILDING
// =============================================================================

/// Build Cypher query based on export arguments.
pub fn build_export_query(args: &ExportArgs) -> String {
    // If a custom query is provided, use it directly
    if let Some(ref query) = args.query {
        return query.clone();
    }

    let mut query = String::with_capacity(256);

    // Build label filter
    let label_filter = args
        .labels
        .as_ref()
        .map(|labels| labels.join("|"))
        .unwrap_or_default();

    // Match nodes with optional label filter
    if label_filter.is_empty() {
        query.push_str("MATCH (n)\nWHERE NOT n:Schema\n");
    } else {
        query.push_str("MATCH (n)\nWHERE NOT n:Schema AND (");
        for (i, label) in args.labels.as_ref().unwrap().iter().enumerate() {
            if i > 0 {
                query.push_str(" OR ");
            }
            query.push_str(&format!("n:{}", label));
        }
        query.push_str(")\n");
    }

    // Add RETURN clause with limit
    query.push_str(&format!(
        "RETURN id(n) AS id, labels(n) AS labels, properties(n) AS props\n\
         ORDER BY id(n)\n\
         LIMIT {}",
        args.limit
    ));

    query
}

/// Build Cypher query for relationships based on export arguments.
pub fn build_relationships_query(args: &ExportArgs) -> Option<String> {
    // Only build relationships query if relationships filter is specified or we want all
    if args.query.is_some() {
        return None; // Custom query handles everything
    }

    let mut query = String::with_capacity(256);

    // Build label filter for nodes
    let label_filter = args
        .labels
        .as_ref()
        .map(|labels| labels.join("|"))
        .unwrap_or_default();

    // Build relationship type filter
    let rel_filter = args
        .relationships
        .as_ref()
        .map(|rels| rels.join("|"))
        .unwrap_or_default();

    // Match relationships
    if label_filter.is_empty() && rel_filter.is_empty() {
        query.push_str("MATCH (a)-[r]->(b)\nWHERE NOT a:Schema AND NOT b:Schema\n");
    } else if !label_filter.is_empty() && rel_filter.is_empty() {
        query.push_str("MATCH (a)-[r]->(b)\nWHERE NOT a:Schema AND NOT b:Schema AND (");
        for (i, label) in args.labels.as_ref().unwrap().iter().enumerate() {
            if i > 0 {
                query.push_str(" OR ");
            }
            query.push_str(&format!("a:{} OR b:{}", label, label));
        }
        query.push_str(")\n");
    } else if !rel_filter.is_empty() {
        query.push_str(&format!(
            "MATCH (a)-[r:{}]->(b)\nWHERE NOT a:Schema AND NOT b:Schema\n",
            rel_filter
        ));
    }

    query.push_str(&format!(
        "RETURN id(r) AS id, id(a) AS start_id, id(b) AS end_id, type(r) AS rel_type, properties(r) AS props\n\
         ORDER BY id(r)\n\
         LIMIT {}",
        args.limit
    ));

    Some(query)
}

// =============================================================================
// FORMATTING
// =============================================================================

/// Format graph data as Cypher CREATE statements.
pub fn format_cypher(graph: &GraphData, include_schema: bool) -> crate::Result<String> {
    let mut output = String::with_capacity(4096);

    if include_schema {
        output.push_str("// Schema\n");
        output.push_str("// (constraints and indexes would be included here)\n\n");
    }

    output.push_str("// Nodes\n");
    for node in &graph.nodes {
        let props = serde_json::to_string(&node.properties)
            .map_err(|e| crate::NovaNetError::Io(std::io::Error::other(e.to_string())))?;
        output.push_str(&format!(
            "CREATE (:{} {})\n",
            node.labels.join(":"),
            if props == "{}" || props == "null" {
                String::new()
            } else {
                props
            }
        ));
    }

    if !graph.relationships.is_empty() {
        output.push_str("\n// Relationships\n");
        for rel in &graph.relationships {
            let props = serde_json::to_string(&rel.properties)
                .map_err(|e| crate::NovaNetError::Io(std::io::Error::other(e.to_string())))?;
            let props_str = if props == "{}" || props == "null" {
                String::new()
            } else {
                format!(" {}", props)
            };
            output.push_str(&format!(
                "MATCH (a), (b) WHERE id(a) = {} AND id(b) = {} CREATE (a)-[:{}{}]->(b)\n",
                rel.start_id, rel.end_id, rel.rel_type, props_str
            ));
        }
    }

    Ok(output)
}

/// Format graph data as JSON.
pub fn format_json(graph: &GraphData) -> crate::Result<String> {
    serde_json::to_string_pretty(graph)
        .map_err(|e| crate::NovaNetError::Io(std::io::Error::other(e.to_string())))
}

/// Format graph data as GraphML XML.
pub fn format_graphml(graph: &GraphData) -> crate::Result<String> {
    let mut output = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<graphml xmlns="http://graphml.graphdrawing.org/xmlns">
  <graph id="G" edgedefault="directed">
"#,
    );

    for node in &graph.nodes {
        output.push_str(&format!(
            "    <node id=\"{}\"><data key=\"labels\">{}</data></node>\n",
            node.id,
            node.labels.join(",")
        ));
    }

    for rel in &graph.relationships {
        output.push_str(&format!(
            "    <edge source=\"{}\" target=\"{}\" label=\"{}\"/>\n",
            rel.start_id, rel.end_id, rel.rel_type
        ));
    }

    output.push_str("  </graph>\n</graphml>");
    Ok(output)
}

/// Format graph data as CSV.
pub fn format_csv(graph: &GraphData) -> crate::Result<String> {
    let mut output = String::from("# Nodes\nid,labels,properties\n");

    for node in &graph.nodes {
        let props = serde_json::to_string(&node.properties)
            .map_err(|e| crate::NovaNetError::Io(std::io::Error::other(e.to_string())))?;
        output.push_str(&format!(
            "{},\"{}\",\"{}\"\n",
            node.id,
            node.labels.join(";"),
            props.replace('"', "\"\"")
        ));
    }

    output.push_str("\n# Relationships\nid,start_id,end_id,type,properties\n");

    for rel in &graph.relationships {
        let props = serde_json::to_string(&rel.properties)
            .map_err(|e| crate::NovaNetError::Io(std::io::Error::other(e.to_string())))?;
        output.push_str(&format!(
            "{},{},{},{},\"{}\"\n",
            rel.id,
            rel.start_id,
            rel.end_id,
            rel.rel_type,
            props.replace('"', "\"\"")
        ));
    }

    Ok(output)
}

// =============================================================================
// EXECUTION
// =============================================================================

/// Run the export command.
pub async fn run_export(
    db: &Db,
    args: ExportArgs,
    _format: OutputFormat,
) -> crate::Result<()> {
    use crate::db::RowExt;

    // Build and execute node query
    let node_query = build_export_query(&args);
    eprintln!("Fetching nodes...");

    let node_rows = db.execute(&node_query).await?;
    let mut nodes: Vec<NodeData> = Vec::with_capacity(node_rows.len());

    for row in &node_rows {
        let id = row.int("id");
        let labels = row.vec_str("labels");
        let props: serde_json::Value = row
            .get("props")
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        nodes.push(NodeData {
            id,
            labels,
            properties: props,
        });
    }

    eprintln!("  {} nodes found", nodes.len());

    // Build and execute relationships query
    let mut relationships: Vec<RelData> = Vec::new();
    if let Some(rel_query) = build_relationships_query(&args) {
        eprintln!("Fetching relationships...");
        let rel_rows = db.execute(&rel_query).await?;

        for row in &rel_rows {
            let id = row.int("id");
            let start_id = row.int("start_id");
            let end_id = row.int("end_id");
            let rel_type = row.str("rel_type");
            let props: serde_json::Value = row
                .get("props")
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
            relationships.push(RelData {
                id,
                start_id,
                end_id,
                rel_type,
                properties: props,
            });
        }

        eprintln!("  {} relationships found", relationships.len());
    }

    let graph = GraphData {
        nodes,
        relationships,
    };

    // Format output
    let content = match args.format {
        ExportFormat::Cypher => format_cypher(&graph, args.include_schema)?,
        ExportFormat::Json => format_json(&graph)?,
        ExportFormat::Graphml => format_graphml(&graph)?,
        ExportFormat::Csv => format_csv(&graph)?,
    };

    // Write output
    if let Some(ref path) = args.output {
        std::fs::write(path, &content)?;
        eprintln!("Exported to {}", path.display());
    } else {
        println!("{}", content);
    }

    Ok(())
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_export_args_parse() {
        let args = ExportArgs::try_parse_from([
            "export",
            "--format",
            "cypher",
            "--output",
            "export.cypher",
            "--labels",
            "Entity,EntityNative",
        ])
        .unwrap();
        assert!(matches!(args.format, ExportFormat::Cypher));
        assert_eq!(
            args.labels,
            Some(vec!["Entity".to_string(), "EntityNative".to_string()])
        );
        assert_eq!(args.output, Some(PathBuf::from("export.cypher")));
    }

    #[test]
    fn test_export_args_defaults() {
        let args = ExportArgs::try_parse_from(["export"]).unwrap();
        assert!(matches!(args.format, ExportFormat::Cypher));
        assert!(args.output.is_none());
        assert_eq!(args.limit, 10000);
        assert!(!args.include_schema);
    }

    #[test]
    fn test_export_args_json_format() {
        let args = ExportArgs::try_parse_from(["export", "--format", "json"]).unwrap();
        assert!(matches!(args.format, ExportFormat::Json));
    }

    #[test]
    fn test_export_args_graphml_format() {
        let args = ExportArgs::try_parse_from(["export", "--format", "graphml"]).unwrap();
        assert!(matches!(args.format, ExportFormat::Graphml));
    }

    #[test]
    fn test_export_args_csv_format() {
        let args = ExportArgs::try_parse_from(["export", "--format", "csv"]).unwrap();
        assert!(matches!(args.format, ExportFormat::Csv));
    }

    #[test]
    fn test_export_args_with_relationships() {
        let args = ExportArgs::try_parse_from([
            "export",
            "--relationships",
            "HAS_NATIVE,FOR_LOCALE",
        ])
        .unwrap();
        assert_eq!(
            args.relationships,
            Some(vec!["HAS_NATIVE".to_string(), "FOR_LOCALE".to_string()])
        );
    }

    #[test]
    fn test_export_args_with_custom_query() {
        let args = ExportArgs::try_parse_from([
            "export",
            "--query",
            "MATCH (n:Entity) RETURN n LIMIT 10",
        ])
        .unwrap();
        assert_eq!(
            args.query,
            Some("MATCH (n:Entity) RETURN n LIMIT 10".to_string())
        );
    }

    #[test]
    fn test_export_args_with_limit() {
        let args = ExportArgs::try_parse_from(["export", "--limit", "500"]).unwrap();
        assert_eq!(args.limit, 500);
    }

    #[test]
    fn test_export_args_include_schema() {
        let args = ExportArgs::try_parse_from(["export", "--include-schema"]).unwrap();
        assert!(args.include_schema);
    }

    #[test]
    fn test_build_export_query_no_filters() {
        let args = ExportArgs {
            format: ExportFormat::Cypher,
            output: None,
            labels: None,
            relationships: None,
            query: None,
            include_schema: false,
            limit: 100,
        };
        let query = build_export_query(&args);
        assert!(query.contains("MATCH (n)"));
        assert!(query.contains("WHERE NOT n:Schema"));
        assert!(query.contains("LIMIT 100"));
    }

    #[test]
    fn test_build_export_query_with_labels() {
        let args = ExportArgs {
            format: ExportFormat::Cypher,
            output: None,
            labels: Some(vec!["Entity".to_string()]),
            relationships: None,
            query: None,
            include_schema: false,
            limit: 100,
        };
        let query = build_export_query(&args);
        assert!(query.contains("Entity"));
        assert!(query.contains("LIMIT 100"));
    }

    #[test]
    fn test_build_export_query_with_multiple_labels() {
        let args = ExportArgs {
            format: ExportFormat::Cypher,
            output: None,
            labels: Some(vec!["Entity".to_string(), "Page".to_string()]),
            relationships: None,
            query: None,
            include_schema: false,
            limit: 50,
        };
        let query = build_export_query(&args);
        assert!(query.contains("n:Entity"));
        assert!(query.contains("n:Page"));
        assert!(query.contains(" OR "));
        assert!(query.contains("LIMIT 50"));
    }

    #[test]
    fn test_build_export_query_with_custom_query() {
        let args = ExportArgs {
            format: ExportFormat::Cypher,
            output: None,
            labels: None,
            relationships: None,
            query: Some("MATCH (n:Custom) RETURN n".to_string()),
            include_schema: false,
            limit: 100,
        };
        let query = build_export_query(&args);
        assert_eq!(query, "MATCH (n:Custom) RETURN n");
    }

    #[test]
    fn test_build_relationships_query_no_filters() {
        let args = ExportArgs {
            format: ExportFormat::Cypher,
            output: None,
            labels: None,
            relationships: None,
            query: None,
            include_schema: false,
            limit: 100,
        };
        let query = build_relationships_query(&args).unwrap();
        assert!(query.contains("MATCH (a)-[r]->(b)"));
        assert!(query.contains("WHERE NOT a:Schema AND NOT b:Schema"));
        assert!(query.contains("LIMIT 100"));
    }

    #[test]
    fn test_build_relationships_query_with_rel_filter() {
        let args = ExportArgs {
            format: ExportFormat::Cypher,
            output: None,
            labels: None,
            relationships: Some(vec!["HAS_NATIVE".to_string(), "FOR_LOCALE".to_string()]),
            query: None,
            include_schema: false,
            limit: 100,
        };
        let query = build_relationships_query(&args).unwrap();
        assert!(query.contains("HAS_NATIVE|FOR_LOCALE"));
    }

    #[test]
    fn test_build_relationships_query_with_custom_query_returns_none() {
        let args = ExportArgs {
            format: ExportFormat::Cypher,
            output: None,
            labels: None,
            relationships: None,
            query: Some("MATCH (n) RETURN n".to_string()),
            include_schema: false,
            limit: 100,
        };
        assert!(build_relationships_query(&args).is_none());
    }

    #[test]
    fn test_format_cypher_empty_graph() {
        let graph = GraphData {
            nodes: vec![],
            relationships: vec![],
        };
        let output = format_cypher(&graph, false).unwrap();
        assert!(output.contains("// Nodes"));
        assert!(!output.contains("// Relationships"));
    }

    #[test]
    fn test_format_cypher_with_nodes() {
        let graph = GraphData {
            nodes: vec![NodeData {
                id: 1,
                labels: vec!["Entity".to_string()],
                properties: serde_json::json!({"key": "test-entity"}),
            }],
            relationships: vec![],
        };
        let output = format_cypher(&graph, false).unwrap();
        assert!(output.contains("CREATE (:Entity"));
        assert!(output.contains("test-entity"));
    }

    #[test]
    fn test_format_cypher_with_schema() {
        let graph = GraphData {
            nodes: vec![],
            relationships: vec![],
        };
        let output = format_cypher(&graph, true).unwrap();
        assert!(output.contains("// Schema"));
    }

    #[test]
    fn test_format_json() {
        let graph = GraphData {
            nodes: vec![NodeData {
                id: 1,
                labels: vec!["Entity".to_string()],
                properties: serde_json::json!({"key": "test"}),
            }],
            relationships: vec![],
        };
        let output = format_json(&graph).unwrap();
        assert!(output.contains("\"nodes\""));
        assert!(output.contains("\"relationships\""));
        assert!(output.contains("\"Entity\""));
    }

    #[test]
    fn test_format_graphml() {
        let graph = GraphData {
            nodes: vec![NodeData {
                id: 1,
                labels: vec!["Entity".to_string()],
                properties: serde_json::json!({}),
            }],
            relationships: vec![RelData {
                id: 10,
                start_id: 1,
                end_id: 2,
                rel_type: "HAS_NATIVE".to_string(),
                properties: serde_json::json!({}),
            }],
        };
        let output = format_graphml(&graph).unwrap();
        assert!(output.contains("<?xml version"));
        assert!(output.contains("<graphml"));
        assert!(output.contains("<node id=\"1\""));
        assert!(output.contains("Entity"));
        assert!(output.contains("<edge source=\"1\" target=\"2\""));
        assert!(output.contains("HAS_NATIVE"));
    }

    #[test]
    fn test_format_csv() {
        let graph = GraphData {
            nodes: vec![NodeData {
                id: 1,
                labels: vec!["Entity".to_string(), "Node".to_string()],
                properties: serde_json::json!({"key": "test"}),
            }],
            relationships: vec![RelData {
                id: 10,
                start_id: 1,
                end_id: 2,
                rel_type: "HAS_NATIVE".to_string(),
                properties: serde_json::json!({}),
            }],
        };
        let output = format_csv(&graph).unwrap();
        assert!(output.contains("# Nodes"));
        assert!(output.contains("id,labels,properties"));
        assert!(output.contains("1,\"Entity;Node\""));
        assert!(output.contains("# Relationships"));
        assert!(output.contains("10,1,2,HAS_NATIVE"));
    }

    #[test]
    fn test_graph_data_serialize() {
        let graph = GraphData {
            nodes: vec![NodeData {
                id: 1,
                labels: vec!["Test".to_string()],
                properties: serde_json::json!({}),
            }],
            relationships: vec![],
        };
        let json = serde_json::to_string(&graph).unwrap();
        assert!(json.contains("\"nodes\""));
        assert!(json.contains("\"relationships\""));
    }

    #[test]
    fn test_graph_data_deserialize() {
        let json = r#"{"nodes":[{"id":1,"labels":["Test"],"properties":{}}],"relationships":[]}"#;
        let graph: GraphData = serde_json::from_str(json).unwrap();
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].id, 1);
        assert_eq!(graph.nodes[0].labels, vec!["Test"]);
    }
}
