//! Generator for Expression Atoms Cypher statements.
//!
//! Transforms parsed ATH 3-voice-lexicon data into Neo4j seed file.
//! Creates ExpressionSet (container) and Expression (atom) nodes per locale.

#![allow(clippy::needless_raw_string_hashes)]

use std::path::PathBuf;

use chrono::Local;

use crate::config::resolve_ath_path;
use crate::generators::cypher_utils::escape_cypher;
use crate::parsers::expression::{Expression, ExpressionData, SemanticField, load_all_expressions};
use crate::{NovaNetError, Result};

/// Generate Cypher for Expression atoms.
pub struct ExpressionGenerator {
    ath_path: PathBuf,
}

impl ExpressionGenerator {
    /// Create a generator with ATH path from env var or explicit path.
    pub fn new(explicit_path: Option<&str>) -> Result<Self> {
        Ok(Self {
            ath_path: resolve_ath_path(explicit_path)?,
        })
    }

    /// Generate the complete Cypher file content.
    pub fn generate(&self) -> Result<String> {
        // Load all expression files
        let expressions = load_all_expressions(&self.ath_path)?;

        if expressions.is_empty() {
            return Err(NovaNetError::Validation(
                "No expression files found".to_string(),
            ));
        }

        // Count total atoms
        let total_atoms: usize = expressions.iter().map(|e| e.total_expressions()).sum();

        // Generate Cypher
        let mut output = String::new();

        // Header
        output.push_str(&self.generate_header(&expressions, total_atoms));

        // Part 1: ExpressionSet nodes (containers)
        output.push_str(&self.generate_expression_sets_section(&expressions));

        // Part 2: Expression nodes (atoms)
        output.push_str(&self.generate_expression_atoms_section(&expressions));

        // Part 3: Locale → ExpressionSet arcs
        output.push_str(&self.generate_locale_arcs_section(&expressions));

        // Part 4: ExpressionSet → Expression arcs
        output.push_str(&self.generate_contains_arcs_section(&expressions));

        Ok(output)
    }

    /// Generate file header.
    fn generate_header(&self, expressions: &[ExpressionData], total_atoms: usize) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let locale_count = expressions.len();

        format!(
            r#"// ============================================================================
// EXPRESSION ATOMS SEED - Generated from ATH 3-voice-lexicon
// Generated: {}
// Source: {}/3-voice-lexicon/
// Locales: {}
// Total Expression atoms: {}
// ============================================================================

"#,
            timestamp,
            self.ath_path.display(),
            locale_count,
            total_atoms
        )
    }

    /// Generate ExpressionSet nodes section.
    fn generate_expression_sets_section(&self, expressions: &[ExpressionData]) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            r#"// ----------------------------------------------------------------------------
// PART 1: ExpressionSet nodes (containers) - {} locales
// ----------------------------------------------------------------------------

"#,
            expressions.len()
        ));

        for e in expressions {
            output.push_str(&self.generate_expression_set_cypher(e));
            output.push('\n');
        }

        output
    }

    /// Generate Cypher for a single ExpressionSet node.
    fn generate_expression_set_cypher(&self, e: &ExpressionData) -> String {
        // Build semantic fields summary JSON
        let fields_summary: Vec<serde_json::Value> = e
            .semantic_fields
            .iter()
            .map(|f| {
                serde_json::json!({
                    "name": f.name,
                    "local_name": f.local_name,
                    "count": f.expressions.len(),
                })
            })
            .collect();
        let fields_json =
            serde_json::to_string(&fields_summary).unwrap_or_else(|_| "[]".to_string());

        format!(
            r#"MERGE (es:ExpressionSet {{key: '{}'}})
SET es.display_name = '{}',
    es.content = '{}',
    es.semantic_fields_count = {},
    es.total_expressions = {},
    es.semantic_fields = '{}',
    es.llm_context = '{}',
    es.template_version = '{}',
    es.source_file = '{}',
    es.last_updated = '{}';
"#,
            e.locale_key,
            escape_cypher(&format!("{} Expressions", e.locale_key)),
            escape_cypher(&format!(
                "Native expressions for {} across {} semantic fields",
                e.locale_key,
                e.semantic_fields.len()
            )),
            e.semantic_fields.len(),
            e.total_expressions(),
            escape_cypher(&fields_json),
            escape_cypher(&e.llm_context),
            e.template_version,
            e.source_file,
            e.last_updated
        )
    }

    /// Generate Expression atom nodes section.
    fn generate_expression_atoms_section(&self, expressions: &[ExpressionData]) -> String {
        let mut output = String::new();

        let total_atoms: usize = expressions.iter().map(|e| e.total_expressions()).sum();

        output.push_str(&format!(
            r#"// ----------------------------------------------------------------------------
// PART 2: Expression nodes (atoms) - {} total
// ----------------------------------------------------------------------------

"#,
            total_atoms
        ));

        for e in expressions {
            for field in &e.semantic_fields {
                output.push_str(&format!("// --- {} / {} ---\n", e.locale_key, field.name));
                for (idx, expr) in field.expressions.iter().enumerate() {
                    output.push_str(&self.generate_expression_atom_cypher(e, field, expr, idx));
                    output.push('\n');
                }
            }
        }

        output
    }

    /// Generate Cypher for a single Expression atom node.
    fn generate_expression_atom_cypher(
        &self,
        data: &ExpressionData,
        field: &SemanticField,
        expr: &Expression,
        idx: usize,
    ) -> String {
        // Generate unique key: locale/field/index (e.g., "fr-FR/SUCCESS/0")
        let atom_key = format!("{}/{}/{}", data.locale_key, field.name, idx);

        // v10.7: Generate llm_context for retrieval optimization
        let llm_context = format!(
            "USE: {} expression for {}. TRIGGERS: {}, {}, {}. NOT: other {} registers.",
            data.locale_key,
            expr.context,
            field.name.to_lowercase(),
            expr.register,
            expr.intention
                .split_whitespace()
                .take(3)
                .collect::<Vec<_>>()
                .join(" "),
            data.locale_key
        );

        format!(
            r#"MERGE (e:Expression {{key: '{}'}})
SET e.locale_key = '{}',
    e.semantic_field = '{}',
    e.intention = '{}',
    e.text = '{}',
    e.register = '{}',
    e.context = '{}',
    e.example = '{}',
    e.llm_context = '{}';
"#,
            atom_key,
            data.locale_key,
            field.name,
            escape_cypher(&expr.intention),
            escape_cypher(&expr.text),
            expr.register,
            escape_cypher(&expr.context),
            escape_cypher(&expr.example),
            escape_cypher(&llm_context)
        )
    }

    /// Generate Locale → ExpressionSet arcs section.
    fn generate_locale_arcs_section(&self, expressions: &[ExpressionData]) -> String {
        let mut output = String::new();

        output.push_str(
            r#"// ----------------------------------------------------------------------------
// PART 3: Arcs Locale → ExpressionSet
// ----------------------------------------------------------------------------

"#,
        );

        for e in expressions {
            output.push_str(&format!(
                r#"MATCH (l:Locale {{key: '{}'}})
MATCH (es:ExpressionSet {{key: '{}'}})
MERGE (l)-[:HAS_EXPRESSIONS]->(es);

"#,
                e.locale_key, e.locale_key
            ));
        }

        output
    }

    /// Generate ExpressionSet → Expression arcs section.
    fn generate_contains_arcs_section(&self, expressions: &[ExpressionData]) -> String {
        let mut output = String::new();

        let total_arcs: usize = expressions.iter().map(|e| e.total_expressions()).sum();

        output.push_str(&format!(
            r#"// ----------------------------------------------------------------------------
// PART 4: Arcs ExpressionSet → Expression (CONTAINS_EXPRESSION) - {} arcs
// ----------------------------------------------------------------------------

"#,
            total_arcs
        ));

        for e in expressions {
            for field in &e.semantic_fields {
                for idx in 0..field.expressions.len() {
                    let atom_key = format!("{}/{}/{}", e.locale_key, field.name, idx);
                    output.push_str(&format!(
                        r#"MATCH (es:ExpressionSet {{key: '{}'}})
MATCH (e:Expression {{key: '{}'}})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

"#,
                        e.locale_key, atom_key
                    ));
                }
            }
        }

        output
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::expression::Expression as ExprData;

    fn create_test_expression_data() -> ExpressionData {
        let mut data = ExpressionData {
            locale_key: "fr-FR".to_string(),
            template_version: "3.1".to_string(),
            last_updated: "2026-01-09".to_string(),
            source_file: "3-voice-lexicon/fr-FR.md".to_string(),
            semantic_fields: vec![
                SemanticField {
                    name: "SUCCESS".to_string(),
                    local_name: Some("Succes".to_string()),
                    expressions: vec![
                        ExprData::new(
                            "Great success",
                            "un succes retentissant",
                            "formal",
                            "Major milestones",
                            "Votre lancement a ete un succes retentissant.",
                        ),
                        ExprData::new(
                            "Winning moment",
                            "carton plein",
                            "semi-formal",
                            "Sales",
                            "Avec cette offre, c'est carton plein !",
                        ),
                    ],
                    cultural_note: Some("French culture values modesty.".to_string()),
                },
                SemanticField {
                    name: "SPEED".to_string(),
                    local_name: None,
                    expressions: vec![ExprData::new(
                        "Very fast",
                        "en un clin d'oeil",
                        "semi-formal",
                        "Quick processes",
                        "Votre compte est cree en un clin d'oeil.",
                    )],
                    cultural_note: None,
                },
            ],
            llm_context: String::new(),
            raw_markdown: "# Voice Lexicon".to_string(),
        };
        data.generate_llm_context();
        data
    }

    #[test]
    fn test_generate_expression_set_cypher() {
        let data = create_test_expression_data();
        let generator = ExpressionGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_expression_set_cypher(&data);

        assert!(cypher.contains("MERGE (es:ExpressionSet {key: 'fr-FR'})"));
        assert!(cypher.contains("es.semantic_fields_count = 2"));
        assert!(cypher.contains("es.total_expressions = 3"));
    }

    #[test]
    fn test_generate_expression_atom_cypher() {
        let data = create_test_expression_data();
        let generator = ExpressionGenerator::new(Some("/tmp/test")).unwrap();
        let field = &data.semantic_fields[0];
        let expr = &field.expressions[0];
        let cypher = generator.generate_expression_atom_cypher(&data, field, expr, 0);

        assert!(cypher.contains("MERGE (e:Expression {key: 'fr-FR/SUCCESS/0'})"));
        assert!(cypher.contains("e.locale_key = 'fr-FR'"));
        assert!(cypher.contains("e.semantic_field = 'SUCCESS'"));
        assert!(cypher.contains("e.text = 'un succes retentissant'"));
        assert!(cypher.contains("e.register = 'formal'"));
        // v10.7: Check llm_context is generated
        assert!(cypher.contains("e.llm_context = '"));
        assert!(cypher.contains("USE: fr-FR expression"));
    }

    #[test]
    fn test_generate_locale_arcs() {
        let expressions = vec![create_test_expression_data()];
        let generator = ExpressionGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_locale_arcs_section(&expressions);

        assert!(cypher.contains("MATCH (l:Locale {key: 'fr-FR'})"));
        assert!(cypher.contains("MATCH (es:ExpressionSet {key: 'fr-FR'})"));
        assert!(cypher.contains("MERGE (l)-[:HAS_EXPRESSIONS]->(es)"));
    }

    #[test]
    fn test_generate_contains_arcs() {
        let expressions = vec![create_test_expression_data()];
        let generator = ExpressionGenerator::new(Some("/tmp/test")).unwrap();
        let cypher = generator.generate_contains_arcs_section(&expressions);

        // Should have 3 CONTAINS_EXPRESSION arcs (2 SUCCESS + 1 SPEED)
        assert!(cypher.contains("fr-FR/SUCCESS/0"));
        assert!(cypher.contains("fr-FR/SUCCESS/1"));
        assert!(cypher.contains("fr-FR/SPEED/0"));
        assert!(cypher.contains("MERGE (es)-[:CONTAINS_EXPRESSION]->(e)"));
    }

    #[test]
    fn test_generate_header() {
        let expressions = vec![create_test_expression_data()];
        let generator = ExpressionGenerator::new(Some("/tmp/test")).unwrap();
        let header = generator.generate_header(&expressions, 3);

        assert!(header.contains("EXPRESSION ATOMS SEED"));
        assert!(header.contains("Locales: 1"));
        assert!(header.contains("Total Expression atoms: 3"));
    }

    #[test]
    fn test_total_expressions() {
        let data = create_test_expression_data();
        assert_eq!(data.total_expressions(), 3);
    }

    #[test]
    fn test_atom_key_format() {
        let data = create_test_expression_data();
        let generator = ExpressionGenerator::new(Some("/tmp/test")).unwrap();
        let field = &data.semantic_fields[1]; // SPEED
        let expr = &field.expressions[0];
        let cypher = generator.generate_expression_atom_cypher(&data, field, expr, 0);

        // Key should be locale/field/index
        assert!(cypher.contains("fr-FR/SPEED/0"));
    }
}
