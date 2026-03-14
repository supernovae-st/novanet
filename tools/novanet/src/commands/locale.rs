//! Locale commands: `novanet locale list`, `novanet locale import`, `novanet locale generate`.
//!
//! Lists locales with their knowledge satellite status from Neo4j,
//! imports locale data from Cypher files, and generates Cypher from CSV + MD sources.
//!
//! Uses tiered knowledge model:
//! - Technical tier: Formatting, Slugification, Adaptation
//! - Style tier: Style
//! - Semantic tier: TermSet, ExpressionSet, PatternSet, CultureSet, TabooSet, AudienceSet
//!
//! The `locale generate` command produces 20-locales.cypher from:
//! - CSV file with 200 locales (basic info: codes, names, is_primary)
//! - 1-identity/*.md files (enrichment: native names, fallback chain)

use std::collections::HashMap;
use std::path::Path;

use crate::db::Db;
use crate::output::OutputFormat;

/// A locale row for display (tiered knowledge model).
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

/// List all locales with their knowledge satellite status (tiered model).
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
        },
        OutputFormat::Table => {
            let rows = db.execute(cypher).await?;
            let locale_rows = extract_locale_rows(&rows);
            let table = tabled::Table::new(&locale_rows)
                .with(tabled::settings::Style::rounded())
                .to_string();
            crate::output::print_output(&table);
            eprintln!("{} locale(s)", locale_rows.len());
        },
        OutputFormat::Json => {
            let rows = db.execute(cypher).await?;
            let locale_rows = extract_locale_rows(&rows);
            crate::output::print_output(&crate::output::format_json(&locale_rows));
        },
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
// LOCALE GENERATE
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
    script: String,
    #[serde(rename = "slug_rule")]
    _slug_rule: String,
    #[serde(rename = "Is RTL")]
    is_rtl: String, // "true" or "false" as string
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
    // Geographic properties for LLM retrieval
    region: String,
    language_family: String,
    script: String,
    text_direction: String,
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

    // Geographic properties
    let region = infer_region(&csv.country_code);
    let language_family = infer_language_family(&csv.language_code);
    let script = normalize_script(&csv.script);
    let text_direction = if csv.is_rtl.to_lowercase() == "true" {
        "rtl".to_string()
    } else {
        "ltr".to_string()
    };

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
        region,
        language_family,
        script,
        text_direction,
    }
}

/// Infer geographic region from ISO 3166-1 alpha-2 country code.
fn infer_region(country_code: &str) -> String {
    match country_code {
        // Europe
        "AT" | "BE" | "BG" | "HR" | "CY" | "CZ" | "DK" | "EE" | "FI" | "FR" | "DE" | "GR"
        | "HU" | "IE" | "IT" | "LV" | "LT" | "LU" | "MT" | "NL" | "PL" | "PT" | "RO" | "SK"
        | "SI" | "ES" | "SE" | "GB" | "UA" | "NO" | "CH" | "IS" | "AL" | "RS" | "BA" | "MK"
        | "ME" | "XK" | "MD" => "europe",
        // Asia
        "CN" | "JP" | "KR" | "TW" | "HK" | "MO" | "MN" | "VN" | "TH" | "ID" | "MY" | "SG"
        | "PH" | "MM" | "LA" | "KH" | "BN" | "IN" | "BD" | "PK" | "LK" | "NP" | "BT" => "asia",
        // Middle East
        "SA" | "AE" | "QA" | "KW" | "BH" | "OM" | "YE" | "IQ" | "SY" | "JO" | "LB" | "IL"
        | "PS" | "IR" | "TR" => "middle_east",
        // Africa
        "EG" | "DZ" | "MA" | "TN" | "LY" | "SD" | "ET" | "KE" | "NG" | "GH" | "ZA" | "TZ"
        | "UG" | "RW" | "SN" | "CI" | "CM" | "CD" | "AO" | "MZ" | "MG" | "ZW" | "ZM" | "BW"
        | "NA" | "MW" | "MU" => "africa",
        // Americas
        "US" | "CA" | "MX" | "BR" | "AR" | "CO" | "CL" | "PE" | "VE" | "EC" | "BO" | "PY"
        | "UY" | "GT" | "CR" | "PA" | "CU" | "DO" | "PR" | "HN" | "SV" | "NI" | "JM" | "TT"
        | "HT" | "BS" => "americas",
        // Oceania
        "AU" | "NZ" | "FJ" | "PG" | "WS" | "TO" | "VU" | "SB" => "oceania",
        _ => "other",
    }
    .to_string()
}

/// Infer language family from ISO 639-1 language code.
fn infer_language_family(language_code: &str) -> String {
    match language_code {
        // Romance languages
        "es" | "pt" | "fr" | "it" | "ro" | "ca" | "gl" => "romance",
        // Germanic languages
        "en" | "de" | "nl" | "sv" | "no" | "da" | "is" | "af" => "germanic",
        // Slavic languages
        "ru" | "uk" | "pl" | "cs" | "sk" | "bg" | "hr" | "sr" | "sl" | "mk" | "bs" | "be" => {
            "slavic"
        },
        // Sino-Tibetan
        "zh" | "my" => "sino_tibetan",
        // Semitic
        "ar" | "he" | "am" => "semitic",
        // Japonic
        "ja" => "japonic",
        // Koreanic
        "ko" => "koreanic",
        // Austronesian
        "id" | "ms" | "tl" | "fil" | "mg" | "jv" | "su" => "austronesian",
        // Indo-Aryan
        "hi" | "bn" | "pa" | "gu" | "mr" | "ne" | "si" | "ur" => "indo_aryan",
        // Dravidian
        "ta" | "te" | "kn" | "ml" => "dravidian",
        // Turkic
        "tr" | "az" | "uz" | "kk" | "ky" | "tk" => "turkic",
        // Uralic
        "fi" | "hu" | "et" => "uralic",
        // Thai-Kadai
        "th" | "lo" => "tai_kadai",
        // Austroasiatic
        "vi" | "km" => "austroasiatic",
        // Greek
        "el" => "hellenic",
        // Celtic
        "ga" | "cy" | "gd" => "celtic",
        // Baltic
        "lv" | "lt" => "baltic",
        // Albanian
        "sq" => "albanian",
        // Armenian
        "hy" => "armenian",
        // Georgian
        "ka" => "kartvelian",
        // Persian
        "fa" => "iranian",
        // Swahili (Bantu)
        "sw" => "bantu",
        _ => "other",
    }
    .to_string()
}

/// Normalize script name from CSV to YAML enum values.
fn normalize_script(script: &str) -> String {
    match script.to_lowercase().as_str() {
        "latin" | "latn" => "latin",
        "cyrillic" | "cyrl" => "cyrillic",
        "arabic" | "arab" => "arabic",
        "hebrew" | "hebr" => "hebrew",
        "han" | "hans" | "hant" | "cjk" => "cjk",
        "devanagari" | "deva" => "devanagari",
        "thai" => "thai",
        "hangul" | "kore" => "korean",
        "greek" | "grek" => "greek",
        "japanese" | "jpan" => "cjk",
        _ => "other",
    }
    .to_string()
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
    cypher.push_str("// 200 Locale nodes with geographic properties + fallback chains\n");
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
            "  l.content = \"{}\",\n",
            escape_cypher(&locale.description)
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
        // Geographic properties
        cypher.push_str(&format!(
            "  l.region = \"{}\",\n",
            escape_cypher(&locale.region)
        ));
        cypher.push_str(&format!(
            "  l.language_family = \"{}\",\n",
            escape_cypher(&locale.language_family)
        ));
        cypher.push_str(&format!(
            "  l.script = \"{}\",\n",
            escape_cypher(&locale.script)
        ));
        cypher.push_str(&format!(
            "  l.text_direction = \"{}\",\n",
            escape_cypher(&locale.text_direction)
        ));
        cypher.push_str("  l.created_at = datetime(),\n");
        cypher.push_str("  l.updated_at = datetime()\n");
        // ON MATCH: update all properties to ensure geographic fields are set
        cypher.push_str("ON MATCH SET\n");
        cypher.push_str(&format!(
            "  l.display_name = \"{}\",\n",
            escape_cypher(&locale.display_name)
        ));
        cypher.push_str(&format!(
            "  l.region = \"{}\",\n",
            escape_cypher(&locale.region)
        ));
        cypher.push_str(&format!(
            "  l.language_family = \"{}\",\n",
            escape_cypher(&locale.language_family)
        ));
        cypher.push_str(&format!(
            "  l.script = \"{}\",\n",
            escape_cypher(&locale.script)
        ));
        cypher.push_str(&format!(
            "  l.text_direction = \"{}\",\n",
            escape_cypher(&locale.text_direction)
        ));
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
