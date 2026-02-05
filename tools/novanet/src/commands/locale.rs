//! Locale commands: `novanet locale list`, `novanet locale import`, `novanet locale generate`.
//!
//! Lists locales with their knowledge satellite status from Neo4j,
//! imports locale data from Cypher files, and generates Cypher from CSV + MD sources.
//!
//! v10.4: Uses tiered knowledge model:
//! - Technical tier: Formatting, Slugification, Adaptation
//! - Style tier: Style
//! - Semantic tier: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
//!
//! v10.6: Adds `locale generate` command to produce 20-locales.cypher from:
//! - CSV file with 200 locales (basic info: codes, names, is_primary)
//! - 1-identity/*.md files (enrichment: native names, fallback chain)

use std::collections::HashMap;
use std::path::Path;

use crate::db::Db;
use crate::output::OutputFormat;

/// A locale row for display (v10.4 tiered knowledge model).
#[derive(Debug, Clone, serde::Serialize, tabled::Tabled)]
pub struct LocaleRow {
    pub key: String,
    pub display_name: String,
    pub language_code: String,
    pub country_code: String,
    /// Technical tier (Formatting, Slugification, Adaptation)
    pub technical: String,
    /// Style tier (Style)
    pub style: String,
    /// Semantic tier (TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet)
    pub semantic: String,
}

/// List all locales with their knowledge satellite status (v10.4 tiered model).
pub async fn run_list(db: &Db, format: OutputFormat) -> crate::Result<()> {
    let cypher = "\
MATCH (l:Locale)
// Technical tier
OPTIONAL MATCH (l)-[:HAS_FORMATTING]->(f:Formatting)
OPTIONAL MATCH (l)-[:HAS_SLUGIFICATION]->(sl:Slugification)
OPTIONAL MATCH (l)-[:HAS_ADAPTATION]->(a:Adaptation)
// Style tier
OPTIONAL MATCH (l)-[:HAS_STYLE]->(st:Style)
// Semantic tier (6 sets)
OPTIONAL MATCH (l)-[:HAS_TERMS]->(t:TermSet)
OPTIONAL MATCH (l)-[:HAS_EXPRESSIONS]->(ex:ExpressionSet)
OPTIONAL MATCH (l)-[:HAS_PATTERNS]->(pa:PatternSet)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(c:CultureSet)
OPTIONAL MATCH (l)-[:HAS_TABOOS]->(tb:TabooSet)
OPTIONAL MATCH (l)-[:HAS_AUDIENCE]->(au:AudienceSet)
RETURN l.key AS key,
       l.display_name AS display_name,
       l.language_code AS language_code,
       l.country_code AS country_code,
       CASE WHEN f IS NOT NULL OR sl IS NOT NULL OR a IS NOT NULL THEN 'yes' ELSE '-' END AS technical,
       CASE WHEN st IS NOT NULL THEN 'yes' ELSE '-' END AS style,
       CASE WHEN t IS NOT NULL OR ex IS NOT NULL OR pa IS NOT NULL OR c IS NOT NULL OR tb IS NOT NULL OR au IS NOT NULL THEN 'yes' ELSE '-' END AS semantic
ORDER BY l.key";

    match format {
        OutputFormat::Cypher => {
            crate::output::print_output(cypher);
        }
        OutputFormat::Table => {
            let rows = db.execute(cypher).await?;
            let locale_rows = extract_locale_rows(&rows);
            let table = tabled::Table::new(&locale_rows)
                .with(tabled::settings::Style::rounded())
                .to_string();
            crate::output::print_output(&table);
            eprintln!("{} locale(s)", locale_rows.len());
        }
        OutputFormat::Json => {
            let rows = db.execute(cypher).await?;
            let locale_rows = extract_locale_rows(&rows);
            crate::output::print_output(&crate::output::format_json(&locale_rows));
        }
    }

    Ok(())
}

/// Import locale data from a Cypher file.
/// Reuses the statement splitter from the db module.
pub async fn run_import(db: &Db, file: &Path) -> crate::Result<()> {
    let content = std::fs::read_to_string(file).map_err(crate::NovaNetError::Io)?;
    let statements = crate::commands::db::split_cypher_statements(&content);

    if statements.is_empty() {
        return Err(crate::NovaNetError::Validation(
            "locale import: file contains no Cypher statements".to_string(),
        ));
    }

    eprintln!(
        "Importing {} statement(s) from {}",
        statements.len(),
        file.display()
    );

    let start = std::time::Instant::now();
    let mut executed = 0u64;

    for stmt in &statements {
        db.execute(stmt).await.map_err(|e| {
            crate::NovaNetError::Validation(format!(
                "Failed at statement #{} in {}:\n  {:.120}\n  Error: {e}",
                executed + 1,
                file.display(),
                stmt,
            ))
        })?;
        executed += 1;
    }

    let elapsed = start.elapsed();
    eprintln!(
        "Import complete: {executed} statement(s) in {:.1}s",
        elapsed.as_secs_f64()
    );

    Ok(())
}

fn extract_locale_rows(rows: &[neo4rs::Row]) -> Vec<LocaleRow> {
    rows.iter()
        .map(|row| LocaleRow {
            key: row.get("key").unwrap_or_default(),
            display_name: row.get("display_name").unwrap_or_default(),
            language_code: row.get("language_code").unwrap_or_default(),
            country_code: row.get("country_code").unwrap_or_default(),
            technical: row.get("technical").unwrap_or_default(),
            style: row.get("style").unwrap_or_default(),
            semantic: row.get("semantic").unwrap_or_default(),
        })
        .collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOCALE GENERATE (v10.6)
// ═══════════════════════════════════════════════════════════════════════════════

/// CSV record from LOCALES-200.csv
#[derive(Debug, Clone, serde::Deserialize)]
struct CsvLocale {
    #[serde(rename = "Locale Code")]
    locale_code: String,
    #[serde(rename = "Language")]
    language: String,
    #[serde(rename = "Language Code")]
    language_code: String,
    #[serde(rename = "Country Code")]
    country_code: String,
    #[serde(rename = "Is Primary")]
    is_primary: String, // "true" or "false" as string
    #[serde(rename = "Script")]
    _script: String,
    #[serde(rename = "slug_rule")]
    _slug_rule: String,
    #[serde(rename = "Is RTL")]
    _is_rtl: String, // "true" or "false" as string
    #[serde(rename = "Timezone")]
    _timezone: String,
}

/// Enrichment data from 1-identity/*.md files
#[derive(Debug, Clone, Default)]
struct IdentityEnrichment {
    language_native: Option<String>,
    country_native: Option<String>,
}

/// Parsed locale with all data needed for Cypher generation
#[derive(Debug, Clone)]
struct ParsedLocale {
    key: String,
    display_name: String,
    language_code: String,
    country_code: String,
    is_primary: bool,
    // Enrichment from MD (stored but currently only used during construction)
    _language_native: String,
    _country_native: String,
    // Computed
    name_native: String,
    description: String,
    llm_context: String,
}

/// Generate 20-locales.cypher from CSV + MD sources.
///
/// Fallback chain logic:
/// - Non-primary locales fallback to primary of same language_code
/// - Primary locales fallback to en-US
/// - en-US has no fallback (root)
pub fn run_generate(
    csv_path: &Path,
    identity_dir: &Path,
    output_path: &Path,
    dry_run: bool,
) -> crate::Result<()> {
    // 1. Parse CSV
    eprintln!("Reading CSV: {}", csv_path.display());
    let csv_locales = parse_csv(csv_path)?;
    eprintln!("  Found {} locales", csv_locales.len());

    // 2. Parse MD files for enrichment
    eprintln!("Reading identity files: {}", identity_dir.display());
    let enrichments = parse_identity_files(identity_dir)?;
    eprintln!("  Found {} identity files", enrichments.len());

    // 3. Build parsed locales with enrichment
    let locales: Vec<ParsedLocale> = csv_locales
        .iter()
        .map(|csv| {
            let enrichment = enrichments
                .get(&csv.locale_code)
                .cloned()
                .unwrap_or_default();
            build_parsed_locale(csv, &enrichment)
        })
        .collect();

    // 4. Build fallback map: language_code -> primary locale key
    let primary_map: HashMap<String, String> = locales
        .iter()
        .filter(|l| l.is_primary)
        .map(|l| (l.language_code.clone(), l.key.clone()))
        .collect();

    // 5. Generate Cypher
    let cypher = generate_cypher(&locales, &primary_map);

    // 6. Write or print
    if dry_run {
        eprintln!("\n--- DRY RUN: Generated Cypher ---\n");
        println!("{cypher}");
        eprintln!("\n--- END DRY RUN ---");
    } else {
        std::fs::write(output_path, &cypher).map_err(crate::NovaNetError::Io)?;
        eprintln!("Wrote {} bytes to {}", cypher.len(), output_path.display());
    }

    eprintln!("Generated {} Locale nodes + fallback arcs", locales.len());

    Ok(())
}

/// Parse CSV file into locale records.
fn parse_csv(path: &Path) -> crate::Result<Vec<CsvLocale>> {
    let file = std::fs::File::open(path).map_err(crate::NovaNetError::Io)?;
    let mut reader = csv::Reader::from_reader(file);

    let mut locales = Vec::new();
    for result in reader.deserialize() {
        let record: CsvLocale =
            result.map_err(|e| crate::NovaNetError::Validation(format!("CSV parse error: {e}")))?;
        locales.push(record);
    }

    Ok(locales)
}

/// Parse 1-identity/*.md files for enrichment data.
fn parse_identity_files(dir: &Path) -> crate::Result<HashMap<String, IdentityEnrichment>> {
    let mut enrichments = HashMap::new();

    if !dir.exists() {
        return Ok(enrichments);
    }

    for entry in std::fs::read_dir(dir).map_err(crate::NovaNetError::Io)? {
        let entry = entry.map_err(crate::NovaNetError::Io)?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                let content = std::fs::read_to_string(&path).map_err(crate::NovaNetError::Io)?;
                let enrichment = parse_identity_md(&content);
                enrichments.insert(stem.to_string(), enrichment);
            }
        }
    }

    Ok(enrichments)
}

/// Parse a single identity MD file to extract language_native and country_native.
fn parse_identity_md(content: &str) -> IdentityEnrichment {
    let mut enrichment = IdentityEnrichment::default();

    // Look for table rows like:
    // | language_native | français |
    // | country_native | France |
    for line in content.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }

        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() >= 3 {
            let field = parts[1].to_lowercase();
            let value = parts[2].to_string();

            if field == "language_native" {
                enrichment.language_native = Some(value);
            } else if field == "country_native" {
                enrichment.country_native = Some(value);
            }
        }
    }

    enrichment
}

/// Build a ParsedLocale from CSV + enrichment data.
fn build_parsed_locale(csv: &CsvLocale, enrichment: &IdentityEnrichment) -> ParsedLocale {
    let is_primary = csv.is_primary.to_lowercase() == "true";

    // Use enrichment or fallback to English names
    let language_native = enrichment
        .language_native
        .clone()
        .unwrap_or_else(|| extract_language_from_display(&csv.language));
    let country_native = enrichment
        .country_native
        .clone()
        .unwrap_or_else(|| extract_country_from_display(&csv.language));

    // Construct name_native: "français (France)"
    let name_native = format!("{} ({})", language_native, country_native);

    // Construct description
    let description = format!(
        "{} locale for {} market",
        extract_language_from_display(&csv.language),
        country_native
    );

    // Construct llm_context
    let llm_context = format!(
        "USE: for {} content targeting {}. TRIGGERS: {}, {}, {}.",
        extract_language_from_display(&csv.language),
        country_native,
        csv.locale_code,
        language_native,
        country_native.to_lowercase()
    );

    ParsedLocale {
        key: csv.locale_code.clone(),
        display_name: csv.language.clone(),
        language_code: csv.language_code.clone(),
        country_code: csv.country_code.clone(),
        is_primary,
        _language_native: language_native,
        _country_native: country_native,
        name_native,
        description,
        llm_context,
    }
}

/// Extract language from display name like "French (France)" -> "French"
fn extract_language_from_display(display: &str) -> String {
    if let Some(idx) = display.find('(') {
        display[..idx].trim().to_string()
    } else {
        display.to_string()
    }
}

/// Extract country from display name like "French (France)" -> "France"
fn extract_country_from_display(display: &str) -> String {
    if let Some(start) = display.find('(') {
        if let Some(end) = display.find(')') {
            return display[start + 1..end].trim().to_string();
        }
    }
    display.to_string()
}

/// Generate Cypher for all locales and their fallback arcs.
fn generate_cypher(locales: &[ParsedLocale], primary_map: &HashMap<String, String>) -> String {
    let mut cypher = String::new();

    // Header
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n",
    );
    cypher.push_str("// 20-locales.cypher - Generated by `novanet locale generate`\n");
    cypher.push_str("// v10.6 - 200 Locale nodes with fallback chains\n");
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n\n",
    );

    // Part 1: Create all Locale nodes
    cypher.push_str(
        "// ─── Locale Nodes ────────────────────────────────────────────────────────────\n\n",
    );

    for locale in locales {
        cypher.push_str(&format!(
            "MERGE (l:Locale {{key: \"{}\"}})\n",
            escape_cypher(&locale.key)
        ));
        cypher.push_str("ON CREATE SET\n");
        cypher.push_str(&format!(
            "  l.display_name = \"{}\",\n",
            escape_cypher(&locale.display_name)
        ));
        cypher.push_str(&format!(
            "  l.description = \"{}\",\n",
            escape_cypher(&locale.description)
        ));
        cypher.push_str(&format!(
            "  l.llm_context = \"{}\",\n",
            escape_cypher(&locale.llm_context)
        ));
        cypher.push_str(&format!(
            "  l.language_code = \"{}\",\n",
            escape_cypher(&locale.language_code)
        ));
        cypher.push_str(&format!(
            "  l.country_code = \"{}\",\n",
            escape_cypher(&locale.country_code)
        ));
        cypher.push_str(&format!(
            "  l.name_native = \"{}\",\n",
            escape_cypher(&locale.name_native)
        ));
        cypher.push_str(&format!("  l.is_primary = {},\n", locale.is_primary));
        cypher.push_str("  l.created_at = datetime(),\n");
        cypher.push_str("  l.updated_at = datetime()\n");
        cypher.push_str("ON MATCH SET\n");
        cypher.push_str("  l.updated_at = datetime();\n\n");
    }

    // Part 2: Create fallback arcs
    cypher.push_str(
        "// ─── Fallback Arcs ───────────────────────────────────────────────────────────\n",
    );
    cypher.push_str("// Logic: non-primary -> primary (same language) -> en-US\n\n");

    for locale in locales {
        // Skip en-US (root, no fallback)
        if locale.key == "en-US" {
            continue;
        }

        let fallback_to = if locale.is_primary {
            // Primary locales fallback to en-US
            "en-US".to_string()
        } else {
            // Non-primary locales fallback to their primary
            primary_map
                .get(&locale.language_code)
                .cloned()
                .unwrap_or_else(|| "en-US".to_string())
        };

        cypher.push_str(&format!(
            "MATCH (from:Locale {{key: \"{}\"}}), (to:Locale {{key: \"{}\"}})\n",
            escape_cypher(&locale.key),
            escape_cypher(&fallback_to)
        ));
        cypher.push_str("MERGE (from)-[:FALLBACK_TO]->(to);\n\n");
    }

    cypher
}

/// Escape a string for Cypher using double quotes.
/// - Escapes backslashes (\\)
/// - Escapes double quotes (\")
/// - Escapes newlines, tabs, carriage returns
/// - Single quotes don't need escaping inside double-quoted strings
fn escape_cypher(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

// ═══════════════════════════════════════════════════════════════════════════════
// LOCALE GENERATE-RULES (v10.6.1)
// ═══════════════════════════════════════════════════════════════════════════════

use crate::parsers::locale_rules::{
    AdaptationData, FormattingData, SlugificationData, parse_adaptation, parse_formatting,
    parse_slugification,
};

/// Generate locale rules Cypher files from markdown sources.
///
/// Reads from:
/// - 2-rules-formatting/*.md
/// - 2-rules-slug/*.md
/// - 2-rules-adaptation/*.md
///
/// Writes to:
/// - 21-formatting.cypher
/// - 22-slugification.cypher
/// - 23-adaptation.cypher
pub fn run_generate_rules(
    source_dir: &Path,
    output_dir: &Path,
    dry_run: bool,
) -> crate::Result<()> {
    let formatting_dir = source_dir.join("2-rules-formatting");
    let slugification_dir = source_dir.join("2-rules-slug");
    let adaptation_dir = source_dir.join("2-rules-adaptation");

    // 1. Parse formatting rules
    eprintln!("Reading formatting rules: {}", formatting_dir.display());
    let formatting_data = load_and_parse_dir(&formatting_dir, parse_formatting)?;
    eprintln!("  Found {} formatting files", formatting_data.len());

    // 2. Parse slugification rules
    eprintln!(
        "Reading slugification rules: {}",
        slugification_dir.display()
    );
    let slugification_data = load_and_parse_dir(&slugification_dir, parse_slugification)?;
    eprintln!("  Found {} slugification files", slugification_data.len());

    // 3. Parse adaptation rules
    eprintln!("Reading adaptation rules: {}", adaptation_dir.display());
    let adaptation_data = load_and_parse_dir(&adaptation_dir, parse_adaptation)?;
    eprintln!("  Found {} adaptation files", adaptation_data.len());

    // 4. Generate Cypher files
    let formatting_cypher = generate_formatting_cypher(&formatting_data);
    let slugification_cypher = generate_slugification_cypher(&slugification_data);
    let adaptation_cypher = generate_adaptation_cypher(&adaptation_data);

    // 5. Write or print
    if dry_run {
        eprintln!("\n--- DRY RUN: Generated Cypher files ---\n");
        eprintln!(
            "=== 21-formatting.cypher ({} bytes) ===",
            formatting_cypher.len()
        );
        eprintln!(
            "=== 22-slugification.cypher ({} bytes) ===",
            slugification_cypher.len()
        );
        eprintln!(
            "=== 23-adaptation.cypher ({} bytes) ===",
            adaptation_cypher.len()
        );
    } else {
        let formatting_path = output_dir.join("21-formatting.cypher");
        let slugification_path = output_dir.join("22-slugification.cypher");
        let adaptation_path = output_dir.join("23-adaptation.cypher");

        std::fs::write(&formatting_path, &formatting_cypher).map_err(crate::NovaNetError::Io)?;
        eprintln!(
            "Wrote {} bytes to {}",
            formatting_cypher.len(),
            formatting_path.display()
        );

        std::fs::write(&slugification_path, &slugification_cypher)
            .map_err(crate::NovaNetError::Io)?;
        eprintln!(
            "Wrote {} bytes to {}",
            slugification_cypher.len(),
            slugification_path.display()
        );

        std::fs::write(&adaptation_path, &adaptation_cypher).map_err(crate::NovaNetError::Io)?;
        eprintln!(
            "Wrote {} bytes to {}",
            adaptation_cypher.len(),
            adaptation_path.display()
        );
    }

    eprintln!(
        "\nGenerated locale rules: {} Formatting, {} Slugification, {} Adaptation",
        formatting_data.len(),
        slugification_data.len(),
        adaptation_data.len()
    );

    Ok(())
}

/// Load and parse all MD files from a directory.
fn load_and_parse_dir<T, F>(dir: &Path, parser: F) -> crate::Result<Vec<T>>
where
    F: Fn(&str) -> crate::Result<T>,
{
    let mut results = Vec::new();

    if !dir.exists() {
        eprintln!("  Warning: Directory not found: {}", dir.display());
        return Ok(results);
    }

    for entry in std::fs::read_dir(dir).map_err(crate::NovaNetError::Io)? {
        let entry = entry.map_err(crate::NovaNetError::Io)?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "md") {
            let content = std::fs::read_to_string(&path).map_err(crate::NovaNetError::Io)?;
            match parser(&content) {
                Ok(data) => results.push(data),
                Err(e) => {
                    eprintln!("  Warning: Failed to parse {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(results)
}

/// Generate Cypher for Formatting nodes.
fn generate_formatting_cypher(data: &[FormattingData]) -> String {
    let mut cypher = String::new();

    // Header
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n",
    );
    cypher.push_str("// 21-formatting.cypher - Generated by `novanet locale generate-rules`\n");
    cypher.push_str("// v10.6.1 - Formatting nodes for locale rules\n");
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n\n",
    );

    for f in data {
        cypher.push_str(&format!(
            "// ─── {} ────────────────────────────────────────────────────────────\n",
            f.locale
        ));
        cypher.push_str(&format!(
            "MATCH (l:Locale {{key: \"{}\"}})\n",
            escape_cypher(&f.locale)
        ));
        cypher.push_str(&format!(
            "MERGE (fmt:Formatting {{key: '{}-formatting'}})\n",
            escape_cypher(&f.locale)
        ));
        cypher.push_str("ON CREATE SET\n");
        cypher.push_str(&format!(
            "  fmt.display_name = 'Formatting: {}',\n",
            escape_cypher(&f.locale)
        ));
        cypher.push_str(&format!(
            "  fmt.description = 'Formatting rules for {}',\n",
            escape_cypher(&f.locale)
        ));
        cypher.push_str(&format!(
            "  fmt.decimal_separator = \"{}\",\n",
            escape_cypher(&f.decimal_separator)
        ));
        cypher.push_str(&format!(
            "  fmt.thousands_separator = \"{}\",\n",
            escape_cypher(&f.thousands_separator)
        ));
        cypher.push_str(&format!(
            "  fmt.date_format = \"{}\",\n",
            escape_cypher(&f.date_format)
        ));
        if let Some(ref time_fmt) = f.time_format {
            cypher.push_str(&format!(
                "  fmt.time_format = \"{}\",\n",
                escape_cypher(time_fmt)
            ));
        }
        cypher.push_str(&format!(
            "  fmt.currency_code = \"{}\",\n",
            escape_cypher(&f.currency_code)
        ));
        cypher.push_str(&format!(
            "  fmt.currency_symbol = \"{}\",\n",
            escape_cypher(&f.currency_symbol)
        ));
        cypher.push_str(&format!(
            "  fmt.currency_position = \"{}\",\n",
            escape_cypher(&f.currency_position)
        ));

        // Optional arrays
        if let Some(ref months) = f.month_names {
            cypher.push_str(&format!(
                "  fmt.month_names = {},\n",
                format_string_array(months)
            ));
        }
        if let Some(ref months_abbr) = f.month_names_abbr {
            cypher.push_str(&format!(
                "  fmt.month_names_abbr = {},\n",
                format_string_array(months_abbr)
            ));
        }
        if let Some(ref days) = f.day_names {
            cypher.push_str(&format!(
                "  fmt.day_names = {},\n",
                format_string_array(days)
            ));
        }
        if let Some(ref days_abbr) = f.day_names_abbr {
            cypher.push_str(&format!(
                "  fmt.day_names_abbr = {},\n",
                format_string_array(days_abbr)
            ));
        }
        if let Some(ref postal) = f.postal_code_format {
            cypher.push_str(&format!(
                "  fmt.postal_code_format = \"{}\",\n",
                escape_cypher(postal)
            ));
        }

        // Skip raw_markdown for now - too large and complex for batch import
        // TODO: Store raw_markdown in separate property or file reference
        cypher.push_str("  fmt.created_at = datetime(),\n");
        cypher.push_str("  fmt.updated_at = datetime()\n");
        cypher.push_str("ON MATCH SET\n");
        cypher.push_str("  fmt.updated_at = datetime()\n");
        cypher.push_str("MERGE (l)-[:HAS_FORMATTING]->(fmt);\n\n");
    }

    cypher
}

/// Generate Cypher for Slugification nodes.
fn generate_slugification_cypher(data: &[SlugificationData]) -> String {
    let mut cypher = String::new();

    // Header
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n",
    );
    cypher.push_str("// 22-slugification.cypher - Generated by `novanet locale generate-rules`\n");
    cypher.push_str("// v10.6.1 - Slugification nodes for locale rules\n");
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n\n",
    );

    for s in data {
        cypher.push_str(&format!(
            "// ─── {} ────────────────────────────────────────────────────────────\n",
            s.locale
        ));
        cypher.push_str(&format!(
            "MATCH (l:Locale {{key: \"{}\"}})\n",
            escape_cypher(&s.locale)
        ));
        cypher.push_str(&format!(
            "MERGE (slug:Slugification {{key: '{}-slugification'}})\n",
            escape_cypher(&s.locale)
        ));
        cypher.push_str("ON CREATE SET\n");
        cypher.push_str(&format!(
            "  slug.display_name = 'Slugification: {}',\n",
            escape_cypher(&s.locale)
        ));
        cypher.push_str(&format!(
            "  slug.description = 'Slugification rules for {}',\n",
            escape_cypher(&s.locale)
        ));
        cypher.push_str(&format!(
            "  slug.slug_rule = \"{}\",\n",
            escape_cypher(&s.slug_rule)
        ));
        cypher.push_str(&format!(
            "  slug.preserve_diacritics = {},\n",
            s.preserve_diacritics
        ));
        cypher.push_str(&format!(
            "  slug.unicode_normalization = \"{}\",\n",
            escape_cypher(&s.unicode_normalization)
        ));
        cypher.push_str(&format!("  slug.min_length = {},\n", s.min_length));
        cypher.push_str(&format!("  slug.max_length = {},\n", s.max_length));
        cypher.push_str(&format!(
            "  slug.separator = \"{}\",\n",
            escape_cypher(&s.separator)
        ));
        cypher.push_str(&format!("  slug.lowercase = {},\n", s.lowercase));
        cypher.push_str(&format!(
            "  slug.preserve_numbers = {},\n",
            s.preserve_numbers
        ));

        // Stop words array
        if !s.stop_words.is_empty() {
            cypher.push_str(&format!(
                "  slug.stop_words = {},\n",
                format_string_array(&s.stop_words)
            ));
        }

        // Optional character filter regex
        if let Some(ref regex) = s.character_filter_regex {
            cypher.push_str(&format!(
                "  slug.character_filter_regex = \"{}\",\n",
                escape_cypher(regex)
            ));
        }

        // Skip raw_markdown for now - too large and complex for batch import
        cypher.push_str("  slug.created_at = datetime(),\n");
        cypher.push_str("  slug.updated_at = datetime()\n");
        cypher.push_str("ON MATCH SET\n");
        cypher.push_str("  slug.updated_at = datetime()\n");
        cypher.push_str("MERGE (l)-[:HAS_SLUGIFICATION]->(slug);\n\n");
    }

    cypher
}

/// Generate Cypher for Adaptation nodes.
fn generate_adaptation_cypher(data: &[AdaptationData]) -> String {
    let mut cypher = String::new();

    // Header
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n",
    );
    cypher.push_str("// 23-adaptation.cypher - Generated by `novanet locale generate-rules`\n");
    cypher.push_str("// v10.6.1 - Adaptation nodes for locale rules\n");
    cypher.push_str(
        "// ═══════════════════════════════════════════════════════════════════════════════\n\n",
    );

    for a in data {
        cypher.push_str(&format!(
            "// ─── {} ────────────────────────────────────────────────────────────\n",
            a.locale
        ));
        cypher.push_str(&format!(
            "MATCH (l:Locale {{key: \"{}\"}})\n",
            escape_cypher(&a.locale)
        ));
        cypher.push_str(&format!(
            "MERGE (adapt:Adaptation {{key: '{}-adaptation'}})\n",
            escape_cypher(&a.locale)
        ));
        cypher.push_str("ON CREATE SET\n");
        cypher.push_str(&format!(
            "  adapt.display_name = 'Adaptation: {}',\n",
            escape_cypher(&a.locale)
        ));
        cypher.push_str(&format!(
            "  adapt.description = 'Adaptation rules (FACTS vs ILLUSTRATIONS) for {}',\n",
            escape_cypher(&a.locale)
        ));
        cypher.push_str(&format!(
            "  adapt.formality_default = \"{}\",\n",
            escape_cypher(&a.formality_default)
        ));
        cypher.push_str(&format!(
            "  adapt.technical_terms_approach = \"{}\",\n",
            escape_cypher(&a.technical_terms_approach)
        ));
        cypher.push_str(&format!(
            "  adapt.measurement_system = \"{}\",\n",
            escape_cypher(&a.measurement_system)
        ));
        cypher.push_str(&format!(
            "  adapt.hemisphere = \"{}\",\n",
            escape_cypher(&a.hemisphere)
        ));
        cypher.push_str(&format!(
            "  adapt.work_week = \"{}\",\n",
            escape_cypher(&a.work_week)
        ));

        // FACTS categories as JSON-like string for Neo4j
        if !a.facts_categories.is_empty() {
            let facts_json = serde_json::to_string(&a.facts_categories).unwrap_or_default();
            cypher.push_str(&format!(
                "  adapt.facts_categories = \"{}\",\n",
                escape_cypher(&facts_json)
            ));
        }

        // ILLUSTRATIONS categories as JSON-like string
        if !a.illustrations_categories.is_empty() {
            let illus_json = serde_json::to_string(&a.illustrations_categories).unwrap_or_default();
            cypher.push_str(&format!(
                "  adapt.illustrations_categories = \"{}\",\n",
                escape_cypher(&illus_json)
            ));
        }

        // Optional punctuation spacing
        if let Some(ref spacing) = a.punctuation_spacing {
            let spacing_json = serde_json::to_string(spacing).unwrap_or_default();
            cypher.push_str(&format!(
                "  adapt.punctuation_spacing = \"{}\",\n",
                escape_cypher(&spacing_json)
            ));
        }

        // Optional legal compliance
        if let Some(ref legal) = a.legal_compliance {
            let legal_json = serde_json::to_string(legal).unwrap_or_default();
            cypher.push_str(&format!(
                "  adapt.legal_compliance = \"{}\",\n",
                escape_cypher(&legal_json)
            ));
        }

        // Skip raw_markdown for now - too large and complex for batch import
        cypher.push_str("  adapt.created_at = datetime(),\n");
        cypher.push_str("  adapt.updated_at = datetime()\n");
        cypher.push_str("ON MATCH SET\n");
        cypher.push_str("  adapt.updated_at = datetime()\n");
        cypher.push_str("MERGE (l)-[:HAS_ADAPTATION]->(adapt);\n\n");
    }

    cypher
}

/// Format a Vec<String> as a Cypher array literal.
fn format_string_array(arr: &[String]) -> String {
    let items: Vec<String> = arr
        .iter()
        .map(|s| format!("\"{}\"", escape_cypher(s)))
        .collect();
    format!("[{}]", items.join(", "))
}

/// Truncate content for Cypher storage (avoid massive strings).
/// Handles UTF-8 character boundaries properly.
#[allow(dead_code)]
fn truncate_for_cypher(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        // Find a valid UTF-8 boundary before max_len
        let truncated = s
            .char_indices()
            .take_while(|(i, _)| *i < max_len)
            .map(|(i, c)| i + c.len_utf8())
            .last()
            .unwrap_or(0);
        format!("{}... [truncated]", &s[..truncated])
    }
}
