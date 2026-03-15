//! Conversion utilities for TUI data.
//!
//! Bolt-to-JSON conversion, label validation, formatting helpers,
//! and shared parsing helpers for Neo4j query results.

use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

use super::types::InstanceArc;

/// Maximum number of instances to load per Class.
/// Reduced from 500 to 300 for better performance with large datasets.
pub const INSTANCE_LIMIT: usize = 300;

// =============================================================================
// SECURITY: Label validation for Cypher injection prevention
// =============================================================================

/// Validates that a Neo4j label is safe for interpolation into Cypher queries.
/// Labels must be alphanumeric (with underscores allowed) and non-empty.
///
/// While our data comes from the schema graph (not direct user input), this provides
/// defense-in-depth against potential injection if the database were compromised.
pub(crate) fn validate_cypher_label(label: &str) -> crate::Result<()> {
    if label.is_empty() {
        return Err(crate::error::NovaNetError::Validation(
            "Empty label not allowed in Cypher queries".into(),
        ));
    }
    // Allow alphanumeric, underscore, and dash (common in NovaNet labels like "locale-knowledge")
    if !label
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(crate::error::NovaNetError::Validation(format!(
            "Invalid characters in label '{}' - only alphanumeric, underscore, and dash allowed",
            label
        )));
    }
    Ok(())
}

/// Clean up Bolt debug output by removing wrapper type names.
/// E.g., "DateTime(BoltDateTime { seconds: BoltInteger { value: 123 }, ... })" -> "123"
fn clean_bolt_debug(debug: &str) -> String {
    const PATTERN: &str = "seconds: BoltInteger { value: ";
    // Extract just the timestamp if it's a DateTime
    // Use find() which returns byte index, but pattern is ASCII so addition is safe
    if let Some(start_byte) = debug.find(PATTERN) {
        // Pattern is ASCII-only, so start_byte + PATTERN.len() is a valid char boundary
        let rest = &debug[start_byte + PATTERN.len()..];
        // Find the end of the value (space or comma) - use chars for safety
        let value: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !value.is_empty() {
            return value;
        }
    }
    // Fallback: just return the debug string but truncated
    debug.chars().take(50).collect()
}

/// Convert a neo4rs BoltType to a serde_json::Value for clean display.
/// This extracts actual values instead of showing Bolt wrapper types.
pub(crate) fn bolt_to_json(bolt: &neo4rs::BoltType) -> JsonValue {
    use neo4rs::BoltType;
    match bolt {
        BoltType::String(s) => JsonValue::String(s.value.clone()),
        BoltType::Integer(i) => JsonValue::Number(i.value.into()),
        BoltType::Float(f) => serde_json::Number::from_f64(f.value)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        BoltType::Boolean(b) => JsonValue::Bool(b.value),
        BoltType::Null(_) => JsonValue::Null,
        BoltType::List(list) => JsonValue::Array(list.value.iter().map(bolt_to_json).collect()),
        BoltType::Map(map) => {
            let obj: serde_json::Map<String, JsonValue> = map
                .value
                .iter()
                .map(|(k, v)| (k.value.clone(), bolt_to_json(v)))
                .collect();
            JsonValue::Object(obj)
        },
        // For complex types (Node, Relationship, etc.), show a simplified representation
        BoltType::Node(n) => {
            let mut obj = serde_json::Map::new();
            obj.insert("_type".to_string(), JsonValue::String("Node".to_string()));
            obj.insert(
                "_labels".to_string(),
                JsonValue::Array(
                    n.labels
                        .iter()
                        .map(|l| JsonValue::String(l.to_string()))
                        .collect(),
                ),
            );
            for (k, v) in &n.properties.value {
                obj.insert(k.value.clone(), bolt_to_json(v));
            }
            JsonValue::Object(obj)
        },
        BoltType::Relation(r) => {
            let mut obj = serde_json::Map::new();
            obj.insert(
                "_type".to_string(),
                JsonValue::String("Relationship".to_string()),
            );
            obj.insert(
                "_rel_type".to_string(),
                JsonValue::String(r.typ.value.clone()),
            );
            JsonValue::Object(obj)
        },
        // DateTime and other complex types - extract what we can
        BoltType::DateTime(_)
        | BoltType::LocalDateTime(_)
        | BoltType::DateTimeZoneId(_)
        | BoltType::Date(_)
        | BoltType::Time(_)
        | BoltType::LocalTime(_)
        | BoltType::Duration(_)
        | BoltType::Point2D(_)
        | BoltType::Point3D(_)
        | BoltType::Path(_)
        | BoltType::UnboundedRelation(_)
        | BoltType::Bytes(_) => {
            // Clean up debug output: extract useful info
            let debug = format!("{:?}", bolt);
            JsonValue::String(clean_bolt_debug(&debug))
        },
    }
}

/// Convert a locale code to a flag emoji (e.g., "fr-FR" → "🇫🇷").
///
/// Extracts the country code from the locale (the part after the hyphen)
/// and converts it to regional indicator symbols.
pub fn locale_to_flag(locale: &str) -> String {
    // Extract country code (e.g., "fr-FR" → "FR", "es-MX" → "MX")
    let country = locale.split('-').nth(1).unwrap_or(locale).to_uppercase();

    if country.len() != 2 {
        return "🏳️".to_string(); // Fallback for invalid codes
    }

    // Convert to regional indicator symbols (A = 🇦, B = 🇧, etc.)
    // Regional indicators start at U+1F1E6 (🇦)
    country
        .chars()
        .filter_map(|c| {
            if c.is_ascii_uppercase() {
                // 'A' is 65, regional indicator 🇦 is U+1F1E6 (127462)
                let offset = c as u32 - 'A' as u32;
                char::from_u32(0x1F1E6 + offset)
            } else {
                None
            }
        })
        .collect()
}

/// Get icon for realm (v11.4: 2 realms - shared + org).
/// Uses unicode symbols instead of emoji for terminal compatibility.
pub(crate) fn realm_icon(key: &str) -> &'static str {
    match key {
        "shared" => "◉", // filled circle - universal/shared
        "org" => "◎",    // circle with dot - scoped/owned
        _ => "○",        // empty circle - unknown
    }
}

/// Convert PascalCase to kebab-case (e.g., "BlockNative" -> "block-native").
/// Pre-allocates capacity to avoid reallocations.
pub(crate) fn to_kebab_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4); // +4 for potential dashes
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('-');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

// =============================================================================
// Shared Neo4j row parsing helpers (used by queries_instances + queries_entities)
// =============================================================================

/// Parse outgoing arcs from a Cypher row's "outgoing" column.
///
/// Expects a `Vec<BoltMap>` with keys: arc_type, target_key, target_class,
/// target_display_name, target_slug. Filters out entries with empty arc_type.
pub(crate) fn parse_outgoing_arcs(row: &neo4rs::Row) -> Vec<InstanceArc> {
    row.get::<Vec<neo4rs::BoltMap>>("outgoing")
        .unwrap_or_default()
        .into_iter()
        .filter_map(|m| {
            let arc_type = m.get::<String>("arc_type").ok()?;
            if arc_type.is_empty() {
                return None;
            }
            Some(InstanceArc {
                arc_type,
                target_key: m.get("target_key").unwrap_or_default(),
                target_class: m.get("target_class").unwrap_or_default(),
                exists: true,
                target_display_name: m.get("target_display_name").ok(),
                target_slug: m.get("target_slug").ok(),
            })
        })
        .collect()
}

/// Parse incoming arcs from a Cypher row's "incoming" column.
///
/// Expects a `Vec<BoltMap>` with keys: arc_type, source_key, source_class,
/// source_display_name, source_slug. Filters out entries with empty arc_type.
pub(crate) fn parse_incoming_arcs(row: &neo4rs::Row) -> Vec<InstanceArc> {
    row.get::<Vec<neo4rs::BoltMap>>("incoming")
        .unwrap_or_default()
        .into_iter()
        .filter_map(|m| {
            let arc_type = m.get::<String>("arc_type").ok()?;
            if arc_type.is_empty() {
                return None;
            }
            Some(InstanceArc {
                arc_type,
                target_key: m.get("source_key").unwrap_or_default(),
                target_class: m.get("source_class").unwrap_or_default(),
                exists: true,
                target_display_name: m.get("source_display_name").ok(),
                target_slug: m.get("source_slug").ok(),
            })
        })
        .collect()
}

/// Parse a BoltMap "props" column into a BTreeMap of JSON values.
pub(crate) fn parse_bolt_props(row: &neo4rs::Row) -> BTreeMap<String, JsonValue> {
    row.get::<neo4rs::BoltMap>("props")
        .map(|m| {
            m.value
                .iter()
                .map(|(k, v)| (k.value.clone(), bolt_to_json(v)))
                .collect()
        })
        .unwrap_or_default()
}

/// Extract entity slug from denomination_forms property (ADR-033).
///
/// Looks for the form with `type: "url"` and returns its value.
pub(crate) fn extract_entity_slug(props: &BTreeMap<String, JsonValue>) -> Option<String> {
    props
        .get("denomination_forms")
        .and_then(|df| df.as_array())
        .and_then(|arr| {
            arr.iter()
                .find(|form| form.get("type").and_then(|t| t.as_str()) == Some("url"))
                .and_then(|form| form.get("value").and_then(|v| v.as_str()))
                .map(|s| s.to_string())
        })
}

/// Calculate relationship_power from HAS_NATIVE arc count.
///
/// Returns 0-100 based on ratio of HAS_NATIVE arcs to expected max (10 locales).
pub(crate) fn relationship_power_from_arcs(outgoing_arcs: &[InstanceArc]) -> u8 {
    let native_count = outgoing_arcs
        .iter()
        .filter(|a| a.arc_type == "HAS_NATIVE")
        .count();
    let max_natives = 10; // Expected max locales
    ((native_count * 100) / max_natives).min(100) as u8
}
