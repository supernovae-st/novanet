# Formatting Parser Design

**Date**: 2026-02-05
**Status**: Approved
**Tier**: Technical (2-rules-formatting)

## Overview

Parser for ATH `2-rules-formatting/*.md` files → Neo4j Formatting nodes.

## Decisions

| Question | Decision |
|----------|----------|
| Graph structure | Single Formatting node per locale (200 nodes) |
| Rust struct strategy | Nested structs for each section |
| Locale-specific features | Optional fields in structs |
| Examples storage | Store both correct and incorrect examples |

## Data Model

### Main Struct

```rust
pub struct Formatting {
    pub key: String,                    // "fr-FR"
    pub display_name: String,           // "French (France) Formatting"
    pub description: String,            // Auto-generated summary
    pub llm_context: String,            // Rich context for LLM
    pub data_sources: Vec<String>,      // ["CLDR", "ISO 4217", ...]
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
```

### Section Structs

```rust
pub struct NumberFormatting {
    pub decimal_separator: String,      // "," or "." or "٫"
    pub thousands_separator: String,    // " " or "," or "٬"
    pub negative_sign: String,          // "-" or "؜-"
    pub positive_sign: String,          // "+"
    pub grouping_pattern: u8,           // 3 (thousands)
    pub numeral_system: Option<String>, // "arabic-indic" for ar-SA
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

pub struct DateFormatting {
    pub pattern: String,                // "DD/MM/YYYY" or "YYYY/MM/DD"
    pub short_pattern: String,
    pub long_pattern: String,
    pub full_pattern: Option<String>,
    pub date_separator: String,         // "/" or "-"
    pub month_names: Vec<String>,       // ["janvier", ...] or ["1月", ...]
    pub month_abbrev: Vec<String>,
    pub day_names: Vec<String>,
    pub day_abbrev: Vec<String>,
    pub hijri_months: Option<Vec<String>>,  // For ar-SA
    pub calendar_system: String,        // "gregorian" or "hijri"
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

pub struct TimeFormatting {
    pub system: String,                 // "24-hour" or "12-hour"
    pub pattern: String,                // "HH:mm" or "h:mm a"
    pub pattern_with_seconds: String,
    pub time_separator: String,         // ":" or "h"
    pub am_indicator: Option<String>,   // "AM", "午前", "ص"
    pub pm_indicator: Option<String>,   // "PM", "午後", "م"
    pub prayer_times: Option<PrayerTimes>,
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

pub struct CurrencyFormatting {
    pub code: String,                   // "EUR", "JPY", "SAR"
    pub symbol: String,                 // "€", "¥", "ر.س"
    pub symbol_position: String,        // "before" or "after"
    pub space_between: bool,            // true for "10 €", false for "¥10"
    pub decimal_places: u8,             // 2 for EUR, 0 for JPY
    pub subunit: Option<String>,        // "centime", "halala"
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

pub struct PhoneFormatting {
    pub country_code: String,           // "+33", "+81", "+966"
    pub national_pattern: String,       // "0X XX XX XX XX"
    pub international_pattern: String,
    pub mobile_prefixes: Vec<String>,   // ["06", "07"] for fr-FR
    pub landline_prefixes: Vec<LandlinePrefix>,
    pub special_prefixes: Option<Vec<String>>,
    pub digit_count: u8,                // 10 typically
    pub correct_examples: Vec<FormatExample>,
    pub incorrect_examples: Vec<FormatExample>,
}

pub struct AddressFormatting {
    pub pattern: String,                // Multi-line template
    pub postal_code_pattern: String,    // "NNNNN" or "NNN-NNNN"
    pub postal_code_position: String,   // "before_city" or "after_city"
    pub city_format: Option<String>,    // "uppercase" for fr-FR
    pub street_types: Option<Vec<String>>,
    pub po_box_format: Option<String>,  // "ص.ب." for ar-SA
    pub example_addresses: Vec<String>,
    pub postal_code_examples: Vec<String>,
}

pub struct MeasurementSystem {
    pub system: String,                 // "metric" or "imperial"
    pub units: Vec<MeasurementUnit>,
    pub paper_size: String,             // "A4" or "Letter"
    pub notes: Vec<String>,             // Exceptions
}

pub struct PercentageFormatting {
    pub format: String,                 // "{number} %" or "{number}%"
    pub space_before_symbol: bool,
    pub examples: Vec<String>,
}

pub struct TemperatureFormatting {
    pub format: String,                 // "{number} °C" or "{number}°م"
    pub default_unit: String,           // "celsius"
    pub examples: Vec<String>,
}

// Helper structs
pub struct FormatExample {
    pub input: String,
    pub output: String,
}

pub struct LandlinePrefix {
    pub code: String,
    pub region: String,
}

pub struct MeasurementUnit {
    pub category: String,
    pub unit: String,
    pub symbol: String,
    pub notes: Option<String>,
}

pub struct PrayerTimes {
    pub fajr: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub isha: String,
}
```

## Parser Logic

### File Structure

```
---
locale: fr-FR
type: rules-formatting
template_version: 2.0
last_updated: 2026-01-11
---

# Rules Formatting: fr-FR

## 1. Number Formatting
### Separators & Signs
- **decimal_separator**: `,`
...
### Examples
**CORRECT Formatting**:
- `1234.56` → `1 234,56`
...
**NEVER use**:
- `1234.56` → `1,234.56`
...

## 2. Date Formatting
...
```

### Parsing Functions

1. `parse_frontmatter()` - Extract YAML header
2. `split_sections()` - Split on `## N.` pattern
3. `parse_number_section()` - Extract NumberFormatting
4. `parse_date_section()` - Extract DateFormatting + Hijri detection
5. `parse_time_section()` - Extract TimeFormatting + prayer times
6. `parse_currency_section()` - Extract CurrencyFormatting
7. `parse_phone_section()` - Extract PhoneFormatting
8. `parse_address_section()` - Extract AddressFormatting
9. `parse_measurement_section()` - Extract MeasurementSystem
10. `parse_validation_section()` - Extract regex patterns from table

### Example Extraction

- Correct examples: Lines after `**CORRECT Formatting**:`
- Incorrect examples: Lines after `**NEVER use**` or `**Incorrect**`
- Format: `- \`input\` → \`output\``

## Cypher Generation

### Graph Structure

```
Locale ──[:HAS_FORMATTING]──> Formatting
```

- 200 Formatting nodes (one per locale)
- 200 HAS_FORMATTING arcs

### Output File

`packages/db/seed/23-formatting.cypher`

```cypher
// Part 1: Formatting nodes
MERGE (f:Formatting {key: 'fr-FR'})
SET f.display_name = 'French (France) Formatting',
    f.description = '...',
    f.llm_context = '...',
    f.data_sources = '["CLDR", "ISO 4217", ...]',
    f.number = '{...}',
    f.date = '{...}',
    f.time = '{...}',
    f.currency = '{...}',
    f.phone = '{...}',
    f.address = '{...}',
    f.measurement = '{...}',
    f.percentage = '{...}',
    f.temperature = '{...}',
    f.validation_patterns = '{...}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-FR.md',
    f.last_updated = '2026-01-11';

// Part 2: Arcs Locale → Formatting
MATCH (l:Locale {key: 'fr-FR'})
MATCH (f:Formatting {key: 'fr-FR'})
MERGE (l)-[:HAS_FORMATTING]->(f);
```

### LLM Context Generation

- Auto-summarize key formatting rules
- Highlight critical differences (12h vs 24h, Hijri, Arabic-Indic numerals)
- Include top 3 "NEVER use" examples per section
- ~500 chars max

## CLI Integration

```bash
novanet knowledge generate --tier=technical
```

Generates:
- `22-slugification.cypher` (existing)
- `23-formatting.cypher` (new)

## Statistics

- Nodes: 200 Formatting
- Arcs: 200 HAS_FORMATTING
- Estimated file size: ~2-3 MB
