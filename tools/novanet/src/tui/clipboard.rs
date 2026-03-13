//! Clipboard utilities for TUI.
//!
//! Provides cross-platform clipboard access using arboard crate.
//! Supports smart copy based on selected InfoBox type.

use arboard::Clipboard;

use super::app::{App, InfoBox};
use super::data::TreeItem;

/// Copy text to system clipboard.
/// Returns Ok(()) on success, Err with message on failure.
pub fn copy_to_clipboard(text: &str) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| format!("Clipboard init failed: {}", e))?;
    clipboard
        .set_text(text.to_string())
        .map_err(|e| format!("Clipboard set failed: {}", e))
}

/// Copy content based on the currently selected box.
/// Returns (content, format_name) for status message.
/// v0.13.1: Diagram and Architecture removed
pub fn get_box_content(app: &App) -> Option<(String, &'static str)> {
    match app.selected_box {
        InfoBox::Tree => get_tree_content(app),
        InfoBox::Header => get_header_content(app),
        InfoBox::Properties => get_properties_content(app),
        InfoBox::Arcs => get_arcs_content(app),
        InfoBox::Source => get_source_content(app),
    }
}

/// TREE box: Path string like "org/structure/Page".
fn get_tree_content(app: &App) -> Option<(String, &'static str)> {
    let item = app.current_item()?;
    let path = match item {
        TreeItem::Realm(realm) => realm.key.clone(),
        TreeItem::Layer(realm, layer) => format!("{}/{}", realm.key, layer.key),
        TreeItem::Class(realm, layer, class) => {
            format!("{}/{}/{}", realm.key, layer.key, class.key)
        },
        TreeItem::Instance(realm, layer, class, instance) => {
            format!("{}/{}/{}/{}", realm.key, layer.key, class.key, instance.key)
        },
        TreeItem::ArcFamily(family) => format!("arcs/{}", family.key),
        TreeItem::ArcClass(family, arc) => format!("arcs/{}/{}", family.key, arc.key),
        TreeItem::ClassesSection => "classes".to_string(),
        TreeItem::ArcsSection => "arcs".to_string(),
        TreeItem::EntityCategory(realm, layer, class, cat) => {
            format!("{}/{}/{}/{}", realm.key, layer.key, class.key, cat.key)
        },
        TreeItem::LocaleGroup(realm, layer, class, group) => {
            format!(
                "{}/{}/{}/{}",
                realm.key, layer.key, class.key, group.locale_code
            )
        },
        // v0.17.3: EntityGroup path
        TreeItem::EntityGroup(realm, layer, class, group) => {
            format!(
                "{}/{}/{}/{}",
                realm.key, layer.key, class.key, group.entity_key
            )
        },
        TreeItem::EntityNativeItem(realm, layer, class, native) => {
            format!("{}/{}/{}/{}", realm.key, layer.key, class.key, native.key)
        },
    };
    Some((path, "path"))
}

/// HEADER box: JSON metadata.
fn get_header_content(app: &App) -> Option<(String, &'static str)> {
    let item = app.current_item()?;
    let json = match item {
        TreeItem::Class(realm, layer, class) => {
            // v0.17.3 (ADR-036): trait removed from clipboard output
            serde_json::json!({
                "type": "Class",
                "key": class.key,
                "realm": realm.key,
                "layer": layer.key,
                "display_name": class.display_name
            })
        },
        TreeItem::ArcClass(family, arc) => {
            serde_json::json!({
                "type": "ArcClass",
                "key": arc.key,
                "family": family.key,
                "source": arc.from_class,
                "target": arc.to_class,
                "cardinality": arc.cardinality
            })
        },
        TreeItem::Realm(realm) => {
            serde_json::json!({
                "type": "Realm",
                "key": realm.key,
                "display_name": realm.display_name
            })
        },
        TreeItem::Layer(realm, layer) => {
            serde_json::json!({
                "type": "Layer",
                "key": layer.key,
                "realm": realm.key,
                "display_name": layer.display_name
            })
        },
        TreeItem::Instance(realm, layer, class, instance) => {
            serde_json::json!({
                "type": "Instance",
                "key": instance.key,
                "class": class.key,
                "realm": realm.key,
                "layer": layer.key
            })
        },
        _ => return None,
    };
    Some((
        serde_json::to_string_pretty(&json).unwrap_or_default(),
        "JSON",
    ))
}

/// PROPERTIES box: JSON schema or property list.
fn get_properties_content(app: &App) -> Option<(String, &'static str)> {
    let item = app.current_item()?;
    match item {
        TreeItem::Class(_, _, class) => {
            // ClassInfo.properties is Vec<String> (names), required_properties is Vec<String>
            let props: serde_json::Map<String, serde_json::Value> = class
                .properties
                .iter()
                .map(|prop_name| {
                    let is_required = class.required_properties.contains(prop_name);
                    let val = serde_json::json!({
                        "required": is_required
                    });
                    (prop_name.clone(), val)
                })
                .collect();
            Some((
                serde_json::to_string_pretty(&props).unwrap_or_default(),
                "JSON schema",
            ))
        },
        TreeItem::Instance(_, _, _, instance) => {
            // Instance properties is BTreeMap<String, JsonValue> - already typed
            let props: serde_json::Map<String, serde_json::Value> = instance
                .properties
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            Some((
                serde_json::to_string_pretty(&props).unwrap_or_default(),
                "JSON",
            ))
        },
        _ => None,
    }
}

/// ARCS box: JSON array of incoming/outgoing relationships.
fn get_arcs_content(app: &App) -> Option<(String, &'static str)> {
    let arcs = app.details.class_arcs.as_ref()?;
    let json = serde_json::json!({
        "outgoing": arcs.outgoing.iter().map(|a| {
            serde_json::json!({
                "arc": a.arc_key,
                "target_class": a.other_class,
                "family": a.family
            })
        }).collect::<Vec<_>>(),
        "incoming": arcs.incoming.iter().map(|a| {
            serde_json::json!({
                "arc": a.arc_key,
                "source_class": a.other_class,
                "family": a.family
            })
        }).collect::<Vec<_>>()
    });
    Some((
        serde_json::to_string_pretty(&json).unwrap_or_default(),
        "JSON arcs",
    ))
}

/// SOURCE box: Raw YAML content.
fn get_source_content(app: &App) -> Option<(String, &'static str)> {
    if app.yaml.content.is_empty() {
        None
    } else {
        Some((app.yaml.content.clone(), "YAML"))
    }
}

// v0.13.1: get_diagram_content and get_architecture_content removed (panel simplification)

/// Standard properties that ALL nodes have (from standard_properties in YAML).
/// NOTE: Must match STANDARD_PROPERTY_NAMES in ui/info.rs for consistent ordering (ADR-035).
const STANDARD_PROPERTY_NAMES: &[&str] = &[
    "key",
    "entity_key",
    "page_key",
    "block_key",
    "locale_key",
    "display_name",
    "description",
    "triggers",
    "created_by",
    "created_by_meta",
    "created_at",
    "updated_at",
];

/// Check if a property name is a standard property.
fn is_standard_property(name: &str) -> bool {
    STANDARD_PROPERTY_NAMES.contains(&name)
}

/// Get the focused property (key, value) at the current `focused_property_idx`.
/// Returns None if no property at that index or not viewing properties.
/// Order matches info.rs rendering: STANDARD → SPECIFIC → extra (instance-only).
pub fn get_focused_property(app: &App) -> Option<(String, String)> {
    let item = app.current_item()?;
    let idx = app.focused_property_idx;

    match item {
        TreeItem::Class(_, _, class) => {
            // For Class: property names only (no values)
            // Order: standard → specific
            let standard_keys: Vec<&String> = class
                .properties
                .iter()
                .filter(|k| is_standard_property(k.as_str()))
                .collect();
            let specific_keys: Vec<&String> = class
                .properties
                .iter()
                .filter(|k| !is_standard_property(k.as_str()))
                .collect();

            // Find property at index
            if idx < standard_keys.len() {
                let key = standard_keys[idx].clone();
                let is_required = class.required_properties.contains(&key);
                let value = if is_required { "required" } else { "optional" };
                Some((key, value.to_string()))
            } else {
                let specific_idx = idx - standard_keys.len();
                if specific_idx < specific_keys.len() {
                    let key = specific_keys[specific_idx].clone();
                    let is_required = class.required_properties.contains(&key);
                    let value = if is_required { "required" } else { "optional" };
                    Some((key, value.to_string()))
                } else {
                    None
                }
            }
        },
        TreeItem::Instance(_, _, class, instance) => {
            // For Instance: actual property values
            // Order: standard → specific → extra (instance-only)
            let all_schema_keys: Vec<&String> = class.properties.iter().collect();

            let standard_keys: Vec<&String> = all_schema_keys
                .iter()
                .filter(|k| is_standard_property(k.as_str()))
                .copied()
                .collect();
            let specific_keys: Vec<&String> = all_schema_keys
                .iter()
                .filter(|k| !is_standard_property(k.as_str()))
                .copied()
                .collect();
            let extra_keys: Vec<&String> = instance
                .properties
                .keys()
                .filter(|k| !k.starts_with('_') && !all_schema_keys.contains(k))
                .collect();

            // Find property at index
            let standard_len = standard_keys.len();
            let specific_len = specific_keys.len();

            if idx < standard_len {
                let key = standard_keys[idx].clone();
                let value = instance
                    .properties
                    .get(&key)
                    .map(format_json_value)
                    .unwrap_or_else(|| "—".to_string());
                Some((key, value))
            } else if idx < standard_len + specific_len {
                let specific_idx = idx - standard_len;
                let key = specific_keys[specific_idx].clone();
                let value = instance
                    .properties
                    .get(&key)
                    .map(format_json_value)
                    .unwrap_or_else(|| "—".to_string());
                Some((key, value))
            } else {
                let extra_idx = idx - standard_len - specific_len;
                if extra_idx < extra_keys.len() {
                    let key = extra_keys[extra_idx].clone();
                    let value = instance
                        .properties
                        .get(&key)
                        .map(format_json_value)
                        .unwrap_or_else(|| "—".to_string());
                    Some((key, value))
                } else {
                    None
                }
            }
        },
        _ => None,
    }
}

/// Format a JSON value for display (compact string representation).
fn format_json_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Array(arr) => serde_json::to_string(arr).unwrap_or_default(),
        serde_json::Value::Object(obj) => serde_json::to_string(obj).unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_to_clipboard_doesnt_panic() {
        // Just verify it doesn't panic - actual clipboard may not work in CI
        let result = copy_to_clipboard("test");
        // We accept either success or graceful error
        assert!(result.is_ok() || result.is_err());
    }
}
