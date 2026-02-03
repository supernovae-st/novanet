//! Parse `relations.yaml` (v9 list format + family + multi-source/target).
//!
//! Every relation in v9 has a `family` (ownership/localization/semantic/generation/mining),
//! and `source`/`target` can be a single label or a list of labels.
//!
//! v9.5 Note: EdgeFamily is now ArcFamily. EdgeFamily is kept as a type alias for backwards compatibility.

use serde::Deserialize;
use smallvec::{SmallVec, smallvec};
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Enums
// ─────────────────────────────────────────────────────────────────────────────

/// The 5 arc families in v9.5 (formerly EdgeFamily in v9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArcFamily {
    Ownership,
    Localization,
    Semantic,
    Generation,
    Mining,
}

/// Type alias for backwards compatibility with v9 code.
pub type EdgeFamily = ArcFamily;

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

/// Top-level relations.yaml document.
#[derive(Debug, Deserialize)]
pub struct RelationsDocument {
    /// The 50 relation definitions.
    pub relations: Vec<RelationDef>,

    /// SEMANTIC_LINK subtypes (10 entries: is_action_on, includes, etc.).
    #[serde(default)]
    pub semantic_link_types: Option<Vec<String>>,

    /// Example Cypher queries (opaque — for documentation only).
    #[serde(default)]
    pub examples: Option<serde_yaml::Value>,
}

/// A single relation/arc definition.
/// v9.5 Note: RelationDef is now ArcDef. RelationDef is kept for backwards compatibility.
#[derive(Debug, Clone, Deserialize)]
pub struct RelationDef {
    /// Relationship type (SCREAMING_SNAKE_CASE), e.g. "HAS_PAGE".
    #[serde(rename = "type")]
    pub rel_type: String,

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

/// Type alias for v9.5 Arc terminology.
pub type ArcDef = RelationDef;

impl RelationDef {
    /// Returns true if this relation has an `inverse_of` field.
    pub fn is_inverse(&self) -> bool {
        self.inverse_of.is_some()
    }

    /// Returns true if this relation carries properties on the edge.
    pub fn has_properties(&self) -> bool {
        self.properties.as_ref().is_some_and(|p| !p.is_empty())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

/// Load and parse `relations.yaml` from `<root>/packages/core/models/relations.yaml`.
///
/// # Errors
///
/// - `NovaNetError::Validation` if the file doesn't exist
/// - `NovaNetError::Schema` if YAML parsing fails (including missing required fields)
/// - `NovaNetError::Io` on filesystem errors
pub fn load_relations(root: &Path) -> crate::Result<RelationsDocument> {
    let path = crate::config::relations_path(root);

    if !path.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "relations.yaml not found: {}",
            path.display()
        )));
    }

    let content = std::fs::read_to_string(&path)?;

    let doc: RelationsDocument =
        serde_yaml::from_str(&content).map_err(|e| crate::NovaNetError::Schema {
            path: path.display().to_string(),
            source: e,
        })?;

    if doc.relations.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "relations.yaml contains no relation definitions".to_string(),
        ));
    }

    Ok(doc)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_family_deserialize() {
        for (yaml, expected) in [
            ("ownership", EdgeFamily::Ownership),
            ("localization", EdgeFamily::Localization),
            ("semantic", EdgeFamily::Semantic),
            ("generation", EdgeFamily::Generation),
            ("mining", EdgeFamily::Mining),
        ] {
            let result: EdgeFamily = serde_yaml::from_str(yaml).unwrap();
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
    fn parse_single_relation() {
        let yaml = r#"
relations:
  - type: HAS_PAGE
    family: ownership
    source: Project
    target: Page
    cardinality: one_to_many
    llm_context: "Project owns pages."
"#;
        let doc: RelationsDocument = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(doc.relations.len(), 1);

        let rel = &doc.relations[0];
        assert_eq!(rel.rel_type, "HAS_PAGE");
        assert_eq!(rel.family, EdgeFamily::Ownership);
        assert_eq!(rel.source.labels().as_slice(), ["Project"]);
        assert_eq!(rel.target.labels().as_slice(), ["Page"]);
        assert_eq!(rel.cardinality, Cardinality::OneToMany);
        assert!(!rel.is_inverse());
        assert!(!rel.has_properties());
    }

    #[test]
    fn parse_relation_with_properties() {
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
        let doc: RelationsDocument = serde_yaml::from_str(yaml).unwrap();
        let rel = &doc.relations[0];
        assert!(rel.has_properties());
        assert_eq!(rel.properties.as_ref().unwrap(), &["position"]);
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
        let doc: RelationsDocument = serde_yaml::from_str(yaml).unwrap();
        let rel = &doc.relations[0];
        assert_eq!(rel.source.labels().as_slice(), ["Page", "Block"]);
        assert_eq!(rel.target.labels().as_slice(), ["PageType", "BlockType"]);
    }

    #[test]
    fn parse_inverse_relation() {
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
        let doc: RelationsDocument = serde_yaml::from_str(yaml).unwrap();
        let rel = &doc.relations[0];
        assert!(rel.is_inverse());
        assert_eq!(rel.inverse_of.as_deref(), Some("HAS_BLOCK"));
    }

    #[test]
    fn parse_self_referential() {
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
        let doc: RelationsDocument = serde_yaml::from_str(yaml).unwrap();
        let rel = &doc.relations[0];
        assert_eq!(rel.is_self_referential, Some(true));
    }

    #[test]
    fn load_relations_integration() {
        // Requires actual monorepo
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let doc = load_relations(root).expect("should parse relations.yaml");

        // Total relation count
        assert_eq!(doc.relations.len(), 75, "expected 75 relations");

        // Family distribution
        let family_count = |f: EdgeFamily| doc.relations.iter().filter(|r| r.family == f).count();
        assert_eq!(family_count(EdgeFamily::Ownership), 15, "ownership count");
        assert_eq!(
            family_count(EdgeFamily::Localization),
            22,
            "localization count"
        );
        assert_eq!(family_count(EdgeFamily::Semantic), 14, "semantic count");
        assert_eq!(family_count(EdgeFamily::Generation), 15, "generation count");
        assert_eq!(family_count(EdgeFamily::Mining), 9, "mining count");

        // All relations have non-empty type and llm_context
        for rel in &doc.relations {
            assert!(!rel.rel_type.is_empty(), "empty rel_type");
            assert!(
                !rel.llm_context.is_empty(),
                "empty llm_context for {}",
                rel.rel_type
            );
            assert!(!rel.source.is_empty(), "empty source for {}", rel.rel_type);
            assert!(!rel.target.is_empty(), "empty target for {}", rel.rel_type);
        }

        // Unique relation types
        let mut types: Vec<&str> = doc.relations.iter().map(|r| r.rel_type.as_str()).collect();
        types.sort();
        types.dedup();
        assert_eq!(types.len(), 75, "all relation types should be unique");

        // Semantic link types
        let slt = doc
            .semantic_link_types
            .as_ref()
            .expect("should have semantic_link_types");
        assert_eq!(slt.len(), 10, "expected 10 semantic link types");

        // Spot-checks
        let has_page = doc
            .relations
            .iter()
            .find(|r| r.rel_type == "HAS_PAGE")
            .unwrap();
        assert_eq!(has_page.family, EdgeFamily::Ownership);
        assert_eq!(has_page.source.labels().as_slice(), ["Project"]);
        assert_eq!(has_page.target.labels().as_slice(), ["Page"]);
        assert_eq!(has_page.cardinality, Cardinality::OneToMany);

        let for_locale = doc
            .relations
            .iter()
            .find(|r| r.rel_type == "FOR_LOCALE")
            .unwrap();
        assert_eq!(for_locale.family, EdgeFamily::Localization);
        assert_eq!(for_locale.source.len(), 9, "FOR_LOCALE has 9 sources");
    }
}
