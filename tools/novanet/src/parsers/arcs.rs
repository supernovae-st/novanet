//! Parse arc definitions from `relations.yaml`.
//!
//! Every arc in v9.5 has a `family` (ownership/localization/semantic/generation/mining),
//! and `source`/`target` can be a single label or a list of labels.
//!
//! Key types:
//! - `ArcsDocument` — top-level document with all arc definitions
//! - `ArcDef` — single arc definition (type, family, source, target, cardinality)
//! - `ArcFamily` — the 5 arc families

use serde::Deserialize;
use smallvec::{SmallVec, smallvec};
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Enums
// ─────────────────────────────────────────────────────────────────────────────

/// The 5 arc families in v9.5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArcFamily {
    Ownership,
    Localization,
    Semantic,
    Generation,
    Mining,
}

impl std::fmt::Display for ArcFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ownership => write!(f, "ownership"),
            Self::Localization => write!(f, "localization"),
            Self::Semantic => write!(f, "semantic"),
            Self::Generation => write!(f, "generation"),
            Self::Mining => write!(f, "mining"),
        }
    }
}

/// Relationship cardinality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cardinality {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

impl std::fmt::Display for Cardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneToOne => write!(f, "1:1"),
            Self::OneToMany => write!(f, "1:N"),
            Self::ManyToOne => write!(f, "N:1"),
            Self::ManyToMany => write!(f, "N:M"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// NodeRef — handles string or list of strings
// ─────────────────────────────────────────────────────────────────────────────

/// A source or target reference: either a single label or a list of labels.
///
/// Examples in YAML:
/// ```yaml
/// source: Project          # Single
/// source:                  # Multiple
///   - Page
///   - Block
/// ```
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum NodeRef {
    Single(String),
    Multiple(Vec<String>),
}

impl NodeRef {
    /// Returns labels as a SmallVec (stack-allocated for ≤4 labels).
    /// Most relations have 1-4 sources/targets, avoiding heap allocation.
    pub fn labels(&self) -> SmallVec<[&str; 4]> {
        match self {
            NodeRef::Single(s) => smallvec![s.as_str()],
            NodeRef::Multiple(v) => v.iter().map(|s| s.as_str()).collect(),
        }
    }

    /// Returns the number of labels.
    pub fn len(&self) -> usize {
        match self {
            NodeRef::Single(_) => 1,
            NodeRef::Multiple(v) => v.len(),
        }
    }

    /// Returns true if empty (should never happen in valid YAML).
    pub fn is_empty(&self) -> bool {
        match self {
            NodeRef::Single(s) => s.is_empty(),
            NodeRef::Multiple(v) => v.is_empty(),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level arcs document (from relations.yaml).
#[derive(Debug, Deserialize)]
pub struct ArcsDocument {
    /// The 77 arc definitions.
    #[serde(alias = "relations")]
    pub arcs: Vec<ArcDef>,

    /// SEMANTIC_LINK subtypes (10 entries: is_action_on, includes, etc.).
    #[serde(default)]
    pub semantic_link_types: Option<Vec<String>>,

    /// Example Cypher queries (opaque — for documentation only).
    #[serde(default)]
    pub examples: Option<serde_yaml::Value>,
}

/// A single arc definition.
#[derive(Debug, Clone, Deserialize)]
pub struct ArcDef {
    /// Arc type (SCREAMING_SNAKE_CASE), e.g. "HAS_PAGE".
    #[serde(rename = "type")]
    pub arc_type: String,

    /// Arc family classification.
    pub family: ArcFamily,

    /// Source node label(s).
    pub source: NodeRef,

    /// Target node label(s).
    pub target: NodeRef,

    /// Cardinality constraint.
    pub cardinality: Cardinality,

    /// LLM context string.
    pub llm_context: String,

    /// Arc property names (optional).
    #[serde(default)]
    pub properties: Option<Vec<String>>,

    /// True if source and target can be the same type (e.g. SEMANTIC_LINK).
    #[serde(default)]
    pub is_self_referential: Option<bool>,

    /// If this is an inverse arc, references the forward arc type.
    #[serde(default)]
    pub inverse_of: Option<String>,
}

impl ArcDef {
    /// Returns true if this arc has an `inverse_of` field.
    pub fn is_inverse(&self) -> bool {
        self.inverse_of.is_some()
    }

    /// Returns true if this arc carries properties.
    pub fn has_properties(&self) -> bool {
        self.properties.as_ref().is_some_and(|p| !p.is_empty())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

/// Load and parse arc definitions from `relations.yaml`.
///
/// # Errors
///
/// - `NovaNetError::Validation` if the file doesn't exist
/// - `NovaNetError::Schema` if YAML parsing fails (including missing required fields)
/// - `NovaNetError::Io` on filesystem errors
pub fn load_arcs(root: &Path) -> crate::Result<ArcsDocument> {
    let path = crate::config::relations_path(root);

    if !path.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "relations.yaml not found: {}",
            path.display()
        )));
    }

    let doc: ArcsDocument = super::utils::load_yaml(&path)?;

    if doc.arcs.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "relations.yaml contains no arc definitions".to_string(),
        ));
    }

    Ok(doc)
}

// ─────────────────────────────────────────────────────────────────────────────
// Individual Arc-Kind YAML Files (v9.9)
// ─────────────────────────────────────────────────────────────────────────────

/// Individual arc-kind YAML file structure (from arc-kinds/{family}/*.yaml).
#[derive(Debug, Clone, Deserialize)]
pub struct ArcKindYaml {
    pub arc: ArcKindDef,
}

/// Arc definition within individual arc-kind YAML file.
#[derive(Debug, Clone, Deserialize)]
pub struct ArcKindDef {
    pub name: String,
    pub family: ArcFamily,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub temperature_threshold: Option<f32>,
    // Other fields are optional for our purposes
    #[serde(default)]
    pub source: Option<serde_yaml::Value>,
    #[serde(default)]
    pub target: Option<serde_yaml::Value>,
    #[serde(default)]
    pub cardinality: Option<String>,
    #[serde(default)]
    pub llm_context: Option<String>,
}

use std::collections::HashMap;

/// Load temperature_threshold values from individual arc-kind YAML files.
///
/// Returns a map of arc_type -> temperature_threshold.
pub fn load_arc_temperatures(root: &Path) -> crate::Result<HashMap<String, f32>> {
    let arc_kinds_dir = crate::config::arc_kinds_dir(root);
    let mut temps = HashMap::new();

    if !arc_kinds_dir.exists() {
        return Ok(temps);
    }

    // Scan all family directories
    for family_dir in std::fs::read_dir(&arc_kinds_dir)? {
        let family_dir = family_dir?;
        if !family_dir.file_type()?.is_dir() {
            continue;
        }

        // Scan YAML files in each family directory
        for entry in std::fs::read_dir(family_dir.path())? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().is_none_or(|e| e != "yaml") {
                continue;
            }

            // Parse the arc-kind YAML
            if let Ok(yaml) = super::utils::load_yaml::<ArcKindYaml>(&path) {
                if let Some(threshold) = yaml.arc.temperature_threshold {
                    temps.insert(yaml.arc.name, threshold);
                }
            }
        }
    }

    Ok(temps)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_family_deserialize() {
        for (yaml, expected) in [
            ("ownership", ArcFamily::Ownership),
            ("localization", ArcFamily::Localization),
            ("semantic", ArcFamily::Semantic),
            ("generation", ArcFamily::Generation),
            ("mining", ArcFamily::Mining),
        ] {
            let result: ArcFamily = serde_yaml::from_str(yaml).unwrap();
            assert_eq!(result, expected);
            assert_eq!(result.to_string(), yaml);
        }
    }

    #[test]
    fn cardinality_deserialize() {
        for (yaml, expected, display) in [
            ("one_to_one", Cardinality::OneToOne, "1:1"),
            ("one_to_many", Cardinality::OneToMany, "1:N"),
            ("many_to_one", Cardinality::ManyToOne, "N:1"),
            ("many_to_many", Cardinality::ManyToMany, "N:M"),
        ] {
            let result: Cardinality = serde_yaml::from_str(yaml).unwrap();
            assert_eq!(result, expected);
            assert_eq!(result.to_string(), display);
        }
    }

    #[test]
    fn node_ref_single() {
        let yaml = "\"Project\"";
        let nr: NodeRef = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(nr.labels().as_slice(), ["Project"]);
        assert_eq!(nr.len(), 1);
    }

    #[test]
    fn node_ref_multiple() {
        let yaml = "- Page\n- Block";
        let nr: NodeRef = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(nr.labels().as_slice(), ["Page", "Block"]);
        assert_eq!(nr.len(), 2);
    }

    #[test]
    fn parse_single_arc() {
        let yaml = r#"
relations:
  - type: HAS_PAGE
    family: ownership
    source: Project
    target: Page
    cardinality: one_to_many
    llm_context: "Project owns pages."
"#;
        let doc: ArcsDocument = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.arcs.len(), 1);

        let arc = &doc.arcs[0];
        assert_eq!(arc.arc_type, "HAS_PAGE");
        assert_eq!(arc.family, ArcFamily::Ownership);
        assert_eq!(arc.source.labels().as_slice(), ["Project"]);
        assert_eq!(arc.target.labels().as_slice(), ["Page"]);
        assert_eq!(arc.cardinality, Cardinality::OneToMany);
        assert!(!arc.is_inverse());
        assert!(!arc.has_properties());
    }

    #[test]
    fn parse_arc_with_properties() {
        let yaml = r#"
relations:
  - type: HAS_BLOCK
    family: ownership
    source: Page
    target: Block
    cardinality: one_to_many
    properties:
      - position
    llm_context: "Page contains ordered blocks."
"#;
        let doc: ArcsDocument = serde_yaml::from_str(yaml).unwrap();
        let arc = &doc.arcs[0];
        assert!(arc.has_properties());
        assert_eq!(arc.properties.as_ref().unwrap(), &["position"]);
    }

    #[test]
    fn parse_multi_source_target() {
        let yaml = r#"
relations:
  - type: OF_TYPE
    family: ownership
    source:
      - Page
      - Block
    target:
      - PageType
      - BlockType
    cardinality: many_to_one
    llm_context: "Types."
"#;
        let doc: ArcsDocument = serde_yaml::from_str(yaml).unwrap();
        let arc = &doc.arcs[0];
        assert_eq!(arc.source.labels().as_slice(), ["Page", "Block"]);
        assert_eq!(arc.target.labels().as_slice(), ["PageType", "BlockType"]);
    }

    #[test]
    fn parse_inverse_arc() {
        let yaml = r#"
relations:
  - type: BLOCK_OF
    family: ownership
    source: Block
    target: Page
    cardinality: many_to_one
    inverse_of: HAS_BLOCK
    llm_context: "Inverse."
"#;
        let doc: ArcsDocument = serde_yaml::from_str(yaml).unwrap();
        let arc = &doc.arcs[0];
        assert!(arc.is_inverse());
        assert_eq!(arc.inverse_of.as_deref(), Some("HAS_BLOCK"));
    }

    #[test]
    fn parse_self_referential_arc() {
        let yaml = r#"
relations:
  - type: FALLBACK_TO
    family: localization
    source: Locale
    target: Locale
    cardinality: many_to_one
    is_self_referential: true
    llm_context: "Fallback chain."
"#;
        let doc: ArcsDocument = serde_yaml::from_str(yaml).unwrap();
        let arc = &doc.arcs[0];
        assert_eq!(arc.is_self_referential, Some(true));
    }

    #[test]
    fn load_arcs_integration() {
        // Requires actual monorepo
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let doc = load_arcs(root).expect("should parse relations.yaml");

        // v10: Total arc count (63 + 10 new knowledge arcs = 73)
        assert_eq!(doc.arcs.len(), 73, "expected 73 arcs");

        // v10: Family distribution
        // Ownership: 15 + 10 knowledge arcs = 25
        // Localization: 22 - 14 deleted = 8
        let family_count = |f: ArcFamily| doc.arcs.iter().filter(|a| a.family == f).count();
        assert_eq!(family_count(ArcFamily::Ownership), 25, "ownership count");
        assert_eq!(
            family_count(ArcFamily::Localization),
            8,
            "localization count"
        );
        assert_eq!(family_count(ArcFamily::Semantic), 16, "semantic count");
        assert_eq!(family_count(ArcFamily::Generation), 15, "generation count");
        assert_eq!(family_count(ArcFamily::Mining), 9, "mining count");

        // All arcs have non-empty type and llm_context
        for arc in &doc.arcs {
            assert!(!arc.arc_type.is_empty(), "empty arc_type");
            assert!(
                !arc.llm_context.is_empty(),
                "empty llm_context for {}",
                arc.arc_type
            );
            assert!(!arc.source.is_empty(), "empty source for {}", arc.arc_type);
            assert!(!arc.target.is_empty(), "empty target for {}", arc.arc_type);
        }

        // v10: Unique arc types (63 + 10 knowledge arcs = 73)
        let mut types: Vec<&str> = doc.arcs.iter().map(|a| a.arc_type.as_str()).collect();
        types.sort();
        types.dedup();
        assert_eq!(types.len(), 73, "all arc types should be unique");

        // Semantic link types
        let slt = doc
            .semantic_link_types
            .as_ref()
            .expect("should have semantic_link_types");
        assert_eq!(slt.len(), 10, "expected 10 semantic link types");

        // Spot-checks
        let has_page = doc.arcs.iter().find(|a| a.arc_type == "HAS_PAGE").unwrap();
        assert_eq!(has_page.family, ArcFamily::Ownership);
        assert_eq!(has_page.source.labels().as_slice(), ["Project"]);
        assert_eq!(has_page.target.labels().as_slice(), ["Page"]);
        assert_eq!(has_page.cardinality, Cardinality::OneToMany);

        let for_locale = doc
            .arcs
            .iter()
            .find(|a| a.arc_type == "FOR_LOCALE")
            .unwrap();
        assert_eq!(for_locale.family, ArcFamily::Localization);
        assert_eq!(for_locale.source.len(), 9, "FOR_LOCALE has 9 sources");
    }

    #[test]
    fn parse_arc_kind_yaml() {
        let yaml = r#"
arc:
  name: SEMANTIC_LINK
  family: semantic
  scope: intra_realm
  temperature_threshold: 0.3
"#;
        let parsed: ArcKindYaml = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(parsed.arc.name, "SEMANTIC_LINK");
        assert_eq!(parsed.arc.family, ArcFamily::Semantic);
        assert_eq!(parsed.arc.temperature_threshold, Some(0.3));
    }

    #[test]
    fn parse_arc_kind_yaml_no_threshold() {
        let yaml = r#"
arc:
  name: HAS_PAGE
  family: ownership
"#;
        let parsed: ArcKindYaml = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(parsed.arc.name, "HAS_PAGE");
        assert_eq!(parsed.arc.temperature_threshold, None);
    }

    #[test]
    fn load_arc_temperatures_integration() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let temps = load_arc_temperatures(root).expect("should load arc temperatures");

        // Check semantic arcs have temperature_threshold
        assert!(
            temps.contains_key("SEMANTIC_LINK"),
            "SEMANTIC_LINK should have threshold"
        );
        assert_eq!(temps.get("SEMANTIC_LINK"), Some(&0.3_f32));
        assert!(
            temps.contains_key("USES_CONCEPT"),
            "USES_CONCEPT should have threshold"
        );
        assert_eq!(temps.get("USES_CONCEPT"), Some(&0.0_f32));

        // Should have multiple semantic arcs
        let semantic_count = temps.len();
        assert!(
            semantic_count >= 14,
            "should have at least 14 semantic arcs with thresholds"
        );
    }
}
