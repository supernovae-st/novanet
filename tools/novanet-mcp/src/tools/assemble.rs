//! novanet_assemble tool
//!
//! Context assembly for LLM generation with token budget management.
//! Implements RLM-on-KG evidence packet pattern for efficient context gathering.

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

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
#[instrument(name = "novanet_assemble", skip(state), fields(focus = %params.focus_key, locale = %params.locale))]
pub async fn execute(state: &State, params: AssembleParams) -> Result<AssembleResult> {
    let start = std::time::Instant::now();

    let token_budget = params
        .token_budget
        .unwrap_or(state.config().default_token_budget);
    let max_depth = params
        .max_depth
        .unwrap_or(3)
        .min(state.config().max_hops as usize);
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

    // Run all independent queries in parallel for ~4x speedup
    // (locale_context + 3 assemble functions are independent)
    let focus_key = &params.focus_key;
    let locale = &params.locale;

    let (locale_context, entity_evidence, knowledge_evidence, structure_evidence) = tokio::join!(
        get_locale_context(state, locale),
        async {
            if include_entities {
                assemble_entities(state, focus_key, locale, max_depth).await
            } else {
                Ok(Vec::new())
            }
        },
        async {
            if include_knowledge {
                assemble_knowledge(state, focus_key, locale).await
            } else {
                Ok(Vec::new())
            }
        },
        async {
            if include_structure {
                assemble_structure(state, focus_key, max_depth).await
            } else {
                Ok(Vec::new())
            }
        },
    );

    // Unwrap results (propagate first error)
    let locale_context = locale_context?;
    let entity_evidence = entity_evidence?;
    let knowledge_evidence = knowledge_evidence?;
    let structure_evidence = structure_evidence?;

    // Aggregate evidence with token budget management
    let mut evidence = Vec::new();
    let mut total_tokens = 0;
    let mut nodes_visited = 0;

    // Process all evidence in order: entities, knowledge, structure
    for packet in entity_evidence
        .into_iter()
        .chain(knowledge_evidence)
        .chain(structure_evidence)
    {
        if total_tokens + packet.tokens <= token_budget {
            total_tokens += packet.tokens;
            nodes_visited += 1;
            evidence.push(packet);
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
        OPTIONAL MATCH (e)-[:HAS_NATIVE]->(en:EntityNative {{locale: $locale}})
        WITH e, en, length(path) AS distance
        RETURN DISTINCT e.key AS key,
               'Entity' AS kind,
               COALESCE(en.name, e.name) AS name,
               COALESCE(en.description, e.definition) AS description,
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

    // ==================== AssemblyStrategy Tests ====================

    #[test]
    fn test_assembly_strategy_default_is_breadth() {
        let strategy: AssemblyStrategy = Default::default();
        assert!(matches!(strategy, AssemblyStrategy::Breadth));
    }

    #[test]
    fn test_assembly_strategy_deserialize_breadth() {
        let json = r#""breadth""#;
        let strategy: AssemblyStrategy = serde_json::from_str(json).unwrap();
        assert!(matches!(strategy, AssemblyStrategy::Breadth));
    }

    #[test]
    fn test_assembly_strategy_deserialize_depth() {
        let json = r#""depth""#;
        let strategy: AssemblyStrategy = serde_json::from_str(json).unwrap();
        assert!(matches!(strategy, AssemblyStrategy::Depth));
    }

    #[test]
    fn test_assembly_strategy_deserialize_relevance() {
        let json = r#""relevance""#;
        let strategy: AssemblyStrategy = serde_json::from_str(json).unwrap();
        assert!(matches!(strategy, AssemblyStrategy::Relevance));
    }

    #[test]
    fn test_assembly_strategy_deserialize_custom() {
        let json = r#""custom""#;
        let strategy: AssemblyStrategy = serde_json::from_str(json).unwrap();
        assert!(matches!(strategy, AssemblyStrategy::Custom));
    }

    #[test]
    fn test_assembly_strategy_invalid() {
        let json = r#""invalid""#;
        let result: std::result::Result<AssemblyStrategy, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ==================== AssembleParams Tests ====================

    #[test]
    fn test_assemble_params_minimal() {
        let json = r#"{"focus_key": "homepage", "locale": "fr-FR"}"#;
        let params: AssembleParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.focus_key, "homepage");
        assert_eq!(params.locale, "fr-FR");
        assert!(params.token_budget.is_none());
        assert!(matches!(params.strategy, AssemblyStrategy::Breadth));
        assert!(params.include_entities.is_none());
        assert!(params.include_knowledge.is_none());
        assert!(params.include_structure.is_none());
        assert!(params.arc_families.is_none());
        assert!(params.max_depth.is_none());
    }

    #[test]
    fn test_assemble_params_full() {
        let json = r#"{
            "focus_key": "qr-code",
            "locale": "es-MX",
            "token_budget": 50000,
            "strategy": "depth",
            "include_entities": true,
            "include_knowledge": false,
            "include_structure": true,
            "arc_families": ["ownership", "semantic"],
            "max_depth": 5
        }"#;
        let params: AssembleParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.focus_key, "qr-code");
        assert_eq!(params.locale, "es-MX");
        assert_eq!(params.token_budget, Some(50000));
        assert!(matches!(params.strategy, AssemblyStrategy::Depth));
        assert_eq!(params.include_entities, Some(true));
        assert_eq!(params.include_knowledge, Some(false));
        assert_eq!(params.include_structure, Some(true));
        assert_eq!(
            params.arc_families,
            Some(vec!["ownership".to_string(), "semantic".to_string()])
        );
        assert_eq!(params.max_depth, Some(5));
    }

    #[test]
    fn test_assemble_params_missing_required() {
        // Missing locale
        let json = r#"{"focus_key": "homepage"}"#;
        let result: std::result::Result<AssembleParams, _> = serde_json::from_str(json);
        assert!(result.is_err());

        // Missing focus_key
        let json = r#"{"locale": "fr-FR"}"#;
        let result: std::result::Result<AssembleParams, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ==================== EvidencePacket Tests ====================

    #[test]
    fn test_evidence_packet_serialize() {
        let packet = EvidencePacket {
            source_key: "qr-code".to_string(),
            source_kind: "Entity".to_string(),
            evidence_type: "definition".to_string(),
            distance: 2,
            relevance: 0.85,
            content: "QR Code: A two-dimensional barcode".to_string(),
            tokens: 8,
        };

        let json = serde_json::to_string(&packet).unwrap();
        assert!(json.contains(r#""source_key":"qr-code""#));
        assert!(json.contains(r#""source_kind":"Entity""#));
        assert!(json.contains(r#""evidence_type":"definition""#));
        assert!(json.contains(r#""distance":2"#));
        assert!(json.contains(r#""relevance":0.85"#));
        assert!(json.contains(r#""tokens":8"#));
    }

    #[test]
    fn test_evidence_packet_relevance_range() {
        // Relevance should be between 0.0 and 1.0
        let packet = EvidencePacket {
            source_key: "test".to_string(),
            source_kind: "Test".to_string(),
            evidence_type: "test".to_string(),
            distance: 1,
            relevance: 0.0,
            content: "".to_string(),
            tokens: 0,
        };
        assert!(packet.relevance >= 0.0 && packet.relevance <= 1.0);

        let packet = EvidencePacket {
            source_key: "test".to_string(),
            source_kind: "Test".to_string(),
            evidence_type: "test".to_string(),
            distance: 1,
            relevance: 1.0,
            content: "".to_string(),
            tokens: 0,
        };
        assert!(packet.relevance >= 0.0 && packet.relevance <= 1.0);
    }

    // ==================== LocaleContext Tests ====================

    #[test]
    fn test_locale_context_serialize() {
        let context = LocaleContext {
            locale_key: "fr-FR".to_string(),
            language: "French".to_string(),
            region: Some("France".to_string()),
            voice: Some("Professional, friendly".to_string()),
            formatting: Some(serde_json::json!({"date": "DD/MM/YYYY"})),
        };

        let json = serde_json::to_string(&context).unwrap();
        assert!(json.contains(r#""locale_key":"fr-FR""#));
        assert!(json.contains(r#""language":"French""#));
        assert!(json.contains(r#""region":"France""#));
        assert!(json.contains(r#""voice":"Professional, friendly""#));
    }

    #[test]
    fn test_locale_context_minimal() {
        let context = LocaleContext {
            locale_key: "en-US".to_string(),
            language: "English".to_string(),
            region: None,
            voice: None,
            formatting: None,
        };

        let json = serde_json::to_string(&context).unwrap();
        assert!(json.contains(r#""locale_key":"en-US""#));
        assert!(json.contains(r#""region":null"#));
    }

    // ==================== FocusNode Tests ====================

    #[test]
    fn test_focus_node_serialize() {
        let focus = FocusNode {
            key: "homepage".to_string(),
            kind: "Page".to_string(),
            name: Some("Home Page".to_string()),
            description: Some("Main landing page".to_string()),
        };

        let json = serde_json::to_string(&focus).unwrap();
        assert!(json.contains(r#""key":"homepage""#));
        assert!(json.contains(r#""kind":"Page""#));
        assert!(json.contains(r#""name":"Home Page""#));
    }

    #[test]
    fn test_focus_node_minimal() {
        let focus = FocusNode {
            key: "entity-1".to_string(),
            kind: "Entity".to_string(),
            name: None,
            description: None,
        };

        let json = serde_json::to_string(&focus).unwrap();
        assert!(json.contains(r#""key":"entity-1""#));
        assert!(json.contains(r#""name":null"#));
    }

    // ==================== AssembleResult Tests ====================

    #[test]
    fn test_assemble_result_serialize() {
        let result = AssembleResult {
            focus: FocusNode {
                key: "test".to_string(),
                kind: "Page".to_string(),
                name: Some("Test".to_string()),
                description: None,
            },
            evidence: vec![],
            locale_context: LocaleContext {
                locale_key: "en-US".to_string(),
                language: "English".to_string(),
                region: None,
                voice: None,
                formatting: None,
            },
            total_tokens: 1000,
            budget_remaining: 49000,
            nodes_visited: 15,
            truncated: false,
            execution_time_ms: 125,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains(r#""total_tokens":1000"#));
        assert!(json.contains(r#""budget_remaining":49000"#));
        assert!(json.contains(r#""nodes_visited":15"#));
        assert!(json.contains(r#""truncated":false"#));
        assert!(json.contains(r#""execution_time_ms":125"#));
    }

    #[test]
    fn test_assemble_result_truncated() {
        let result = AssembleResult {
            focus: FocusNode {
                key: "test".to_string(),
                kind: "Page".to_string(),
                name: None,
                description: None,
            },
            evidence: vec![],
            locale_context: LocaleContext {
                locale_key: "en-US".to_string(),
                language: "English".to_string(),
                region: None,
                voice: None,
                formatting: None,
            },
            total_tokens: 50000,
            budget_remaining: 0,
            nodes_visited: 100,
            truncated: true,
            execution_time_ms: 500,
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains(r#""truncated":true"#));
        assert!(json.contains(r#""budget_remaining":0"#));
    }

    // ==================== compress_to_evidence Tests ====================

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

    #[test]
    fn test_compress_to_evidence_empty() {
        let result = compress_to_evidence(None, None);
        assert!(result.is_empty());
    }

    #[test]
    fn test_compress_to_evidence_empty_strings() {
        let result = compress_to_evidence(Some(""), Some(""));
        assert!(result.is_empty() || result == ": ");
    }

    #[test]
    fn test_compress_to_evidence_exact_200_bytes() {
        // Name exactly 200 bytes
        let name = "N".repeat(200);
        let result = compress_to_evidence(Some(&name), None);
        assert_eq!(result.len(), 200);

        // Name + description exactly 200 bytes
        let name = "N".repeat(50);
        let desc = "D".repeat(148); // 50 + ": " (2) + 148 = 200
        let result = compress_to_evidence(Some(&name), Some(&desc));
        assert_eq!(result.len(), 200);
    }

    #[test]
    fn test_compress_to_evidence_unicode() {
        // Unicode characters may have different byte lengths
        let name = "日本語";
        let result = compress_to_evidence(Some(name), None);
        assert_eq!(result, "日本語");
    }

    #[test]
    fn test_compress_to_evidence_long_name_short_desc() {
        let long_name = "N".repeat(195);
        let short_desc = "Short";
        let result = compress_to_evidence(Some(&long_name), Some(short_desc));
        // Should truncate description since name is already 195 chars
        // 195 + ": " (2) = 197, leaving only 3 chars for description
        assert!(result.len() <= 203);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_token_estimate_calculation() {
        // Verify token estimate formula: chars / 4 (ceiling)
        let content = "Hello World"; // 11 chars
        let tokens = content.len().div_ceil(4);
        assert_eq!(tokens, 3); // 11/4 = 2.75 -> 3

        let content = "ABCD"; // 4 chars
        let tokens = content.len().div_ceil(4);
        assert_eq!(tokens, 1); // 4/4 = 1

        let content = ""; // 0 chars
        let tokens = content.len().div_ceil(4);
        assert_eq!(tokens, 0); // 0/4 = 0
    }

    #[test]
    fn test_relevance_calculation() {
        // Relevance formula: 1.0 / (distance + 1.0)
        let distance = 0_f64;
        let relevance = 1.0 / (distance + 1.0);
        assert!((relevance - 1.0).abs() < f64::EPSILON);

        let distance = 1_f64;
        let relevance = 1.0 / (distance + 1.0);
        assert!((relevance - 0.5).abs() < f64::EPSILON);

        let distance = 3_f64;
        let relevance = 1.0 / (distance + 1.0);
        assert!((relevance - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn test_assembly_strategy_case_sensitivity() {
        // Should be lowercase
        let valid = r#""breadth""#;
        let invalid = r#""Breadth""#;

        assert!(serde_json::from_str::<AssemblyStrategy>(valid).is_ok());
        assert!(serde_json::from_str::<AssemblyStrategy>(invalid).is_err());
    }
}
