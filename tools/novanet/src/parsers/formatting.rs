//! Parser for ATH 2-rules-formatting data.
//!
//! Transforms ATH formatting markdown files into Rust structs for Cypher generation.

use std::collections::HashMap;
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
// Note: High-priority patterns for file metadata and frequently-parsed sections.
// Additional patterns can be migrated incrementally.

/// Template version extraction
static RE_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"template_version:\s*(.+)").expect("valid version regex"));

/// Last updated date extraction
static RE_DATE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"last_updated:\s*(.+)").expect("valid date regex"));

/// Data sources extraction: **Data Sources**: ...
static RE_DATA_SOURCES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\*\*Data Sources\*\*:\s*(.+)").expect("valid data sources regex")
});

/// Section header: ## N. Title
static RE_SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"##\s+\d+\.\s+(.+)").expect("valid section regex"));

/// Field pattern: - **FieldName**: `value` or - **FieldName**: value
/// Used by: parse_number_section, parse_date_section, parse_time_section,
///          parse_currency_section, parse_measurement_section, parse_calendar_section
static RE_FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"-\s+\*\*(\w+)\*\*:\s*`?([^`\n]+)`?").expect("valid field regex")
});

// ============================================================================
// Main Structs
// ============================================================================

/// Complete formatting rules for a locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formatting {
    pub key: String,
    pub display_name: String,
    pub description: String,
    pub llm_context: String,
    pub data_sources: Vec<String>,
    pub number: NumberFormatting,
    pub date: DateFormatting,
    pub time: TimeFormatting,
    pub currency: CurrencyFormatting,
    pub phone: PhoneFormatting,
    pub address: AddressFormatting,
    pub measurement: MeasurementSystem,
    pub percentage: PercentageFormatting,
    pub temperature: TemperatureFormatting,
    pub validation_patterns: HashMap<String, String>,
    pub template_version: String,
    pub source_file: String,
    pub last_updated: String,
}

impl Formatting {
    /// Generate LLM context from formatting data.
    pub fn generate_llm_context(&mut self) {
        let mut parts = Vec::new();

        // Key formatting characteristics
        parts.push(format!(
            "{}: Numbers use '{}' decimal, '{}' thousands.",
            self.key, self.number.decimal_separator, self.number.thousands_separator
        ));

        // Date format
        parts.push(format!(
            "Dates: {} ({})",
            self.date.pattern, self.date.calendar_system
        ));

        // Time system
        parts.push(format!("Time: {}", self.time.system));

        // Currency
        parts.push(format!(
            "Currency: {} {} amount",
            self.currency.symbol,
            if self.currency.symbol_position == "before" {
                "before"
            } else {
                "after"
            }
        ));

        // Top incorrect examples (most valuable for LLM)
        if !self.number.incorrect_examples.is_empty() {
            let ex = &self.number.incorrect_examples[0];
            parts.push(format!("NEVER: {} -> {}", ex.input, ex.output));
        }

        self.llm_context = parts.join(" ");
    }
}

// ============================================================================
// Section Structs
// ============================================================================

/// Number formatting rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberFormatting {
    pub decimal_separator: String,
    pub thousands_separator: String,
    pub negative_sign: String,
    pub positive_sign: String,
    pub grouping_pattern: u8,
    pub numeral_system: Option<String>,
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

impl Default for NumberFormatting {
    fn default() -> Self {
        Self {
            decimal_separator: ".".to_string(),
            thousands_separator: ",".to_string(),
            negative_sign: "-".to_string(),
            positive_sign: "+".to_string(),
            grouping_pattern: 3,
            numeral_system: None,
            correct_examples: Vec::new(),
            incorrect_examples: Vec::new(),
        }
    }
}

/// Date formatting rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateFormatting {
    pub pattern: String,
    pub short_pattern: String,
    pub long_pattern: String,
    pub full_pattern: Option<String>,
    pub date_separator: String,
    pub month_names: Vec<String>,
    pub month_abbrev: Vec<String>,
    pub day_names: Vec<String>,
    pub day_abbrev: Vec<String>,
    pub hijri_months: Option<Vec<String>>,
    pub calendar_system: String,
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

impl Default for DateFormatting {
    fn default() -> Self {
        Self {
            pattern: "DD/MM/YYYY".to_string(),
            short_pattern: "DD/MM/YY".to_string(),
            long_pattern: "D MMMM YYYY".to_string(),
            full_pattern: None,
            date_separator: "/".to_string(),
            month_names: Vec::new(),
            month_abbrev: Vec::new(),
            day_names: Vec::new(),
            day_abbrev: Vec::new(),
            hijri_months: None,
            calendar_system: "gregorian".to_string(),
            correct_examples: Vec::new(),
            incorrect_examples: Vec::new(),
        }
    }
}

/// Time formatting rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeFormatting {
    pub system: String,
    pub pattern: String,
    pub pattern_with_seconds: String,
    pub time_separator: String,
    pub am_indicator: Option<String>,
    pub pm_indicator: Option<String>,
    pub prayer_times: Option<PrayerTimes>,
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

impl Default for TimeFormatting {
    fn default() -> Self {
        Self {
            system: "24-hour".to_string(),
            pattern: "HH:mm".to_string(),
            pattern_with_seconds: "HH:mm:ss".to_string(),
            time_separator: ":".to_string(),
            am_indicator: None,
            pm_indicator: None,
            prayer_times: None,
            correct_examples: Vec::new(),
            incorrect_examples: Vec::new(),
        }
    }
}

/// Prayer times for Islamic locales.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrayerTimes {
    pub fajr: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub isha: String,
}

/// Currency formatting rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyFormatting {
    pub code: String,
    pub symbol: String,
    pub symbol_position: String,
    pub space_between: bool,
    pub decimal_places: u8,
    pub subunit: Option<String>,
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

impl Default for CurrencyFormatting {
    fn default() -> Self {
        Self {
            code: "USD".to_string(),
            symbol: "$".to_string(),
            symbol_position: "before".to_string(),
            space_between: false,
            decimal_places: 2,
            subunit: None,
            correct_examples: Vec::new(),
            incorrect_examples: Vec::new(),
        }
    }
}

/// Phone formatting rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneFormatting {
    pub country_code: String,
    pub national_pattern: String,
    pub international_pattern: String,
    pub mobile_prefixes: Vec<String>,
    pub landline_prefixes: Vec<LandlinePrefix>,
    pub special_prefixes: Option<Vec<String>>,
    pub digit_count: u8,
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

impl Default for PhoneFormatting {
    fn default() -> Self {
        Self {
            country_code: "+1".to_string(),
            national_pattern: String::new(),
            international_pattern: String::new(),
            mobile_prefixes: Vec::new(),
            landline_prefixes: Vec::new(),
            special_prefixes: None,
            digit_count: 10,
            correct_examples: Vec::new(),
            incorrect_examples: Vec::new(),
        }
    }
}

/// Landline prefix with region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandlinePrefix {
    pub code: String,
    pub region: String,
}

/// Address formatting rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressFormatting {
    pub pattern: String,
    pub postal_code_pattern: String,
    pub postal_code_position: String,
    pub city_format: Option<String>,
    pub street_types: Option<Vec<String>>,
    pub po_box_format: Option<String>,
    pub example_addresses: Vec<String>,
    pub postal_code_examples: Vec<String>,
}

impl Default for AddressFormatting {
    fn default() -> Self {
        Self {
            pattern: String::new(),
            postal_code_pattern: String::new(),
            postal_code_position: "after_city".to_string(),
            city_format: None,
            street_types: None,
            po_box_format: None,
            example_addresses: Vec::new(),
            postal_code_examples: Vec::new(),
        }
    }
}

/// Measurement system rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementSystem {
    pub system: String,
    pub units: Vec<MeasurementUnit>,
    pub paper_size: String,
    pub notes: Vec<String>,
}

impl Default for MeasurementSystem {
    fn default() -> Self {
        Self {
            system: "metric".to_string(),
            units: Vec::new(),
            paper_size: "A4".to_string(),
            notes: Vec::new(),
        }
    }
}

/// Single measurement unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementUnit {
    pub category: String,
    pub unit: String,
    pub symbol: String,
    pub notes: Option<String>,
}

/// Percentage formatting rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercentageFormatting {
    pub format: String,
    pub space_before_symbol: bool,
    pub examples: Vec<String>,
}

impl Default for PercentageFormatting {
    fn default() -> Self {
        Self {
            format: "{number}%".to_string(),
            space_before_symbol: false,
            examples: Vec::new(),
        }
    }
}

/// Temperature formatting rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureFormatting {
    pub format: String,
    pub default_unit: String,
    pub examples: Vec<String>,
}

impl Default for TemperatureFormatting {
    fn default() -> Self {
        Self {
            format: "{number}°C".to_string(),
            default_unit: "celsius".to_string(),
            examples: Vec::new(),
        }
    }
}

/// Format example with input and output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatExample {
    pub input: String,
    pub output: String,
}

// ============================================================================
// Loading Functions
// ============================================================================

/// Load all formatting files from ATH data directory.
pub fn load_all_formattings(ath_path: &Path) -> Result<Vec<Formatting>> {
    let formatting_dir = ath_path.join("2-rules-formatting");

    if !formatting_dir.exists() {
        return Err(NovaNetError::Validation(format!(
            "Formatting directory not found: {}",
            formatting_dir.display()
        )));
    }

    // Collect all .md files
    let files: Vec<_> = WalkDir::new(&formatting_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .collect();

    // Parse in parallel
    let formattings: Vec<Formatting> = files
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();
            match parse_formatting_file(path) {
                Ok(f) => Some(f),
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                    None
                }
            }
        })
        .collect();

    Ok(formattings)
}

/// Parse a single formatting file.
pub fn parse_formatting_file(path: &Path) -> Result<Formatting> {
    let content = fs::read_to_string(path)?;
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Extract locale from filename (e.g., "fr-FR.md" -> "fr-FR")
    let locale = filename.trim_end_matches(".md");

    // Parse frontmatter
    let (template_version, last_updated) = parse_frontmatter(&content)?;

    // Parse data sources
    let data_sources = parse_data_sources(&content);

    // Split into sections
    let sections = split_sections(&content);

    // Parse each section
    let number = parse_number_section(sections.get("number").map(|s| s.as_str()).unwrap_or(""));
    let date = parse_date_section(sections.get("date").map(|s| s.as_str()).unwrap_or(""));
    let time = parse_time_section(sections.get("time").map(|s| s.as_str()).unwrap_or(""));
    let currency =
        parse_currency_section(sections.get("currency").map(|s| s.as_str()).unwrap_or(""));
    let phone = parse_phone_section(sections.get("phone").map(|s| s.as_str()).unwrap_or(""));
    let address = parse_address_section(sections.get("address").map(|s| s.as_str()).unwrap_or(""));
    let measurement = parse_measurement_section(
        sections
            .get("measurement")
            .map(|s| s.as_str())
            .unwrap_or(""),
    );
    let (percentage, temperature) = parse_percentage_temperature_section(
        sections.get("percentage").map(|s| s.as_str()).unwrap_or(""),
    );
    let validation_patterns =
        parse_validation_section(sections.get("validation").map(|s| s.as_str()).unwrap_or(""));

    let mut formatting = Formatting {
        key: locale.to_string(),
        display_name: format!("{} Formatting", locale),
        description: format!("Formatting rules for {}", locale),
        llm_context: String::new(),
        data_sources,
        number,
        date,
        time,
        currency,
        phone,
        address,
        measurement,
        percentage,
        temperature,
        validation_patterns,
        template_version,
        source_file: format!("2-rules-formatting/{}", filename),
        last_updated,
    };

    formatting.generate_llm_context();

    Ok(formatting)
}

// ============================================================================
// Parsing Functions
// ============================================================================

/// Parse YAML frontmatter.
fn parse_frontmatter(content: &str) -> Result<(String, String)> {
    let version = RE_VERSION
        .captures(content)
        .and_then(|c: regex::Captures| c.get(1))
        .map(|m: regex::Match| m.as_str().trim().to_string())
        .unwrap_or_else(|| "2.0".to_string());

    let date = RE_DATE
        .captures(content)
        .and_then(|c: regex::Captures| c.get(1))
        .map(|m: regex::Match| m.as_str().trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    Ok((version, date))
}

/// Parse data sources from content.
fn parse_data_sources(content: &str) -> Vec<String> {
    RE_DATA_SOURCES
        .captures(content)
        .and_then(|c: regex::Captures| c.get(1))
        .map(|m: regex::Match| {
            m.as_str()
                .split(',')
                .map(|s| {
                    s.trim()
                        .trim_start_matches('(')
                        .split(')')
                        .next()
                        .unwrap_or(s)
                        .trim()
                        .to_string()
                })
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// Split content into sections by ## headers.
fn split_sections(content: &str) -> HashMap<String, String> {
    let mut sections = HashMap::new();

    let mut current_section: Option<String> = None;
    let mut current_content = String::new();

    for line in content.lines() {
        if let Some(caps) = RE_SECTION.captures(line) {
            // Save previous section
            if let Some(ref name) = current_section {
                sections.insert(name.clone(), current_content.clone());
            }

            // Start new section
            let section_name = caps
                .get(1)
                .map(|m: regex::Match| m.as_str().to_lowercase())
                .unwrap_or_default();

            current_section = Some(normalize_section_name(&section_name));
            current_content = String::new();
        } else if current_section.is_some() {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Save last section
    if let Some(name) = current_section {
        sections.insert(name, current_content);
    }

    sections
}

/// Normalize section name to standard key.
fn normalize_section_name(name: &str) -> String {
    if name.contains("number") {
        "number".to_string()
    } else if name.contains("date") {
        "date".to_string()
    } else if name.contains("time") {
        "time".to_string()
    } else if name.contains("currency") {
        "currency".to_string()
    } else if name.contains("phone") {
        "phone".to_string()
    } else if name.contains("address") {
        "address".to_string()
    } else if name.contains("measurement") {
        "measurement".to_string()
    } else if name.contains("percentage") || name.contains("temperature") {
        "percentage".to_string()
    } else if name.contains("validation") {
        "validation".to_string()
    } else {
        name.to_string()
    }
}

/// Parse number formatting section.
fn parse_number_section(content: &str) -> NumberFormatting {
    let mut number = NumberFormatting::default();

    // Parse key-value pairs using module-level RE_FIELD (compiled once via LazyLock)
    for caps in RE_FIELD.captures_iter(content) {
        let field = caps.get(1).map(|m: regex::Match| m.as_str()).unwrap_or("");
        let value = caps
            .get(2)
            .map(|m: regex::Match| m.as_str().trim())
            .unwrap_or("");

        match field {
            "decimal_separator" => number.decimal_separator = clean_value(value),
            "thousands_separator" => number.thousands_separator = clean_value(value),
            "negative_sign" => number.negative_sign = clean_value(value),
            "positive_sign" => number.positive_sign = clean_value(value),
            "grouping_pattern" => {
                number.grouping_pattern = value.parse().unwrap_or(3);
            }
            _ => {}
        }
    }

    // Detect Arabic-Indic numerals
    if content.contains("Arabic-Indic") || content.contains("٠") {
        number.numeral_system = Some("arabic-indic".to_string());
    }

    // Parse examples
    number.correct_examples = parse_examples(content, true);
    number.incorrect_examples = parse_examples(content, false);

    number
}

/// Parse date formatting section.
fn parse_date_section(content: &str) -> DateFormatting {
    let mut date = DateFormatting::default();

    // Parse key-value pairs using module-level RE_FIELD (compiled once via LazyLock)
    for caps in RE_FIELD.captures_iter(content) {
        let field = caps.get(1).map(|m: regex::Match| m.as_str()).unwrap_or("");
        let value = caps
            .get(2)
            .map(|m: regex::Match| m.as_str().trim())
            .unwrap_or("");

        match field {
            "pattern" => date.pattern = clean_value(value),
            "short_pattern" => date.short_pattern = clean_value(value),
            "long_pattern" => date.long_pattern = clean_value(value),
            "full_pattern" => date.full_pattern = Some(clean_value(value)),
            "date_separator" => date.date_separator = clean_value(value),
            _ => {}
        }
    }

    // Parse month names
    date.month_names = parse_month_names(content, false);
    date.month_abbrev = parse_month_names(content, true);

    // Parse day names
    date.day_names = parse_day_names(content, false);
    date.day_abbrev = parse_day_names(content, true);

    // Check for Hijri calendar
    if content.contains("Hijri") || content.contains("هـ") {
        date.calendar_system = "hijri".to_string();
        date.hijri_months = Some(parse_hijri_months(content));
    }

    // Parse examples
    date.correct_examples = parse_examples(content, true);
    date.incorrect_examples = parse_examples(content, false);

    date
}

/// Parse time formatting section.
fn parse_time_section(content: &str) -> TimeFormatting {
    let mut time = TimeFormatting::default();

    // Parse key-value pairs using module-level RE_FIELD (compiled once via LazyLock)
    for caps in RE_FIELD.captures_iter(content) {
        let field = caps.get(1).map(|m: regex::Match| m.as_str()).unwrap_or("");
        let value = caps
            .get(2)
            .map(|m: regex::Match| m.as_str().trim())
            .unwrap_or("");

        match field {
            "system" => time.system = clean_value(value),
            "pattern" => time.pattern = clean_value(value),
            "pattern_with_seconds" => time.pattern_with_seconds = clean_value(value),
            "time_separator" => time.time_separator = clean_value(value),
            "am_indicator" => {
                let v = clean_value(value);
                if v != "N/A" && !v.is_empty() {
                    time.am_indicator = Some(v);
                }
            }
            "pm_indicator" => {
                let v = clean_value(value);
                if v != "N/A" && !v.is_empty() {
                    time.pm_indicator = Some(v);
                }
            }
            _ => {}
        }
    }

    // Parse prayer times if present
    if content.contains("Prayer Time") || content.contains("الفجر") {
        time.prayer_times = Some(parse_prayer_times(content));
    }

    // Parse examples
    time.correct_examples = parse_examples(content, true);
    time.incorrect_examples = parse_examples(content, false);

    time
}

/// Parse currency formatting section.
fn parse_currency_section(content: &str) -> CurrencyFormatting {
    let mut currency = CurrencyFormatting::default();

    // Parse key-value pairs using module-level RE_FIELD (compiled once via LazyLock)
    for caps in RE_FIELD.captures_iter(content) {
        let field = caps.get(1).map(|m: regex::Match| m.as_str()).unwrap_or("");
        let value = caps
            .get(2)
            .map(|m: regex::Match| m.as_str().trim())
            .unwrap_or("");

        match field {
            "code" => currency.code = clean_value(value),
            "symbol" => currency.symbol = clean_value(value),
            "symbol_position" => {
                let v = value.to_lowercase();
                currency.symbol_position = if v.contains("before") {
                    "before".to_string()
                } else {
                    "after".to_string()
                };
                currency.space_between = v.contains("space");
            }
            "decimal_places" => {
                currency.decimal_places = value
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse()
                    .unwrap_or(2);
            }
            "subunit" => currency.subunit = Some(clean_value(value)),
            _ => {}
        }
    }

    // Parse examples
    currency.correct_examples = parse_examples(content, true);
    currency.incorrect_examples = parse_examples(content, false);

    currency
}

/// Parse phone formatting section.
fn parse_phone_section(content: &str) -> PhoneFormatting {
    let mut phone = PhoneFormatting::default();

    // Parse key-value pairs using module-level RE_FIELD (compiled once via LazyLock)
    for caps in RE_FIELD.captures_iter(content) {
        let field = caps.get(1).map(|m: regex::Match| m.as_str()).unwrap_or("");
        let value = caps
            .get(2)
            .map(|m: regex::Match| m.as_str().trim())
            .unwrap_or("");

        match field {
            "country_code" => phone.country_code = clean_value(value),
            "national_pattern" => phone.national_pattern = clean_value(value),
            "international_pattern" => phone.international_pattern = clean_value(value),
            "mobile_prefixes" => {
                phone.mobile_prefixes = value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "digit_count" => {
                phone.digit_count = value
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse()
                    .unwrap_or(10);
            }
            _ => {}
        }
    }

    // Parse landline prefixes
    phone.landline_prefixes = parse_landline_prefixes(content);

    // Parse examples
    phone.correct_examples = parse_examples(content, true);
    phone.incorrect_examples = parse_examples(content, false);

    phone
}

/// Parse address formatting section.
fn parse_address_section(content: &str) -> AddressFormatting {
    let mut address = AddressFormatting::default();

    // Parse key-value pairs using module-level RE_FIELD (compiled once via LazyLock)
    for caps in RE_FIELD.captures_iter(content) {
        let field = caps.get(1).map(|m: regex::Match| m.as_str()).unwrap_or("");
        let value = caps
            .get(2)
            .map(|m: regex::Match| m.as_str().trim())
            .unwrap_or("");

        match field {
            "postal_code_pattern" => address.postal_code_pattern = clean_value(value),
            "postal_code_position" => {
                let v = value.to_lowercase();
                address.postal_code_position = if v.contains("before") {
                    "before_city".to_string()
                } else {
                    "after_city".to_string()
                };
            }
            "city_format" => address.city_format = Some(clean_value(value)),
            _ => {}
        }
    }

    // Parse address pattern from code block
    let pattern_re = Regex::new(r"```text\n([\s\S]*?)```").unwrap();
    if let Some(caps) = pattern_re.captures(content) {
        if let Some(pattern) = caps.get(1) {
            address.pattern = pattern.as_str().trim().to_string();
        }
    }

    // Parse street types if present
    let street_re = Regex::new(r"Street Types.*?:\s*\n-\s*(.+)").unwrap();
    if let Some(caps) = street_re.captures(content) {
        if let Some(types) = caps.get(1) {
            address.street_types = Some(
                types
                    .as_str()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
            );
        }
    }

    // Parse example addresses from code blocks
    let example_re = Regex::new(r"```text\n([\s\S]*?)```").unwrap();
    address.example_addresses = example_re
        .captures_iter(content)
        .skip(1) // Skip the pattern block
        .filter_map(|c| c.get(1).map(|m| m.as_str().trim().to_string()))
        .collect();

    // Parse postal code examples
    let postal_re = Regex::new(r"Valid:\s*`([^`]+)`").unwrap();
    if let Some(caps) = postal_re.captures(content) {
        if let Some(codes) = caps.get(1) {
            address.postal_code_examples = codes
                .as_str()
                .split(',')
                .map(|s| s.trim().trim_matches('`').to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }

    // Check for PO Box format
    if content.contains("ص.ب.") {
        address.po_box_format = Some("ص.ب.".to_string());
    } else if content.contains("PO Box") {
        address.po_box_format = Some("PO Box".to_string());
    }

    address
}

/// Parse measurement section.
fn parse_measurement_section(content: &str) -> MeasurementSystem {
    let mut measurement = MeasurementSystem::default();

    // Detect system
    if content.to_lowercase().contains("imperial") {
        measurement.system = "imperial".to_string();
    }

    // Parse units table
    let table_re = Regex::new(r"\|\s*(\w+)\s*\|\s*(\w+)\s*\|\s*([^\|]+)\s*\|").unwrap();
    for caps in table_re.captures_iter(content) {
        let category = caps
            .get(1)
            .map(|m: regex::Match| m.as_str().trim())
            .unwrap_or("");
        let unit = caps
            .get(2)
            .map(|m: regex::Match| m.as_str().trim())
            .unwrap_or("");
        let symbol = caps
            .get(3)
            .map(|m: regex::Match| m.as_str().trim())
            .unwrap_or("");

        if category != "Category" && !category.is_empty() && !category.contains("-") {
            measurement.units.push(MeasurementUnit {
                category: category.to_string(),
                unit: unit.to_string(),
                symbol: symbol.to_string(),
                notes: None,
            });
        }
    }

    // Parse paper size
    if content.contains("Letter") {
        measurement.paper_size = "Letter".to_string();
    }

    // Parse conversion notes
    let notes_re = Regex::new(r"-\s+([^-\n][^\n]+)").unwrap();
    let mut in_notes = false;
    for line in content.lines() {
        if line.contains("Conversion Notes") || line.contains("Notes") {
            in_notes = true;
            continue;
        }
        if in_notes {
            if line.starts_with("##") || line.starts_with("---") {
                break;
            }
            if let Some(caps) = notes_re.captures(line) {
                if let Some(note) = caps.get(1) {
                    measurement.notes.push(note.as_str().trim().to_string());
                }
            }
        }
    }

    measurement
}

/// Parse percentage and temperature section.
fn parse_percentage_temperature_section(
    content: &str,
) -> (PercentageFormatting, TemperatureFormatting) {
    let mut percentage = PercentageFormatting::default();
    let mut temperature = TemperatureFormatting::default();

    // Parse percentage
    if content.contains("space_before_symbol") {
        percentage.space_before_symbol = content.contains("true");
    }
    if content.contains(" %") || content.contains(" ٪") {
        percentage.space_before_symbol = true;
        percentage.format = "{number} %".to_string();
    }

    // Parse percentage examples
    let pct_ex_re = Regex::new(r"`(\d+(?:\.\d+)?)\s*%?`|`(\d+(?:,\d+)?)\s*[%٪]?`").unwrap();
    for caps in pct_ex_re.captures_iter(content) {
        if let Some(m) = caps.get(1).or(caps.get(2)) {
            percentage.examples.push(m.as_str().to_string());
        }
    }

    // Parse temperature
    if content.contains("°م") {
        temperature.format = "{number}°م".to_string();
    } else if content.contains(" °C") {
        temperature.format = "{number} °C".to_string();
    }

    // Parse temperature examples
    let temp_ex_re = Regex::new(r"`(-?\d+(?:\.\d+)?)\s*°[Cم]?`").unwrap();
    for caps in temp_ex_re.captures_iter(content) {
        if let Some(m) = caps.get(1) {
            temperature.examples.push(format!("{}°C", m.as_str()));
        }
    }

    (percentage, temperature)
}

/// Parse validation patterns section.
fn parse_validation_section(content: &str) -> HashMap<String, String> {
    let mut patterns = HashMap::new();

    // Parse table rows
    let row_re = Regex::new(r"\|\s*(\w+(?:\s+\([^)]+\))?)\s*\|\s*`([^`]+)`\s*\|").unwrap();

    for caps in row_re.captures_iter(content) {
        let type_name = caps
            .get(1)
            .map(|m: regex::Match| m.as_str().trim().to_lowercase())
            .unwrap_or_default();
        let pattern = caps
            .get(2)
            .map(|m: regex::Match| m.as_str().to_string())
            .unwrap_or_default();

        if !type_name.is_empty() && !pattern.is_empty() && type_name != "type" {
            patterns.insert(type_name, pattern);
        }
    }

    patterns
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Clean value by removing comments and extra characters.
fn clean_value(value: &str) -> String {
    value
        .split('#')
        .next()
        .unwrap_or(value)
        .split("//")
        .next()
        .unwrap_or(value)
        .trim()
        .trim_matches('`')
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

/// Parse examples from content.
fn parse_examples(content: &str, correct: bool) -> Vec<FormatExample> {
    let mut examples = Vec::new();

    // Find the right section
    let section_markers = if correct {
        vec!["**CORRECT Formatting**", "**Correct Formatting**"]
    } else {
        vec!["**NEVER use**", "**Incorrect Formatting**", "**Incorrect**"]
    };

    let end_markers = ["**CORRECT", "**NEVER", "**Incorrect", "---", "##"];

    let mut in_section = false;
    let example_re = Regex::new(r"-\s*`([^`]+)`\s*[→→-]\s*`([^`]+)`").unwrap();

    for line in content.lines() {
        // Check for section start
        if section_markers.iter().any(|m| line.contains(m)) {
            in_section = true;
            continue;
        }

        // Check for section end
        if in_section && end_markers.iter().any(|m| line.contains(m)) {
            break;
        }

        // Parse examples
        if in_section {
            if let Some(caps) = example_re.captures(line) {
                let input = caps.get(1).map(|m: regex::Match| m.as_str()).unwrap_or("");
                let output = caps.get(2).map(|m: regex::Match| m.as_str()).unwrap_or("");

                if !input.is_empty() && !output.is_empty() {
                    examples.push(FormatExample {
                        input: input.to_string(),
                        output: output.to_string(),
                    });
                }
            }
        }
    }

    examples
}

/// Parse month names from content.
fn parse_month_names(content: &str, abbreviated: bool) -> Vec<String> {
    let mut names = Vec::new();

    let section = if abbreviated {
        "Abbreviated Month Names"
    } else {
        "Full Month Names"
    };

    let mut in_section = false;
    let name_re = Regex::new(r"-\s+\w+:\s*(.+)").unwrap();

    for line in content.lines() {
        if line.contains(section) {
            in_section = true;
            continue;
        }
        if in_section {
            if line.starts_with("**") || line.starts_with("###") || line.starts_with("##") {
                break;
            }
            if let Some(caps) = name_re.captures(line) {
                if let Some(name) = caps.get(1) {
                    names.push(name.as_str().trim().to_string());
                }
            }
            // Handle comma-separated abbreviations
            if abbreviated && line.contains(',') && !line.starts_with("-") {
                names = line
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty() && !s.starts_with('#'))
                    .collect();
            }
        }
    }

    names
}

/// Parse day names from content.
fn parse_day_names(content: &str, abbreviated: bool) -> Vec<String> {
    let mut names = Vec::new();

    let section = if abbreviated {
        "Abbreviated Day Names"
    } else {
        "Full Day Names"
    };

    let mut in_section = false;
    let name_re = Regex::new(r"-\s+\w+:\s*(.+)").unwrap();

    for line in content.lines() {
        if line.contains(section) {
            in_section = true;
            continue;
        }
        if in_section {
            if line.starts_with("**") || line.starts_with("###") || line.starts_with("##") {
                break;
            }
            if let Some(caps) = name_re.captures(line) {
                if let Some(name) = caps.get(1) {
                    names.push(name.as_str().trim().to_string());
                }
            }
            // Handle comma-separated abbreviations
            if abbreviated && line.contains(',') && !line.starts_with("-") {
                names = line
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty() && !s.starts_with('#'))
                    .collect();
            }
        }
    }

    names
}

/// Parse Hijri month names.
fn parse_hijri_months(content: &str) -> Vec<String> {
    let mut months = Vec::new();

    let mut in_section = false;
    let name_re = Regex::new(r"-\s+\w+[^:]*:\s*(.+)").unwrap();

    for line in content.lines() {
        if line.contains("Hijri Month Names") {
            in_section = true;
            continue;
        }
        if in_section {
            if line.starts_with("**") || line.starts_with("###") || line.starts_with("##") {
                break;
            }
            if let Some(caps) = name_re.captures(line) {
                if let Some(name) = caps.get(1) {
                    months.push(name.as_str().trim().to_string());
                }
            }
        }
    }

    months
}

/// Parse prayer times from content.
fn parse_prayer_times(content: &str) -> PrayerTimes {
    let mut times = PrayerTimes {
        fajr: "~5:00".to_string(),
        dhuhr: "~12:30".to_string(),
        asr: "~15:30".to_string(),
        maghrib: "~18:00".to_string(),
        isha: "~19:30".to_string(),
    };

    let prayer_re = Regex::new(r"-\s*(الفجر|Fajr).*?:\s*~?(\d{1,2}:\d{2})").unwrap();
    if let Some(caps) = prayer_re.captures(content) {
        if let Some(t) = caps.get(2) {
            times.fajr = t.as_str().to_string();
        }
    }

    let dhuhr_re = Regex::new(r"-\s*(الظهر|Dhuhr).*?:\s*~?(\d{1,2}:\d{2})").unwrap();
    if let Some(caps) = dhuhr_re.captures(content) {
        if let Some(t) = caps.get(2) {
            times.dhuhr = t.as_str().to_string();
        }
    }

    let asr_re = Regex::new(r"-\s*(العصر|Asr).*?:\s*~?(\d{1,2}:\d{2})").unwrap();
    if let Some(caps) = asr_re.captures(content) {
        if let Some(t) = caps.get(2) {
            times.asr = t.as_str().to_string();
        }
    }

    let maghrib_re = Regex::new(r"-\s*(المغرب|Maghrib).*?:\s*~?(\d{1,2}:\d{2})").unwrap();
    if let Some(caps) = maghrib_re.captures(content) {
        if let Some(t) = caps.get(2) {
            times.maghrib = t.as_str().to_string();
        }
    }

    let isha_re = Regex::new(r"-\s*(العشاء|Isha).*?:\s*~?(\d{1,2}:\d{2})").unwrap();
    if let Some(caps) = isha_re.captures(content) {
        if let Some(t) = caps.get(2) {
            times.isha = t.as_str().to_string();
        }
    }

    times
}

/// Parse landline prefixes with regions.
fn parse_landline_prefixes(content: &str) -> Vec<LandlinePrefix> {
    let mut prefixes = Vec::new();

    let prefix_re = Regex::new(r"(\d{2,4})\s*\(([^)]+)\)").unwrap();

    for caps in prefix_re.captures_iter(content) {
        let code = caps.get(1).map(|m: regex::Match| m.as_str()).unwrap_or("");
        let region = caps.get(2).map(|m: regex::Match| m.as_str()).unwrap_or("");

        if !code.is_empty() {
            prefixes.push(LandlinePrefix {
                code: code.to_string(),
                region: region.to_string(),
            });
        }
    }

    prefixes
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_value() {
        assert_eq!(clean_value("`value`"), "value");
        assert_eq!(clean_value("value # comment"), "value");
        assert_eq!(clean_value("value // comment"), "value");
        assert_eq!(clean_value("  value  "), "value");
    }

    #[test]
    fn test_normalize_section_name() {
        assert_eq!(normalize_section_name("number formatting"), "number");
        assert_eq!(normalize_section_name("date formatting"), "date");
        assert_eq!(
            normalize_section_name("percentage & temperature"),
            "percentage"
        );
    }

    #[test]
    fn test_parse_examples() {
        let content = r#"
**CORRECT Formatting**:
- `1234.56` → `1 234,56`
- `10000` → `10 000`

**NEVER use**:
- `1234.56` → `1,234.56`
"#;

        let correct = parse_examples(content, true);
        assert_eq!(correct.len(), 2);
        assert_eq!(correct[0].input, "1234.56");
        assert_eq!(correct[0].output, "1 234,56");

        let incorrect = parse_examples(content, false);
        assert_eq!(incorrect.len(), 1);
        assert_eq!(incorrect[0].output, "1,234.56");
    }

    #[test]
    fn test_parse_number_section() {
        let content = r#"
- **decimal_separator**: `,`
- **thousands_separator**: ` `
- **negative_sign**: `-`
- **grouping_pattern**: 3
"#;

        let number = parse_number_section(content);
        assert_eq!(number.decimal_separator, ",");
        assert_eq!(number.thousands_separator, "");
        assert_eq!(number.grouping_pattern, 3);
    }

    #[test]
    fn test_parse_currency_section() {
        let content = r#"
- **code**: EUR
- **symbol**: `€`
- **symbol_position**: AFTER amount WITH non-breaking space
- **decimal_places**: 2
"#;

        let currency = parse_currency_section(content);
        assert_eq!(currency.code, "EUR");
        assert_eq!(currency.symbol, "€");
        assert_eq!(currency.symbol_position, "after");
        assert!(currency.space_between);
        assert_eq!(currency.decimal_places, 2);
    }

    #[test]
    fn test_parse_time_section() {
        let content = r#"
- **system**: 24-hour
- **pattern**: HH:mm
- **am_indicator**: N/A
- **pm_indicator**: N/A
"#;

        let time = parse_time_section(content);
        assert_eq!(time.system, "24-hour");
        assert_eq!(time.pattern, "HH:mm");
        assert!(time.am_indicator.is_none());
    }

    #[test]
    fn test_parse_time_section_12h() {
        let content = r#"
- **system**: 12-hour
- **am_indicator**: ص
- **pm_indicator**: م
"#;

        let time = parse_time_section(content);
        assert_eq!(time.system, "12-hour");
        assert_eq!(time.am_indicator, Some("ص".to_string()));
        assert_eq!(time.pm_indicator, Some("م".to_string()));
    }

    #[test]
    fn test_detect_arabic_indic() {
        let content = "Saudi Arabia strongly prefers Arabic-Indic numerals (٠-٩)";
        let number = parse_number_section(content);
        assert_eq!(number.numeral_system, Some("arabic-indic".to_string()));
    }

    #[test]
    fn test_parse_hijri_detection() {
        let content = r#"
- **pattern**: DD/MM/YYYY هـ

**Hijri Month Names (PRIMARY in Saudi Arabia)**:
- Muharram: محرم
- Safar: صفر
"#;

        let date = parse_date_section(content);
        assert_eq!(date.calendar_system, "hijri");
        assert!(date.hijri_months.is_some());
    }

    #[test]
    fn test_parse_landline_prefixes() {
        let content = "01 (Ile-de-France), 02 (Nord-Ouest), 03 (Nord-Est)";
        let prefixes = parse_landline_prefixes(content);

        assert_eq!(prefixes.len(), 3);
        assert_eq!(prefixes[0].code, "01");
        assert_eq!(prefixes[0].region, "Ile-de-France");
    }

    #[test]
    fn test_parse_validation_section() {
        let content = r#"
| Type | Pattern | Example |
|------|---------|---------|
| Date | `^\d{2}/\d{2}/\d{4}$` | `15/01/2026` |
| Phone | `^0[1-9]( \d{2}){4}$` | `06 12 34 56 78` |
"#;

        let patterns = parse_validation_section(content);
        assert_eq!(
            patterns.get("date"),
            Some(&r"^\d{2}/\d{2}/\d{4}$".to_string())
        );
        assert!(patterns.contains_key("phone"));
    }
}
