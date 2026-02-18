# Locale Knowledge 7-Node Architecture Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Migrate locale knowledge data from ath-know-l10n to NovaNet's new 7-node architecture with 2 new nodes (Culture, Market), updated Style/Adaptation, and Knowledge Atoms.

**Architecture:** 3-tier property structure (Indexed Scalars / Structured JSON / LLM Context) for all config nodes. YAML-first with Rust parsers. Data flows from markdown sources → Rust parser → Cypher seeds → Neo4j.

**Tech Stack:** Rust (parsers), YAML (schema), Cypher (Neo4j seeds), TypeScript (types)

---

## Phase 1: Schema Updates (YAML Definitions)

### Task 1.1: Create Culture Node YAML

**Files:**
- Create: `packages/core/models/node-classes/global/config/culture.yaml`

**Step 1: Write the YAML schema**

```yaml
node:
  name: Culture
  realm: global
  layer: config
  trait: knowledge
  display_name: "Culture"
  description: "Cultural context for a locale including calendar, seasons, business norms, communication style, and core values. Used by LLM for culturally-appropriate content generation."
  llm_context: "Cultural configuration that influences tone, timing references, and value expressions in generated content."

  properties:
    # TIER 1: INDEXED SCALARS (Cypher-queryable)
    - name: hemisphere
      type: string
      enum: ["northern", "southern"]
      required: true
      description: "Geographic hemisphere affecting season references"
      llm_context: "Determines which months map to summer/winter in generated content"

    - name: work_week_start
      type: string
      enum: ["sunday", "monday", "saturday"]
      required: true
      description: "First day of the business week"
      llm_context: "Affects 'start of week' and 'weekend' references"

    - name: communication_directness
      type: string
      enum: ["direct", "indirect", "contextual"]
      required: true
      description: "Cultural communication style preference"
      llm_context: "Direct cultures prefer explicit CTAs; indirect cultures prefer suggestion"

    - name: hierarchy_importance
      type: string
      enum: ["high", "medium", "low"]
      required: true
      description: "Importance of social/business hierarchy"
      llm_context: "High hierarchy = formal titles; Low = first names acceptable"

    - name: individualism_level
      type: string
      enum: ["individualist", "collectivist", "mixed"]
      required: true
      description: "Individual vs collective orientation"
      llm_context: "Individualist = 'you' focus; Collectivist = 'we/together' focus"

    # TIER 2: STRUCTURED JSON (programmatic lookup)
    - name: seasons
      type: json
      required: true
      description: "Season definitions with month mappings and local names"
      example: '{"summer": {"months": [6,7,8], "name": "été"}, "winter": {"months": [12,1,2], "name": "hiver"}}'

    - name: holidays
      type: json
      required: true
      description: "Major holidays with dates and importance levels"
      example: '[{"key": "christmas", "date": "12-25", "importance": "major", "name": "Noël"}]'

    - name: business_hours
      type: json
      required: true
      description: "Typical business operating hours"
      example: '{"start": "09:00", "end": "18:00", "lunch_break": true, "lunch_start": "12:00", "lunch_end": "14:00"}'

    - name: values
      type: json
      required: true
      description: "Core cultural values as ordered list"
      example: '["liberté", "égalité", "fraternité"]'

    - name: communication_norms
      type: json
      required: true
      description: "Communication conventions for greetings, closings, etc."
      example: '{"greetings": "formal", "closings": "warm", "small_talk": "expected"}'

    # TIER 3: LLM CONTEXT (narrative text)
    - name: culture_summary
      type: text
      required: true
      description: "200-word narrative summary of cultural context for LLM"

    - name: taboos_summary
      type: text
      required: false
      description: "Brief summary of cultural taboos and sensitivities"

    - name: raw_markdown
      type: text
      required: false
      description: "Full source markdown for edge case reference"
```

**Step 2: Validate YAML syntax**

Run: `cargo run -- schema validate`
Expected: No errors for culture.yaml

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/config/culture.yaml
git commit -m "feat(schema): add Culture node for cultural context

3-tier structure: indexed scalars, structured JSON, LLM context.
Covers calendar, seasons, business norms, values, communication style.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.2: Create Market Node YAML

**Files:**
- Create: `packages/core/models/node-classes/global/config/market.yaml`

**Step 1: Write the YAML schema**

```yaml
node:
  name: Market
  realm: global
  layer: config
  trait: knowledge
  display_name: "Market"
  description: "Market intelligence for a locale including demographics, digital maturity, e-commerce landscape, and consumer behavior. Used for market-appropriate content strategy."
  llm_context: "Market data that influences urgency, value propositions, and platform references in generated content."

  properties:
    # TIER 1: INDEXED SCALARS (Cypher-queryable)
    - name: population_millions
      type: float
      required: true
      description: "Population in millions"

    - name: gdp_per_capita_usd
      type: integer
      required: false
      description: "GDP per capita in USD"

    - name: internet_penetration
      type: float
      required: true
      description: "Internet penetration rate (0.0-1.0)"

    - name: mobile_penetration
      type: float
      required: true
      description: "Mobile device penetration rate (0.0-1.0)"

    - name: ecommerce_maturity
      type: string
      enum: ["emerging", "growing", "mature", "advanced"]
      required: true
      description: "E-commerce market maturity level"
      llm_context: "Emerging = explain benefits; Advanced = assume familiarity"

    - name: digital_maturity_score
      type: integer
      required: true
      description: "Digital maturity score 0-100"

    # TIER 2: STRUCTURED JSON (programmatic lookup)
    - name: payment_methods
      type: json
      required: true
      description: "Payment method preferences with market share"
      example: '[{"method": "card", "share": 0.45, "providers": ["Visa", "Mastercard"]}]'

    - name: popular_platforms
      type: json
      required: true
      description: "Popular e-commerce platforms with market share"
      example: '[{"name": "Amazon", "market_share": 0.35, "category": "marketplace"}]'

    - name: social_networks
      type: json
      required: true
      description: "Social network penetration rates"
      example: '[{"name": "Instagram", "penetration": 0.65}, {"name": "TikTok", "penetration": 0.45}]'

    - name: demographics
      type: json
      required: true
      description: "Key demographic indicators"
      example: '{"median_age": 42, "urban_rate": 0.81, "literacy_rate": 0.99}'

    # TIER 3: LLM CONTEXT (narrative text)
    - name: market_summary
      type: text
      required: true
      description: "200-word narrative summary of market context for LLM"

    - name: buying_behavior
      type: text
      required: false
      description: "Consumer expectations and purchasing patterns"

    - name: raw_markdown
      type: text
      required: false
      description: "Full source markdown for edge case reference"
```

**Step 2: Validate YAML syntax**

Run: `cargo run -- schema validate`
Expected: No errors for market.yaml

**Step 3: Commit**

```bash
git add packages/core/models/node-classes/global/config/market.yaml
git commit -m "feat(schema): add Market node for market intelligence

3-tier structure: demographics, digital maturity, e-commerce, payments.
Enables market-appropriate content strategy.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.3: Update Style Node YAML (Add Formality)

**Files:**
- Modify: `packages/core/models/node-classes/global/config/style.yaml`

**Step 1: Read current style.yaml**

Run: `cat packages/core/models/node-classes/global/config/style.yaml`
Note the existing properties.

**Step 2: Add new properties from Adaptation migration**

Add these properties to the existing style.yaml:

```yaml
    # === ADDITIONS FROM ADAPTATION MIGRATION ===

    # TIER 1: INDEXED SCALARS
    - name: formality_default
      type: string
      enum: ["formal", "informal", "neutral"]
      required: true
      description: "Default formality level for this locale"
      llm_context: "Base formality unless context specifies otherwise"

    - name: pronoun_preference
      type: string
      enum: ["tu", "vous", "usted", "du", "sie", "mixed", "n_a"]
      required: false
      description: "Preferred pronoun form for direct address"
      llm_context: "Use this pronoun form when addressing the reader"

    - name: directness_level
      type: string
      enum: ["direct", "indirect", "balanced"]
      required: true
      description: "Communication directness preference"

    - name: warmth_level
      type: string
      enum: ["warm", "neutral", "reserved"]
      required: true
      description: "Emotional warmth in communication"

    # TIER 2: STRUCTURED JSON
    - name: formality_rules
      type: json
      required: false
      description: "Context-specific formality overrides"
      example: '{"b2b": "formal", "b2c_youth": "informal", "legal": "formal"}'

    - name: length_preferences
      type: json
      required: false
      description: "Preferred content lengths by type"
      example: '{"headline": 60, "subhead": 120, "body_paragraph": 150}'

    - name: seo_preferences
      type: json
      required: false
      description: "SEO-related style preferences"
      example: '{"keywords_per_100_words": 2, "heading_style": "question"}'

    - name: tone_modifiers
      type: json
      required: false
      description: "Tone adjustments by context"
      example: '{"urgency": "soft", "celebration": "expressive", "error": "apologetic"}'
```

**Step 3: Validate YAML syntax**

Run: `cargo run -- schema validate`
Expected: No errors

**Step 4: Commit**

```bash
git add packages/core/models/node-classes/global/config/style.yaml
git commit -m "feat(schema): expand Style with formality from Adaptation

Adds: formality_default, pronoun_preference, directness_level, warmth_level,
formality_rules, length_preferences, seo_preferences, tone_modifiers.
Consolidates all tone/style properties in one node.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.4: Simplify Adaptation Node YAML

**Files:**
- Modify: `packages/core/models/node-classes/global/config/adaptation.yaml`

**Step 1: Read current adaptation.yaml**

Run: `cat packages/core/models/node-classes/global/config/adaptation.yaml`
Identify properties to remove (migrated to Style/Culture).

**Step 2: Rewrite adaptation.yaml with simplified schema**

Replace the entire file with:

```yaml
node:
  name: Adaptation
  realm: global
  layer: config
  trait: knowledge
  display_name: "Adaptation"
  description: "Content adaptation rules for FACTS vs ILLUSTRATIONS classification and technical term handling. Core decision engine for what to translate literally vs generate natively."
  llm_context: "Use this to decide: Is this content a FACT (translate literally) or ILLUSTRATION (generate natively)? Also handles technical term preferences."

  properties:
    # TIER 1: INDEXED SCALARS (Cypher-queryable)
    - name: technical_terms_approach
      type: string
      enum: ["local_only", "english_accepted", "mixed"]
      required: true
      description: "How to handle technical/borrowed terms"
      llm_context: "local_only = always use local equivalent; english_accepted = English terms OK; mixed = depends on context"

    - name: illustration_density
      type: string
      enum: ["high", "medium", "low"]
      required: true
      description: "Preference for culturally-specific illustrations"
      llm_context: "high = maximize local references; low = prefer universal examples"

    # TIER 2: STRUCTURED JSON (programmatic lookup)
    - name: technical_terms_preferred
      type: json
      required: false
      description: "Mapping of English terms to preferred local equivalents"
      example: '{"computer": "ordinateur", "software": "logiciel", "download": "télécharger"}'

    - name: technical_terms_accepted
      type: json
      required: false
      description: "English terms acceptable without translation"
      example: '["smartphone", "email", "Wi-Fi", "QR code", "URL"]'

    - name: common_errors
      type: json
      required: false
      description: "Common adaptation errors to avoid"
      example: '[{"error": "using English word X", "correct": "use Y instead", "why": "perceived as lazy"}]'

    - name: false_friends
      type: json
      required: false
      description: "False cognates to watch for"
      example: '[{"word": "actually", "trap": "actuellement", "correct": "en fait"}]'

    - name: facts_classification
      type: json
      required: true
      description: "Classification rules for FACT vs ILLUSTRATION"
      example: '{"proper_names": "FACT", "brand_names": "FACT", "idioms": "ILLUSTRATION", "metaphors": "ILLUSTRATION", "statistics": "FACT", "cultural_references": "ILLUSTRATION"}'

    # TIER 3: LLM CONTEXT (narrative text)
    - name: adaptation_summary
      type: text
      required: true
      description: "200-word distilled summary always loaded in LLM context"

    - name: decision_algorithm
      type: text
      required: true
      description: "Step-by-step flowchart for FACT/ILLUSTRATION classification"

    - name: validation_checklist
      type: text
      required: false
      description: "QA checklist for adaptation review"

    - name: raw_markdown
      type: text
      required: false
      description: "Full source markdown for edge cases"
```

**Step 3: Validate YAML syntax**

Run: `cargo run -- schema validate`
Expected: No errors

**Step 4: Commit**

```bash
git add packages/core/models/node-classes/global/config/adaptation.yaml
git commit -m "refactor(schema): simplify Adaptation to core FACTS/ILLUSTRATIONS

Removes: formality (→ Style), calendar/seasons (→ Culture), measurement (→ Formatting).
Focuses on: technical terms, FACT/ILLUSTRATION classification, common errors.
3-tier structure maintained.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.5: Add Arc Definitions for New Nodes

**Files:**
- Create: `packages/core/models/arc-classes/ownership/has-culture.yaml`
- Create: `packages/core/models/arc-classes/ownership/has-market.yaml`

**Step 1: Create HAS_CULTURE arc**

```yaml
arc:
  name: HAS_CULTURE
  family: ownership
  scope: intra_realm
  cardinality: one_to_one
  source:
    kind: Locale
    required: true
  target:
    kind: Culture
    required: true
  description: "Links a Locale to its Culture configuration"
  llm_context: "Every locale has exactly one culture definition"
```

**Step 2: Create HAS_MARKET arc**

```yaml
arc:
  name: HAS_MARKET
  family: ownership
  scope: intra_realm
  cardinality: one_to_one
  source:
    kind: Locale
    required: true
  target:
    kind: Market
    required: true
  description: "Links a Locale to its Market intelligence"
  llm_context: "Every locale has exactly one market definition"
```

**Step 3: Validate**

Run: `cargo run -- schema validate`
Expected: No errors

**Step 4: Commit**

```bash
git add packages/core/models/arc-classes/ownership/has-culture.yaml
git add packages/core/models/arc-classes/ownership/has-market.yaml
git commit -m "feat(schema): add HAS_CULTURE and HAS_MARKET arcs

Links Locale to new Culture and Market nodes.
Both are 1:1 ownership relationships.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 1.6: Regenerate All Artifacts

**Files:**
- Generated: `packages/core/src/types/generated/*.ts`
- Generated: `packages/db/seed/*.cypher`

**Step 1: Run schema generator**

Run: `cargo run -- schema generate`
Expected: All generators complete successfully

**Step 2: Verify TypeScript types generated**

Run: `ls -la packages/core/src/types/generated/`
Expected: Updated type files with Culture, Market

**Step 3: Verify Cypher seeds generated**

Run: `ls -la packages/db/seed/`
Expected: Updated seed files

**Step 4: Run type-check**

Run: `pnpm type-check`
Expected: No TypeScript errors

**Step 5: Commit generated files**

```bash
git add packages/core/src/types/generated/
git add packages/db/seed/
git commit -m "chore: regenerate artifacts for 7-node architecture

Updates TypeScript types and Cypher seeds for:
- Culture node (NEW)
- Market node (NEW)
- Style (expanded)
- Adaptation (simplified)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 2: Rust Parsers

### Task 2.1: Create Culture Parser

**Files:**
- Create: `tools/novanet/src/parsers/culture.rs`
- Modify: `tools/novanet/src/parsers/mod.rs`

**Step 1: Write the failing test**

Add to `tools/novanet/src/parsers/culture.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_culture_markdown() {
        let input = r#"# Culture: fr-FR

## Hemisphere
Northern

## Work Week
Starts Monday

## Communication Style
- Directness: indirect
- Hierarchy: high
- Individualism: mixed

## Seasons
| Season | Months | Local Name |
|--------|--------|------------|
| Spring | 3,4,5 | printemps |
| Summer | 6,7,8 | été |

## Values
1. Liberté
2. Égalité
3. Fraternité
"#;

        let result = parse_culture_markdown(input, "fr-FR").unwrap();

        assert_eq!(result.hemisphere, "northern");
        assert_eq!(result.work_week_start, "monday");
        assert_eq!(result.communication_directness, "indirect");
        assert_eq!(result.hierarchy_importance, "high");
        assert_eq!(result.values.len(), 3);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_parse_culture_markdown -- --nocapture`
Expected: FAIL with "cannot find function `parse_culture_markdown`"

**Step 3: Write minimal implementation**

```rust
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CultureData {
    pub locale_key: String,
    pub hemisphere: String,
    pub work_week_start: String,
    pub communication_directness: String,
    pub hierarchy_importance: String,
    pub individualism_level: String,
    pub seasons: serde_json::Value,
    pub holidays: serde_json::Value,
    pub business_hours: serde_json::Value,
    pub values: Vec<String>,
    pub communication_norms: serde_json::Value,
    pub culture_summary: String,
    pub taboos_summary: Option<String>,
    pub raw_markdown: String,
}

pub fn parse_culture_markdown(content: &str, locale_key: &str) -> Result<CultureData> {
    let mut data = CultureData {
        locale_key: locale_key.to_string(),
        hemisphere: String::new(),
        work_week_start: String::new(),
        communication_directness: String::new(),
        hierarchy_importance: String::new(),
        individualism_level: String::new(),
        seasons: serde_json::json!({}),
        holidays: serde_json::json!([]),
        business_hours: serde_json::json!({}),
        values: Vec::new(),
        communication_norms: serde_json::json!({}),
        culture_summary: String::new(),
        taboos_summary: None,
        raw_markdown: content.to_string(),
    };

    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("## ") {
            current_section = line[3..].to_lowercase();
            continue;
        }

        match current_section.as_str() {
            "hemisphere" => {
                if !line.is_empty() && !line.starts_with('#') {
                    data.hemisphere = line.to_lowercase();
                }
            }
            "work week" => {
                if line.to_lowercase().contains("monday") {
                    data.work_week_start = "monday".to_string();
                } else if line.to_lowercase().contains("sunday") {
                    data.work_week_start = "sunday".to_string();
                } else if line.to_lowercase().contains("saturday") {
                    data.work_week_start = "saturday".to_string();
                }
            }
            "communication style" => {
                if line.starts_with("- Directness:") {
                    data.communication_directness = line.split(':').nth(1)
                        .map(|s| s.trim().to_lowercase())
                        .unwrap_or_default();
                } else if line.starts_with("- Hierarchy:") {
                    data.hierarchy_importance = line.split(':').nth(1)
                        .map(|s| s.trim().to_lowercase())
                        .unwrap_or_default();
                } else if line.starts_with("- Individualism:") {
                    data.individualism_level = line.split(':').nth(1)
                        .map(|s| s.trim().to_lowercase())
                        .unwrap_or_default();
                }
            }
            "values" => {
                if line.starts_with(|c: char| c.is_ascii_digit()) {
                    if let Some(value) = line.split('.').nth(1) {
                        data.values.push(value.trim().to_string());
                    }
                }
            }
            _ => {}
        }
    }

    Ok(data)
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_parse_culture_markdown -- --nocapture`
Expected: PASS

**Step 5: Add to mod.rs**

Add to `tools/novanet/src/parsers/mod.rs`:
```rust
pub mod culture;
pub use culture::*;
```

**Step 6: Commit**

```bash
git add tools/novanet/src/parsers/culture.rs
git add tools/novanet/src/parsers/mod.rs
git commit -m "feat(parser): add Culture markdown parser

Parses 4-culture-norms and 2-rules-adaptation calendar data.
Extracts: hemisphere, work week, communication style, values.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2.2: Create Market Parser

**Files:**
- Create: `tools/novanet/src/parsers/market.rs`
- Modify: `tools/novanet/src/parsers/mod.rs`

**Step 1: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_market_markdown() {
        let input = r#"# Market: fr-FR

## Demographics
- Population: 67.5 million
- Median Age: 42
- Urban Rate: 81%

## Digital Maturity
- Internet Penetration: 92%
- Mobile Penetration: 98%
- E-commerce Maturity: mature
- Digital Score: 78/100

## Payment Methods
| Method | Share | Providers |
|--------|-------|-----------|
| Card | 45% | Visa, Mastercard |
| Bank Transfer | 25% | SEPA |
"#;

        let result = parse_market_markdown(input, "fr-FR").unwrap();

        assert!((result.population_millions - 67.5).abs() < 0.1);
        assert!((result.internet_penetration - 0.92).abs() < 0.01);
        assert_eq!(result.ecommerce_maturity, "mature");
        assert_eq!(result.digital_maturity_score, 78);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_parse_market_markdown -- --nocapture`
Expected: FAIL

**Step 3: Write minimal implementation**

```rust
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub locale_key: String,
    pub population_millions: f64,
    pub gdp_per_capita_usd: Option<i32>,
    pub internet_penetration: f64,
    pub mobile_penetration: f64,
    pub ecommerce_maturity: String,
    pub digital_maturity_score: i32,
    pub payment_methods: serde_json::Value,
    pub popular_platforms: serde_json::Value,
    pub social_networks: serde_json::Value,
    pub demographics: serde_json::Value,
    pub market_summary: String,
    pub buying_behavior: Option<String>,
    pub raw_markdown: String,
}

pub fn parse_market_markdown(content: &str, locale_key: &str) -> Result<MarketData> {
    let mut data = MarketData {
        locale_key: locale_key.to_string(),
        population_millions: 0.0,
        gdp_per_capita_usd: None,
        internet_penetration: 0.0,
        mobile_penetration: 0.0,
        ecommerce_maturity: "growing".to_string(),
        digital_maturity_score: 0,
        payment_methods: serde_json::json!([]),
        popular_platforms: serde_json::json!([]),
        social_networks: serde_json::json!([]),
        demographics: serde_json::json!({}),
        market_summary: String::new(),
        buying_behavior: None,
        raw_markdown: content.to_string(),
    };

    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("## ") {
            current_section = line[3..].to_lowercase();
            continue;
        }

        match current_section.as_str() {
            "demographics" => {
                if line.starts_with("- Population:") {
                    if let Some(pop_str) = line.split(':').nth(1) {
                        let cleaned = pop_str.trim()
                            .replace("million", "")
                            .replace(" ", "")
                            .trim()
                            .to_string();
                        data.population_millions = cleaned.parse().unwrap_or(0.0);
                    }
                }
            }
            "digital maturity" => {
                if line.starts_with("- Internet Penetration:") {
                    if let Some(pct) = extract_percentage(line) {
                        data.internet_penetration = pct;
                    }
                } else if line.starts_with("- Mobile Penetration:") {
                    if let Some(pct) = extract_percentage(line) {
                        data.mobile_penetration = pct;
                    }
                } else if line.starts_with("- E-commerce Maturity:") {
                    data.ecommerce_maturity = line.split(':').nth(1)
                        .map(|s| s.trim().to_lowercase())
                        .unwrap_or_default();
                } else if line.starts_with("- Digital Score:") {
                    if let Some(score_str) = line.split(':').nth(1) {
                        let cleaned = score_str.trim().split('/').next().unwrap_or("0");
                        data.digital_maturity_score = cleaned.parse().unwrap_or(0);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(data)
}

fn extract_percentage(line: &str) -> Option<f64> {
    line.split(':').nth(1)
        .and_then(|s| s.trim().strip_suffix('%'))
        .and_then(|s| s.parse::<f64>().ok())
        .map(|p| p / 100.0)
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_parse_market_markdown -- --nocapture`
Expected: PASS

**Step 5: Add to mod.rs and commit**

```bash
git add tools/novanet/src/parsers/market.rs
git add tools/novanet/src/parsers/mod.rs
git commit -m "feat(parser): add Market markdown parser

Parses 5-market data for demographics and digital maturity.
Extracts: population, internet/mobile penetration, e-commerce maturity.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2.3: Create Expression Parser (for 3-voice-lexicon)

**Files:**
- Create: `tools/novanet/src/parsers/expression.rs`
- Modify: `tools/novanet/src/parsers/mod.rs`

**Step 1: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression_markdown() {
        let input = r#"# Lexicon: fr-FR

## SUCCESS (Succès)
| Expression | Register | Intensity | Context | Example |
|------------|----------|-----------|---------|---------|
| C'est gagné ! | informal | high | celebration | Vous avez réussi ! C'est gagné ! |
| Mission accomplie | neutral | medium | completion | Notre mission accomplie avec brio |

## SPEED (Rapidité)
| Expression | Register | Intensity | Context | Example |
|------------|----------|-----------|---------|---------|
| En un clin d'œil | neutral | high | instant | Créez votre QR code en un clin d'œil |
"#;

        let result = parse_expression_markdown(input, "fr-FR").unwrap();

        assert_eq!(result.locale_key, "fr-FR");
        assert_eq!(result.semantic_fields.len(), 2);
        assert_eq!(result.semantic_fields[0].name, "SUCCESS");
        assert_eq!(result.semantic_fields[0].expressions.len(), 2);
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_parse_expression_markdown -- --nocapture`
Expected: FAIL

**Step 3: Write minimal implementation**

```rust
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpressionData {
    pub locale_key: String,
    pub semantic_fields: Vec<SemanticField>,
    pub raw_markdown: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticField {
    pub name: String,
    pub local_name: String,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Expression {
    pub text: String,
    pub register: String,
    pub intensity: String,
    pub context: String,
    pub example: String,
}

pub fn parse_expression_markdown(content: &str, locale_key: &str) -> Result<ExpressionData> {
    let mut data = ExpressionData {
        locale_key: locale_key.to_string(),
        semantic_fields: Vec::new(),
        raw_markdown: content.to_string(),
    };

    let mut current_field: Option<SemanticField> = None;
    let mut in_table = false;

    for line in content.lines() {
        let line = line.trim();

        // New semantic field header: ## FIELD_NAME (Local Name)
        if line.starts_with("## ") {
            // Save previous field if exists
            if let Some(field) = current_field.take() {
                data.semantic_fields.push(field);
            }

            let header = &line[3..];
            let (name, local_name) = if let Some(paren_start) = header.find('(') {
                let name = header[..paren_start].trim().to_string();
                let local = header[paren_start+1..].trim_end_matches(')').to_string();
                (name, local)
            } else {
                (header.to_string(), header.to_string())
            };

            current_field = Some(SemanticField {
                name,
                local_name,
                expressions: Vec::new(),
            });
            in_table = false;
            continue;
        }

        // Skip table header row
        if line.starts_with("| Expression") || line.starts_with("|---") {
            in_table = true;
            continue;
        }

        // Parse table data row
        if in_table && line.starts_with('|') && !line.starts_with("|---") {
            if let Some(ref mut field) = current_field {
                let cols: Vec<&str> = line.split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                if cols.len() >= 5 {
                    field.expressions.push(Expression {
                        text: cols[0].to_string(),
                        register: cols[1].to_string(),
                        intensity: cols[2].to_string(),
                        context: cols[3].to_string(),
                        example: cols[4].to_string(),
                    });
                }
            }
        }
    }

    // Save last field
    if let Some(field) = current_field {
        data.semantic_fields.push(field);
    }

    Ok(data)
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_parse_expression_markdown -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet/src/parsers/expression.rs
git add tools/novanet/src/parsers/mod.rs
git commit -m "feat(parser): add Expression markdown parser

Parses 3-voice-lexicon data with 10 semantic fields.
Extracts: expressions with register, intensity, context, example.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 2.4: Update Adaptation Parser for Simplified Schema

**Files:**
- Modify: `tools/novanet/src/parsers/adaptation.rs`

**Step 1: Read current parser**

Run: `cat tools/novanet/src/parsers/adaptation.rs`
Identify what needs to change for simplified schema.

**Step 2: Update struct and parser for 3-tier structure**

Focus on FACTS/ILLUSTRATIONS classification and technical terms.
Remove calendar, formality, measurement parsing (moved to other nodes).

**Step 3: Run existing tests**

Run: `cargo test adaptation -- --nocapture`
Expected: Tests may fail if they expect old properties

**Step 4: Update tests for new schema**

Adjust test expectations to match simplified schema.

**Step 5: Run tests to verify they pass**

Run: `cargo test adaptation -- --nocapture`
Expected: PASS

**Step 6: Commit**

```bash
git add tools/novanet/src/parsers/adaptation.rs
git commit -m "refactor(parser): simplify Adaptation parser for new schema

Removes: formality, calendar, measurement (migrated to Style/Culture).
Focuses: FACTS/ILLUSTRATIONS, technical terms, common errors.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

## Phase 3: Cypher Generators

### Task 3.1: Create Culture Cypher Generator

**Files:**
- Create: `tools/novanet/src/generators/culture_seed.rs`
- Modify: `tools/novanet/src/generators/mod.rs`

**Step 1: Write the failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_culture_cypher() {
        let data = CultureData {
            locale_key: "fr-FR".to_string(),
            hemisphere: "northern".to_string(),
            work_week_start: "monday".to_string(),
            // ... other fields
        };

        let cypher = generate_culture_cypher(&data);

        assert!(cypher.contains("MERGE (c:Culture {locale_key: 'fr-FR'})"));
        assert!(cypher.contains("hemisphere: 'northern'"));
        assert!(cypher.contains("MATCH (l:Locale {key: 'fr-FR'})"));
        assert!(cypher.contains("MERGE (l)-[:HAS_CULTURE]->(c)"));
    }
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test test_generate_culture_cypher -- --nocapture`
Expected: FAIL

**Step 3: Write minimal implementation**

```rust
use crate::parsers::CultureData;

pub fn generate_culture_cypher(data: &CultureData) -> String {
    format!(r#"
// Culture for {locale_key}
MERGE (c:Culture {{locale_key: '{locale_key}'}})
SET c.hemisphere = '{hemisphere}',
    c.work_week_start = '{work_week_start}',
    c.communication_directness = '{communication_directness}',
    c.hierarchy_importance = '{hierarchy_importance}',
    c.individualism_level = '{individualism_level}',
    c.seasons = '{seasons}',
    c.holidays = '{holidays}',
    c.business_hours = '{business_hours}',
    c.values = '{values}',
    c.communication_norms = '{communication_norms}',
    c.culture_summary = '{culture_summary}',
    c.raw_markdown = '{raw_markdown}';

MATCH (l:Locale {{key: '{locale_key}'}})
MERGE (l)-[:HAS_CULTURE]->(c);
"#,
        locale_key = data.locale_key,
        hemisphere = data.hemisphere,
        work_week_start = data.work_week_start,
        communication_directness = data.communication_directness,
        hierarchy_importance = data.hierarchy_importance,
        individualism_level = data.individualism_level,
        seasons = escape_json(&data.seasons),
        holidays = escape_json(&data.holidays),
        business_hours = escape_json(&data.business_hours),
        values = escape_json(&serde_json::json!(data.values)),
        communication_norms = escape_json(&data.communication_norms),
        culture_summary = escape_cypher(&data.culture_summary),
        raw_markdown = escape_cypher(&data.raw_markdown),
    )
}

fn escape_cypher(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('\'', "\\'")
     .replace('\n', "\\n")
}

fn escape_json(v: &serde_json::Value) -> String {
    escape_cypher(&v.to_string())
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test test_generate_culture_cypher -- --nocapture`
Expected: PASS

**Step 5: Commit**

```bash
git add tools/novanet/src/generators/culture_seed.rs
git add tools/novanet/src/generators/mod.rs
git commit -m "feat(generator): add Culture Cypher seed generator

Generates MERGE statements for Culture nodes.
Creates HAS_CULTURE arc from Locale.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 3.2: Create Market Cypher Generator

**Files:**
- Create: `tools/novanet/src/generators/market_seed.rs`
- Modify: `tools/novanet/src/generators/mod.rs`

Follow same pattern as Task 3.1 for Market node.

---

### Task 3.3: Create Expression Atoms Cypher Generator

**Files:**
- Create: `tools/novanet/src/generators/expression_seed.rs`

This generates ExpressionSet containers and Expression atoms:

```rust
pub fn generate_expression_cypher(data: &ExpressionData) -> String {
    let mut cypher = String::new();

    for field in &data.semantic_fields {
        // Create ExpressionSet container
        cypher.push_str(&format!(r#"
MERGE (es:ExpressionSet {{locale_key: '{locale}', semantic_field: '{field}'}})
SET es.local_name = '{local_name}';

MATCH (l:Locale {{key: '{locale}'}})
MERGE (l)-[:HAS_EXPRESSIONS]->(es);
"#,
            locale = data.locale_key,
            field = field.name,
            local_name = escape_cypher(&field.local_name),
        ));

        // Create Expression atoms
        for (i, expr) in field.expressions.iter().enumerate() {
            cypher.push_str(&format!(r#"
MERGE (e:Expression {{locale_key: '{locale}', semantic_field: '{field}', idx: {idx}}})
SET e.text = '{text}',
    e.register = '{register}',
    e.intensity = '{intensity}',
    e.context = '{context}',
    e.example = '{example}';

MATCH (es:ExpressionSet {{locale_key: '{locale}', semantic_field: '{field}'}})
MERGE (es)-[:CONTAINS]->(e);
"#,
                locale = data.locale_key,
                field = field.name,
                idx = i,
                text = escape_cypher(&expr.text),
                register = expr.register,
                intensity = expr.intensity,
                context = escape_cypher(&expr.context),
                example = escape_cypher(&expr.example),
            ));
        }
    }

    cypher
}
```

---

## Phase 4: Data Migration Commands

### Task 4.1: Add `migrate culture` Command

**Files:**
- Create: `tools/novanet/src/commands/migrate/culture.rs`
- Modify: `tools/novanet/src/commands/mod.rs`

**Step 1: Write the command structure**

```rust
use clap::Args;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Args)]
pub struct CultureMigrateArgs {
    /// Source directory containing culture markdown files
    #[arg(long, default_value = "/Users/thibaut/Projects/traduction_ai/ath-know-l10n/outputs/localization-data/4-culture-norms")]
    source: PathBuf,

    /// Output directory for generated Cypher
    #[arg(long, default_value = "packages/db/seed/culture")]
    output: PathBuf,

    /// Specific locale to migrate (optional, migrates all if not specified)
    #[arg(long)]
    locale: Option<String>,
}

pub async fn run(args: CultureMigrateArgs) -> Result<()> {
    println!("Migrating culture data from {:?}", args.source);

    let files = std::fs::read_dir(&args.source)?;

    for entry in files {
        let path = entry?.path();
        if path.extension().map_or(false, |e| e == "md") {
            let locale_key = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            if let Some(ref filter) = args.locale {
                if locale_key != filter {
                    continue;
                }
            }

            let content = std::fs::read_to_string(&path)?;
            let data = crate::parsers::parse_culture_markdown(&content, locale_key)?;
            let cypher = crate::generators::generate_culture_cypher(&data);

            let output_path = args.output.join(format!("{}.cypher", locale_key));
            std::fs::write(&output_path, cypher)?;

            println!("  ✓ {}", locale_key);
        }
    }

    println!("Migration complete!");
    Ok(())
}
```

**Step 2: Register command in CLI**

Add to main CLI:
```rust
#[derive(Subcommand)]
enum MigrateCommands {
    Culture(migrate::CultureMigrateArgs),
    Market(migrate::MarketMigrateArgs),
    Expression(migrate::ExpressionMigrateArgs),
    // ...
}
```

**Step 3: Test the command**

Run: `cargo run -- migrate culture --locale=fr-FR`
Expected: Generates `packages/db/seed/culture/fr-FR.cypher`

**Step 4: Commit**

```bash
git add tools/novanet/src/commands/migrate/
git commit -m "feat(cli): add migrate culture command

Parses 4-culture-norms markdown and generates Cypher seeds.
Usage: novanet migrate culture [--locale=fr-FR]

Co-Authored-By: Claude <noreply@anthropic.com>"
```

---

### Task 4.2: Add `migrate market` Command

Follow same pattern as Task 4.1 for Market data from `5-market/`.

---

### Task 4.3: Add `migrate expression` Command

Follow same pattern for Expression data from `3-voice-lexicon/`.

---

### Task 4.4: Add `migrate adaptation` Command

Update existing adaptation migration for simplified schema.

---

### Task 4.5: Add `migrate style` Command

Merge data from `3-voice-style/` + formality from `2-rules-adaptation/`.

---

## Phase 5: Seed and Verify

### Task 5.1: Run Full Migration

**Step 1: Run all migrations**

```bash
cargo run -- migrate culture
cargo run -- migrate market
cargo run -- migrate expression
cargo run -- migrate adaptation
cargo run -- migrate style
```

**Step 2: Seed the database**

```bash
pnpm infra:seed
```

**Step 3: Verify node counts**

Run: `cargo run -- data --format=json | jq '.nodes | group_by(.label) | map({label: .[0].label, count: length})'`

Expected counts:
- Locale: 200
- Formatting: 200
- Slugification: 200
- Adaptation: 200
- Style: 200
- Culture: 200 (NEW)
- Market: 200 (NEW)
- ExpressionSet: ~2000 (10 fields × 200 locales)
- Expression: ~16000 (80 per locale × 200)

---

### Task 5.2: Run All Tests

**Step 1: Rust tests**

Run: `cargo test`
Expected: All tests pass

**Step 2: TypeScript tests**

Run: `pnpm test`
Expected: All tests pass

**Step 3: Type check**

Run: `pnpm type-check`
Expected: No errors

**Step 4: Lint**

Run: `pnpm lint`
Expected: No errors

---

### Task 5.3: Final Commit and Tag

**Step 1: Final commit**

```bash
git add .
git commit -m "feat: complete 7-node locale knowledge architecture

- Culture node (NEW): calendar, seasons, values, communication style
- Market node (NEW): demographics, digital maturity, e-commerce
- Style (expanded): formality, pronouns, directness, warmth
- Adaptation (simplified): FACTS/ILLUSTRATIONS, technical terms
- Expression atoms: 10 semantic fields, 80-90 expressions per locale

Migration complete for 200 locales.

Co-Authored-By: Claude <noreply@anthropic.com>"
```

**Step 2: Update CHANGELOG.md**

Add entry for new version with all changes.

**Step 3: Tag release**

```bash
git tag -a v10.7.0 -m "v10.7.0: 7-Node Locale Knowledge Architecture"
git push origin main --tags
```

---

## Summary

| Phase | Tasks | Description |
|-------|-------|-------------|
| **1** | 1.1-1.6 | Schema Updates (YAML + arcs + regenerate) |
| **2** | 2.1-2.4 | Rust Parsers (Culture, Market, Expression, Adaptation) |
| **3** | 3.1-3.3 | Cypher Generators |
| **4** | 4.1-4.5 | Migration Commands |
| **5** | 5.1-5.3 | Seed, Verify, Release |

**Total:** 18 tasks, ~50 steps

---

**Plan complete and saved to `docs/plans/2026-02-06-locale-knowledge-7node-architecture.md`.**
