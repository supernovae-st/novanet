//! novanet_atoms tool
//!
//! Retrieve knowledge atoms (Terms, Expressions, Patterns, CultureRefs, Taboos, AudienceTraits)
//! for a specific locale and domain. Enables selective LLM context loading.
//!
//! Uses a generic `fetch_atoms` function to avoid code duplication across 6 atom types.

use crate::error::Result;
use crate::server::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Configuration for fetching a specific atom type
struct AtomConfig {
    /// Arc from Locale to container (e.g., "HAS_TERMS")
    locale_arc: &'static str,
    /// Container label (e.g., "TermSet")
    container_label: &'static str,
    /// Arc from container to atom (e.g., "CONTAINS_TERM")
    contains_arc: &'static str,
    /// Atom label (e.g., "Term")
    atom_label: &'static str,
    /// Atom type string for output (e.g., "Term")
    atom_type_name: &'static str,
    /// Property name for main value (e.g., "value", "template", "reference")
    value_property: &'static str,
    /// Filter field name if any (e.g., "domain", "register")
    filter_field: Option<&'static str>,
    /// Additional properties to extract
    extra_properties: &'static [(&'static str, &'static str)],
    /// Search fields for query filter
    search_fields: &'static [&'static str],
}

/// Static configurations for each atom type
const TERM_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_TERMS",
    container_label: "TermSet",
    contains_arc: "CONTAINS_TERM",
    atom_label: "Term",
    atom_type_name: "Term",
    value_property: "value",
    filter_field: Some("domain"),
    extra_properties: &[("definition", "definition")],
    search_fields: &["key", "value"],
};

const EXPRESSION_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_EXPRESSIONS",
    container_label: "ExpressionSet",
    contains_arc: "CONTAINS_EXPRESSION",
    atom_label: "Expression",
    atom_type_name: "Expression",
    value_property: "value",
    filter_field: Some("register"),
    extra_properties: &[("context", "context")],
    search_fields: &["key", "value"],
};

const PATTERN_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_PATTERNS",
    container_label: "PatternSet",
    contains_arc: "CONTAINS_PATTERN",
    atom_label: "Pattern",
    atom_type_name: "Pattern",
    value_property: "template",
    filter_field: None,
    extra_properties: &[("purpose", "purpose")],
    search_fields: &["key", "template"],
};

const CULTURE_REF_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_CULTURE",
    container_label: "CultureSet",
    contains_arc: "CONTAINS_CULTURE_REF",
    atom_label: "CultureRef",
    atom_type_name: "CultureRef",
    value_property: "reference",
    filter_field: None,
    extra_properties: &[
        ("context", "context"),
        ("appropriateness", "appropriateness"),
    ],
    search_fields: &["key", "reference"],
};

const TABOO_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_TABOOS",
    container_label: "TabooSet",
    contains_arc: "CONTAINS_TABOO",
    atom_label: "Taboo",
    atom_type_name: "Taboo",
    value_property: "description",
    filter_field: None,
    extra_properties: &[("severity", "severity"), ("category", "category")],
    search_fields: &["key", "description"],
};

const AUDIENCE_TRAIT_CONFIG: AtomConfig = AtomConfig {
    locale_arc: "HAS_AUDIENCE",
    container_label: "AudienceSet",
    contains_arc: "CONTAINS_AUDIENCE_TRAIT",
    atom_label: "AudienceTrait",
    atom_type_name: "AudienceTrait",
    value_property: "trait",
    filter_field: None,
    extra_properties: &[("demographic", "demographic"), ("behavior", "behavior")],
    search_fields: &["key", "trait"],
};

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
#[instrument(name = "novanet_atoms", skip(state), fields(locale = %params.locale, atom_type = ?params.atom_type))]
pub async fn execute(state: &State, params: AtomsParams) -> Result<AtomsResult> {
    let start = std::time::Instant::now();

    let limit = params.limit.unwrap_or(50).min(200);
    let include_containers = params.include_containers.unwrap_or(false);

    let mut all_atoms = Vec::new();

    // Retrieve atoms based on type using generic fetch function
    match params.atom_type {
        AtomType::Term => {
            all_atoms.extend(fetch_atoms(state, &params, &TERM_CONFIG, limit).await?);
        }
        AtomType::Expression => {
            all_atoms.extend(fetch_atoms(state, &params, &EXPRESSION_CONFIG, limit).await?);
        }
        AtomType::Pattern => {
            all_atoms.extend(fetch_atoms(state, &params, &PATTERN_CONFIG, limit).await?);
        }
        AtomType::CultureRef => {
            all_atoms.extend(fetch_atoms(state, &params, &CULTURE_REF_CONFIG, limit).await?);
        }
        AtomType::Taboo => {
            all_atoms.extend(fetch_atoms(state, &params, &TABOO_CONFIG, limit).await?);
        }
        AtomType::AudienceTrait => {
            all_atoms.extend(fetch_atoms(state, &params, &AUDIENCE_TRAIT_CONFIG, limit).await?);
        }
        AtomType::All => {
            let per_type_limit = (limit / 6).max(5);
            all_atoms.extend(fetch_atoms(state, &params, &TERM_CONFIG, per_type_limit).await?);
            all_atoms
                .extend(fetch_atoms(state, &params, &EXPRESSION_CONFIG, per_type_limit).await?);
            all_atoms.extend(fetch_atoms(state, &params, &PATTERN_CONFIG, per_type_limit).await?);
            all_atoms
                .extend(fetch_atoms(state, &params, &CULTURE_REF_CONFIG, per_type_limit).await?);
            all_atoms.extend(fetch_atoms(state, &params, &TABOO_CONFIG, per_type_limit).await?);
            all_atoms
                .extend(fetch_atoms(state, &params, &AUDIENCE_TRAIT_CONFIG, per_type_limit).await?);
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

/// Generic function to fetch atoms of any type using configuration
async fn fetch_atoms(
    state: &State,
    params: &AtomsParams,
    config: &AtomConfig,
    limit: usize,
) -> Result<Vec<Atom>> {
    // Build filter for domain/register (if applicable) using parameterized queries
    // SECURITY: Always use parameters, never string interpolation for user input
    let (field_filter, field_param_name, field_param_value): (String, Option<&str>, Option<&str>) =
        match config.filter_field {
            Some("domain") => match &params.domain {
                Some(d) => (
                    "AND a.domain = $domain_filter".to_string(),
                    Some("domain_filter"),
                    Some(d.as_str()),
                ),
                None => (String::new(), None, None),
            },
            Some("register") => match &params.register {
                Some(r) => (
                    "AND a.register = $register_filter".to_string(),
                    Some("register_filter"),
                    Some(r.as_str()),
                ),
                None => (String::new(), None, None),
            },
            _ => (String::new(), None, None),
        };

    // Build search query filter
    let query_filter = params
        .query
        .as_ref()
        .map(|_| {
            let fields: Vec<String> = config
                .search_fields
                .iter()
                .map(|f| format!("toLower(a.{}) CONTAINS toLower($query)", f))
                .collect();
            format!("AND ({})", fields.join(" OR "))
        })
        .unwrap_or_default();

    // Build extra properties return clause
    let extra_props: Vec<String> = config
        .extra_properties
        .iter()
        .map(|(prop, alias)| format!("a.{} AS {}", prop, alias))
        .collect();
    let extra_props_clause = if extra_props.is_empty() {
        String::new()
    } else {
        format!(", {}", extra_props.join(", "))
    };

    let cypher = format!(
        r#"
        MATCH (l:Locale {{key: $locale}})-[:{locale_arc}]->(c:{container})-[:{contains_arc}]->(a:{atom})
        WHERE true {field_filter} {query_filter}
        RETURN a.key AS key, a.{value_prop} AS value, c.key AS container_key{extra_props}
        LIMIT {limit}
        "#,
        locale_arc = config.locale_arc,
        container = config.container_label,
        contains_arc = config.contains_arc,
        atom = config.atom_label,
        value_prop = config.value_property,
        field_filter = field_filter,
        query_filter = query_filter,
        extra_props = extra_props_clause,
        limit = limit
    );

    let mut query_params = serde_json::Map::new();
    query_params.insert("locale".to_string(), serde_json::json!(params.locale));
    if let Some(q) = &params.query {
        query_params.insert("query".to_string(), serde_json::json!(q));
    }
    // SECURITY: Add parameterized field filter value
    if let (Some(param_name), Some(param_value)) = (field_param_name, field_param_value) {
        query_params.insert(param_name.to_string(), serde_json::json!(param_value));
    }

    let rows = state
        .pool()
        .execute_query(&cypher, Some(query_params))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            // Build properties object from extra_properties
            let properties = if config.extra_properties.is_empty() {
                None
            } else {
                let mut props = serde_json::Map::new();
                for (_, alias) in config.extra_properties {
                    if let Some(val) = row.get(*alias) {
                        if !val.is_null() {
                            props.insert(alias.to_string(), val.clone());
                        }
                    }
                }
                if props.is_empty() {
                    None
                } else {
                    Some(serde_json::Value::Object(props))
                }
            };

            Atom {
                key: row["key"].as_str().unwrap_or_default().to_string(),
                atom_type: config.atom_type_name.to_string(),
                value: row["value"].as_str().unwrap_or_default().to_string(),
                domain: if config.filter_field == Some("domain") {
                    row.get("domain")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                },
                register: if config.filter_field == Some("register") {
                    row.get("register")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                } else {
                    None
                },
                properties,
                container_key: row["container_key"].as_str().map(|s| s.to_string()),
            }
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

    // ══════════════════════════════════════════════════════════════
    // ATOM TYPE TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_atom_type_default() {
        let atom_type: AtomType = Default::default();
        assert!(matches!(atom_type, AtomType::All));
    }

    #[test]
    fn test_atom_type_deserialize_term() {
        let json = r#""term""#;
        let atom_type: AtomType = serde_json::from_str(json).unwrap();
        assert!(matches!(atom_type, AtomType::Term));
    }

    #[test]
    fn test_atom_type_deserialize_expression() {
        let json = r#""expression""#;
        let atom_type: AtomType = serde_json::from_str(json).unwrap();
        assert!(matches!(atom_type, AtomType::Expression));
    }

    #[test]
    fn test_atom_type_deserialize_pattern() {
        let json = r#""pattern""#;
        let atom_type: AtomType = serde_json::from_str(json).unwrap();
        assert!(matches!(atom_type, AtomType::Pattern));
    }

    #[test]
    fn test_atom_type_deserialize_cultureref() {
        let json = r#""cultureref""#;
        let atom_type: AtomType = serde_json::from_str(json).unwrap();
        assert!(matches!(atom_type, AtomType::CultureRef));
    }

    #[test]
    fn test_atom_type_deserialize_taboo() {
        let json = r#""taboo""#;
        let atom_type: AtomType = serde_json::from_str(json).unwrap();
        assert!(matches!(atom_type, AtomType::Taboo));
    }

    #[test]
    fn test_atom_type_deserialize_audiencetrait() {
        let json = r#""audiencetrait""#;
        let atom_type: AtomType = serde_json::from_str(json).unwrap();
        assert!(matches!(atom_type, AtomType::AudienceTrait));
    }

    #[test]
    fn test_atom_type_deserialize_all() {
        let json = r#""all""#;
        let atom_type: AtomType = serde_json::from_str(json).unwrap();
        assert!(matches!(atom_type, AtomType::All));
    }

    // ══════════════════════════════════════════════════════════════
    // ATOMS PARAMS TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_atoms_params_deserialize_minimal() {
        let json = r#"{"locale": "fr-FR"}"#;
        let params: AtomsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.locale, "fr-FR");
        assert!(matches!(params.atom_type, AtomType::All));
        assert!(params.domain.is_none());
        assert!(params.register.is_none());
        assert!(params.query.is_none());
        assert!(params.limit.is_none());
        assert!(params.include_containers.is_none());
    }

    #[test]
    fn test_atoms_params_deserialize_full() {
        let json = r#"{
            "locale": "es-MX",
            "atom_type": "term",
            "domain": "technical",
            "register": "formal",
            "query": "QR",
            "limit": 100,
            "include_containers": true
        }"#;
        let params: AtomsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.locale, "es-MX");
        assert!(matches!(params.atom_type, AtomType::Term));
        assert_eq!(params.domain, Some("technical".to_string()));
        assert_eq!(params.register, Some("formal".to_string()));
        assert_eq!(params.query, Some("QR".to_string()));
        assert_eq!(params.limit, Some(100));
        assert_eq!(params.include_containers, Some(true));
    }

    // ══════════════════════════════════════════════════════════════
    // ATOM STRUCT TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_atom_serialize_minimal() {
        let atom = Atom {
            key: "qr-code".to_string(),
            atom_type: "Term".to_string(),
            value: "code QR".to_string(),
            domain: None,
            register: None,
            properties: None,
            container_key: None,
        };
        let json = serde_json::to_value(&atom).unwrap();
        assert_eq!(json["key"], "qr-code");
        assert_eq!(json["atom_type"], "Term");
        assert_eq!(json["value"], "code QR");
        // Optional fields should be skipped
        assert!(json.get("domain").is_none());
        assert!(json.get("register").is_none());
        assert!(json.get("properties").is_none());
        assert!(json.get("container_key").is_none());
    }

    #[test]
    fn test_atom_serialize_with_optional_fields() {
        let atom = Atom {
            key: "formal-greeting".to_string(),
            atom_type: "Expression".to_string(),
            value: "Bonjour".to_string(),
            domain: Some("greeting".to_string()),
            register: Some("formal".to_string()),
            properties: Some(serde_json::json!({"context": "business"})),
            container_key: Some("greetings-fr".to_string()),
        };
        let json = serde_json::to_value(&atom).unwrap();
        assert_eq!(json["domain"], "greeting");
        assert_eq!(json["register"], "formal");
        assert_eq!(json["properties"]["context"], "business");
        assert_eq!(json["container_key"], "greetings-fr");
    }

    // ══════════════════════════════════════════════════════════════
    // ATOM CONTAINER TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_atom_container_serialize() {
        let container = AtomContainer {
            key: "tech-terms-fr".to_string(),
            container_type: "TermSet".to_string(),
            domain: Some("technical".to_string()),
            atom_count: 150,
        };
        let json = serde_json::to_value(&container).unwrap();
        assert_eq!(json["key"], "tech-terms-fr");
        assert_eq!(json["container_type"], "TermSet");
        assert_eq!(json["domain"], "technical");
        assert_eq!(json["atom_count"], 150);
    }

    #[test]
    fn test_atom_container_serialize_no_domain() {
        let container = AtomContainer {
            key: "patterns-fr".to_string(),
            container_type: "PatternSet".to_string(),
            domain: None,
            atom_count: 25,
        };
        let json = serde_json::to_value(&container).unwrap();
        assert!(json.get("domain").is_none());
    }

    // ══════════════════════════════════════════════════════════════
    // ATOMS RESULT TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_atoms_result_serialize() {
        let result = AtomsResult {
            locale: "fr-FR".to_string(),
            atoms: vec![Atom {
                key: "test".to_string(),
                atom_type: "Term".to_string(),
                value: "test value".to_string(),
                domain: None,
                register: None,
                properties: None,
                container_key: None,
            }],
            containers: None,
            total_count: 1,
            token_estimate: 50,
            execution_time_ms: 25,
        };
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["locale"], "fr-FR");
        assert_eq!(json["atoms"].as_array().unwrap().len(), 1);
        assert_eq!(json["total_count"], 1);
        assert_eq!(json["token_estimate"], 50);
        assert_eq!(json["execution_time_ms"], 25);
        assert!(json.get("containers").is_none()); // Skipped when None
    }

    #[test]
    fn test_atoms_result_with_containers() {
        let result = AtomsResult {
            locale: "es-MX".to_string(),
            atoms: vec![],
            containers: Some(vec![AtomContainer {
                key: "terms-es".to_string(),
                container_type: "TermSet".to_string(),
                domain: Some("general".to_string()),
                atom_count: 100,
            }]),
            total_count: 0,
            token_estimate: 20,
            execution_time_ms: 10,
        };
        let json = serde_json::to_value(&result).unwrap();
        let containers = json["containers"].as_array().unwrap();
        assert_eq!(containers.len(), 1);
        assert_eq!(containers[0]["key"], "terms-es");
    }

    // ══════════════════════════════════════════════════════════════
    // CONFIG TESTS
    // ══════════════════════════════════════════════════════════════

    #[test]
    fn test_term_config_values() {
        assert_eq!(TERM_CONFIG.locale_arc, "HAS_TERMS");
        assert_eq!(TERM_CONFIG.container_label, "TermSet");
        assert_eq!(TERM_CONFIG.contains_arc, "CONTAINS_TERM");
        assert_eq!(TERM_CONFIG.atom_label, "Term");
        assert_eq!(TERM_CONFIG.atom_type_name, "Term");
        assert_eq!(TERM_CONFIG.value_property, "value");
        assert_eq!(TERM_CONFIG.filter_field, Some("domain"));
    }

    #[test]
    fn test_expression_config_values() {
        assert_eq!(EXPRESSION_CONFIG.locale_arc, "HAS_EXPRESSIONS");
        assert_eq!(EXPRESSION_CONFIG.container_label, "ExpressionSet");
        assert_eq!(EXPRESSION_CONFIG.filter_field, Some("register"));
    }

    #[test]
    fn test_pattern_config_has_no_filter_field() {
        assert_eq!(PATTERN_CONFIG.filter_field, None);
        assert_eq!(PATTERN_CONFIG.value_property, "template");
    }

    #[test]
    fn test_culture_ref_config_extra_properties() {
        assert!(CULTURE_REF_CONFIG.extra_properties.contains(&("context", "context")));
        assert!(CULTURE_REF_CONFIG
            .extra_properties
            .contains(&("appropriateness", "appropriateness")));
    }

    #[test]
    fn test_taboo_config_extra_properties() {
        assert!(TABOO_CONFIG.extra_properties.contains(&("severity", "severity")));
        assert!(TABOO_CONFIG.extra_properties.contains(&("category", "category")));
    }

    #[test]
    fn test_audience_trait_config_extra_properties() {
        assert!(AUDIENCE_TRAIT_CONFIG
            .extra_properties
            .contains(&("demographic", "demographic")));
        assert!(AUDIENCE_TRAIT_CONFIG
            .extra_properties
            .contains(&("behavior", "behavior")));
    }
}
