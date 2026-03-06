//! Parse arc definitions from `relations.yaml`.
//!
//! Every arc in v9.5 has a `family` (ownership/localization/semantic/generation/mining),
//! and `source`/`target` can be a single label or a list of labels.
//!
//! Key types:
//! - `ArcsDocument` — top-level document with all arc definitions
//! - `ArcDef` — single arc definition (type, family, source, target, cardinality)
//! - `ArcFamily` — the 6 arc families
//! - `ArcPropertyDef` — detailed arc property definition (v0.13.1: ADR-030 alignment)

use serde::Deserialize;
use smallvec::{SmallVec, smallvec};
use std::path::Path;

// ─────────────────────────────────────────────────────────────────────────────
// Enums
// ─────────────────────────────────────────────────────────────────────────────

/// The 6 arc families in v0.13.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArcFamily {
    Ownership,
    Localization,
    Semantic,
    Generation,
    Mining,
    Schema,
}

impl std::fmt::Display for ArcFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ownership => write!(f, "ownership"),
            Self::Localization => write!(f, "localization"),
            Self::Semantic => write!(f, "semantic"),
            Self::Generation => write!(f, "generation"),
            Self::Mining => write!(f, "mining"),
            Self::Schema => write!(f, "schema"),
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
// ArcPropertyDef — detailed arc property definition (v0.13.1: ADR-030 alignment)
// ─────────────────────────────────────────────────────────────────────────────

/// Detailed arc property definition extracted from YAML.
/// Captures type, required status, enum values per ADR-030 (TARGETS arc properties).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArcPropertyDef {
    /// Property name (e.g., "rank", "is_slug_source").
    pub name: String,
    /// Property type (e.g., "string", "boolean", "int", "float", "datetime", "array").
    pub prop_type: String,
    /// Whether the property is required.
    pub required: bool,
    /// Enum values if property is constrained (e.g., ["primary", "secondary", "tertiary"]).
    pub enum_values: Option<Vec<String>>,
    /// Property description for documentation.
    pub description: Option<String>,
    /// Default value if specified.
    pub default: Option<String>,
}

impl ArcPropertyDef {
    /// Create a simple property with just a name (for backward compatibility).
    pub fn simple(name: String) -> Self {
        Self {
            name,
            prop_type: "string".to_string(),
            required: false,
            enum_values: None,
            description: None,
            default: None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// YAML Structs
// ─────────────────────────────────────────────────────────────────────────────

/// Top-level arcs document (from relations.yaml).
#[derive(Debug, Deserialize)]
pub struct ArcsDocument {
    /// The arc definitions (v0.12.5: 146 arcs across 5 families).
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

    /// Arc scope: intra_realm (same realm) or cross_realm (different realms).
    /// Used by validation to verify source/target realms match declared scope.
    #[serde(default)]
    pub scope: Option<String>,

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

    /// Detailed arc property definitions (v0.13.1: ADR-030).
    /// Populated by load_arc_classes_from_files() for detailed property info.
    #[serde(skip)]
    pub property_defs: Option<Vec<ArcPropertyDef>>,

    /// True if source and target can be the same type (e.g. SEMANTIC_LINK).
    #[serde(default)]
    pub is_self_referential: Option<bool>,

    /// If this is an inverse arc, references the forward arc type.
    /// Used in relations.yaml format to mark inverse arcs that should be filtered.
    #[serde(default)]
    pub inverse_of: Option<String>,

    /// The name of this arc's inverse (for display purposes).
    /// e.g., HAS_PAGE has inverse "PAGE_OF"
    #[serde(skip)]
    pub inverse_name: Option<String>,
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
// Arc-Class YAML Files (v10.9 - consolidated from arc-classes/ directory)
// ─────────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

/// Individual arc-class YAML file structure (from arc-classes/{family}/*.yaml).
#[derive(Debug, Clone, Deserialize)]
pub struct ArcClassYaml {
    pub arc: ArcClassDef,
}

/// Arc definition within individual arc-class YAML file.
#[derive(Debug, Clone, Deserialize)]
pub struct ArcClassDef {
    /// Arc type name (SCREAMING_SNAKE_CASE), e.g., "HAS_PAGE".
    pub name: String,

    /// Arc family classification.
    pub family: ArcFamily,

    /// Scope (intra_realm or cross_realm).
    #[serde(default)]
    pub scope: Option<String>,

    /// Temperature threshold for spreading activation (0.0 - 1.0).
    #[serde(default)]
    pub temperature_threshold: Option<f32>,

    /// Source node label(s).
    pub source: NodeRef,

    /// Target node label(s).
    pub target: NodeRef,

    /// Cardinality constraint.
    pub cardinality: Cardinality,

    /// LLM context description.
    #[serde(default)]
    pub llm_context: Option<String>,

    /// Arc properties (optional) - can be list of strings, list of objects, or map.
    /// We store as opaque Value since format varies.
    #[serde(default)]
    pub properties: Option<serde_yaml::Value>,

    /// Inverse arc reference (optional).
    #[serde(default)]
    pub inverse: Option<String>,

    /// Cypher pattern (optional, can be auto-generated).
    #[serde(default)]
    pub cypher_pattern: Option<String>,
}

impl ArcClassDef {
    /// Extract property names from the various formats.
    fn extract_property_names(&self) -> Option<Vec<String>> {
        self.properties.as_ref().and_then(|v| match v {
            // List of strings: ["position", "status"]
            serde_yaml::Value::Sequence(seq) => {
                let names: Vec<String> = seq
                    .iter()
                    .filter_map(|item| {
                        // Could be string or object with "name" field
                        match item {
                            serde_yaml::Value::String(s) => Some(s.clone()),
                            serde_yaml::Value::Mapping(m) => m
                                .get(serde_yaml::Value::String("name".to_string()))
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            _ => None,
                        }
                    })
                    .collect();
                if names.is_empty() { None } else { Some(names) }
            }
            // Map: {segment: {type: string, ...}}
            serde_yaml::Value::Mapping(m) => {
                let names: Vec<String> = m
                    .keys()
                    .filter_map(|k| k.as_str().map(|s| s.to_string()))
                    .collect();
                if names.is_empty() { None } else { Some(names) }
            }
            _ => None,
        })
    }

    /// Extract detailed property definitions (v0.13.1: ADR-030 alignment).
    /// Returns full property info including type, required, enum values.
    fn extract_property_defs(&self) -> Option<Vec<ArcPropertyDef>> {
        self.properties.as_ref().and_then(|v| match v {
            // List format: [{name: "rank", type: "string", required: true, enum: [...]}]
            serde_yaml::Value::Sequence(seq) => {
                let defs: Vec<ArcPropertyDef> = seq
                    .iter()
                    .filter_map(|item| {
                        match item {
                            // Simple string: just the name
                            serde_yaml::Value::String(s) => Some(ArcPropertyDef::simple(s.clone())),
                            // Object with full definition
                            serde_yaml::Value::Mapping(m) => {
                                let name = m
                                    .get(serde_yaml::Value::String("name".to_string()))
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string())?;

                                let prop_type = m
                                    .get(serde_yaml::Value::String("type".to_string()))
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| "string".to_string());

                                let required = m
                                    .get(serde_yaml::Value::String("required".to_string()))
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or(false);

                                let enum_values = m
                                    .get(serde_yaml::Value::String("enum".to_string()))
                                    .and_then(|v| v.as_sequence())
                                    .map(|seq| {
                                        seq.iter()
                                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                            .collect()
                                    });

                                let description = m
                                    .get(serde_yaml::Value::String("description".to_string()))
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());

                                let default = m
                                    .get(serde_yaml::Value::String("default".to_string()))
                                    .map(|v| match v {
                                        serde_yaml::Value::Bool(b) => b.to_string(),
                                        serde_yaml::Value::Number(n) => n.to_string(),
                                        serde_yaml::Value::String(s) => s.clone(),
                                        _ => format!("{:?}", v),
                                    });

                                Some(ArcPropertyDef {
                                    name,
                                    prop_type,
                                    required,
                                    enum_values,
                                    description,
                                    default,
                                })
                            }
                            _ => None,
                        }
                    })
                    .collect();
                if defs.is_empty() { None } else { Some(defs) }
            }
            // Map format: {segment: {type: string, ...}}
            serde_yaml::Value::Mapping(m) => {
                let defs: Vec<ArcPropertyDef> = m
                    .iter()
                    .filter_map(|(k, v)| {
                        let name = k.as_str()?.to_string();

                        if let Some(def_map) = v.as_mapping() {
                            let prop_type = def_map
                                .get(serde_yaml::Value::String("type".to_string()))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "string".to_string());

                            let required = def_map
                                .get(serde_yaml::Value::String("required".to_string()))
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);

                            let enum_values = def_map
                                .get(serde_yaml::Value::String("enum".to_string()))
                                .and_then(|v| v.as_sequence())
                                .map(|seq| {
                                    seq.iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect()
                                });

                            let description = def_map
                                .get(serde_yaml::Value::String("description".to_string()))
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());

                            let default = def_map
                                .get(serde_yaml::Value::String("default".to_string()))
                                .map(|v| match v {
                                    serde_yaml::Value::Bool(b) => b.to_string(),
                                    serde_yaml::Value::Number(n) => n.to_string(),
                                    serde_yaml::Value::String(s) => s.clone(),
                                    _ => format!("{:?}", v),
                                });

                            Some(ArcPropertyDef {
                                name,
                                prop_type,
                                required,
                                enum_values,
                                description,
                                default,
                            })
                        } else {
                            // Simple string value: treat as type
                            Some(ArcPropertyDef::simple(name))
                        }
                    })
                    .collect();
                if defs.is_empty() { None } else { Some(defs) }
            }
            _ => None,
        })
    }

    /// Convert to ArcDef format for generator compatibility.
    ///
    /// Note: The `inverse` field in arc-kind YAML means "the inverse of THIS arc is called X",
    /// NOT "this arc IS the inverse of X". So we set `inverse_of` to None here.
    /// The inverse name is stored in `inverse_name` for display purposes in the generator.
    pub fn to_arc_def(&self) -> ArcDef {
        ArcDef {
            arc_type: self.name.clone(),
            family: self.family,
            scope: self.scope.clone(),
            source: self.source.clone(),
            target: self.target.clone(),
            cardinality: self.cardinality,
            llm_context: self.llm_context.clone().unwrap_or_default(),
            properties: self.extract_property_names(),
            property_defs: self.extract_property_defs(),
            is_self_referential: None,
            // Don't set inverse_of here - the `inverse` field means "this arc's inverse IS X",
            // not "this arc IS the inverse OF X". Setting inverse_of would incorrectly filter
            // this arc as an inverse relation.
            inverse_of: None,
            // Store the inverse name for display in the generator
            inverse_name: self.inverse.clone(),
        }
    }
}

/// Load all arc definitions from individual arc-class YAML files.
///
/// This is the v10.7+ replacement for `load_arcs()` which reads from relations.yaml.
/// Returns an ArcsDocument compatible with existing generator code.
pub fn load_arc_classes_from_files(root: &Path) -> crate::Result<ArcsDocument> {
    let arc_classes_dir = crate::config::arc_classes_dir(root);

    if !arc_classes_dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "arc-classes directory not found: {}",
            arc_classes_dir.display()
        )));
    }

    let mut arcs = Vec::new();

    // Scan all family directories
    for family_dir in std::fs::read_dir(&arc_classes_dir)? {
        let family_dir = family_dir?;
        if !family_dir.file_type()?.is_dir() {
            continue;
        }

        // Scan YAML files in each family directory
        for entry in std::fs::read_dir(family_dir.path())? {
            let entry = entry?;
            let path = entry.path();

            // Skip non-YAML and index files
            if path.extension().is_none_or(|e| e != "yaml") {
                continue;
            }
            if path.file_name().is_some_and(|n| n == "_index.yaml") {
                continue;
            }

            // Parse the arc-class YAML
            match super::utils::load_yaml::<ArcClassYaml>(&path) {
                Ok(yaml) => {
                    arcs.push(yaml.arc.to_arc_def());
                }
                Err(e) => {
                    // Try to get more detailed error info
                    let content = std::fs::read_to_string(&path).unwrap_or_default();
                    match serde_yaml::from_str::<serde_yaml::Value>(&content) {
                        Ok(v) => {
                            eprintln!(
                                "Warning: Failed to parse {}: {} (valid YAML but wrong schema)",
                                path.display(),
                                e
                            );
                            // Try to identify the issue
                            if let Some(arc) = v.get("arc") {
                                if arc.get("source").is_none() {
                                    eprintln!("  -> Missing 'source' field");
                                }
                                if arc.get("target").is_none() {
                                    eprintln!("  -> Missing 'target' field");
                                }
                                if arc.get("cardinality").is_none() {
                                    eprintln!("  -> Missing 'cardinality' field");
                                }
                            }
                        }
                        Err(_) => {
                            eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
    }

    if arcs.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "No arc-class YAML files found".to_string(),
        ));
    }

    // Sort by family then by type for deterministic output
    arcs.sort_by(|a, b| {
        a.family
            .to_string()
            .cmp(&b.family.to_string())
            .then_with(|| a.arc_type.cmp(&b.arc_type))
    });

    Ok(ArcsDocument {
        arcs,
        semantic_link_types: None,
        examples: None,
    })
}

/// Load temperature_threshold values from individual arc-class YAML files.
///
/// Returns a map of arc_type -> temperature_threshold.
pub fn load_arc_temperatures(root: &Path) -> crate::Result<HashMap<String, f32>> {
    let arc_classes_dir = crate::config::arc_classes_dir(root);
    let mut temps = HashMap::new();

    if !arc_classes_dir.exists() {
        return Ok(temps);
    }

    // Scan all family directories
    for family_dir in std::fs::read_dir(&arc_classes_dir)? {
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

            // Parse the arc-class YAML
            if let Ok(yaml) = super::utils::load_yaml::<ArcClassYaml>(&path) {
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
arcs:
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
arcs:
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
arcs:
  # v0.12.5: PageStructure deleted, use BELONGS_TO as multi-source/target example
  - type: BELONGS_TO
    family: semantic
    source:
      - Entity
      - Block
    target:
      - EntityCategory
      - BlockType
    cardinality: many_to_one
    llm_context: "Classification."
"#;
        let doc: ArcsDocument = serde_yaml::from_str(yaml).unwrap();
        let arc = &doc.arcs[0];
        assert_eq!(arc.source.labels().as_slice(), ["Entity", "Block"]);
        assert_eq!(
            arc.target.labels().as_slice(),
            ["EntityCategory", "BlockType"]
        );
    }

    #[test]
    fn parse_inverse_arc() {
        let yaml = r#"
arcs:
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
arcs:
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
    fn load_arc_classes_from_files_integration() {
        // Requires actual monorepo
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let doc = load_arc_classes_from_files(root).expect("should load arc-classes from files");

        // v11.0: Total arc count from individual arc-class files
        // Should have a reasonable number of arcs (more than legacy relations.yaml)
        assert!(
            doc.arcs.len() > 50,
            "expected at least 50 arcs, got {}",
            doc.arcs.len()
        );

        // All 5 families should be represented
        let family_count = |f: ArcFamily| doc.arcs.iter().filter(|a| a.family == f).count();
        assert!(
            family_count(ArcFamily::Ownership) > 0,
            "should have ownership arcs"
        );
        assert!(
            family_count(ArcFamily::Localization) > 0,
            "should have localization arcs"
        );
        assert!(
            family_count(ArcFamily::Semantic) > 0,
            "should have semantic arcs"
        );
        assert!(
            family_count(ArcFamily::Generation) > 0,
            "should have generation arcs"
        );
        assert!(
            family_count(ArcFamily::Mining) > 0,
            "should have mining arcs"
        );

        // All arcs have non-empty type
        for arc in &doc.arcs {
            assert!(!arc.arc_type.is_empty(), "empty arc_type");
            assert!(!arc.source.is_empty(), "empty source for {}", arc.arc_type);
            assert!(!arc.target.is_empty(), "empty target for {}", arc.arc_type);
        }

        // All arc types should be unique
        let mut types: Vec<&str> = doc.arcs.iter().map(|a| a.arc_type.as_str()).collect();
        types.sort();
        let original_len = types.len();
        types.dedup();
        assert_eq!(types.len(), original_len, "all arc types should be unique");

        // Spot-check: HAS_PAGE should exist
        let has_page = doc.arcs.iter().find(|a| a.arc_type == "HAS_PAGE");
        assert!(has_page.is_some(), "HAS_PAGE arc should exist");
        if let Some(has_page) = has_page {
            assert_eq!(has_page.family, ArcFamily::Ownership);
            assert_eq!(has_page.cardinality, Cardinality::OneToMany);
        }
    }

    #[test]
    fn parse_arc_class_yaml() {
        let yaml = r#"
arc:
  name: SEMANTIC_LINK
  family: semantic
  scope: intra_realm
  source: Entity
  target: Entity
  cardinality: many_to_many
  temperature_threshold: 0.3
"#;
        let parsed: ArcClassYaml = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(parsed.arc.name, "SEMANTIC_LINK");
        assert_eq!(parsed.arc.family, ArcFamily::Semantic);
        assert_eq!(parsed.arc.temperature_threshold, Some(0.3));
    }

    #[test]
    fn parse_arc_class_yaml_no_threshold() {
        let yaml = r#"
arc:
  name: HAS_PAGE
  family: ownership
  source: Project
  target: Page
  cardinality: one_to_many
"#;
        let parsed: ArcClassYaml = serde_yaml::from_str(yaml).unwrap();
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
            temps.contains_key("USES_ENTITY"),
            "USES_ENTITY should have threshold"
        );
        assert_eq!(temps.get("USES_ENTITY"), Some(&0.0_f32));

        // Should have multiple semantic arcs (v10.1: 10 after removing Thing arcs)
        let semantic_count = temps.len();
        assert!(
            semantic_count >= 10,
            "should have at least 10 semantic arcs with thresholds"
        );
    }
}

#[cfg(test)]
mod arc_class_tests {
    use super::*;

    #[test]
    fn parse_has_audience_yaml() {
        let yaml = r#"
arc:
  name: HAS_AUDIENCE
  family: ownership
  scope: intra_realm
  source: Locale
  target: AudienceSet
  cardinality: one_to_many
  llm_context: Locale has multiple audience sets, one per segment.
  cypher_pattern: "(Locale)-[:HAS_AUDIENCE {segment: $segment}]->(AudienceSet)"
  properties:
    segment:
      type: string
      required: true
      description: "Audience segment (b2b, b2c, general)"
"#;
        let result: Result<ArcClassYaml, _> = serde_yaml::from_str(yaml);
        match &result {
            Ok(v) => println!("Parsed: {}", v.arc.name),
            Err(e) => println!("Error: {}", e),
        }
        result.expect("should parse HAS_AUDIENCE");
    }
}

#[test]
fn parse_has_audience_file() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("should find root");

    let path = root.join("packages/core/models/arc-classes/ownership/has-audience.yaml");
    eprintln!("Loading from: {}", path.display());

    let content = std::fs::read_to_string(&path).expect("should read file");
    eprintln!("Content:\n{}", &content[..200.min(content.len())]);

    match serde_yaml::from_str::<ArcClassYaml>(&content) {
        Ok(v) => eprintln!("Parsed: {}", v.arc.name),
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Failed to parse: {}", e);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// v0.13.1: YAML/Cypher Alignment Tests for Arc Properties (ADR-030, ADR-032)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod arc_yaml_cypher_alignment_tests {
    use super::*;

    fn test_root() -> Option<std::path::PathBuf> {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest_dir.parent().and_then(|p| p.parent());
        let root = root?;
        if !root.join("pnpm-workspace.yaml").exists() {
            return None;
        }
        Some(root.to_path_buf())
    }

    /// Parse the TARGETS_KEYWORD arc YAML and verify priority + is_slug_source properties.
    /// ADR-030: is_slug_source marks the keyword used for URL slug derivation.
    #[test]
    fn targets_keyword_arc_has_priority_and_is_slug_source_properties() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let path = root.join("packages/core/models/arc-classes/mining/targets-keyword.yaml");
        let content =
            std::fs::read_to_string(&path).expect("should read TARGETS_KEYWORD arc YAML");
        let parsed: ArcClassYaml =
            serde_yaml::from_str(&content).expect("should parse TARGETS_KEYWORD arc YAML");

        assert_eq!(parsed.arc.name, "TARGETS_KEYWORD", "arc name mismatch");
        assert_eq!(
            parsed.arc.family,
            ArcFamily::Mining,
            "TARGETS_KEYWORD must be mining family"
        );

        // Check properties contains priority and is_slug_source
        let props = parsed
            .arc
            .properties
            .as_ref()
            .expect("TARGETS_KEYWORD must have properties");

        // Properties can be list of objects with name field
        if let serde_yaml::Value::Sequence(seq) = props {
            let prop_names: Vec<String> = seq
                .iter()
                .filter_map(|item| {
                    if let serde_yaml::Value::Mapping(m) = item {
                        m.get(serde_yaml::Value::String("name".to_string()))
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                    } else {
                        None
                    }
                })
                .collect();

            assert!(
                prop_names.contains(&"priority".to_string()),
                "TARGETS_KEYWORD must have 'priority' property (primary/secondary/tertiary)"
            );
            assert!(
                prop_names.contains(&"is_slug_source".to_string()),
                "TARGETS_KEYWORD must have 'is_slug_source' property (ADR-030)"
            );
        }
    }

    /// Parse the TARGETS_KEYWORD arc YAML and verify the priority enum values.
    #[test]
    fn targets_keyword_arc_priority_property_has_enum_values() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let path = root.join("packages/core/models/arc-classes/mining/targets-keyword.yaml");
        let content =
            std::fs::read_to_string(&path).expect("should read TARGETS_KEYWORD arc YAML");
        let parsed: ArcClassYaml =
            serde_yaml::from_str(&content).expect("should parse TARGETS_KEYWORD arc YAML");

        let props = parsed.arc.properties.as_ref().unwrap();
        if let serde_yaml::Value::Sequence(seq) = props {
            // Find the priority property
            let priority_prop = seq.iter().find(|item| {
                if let serde_yaml::Value::Mapping(m) = item {
                    m.get(serde_yaml::Value::String("name".to_string()))
                        .and_then(|v| v.as_str())
                        == Some("priority")
                } else {
                    false
                }
            });

            let priority_prop = priority_prop.expect("priority property must exist");
            if let serde_yaml::Value::Mapping(m) = priority_prop {
                let enum_val = m
                    .get(serde_yaml::Value::String("enum".to_string()))
                    .expect("priority property must have enum");

                if let serde_yaml::Value::Sequence(enum_seq) = enum_val {
                    let enum_strs: Vec<&str> =
                        enum_seq.iter().filter_map(|v| v.as_str()).collect();
                    assert!(
                        enum_strs.contains(&"primary"),
                        "priority enum must contain 'primary'"
                    );
                    assert!(
                        enum_strs.contains(&"secondary"),
                        "priority enum must contain 'secondary'"
                    );
                    assert!(
                        enum_strs.contains(&"tertiary"),
                        "priority enum must contain 'tertiary'"
                    );
                }
            }
        }
    }

    /// Parse the DERIVED_SLUG_FROM arc YAML and verify derivation properties.
    /// ADR-030: Audit trail for slug derivation decisions.
    #[test]
    fn derived_slug_from_arc_has_derivation_properties() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let path = root.join("packages/core/models/arc-classes/generation/derived-slug-from.yaml");
        let content =
            std::fs::read_to_string(&path).expect("should read DERIVED_SLUG_FROM arc YAML");
        let parsed: ArcClassYaml =
            serde_yaml::from_str(&content).expect("should parse DERIVED_SLUG_FROM arc YAML");

        assert_eq!(parsed.arc.name, "DERIVED_SLUG_FROM", "arc name mismatch");
        assert_eq!(
            parsed.arc.family,
            ArcFamily::Generation,
            "DERIVED_SLUG_FROM must be generation family"
        );

        // Verify source and target
        assert_eq!(
            parsed.arc.source.labels().as_slice(),
            ["BlockNative"],
            "DERIVED_SLUG_FROM source must be BlockNative"
        );
        assert_eq!(
            parsed.arc.target.labels().as_slice(),
            ["EntityNative"],
            "DERIVED_SLUG_FROM target must be EntityNative"
        );

        // Check properties
        let props = parsed
            .arc
            .properties
            .as_ref()
            .expect("DERIVED_SLUG_FROM must have properties");

        if let serde_yaml::Value::Sequence(seq) = props {
            let prop_names: Vec<String> = seq
                .iter()
                .filter_map(|item| {
                    if let serde_yaml::Value::Mapping(m) = item {
                        m.get(serde_yaml::Value::String("name".to_string()))
                            .and_then(|v| v.as_str().map(|s| s.to_string()))
                    } else {
                        None
                    }
                })
                .collect();

            assert!(
                prop_names.contains(&"derivation_score".to_string()),
                "DERIVED_SLUG_FROM must have 'derivation_score' property"
            );
            assert!(
                prop_names.contains(&"derivation_rationale".to_string()),
                "DERIVED_SLUG_FROM must have 'derivation_rationale' property"
            );
            assert!(
                prop_names.contains(&"derivation_timestamp".to_string()),
                "DERIVED_SLUG_FROM must have 'derivation_timestamp' property"
            );
            assert!(
                prop_names.contains(&"no_repetition_applied".to_string()),
                "DERIVED_SLUG_FROM must have 'no_repetition_applied' property (ADR-032)"
            );
        }
    }

    /// Unit test: Parse TARGETS arc with all properties inline.
    #[test]
    fn parse_targets_arc_yaml_inline() {
        let yaml = r#"
arc:
  name: TARGETS
  family: semantic
  scope: cross_realm
  temperature_threshold: 0.6
  source: EntityNative
  target: SEOKeyword
  cardinality: many_to_many
  properties:
    - name: rank
      type: string
      required: true
      enum: [primary, secondary, tertiary]
      description: "Targeting rank for this keyword"
    - name: is_slug_source
      type: boolean
      required: false
      default: false
      description: "True if this keyword's slug_form was used for the URL slug"
    - name: target_position
      type: int
      required: false
      description: "Target ranking position (1-10)"
    - name: created_at
      type: datetime
      required: false
      description: "Arc creation timestamp"
  llm_context: |
    USE: when finding SEO keywords targeted by localized content.
    TRIGGERS: targets keyword, SEO targeting.
"#;
        let parsed: ArcClassYaml = serde_yaml::from_str(yaml).expect("should parse TARGETS yaml");
        assert_eq!(parsed.arc.name, "TARGETS");
        assert_eq!(parsed.arc.family, ArcFamily::Semantic);
        assert_eq!(parsed.arc.temperature_threshold, Some(0.6));

        // Convert to ArcDef and verify properties are extracted
        let arc_def = parsed.arc.to_arc_def();
        let props = arc_def.properties.expect("should have properties");
        assert!(props.contains(&"rank".to_string()), "should extract 'rank'");
        assert!(
            props.contains(&"is_slug_source".to_string()),
            "should extract 'is_slug_source'"
        );
    }

    /// Unit test: Parse DERIVED_SLUG_FROM arc with all properties inline.
    #[test]
    fn parse_derived_slug_from_arc_yaml_inline() {
        let yaml = r#"
arc:
  name: DERIVED_SLUG_FROM
  family: generation
  scope: intra_realm
  source: BlockNative
  target: EntityNative
  cardinality: many_to_one
  properties:
    - name: derivation_score
      type: float
      required: true
      description: "Score = volume x sem_coef x convergence_boost"
    - name: derivation_rationale
      type: string
      required: false
      description: "LLM explanation of why this keyword was chosen"
    - name: derivation_timestamp
      type: datetime
      required: false
      description: "When the slug derivation was computed"
    - name: no_repetition_applied
      type: boolean
      required: false
      default: false
      description: "Whether the no-repetition rule modified the slug"
    - name: brand_invariant
      type: boolean
      required: false
      default: false
      description: "Whether slug is brand name (no translation)"
"#;
        let parsed: ArcClassYaml =
            serde_yaml::from_str(yaml).expect("should parse DERIVED_SLUG_FROM yaml");
        assert_eq!(parsed.arc.name, "DERIVED_SLUG_FROM");
        assert_eq!(parsed.arc.family, ArcFamily::Generation);
        assert_eq!(parsed.arc.cardinality, Cardinality::ManyToOne);

        // Convert to ArcDef and verify properties are extracted
        let arc_def = parsed.arc.to_arc_def();
        let props = arc_def.properties.expect("should have properties");
        assert!(
            props.contains(&"derivation_score".to_string()),
            "should extract 'derivation_score'"
        );
        assert!(
            props.contains(&"derivation_timestamp".to_string()),
            "should extract 'derivation_timestamp'"
        );
        assert!(
            props.contains(&"no_repetition_applied".to_string()),
            "should extract 'no_repetition_applied'"
        );
    }

    /// Integration test: Verify all arc files load successfully.
    #[test]
    fn all_arc_yaml_files_parse_successfully() {
        let Some(root) = test_root() else {
            eprintln!("Skipping: not in monorepo");
            return;
        };

        let doc = load_arc_classes_from_files(&root).expect("should load all arc classes");

        // v0.17.1: Should have TARGETS_KEYWORD and DERIVED_SLUG_FROM
        let targets = doc.arcs.iter().find(|a| a.arc_type == "TARGETS_KEYWORD");
        assert!(targets.is_some(), "TARGETS_KEYWORD arc must exist");
        let targets = targets.unwrap();
        assert_eq!(targets.family, ArcFamily::Mining);

        let derived = doc.arcs.iter().find(|a| a.arc_type == "DERIVED_SLUG_FROM");
        assert!(derived.is_some(), "DERIVED_SLUG_FROM arc must exist");
        let derived = derived.unwrap();
        assert_eq!(derived.family, ArcFamily::Generation);
    }
}
