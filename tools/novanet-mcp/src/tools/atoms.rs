//! novanet_atoms tool
//!
//! Retrieve knowledge atoms (Terms, Expressions, Patterns, CultureRefs, Taboos, AudienceTraits)
//! for a specific locale and domain. Enables selective LLM context loading.

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Type of knowledge atom to retrieve
#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AtomType {
    /// Technical terms with definitions
    Term,
    /// Idiomatic expressions
    Expression,
    /// Text patterns/templates
    Pattern,
    /// Cultural references
    CultureRef,
    /// Cultural taboos to avoid
    Taboo,
    /// Audience characteristics
    AudienceTrait,
    /// All atom types
    #[default]
    All,
}

/// Parameters for novanet_atoms tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AtomsParams {
    /// Target locale (e.g., "fr-FR")
    pub locale: String,
    /// Atom type to retrieve
    #[serde(default)]
    pub atom_type: AtomType,
    /// Filter by domain (e.g., "technical", "legal", "marketing")
    #[serde(default)]
    pub domain: Option<String>,
    /// Filter by register (e.g., "formal", "casual")
    #[serde(default)]
    pub register: Option<String>,
    /// Search query to filter atoms
    #[serde(default)]
    pub query: Option<String>,
    /// Maximum number of atoms to return (default: 50)
    #[serde(default)]
    pub limit: Option<usize>,
    /// Include container metadata (TermSet, ExpressionSet, etc.)
    #[serde(default)]
    pub include_containers: Option<bool>,
}

/// A knowledge atom
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct Atom {
    /// Atom key
    pub key: String,
    /// Atom type (Term, Expression, Pattern, etc.)
    pub atom_type: String,
    /// Primary value/content
    pub value: String,
    /// Domain (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Register (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register: Option<String>,
    /// Additional properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Container key (TermSet, ExpressionSet, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_key: Option<String>,
}

/// Container set information
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AtomContainer {
    /// Container key
    pub key: String,
    /// Container type (TermSet, ExpressionSet, etc.)
    pub container_type: String,
    /// Domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Atom count in this container
    pub atom_count: usize,
}

/// Result from novanet_atoms tool
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AtomsResult {
    /// Locale key
    pub locale: String,
    /// Retrieved atoms
    pub atoms: Vec<Atom>,
    /// Container information (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<AtomContainer>>,
    /// Total atoms matching query (may be > returned if limited)
    pub total_count: usize,
    /// Token estimate for the result
    pub token_estimate: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Execute the novanet_atoms tool
pub async fn execute(state: &State, params: AtomsParams) -> Result<AtomsResult> {
    let start = std::time::Instant::now();

    let limit = params.limit.unwrap_or(50).min(200);
    let include_containers = params.include_containers.unwrap_or(false);

    let mut all_atoms = Vec::new();

    // Retrieve atoms based on type
    match params.atom_type {
        AtomType::Term => {
            all_atoms.extend(get_terms(state, &params, limit).await?);
        }
        AtomType::Expression => {
            all_atoms.extend(get_expressions(state, &params, limit).await?);
        }
        AtomType::Pattern => {
            all_atoms.extend(get_patterns(state, &params, limit).await?);
        }
        AtomType::CultureRef => {
            all_atoms.extend(get_culture_refs(state, &params, limit).await?);
        }
        AtomType::Taboo => {
            all_atoms.extend(get_taboos(state, &params, limit).await?);
        }
        AtomType::AudienceTrait => {
            all_atoms.extend(get_audience_traits(state, &params, limit).await?);
        }
        AtomType::All => {
            let per_type_limit = (limit / 6).max(5);
            all_atoms.extend(get_terms(state, &params, per_type_limit).await?);
            all_atoms.extend(get_expressions(state, &params, per_type_limit).await?);
            all_atoms.extend(get_patterns(state, &params, per_type_limit).await?);
            all_atoms.extend(get_culture_refs(state, &params, per_type_limit).await?);
            all_atoms.extend(get_taboos(state, &params, per_type_limit).await?);
            all_atoms.extend(get_audience_traits(state, &params, per_type_limit).await?);
        }
    }

    // Get containers if requested
    let containers = if include_containers {
        Some(get_containers(state, &params.locale).await?)
    } else {
        None
    };

    let total_count = all_atoms.len();
    let json_string = serde_json::to_string(&all_atoms).unwrap_or_default();
    let token_estimate = json_string.len().div_ceil(4);

    Ok(AtomsResult {
        locale: params.locale,
        atoms: all_atoms,
        containers,
        total_count,
        token_estimate,
        execution_time_ms: start.elapsed().as_millis() as u64,
    })
}

/// Get Terms for a locale
async fn get_terms(state: &State, params: &AtomsParams, limit: usize) -> Result<Vec<Atom>> {
    let domain_filter = params
        .domain
        .as_ref()
        .map(|d| format!("AND t.domain = '{}'", d))
        .unwrap_or_default();

    let query_filter = params.query.as_ref()
        .map(|_| "AND (toLower(t.key) CONTAINS toLower($query) OR toLower(t.value) CONTAINS toLower($query))")
        .unwrap_or_default();

    let cypher = format!(
        r#"
        MATCH (l:Locale {{key: $locale}})-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)
        WHERE true {domain_filter} {query_filter}
        RETURN t.key AS key, t.value AS value, t.domain AS domain,
               t.definition AS definition, ts.key AS container_key
        LIMIT {limit}
        "#,
        domain_filter = domain_filter,
        query_filter = query_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("locale".to_string(), serde_json::json!(params.locale));
    if let Some(q) = &params.query {
        query_params.insert("query".to_string(), serde_json::json!(q));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| Atom {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            atom_type: "Term".to_string(),
            value: row["value"].as_str().unwrap_or_default().to_string(),
            domain: row["domain"].as_str().map(|s| s.to_string()),
            register: None,
            properties: row
                .get("definition")
                .map(|d| serde_json::json!({"definition": d})),
            container_key: row["container_key"].as_str().map(|s| s.to_string()),
        })
        .collect())
}

/// Get Expressions for a locale
async fn get_expressions(state: &State, params: &AtomsParams, limit: usize) -> Result<Vec<Atom>> {
    let register_filter = params
        .register
        .as_ref()
        .map(|r| format!("AND e.register = '{}'", r))
        .unwrap_or_default();

    let query_filter = params.query.as_ref()
        .map(|_| "AND (toLower(e.key) CONTAINS toLower($query) OR toLower(e.value) CONTAINS toLower($query))")
        .unwrap_or_default();

    let cypher = format!(
        r#"
        MATCH (l:Locale {{key: $locale}})-[:HAS_EXPRESSIONS]->(es:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e:Expression)
        WHERE true {register_filter} {query_filter}
        RETURN e.key AS key, e.value AS value, e.register AS register,
               e.context AS context, es.key AS container_key
        LIMIT {limit}
        "#,
        register_filter = register_filter,
        query_filter = query_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("locale".to_string(), serde_json::json!(params.locale));
    if let Some(q) = &params.query {
        query_params.insert("query".to_string(), serde_json::json!(q));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| Atom {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            atom_type: "Expression".to_string(),
            value: row["value"].as_str().unwrap_or_default().to_string(),
            domain: None,
            register: row["register"].as_str().map(|s| s.to_string()),
            properties: row
                .get("context")
                .map(|c| serde_json::json!({"context": c})),
            container_key: row["container_key"].as_str().map(|s| s.to_string()),
        })
        .collect())
}

/// Get Patterns for a locale
async fn get_patterns(state: &State, params: &AtomsParams, limit: usize) -> Result<Vec<Atom>> {
    let query_filter = params.query.as_ref()
        .map(|_| "AND (toLower(p.key) CONTAINS toLower($query) OR toLower(p.template) CONTAINS toLower($query))")
        .unwrap_or_default();

    let cypher = format!(
        r#"
        MATCH (l:Locale {{key: $locale}})-[:HAS_PATTERNS]->(ps:PatternSet)-[:CONTAINS_PATTERN]->(p:Pattern)
        WHERE true {query_filter}
        RETURN p.key AS key, p.template AS value, p.purpose AS purpose,
               ps.key AS container_key
        LIMIT {limit}
        "#,
        query_filter = query_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("locale".to_string(), serde_json::json!(params.locale));
    if let Some(q) = &params.query {
        query_params.insert("query".to_string(), serde_json::json!(q));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| Atom {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            atom_type: "Pattern".to_string(),
            value: row["value"].as_str().unwrap_or_default().to_string(),
            domain: None,
            register: None,
            properties: row
                .get("purpose")
                .map(|p| serde_json::json!({"purpose": p})),
            container_key: row["container_key"].as_str().map(|s| s.to_string()),
        })
        .collect())
}

/// Get CultureRefs for a locale
async fn get_culture_refs(state: &State, params: &AtomsParams, limit: usize) -> Result<Vec<Atom>> {
    let query_filter = params.query.as_ref()
        .map(|_| "AND (toLower(c.key) CONTAINS toLower($query) OR toLower(c.reference) CONTAINS toLower($query))")
        .unwrap_or_default();

    let cypher = format!(
        r#"
        MATCH (l:Locale {{key: $locale}})-[:HAS_CULTURE]->(cs:CultureSet)-[:CONTAINS_CULTURE_REF]->(c:CultureRef)
        WHERE true {query_filter}
        RETURN c.key AS key, c.reference AS value, c.context AS context,
               c.appropriateness AS appropriateness, cs.key AS container_key
        LIMIT {limit}
        "#,
        query_filter = query_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("locale".to_string(), serde_json::json!(params.locale));
    if let Some(q) = &params.query {
        query_params.insert("query".to_string(), serde_json::json!(q));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| Atom {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            atom_type: "CultureRef".to_string(),
            value: row["value"].as_str().unwrap_or_default().to_string(),
            domain: None,
            register: None,
            properties: Some(serde_json::json!({
                "context": row.get("context"),
                "appropriateness": row.get("appropriateness")
            })),
            container_key: row["container_key"].as_str().map(|s| s.to_string()),
        })
        .collect())
}

/// Get Taboos for a locale
async fn get_taboos(state: &State, params: &AtomsParams, limit: usize) -> Result<Vec<Atom>> {
    let query_filter = params.query.as_ref()
        .map(|_| "AND (toLower(t.key) CONTAINS toLower($query) OR toLower(t.description) CONTAINS toLower($query))")
        .unwrap_or_default();

    let cypher = format!(
        r#"
        MATCH (l:Locale {{key: $locale}})-[:HAS_TABOOS]->(ts:TabooSet)-[:CONTAINS_TABOO]->(t:Taboo)
        WHERE true {query_filter}
        RETURN t.key AS key, t.description AS value, t.severity AS severity,
               t.category AS category, ts.key AS container_key
        LIMIT {limit}
        "#,
        query_filter = query_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("locale".to_string(), serde_json::json!(params.locale));
    if let Some(q) = &params.query {
        query_params.insert("query".to_string(), serde_json::json!(q));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| Atom {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            atom_type: "Taboo".to_string(),
            value: row["value"].as_str().unwrap_or_default().to_string(),
            domain: None,
            register: None,
            properties: Some(serde_json::json!({
                "severity": row.get("severity"),
                "category": row.get("category")
            })),
            container_key: row["container_key"].as_str().map(|s| s.to_string()),
        })
        .collect())
}

/// Get AudienceTraits for a locale
async fn get_audience_traits(
    state: &State,
    params: &AtomsParams,
    limit: usize,
) -> Result<Vec<Atom>> {
    let query_filter = params.query.as_ref()
        .map(|_| "AND (toLower(a.key) CONTAINS toLower($query) OR toLower(a.trait) CONTAINS toLower($query))")
        .unwrap_or_default();

    let cypher = format!(
        r#"
        MATCH (l:Locale {{key: $locale}})-[:HAS_AUDIENCE]->(as:AudienceSet)-[:CONTAINS_AUDIENCE_TRAIT]->(a:AudienceTrait)
        WHERE true {query_filter}
        RETURN a.key AS key, a.trait AS value, a.demographic AS demographic,
               a.behavior AS behavior, as.key AS container_key
        LIMIT {limit}
        "#,
        query_filter = query_filter,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("locale".to_string(), serde_json::json!(params.locale));
    if let Some(q) = &params.query {
        query_params.insert("query".to_string(), serde_json::json!(q));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| Atom {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            atom_type: "AudienceTrait".to_string(),
            value: row["value"].as_str().unwrap_or_default().to_string(),
            domain: None,
            register: None,
            properties: Some(serde_json::json!({
                "demographic": row.get("demographic"),
                "behavior": row.get("behavior")
            })),
            container_key: row["container_key"].as_str().map(|s| s.to_string()),
        })
        .collect())
}

/// Get container information for a locale
async fn get_containers(state: &State, locale: &str) -> Result<Vec<AtomContainer>> {
    let cypher = r#"
        MATCH (l:Locale {key: $locale})
        OPTIONAL MATCH (l)-[:HAS_TERMS]->(ts:TermSet)
        OPTIONAL MATCH (ts)-[:CONTAINS_TERM]->(t:Term)
        WITH l, ts, count(t) AS term_count
        OPTIONAL MATCH (l)-[:HAS_EXPRESSIONS]->(es:ExpressionSet)
        OPTIONAL MATCH (es)-[:CONTAINS_EXPRESSION]->(e:Expression)
        WITH l, ts, term_count, es, count(e) AS expr_count
        OPTIONAL MATCH (l)-[:HAS_PATTERNS]->(ps:PatternSet)
        OPTIONAL MATCH (ps)-[:CONTAINS_PATTERN]->(p:Pattern)
        WITH l, ts, term_count, es, expr_count, ps, count(p) AS pattern_count
        OPTIONAL MATCH (l)-[:HAS_CULTURE]->(cs:CultureSet)
        OPTIONAL MATCH (cs)-[:CONTAINS_CULTURE_REF]->(c:CultureRef)
        WITH l, ts, term_count, es, expr_count, ps, pattern_count, cs, count(c) AS culture_count
        OPTIONAL MATCH (l)-[:HAS_TABOOS]->(tbs:TabooSet)
        OPTIONAL MATCH (tbs)-[:CONTAINS_TABOO]->(tb:Taboo)
        WITH l, ts, term_count, es, expr_count, ps, pattern_count, cs, culture_count, tbs, count(tb) AS taboo_count
        OPTIONAL MATCH (l)-[:HAS_AUDIENCE]->(aus:AudienceSet)
        OPTIONAL MATCH (aus)-[:CONTAINS_AUDIENCE_TRAIT]->(au:AudienceTrait)
        RETURN
            ts.key AS term_set_key, ts.domain AS term_set_domain, term_count,
            es.key AS expr_set_key, es.register AS expr_set_register, expr_count,
            ps.key AS pattern_set_key, pattern_count,
            cs.key AS culture_set_key, culture_count,
            tbs.key AS taboo_set_key, taboo_count,
            aus.key AS audience_set_key, count(au) AS audience_count
    "#;

    let mut params = serde_json::Map::new();
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    let mut containers = Vec::new();

    if let Some(row) = rows.first() {
        if let Some(key) = row["term_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "TermSet".to_string(),
                domain: row["term_set_domain"].as_str().map(|s| s.to_string()),
                atom_count: row["term_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["expr_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "ExpressionSet".to_string(),
                domain: row["expr_set_register"].as_str().map(|s| s.to_string()),
                atom_count: row["expr_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["pattern_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "PatternSet".to_string(),
                domain: None,
                atom_count: row["pattern_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["culture_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "CultureSet".to_string(),
                domain: None,
                atom_count: row["culture_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["taboo_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "TabooSet".to_string(),
                domain: None,
                atom_count: row["taboo_count"].as_u64().unwrap_or(0) as usize,
            });
        }
        if let Some(key) = row["audience_set_key"].as_str() {
            containers.push(AtomContainer {
                key: key.to_string(),
                container_type: "AudienceSet".to_string(),
                domain: None,
                atom_count: row["audience_count"].as_u64().unwrap_or(0) as usize,
            });
        }
    }

    Ok(containers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom_type_default() {
        let atom_type: AtomType = Default::default();
        assert!(matches!(atom_type, AtomType::All));
    }
}
