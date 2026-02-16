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
pub fn get_box_content(app: &App) -> Option<(String, &'static str)> {
    match app.selected_box {
        InfoBox::Tree => get_tree_content(app),
        InfoBox::Header => get_header_content(app),
        InfoBox::Properties => get_properties_content(app),
        InfoBox::Arcs => get_arcs_content(app),
        InfoBox::Source => get_source_content(app),
        InfoBox::Diagram => get_diagram_content(app),
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
        }
        TreeItem::Instance(realm, layer, class, instance) => {
            format!("{}/{}/{}/{}", realm.key, layer.key, class.key, instance.key)
        }
        TreeItem::ArcFamily(family) => format!("arcs/{}", family.key),
        TreeItem::ArcClass(family, arc) => format!("arcs/{}/{}", family.key, arc.key),
        TreeItem::ClassesSection => "classes".to_string(),
        TreeItem::ArcsSection => "arcs".to_string(),
        TreeItem::EntityCategory(realm, layer, class, cat) => {
            format!(
                "{}/{}/{}/{}",
                realm.key, layer.key, class.key, cat.key
            )
        }
    };
    Some((path, "path"))
}

/// HEADER box: JSON metadata.
fn get_header_content(app: &App) -> Option<(String, &'static str)> {
    let item = app.current_item()?;
    let json = match item {
        TreeItem::Class(realm, layer, class) => {
            serde_json::json!({
                "type": "Class",
                "key": class.key,
                "realm": realm.key,
                "layer": layer.key,
                "trait": class.trait_name,
                "display_name": class.display_name
            })
        }
        TreeItem::ArcClass(family, arc) => {
            serde_json::json!({
                "type": "ArcClass",
                "key": arc.key,
                "family": family.key,
                "source": arc.from_class,
                "target": arc.to_class,
                "cardinality": arc.cardinality
            })
        }
        TreeItem::Realm(realm) => {
            serde_json::json!({
                "type": "Realm",
                "key": realm.key,
                "display_name": realm.display_name
            })
        }
        TreeItem::Layer(realm, layer) => {
            serde_json::json!({
                "type": "Layer",
                "key": layer.key,
                "realm": realm.key,
                "display_name": layer.display_name
            })
        }
        TreeItem::Instance(realm, layer, class, instance) => {
            serde_json::json!({
                "type": "Instance",
                "key": instance.key,
                "class": class.key,
                "realm": realm.key,
                "layer": layer.key
            })
        }
        _ => return None,
    };
    Some((serde_json::to_string_pretty(&json).unwrap_or_default(), "JSON"))
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
        }
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
        }
        _ => None,
    }
}

/// ARCS box: JSON array of incoming/outgoing relationships.
fn get_arcs_content(app: &App) -> Option<(String, &'static str)> {
    let arcs = app.class_arcs.as_ref()?;
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
    Some((serde_json::to_string_pretty(&json).unwrap_or_default(), "JSON arcs"))
}

/// SOURCE box: Raw YAML content.
fn get_source_content(app: &App) -> Option<(String, &'static str)> {
    if app.yaml_content.is_empty() {
        None
    } else {
        Some((app.yaml_content.clone(), "YAML"))
    }
}

/// DIAGRAM box: ASCII/Mermaid diagram content.
/// Currently returns a simple hierarchy representation.
fn get_diagram_content(app: &App) -> Option<(String, &'static str)> {
    let item = app.current_item()?;
    match item {
        TreeItem::Class(realm, layer, class) => {
            // Generate simple hierarchy diagram
            let diagram = format!(
                "REALM: {}\n  └── LAYER: {}\n        └── CLASS: {} ({})",
                realm.display_name, layer.display_name, class.display_name, class.trait_name
            );
            Some((diagram, "ASCII"))
        }
        TreeItem::Realm(realm) => {
            let layers: Vec<String> = app
                .tree
                .realms
                .iter()
                .find(|r| r.key == realm.key)
                .map(|r| r.layers.iter().map(|l| l.display_name.clone()).collect())
                .unwrap_or_default();
            let diagram = format!(
                "REALM: {}\n{}",
                realm.display_name,
                layers
                    .iter()
                    .map(|l| format!("  └── {}", l))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            Some((diagram, "ASCII"))
        }
        _ => None,
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
