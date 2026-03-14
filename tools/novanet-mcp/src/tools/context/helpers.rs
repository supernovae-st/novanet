//! Internal helper functions for novanet_context tool
//!
//! All functions are `pub(super)` — only called from `mod.rs`.

use crate::error::Result;
use crate::server::State;
use std::collections::HashMap;
use std::fmt::Write;

use super::types::*;

// =============================================================================
// PAGE/BLOCK HELPERS — Structure discovery
// =============================================================================

/// Discover page structure by walking ownership arcs
pub(super) async fn get_structure(
    state: &State,
    focus_key: &str,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (p {key: $key})-[:HAS_BLOCK]->(b)
        OPTIONAL MATCH (b)-[:OF_TYPE]->(bt)
        RETURN b.key AS key, labels(b)[0] AS kind, b.display_name AS name,
               b.content AS content, bt.key AS block_type
        ORDER BY b.position
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let content = row["content"]
                .as_str()
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>();
            let tokens = content.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: row["kind"].as_str().unwrap_or("Block").to_string(),
                evidence_type: "structure".to_string(),
                distance: 1,
                relevance: 1.0 - (i as f64 * 0.05).min(0.5),
                content,
                tokens,
            }
        })
        .collect())
}

/// Assemble entity evidence for page/block modes
pub(super) async fn assemble_entities_internal(
    state: &State,
    focus_key: &str,
    locale: &str,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (n {key: $key})-[:USES_ENTITY|HAS_ENTITY*1..2]->(e:Entity)
        OPTIONAL MATCH (e)-[:HAS_NATIVE]->(en:EntityNative)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        RETURN DISTINCT e.key AS key, e.display_name AS name, e.content AS content,
               en.denomination_forms AS forms
        LIMIT 20
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let content = row["content"]
                .as_str()
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>();
            let tokens = content.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: "Entity".to_string(),
                evidence_type: "entity".to_string(),
                distance: 1,
                relevance: 0.95 - (i as f64 * 0.05).min(0.4),
                content,
                tokens,
            }
        })
        .collect())
}

/// Assemble knowledge atoms as evidence for page/block modes
pub(super) async fn assemble_knowledge_internal(
    state: &State,
    locale: &str,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (l:Locale {key: $locale})-[:HAS_EXPRESSIONS]->(es:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e:Expression)
        RETURN e.key AS key, e.value AS value
        LIMIT 20
    "#;

    let mut params = serde_json::Map::new();
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let value = row["value"].as_str().unwrap_or_default().to_string();
            let tokens = value.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: "Expression".to_string(),
                evidence_type: "knowledge".to_string(),
                distance: 2,
                relevance: 0.7 - (i as f64 * 0.02).min(0.3),
                content: value,
                tokens,
            }
        })
        .collect())
}

/// Get context anchors (cross-page references)
pub(super) async fn get_context_anchors(
    state: &State,
    focus_key: &str,
    locale: &str,
) -> Result<Vec<ContextAnchor>> {
    let cypher = r#"
        MATCH (b {key: $key})-[:REFERENCES_PAGE]->(p:Page)
        OPTIONAL MATCH (p)-[:HAS_NATIVE]->(pn:PageNative)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        RETURN p.key AS page_key, p.display_name AS name,
               pn.slug AS slug, pn.meta_description AS hint
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .map(|row| ContextAnchor {
            page_key: row["page_key"].as_str().unwrap_or_default().to_string(),
            anchor_text: row["name"].as_str().unwrap_or_default().to_string(),
            slug: row["slug"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            context_hint: row["hint"]
                .as_str()
                .unwrap_or("Link when relevant")
                .to_string(),
        })
        .collect())
}

/// Fetch denomination forms (ADR-033)
pub(super) async fn fetch_denomination_forms(
    state: &State,
    focus_key: &str,
    locale: &str,
) -> Result<HashMap<String, DenominationForm>> {
    let cypher = r#"
        MATCH (n {key: $key})-[:USES_ENTITY|HAS_ENTITY*1..2]->(e:Entity)
        MATCH (e)-[:HAS_NATIVE]->(en:EntityNative)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        WHERE en.denomination_forms IS NOT NULL
        RETURN e.key AS entity_key, en.denomination_forms AS forms
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    let mut forms_map = HashMap::new();
    for row in rows {
        let entity_key = row["entity_key"].as_str().unwrap_or_default().to_string();
        if let Some(forms_val) = row.get("forms") {
            // Parse denomination_forms from JSON array
            if let Some(forms_arr) = forms_val.as_array() {
                let mut form = DenominationForm {
                    text: None,
                    title: None,
                    abbrev: None,
                    url: None,
                    mixed: None,
                    base: None,
                };
                for f in forms_arr {
                    let form_type = f["type"].as_str().unwrap_or_default();
                    let value = f["value"].as_str().unwrap_or_default().to_string();
                    match form_type {
                        "text" => form.text = Some(value),
                        "title" => form.title = Some(value),
                        "abbrev" => form.abbrev = Some(value),
                        "url" => form.url = Some(value),
                        "mixed" => form.mixed = Some(value),
                        "base" => form.base = Some(value),
                        _ => {}
                    }
                }
                forms_map.insert(entity_key, form);
            }
        }
    }

    Ok(forms_map)
}

// =============================================================================
// ASSEMBLE MODE HELPERS
// =============================================================================

/// Get focus node info
pub(super) async fn get_focus_node(state: &State, key: &str) -> Result<FocusNode> {
    let cypher = r#"
        MATCH (n {key: $key})
        RETURN n.key AS key, labels(n)[0] AS kind, n.display_name AS name,
               n.content AS description
        LIMIT 1
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(key));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .first()
        .map(|row| FocusNode {
            key: row["key"].as_str().unwrap_or_default().to_string(),
            kind: row["kind"].as_str().unwrap_or("Unknown").to_string(),
            name: row["name"].as_str().unwrap_or_default().to_string(),
            description: row["description"].as_str().map(|s| s.to_string()),
        })
        .unwrap_or(FocusNode {
            key: key.to_string(),
            kind: "Unknown".to_string(),
            name: key.to_string(),
            description: None,
        }))
}

/// Assemble entity evidence packets (for assemble mode)
pub(super) async fn assemble_entities_for_focus(
    state: &State,
    focus_key: &str,
    locale: &str,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (n {key: $key})-[:USES_ENTITY|HAS_ENTITY|HAS_BLOCK*1..3]->(e:Entity)
        OPTIONAL MATCH (e)-[:HAS_NATIVE]->(en:EntityNative)-[:FOR_LOCALE]->(l:Locale {key: $locale})
        RETURN DISTINCT e.key AS key, e.content AS content, en.content AS native_content
        LIMIT 30
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let content = row["native_content"]
                .as_str()
                .or_else(|| row["content"].as_str())
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>();
            let tokens = content.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: "Entity".to_string(),
                evidence_type: "definition".to_string(),
                distance: 1 + i.min(3),
                relevance: 0.9 - (i as f64 * 0.03).min(0.5),
                content,
                tokens,
            }
        })
        .collect())
}

/// Assemble knowledge evidence packets (for assemble mode)
pub(super) async fn assemble_knowledge_for_focus(
    state: &State,
    locale: &str,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (l:Locale {key: $locale})-[:HAS_EXPRESSIONS]->(es)-[:CONTAINS_EXPRESSION]->(e:Expression)
        RETURN e.key AS key, e.value AS value
        LIMIT 15
        UNION ALL
        MATCH (l:Locale {key: $locale})-[:HAS_PATTERNS]->(ps)-[:CONTAINS_PATTERN]->(p:Pattern)
        RETURN p.key AS key, p.template AS value
        LIMIT 10
    "#;

    let mut params = serde_json::Map::new();
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let value = row["value"].as_str().unwrap_or_default().to_string();
            let tokens = value.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: "Knowledge".to_string(),
                evidence_type: "knowledge".to_string(),
                distance: 2,
                relevance: 0.6 - (i as f64 * 0.01).min(0.3),
                content: value,
                tokens,
            }
        })
        .collect())
}

/// Assemble structure evidence packets (for assemble mode)
pub(super) async fn assemble_structure_for_focus(
    state: &State,
    focus_key: &str,
) -> Result<Vec<EvidencePacket>> {
    let cypher = r#"
        MATCH (n {key: $key})-[:HAS_BLOCK|HAS_PAGE*1..2]->(child)
        RETURN child.key AS key, labels(child)[0] AS kind, child.content AS content
        LIMIT 20
    "#;

    let mut params = serde_json::Map::new();
    params.insert("key".to_string(), serde_json::json!(focus_key));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let content = row["content"]
                .as_str()
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>();
            let tokens = content.len().div_ceil(4);
            EvidencePacket {
                source_key: row["key"].as_str().unwrap_or_default().to_string(),
                source_kind: row["kind"].as_str().unwrap_or("Unknown").to_string(),
                evidence_type: "structure".to_string(),
                distance: 1,
                relevance: 0.8 - (i as f64 * 0.04).min(0.4),
                content,
                tokens,
            }
        })
        .collect())
}

// =============================================================================
// KNOWLEDGE MODE HELPERS — Atom fetching
// =============================================================================

/// Generic function to fetch atoms of any type using configuration
pub(super) async fn fetch_atoms(
    state: &State,
    params: &ContextParams,
    config: &AtomConfig,
    limit: usize,
) -> Result<Vec<Atom>> {
    // Build filter for domain/register (if applicable) using parameterized queries
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
pub(super) async fn get_containers(state: &State, locale: &str) -> Result<Vec<AtomContainer>> {
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

// =============================================================================
// SHARED HELPERS
// =============================================================================

/// Get locale context information
pub(super) async fn get_locale_context(state: &State, locale: &str) -> Result<LocaleContext> {
    let cypher = r#"
        MATCH (l:Locale {key: $locale})
        RETURN l.key AS key, l.language AS language, l.region AS region,
               l.voice AS voice, l.formatting AS formatting
    "#;

    let mut params = serde_json::Map::new();
    params.insert("locale".to_string(), serde_json::json!(locale));

    let rows = state.pool().execute_query(cypher, Some(params)).await?;

    Ok(rows
        .first()
        .map(|row| LocaleContext {
            locale_key: row["key"].as_str().unwrap_or(locale).to_string(),
            language: row["language"].as_str().unwrap_or("Unknown").to_string(),
            region: row["region"].as_str().map(|s| s.to_string()),
            voice: row["voice"].as_str().map(|s| s.to_string()),
            formatting: row["formatting"].as_str().map(|s| s.to_string()),
        })
        .unwrap_or(LocaleContext {
            locale_key: locale.to_string(),
            language: "Unknown".to_string(),
            region: None,
            voice: None,
            formatting: None,
        }))
}

/// Estimate token count for locale context
pub(super) fn estimate_locale_tokens(ctx: &LocaleContext) -> usize {
    let mut total = ctx.locale_key.len() + ctx.language.len();
    if let Some(ref r) = ctx.region {
        total += r.len();
    }
    if let Some(ref v) = ctx.voice {
        total += v.len();
    }
    if let Some(ref f) = ctx.formatting {
        total += f.len();
    }
    total.div_ceil(4)
}

/// Build generation prompt from assembled context
#[allow(clippy::too_many_arguments)]
pub(super) fn build_prompt(
    focus_key: &str,
    locale: &str,
    mode: &str,
    locale_ctx: &LocaleContext,
    entities: &[EvidencePacket],
    atoms: &[EvidencePacket],
    anchors: &[ContextAnchor],
    denomination_forms: &HashMap<String, DenominationForm>,
) -> String {
    let mut prompt = String::with_capacity(4096);

    let _ = writeln!(prompt, "# Generation Context for {} ({})", focus_key, locale);
    let _ = writeln!(prompt, "## Mode: {}\n", mode);

    // Locale
    let _ = writeln!(prompt, "## Locale: {}", locale_ctx.language);
    if let Some(ref region) = locale_ctx.region {
        let _ = writeln!(prompt, "Region: {}", region);
    }
    if let Some(ref voice) = locale_ctx.voice {
        let _ = writeln!(prompt, "Voice: {}", voice);
    }
    let _ = writeln!(prompt);

    // Denomination forms (ADR-033)
    if !denomination_forms.is_empty() {
        let _ = writeln!(prompt, "## Entity Reference Forms (ADR-033)");
        let _ = writeln!(prompt, "Use ONLY these forms when referencing entities:\n");
        for (entity_key, form) in denomination_forms {
            let _ = writeln!(prompt, "### {}", entity_key);
            if let Some(ref text) = form.text {
                let _ = writeln!(prompt, "- text (body): {}", text);
            }
            if let Some(ref title) = form.title {
                let _ = writeln!(prompt, "- title (headings): {}", title);
            }
            if let Some(ref abbrev) = form.abbrev {
                let _ = writeln!(prompt, "- abbrev (after first): {}", abbrev);
            }
            if let Some(ref url) = form.url {
                let _ = writeln!(prompt, "- url (slugs): {}", url);
            }
            let _ = writeln!(prompt);
        }
    }

    // Entities
    if !entities.is_empty() {
        let _ = writeln!(prompt, "## Entities ({} loaded)\n", entities.len());
        for e in entities {
            let _ = writeln!(prompt, "- **{}**: {}", e.source_key, e.content);
        }
        let _ = writeln!(prompt);
    }

    // Knowledge atoms
    if !atoms.is_empty() {
        let _ = writeln!(prompt, "## Knowledge Atoms ({} loaded)\n", atoms.len());
        for a in atoms {
            let _ = writeln!(prompt, "- {}: {}", a.source_key, a.content);
        }
        let _ = writeln!(prompt);
    }

    // Anchors
    if !anchors.is_empty() {
        let _ = writeln!(prompt, "## Cross-Page Links\n");
        let _ = writeln!(
            prompt,
            "Use {{{{anchor:key|text}}}} syntax for internal links:\n"
        );
        for a in anchors {
            let _ = writeln!(
                prompt,
                "- {{{{anchor:{}|{}}}}} → {} ({})",
                a.page_key, a.anchor_text, a.slug, a.context_hint
            );
        }
    }

    prompt
}
