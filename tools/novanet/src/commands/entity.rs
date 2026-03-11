//! `novanet entity seed` command.
//!
//! Seeds Entity nodes and typed semantic arcs from phase YAML files.
//! Data source: `packages/core/data/entities/{project}/phase-*.yaml`
//!
//! # v11 Typed Semantic Arcs
//!
//! Instead of generic SEMANTIC_LINK with link_type property, v11 uses typed arcs:
//! - VARIANT_OF / HAS_VARIANT
//! - REQUIRES / REQUIRED_BY
//! - ENABLES / ENABLED_BY
//! - TYPE_OF / HAS_TYPE
//! - INCLUDES / INCLUDED_IN
//! - SIMILAR_TO (bidirectional)
//! - ALTERNATIVE_TO (bidirectional)
//! - COMPETES_WITH (bidirectional)
//! - APPLIES_TO / HAS_APPLICATION

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Instant;

use serde::Deserialize;
use tracing::instrument;

use crate::Result;
use crate::error::NovaNetError;

// ═══════════════════════════════════════════════════════════════════════════════
// YAML TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Entity phase file structure.
#[derive(Debug, Deserialize)]
pub struct EntityPhaseFile {
    pub project: String,
    pub phase: u32,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub entities: Vec<EntityDef>,
    #[serde(default)]
    pub arcs: Vec<ArcDef>,
}

/// Entity definition.
#[derive(Debug, Deserialize)]
pub struct EntityDef {
    pub key: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    #[serde(default)]
    pub is_pillar: bool,
    pub display_name: String,
    pub description: String,
    #[serde(default)]
    pub entity_summary: Option<String>,
    #[serde(default)]
    pub llm_context: Option<String>,
    #[serde(default)]
    pub schema_org_type: Option<String>,
}

/// Arc definition between entities.
#[derive(Debug, Deserialize)]
pub struct ArcDef {
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub arc_type: String,
    #[serde(default = "default_strength")]
    pub strength: f64,
}

fn default_strength() -> f64 {
    0.80
}

// ═══════════════════════════════════════════════════════════════════════════════
// VALIDATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Valid Entity.type values (13 types).
const VALID_ENTITY_TYPES: &[&str] = &[
    "THING",
    "CONTENT_TYPE",
    "FEATURE",
    "TOOL",
    "MEDIUM",
    "USE_CASE",
    "INDUSTRY",
    "ACTION",
    "GUIDE",
    "COMPARISON",
    "CONCEPT",
    "BRAND",
    "INTEGRATION",
];

/// Valid typed semantic arc types (v11).
const VALID_ARC_TYPES: &[&str] = &[
    // Hierarchical
    "VARIANT_OF",
    "HAS_VARIANT",
    "REQUIRES",
    "REQUIRED_BY",
    "ENABLES",
    "ENABLED_BY",
    "TYPE_OF",
    "HAS_TYPE",
    "INCLUDES",
    "INCLUDED_IN",
    // Bidirectional semantic
    "SIMILAR_TO",
    "ALTERNATIVE_TO",
    "COMPETES_WITH",
    "APPLIES_TO",
    "HAS_APPLICATION",
    // Removed: "SUBTOPIC_OF" (v10 legacy, replaced by VARIANT_OF/TYPE_OF in v11)
];

/// Validate entity type.
fn validate_entity_type(entity_type: &str) -> Result<()> {
    if VALID_ENTITY_TYPES.contains(&entity_type) {
        Ok(())
    } else {
        Err(NovaNetError::Validation(format!(
            "Invalid entity type '{}'. Valid types: {}",
            entity_type,
            VALID_ENTITY_TYPES.join(", ")
        )))
    }
}

/// Validate arc type.
fn validate_arc_type(arc_type: &str) -> Result<()> {
    if VALID_ARC_TYPES.contains(&arc_type) {
        Ok(())
    } else {
        Err(NovaNetError::Validation(format!(
            "Invalid arc type '{}'. Valid types: {}",
            arc_type,
            VALID_ARC_TYPES.join(", ")
        )))
    }
}

/// Validate all entities and arcs in a phase file.
fn validate_phase(phase: &EntityPhaseFile) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Collect all entity keys
    let entity_keys: HashSet<&str> = phase.entities.iter().map(|e| e.key.as_str()).collect();

    // Validate entity types
    for entity in &phase.entities {
        validate_entity_type(&entity.entity_type)?;

        // Check key format (kebab-case)
        if !entity
            .key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
        {
            warnings.push(format!(
                "Entity key '{}' should be kebab-case (lowercase + hyphens)",
                entity.key
            ));
        }
    }

    // Validate arcs
    for arc in &phase.arcs {
        validate_arc_type(&arc.arc_type)?;

        // Check arc targets exist (warning only - might be in different phase)
        if !entity_keys.contains(arc.from.as_str()) {
            warnings.push(format!(
                "Arc source '{}' not found in this phase (might be in another phase)",
                arc.from
            ));
        }
        if !entity_keys.contains(arc.to.as_str()) {
            warnings.push(format!(
                "Arc target '{}' not found in this phase (might be in another phase)",
                arc.to
            ));
        }

        // Check strength range
        if !(0.0..=1.0).contains(&arc.strength) {
            return Err(NovaNetError::Validation(format!(
                "Arc strength {} out of range [0.0, 1.0]",
                arc.strength
            )));
        }
    }

    Ok(warnings)
}

// ═══════════════════════════════════════════════════════════════════════════════
// CYPHER GENERATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Escape string for Cypher.
fn escape_cypher(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Generate Cypher for a phase file.
fn generate_cypher(phase: &EntityPhaseFile) -> String {
    let mut cypher = String::new();

    // Header
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n",
    );
    cypher.push_str(&format!(
        "// Phase {}: {} — {} entities, {} arcs\n",
        phase.phase,
        phase.name,
        phase.entities.len(),
        phase.arcs.len()
    ));
    cypher.push_str(&format!("// Project: {}\n", phase.project));
    cypher.push_str("// Generated by: novanet entity seed\n");
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n\n",
    );

    // Merge Project node
    cypher.push_str(
        "// ─────────────────────────────────────────────────────────────────────────────\n",
    );
    cypher.push_str("// PROJECT NODE\n");
    cypher.push_str(
        "// ─────────────────────────────────────────────────────────────────────────────\n\n",
    );
    // Use MATCH since Project must exist (created in 30-tenant-config.cypher)
    cypher.push_str(&format!(
        "MATCH (proj:Project {{key: \"{}\"}})\n",
        escape_cypher(&phase.project)
    ));
    cypher.push_str("SET proj.updated_at = datetime();\n\n");

    // Entity nodes
    if !phase.entities.is_empty() {
        cypher.push_str(
            "// ─────────────────────────────────────────────────────────────────────────────\n",
        );
        cypher.push_str(&format!("// ENTITIES ({})\n", phase.entities.len()));
        cypher.push_str(
            "// ─────────────────────────────────────────────────────────────────────────────\n\n",
        );

        for entity in &phase.entities {
            cypher.push_str(&generate_entity_cypher(entity, &phase.project));
        }
    }

    // Semantic arcs
    if !phase.arcs.is_empty() {
        cypher.push_str(
            "\n// ─────────────────────────────────────────────────────────────────────────────\n",
        );
        cypher.push_str(&format!("// SEMANTIC ARCS ({})\n", phase.arcs.len()));
        cypher.push_str(
            "// ─────────────────────────────────────────────────────────────────────────────\n\n",
        );

        for arc in &phase.arcs {
            cypher.push_str(&generate_arc_cypher(arc, &phase.project));
        }
    }

    cypher
}

/// Generate Cypher for a single entity.
fn generate_entity_cypher(entity: &EntityDef, project: &str) -> String {
    let mut cypher = String::new();

    // MERGE Entity node
    cypher.push_str(&format!(
        "MERGE (e:Entity {{key: \"{}\"}})\n",
        escape_cypher(&entity.key)
    ));

    // ON CREATE SET
    cypher.push_str("ON CREATE SET\n");
    cypher.push_str(&format!(
        "  e.display_name = \"{}\",\n",
        escape_cypher(&entity.display_name)
    ));
    cypher.push_str(&format!(
        "  e.content = \"{}\",\n",
        escape_cypher(&entity.description)
    ));
    cypher.push_str(&format!("  e.type = \"{}\",\n", entity.entity_type));
    cypher.push_str(&format!("  e.is_pillar = {},\n", entity.is_pillar));

    if let Some(summary) = &entity.entity_summary {
        cypher.push_str(&format!(
            "  e.entity_summary = \"{}\",\n",
            escape_cypher(summary.trim())
        ));
    }

    if let Some(llm_context) = &entity.llm_context {
        cypher.push_str(&format!(
            "  e.llm_context = \"{}\",\n",
            escape_cypher(llm_context.trim())
        ));
    }

    if let Some(schema_org) = &entity.schema_org_type {
        cypher.push_str(&format!(
            "  e.schema_org_type = \"{}\",\n",
            escape_cypher(schema_org)
        ));
    }

    cypher.push_str("  e.created_at = datetime(),\n");
    cypher.push_str("  e.updated_at = datetime()\n");

    // ON MATCH SET (update timestamps)
    cypher.push_str("ON MATCH SET\n");
    cypher.push_str("  e.updated_at = datetime();\n");

    // Wire to Project via HAS_ENTITY
    cypher.push_str(&format!(
        "\nMATCH (proj:Project {{key: \"{}\"}})\n",
        escape_cypher(project)
    ));
    cypher.push_str(&format!(
        "MATCH (e:Entity {{key: \"{}\"}})\n",
        escape_cypher(&entity.key)
    ));
    cypher.push_str("MERGE (proj)-[:HAS_ENTITY]->(e);\n\n");

    cypher
}

/// Generate Cypher for a semantic arc.
fn generate_arc_cypher(arc: &ArcDef, _project: &str) -> String {
    let mut cypher = String::new();

    // Match source and target
    cypher.push_str(&format!(
        "MATCH (from:Entity {{key: \"{}\"}})\n",
        escape_cypher(&arc.from)
    ));
    cypher.push_str(&format!(
        "MATCH (to:Entity {{key: \"{}\"}})\n",
        escape_cypher(&arc.to)
    ));

    // Create typed arc with strength property
    cypher.push_str(&format!("MERGE (from)-[r:{}]->(to)\n", arc.arc_type));
    cypher.push_str(&format!(
        "ON CREATE SET r.strength = {:.2}, r.created_at = datetime()\n",
        arc.strength
    ));
    cypher.push_str("ON MATCH SET r.updated_at = datetime();\n\n");

    cypher
}

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC API
// ═══════════════════════════════════════════════════════════════════════════════

/// Result of entity seed operation.
pub struct EntitySeedResult {
    pub phase: u32,
    pub phase_name: String,
    pub output_path: String,
    pub entity_count: usize,
    pub arc_count: usize,
    pub bytes: usize,
    pub duration_ms: u128,
    pub warnings: Vec<String>,
}

/// Seed entities for a project.
///
/// If `phase` is None, seeds all phases found.
/// If `phase` is Some(n), seeds only phase n.
#[instrument(skip(root))]
pub fn entity_seed(
    root: &Path,
    project: &str,
    phase: Option<u32>,
    dry_run: bool,
) -> Result<Vec<EntitySeedResult>> {
    let mut results = Vec::new();

    // Find entity data directory
    let data_dir = root.join(format!("packages/core/data/entities/{}", project));

    if !data_dir.exists() {
        return Err(NovaNetError::Validation(format!(
            "Entity data directory not found: {}",
            data_dir.display()
        )));
    }

    // Find phase YAML files
    let mut phase_files: Vec<_> = fs::read_dir(&data_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_name().to_string_lossy().starts_with("phase-")
                && entry.file_name().to_string_lossy().ends_with(".yaml")
        })
        .collect();

    // Sort by filename
    phase_files.sort_by_key(|e| e.file_name());

    if phase_files.is_empty() {
        return Err(NovaNetError::Validation(format!(
            "No phase YAML files found in {}",
            data_dir.display()
        )));
    }

    // Process each phase file
    for entry in phase_files {
        let start = Instant::now();
        let path = entry.path();

        // Parse YAML
        let content = fs::read_to_string(&path)?;
        let phase_data: EntityPhaseFile =
            serde_yaml::from_str(&content).map_err(|e| NovaNetError::Schema {
                path: path.display().to_string(),
                source: e,
            })?;

        // Filter by phase number if specified
        if let Some(target_phase) = phase {
            if phase_data.phase != target_phase {
                continue;
            }
        }

        // Validate
        let warnings = validate_phase(&phase_data)?;

        // Generate Cypher
        let cypher = generate_cypher(&phase_data);

        // Output path
        let output_filename = format!("{}-phase-{:02}.cypher", project, phase_data.phase);
        let output_dir = root.join("packages/db/seed/entities");
        let output_path = output_dir.join(&output_filename);

        // Write file (unless dry run)
        if !dry_run {
            fs::create_dir_all(&output_dir)?;
            fs::write(&output_path, &cypher)?;
        }

        let duration = start.elapsed();

        results.push(EntitySeedResult {
            phase: phase_data.phase,
            phase_name: phase_data.name,
            output_path: format!("packages/db/seed/entities/{}", output_filename),
            entity_count: phase_data.entities.len(),
            arc_count: phase_data.arcs.len(),
            bytes: cypher.len(),
            duration_ms: duration.as_millis(),
            warnings,
        });
    }

    Ok(results)
}

/// List available phases for a project.
pub fn entity_list(root: &Path, project: &str) -> Result<Vec<EntityPhaseInfo>> {
    let data_dir = root.join(format!("packages/core/data/entities/{}", project));

    if !data_dir.exists() {
        return Ok(Vec::new());
    }

    let mut phases = Vec::new();

    for entry in fs::read_dir(&data_dir)? {
        let entry = entry?;
        let filename = entry.file_name().to_string_lossy().to_string();

        if filename.starts_with("phase-") && filename.ends_with(".yaml") {
            let content = fs::read_to_string(entry.path())?;
            let phase_data: EntityPhaseFile =
                serde_yaml::from_str(&content).map_err(|e| NovaNetError::Schema {
                    path: entry.path().display().to_string(),
                    source: e,
                })?;

            phases.push(EntityPhaseInfo {
                phase: phase_data.phase,
                name: phase_data.name,
                entity_count: phase_data.entities.len(),
                arc_count: phase_data.arcs.len(),
                file: filename,
            });
        }
    }

    phases.sort_by_key(|p| p.phase);
    Ok(phases)
}

/// Validate entity data without generating.
pub fn entity_validate(root: &Path, project: &str) -> Result<Vec<EntityValidationResult>> {
    let data_dir = root.join(format!("packages/core/data/entities/{}", project));

    if !data_dir.exists() {
        return Err(NovaNetError::Validation(format!(
            "Entity data directory not found: {}",
            data_dir.display()
        )));
    }

    let mut results = Vec::new();

    for entry in fs::read_dir(&data_dir)? {
        let entry = entry?;
        let filename = entry.file_name().to_string_lossy().to_string();

        if filename.starts_with("phase-") && filename.ends_with(".yaml") {
            let content = fs::read_to_string(entry.path())?;
            let phase_data: EntityPhaseFile =
                serde_yaml::from_str(&content).map_err(|e| NovaNetError::Schema {
                    path: entry.path().display().to_string(),
                    source: e,
                })?;

            match validate_phase(&phase_data) {
                Ok(warnings) => {
                    results.push(EntityValidationResult {
                        phase: phase_data.phase,
                        file: filename,
                        valid: true,
                        errors: Vec::new(),
                        warnings,
                    });
                },
                Err(e) => {
                    results.push(EntityValidationResult {
                        phase: phase_data.phase,
                        file: filename,
                        valid: false,
                        errors: vec![e.to_string()],
                        warnings: Vec::new(),
                    });
                },
            }
        }
    }

    results.sort_by_key(|r| r.phase);
    Ok(results)
}

/// Phase info for listing.
pub struct EntityPhaseInfo {
    pub phase: u32,
    pub name: String,
    pub entity_count: usize,
    pub arc_count: usize,
    pub file: String,
}

/// Validation result for a phase.
pub struct EntityValidationResult {
    pub phase: u32,
    pub file: String,
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_entity_type_valid() {
        assert!(validate_entity_type("THING").is_ok());
        assert!(validate_entity_type("ACTION").is_ok());
        assert!(validate_entity_type("CONCEPT").is_ok());
        assert!(validate_entity_type("BRAND").is_ok());
    }

    #[test]
    fn test_validate_entity_type_invalid() {
        assert!(validate_entity_type("INVALID").is_err());
        assert!(validate_entity_type("thing").is_err());
        assert!(validate_entity_type("").is_err());
    }

    #[test]
    fn test_validate_arc_type_valid() {
        assert!(validate_arc_type("INCLUDES").is_ok());
        assert!(validate_arc_type("ENABLES").is_ok());
        assert!(validate_arc_type("SIMILAR_TO").is_ok());
        assert!(validate_arc_type("VARIANT_OF").is_ok());
    }

    #[test]
    fn test_validate_arc_type_invalid() {
        assert!(validate_arc_type("INVALID").is_err());
        assert!(validate_arc_type("includes").is_err());
    }

    #[test]
    fn test_escape_cypher() {
        assert_eq!(escape_cypher("hello"), "hello");
        assert_eq!(escape_cypher("hello\"world"), "hello\\\"world");
        assert_eq!(escape_cypher("line1\nline2"), "line1\\nline2");
        assert_eq!(escape_cypher("path\\file"), "path\\\\file");
    }

    #[test]
    fn test_parse_entity_phase_file() {
        let yaml = r#"
project: test-project
phase: 1
name: Test Phase
description: A test phase

entities:
  - key: test-entity
    type: THING
    is_pillar: true
    display_name: Test Entity
    description: A test entity

arcs:
  - from: test-entity
    to: other-entity
    type: INCLUDES
    strength: 0.80
"#;

        let phase: EntityPhaseFile = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(phase.project, "test-project");
        assert_eq!(phase.phase, 1);
        assert_eq!(phase.entities.len(), 1);
        assert_eq!(phase.arcs.len(), 1);
    }

    #[test]
    fn test_generate_entity_cypher() {
        let entity = EntityDef {
            key: "test-entity".to_string(),
            entity_type: "THING".to_string(),
            is_pillar: true,
            display_name: "Test Entity".to_string(),
            description: "A test entity".to_string(),
            entity_summary: Some("This is a summary.".to_string()),
            llm_context: None,
            schema_org_type: None,
        };

        let cypher = generate_entity_cypher(&entity, "test-project");

        assert!(cypher.contains("MERGE (e:Entity {key: \"test-entity\"})"));
        assert!(cypher.contains("e.type = \"THING\""));
        assert!(cypher.contains("e.is_pillar = true"));
        assert!(cypher.contains("e.entity_summary = \"This is a summary.\""));
        assert!(cypher.contains("[:HAS_ENTITY]"));
    }

    #[test]
    fn test_generate_arc_cypher() {
        let arc = ArcDef {
            from: "entity-a".to_string(),
            to: "entity-b".to_string(),
            arc_type: "ENABLES".to_string(),
            strength: 0.90,
        };

        let cypher = generate_arc_cypher(&arc, "test-project");

        assert!(cypher.contains("MATCH (from:Entity {key: \"entity-a\"})"));
        assert!(cypher.contains("MATCH (to:Entity {key: \"entity-b\"})"));
        assert!(cypher.contains("MERGE (from)-[r:ENABLES]->(to)"));
        assert!(cypher.contains("r.strength = 0.90"));
    }
}
