# Slugification Parser Design

> **Status**: Approved
> **Date**: 2026-02-05
> **Author**: Claude + Thibaut
> **Tier**: Technical (2-rules-slug)

## Overview

Parser for ATH `2-rules-slug/*.md` files → Neo4j `SlugRule` + `Slugification` nodes.

```
ATH Source: /Users/thibaut/Projects/traduction_ai/ath-know-l10n/outputs/localization-data/2-rules-slug/
Output: packages/db/seed/22-slugification.cypher
```

## Data Model

### SlugRule (5 nodes)

Meta-level rules that define slugification behavior for groups of locales.

| Key | Display Name | Encoding | Locales | Description |
|-----|--------------|----------|---------|-------------|
| `latin_strip` | Latin Strip | ASCII | ~150 | Remove diacritics, transliterate non-Latin |
| `latin_preserve` | Latin Preserve | UTF-8 | ~20 | Keep diacritics (Turkish ı→i, Vietnamese) |
| `latin_transform` | Latin Transform | ASCII | ~5 | Special transforms (German ß→ss) |
| `native_script` | Native Script | UTF-8 | ~15 | Keep native script (Arabic, Hebrew, Thai) |
| `romanized` | Romanized | ASCII | ~10 | Formal romanization (Japanese, Chinese, Korean) |

#### SlugRule Properties

```rust
pub struct SlugRule {
    pub key: String,              // "latin_strip"
    pub display_name: String,     // "Latin Strip"
    pub output_encoding: String,  // "ASCII" | "UTF-8"
    pub has_case: bool,           // true for Latin, false for Arabic
    pub normalization: String,    // "NFD" | "NFC" | "none"

    // Behavior
    pub diacritics: String,       // "remove" | "preserve" | "transform"
    pub non_latin: String,        // "transliterate" | "keep" | "romanize"
    pub case_handling: String,    // "lowercase" | "none"
    pub spaces: String,           // "hyphen"
    pub special_chars: String,    // "removed"

    // Script-specific
    pub script_rules: Option<HashMap<String, ScriptRule>>,

    // Stats
    pub locale_count: u32,
    pub llm_context: String,
}

pub struct ScriptRule {
    pub script: String,           // "arabic", "japanese", "thai"
    pub handling: String,         // "preserve", "romanize"
    pub special: Option<String>,  // "remove_tashkeel", "convert_numerals"
}
```

### Slugification (200 nodes)

Per-locale slugification configuration.

#### Slugification Properties

```rust
pub struct Slugification {
    pub key: String,              // "fr-FR"
    pub display_name: String,     // "French (France) Slugification"
    pub description: String,
    pub llm_context: String,

    // Rule reference
    pub slug_rule: String,        // FK to SlugRule.key

    // Stopwords (JSON string in Neo4j)
    pub stopwords: HashMap<String, Vec<String>>,
    pub stopwords_count: u32,

    // Regional additions (JSON string in Neo4j)
    pub regional_additions: Vec<RegionalAddition>,

    // Script config for non-Latin (JSON string in Neo4j)
    pub script_config: Option<ScriptConfig>,

    // Validation overrides
    pub validation_overrides: Option<HashMap<String, String>>,
    pub warnings: Vec<Warning>,

    // Examples (JSON string in Neo4j)
    pub examples: Vec<SlugExample>,

    // Metadata
    pub template_version: String,
    pub source_file: String,
    pub last_updated: String,
}

pub struct RegionalAddition {
    pub word: String,
    pub category: String,
    pub reason: String,
}

pub struct SlugExample {
    pub input: String,
    pub output: String,
    pub rules_applied: Vec<String>,
}

pub struct ScriptConfig {
    pub primary_script: String,
    pub diacritic_handling: Option<String>,  // "remove_tashkeel" for Arabic
    pub numeral_handling: Option<String>,    // "convert" for Thai
    pub special_chars: Option<Vec<String>>,  // ZWNJ for Persian
}

pub struct Warning {
    pub condition: String,
    pub message: String,
}
```

## Stopword Categories (30)

Discovered from ATH data analysis:

| Category | Description | Example Languages |
|----------|-------------|-------------------|
| `article` | Definite/indefinite articles | French, German, Spanish |
| `preposition` | Spatial/temporal prepositions | All |
| `conjunction` | Coordinating/subordinating | All |
| `pronoun` | Personal/demonstrative | All |
| `auxiliary` | Auxiliary verbs | English, French |
| `demonstrative` | This/that/these | All |
| `possessive` | My/your/his | All |
| `interrogative` | Question words | All |
| `negation` | Negative particles | All |
| `adverb` | Common adverbs | All |
| `verb` | Common verbs (be, have) | All |
| `particle_topic` | Topic markers (は) | Japanese |
| `particle_subject` | Subject markers (が) | Japanese |
| `particle_object` | Object markers (を) | Japanese |
| `particle_direction` | Direction markers (へ) | Japanese |
| `particle_location` | Location markers (で) | Japanese |
| `particle_possessive` | Possessive markers (の) | Japanese |
| `particle_quotation` | Quote markers (と) | Japanese |
| `honorific` | Honorific prefixes | Japanese, Korean |
| `classifier` | Measure words | Chinese, Japanese, Thai |
| `copula` | Copular verbs | Japanese, Korean |
| `proper_noun` | Country/region names | Regional |
| `currency` | Currency terms | Regional |
| `relative_pronoun` | Who/which/that | All |
| `indefinite` | Some/any/no | All |
| `quantifier` | All/many/few | All |
| `interjection` | Common interjections | All |
| `abbreviation` | Common abbreviations | All |
| `filler` | Filler words | All |
| `honorific_suffix` | -san, -님 | Japanese, Korean |

## Script-Specific Rules

### Arabic (native_script)
- Remove tashkeel: `[\u064B-\u065F\u0670]`
- Preserve alef variants: ا، أ، إ، آ
- Preserve hamza carriers: ؤ، ئ
- Keep Arabic-Indic numerals: ٠-٩

### Japanese (romanized)
- Dakuten is NOT a diacritic (changes letter identity)
- Use Hepburn romanization
- Particles are stopwords (7+ categories)

### Thai (native_script)
- Convert Thai numerals: ๐-๙ → 0-9
- No spaces in Thai (word boundaries complex)

### Persian (native_script)
- ZWNJ (U+200C) → hyphen
- Similar to Arabic but different script rules

### Greek (latin_strip)
- Final sigma (ς) → s
- Standard transliteration

### German (latin_transform)
- ß → ss (not strip, transform)
- ü → ue (traditional) or u (modern)

### Korean (romanized)
- Revised Romanization
- Agglutinative morphology affects stopwords

## CLI Integration

```bash
# Generate slugification nodes
novanet knowledge generate --tier=technical

# Generates:
# - packages/db/seed/22-slugification.cypher (SlugRule + Slugification nodes)
# - Arcs: Locale -[:HAS_SLUGIFICATION]-> Slugification -[:FOLLOWS_RULE]-> SlugRule
```

## Parser Logic

### File Discovery

```rust
let ath_path = "/Users/thibaut/Projects/traduction_ai/ath-know-l10n/outputs/localization-data";
let slug_files = glob(&format!("{}/2-rules-slug/*.md", ath_path))?;
// ~200 files: ar-SA.md, en-US.md, fr-FR.md, ja-JP.md, zh-CN.md, ...
```

### Parsing Strategy

```rust
pub fn parse_slugification(content: &str) -> Result<Slugification> {
    // 1. Parse YAML frontmatter
    let frontmatter = parse_frontmatter(content)?;
    let locale = frontmatter.get("locale")?;
    let template_version = frontmatter.get("template_version")?;
    let last_updated = frontmatter.get("last_updated")?;

    // 2. Extract slug rule from "## 1. Base Rule: {rule}"
    let slug_rule = extract_section(content, "Base Rule")
        .and_then(|s| parse_rule_name(&s))?;

    // 3. Extract stopwords from "## 2. Stopwords" tables
    let stopwords = extract_section(content, "Stopwords")
        .and_then(|s| parse_stopword_tables(&s))?;

    // 4. Extract regional additions from "Locale-Specific Additions" table
    let regional_additions = extract_section(content, "Locale-Specific")
        .map(|s| parse_regional_table(&s))
        .unwrap_or_default();

    // 5. Extract examples from "## 4. Examples" table
    let examples = extract_section(content, "Examples")
        .and_then(|s| parse_examples_table(&s))?;

    // 6. Extract validation rules and warnings
    let (validation_overrides, warnings) = extract_section(content, "Validation")
        .map(|s| parse_validation(&s))
        .unwrap_or_default();

    // 7. Extract script config for non-Latin locales
    let script_config = extract_section(content, "Character Handling")
        .and_then(|s| parse_script_config(&s, &slug_rule));

    // 8. Generate llm_context
    let llm_context = generate_llm_context(&locale, &slug_rule, &stopwords, &regional_additions);

    Ok(Slugification {
        key: locale.clone(),
        display_name: format!("{} Slugification", get_locale_name(&locale)),
        description: format!("URL slug generation rules for {}", locale),
        llm_context,
        slug_rule,
        stopwords,
        stopwords_count: count_stopwords(&stopwords),
        regional_additions,
        script_config,
        validation_overrides,
        warnings,
        examples,
        template_version,
        source_file: format!("2-rules-slug/{}.md", locale),
        last_updated,
    })
}
```

### SlugRule Aggregation

```rust
pub fn aggregate_slug_rules(slugifications: &[Slugification]) -> Vec<SlugRule> {
    let mut rules: HashMap<String, SlugRule> = HashMap::new();

    for s in slugifications {
        let rule = rules.entry(s.slug_rule.clone()).or_insert_with(|| {
            SlugRule::new(&s.slug_rule)
        });
        rule.locale_count += 1;
        rule.add_locale_example(&s.key);
    }

    rules.into_values().collect()
}
```

## Cypher Generation

### Output File

`packages/db/seed/22-slugification.cypher`

### Structure

```cypher
// ============================================================================
// SLUGIFICATION SEED - Generated from ATH 2-rules-slug
// Generated: 2026-02-05
// Source: /Users/thibaut/Projects/traduction_ai/ath-know-l10n/outputs/localization-data/2-rules-slug/
// ============================================================================

// ----------------------------------------------------------------------------
// PART 1: SlugRule nodes (5 total)
// ----------------------------------------------------------------------------

MERGE (sr:SlugRule {key: 'latin_strip'})
SET sr.display_name = 'Latin Strip',
    sr.output_encoding = 'ASCII',
    sr.has_case = true,
    sr.normalization = 'NFD',
    sr.diacritics = 'remove',
    sr.non_latin = 'transliterate',
    sr.case_handling = 'lowercase',
    sr.spaces = 'hyphen',
    sr.special_chars = 'removed',
    sr.locale_count = 150,
    sr.llm_context = 'The Latin Strip rule produces ASCII slugs. Diacritics are removed via NFD normalization. Non-Latin scripts are transliterated. Used by 150 locales including en-US, fr-FR, es-ES.';

MERGE (sr:SlugRule {key: 'latin_preserve'})
SET sr.display_name = 'Latin Preserve',
    sr.output_encoding = 'UTF-8',
    sr.has_case = true,
    sr.normalization = 'NFC',
    sr.diacritics = 'preserve',
    sr.non_latin = 'transliterate',
    sr.case_handling = 'lowercase',
    sr.spaces = 'hyphen',
    sr.special_chars = 'removed',
    sr.locale_count = 20,
    sr.llm_context = 'The Latin Preserve rule produces UTF-8 slugs keeping diacritics. Used for languages where diacritics change meaning (Turkish ı vs i, Vietnamese tones). Used by 20 locales including tr-TR, vi-VN.';

MERGE (sr:SlugRule {key: 'latin_transform'})
SET sr.display_name = 'Latin Transform',
    sr.output_encoding = 'ASCII',
    sr.has_case = true,
    sr.normalization = 'custom',
    sr.diacritics = 'transform',
    sr.non_latin = 'transliterate',
    sr.case_handling = 'lowercase',
    sr.spaces = 'hyphen',
    sr.special_chars = 'removed',
    sr.char_transforms = '{"ß":"ss","ü":"ue","ö":"oe","ä":"ae"}',
    sr.locale_count = 5,
    sr.llm_context = 'The Latin Transform rule produces ASCII slugs with special character transformations. German ß becomes ss, umlauts become ae/oe/ue. Used by 5 locales including de-DE, de-AT, de-CH.';

MERGE (sr:SlugRule {key: 'native_script'})
SET sr.display_name = 'Native Script',
    sr.output_encoding = 'UTF-8',
    sr.has_case = false,
    sr.normalization = 'NFC',
    sr.diacritics = 'script_specific',
    sr.non_latin = 'keep',
    sr.case_handling = 'none',
    sr.spaces = 'hyphen',
    sr.special_chars = 'removed',
    sr.locale_count = 15,
    sr.llm_context = 'The Native Script rule preserves non-Latin scripts in UTF-8 slugs. Arabic removes tashkeel but keeps script. Thai converts numerals. Hebrew preserves RTL. Used by 15 locales including ar-SA, he-IL, th-TH.';

MERGE (sr:SlugRule {key: 'romanized'})
SET sr.display_name = 'Romanized',
    sr.output_encoding = 'ASCII',
    sr.has_case = true,
    sr.normalization = 'NFD',
    sr.diacritics = 'remove',
    sr.non_latin = 'romanize',
    sr.case_handling = 'lowercase',
    sr.spaces = 'hyphen',
    sr.special_chars = 'removed',
    sr.romanization_systems = '{"ja":"hepburn","zh":"pinyin","ko":"revised"}',
    sr.locale_count = 10,
    sr.llm_context = 'The Romanized rule converts non-Latin scripts to ASCII via formal romanization systems. Japanese uses Hepburn, Chinese uses Pinyin, Korean uses Revised Romanization. Used by 10 locales including ja-JP, zh-CN, ko-KR.';

// ----------------------------------------------------------------------------
// PART 2: Slugification nodes (200 per locale)
// ----------------------------------------------------------------------------

MERGE (s:Slugification {key: 'fr-FR'})
SET s.display_name = 'French (France) Slugification',
    s.description = 'URL slug generation rules for fr-FR',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"article":["le","la","les","un","une","des"],"preposition":["de","du","dans","en","sur","pour","avec","par"],"conjunction":["et","ou","mais","donc"],"pronoun":["je","tu","il","elle","nous","vous","ils","elles"],"demonstrative":["ce","cette","ces"],"auxiliary":["est","sont","a","ont","être","avoir"]}',
    s.stopwords_count = 35,
    s.regional_additions = '[]',
    s.examples = '[{"input":"Les meilleures pizzerias de Paris","output":"meilleures-pizzerias-paris","rules_applied":["stopwords_removed","lowercase","spaces_to_hyphens"]},{"input":"Château de Versailles","output":"chateau-versailles","rules_applied":["diacritics_removed","stopwords_removed","lowercase"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-FR.md',
    s.last_updated = '2026-01-11',
    s.llm_context = 'URL slugification rules for French (France). Uses latin_strip rule (ASCII output). 35 stopwords across 6 categories including articles (le, la, les), prepositions (de, du, dans), and conjunctions (et, ou).';

// ... (198 more Slugification nodes)

// ----------------------------------------------------------------------------
// PART 3: Arcs Locale → Slugification
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-FR'})
MATCH (s:Slugification {key: 'fr-FR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

// ... (199 more arcs)

// ----------------------------------------------------------------------------
// PART 4: Arcs Slugification → SlugRule
// ----------------------------------------------------------------------------

MATCH (s:Slugification {key: 'fr-FR'})
MATCH (sr:SlugRule {key: 'latin_strip'})
MERGE (s)-[:FOLLOWS_RULE]->(sr);

// ... (199 more arcs)
```

## llm_context Templates

### SlugRule Template

```
The {display_name} rule produces {output_encoding} slugs.
Diacritics are {diacritics_description}.
Non-Latin scripts are {non_latin_description}.
Used by {locale_count} locales including {top_3_locales}.
```

### Slugification Template

```
URL slugification rules for {locale_display_name}.
Uses {slug_rule} rule ({encoding} output).
{stopwords_count} stopwords across {category_count} categories
including {top_categories_with_examples}.
{regional_note if regional_additions not empty}
```

## File Structure

```
tools/novanet/src/
├── parsers/
│   ├── mod.rs              # Add slugification module
│   └── slugification.rs    # Parse MD → structs
├── generators/
│   ├── mod.rs              # Add slugification module
│   └── slugification.rs    # structs → Cypher
└── commands/
    ├── mod.rs              # Add knowledge module
    └── knowledge.rs        # CLI: novanet knowledge generate
```

## Validation Rules

### Required
- `key` must match filename (fr-FR.md → key: fr-FR)
- `slug_rule` must be one of 5 valid rules
- `stopwords` must have at least 1 category
- `examples` must have at least 1 entry

### Warnings
- Stopwords count > 50 (unusually high)
- No regional additions for major locales
- Mixed script in examples

## Testing

```bash
# Unit tests
cargo test slugification

# Integration test (requires ATH data)
cargo test --ignored test_parse_all_slug_files

# Snapshot tests
cargo insta test -p novanet --review
```

## References

- ATH Source: `/Users/thibaut/Projects/traduction_ai/ath-know-l10n/outputs/localization-data/2-rules-slug/`
- Neo4j Taxonomy: `packages/db/seed/00.5-taxonomy.cypher`
- Existing locale parser: `tools/novanet/src/commands/locale.rs`
