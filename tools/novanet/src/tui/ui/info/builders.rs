//! Content builder dispatcher for the info panel.
//!
//! Routes `build_unified_content` to focused submodules:
//! - `build_schema`: ClassesSection, ArcsSection, Realm, Layer
//! - `build_class`: Class (NodeClass)
//! - `build_arcs`: ArcFamily, ArcClass
//! - `build_instance`: Instance
//! - `build_groups`: EntityCategory, EntityGroup, empty state

use crate::tui::app::App;
use crate::tui::data::{InstanceInfo, TreeItem};

use super::UnifiedContent;

use super::build_schema::{
    build_arcs_section_content, build_classes_section_content, build_layer_content,
    build_realm_content,
};
use super::build_arcs::{build_arc_class_content, build_arc_family_content};
use super::build_class::build_class_content;
use super::build_groups::{
    build_category_content, build_empty_content, build_entity_group_content,
};
use super::build_instance::build_instance_content;

/// Build unified content for the current tree item.
/// Returns all 6 sections populated with appropriate content.
pub fn build_unified_content(app: &App) -> UnifiedContent<'static> {
    match app.current_item() {
        Some(TreeItem::ClassesSection) => build_classes_section_content(app),
        Some(TreeItem::ArcsSection) => build_arcs_section_content(app),
        Some(TreeItem::Realm(realm)) => build_realm_content(app, realm),
        Some(TreeItem::Layer(realm, layer)) => build_layer_content(app, realm, layer),
        Some(TreeItem::Class(realm, layer, class)) => {
            build_class_content(app, realm, layer, class)
        },
        Some(TreeItem::ArcFamily(family)) => build_arc_family_content(family),
        Some(TreeItem::ArcClass(family, arc_class)) => {
            build_arc_class_content(family, arc_class)
        },
        Some(TreeItem::Instance(realm, layer, class, instance)) => {
            build_instance_content(app, realm, layer, class, instance)
        },
        Some(TreeItem::EntityCategory(_, _, _, cat)) => build_category_content(cat),
        // EntityGroup shows parent Entity as INSTANCE panel
        Some(TreeItem::EntityGroup(_, _, _, group)) => {
            if let Some((entity_realm, entity_layer, entity_class)) =
                app.tree.find_class("Entity")
            {
                if let Some(instances) = app.tree.instances.get("Entity") {
                    if let Some(entity_instance) =
                        instances.iter().find(|i| i.key == group.entity_key)
                    {
                        return build_instance_content(
                            app,
                            entity_realm,
                            entity_layer,
                            entity_class,
                            entity_instance,
                        );
                    }
                }
            }
            // Fallback to custom content if lookup fails
            build_entity_group_content(app, group)
        },
        // EntityNativeItem shows as INSTANCE panel with full layout
        Some(TreeItem::EntityNativeItem(realm, layer, class, native)) => {
            let filled = native.properties.len();
            let total = class.properties.len();
            let missing_required = class
                .required_properties
                .iter()
                .filter(|k| !native.properties.contains_key(*k))
                .count();

            let instance = InstanceInfo {
                key: native.key.clone(),
                display_name: native.display_name.clone(),
                class_key: class.key.clone(),
                properties: native.properties.clone(),
                outgoing_arcs: vec![],
                incoming_arcs: vec![],
                arcs_loading: false,
                missing_required_count: missing_required,
                filled_properties: filled,
                total_properties: total,
                entity_slug: None,
                relationship_power: 0,
            };
            build_instance_content(app, realm, layer, class, &instance)
        },
        None => build_empty_content(),
    }
}
