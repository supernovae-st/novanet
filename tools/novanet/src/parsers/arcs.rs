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
    /// The 116 arc definitions (v11.3).
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
// Arc-Kind YAML Files (v10.9 - consolidated from arc-kinds/ directory)
// ─────────────────────────────────────────────────────────────────────────────

use std::collections::HashMap;

/// Individual arc-kind YAML file structure (from arc-kinds/{family}/*.yaml).
#[derive(Debug, Clone, Deserialize)]
pub struct ArcKindYaml {
    pub arc: ArcKindDef,
}

/// Arc definition within individual arc-kind YAML file.
#[derive(Debug, Clone, Deserialize)]
pub struct ArcKindDef {
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

impl ArcKindDef {
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

/// Load all arc definitions from individual arc-kind YAML files.
///
/// This is the v10.7+ replacement for `load_arcs()` which reads from relations.yaml.
/// Returns an ArcsDocument compatible with existing generator code.
pub fn load_arc_kinds_from_files(root: &Path) -> crate::Result<ArcsDocument> {
    let arc_kinds_dir = crate::config::arc_kinds_dir(root);

    if !arc_kinds_dir.exists() {
        return Err(crate::NovaNetError::Validation(format!(
            "arc-kinds directory not found: {}",
            arc_kinds_dir.display()
        )));
    }

    let mut arcs = Vec::new();

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

            // Skip non-YAML and index files
            if path.extension().is_none_or(|e| e != "yaml") {
                continue;
            }
            if path.file_name().is_some_and(|n| n == "_index.yaml") {
                continue;
            }

            // Parse the arc-kind YAML
            match super::utils::load_yaml::<ArcKindYaml>(&path) {
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
            "No arc-kind YAML files found".to_string(),
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
    fn load_arc_kinds_from_files_integration() {
        // Requires actual monorepo
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent());

        let Some(root) = root else { return };
        if !root.join("pnpm-workspace.yaml").exists() {
            return;
        }

        let doc = load_arc_kinds_from_files(root).expect("should load arc-kinds from files");

        // v11.0: Total arc count from individual arc-kind files
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
    fn parse_arc_kind_yaml() {
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
  source: Project
  target: Page
  cardinality: one_to_many
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
mod arc_kind_tests {
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
        let result: Result<ArcKindYaml, _> = serde_yaml::from_str(yaml);
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

    let path = root.join("packages/core/models/arc-kinds/ownership/has-audience.yaml");
    eprintln!("Loading from: {}", path.display());

    let content = std::fs::read_to_string(&path).expect("should read file");
    eprintln!("Content:\n{}", &content[..200.min(content.len())]);

    match serde_yaml::from_str::<ArcKindYaml>(&content) {
        Ok(v) => eprintln!("Parsed: {}", v.arc.name),
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Failed to parse: {}", e);
        }
    }
}
