//! MCP Resources module
//!
//! Provides read-only access to NovaNet knowledge graph data via MCP resources.
//!
//! Resources:
//! - `entity://{key}` - Entity with localized content
//! - `class://{name}` - NodeClass definition from schema-graph (v0.12.0: was kind://)
//! - `locale://{key}` - Locale configuration and knowledge summary
//! - `view://{id}` - Saved view/query definition

use crate::error::Result;
use crate::server::State;
use serde::{Deserialize, Serialize};

/// Resource types available in NovaNet MCP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    /// Entity resource: entity://{key}
    Entity,
    /// Class resource: class://{name} (v0.12.0: was Kind)
    Class,
    /// Locale resource: locale://{key}
    Locale,
    /// View resource: view://{id}
    View,
}

impl ResourceType {
    /// Parse a resource URI into its type and key
    #[allow(clippy::manual_map)] // Consistent if-else chain is more readable here
    pub fn parse_uri(uri: &str) -> Option<(Self, String)> {
        if let Some(key) = uri.strip_prefix("entity://") {
            Some((Self::Entity, key.to_string()))
        } else if let Some(name) = uri.strip_prefix("class://") {
            Some((Self::Class, name.to_string()))
        } else if let Some(key) = uri.strip_prefix("locale://") {
            Some((Self::Locale, key.to_string()))
        } else if let Some(id) = uri.strip_prefix("view://") {
            Some((Self::View, id.to_string()))
        } else {
            None
        }
    }

    /// Get the scheme for this resource type
    pub fn scheme(&self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Class => "class",
            Self::Locale => "locale",
            Self::View => "view",
        }
    }
}

/// Entity resource data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityResource {
    /// Entity key
    pub key: String,
    /// Entity name (universal, not locale-specific)
    pub name: String,
    /// Entity definition
    pub definition: Option<String>,
    /// Entity category
    pub category: Option<String>,
    /// Localized content (keyed by locale)
    pub content: Vec<LocalizedContent>,
    /// Related entities
    pub related: Vec<RelatedEntity>,
}

/// Localized entity content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedContent {
    /// Locale key
    pub locale: String,
    /// Localized name
    pub name: Option<String>,
    /// Localized description
    pub description: Option<String>,
}

/// Related entity reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedEntity {
    /// Related entity key
    pub key: String,
    /// Related entity name
    pub name: String,
    /// Relationship type
    pub relationship: String,
    /// Direction (outgoing/incoming)
    pub direction: String,
}

/// Class resource data (v0.12.0: was KindResource)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassResource {
    /// Class name
    pub name: String,
    /// Display name
    pub display_name: Option<String>,
    /// Realm (shared/org)
    pub realm: String,
    /// Layer
    pub layer: String,
    /// Trait
    pub trait_type: String,
    /// Description
    pub description: Option<String>,
    /// LLM context hint
    pub llm_context: Option<String>,
    /// Properties schema
    pub properties: Vec<PropertyDefinition>,
    /// Outgoing arc classes
    pub outgoing_arcs: Vec<String>,
    /// Incoming arc classes
    pub incoming_arcs: Vec<String>,
    /// Instance count
    pub instance_count: usize,
}

/// Property definition for a Class
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    /// Property name
    pub name: String,
    /// Property type
    pub property_type: String,
    /// Is required
    pub required: bool,
    /// Description
    pub description: Option<String>,
}

/// Locale resource data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleResource {
    /// Locale key (BCP-47)
    pub key: String,
    /// Language name
    pub language: String,
    /// Region
    pub region: Option<String>,
    /// Script
    pub script: Option<String>,
    /// Writing direction
    pub direction: Option<String>,
    /// Knowledge summary
    pub knowledge_summary: LocaleKnowledgeSummary,
}

/// Summary of locale knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleKnowledgeSummary {
    /// Number of terms
    pub term_count: usize,
    /// Number of expressions
    pub expression_count: usize,
    /// Number of patterns
    pub pattern_count: usize,
    /// Number of culture refs
    pub culture_ref_count: usize,
    /// Number of taboos
    pub taboo_count: usize,
    /// Number of audience traits
    pub audience_trait_count: usize,
}

/// View resource data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewResource {
    /// View ID
    pub id: String,
    /// View name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Category
    pub category: String,
    /// Cypher query template
    pub cypher: String,
    /// Required parameters
    pub parameters: Vec<ViewParameter>,
}

/// View parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewParameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Is required
    pub required: bool,
    /// Default value
    pub default: Option<String>,
}

/// Fetch an entity resource
pub async fn fetch_entity(state: &State, key: &str) -> Result<EntityResource> {
    let query = r#"
        MATCH (e:Entity {key: $key})
        OPTIONAL MATCH (e)-[:BELONGS_TO]->(c:EntityCategory)
        OPTIONAL MATCH (e)-[:HAS_CONTENT]->(ec:EntityContent)
        OPTIONAL MATCH (e)-[r1]->(related:Entity)
        OPTIONAL MATCH (e)<-[r2]-(incoming:Entity)
        RETURN e.key AS key,
               e.name AS name,
               e.definition AS definition,
               c.category_key AS category,
               collect(DISTINCT {locale: ec.locale, name: ec.name, description: ec.description}) AS content,
               collect(DISTINCT {key: related.key, name: related.name, rel: type(r1), dir: 'outgoing'}) AS outgoing,
               collect(DISTINCT {key: incoming.key, name: incoming.name, rel: type(r2), dir: 'incoming'}) AS incoming
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(key));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    let row = rows
        .first()
        .ok_or_else(|| crate::error::Error::not_found(key))?;

    let content: Vec<LocalizedContent> = row["content"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|c| {
                    Some(LocalizedContent {
                        locale: c["locale"].as_str()?.to_string(),
                        name: c["name"].as_str().map(|s| s.to_string()),
                        description: c["description"].as_str().map(|s| s.to_string()),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let mut related = Vec::new();
    if let Some(outgoing) = row["outgoing"].as_array() {
        for r in outgoing {
            if let (Some(k), Some(n), Some(rel)) =
                (r["key"].as_str(), r["name"].as_str(), r["rel"].as_str())
            {
                related.push(RelatedEntity {
                    key: k.to_string(),
                    name: n.to_string(),
                    relationship: rel.to_string(),
                    direction: "outgoing".to_string(),
                });
            }
        }
    }
    if let Some(incoming) = row["incoming"].as_array() {
        for r in incoming {
            if let (Some(k), Some(n), Some(rel)) =
                (r["key"].as_str(), r["name"].as_str(), r["rel"].as_str())
            {
                related.push(RelatedEntity {
                    key: k.to_string(),
                    name: n.to_string(),
                    relationship: rel.to_string(),
                    direction: "incoming".to_string(),
                });
            }
        }
    }

    Ok(EntityResource {
        key: row["key"].as_str().unwrap_or(key).to_string(),
        name: row["name"].as_str().unwrap_or_default().to_string(),
        definition: row["definition"].as_str().map(|s| s.to_string()),
        category: row["category"].as_str().map(|s| s.to_string()),
        content,
        related,
    })
}

/// Fetch a class resource (v0.12.0: was fetch_kind)
pub async fn fetch_class(state: &State, name: &str) -> Result<ClassResource> {
    let query = r#"
        MATCH (c:Class {name: $name})
        OPTIONAL MATCH (c)<-[:OF_CLASS]-(instance)
        WITH c, count(instance) AS instance_count
        OPTIONAL MATCH (a:ArcClass)
        WHERE a.source = c.name OR a.target = c.name
        WITH c, instance_count, collect(DISTINCT a) AS arcs
        RETURN c.name AS name,
               c.display_name AS display_name,
               c.realm AS realm,
               c.layer AS layer,
               c.trait AS trait_type,
               c.description AS description,
               c.llm_context AS llm_context,
               c.properties AS properties,
               instance_count,
               [a IN arcs WHERE a.source = c.name | a.name] AS outgoing_arcs,
               [a IN arcs WHERE a.target = c.name | a.name] AS incoming_arcs
    "#;

    let mut params = serde_json::Map::new();
    params.insert("name".to_string(), serde_json::json!(name));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    let row = rows
        .first()
        .ok_or_else(|| crate::error::Error::not_found(name))?;

    let properties: Vec<PropertyDefinition> = row["properties"]
        .as_object()
        .map(|obj| {
            obj.iter()
                .map(|(name, val)| PropertyDefinition {
                    name: name.clone(),
                    property_type: val["type"].as_str().unwrap_or("string").to_string(),
                    required: val["required"].as_bool().unwrap_or(false),
                    description: val["description"].as_str().map(|s| s.to_string()),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(ClassResource {
        name: row["name"].as_str().unwrap_or(name).to_string(),
        display_name: row["display_name"].as_str().map(|s| s.to_string()),
        realm: row["realm"].as_str().unwrap_or("unknown").to_string(),
        layer: row["layer"].as_str().unwrap_or("unknown").to_string(),
        trait_type: row["trait_type"].as_str().unwrap_or("unknown").to_string(),
        description: row["description"].as_str().map(|s| s.to_string()),
        llm_context: row["llm_context"].as_str().map(|s| s.to_string()),
        properties,
        outgoing_arcs: row["outgoing_arcs"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default(),
        incoming_arcs: row["incoming_arcs"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default(),
        instance_count: row["instance_count"].as_u64().unwrap_or(0) as usize,
    })
}

/// Fetch a locale resource
pub async fn fetch_locale(state: &State, key: &str) -> Result<LocaleResource> {
    let query = r#"
        MATCH (l:Locale {key: $key})
        OPTIONAL MATCH (l)-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)
        WITH l, count(t) AS term_count
        OPTIONAL MATCH (l)-[:HAS_EXPRESSIONS]->(es:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e:Expression)
        WITH l, term_count, count(e) AS expr_count
        OPTIONAL MATCH (l)-[:HAS_PATTERNS]->(ps:PatternSet)-[:CONTAINS_PATTERN]->(p:Pattern)
        WITH l, term_count, expr_count, count(p) AS pattern_count
        OPTIONAL MATCH (l)-[:HAS_CULTURE]->(cs:CultureSet)-[:CONTAINS_CULTURE_REF]->(c:CultureRef)
        WITH l, term_count, expr_count, pattern_count, count(c) AS culture_count
        OPTIONAL MATCH (l)-[:HAS_TABOOS]->(tbs:TabooSet)-[:CONTAINS_TABOO]->(tb:Taboo)
        WITH l, term_count, expr_count, pattern_count, culture_count, count(tb) AS taboo_count
        OPTIONAL MATCH (l)-[:HAS_AUDIENCE]->(as:AudienceSet)-[:CONTAINS_AUDIENCE_TRAIT]->(a:AudienceTrait)
        RETURN l.key AS key,
               l.language AS language,
               l.region AS region,
               l.script AS script,
               l.direction AS direction,
               term_count, expr_count, pattern_count, culture_count, taboo_count, count(a) AS audience_count
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(key));

    let rows = state.pool().execute_query(query, Some(params)).await?;

    let row = rows
        .first()
        .ok_or_else(|| crate::error::Error::not_found(key))?;

    Ok(LocaleResource {
        key: row["key"].as_str().unwrap_or(key).to_string(),
        language: row["language"].as_str().unwrap_or("unknown").to_string(),
        region: row["region"].as_str().map(|s| s.to_string()),
        script: row["script"].as_str().map(|s| s.to_string()),
        direction: row["direction"].as_str().map(|s| s.to_string()),
        knowledge_summary: LocaleKnowledgeSummary {
            term_count: row["term_count"].as_u64().unwrap_or(0) as usize,
            expression_count: row["expr_count"].as_u64().unwrap_or(0) as usize,
            pattern_count: row["pattern_count"].as_u64().unwrap_or(0) as usize,
            culture_ref_count: row["culture_count"].as_u64().unwrap_or(0) as usize,
            taboo_count: row["taboo_count"].as_u64().unwrap_or(0) as usize,
            audience_trait_count: row["audience_count"].as_u64().unwrap_or(0) as usize,
        },
    })
}

/// Fetch a view resource
pub async fn fetch_view(_state: &State, id: &str) -> Result<ViewResource> {
    // Views are defined in YAML, not in Neo4j
    // This is a placeholder that returns a basic view structure
    // In a full implementation, this would read from a view registry

    Ok(ViewResource {
        id: id.to_string(),
        name: id.to_string(),
        description: Some(format!("View: {}", id)),
        category: "custom".to_string(),
        cypher: format!("// View {} - query not loaded", id),
        parameters: vec![],
    })
}

/// List available resources of a given type
pub async fn list_resources(state: &State, resource_type: ResourceType) -> Result<Vec<String>> {
    let query = match resource_type {
        ResourceType::Entity => "MATCH (e:Entity) RETURN e.key AS key ORDER BY key LIMIT 100",
        ResourceType::Class => "MATCH (c:Class) RETURN c.name AS key ORDER BY key",
        ResourceType::Locale => "MATCH (l:Locale) RETURN l.key AS key ORDER BY key",
        ResourceType::View => {
            // Views are not stored in Neo4j, return empty for now
            return Ok(vec![]);
        }
    };

    let rows = state.pool().execute_query(query, None).await?;

    Ok(rows
        .into_iter()
        .filter_map(|row| row["key"].as_str().map(|s| s.to_string()))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uri_entity() {
        let (rt, key) = ResourceType::parse_uri("entity://my-entity").unwrap();
        assert_eq!(rt, ResourceType::Entity);
        assert_eq!(key, "my-entity");
    }

    #[test]
    fn test_parse_uri_class() {
        let (rt, key) = ResourceType::parse_uri("class://Entity").unwrap();
        assert_eq!(rt, ResourceType::Class);
        assert_eq!(key, "Entity");
    }

    #[test]
    fn test_parse_uri_locale() {
        let (rt, key) = ResourceType::parse_uri("locale://fr-FR").unwrap();
        assert_eq!(rt, ResourceType::Locale);
        assert_eq!(key, "fr-FR");
    }

    #[test]
    fn test_parse_uri_view() {
        let (rt, key) = ResourceType::parse_uri("view://composition").unwrap();
        assert_eq!(rt, ResourceType::View);
        assert_eq!(key, "composition");
    }

    #[test]
    fn test_parse_uri_invalid() {
        assert!(ResourceType::parse_uri("invalid://test").is_none());
        assert!(ResourceType::parse_uri("https://example.com").is_none());
    }

    #[test]
    fn test_resource_type_scheme() {
        assert_eq!(ResourceType::Entity.scheme(), "entity");
        assert_eq!(ResourceType::Class.scheme(), "class");
        assert_eq!(ResourceType::Locale.scheme(), "locale");
        assert_eq!(ResourceType::View.scheme(), "view");
    }
}
