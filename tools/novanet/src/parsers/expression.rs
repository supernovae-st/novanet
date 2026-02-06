//! Parser for ATH 3-voice-lexicon (Expression/Lexicon) data.
//!
//! Transforms ATH voice lexicon markdown files into Rust structs for Cypher generation.
//! Extracts semantic fields (SUCCESS, SPEED, QUALITY, etc.) with their expressions.

use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{NovaNetError, Result};

// ============================================================================
// Lazy-compiled Regex Patterns
// ============================================================================

/// Template version extraction
static RE_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"template_version:\s*(.+)").expect("valid version regex"));

/// Last updated date extraction
static RE_DATE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"last_updated:\s*(.+)").expect("valid date regex"));

/// Semantic field header: ### N.N FIELD_NAME / LOCAL_NAME or ### N.N FIELD_NAME (local_name)
static RE_SEMANTIC_FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"###\s+\d+\.\d+\s+([A-Z_]+(?:\s*/\s*[A-Z_]+)?)\s*(?:\(([^)]+)\)|$)")
        .expect("valid semantic field regex")
});

/// Table row pattern: | col1 | col2 | col3 | col4 | col5 |
static RE_TABLE_ROW: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|")
        .expect("valid table row regex")
});

/// Cultural note pattern
static RE_CULTURAL_NOTE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*Cultural Note\*\*:\s*(.+)").expect("valid cultural note regex")
});

// ============================================================================
// Main Structs
// ============================================================================

/// Complete expression/lexicon data for a locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionData {
    /// Locale key (e.g., "fr-FR")
    pub locale_key: String,

    /// Template version
    pub template_version: String,

    /// Last updated date
    pub last_updated: String,

    /// Source file path
    pub source_file: String,

    /// Semantic fields with their expressions
    pub semantic_fields: Vec<SemanticField>,

    /// LLM context summary
    pub llm_context: String,

    /// Full source markdown
    pub raw_markdown: String,
}

impl Default for ExpressionData {
    fn default() -> Self {
        Self {
            locale_key: String::new(),
            template_version: "3.1".to_string(),
            last_updated: String::new(),
            source_file: String::new(),
            semantic_fields: Vec::new(),
            llm_context: String::new(),
            raw_markdown: String::new(),
        }
    }
}

impl ExpressionData {
    /// Generate LLM context summary from expression data.
    pub fn generate_llm_context(&mut self) {
        let mut parts = Vec::new();

        parts.push(format!(
            "{}: {} semantic fields with {} total expressions.",
            self.locale_key,
            self.semantic_fields.len(),
            self.semantic_fields
                .iter()
                .map(|f| f.expressions.len())
                .sum::<usize>()
        ));

        // Add summary of top fields
        for field in self.semantic_fields.iter().take(3) {
            parts.push(format!(
                "{}: {} expressions",
                field.name,
                field.expressions.len()
            ));
        }

        self.llm_context = parts.join(" ");
    }

    /// Get total expression count across all fields.
    pub fn total_expressions(&self) -> usize {
        self.semantic_fields
            .iter()
            .map(|f| f.expressions.len())
            .sum()
    }
}

/// A semantic field containing expressions for a specific intention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticField {
    /// Field name in English (e.g., "SUCCESS", "SPEED")
    pub name: String,

    /// Local name in the target language (e.g., "Succes" for fr-FR)
    pub local_name: Option<String>,

    /// Expressions within this field
    pub expressions: Vec<Expression>,

    /// Cultural note for this field
    pub cultural_note: Option<String>,
}

impl SemanticField {
    /// Create a new semantic field with the given name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            local_name: None,
            expressions: Vec::new(),
            cultural_note: None,
        }
    }
}

/// A single expression within a semantic field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expression {
    /// The intention/purpose of the expression
    pub intention: String,

    /// The native expression text
    pub text: String,

    /// Register: formal, semi-formal, casual
    pub register: String,

    /// Context for when to use
    pub context: String,

    /// Example sentence showing usage
    pub example: String,
}

impl Expression {
    /// Create a new expression.
    pub fn new(intention: &str, text: &str, register: &str, context: &str, example: &str) -> Self {
        Self {
            intention: intention.trim().to_string(),
            text: text.trim().to_string(),
            register: register.trim().to_lowercase(),
            context: context.trim().to_string(),
            example: example.trim().to_string(),
        }
    }

    /// Check if this is a valid expression (not a header or separator).
    pub fn is_valid(&self) -> bool {
        !self.text.is_empty()
            && !self.text.contains("---")
            && !self.intention.to_lowercase().contains("intention")
            && !self.text.to_lowercase().contains("expression")
    }
}

// ============================================================================
// Loading Functions
// ============================================================================

/// Load all expression files from ATH data directory.
pub fn load_all_expressions(ath_path: &Path) -> Result<Vec<ExpressionData>> {
    let lexicon_dir = ath_path.join("3-voice-lexicon");

    if !lexicon_dir.exists() {
        return Err(NovaNetError::Validation(format!(
            "Voice lexicon directory not found: {}",
            lexicon_dir.display()
        )));
    }

    // Collect all .md files
    let files: Vec<_> = WalkDir::new(&lexicon_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .collect();

    // Parse in parallel
    let expressions: Vec<ExpressionData> = files
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            match parse_expression_file(path) {
                Ok(e) => Some(e),
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    None
                }
            }
        })
        .collect();

    Ok(expressions)
}

/// Parse a single expression markdown file.
pub fn parse_expression_file(path: &Path) -> Result<ExpressionData> {
    let content = fs::read_to_string(path)?;
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Extract locale from filename (e.g., "fr-FR.md" -> "fr-FR")
    let locale = filename.trim_end_matches(".md");

    parse_expression_markdown(&content, locale, filename)
}

/// Parse expression markdown content.
pub fn parse_expression_markdown(
    content: &str,
    locale_key: &str,
    source_file: &str,
) -> Result<ExpressionData> {
    // Parse frontmatter
    let (template_version, last_updated) = parse_frontmatter(content);

    // Parse semantic fields
    let semantic_fields = parse_semantic_fields(content);

    let mut data = ExpressionData {
        locale_key: locale_key.to_string(),
        template_version,
        last_updated,
        source_file: format!("3-voice-lexicon/{}", source_file),
        semantic_fields,
        llm_context: String::new(),
        raw_markdown: content.to_string(),
    };

    // Generate LLM context
    data.generate_llm_context();

    Ok(data)
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parse YAML frontmatter.
fn parse_frontmatter(content: &str) -> (String, String) {
    let version = RE_VERSION
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "3.1".to_string());

    let date = RE_DATE
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    (version, date)
}

/// Parse all semantic fields from content.
fn parse_semantic_fields(content: &str) -> Vec<SemanticField> {
    let mut fields = Vec::new();
    let mut current_field: Option<SemanticField> = None;
    let mut in_table = false;

    for line in content.lines() {
        // Check for semantic field header
        if let Some(caps) = RE_SEMANTIC_FIELD.captures(line) {
            // Save previous field
            if let Some(field) = current_field.take() {
                if !field.expressions.is_empty() {
                    fields.push(field);
                }
            }

            // Start new field
            let name = caps
                .get(1)
                .map(|m| normalize_field_name(m.as_str()))
                .unwrap_or_default();
            let local_name = caps.get(2).map(|m| m.as_str().trim().to_string());

            let mut new_field = SemanticField::new(&name);
            new_field.local_name = local_name;
            current_field = Some(new_field);
            in_table = false;
            continue;
        }

        // Check for cultural note
        if let Some(caps) = RE_CULTURAL_NOTE.captures(line) {
            if let Some(ref mut field) = current_field {
                field.cultural_note = Some(
                    caps.get(1)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default(),
                );
            }
            continue;
        }

        // Check for table header (starts the table)
        if line.contains("| Intention |") || line.contains("| Expression |") {
            in_table = true;
            continue;
        }

        // Check for table separator
        if line.contains("|---") || line.contains("| ---") {
            continue;
        }

        // Check for end of section
        if line.starts_with("---") || (line.starts_with("##") && !line.starts_with("###")) {
            in_table = false;
            continue;
        }

        // Parse table row
        if in_table {
            if let Some(caps) = RE_TABLE_ROW.captures(line) {
                let intention = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
                let text = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");
                let register = caps.get(3).map(|m| m.as_str().trim()).unwrap_or("");
                let context = caps.get(4).map(|m| m.as_str().trim()).unwrap_or("");
                let example = caps.get(5).map(|m| m.as_str().trim()).unwrap_or("");

                let expr = Expression::new(intention, text, register, context, example);

                if expr.is_valid() {
                    if let Some(ref mut field) = current_field {
                        field.expressions.push(expr);
                    }
                }
            }
        }
    }

    // Save last field
    if let Some(field) = current_field {
        if !field.expressions.is_empty() {
            fields.push(field);
        }
    }

    fields
}

/// Normalize field name to standard format.
fn normalize_field_name(name: &str) -> String {
    // Handle "SUCCESS / ACHIEVEMENT" -> "SUCCESS"
    // Handle "SUCCESS" -> "SUCCESS"
    name.split('/').next().unwrap_or(name).trim().to_uppercase()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const FR_FR_SAMPLE: &str = r#"---
locale: fr-FR
type: voice-lexicon
template_version: 3.1
last_updated: 2026-01-09
---

# Voice Lexicon: fr-FR

> **Purpose**: Define WHICH native expressions to use, organized by semantic INTENTION.

---

## 1. Semantic Fields

### 1.1 SUCCESS / ACHIEVEMENT

**Intention**: Expressing accomplishment, winning, reaching goals, victory

**Use for**: CTAs, success messaging, gamification, testimonials

| Intention | Expression | Register | Context | Example Sentence |
|-----------|------------|----------|---------|------------------|
| Great success | un succes retentissant | formal | Major milestones, PR | Votre lancement a ete un succes retentissant. |
| Great success | carton plein | semi-formal | Sales, campaigns | Avec cette offre, c'est carton plein ! |
| Winning moment | decrocher le jackpot | semi-formal | Promotions, rewards | Vous venez de decrocher le jackpot avec cette remise exclusive. |

**Cultural Note**: French culture values collective success with a touch of modesty.

---

### 1.2 SPEED / EFFICIENCY

**Intention**: Expressing quickness, immediate action, no delays

| Intention | Expression | Register | Context | Example Sentence |
|-----------|------------|----------|---------|------------------|
| Very fast | en un clin d'oeil | semi-formal | Quick processes | Votre compte est cree en un clin d'oeil. |
| Instant | sur-le-champ | formal | Immediate actions | Recevez votre confirmation sur-le-champ. |

**Cultural Note**: French audiences appreciate efficiency but are skeptical of pushy speed claims.

---

### 1.3 SIMPLICITY / EASE

| Intention | Expression | Register | Context | Example Sentence |
|-----------|------------|----------|---------|------------------|
| Very easy | d'une simplicite enfantine | formal | Product benefits | Une interface d'une simplicite enfantine. |
| Child's play | un jeu d'enfant | semi-formal | Easy processes | Configurer votre compte est un jeu d'enfant. |

---
"#;

    #[test]
    fn test_parse_expression_markdown_basic() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert_eq!(result.locale_key, "fr-FR");
        assert_eq!(result.template_version, "3.1");
        assert_eq!(result.last_updated, "2026-01-09");
        assert_eq!(result.source_file, "3-voice-lexicon/fr-FR.md");
    }

    #[test]
    fn test_parse_semantic_fields_count() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        // Should have 3 semantic fields: SUCCESS, SPEED, SIMPLICITY
        assert_eq!(result.semantic_fields.len(), 3);
    }

    #[test]
    fn test_parse_field_names() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        let field_names: Vec<&str> = result
            .semantic_fields
            .iter()
            .map(|f| f.name.as_str())
            .collect();
        assert_eq!(field_names, vec!["SUCCESS", "SPEED", "SIMPLICITY"]);
    }

    #[test]
    fn test_parse_expressions_per_field() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        // SUCCESS has 3 expressions
        assert_eq!(result.semantic_fields[0].expressions.len(), 3);
        // SPEED has 2 expressions
        assert_eq!(result.semantic_fields[1].expressions.len(), 2);
        // SIMPLICITY has 2 expressions
        assert_eq!(result.semantic_fields[2].expressions.len(), 2);
    }

    #[test]
    fn test_parse_expression_details() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        let first_expr = &result.semantic_fields[0].expressions[0];
        assert_eq!(first_expr.intention, "Great success");
        assert_eq!(first_expr.text, "un succes retentissant");
        assert_eq!(first_expr.register, "formal");
        assert_eq!(first_expr.context, "Major milestones, PR");
        assert!(first_expr.example.contains("Votre lancement"));
    }

    #[test]
    fn test_parse_cultural_note() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        let success_field = &result.semantic_fields[0];
        assert!(success_field.cultural_note.is_some());
        assert!(
            success_field
                .cultural_note
                .as_ref()
                .unwrap()
                .contains("collective success")
        );
    }

    #[test]
    fn test_total_expressions() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        // 3 + 2 + 2 = 7 total expressions
        assert_eq!(result.total_expressions(), 7);
    }

    #[test]
    fn test_llm_context_generated() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert!(!result.llm_context.is_empty());
        assert!(result.llm_context.contains("fr-FR"));
        assert!(result.llm_context.contains("3 semantic fields"));
        assert!(result.llm_context.contains("7 total expressions"));
    }

    #[test]
    fn test_expression_is_valid() {
        let valid = Expression::new(
            "Great success",
            "carton plein",
            "formal",
            "Sales",
            "Example",
        );
        assert!(valid.is_valid());

        let header = Expression::new("Intention", "Expression", "Register", "Context", "Example");
        assert!(!header.is_valid());

        let separator = Expression::new("---", "---", "---", "---", "---");
        assert!(!separator.is_valid());
    }

    #[test]
    fn test_normalize_field_name() {
        assert_eq!(normalize_field_name("SUCCESS / ACHIEVEMENT"), "SUCCESS");
        assert_eq!(normalize_field_name("SPEED / EFFICIENCY"), "SPEED");
        assert_eq!(normalize_field_name("SIMPLICITY"), "SIMPLICITY");
        assert_eq!(normalize_field_name("success"), "SUCCESS");
    }

    #[test]
    fn test_register_normalized_to_lowercase() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        for field in &result.semantic_fields {
            for expr in &field.expressions {
                assert!(
                    expr.register == "formal"
                        || expr.register == "semi-formal"
                        || expr.register == "casual",
                    "Unexpected register: {}",
                    expr.register
                );
            }
        }
    }

    // Test with en-US style content
    const EN_US_SAMPLE: &str = r#"---
locale: en-US
type: voice-lexicon
template_version: 3.1
last_updated: 2026-01-09
---

# Voice Lexicon: en-US

---

## 1. Semantic Fields

### 1.1 SUCCESS / ACHIEVEMENT

| Intention | Expression | Register | Context | Example Sentence |
|-----------|------------|----------|---------|------------------|
| Great success | Knocked it out of the park | casual | Outstanding achievement | Your team knocked it out of the park this quarter. |
| Winning moment | Nailed it | casual | Perfect execution | You nailed it - the presentation was flawless. |

**Cultural Note**: American culture celebrates success openly and enthusiastically.

---
"#;

    #[test]
    fn test_parse_en_us() {
        let result = parse_expression_markdown(EN_US_SAMPLE, "en-US", "en-US.md").unwrap();

        assert_eq!(result.locale_key, "en-US");
        assert_eq!(result.semantic_fields.len(), 1);
        assert_eq!(result.semantic_fields[0].expressions.len(), 2);
        assert_eq!(result.semantic_fields[0].expressions[0].register, "casual");
    }

    #[test]
    fn test_raw_markdown_preserved() {
        let result = parse_expression_markdown(FR_FR_SAMPLE, "fr-FR", "fr-FR.md").unwrap();

        assert!(!result.raw_markdown.is_empty());
        assert!(result.raw_markdown.contains("Voice Lexicon: fr-FR"));
    }
}
