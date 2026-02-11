//! Generate TypeScript view definitions from YAML sources.
//!
//! Eliminates runtime YAML parsing in TypeScript by generating
//! typed view definitions at build time.
//!
//! Output target: `packages/core/src/filters/views.generated.ts`

#![allow(clippy::needless_raw_string_hashes)]

use crate::parsers::views::{
    self, Direction, IncludeRule, ViewDef, ViewIcon, ViewRegistry, ViewRegistryEntry,
};
use minijinja::{Environment, Value, context};
use serde::Serialize;
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Template data structures
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct TemplateView {
    id: String,
    name: String,
    description: String,
    version: Option<String>,
    root_type: String,
    root_key: Option<String>,
    include: Vec<TemplateInclude>,
    filters: Option<TemplateFilters>,
    category: Option<String>,
}

#[derive(Serialize)]
struct TemplateInclude {
    relation: String,
    direction: String,
    depth: Option<u32>,
    target_types: Option<Vec<String>>,
    include: Option<Vec<TemplateInclude>>,
}

#[derive(Serialize)]
struct TemplateFilters {
    locale: Option<String>,
    max_depth: Option<u32>,
}

/// Icon template data with web (Lucide) and terminal (Unicode) variants.
#[derive(Serialize)]
struct TemplateIcon {
    web: String,
    terminal: String,
}

impl From<&ViewIcon> for TemplateIcon {
    fn from(icon: &ViewIcon) -> Self {
        Self {
            web: icon.web.clone(),
            terminal: icon.terminal.clone(),
        }
    }
}

#[derive(Serialize)]
struct TemplateRegistryEntry {
    id: String,
    description: String,
    category: String,
    modes: Vec<String>,
    /// Icon with web (Lucide) and terminal (Unicode) variants.
    icon: Option<TemplateIcon>,
    /// Color for the view (hex string).
    color: Option<String>,
    /// Cypher query template.
    cypher: Option<String>,
    /// Whether this view is contextual (requires nodeKey).
    contextual: Option<bool>,
    /// Node types this view applies to.
    applicable_types: Option<Vec<String>>,
    /// Parameters for the Cypher template.
    params: Option<Vec<String>>,
}

#[derive(Serialize)]
struct TemplateData {
    version: String,
    views: Vec<TemplateView>,
    registry: Vec<TemplateRegistryEntry>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Conversion helpers
// ─────────────────────────────────────────────────────────────────────────────

fn direction_to_string(dir: Direction) -> String {
    match dir {
        Direction::Outgoing => "outgoing".to_string(),
        Direction::Incoming => "incoming".to_string(),
        Direction::Both => "both".to_string(),
    }
}

fn convert_include(rule: &IncludeRule) -> TemplateInclude {
    TemplateInclude {
        relation: rule.relation.clone(),
        direction: direction_to_string(rule.direction),
        depth: rule.depth,
        target_types: rule.target_types.clone(),
        include: rule
            .include
            .as_ref()
            .map(|nested| nested.iter().map(convert_include).collect()),
    }
}

fn convert_view(view: &ViewDef, registry: &ViewRegistry) -> TemplateView {
    // Find category from registry
    let category = registry
        .views
        .iter()
        .find(|e| e.id == view.id)
        .map(|e| e.category.clone());

    // Extract filters if present
    let filters = view.filters.as_ref().and_then(|f| {
        let mapping = f.as_mapping()?;
        Some(TemplateFilters {
            locale: mapping
                .get("locale")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            max_depth: mapping
                .get("maxDepth")
                .and_then(|v| v.as_u64())
                .map(|n| n as u32),
        })
    });

    TemplateView {
        id: view.id.clone(),
        name: view.name.clone(),
        description: view.description.clone(),
        version: view.version.clone(),
        root_type: view.root.node_type.clone(),
        root_key: None, // Root key is always provided at runtime
        include: view.include.iter().map(convert_include).collect(),
        filters,
        category,
    }
}

fn convert_registry_entry(entry: &ViewRegistryEntry) -> TemplateRegistryEntry {
    TemplateRegistryEntry {
        id: entry.id.clone(),
        description: entry.description.clone(),
        category: entry.category.clone(),
        modes: entry
            .modes
            .clone()
            .unwrap_or_else(|| vec!["data".to_string()]),
        icon: entry.icon.as_ref().map(TemplateIcon::from),
        color: entry.color.clone(),
        cypher: entry.cypher.clone(),
        contextual: entry.contextual,
        applicable_types: entry.applicable_types.clone(),
        params: entry.params.clone(),
    }
}

/// Recursively remove null values from JSON (TypeScript uses undefined, not null)
fn remove_nulls(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let cleaned: serde_json::Map<String, serde_json::Value> = map
                .into_iter()
                .filter_map(|(k, v)| {
                    if v.is_null() {
                        None
                    } else {
                        Some((k, remove_nulls(v)))
                    }
                })
                .collect();
            serde_json::Value::Object(cleaned)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(remove_nulls).collect())
        }
        other => other,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TypeScript template
// ─────────────────────────────────────────────────────────────────────────────

const VIEWS_TEMPLATE: &str = r#"// Auto-generated by novanet v11.7.0 — DO NOT EDIT
// Source: packages/core/models/views/*.yaml
// Generated: {{ now }}

// ─────────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────────

export type Direction = 'outgoing' | 'incoming' | 'both';

/** Icon with web (Lucide) and terminal (Unicode) variants. */
export interface ViewIcon {
  /** Lucide icon name for web/Studio. */
  web: string;
  /** Unicode symbol for terminal/TUI. */
  terminal: string;
}

export interface IncludeRule {
  relation: string;
  direction: Direction;
  depth?: number;
  targetTypes?: string[];
  include?: IncludeRule[];
}

export interface ViewFilters {
  locale?: string;
  maxDepth?: number;
}

export interface ViewDefinition {
  id: string;
  name: string;
  description: string;
  version?: string;
  root: {
    type: string;
    key?: string;
  };
  include: IncludeRule[];
  filters?: ViewFilters;
  category?: string;
}

export interface ViewRegistryEntry {
  id: string;
  description: string;
  category: string;
  modes: string[];
  /** Icon with web (Lucide) and terminal (Unicode) variants. */
  icon?: ViewIcon;
  /** Color for the view (hex string). */
  color?: string;
  /** Cypher query template (parameterized). */
  cypher?: string;
  /** Whether this view is contextual (appears in node sidebar). */
  contextual?: boolean;
  /** Node types this view applies to (for contextual views). */
  applicableTypes?: string[];
  /** Parameters for the Cypher template. */
  params?: string[];
}

// ─────────────────────────────────────────────────────────────────────────────
// Generated Views ({{ views | length }} views)
// ─────────────────────────────────────────────────────────────────────────────

export const VIEWS: Record<string, ViewDefinition> = {
{%- for view in views %}
  '{{ view.id }}': {
    id: '{{ view.id }}',
    name: '{{ view.name }}',
    description: '{{ view.description | replace("'", "\\'") }}',
{%- if view.version %}
    version: '{{ view.version }}',
{%- endif %}
    root: {
      type: '{{ view.root_type }}',
    },
    include: {{ view.include | tojson }},
{%- if view.filters %}
    filters: {
{%- if view.filters.locale %}
      locale: '{{ view.filters.locale }}',
{%- endif %}
{%- if view.filters.max_depth %}
      maxDepth: {{ view.filters.max_depth }},
{%- endif %}
    },
{%- endif %}
{%- if view.category %}
    category: '{{ view.category }}',
{%- endif %}
  },
{%- endfor %}
};

// ─────────────────────────────────────────────────────────────────────────────
// Generated Registry ({{ registry | length }} entries)
// ─────────────────────────────────────────────────────────────────────────────

export const VIEW_REGISTRY: ViewRegistryEntry[] = [
{%- for entry in registry %}
  {
    id: '{{ entry.id }}',
    description: '{{ entry.description | replace("'", "\\'") }}',
    category: '{{ entry.category }}',
    modes: {{ entry.modes | tojson }},
{%- if entry.icon %}
    icon: { web: '{{ entry.icon.web }}', terminal: '{{ entry.icon.terminal }}' },
{%- endif %}
{%- if entry.color %}
    color: '{{ entry.color }}',
{%- endif %}
{%- if entry.cypher %}
    cypher: `{{ entry.cypher | replace("`", "\\`") }}`,
{%- endif %}
{%- if entry.contextual %}
    contextual: {{ entry.contextual }},
{%- endif %}
{%- if entry.applicable_types %}
    applicableTypes: {{ entry.applicable_types | tojson }},
{%- endif %}
{%- if entry.params %}
    params: {{ entry.params | tojson }},
{%- endif %}
  },
{%- endfor %}
];

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/** Get a view by ID (throws if not found) */
export function getView(id: string): ViewDefinition {
  const view = VIEWS[id];
  if (!view) {
    throw new Error(`View '${id}' not found. Available: ${Object.keys(VIEWS).join(', ')}`);
  }
  return view;
}

/** Get all views in a category */
export function getViewsByCategory(category: string): ViewDefinition[] {
  return Object.values(VIEWS).filter(v => v.category === category);
}

/** Get registry entries for a navigation mode */
export function getViewsForMode(mode: string): ViewRegistryEntry[] {
  return VIEW_REGISTRY.filter(e => e.modes.includes(mode));
}

/** All view IDs */
export const VIEW_IDS = Object.keys(VIEWS) as readonly string[];

/** All category names */
export const VIEW_CATEGORIES = [...new Set(VIEW_REGISTRY.map(e => e.category))] as const;
"#;

// ─────────────────────────────────────────────────────────────────────────────
// Generator
// ─────────────────────────────────────────────────────────────────────────────

pub struct ViewsGenerator;

impl super::Generator for ViewsGenerator {
    fn name(&self) -> &'static str {
        "views"
    }

    fn generate(&self, root: &Path) -> crate::Result<String> {
        let views = views::load_all_views(root)?;
        let registry = views::load_registry(root)?;

        generate_views(&views, &registry)
    }
}

/// Generate TypeScript view definitions from parsed YAML data.
pub fn generate_views(views: &[ViewDef], registry: &ViewRegistry) -> crate::Result<String> {
    let template_views: Vec<TemplateView> =
        views.iter().map(|v| convert_view(v, registry)).collect();

    let template_registry: Vec<TemplateRegistryEntry> =
        registry.views.iter().map(convert_registry_entry).collect();

    let data = TemplateData {
        version: registry.version.clone(),
        views: template_views,
        registry: template_registry,
    };

    let mut env = Environment::new();

    // Add tojson filter (removes null values for TypeScript compatibility)
    env.add_filter("tojson", |value: Value| -> String {
        // Convert to serde_json::Value, remove nulls, then serialize
        let json_value: serde_json::Value = serde_json::from_str(
            &serde_json::to_string(&value).unwrap_or_else(|_| "null".to_string()),
        )
        .unwrap_or(serde_json::Value::Null);
        let cleaned = remove_nulls(json_value);
        serde_json::to_string(&cleaned).unwrap_or_else(|_| "null".to_string())
    });

    env.add_template("views.ts", VIEWS_TEMPLATE)
        .expect("template should compile");

    let now = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();

    let output = env
        .get_template("views.ts")
        .expect("template exists")
        .render(context! {
            now => now,
            views => data.views,
            registry => data.registry,
        })
        .map_err(|e| crate::NovaNetError::Generator {
            generator: "views".to_string(),
            detail: e.to_string(),
        })?;

    Ok(output)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::Generator;
    use crate::parsers::views::{RootDef, ViewRegistryEntry};

    fn make_view(id: &str, root_type: &str) -> ViewDef {
        ViewDef {
            id: id.to_string(),
            name: format!("{} View", id),
            description: format!("Description for {}", id),
            version: Some("1.0.0".to_string()),
            root: RootDef {
                node_type: root_type.to_string(),
            },
            include: vec![IncludeRule {
                relation: "HAS_PAGE".to_string(),
                direction: Direction::Outgoing,
                depth: None,
                target_types: None,
                include: None,
            }],
            filters: None,
            docs: None,
        }
    }

    fn make_registry(views: &[ViewDef]) -> ViewRegistry {
        ViewRegistry {
            version: "11.7.0".to_string(),
            description: Some("Test registry".to_string()),
            views: views
                .iter()
                .map(|v| ViewRegistryEntry {
                    id: v.id.clone(),
                    file: Some(format!("{}.yaml", v.id)),
                    description: v.description.clone(),
                    category: "overview".to_string(),
                    modes: Some(vec!["data".to_string(), "meta".to_string()]),
                    icon: None,
                    color: None,
                    cypher: None,
                    contextual: None,
                    applicable_types: None,
                    params: None,
                })
                .collect(),
        }
    }

    #[test]
    fn generate_views_basic() {
        let views = vec![make_view("test-view", "Project")];
        let registry = make_registry(&views);

        let output = generate_views(&views, &registry).expect("should generate");

        assert!(output.contains("export const VIEWS"));
        assert!(output.contains("'test-view'"));
        assert!(output.contains("type: 'Project'"));
        assert!(output.contains("HAS_PAGE"));
    }

    #[test]
    fn generate_views_multiple() {
        let views = vec![make_view("view-a", "Block"), make_view("view-b", "Locale")];
        let registry = make_registry(&views);

        let output = generate_views(&views, &registry).expect("should generate");

        assert!(output.contains("'view-a'"));
        assert!(output.contains("'view-b'"));
        assert!(output.contains("type: 'Block'"));
        assert!(output.contains("type: 'Locale'"));
    }

    #[test]
    fn generate_registry() {
        let views = vec![make_view("my-view", "Page")];
        let registry = make_registry(&views);

        let output = generate_views(&views, &registry).expect("should generate");

        assert!(output.contains("export const VIEW_REGISTRY"));
        assert!(output.contains("category: 'overview'"));
        assert!(output.contains("modes: [\"data\",\"meta\"]"));
    }

    #[test]
    fn generate_helpers() {
        let views = vec![make_view("helper-test", "Concept")];
        let registry = make_registry(&views);

        let output = generate_views(&views, &registry).expect("should generate");

        assert!(output.contains("export function getView"));
        assert!(output.contains("export function getViewsByCategory"));
        assert!(output.contains("export function getViewsForMode"));
        assert!(output.contains("export const VIEW_IDS"));
        assert!(output.contains("export const VIEW_CATEGORIES"));
    }

    #[test]
    fn nested_includes() {
        let mut view = make_view("nested", "Project");
        view.include = vec![IncludeRule {
            relation: "HAS_PAGE".to_string(),
            direction: Direction::Outgoing,
            depth: Some(2),
            target_types: None,
            include: Some(vec![IncludeRule {
                relation: "HAS_BLOCK".to_string(),
                direction: Direction::Outgoing,
                depth: None,
                target_types: None,
                include: None,
            }]),
        }];
        let registry = make_registry(&[view.clone()]);

        let output = generate_views(&[view], &registry).expect("should generate");

        assert!(output.contains("HAS_PAGE"));
        assert!(output.contains("HAS_BLOCK"));
        assert!(output.contains("\"depth\":2"));
    }

    fn test_root() -> Option<std::path::PathBuf> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());
        let root = root?;
        if !root.join("pnpm-workspace.yaml").exists() {
            return None;
        }
        Some(root.to_path_buf())
    }

    #[test]
    fn generate_registry_with_icon() {
        let views = vec![make_view("icon-test", "Block")];
        let mut registry = make_registry(&views);

        // Add icon to the registry entry
        registry.views[0].icon = Some(ViewIcon {
            web: "diamond".to_string(),
            terminal: "◆".to_string(),
        });
        registry.views[0].color = Some("#8b5cf6".to_string());
        registry.views[0].cypher = Some("MATCH (n:Meta) RETURN n".to_string());
        registry.views[0].contextual = Some(true);
        registry.views[0].applicable_types = Some(vec!["Block".to_string(), "Page".to_string()]);

        let output = generate_views(&views, &registry).expect("should generate");

        // Check icon output
        assert!(output.contains("icon: { web: 'diamond', terminal: '◆' }"));
        assert!(output.contains("color: '#8b5cf6'"));
        assert!(output.contains("cypher: `MATCH (n:Meta) RETURN n`"));
        assert!(output.contains("contextual: true"));
        assert!(output.contains("applicableTypes: [\"Block\",\"Page\"]"));
    }

    #[test]
    fn generate_registry_without_icon() {
        let views = vec![make_view("no-icon-test", "Page")];
        let registry = make_registry(&views);

        let output = generate_views(&views, &registry).expect("should generate");

        // Icon should not appear when not set
        assert!(!output.contains("icon: {"));
        assert!(!output.contains("color:"));
        assert!(!output.contains("cypher:"));
    }

    #[test]
    fn generate_view_icon_interface() {
        let views = vec![make_view("interface-test", "Project")];
        let registry = make_registry(&views);

        let output = generate_views(&views, &registry).expect("should generate");

        // Check ViewIcon interface is generated
        assert!(output.contains("export interface ViewIcon {"));
        assert!(output.contains("web: string;"));
        assert!(output.contains("terminal: string;"));

        // Check ViewRegistryEntry includes icon field
        assert!(output.contains("icon?: ViewIcon;"));
    }

    #[test]
    fn generate_views_integration() {
        let Some(root) = test_root() else { return };

        let generator = ViewsGenerator;
        let output = generator.generate(&root).expect("should generate views.ts");

        // Should have views from the new unified format
        assert!(output.contains("meta-complete") || output.contains("block-generation"));

        // Should have proper TypeScript structure
        assert!(output.contains("export type Direction"));
        assert!(output.contains("export interface ViewDefinition"));
        assert!(output.contains("export interface ViewIcon"));
        assert!(output.contains("export const VIEWS"));
        assert!(output.contains("export const VIEW_REGISTRY"));

        // Should have helpers
        assert!(output.contains("export function getView"));

        // Should have icon fields in interface
        assert!(output.contains("icon?: ViewIcon;"));
    }

    #[test]
    fn generate_views_with_icons_integration() {
        let Some(root) = test_root() else { return };

        let generator = ViewsGenerator;
        let output = generator.generate(&root).expect("should generate views.ts");

        // New format should have icons in registry entries
        // Check for dual icon format
        if output.contains("icon: { web:") {
            assert!(output.contains("icon: { web: '"));
            assert!(output.contains("', terminal: '"));
        }
    }
}
