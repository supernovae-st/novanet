//! Tree view — hierarchical Realm > Layer > Kind.
//!

use crate::blueprint::ascii::{realm_icon, truncate};
use crate::blueprint::sources::BlueprintData;
use std::collections::HashMap;

/// Render tree view.
pub fn render(data: &BlueprintData) -> String {
    let mut out = String::new();
    out.push_str(
        "╭──────────────────────────────────────────────────────────────────────────────╮\n",
    );
    out.push_str(
        "│  ◉ NOVANET TREE                                                             │\n",
    );
    out.push_str(
        "│                                                                              │\n",
    );
    out.push_str(
        "│  Hierarchy: Realm > Layer > Kind                                             │\n",
    );
    out.push_str(
        "╰──────────────────────────────────────────────────────────────────────────────╯\n\n",
    );

    // Group nodes by realm and layer
    let mut by_realm_layer: HashMap<(&str, &str), Vec<&crate::parsers::yaml_node::ParsedNode>> =
        HashMap::new();
    for node in &data.node_classes {
        by_realm_layer
            .entry((node.realm.as_str(), node.layer.as_str()))
            .or_default()
            .push(node);
    }

    // Build tree for each realm
    for realm_def in &data.taxonomy.node_realms {
        let realm_icon = realm_icon(&realm_def.key);
        let realm_desc = match realm_def.key.as_str() {
            "shared" => "(read-only, universal)",
            "org" => "(business-specific)",
            _ => "",
        };

        out.push_str(&format!(
            "{} {} {}\n",
            realm_icon,
            realm_def.key.to_uppercase(),
            realm_desc
        ));

        let layer_count = realm_def.layers.len();
        for (layer_idx, layer_def) in realm_def.layers.iter().enumerate() {
            let is_last_layer = layer_idx == layer_count - 1;
            let layer_prefix = if is_last_layer {
                "└── "
            } else {
                "├── "
            };
            let child_prefix = if is_last_layer { "    " } else { "│   " };

            let nodes = by_realm_layer
                .get(&(realm_def.key.as_str(), layer_def.key.as_str()))
                .map(|v| v.as_slice())
                .unwrap_or(&[]);

            out.push_str(&format!(
                "{}{} {} ({} kinds)\n",
                layer_prefix,
                layer_def.emoji(),
                layer_def.key,
                nodes.len()
            ));

            // Sort nodes by name for consistent output
            let mut sorted_nodes: Vec<_> = nodes.iter().collect();
            sorted_nodes.sort_by_key(|n| &n.def.name);

            let node_count = sorted_nodes.len();
            for (node_idx, node) in sorted_nodes.iter().enumerate() {
                let is_last_node = node_idx == node_count - 1;
                let node_prefix = if is_last_node {
                    "└── "
                } else {
                    "├── "
                };
                let description = truncate(&node.def.description, 40);

                out.push_str(&format!(
                    "{}{}{} — {}\n",
                    child_prefix, node_prefix, node.def.name, description
                ));
            }
        }
        out.push('\n');
    }


    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_view() {
        let root = crate::config::resolve_root(None).expect("Failed to resolve root");
        let data = BlueprintData::from_yaml(&root).expect("Failed to load blueprint data");

        let output = render(&data);

        assert!(output.contains("NOVANET TREE"), "Should have header");
        assert!(output.contains("SHARED"), "Should have shared realm");
        assert!(output.contains("ORG"), "Should have org realm");
    }
}
