// =============================================================================
// Migration 006: LocaleRulesFormatting Property Alignment
// =============================================================================
//
// Aligns LocaleRulesFormatting property names with YAML v7.11.0 schema.
//
// RENAMES:
//   date_format      → date_pattern
//   currency_format  → (split into currency_code, currency_symbol, etc.)
//
// ADDS missing properties to match YAML schema exactly.
//
// =============================================================================

// =============================================================================
// 1. EN-US: Align properties with YAML
// =============================================================================

MATCH (l:Locale {key: "en-US"})-[:HAS_RULES_FORMATTING]->(lrf:LocaleRulesFormatting)
SET lrf.date_pattern = COALESCE(lrf.date_format, "MM/DD/YYYY"),
    lrf.date_pattern_short = "M/D/YY",
    lrf.date_pattern_long = "MMMM D, YYYY",
    lrf.first_day_of_week = "sunday",
    lrf.decimal_separator = ".",
    lrf.thousands_separator = ",",
    lrf.grouping_pattern = 3,
    lrf.currency_code = "USD",
    lrf.currency_symbol = "$",
    lrf.currency_position = COALESCE(lrf.currency_position, "before"),
    lrf.currency_spacing = false,
    lrf.time_format = COALESCE(lrf.time_format, "12h"),
    lrf.time_separator = ":",
    lrf.percentage_spacing = false,
    lrf.updated_at = datetime()
REMOVE lrf.date_format, lrf.currency_format, lrf.percentage_format, lrf.key;

// =============================================================================
// 2. FR-FR: Align properties with YAML
// =============================================================================

MATCH (l:Locale {key: "fr-FR"})-[:HAS_RULES_FORMATTING]->(lrf:LocaleRulesFormatting)
SET lrf.date_pattern = COALESCE(lrf.date_format, "DD/MM/YYYY"),
    lrf.date_pattern_short = "DD/MM/YY",
    lrf.date_pattern_long = "D MMMM YYYY",
    lrf.first_day_of_week = "monday",
    lrf.decimal_separator = ",",
    lrf.thousands_separator = " ",
    lrf.grouping_pattern = 3,
    lrf.currency_code = "EUR",
    lrf.currency_symbol = "€",
    lrf.currency_position = COALESCE(lrf.currency_position, "after"),
    lrf.currency_spacing = true,
    lrf.time_format = "24h",
    lrf.time_separator = "h",
    lrf.percentage_spacing = true,
    lrf.updated_at = datetime()
REMOVE lrf.date_format, lrf.currency_format, lrf.percentage_format, lrf.key;

// =============================================================================
// 3. FR-CA: Align properties with YAML
// =============================================================================

MATCH (l:Locale {key: "fr-CA"})-[:HAS_RULES_FORMATTING]->(lrf:LocaleRulesFormatting)
SET lrf.date_pattern = COALESCE(lrf.date_format, lrf.date_pattern, "YYYY-MM-DD"),
    lrf.date_pattern_short = "YY-MM-DD",
    lrf.date_pattern_long = "D MMMM YYYY",
    lrf.first_day_of_week = "sunday",
    lrf.decimal_separator = ",",
    lrf.thousands_separator = " ",
    lrf.grouping_pattern = 3,
    lrf.currency_code = COALESCE(lrf.currency_code, "CAD"),
    lrf.currency_symbol = "$",
    lrf.currency_position = COALESCE(lrf.currency_position, "after"),
    lrf.currency_spacing = true,
    lrf.time_format = "24h",
    lrf.time_separator = "h",
    lrf.percentage_spacing = true,
    lrf.updated_at = datetime()
REMOVE lrf.date_format, lrf.currency_format, lrf.percentage_format, lrf.key;

// =============================================================================
// 4. ES-ES: Align properties with YAML
// =============================================================================

MATCH (l:Locale {key: "es-ES"})-[:HAS_RULES_FORMATTING]->(lrf:LocaleRulesFormatting)
SET lrf.date_pattern = COALESCE(lrf.date_format, lrf.date_pattern, "DD/MM/YYYY"),
    lrf.date_pattern_short = "DD/MM/YY",
    lrf.date_pattern_long = "D de MMMM de YYYY",
    lrf.first_day_of_week = "monday",
    lrf.decimal_separator = ",",
    lrf.thousands_separator = ".",
    lrf.grouping_pattern = 3,
    lrf.currency_code = COALESCE(lrf.currency_code, "EUR"),
    lrf.currency_symbol = "€",
    lrf.currency_position = COALESCE(lrf.currency_position, "after"),
    lrf.currency_spacing = true,
    lrf.time_format = "24h",
    lrf.time_separator = ":",
    lrf.percentage_spacing = true,
    lrf.updated_at = datetime()
REMOVE lrf.date_format, lrf.currency_format, lrf.percentage_format, lrf.key;

// =============================================================================
// 5. DE-DE: Align properties with YAML
// =============================================================================

MATCH (l:Locale {key: "de-DE"})-[:HAS_RULES_FORMATTING]->(lrf:LocaleRulesFormatting)
SET lrf.date_pattern = COALESCE(lrf.date_format, lrf.date_pattern, "DD.MM.YYYY"),
    lrf.date_pattern_short = "DD.MM.YY",
    lrf.date_pattern_long = "D. MMMM YYYY",
    lrf.first_day_of_week = "monday",
    lrf.decimal_separator = ",",
    lrf.thousands_separator = ".",
    lrf.grouping_pattern = 3,
    lrf.currency_code = COALESCE(lrf.currency_code, "EUR"),
    lrf.currency_symbol = "€",
    lrf.currency_position = COALESCE(lrf.currency_position, "after"),
    lrf.currency_spacing = true,
    lrf.time_format = "24h",
    lrf.time_separator = ":",
    lrf.percentage_spacing = true,
    lrf.updated_at = datetime()
REMOVE lrf.date_format, lrf.currency_format, lrf.percentage_format, lrf.key;

// =============================================================================
// 6. JA-JP: Align properties with YAML
// =============================================================================

MATCH (l:Locale {key: "ja-JP"})-[:HAS_RULES_FORMATTING]->(lrf:LocaleRulesFormatting)
SET lrf.date_pattern = COALESCE(lrf.date_format, "YYYY/MM/DD"),
    lrf.date_pattern_short = "YY/MM/DD",
    lrf.date_pattern_long = "YYYY年M月D日",
    lrf.first_day_of_week = "sunday",
    lrf.decimal_separator = ".",
    lrf.thousands_separator = ",",
    lrf.grouping_pattern = 3,
    lrf.currency_code = "JPY",
    lrf.currency_symbol = "¥",
    lrf.currency_position = COALESCE(lrf.currency_position, "before"),
    lrf.currency_spacing = false,
    lrf.time_format = "24h",
    lrf.time_separator = ":",
    lrf.percentage_spacing = false,
    lrf.updated_at = datetime()
REMOVE lrf.date_format, lrf.currency_format, lrf.percentage_format, lrf.key;

// =============================================================================
// END MIGRATION 006
// =============================================================================
