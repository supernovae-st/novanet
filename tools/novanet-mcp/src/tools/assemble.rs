//! novanet_assemble tool
//!
//! Context assembly for LLM generation with token budget management.
//! Implements RLM-on-KG evidence packet pattern for efficient context gathering.

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Context assembly strategy
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AssemblyStrategy {
    /// Breadth-first traversal from focus node
    #[default]
    Breadth,
    /// Depth-first traversal following ownership arcs
    Depth,
    /// Prioritize by relevance score
    Relevance,
    /// Custom traversal order via arc families
    Custom,
}

/// Parameters for novanet_assemble tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AssembleParams {
    /// Focus node key (starting point for assembly)
    pub focus_key: String,
    /// Target locale for localized content
    pub locale: String,
    /// Maximum token budget for assembled context
    #[serde(default)]
    pub token_budget: Option<usize>,
    /// Assembly strategy
    #[serde(default)]
    pub strategy: AssemblyStrategy,
    /// Include entity definitions
    #[serde(default)]
    pub include_entities: Option<bool>,
    /// Include locale knowledge (Terms, Expressions)
    #[serde(default)]
    pub include_knowledge: Option<bool>,
    /// Include page/block structure
    #[serde(default)]
    pub include_structure: Option<bool>,
    /// Arc families to follow during assembly
    #[serde(default)]
    pub arc_families: Option<Vec<String>>,
    /// Maximum traversal depth (default: 3)
    #[serde(default)]
    pub max_depth: Option<usize>,
}

/// An evidence packet in the assembled context
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct EvidencePacket {
    /// Source node key
    pub source_key: String,
    /// Source node kind
    pub source_kind: String,
    /// Evidence type (definition, content, knowledge, structure)
    pub evidence_type: String,
    /// Distance from focus node (hops)
    pub distance: usize,
    /// Relevance score (0.0 - 1.0)
    pub relevance: f64,
    /// Compressed content (~200 bytes target)
    pub content: String,
    /// Token count for this packet
    pub tokens: usize,
}

/// Locale context for generation
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct LocaleContext {
    /// Locale key (e.g., "fr-FR")
    pub locale_key: String,
    /// Language name
    pub language: String,
    /// Region/country
    pub region: Option<String>,
    /// Voice/tone guidelines
    pub voice: Option<String>,
    /// Formatting rules
    pub formatting: Option<serde_json::Value>,
}

/// Result from novanet_assemble tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AssembleResult {
    /// Focus node information
    pub focus: FocusNode,
    /// Assembled evidence packets
    pub evidence: Vec<EvidencePacket>,
    /// Locale context for generation
    pub locale_context: LocaleContext,
    /// Total tokens used
    pub total_tokens: usize,
    /// Token budget remaining
    pub budget_remaining: usize,
    /// Number of nodes visited
    pub nodes_visited: usize,
    /// Assembly was truncated due to budget
    pub truncated: bool,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Focus node details
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct FocusNode {
    /// Node key
    pub key: String,
    /// Node kind
    pub kind: String,
    /// Node name/title
    pub name: Option<String>,
    /// Node description
    pub description: Option<String>,
}

/// Execute the novanet_assemble tool
pub async fn execute(state: &State, params: AssembleParams) -> Result<AssembleResult> {
    let start = std::time::Instant::now();

    let token_budget = params.token_budget.unwrap_or(100_000);
    let max_depth = params.max_depth.unwrap_or(3).min(5);
    let include_entities = params.include_entities.unwrap_or(true);
    let include_knowledge = params.include_knowledge.unwrap_or(true);
    let include_structure = params.include_structure.unwrap_or(true);

    // Get focus node
    let focus_query = r#"
        MATCH (n {key: $key})
        RETURN n.key AS key, labels(n)[0] AS kind, n.name AS name, n.description AS description
    "#;

    let mut query_params = serde_json::Map::new();
    query_params.insert("key".to_string(), serde_json::json!(params.focus_key));
    query_params.insert("locale".to_string(), serde_json::json!(params.locale));

    let focus_rows = state
        .pool()
        .execute_query(focus_query, Some(query_params.clone()))
        .await?;

    let focus_row = focus_rows
        .first()
        .ok_or_else(|| crate::error::Error::not_found(&params.focus_key))?;

    let focus = FocusNode {
        key: focus_row["key"].as_str().unwrap_or_default().to_string(),
        kind: focus_row["kind"].as_str().unwrap_or_default().to_string(),
        name: focus_row["name"].as_str().map(|s| s.to_string()),
        description: focus_row["description"].as_str().map(|s| s.to_string()),
    };

    // Get locale context
    let locale_context = get_locale_context(state, &params.locale).await?;

    let mut evidence = Vec::new();
    let mut total_tokens = 0;
    let mut nodes_visited = 0;

    // Assemble entity definitions if requested
    if include_entities {
        let entity_evidence =
            assemble_entities(state, &params.focus_key, &params.locale, max_depth).await?;
        for packet in entity_evidence {
            if total_tokens + packet.tokens <= token_budget {
                total_tokens += packet.tokens;
                nodes_visited += 1;
                evidence.push(packet);
            }
        }
    }

    // Assemble locale knowledge if requested
    if include_knowledge {
        let knowledge_evidence =
            assemble_knowledge(state, &params.focus_key, &params.locale).await?;
        for packet in knowledge_evidence {
            if total_tokens + packet.tokens <= token_budget {
                total_tokens += packet.tokens;
                nodes_visited += 1;
                evidence.push(packet);
            }
        }
    }

    // Assemble structure if requested
    if include_structure {
        let structure_evidence = assemble_structure(state, &params.focus_key, max_depth).await?;
        for packet in structure_evidence {
            if total_tokens + packet.tokens <= token_budget {
                total_tokens += packet.tokens;
                nodes_visited += 1;
                evidence.push(packet);
            }
        }
    }

    // Sort by relevance
    evidence.sort_by(|a, b| {
        b.relevance
            .partial_cmp(&a.relevance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let truncated = total_tokens >= token_budget;

    Ok(AssembleResult {
        focus,
        evidence,
        locale_context,
        total_tokens,
        budget_remaining: token_budget.saturating_sub(total_tokens),
        nodes_visited,
        truncated,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Get locale context for generation
async fn get_locale_context(state: &State, locale_key: &str) -> Result<LocaleContext> {
    let query = r#"
        MATCH (l:Locale {key: $locale})
        OPTIONAL MATCH (l)-[:HAS_CULTURE]->(c:Culture)
        OPTIONAL MATCH (l)-[:HAS_STYLE]->(s:Style)
        RETURN l.key AS locale_key,
               l.language AS language,
               l.region AS region,
               s.voice AS voice,
               l.formatting AS formatting
    "#;

    let mut params = serde_json::Map::new();
    params.insert("locale".to_string(), serde_json::json!(locale_key));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    if let Some(row) = rows.first() {
        Ok(LocaleContext {
            locale_key: row["locale_key"].as_str().unwrap_or(locale_key).to_string(),
            language: row["language"].as_str().unwrap_or("unknown").to_string(),
            region: row["region"].as_str().map(|s| s.to_string()),
            voice: row["voice"].as_str().map(|s| s.to_string()),
            formatting: row.get("formatting").cloned(),
        })
    } else {
        // Return minimal context if locale not found
        Ok(LocaleContext {
            locale_key: locale_key.to_string(),
            language: "unknown".to_string(),
            region: None,
            voice: None,
            formatting: None,
        })
    }
}

/// Assemble entity definitions connected to focus node
async fn assemble_entities(
    state: &State,
    focus_key: &str,
    locale: &str,
    max_depth: usize,
) -> Result<Vec<EvidencePacket>> {
    let query = format!(
        r#"
        MATCH (focus {{key: $key}})
        MATCH path = (focus)-[:USES_ENTITY|REFERENCES|HAS_ENTITY*1..{max_depth}]->(e:Entity)
        OPTIONAL MATCH (e)-[:HAS_CONTENT]->(ec:EntityContent {{locale: $locale}})
        WITH e, ec, length(path) AS distance
        RETURN DISTINCT e.key AS key,
               'Entity' AS kind,
               COALESCE(ec.name, e.name) AS name,
               COALESCE(ec.description, e.definition) AS description,
               distance
        ORDER BY distance
        LIMIT 20
        "#,
        max_depth = max_depth
    );

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(&query, Some(params)).await?;

    let mut evidence = Vec::new();
    for row in rows {
        let content = compress_to_evidence(row["name"].as_str(), row["description"].as_str());
        let tokens = content.len().div_ceil(4); // Estimate

        evidence.push(EvidencePacket {
            source_key: row["key"].as_str().unwrap_or_default().to_string(),
            source_kind: row["kind"].as_str().unwrap_or("Entity").to_string(),
            evidence_type: "definition".to_string(),
            distance: row["distance"].as_u64().unwrap_or(1) as usize,
            relevance: 1.0 / (row["distance"].as_f64().unwrap_or(1.0) + 1.0),
            content,
            tokens,
        });
    }

    Ok(evidence)
}

/// Assemble locale knowledge (Terms, Expressions, Patterns)
async fn assemble_knowledge(
    state: &State,
    focus_key: &str,
    locale: &str,
) -> Result<Vec<EvidencePacket>> {
    let query = r#"
        MATCH (focus {key: $key})
        OPTIONAL MATCH (focus)-[:USES_TERM]->(t:Term)<-[:CONTAINS_TERM]-(ts:TermSet)<-[:HAS_TERMS]-(l:Locale {key: $locale})
        OPTIONAL MATCH (focus)-[:USES_EXPRESSION]->(ex:Expression)<-[:CONTAINS_EXPRESSION]-(es:ExpressionSet)<-[:HAS_EXPRESSIONS]-(l)
        WITH collect(DISTINCT {type: 'term', key: t.key, value: t.value, domain: t.domain}) AS terms,
             collect(DISTINCT {type: 'expression', key: ex.key, value: ex.value, register: ex.register}) AS expressions
        RETURN terms, expressions
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    let mut evidence = Vec::new();

    if let Some(row) = rows.first() {
        // Process terms
        if let Some(terms) = row["terms"].as_array() {
            for term in terms {
                if let (Some(key), Some(value)) = (term["key"].as_str(), term["value"].as_str()) {
                    let content = format!("{}: {}", key, value);
                    let tokens = content.len().div_ceil(4);

                    evidence.push(EvidencePacket {
                        source_key: key.to_string(),
                        source_kind: "Term".to_string(),
                        evidence_type: "knowledge".to_string(),
                        distance: 1,
                        relevance: 0.8,
                        content,
                        tokens,
                    });
                }
            }
        }

        // Process expressions
        if let Some(expressions) = row["expressions"].as_array() {
            for expr in expressions {
                if let (Some(key), Some(value)) = (expr["key"].as_str(), expr["value"].as_str()) {
                    let content = format!("{}: {}", key, value);
                    let tokens = content.len().div_ceil(4);

                    evidence.push(EvidencePacket {
                        source_key: key.to_string(),
                        source_kind: "Expression".to_string(),
                        evidence_type: "knowledge".to_string(),
                        distance: 1,
                        relevance: 0.7,
                        content,
                        tokens,
                    });
                }
            }
        }
    }

    Ok(evidence)
}

/// Assemble structural context (Pages, Blocks)
async fn assemble_structure(
    state: &State,
    focus_key: &str,
    max_depth: usize,
) -> Result<Vec<EvidencePacket>> {
    let query = format!(
        r#"
        MATCH (focus {{key: $key}})
        OPTIONAL MATCH path = (focus)-[:HAS_PAGE|HAS_BLOCK|BLOCK_OF*1..{max_depth}]->(n)
        WHERE n:Page OR n:Block
        WITH n, length(path) AS distance
        RETURN DISTINCT n.key AS key,
               labels(n)[0] AS kind,
               n.name AS name,
               n.slug AS slug,
               distance
        ORDER BY distance
        LIMIT 15
        "#,
        max_depth = max_depth
    );

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));

    let rows = state.pool().execute_query(&query, Some(params)).await?;

    let mut evidence = Vec::new();
    for row in rows {
        let name = row["name"].as_str().or(row["slug"].as_str());
        let content = compress_to_evidence(name, None);
        let tokens = content.len().div_ceil(4);

        evidence.push(EvidencePacket {
            source_key: row["key"].as_str().unwrap_or_default().to_string(),
            source_kind: row["kind"].as_str().unwrap_or("Unknown").to_string(),
            evidence_type: "structure".to_string(),
            distance: row["distance"].as_u64().unwrap_or(1) as usize,
            relevance: 0.6 / (row["distance"].as_f64().unwrap_or(1.0) + 1.0),
            content,
            tokens,
        });
    }

    Ok(evidence)
}

/// Compress content to ~200 byte evidence packet
fn compress_to_evidence(name: Option<&str>, description: Option<&str>) -> String {
    let mut content = String::with_capacity(200);

    if let Some(n) = name {
        content.push_str(n);
    }

    if let Some(d) = description {
        if !content.is_empty() {
            content.push_str(": ");
        }
        // Truncate description to fit ~200 bytes
        let remaining = 200_usize.saturating_sub(content.len()).saturating_sub(3);
        if d.len() > remaining {
            content.push_str(&d[..remaining]);
            content.push_str("...");
        } else {
            content.push_str(d);
        }
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_to_evidence() {
        let result = compress_to_evidence(Some("Test"), Some("A description"));
        assert_eq!(result, "Test: A description");

        let result = compress_to_evidence(Some("Name"), None);
        assert_eq!(result, "Name");

        let result = compress_to_evidence(None, Some("Description only"));
        assert_eq!(result, "Description only");
    }

    #[test]
    fn test_compress_truncates_long_description() {
        let long_desc = "A".repeat(300);
        let result = compress_to_evidence(Some("Title"), Some(&long_desc));
        assert!(result.len() <= 203); // 200 + "..."
        assert!(result.ends_with("..."));
    }
}
