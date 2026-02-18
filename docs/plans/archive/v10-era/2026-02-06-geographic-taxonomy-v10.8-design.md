# Geographic Taxonomy Design v10.8

**Status**: Draft
**Author**: Claude + Thibaut
**Date**: 2026-02-06
**Version**: v10.8.0

## Executive Summary

Refactor NovaNet's geographic architecture from **string properties** on Locale nodes to a **full graph-native taxonomy** with traversable nodes for geographic, cultural, economic, and linguistic classification.

### Goals

1. **LLM Retrieval** — Find similar locales via graph traversal
2. **Data Modeling** — Attach metadata to geographic entities (timezone, currency, etc.)
3. **UI Navigation** — Hierarchical pickers in Studio

---

## Research Sources

| Source | Data |
|--------|------|
| UN M49 | Continents, Regions, SubRegions (unstats.un.org) |
| ISO 639-5 | Language Families (loc.gov) |
| World Bank | Income Groups, Lending Categories |
| Broek & Webb | Cultural Realms (6 major + sub-realms) |
| REST Countries API | Region/SubRegion data per country |
| dr5hn/countrystatecity | 223 code snippets, comprehensive geo data |
| biter777/countries | Go lib with UN M49 region codes |
| annexare/countries | JSON with continents, countries, languages |

---

## Part 1: Node Types (14 new nodes)

### 1.1 Geographic Hierarchy (UN M49)

```yaml
# Hierarchy: Continent > Region > SubRegion > Country (implicit via Locale)

# Node: Continent (6 nodes)
node:
  name: Continent
  realm: global
  layer: config
  trait: invariant
  description: "UN M49 continental region"
  properties:
    key: { type: string, pattern: "^[A-Z]{2}$", examples: ["AF", "EU", "AS"] }
    name: { type: string, examples: ["Africa", "Europe", "Asia"] }
    m49_code: { type: integer, examples: [2, 150, 142] }
    llm_context: { type: string }

# Node: GeoRegion (22 nodes - UN M49 regions)
node:
  name: GeoRegion
  realm: global
  layer: config
  trait: invariant
  description: "UN M49 geographic region"
  properties:
    key: { type: string, examples: ["northern-africa", "western-europe"] }
    name: { type: string, examples: ["Northern Africa", "Western Europe"] }
    m49_code: { type: integer, examples: [15, 155] }
    llm_context: { type: string }

# Node: GeoSubRegion (optional intermediate regions)
node:
  name: GeoSubRegion
  realm: global
  layer: config
  trait: invariant
  description: "UN M49 intermediate/sub-region"
  properties:
    key: { type: string, examples: ["channel-islands", "caribbean"] }
    name: { type: string }
    m49_code: { type: integer }
    llm_context: { type: string }
```

**Data (UN M49):**

| Continent | Code | M49 | Regions |
|-----------|------|-----|---------|
| Africa | AF | 002 | Northern, Eastern, Middle, Southern, Western |
| Americas | AM | 019 | Northern, Caribbean, Central, South |
| Asia | AS | 142 | Central, Eastern, South-eastern, Southern, Western |
| Europe | EU | 150 | Eastern, Northern, Southern, Western |
| Oceania | OC | 009 | Australia/NZ, Melanesia, Micronesia, Polynesia |
| Antarctica | AN | 010 | - |

### 1.2 Language Families (ISO 639-5)

```yaml
# Node: LanguageFamily (top-level families)
node:
  name: LanguageFamily
  realm: global
  layer: locale-knowledge
  trait: knowledge
  description: "ISO 639-5 language family"
  properties:
    key: { type: string, pattern: "^[a-z]{3}$", examples: ["ine", "sit", "afa"] }
    name: { type: string, examples: ["Indo-European", "Sino-Tibetan"] }
    name_french: { type: string }
    llm_context: { type: string }

# Node: LanguageBranch (sub-families)
node:
  name: LanguageBranch
  realm: global
  layer: locale-knowledge
  trait: knowledge
  description: "ISO 639-5 language branch/group"
  properties:
    key: { type: string, examples: ["roa", "gem", "sla", "sem"] }
    name: { type: string, examples: ["Romance", "Germanic", "Slavic"] }
    parent_family: { type: string }  # Reference to LanguageFamily.key
    llm_context: { type: string }
```

**Data (ISO 639-5 Major Families):**

| Family | Code | Branches |
|--------|------|----------|
| Indo-European | ine | Romance (roa), Germanic (gem), Slavic (sla), Celtic (cel), Indo-Iranian (iir) |
| Sino-Tibetan | sit | Chinese (zhx), Tibeto-Burman (tbq) |
| Afro-Asiatic | afa | Semitic (sem), Berber (ber), Cushitic (cus) |
| Austronesian | map | Malayo-Polynesian (poz), Philippine (phi) |
| Dravidian | dra | - |
| Uralic | urj | Finno-Ugric (fiu), Samoyedic (syd) |
| Altaic | tut | Turkic (trk), Mongolic (xgn), Tungusic (tuw) |
| Japonic | jpx | - |
| Koreanic | kor | - |

### 1.3 Cultural Realms (Broek & Webb)

```yaml
# Node: CulturalRealm (6 major realms)
node:
  name: CulturalRealm
  realm: global
  layer: locale-knowledge
  trait: knowledge
  description: "Major cultural realm (Broek & Webb classification)"
  properties:
    key: { type: string, examples: ["occidental", "islamic", "indic", "east-asian"] }
    name: { type: string }
    description: { type: string }
    primary_religion: { type: string }
    characteristics: { type: "string[]" }
    llm_context: { type: string }

# Node: CulturalSubRealm (sub-divisions)
node:
  name: CulturalSubRealm
  realm: global
  layer: locale-knowledge
  trait: knowledge
  description: "Cultural sub-realm within a major realm"
  properties:
    key: { type: string, examples: ["west-european", "mediterranean", "anglo-american"] }
    name: { type: string }
    parent_realm: { type: string }
    llm_context: { type: string }
```

**Data (Cultural Realms):**

| Realm | Key | Sub-Realms |
|-------|-----|------------|
| Occidental | occidental | West European, Continental European, Mediterranean, Anglo-American, Australian, Latin American |
| Islamic | islamic | Arab, Persian, Turkic, South Asian Muslim, Southeast Asian Muslim |
| Indic | indic | Hindu, Buddhist (South Asian) |
| East Asian | east-asian | Sinosphere (China), Japanese, Korean |
| Southeast Asian | southeast-asian | Buddhist (Myanmar, Thailand, Vietnam), Malay (Malaysia, Indonesia) |
| Meso-African | meso-african | Sub-Saharan, Nilotic, Bantu |

### 1.4 Economic Classification (World Bank)

```yaml
# Node: IncomeGroup (4 groups)
node:
  name: IncomeGroup
  realm: global
  layer: config
  trait: invariant
  description: "World Bank income classification"
  properties:
    key: { type: string, enum: ["lic", "lmic", "umic", "hic"] }
    name: { type: string, examples: ["Low income", "High income"] }
    gni_threshold_min: { type: integer }  # GNI per capita USD
    gni_threshold_max: { type: integer }
    fiscal_year: { type: string, examples: ["FY2026"] }
    llm_context: { type: string }

# Node: LendingCategory (3 categories)
node:
  name: LendingCategory
  realm: global
  layer: config
  trait: invariant
  description: "World Bank lending classification"
  properties:
    key: { type: string, enum: ["ida", "blend", "ibrd"] }
    name: { type: string }
    description: { type: string }
    llm_context: { type: string }

# Node: EconomicRegion (7 WB regions)
node:
  name: EconomicRegion
  realm: global
  layer: config
  trait: invariant
  description: "World Bank economic region"
  properties:
    key: { type: string, examples: ["eap", "eca", "lac", "mena", "na", "sa", "ssa"] }
    name: { type: string }
    llm_context: { type: string }
```

**Data (World Bank FY2026):**

| Income Group | Key | GNI Threshold (USD) |
|--------------|-----|---------------------|
| Low income | lic | ≤ $1,135 |
| Lower-middle income | lmic | $1,136 - $4,495 |
| Upper-middle income | umic | $4,496 - $13,935 |
| High income | hic | > $13,935 |

### 1.5 Population Clusters (Image Generation)

```yaml
# Node: PopulationCluster (7 macro clusters)
node:
  name: PopulationCluster
  realm: global
  layer: locale-knowledge
  trait: knowledge
  description: "Macro population cluster for image generation diversity"
  properties:
    key: { type: string, examples: ["west-eurasian", "east-asian", "sub-saharan"] }
    name: { type: string, examples: ["West Eurasian", "East Asian"] }
    visual_traits: { type: "string[]", description: "Physical traits for image prompts" }
    llm_context: { type: string }

# Node: PopulationSubCluster (~25 sub-clusters)
node:
  name: PopulationSubCluster
  realm: global
  layer: locale-knowledge
  trait: knowledge
  description: "Sub-cluster population group for fine-grained image diversity"
  properties:
    key: { type: string, examples: ["northern-european", "mediterranean", "han-chinese"] }
    name: { type: string, examples: ["Northern European", "Mediterranean", "Han Chinese"] }
    parent_cluster: { type: string }  # Reference to PopulationCluster.key
    visual_traits: { type: "string[]", description: "Specific physical traits for image prompts" }
    skin_tone_range: { type: string, examples: ["very light to light", "olive to light brown"] }
    hair_colors: { type: "string[]", examples: ["blonde", "brown", "black"] }
    eye_colors: { type: "string[]", examples: ["blue", "green", "brown"] }
    llm_context: { type: string }
```

**Data (Population Clusters):**

| Cluster | Key | Sub-Clusters |
|---------|-----|--------------|
| West Eurasian | west-eurasian | Northern European, Mediterranean, Middle Eastern, South Asian (light), Central Asian |
| East Asian | east-asian | Han Chinese, Japanese, Korean, Mongolian |
| Southeast Asian | southeast-asian | Malay, Filipino, Vietnamese, Thai |
| South Asian | south-asian | North Indian, South Indian, Bengali, Sri Lankan |
| Sub-Saharan African | sub-saharan | West African, East African, Central African, Southern African |
| Native American | native-american | North American Indigenous, Mesoamerican, South American Indigenous |
| Oceanian | oceanian | Australian Aboriginal, Melanesian, Polynesian |

**Visual Traits Example (Northern European):**
```yaml
visual_traits:
  - "light to very light skin tone"
  - "blonde to light brown hair"
  - "blue, green, or grey eyes"
  - "tall stature"
skin_tone_range: "very light to light"
hair_colors: ["blonde", "light brown", "red"]
eye_colors: ["blue", "green", "grey", "light brown"]
llm_context: "USE: for people imagery in Nordic/Germanic markets. TRIGGERS: Scandinavia, Germany, Netherlands, UK. NOT: Mediterranean, Slavic."
```

**Use Case (QR Code AI Image Generation):**
```
When generating images with people for fr-FR market:
1. Retrieve Locale.fr-FR → HAS_POPULATION → PopulationSubCluster
2. Get visual_traits from PopulationSubCluster nodes
3. Build image prompt: "professional French person, {visual_traits}"
4. Generate diverse representation weighted by population percentages
```

---

## Part 2: Arc Types (15 new arcs)

### 2.1 Geographic Hierarchy Arcs

```yaml
# Locale → GeoSubRegion (or GeoRegion if no subregion)
arc:
  name: IN_SUBREGION
  family: localization
  source: Locale
  target: GeoSubRegion
  cardinality: N:1
  scope: cross_realm  # Locale is tenant, GeoSubRegion is global
  description: "Locale belongs to geographic subregion"

# GeoSubRegion → GeoRegion
arc:
  name: IN_REGION
  family: ownership
  source: GeoSubRegion
  target: GeoRegion
  cardinality: N:1
  scope: intra_realm

# GeoRegion → Continent
arc:
  name: IN_CONTINENT
  family: ownership
  source: GeoRegion
  target: Continent
  cardinality: N:1
  scope: intra_realm
```

### 2.2 Language Family Arcs

```yaml
# Locale → LanguageBranch
arc:
  name: SPEAKS_BRANCH
  family: localization
  source: Locale
  target: LanguageBranch
  cardinality: N:1
  scope: cross_realm
  description: "Primary language branch of locale"

# LanguageBranch → LanguageFamily
arc:
  name: BRANCH_OF
  family: ownership
  source: LanguageBranch
  target: LanguageFamily
  cardinality: N:1
  scope: intra_realm
```

### 2.3 Cultural Realm Arcs

```yaml
# Locale → CulturalSubRealm
arc:
  name: IN_CULTURAL_SUBREALM
  family: localization
  source: Locale
  target: CulturalSubRealm
  cardinality: N:1
  scope: cross_realm

# CulturalSubRealm → CulturalRealm
arc:
  name: PART_OF_REALM
  family: ownership
  source: CulturalSubRealm
  target: CulturalRealm
  cardinality: N:1
  scope: intra_realm
```

### 2.4 Economic Classification Arcs

```yaml
# Locale → IncomeGroup
arc:
  name: HAS_INCOME_LEVEL
  family: localization
  source: Locale
  target: IncomeGroup
  cardinality: N:1
  scope: cross_realm

# Locale → LendingCategory
arc:
  name: HAS_LENDING_TYPE
  family: localization
  source: Locale
  target: LendingCategory
  cardinality: N:1
  scope: cross_realm

# Locale → EconomicRegion
arc:
  name: IN_ECONOMIC_REGION
  family: localization
  source: Locale
  target: EconomicRegion
  cardinality: N:1
  scope: cross_realm
```

### 2.5 Proximity/Similarity Arcs (LLM-optimized)

```yaml
# Locale ↔ Locale (similarity score)
arc:
  name: CULTURALLY_SIMILAR
  family: semantic
  source: Locale
  target: Locale
  cardinality: N:M
  scope: intra_realm
  properties:
    similarity_score: { type: float, range: [0.0, 1.0] }
    factors: { type: "string[]" }  # ["language", "religion", "history"]
  description: "Pre-computed cultural similarity for LLM context"
```

### 2.6 Population Arcs

```yaml
# Locale → PopulationCluster (primary population)
arc:
  name: HAS_PRIMARY_POPULATION
  family: localization
  source: Locale
  target: PopulationCluster
  cardinality: N:1
  scope: cross_realm
  description: "Primary/majority population cluster for this locale"

# Locale → PopulationSubCluster (detailed population with percentage)
arc:
  name: HAS_POPULATION
  family: localization
  source: Locale
  target: PopulationSubCluster
  cardinality: N:M
  scope: cross_realm
  properties:
    percentage: { type: float, range: [0.0, 1.0], description: "Population percentage" }
    is_majority: { type: boolean, description: "True if >50%" }
  description: "Population sub-cluster with percentage for diversity in image generation"

# PopulationSubCluster → PopulationCluster (hierarchy)
arc:
  name: CLUSTER_OF
  family: ownership
  source: PopulationSubCluster
  target: PopulationCluster
  cardinality: N:1
  scope: intra_realm
  description: "Sub-cluster belongs to macro population cluster"
```

---

## Part 3: Graph Architecture

```
                                    ┌─────────────┐
                                    │  Continent  │ (6)
                                    │  AF,EU,AS...│
                                    └──────▲──────┘
                                           │ IN_CONTINENT
                                    ┌──────┴──────┐
                                    │  GeoRegion  │ (22)
                                    │ W.Europe... │
                                    └──────▲──────┘
                                           │ IN_REGION
                                    ┌──────┴──────┐
                                    │GeoSubRegion │ (optional)
                                    └──────▲──────┘
                                           │ IN_SUBREGION
┌──────────────┐                    ┌──────┴──────┐                    ┌──────────────┐
│LanguageFamily│◄── BRANCH_OF ─────│   Locale    │───IN_CULTURAL_SR──►│CulturalSubRea│
│  (Indo-Eur)  │                    │   fr-FR     │                    │ (W.European) │
└──────▲───────┘                    └──────┬──────┘                    └──────┬───────┘
       │                                   │                                  │
┌──────┴───────┐                           │                           ┌──────▼───────┐
│LanguageBranch│◄── SPEAKS_BRANCH ─────────┘                           │CulturalRealm │
│  (Romance)   │                                                       │ (Occidental) │
└──────────────┘                                                       └──────────────┘
                                    ┌──────┴──────┐
                     ┌──────────────┤   Locale    ├──────────────┐
                     │              │   fr-FR     │              │
                     ▼              └─────────────┘              ▼
              ┌─────────────┐                            ┌─────────────┐
              │ IncomeGroup │                            │EconomicReg. │
              │    (HIC)    │                            │    (ECA)    │
              └─────────────┘                            └─────────────┘
```

---

## Part 4: Implementation Plan

### Phase 1: Geographic Hierarchy (UN M49)

**Duration**: ~2-3 hours
**Nodes**: Continent (6), GeoRegion (22), GeoSubRegion (~15)
**Arcs**: IN_CONTINENT, IN_REGION, IN_SUBREGION

**Tasks**:
1. Create YAML definitions for Continent, GeoRegion, GeoSubRegion
2. Create YAML definitions for arcs
3. Create seed data generator (`generators/geo_hierarchy.rs`)
4. Add Neo4j constraints/indexes
5. Update Locale generator to create IN_SUBREGION arcs
6. Regenerate and reseed

**Cypher Example**:
```cypher
// Create continent
MERGE (c:Continent {key: 'EU'})
SET c.name = 'Europe',
    c.m49_code = 150,
    c.llm_context = 'USE: for European countries. TRIGGERS: EU, europe, european. NOT: Eurasia.';

// Create region
MERGE (r:GeoRegion {key: 'western-europe'})
SET r.name = 'Western Europe',
    r.m49_code = 155,
    r.llm_context = 'USE: for W.European countries (FR, DE, NL...). TRIGGERS: western europe, ouest.';

// Link region to continent
MATCH (r:GeoRegion {key: 'western-europe'})
MATCH (c:Continent {key: 'EU'})
MERGE (r)-[:IN_CONTINENT]->(c);

// Link locale to region
MATCH (l:Locale {key: 'fr-FR'})
MATCH (r:GeoRegion {key: 'western-europe'})
MERGE (l)-[:IN_SUBREGION]->(r);
```

**LLM Query Example**:
```cypher
// Find all locales in same region as fr-FR
MATCH (l:Locale {key: 'fr-FR'})-[:IN_SUBREGION]->(r:GeoRegion)
MATCH (r)<-[:IN_SUBREGION]-(similar:Locale)
WHERE similar.key <> 'fr-FR'
RETURN similar.key AS locale, r.name AS region
```

---

### Phase 2: Language Families (ISO 639-5)

**Duration**: ~2 hours
**Nodes**: LanguageFamily (~15), LanguageBranch (~30)
**Arcs**: BRANCH_OF, SPEAKS_BRANCH

**Tasks**:
1. Create YAML definitions
2. Create seed data generator (`generators/language_families.rs`)
3. Map Locale.language_code → LanguageBranch
4. Add constraints/indexes
5. Regenerate and reseed

**Mapping Table** (partial):

| Language Code | Branch | Family |
|---------------|--------|--------|
| fr | Romance (roa) | Indo-European (ine) |
| de | Germanic (gem) | Indo-European (ine) |
| ru | Slavic (sla) | Indo-European (ine) |
| zh | Chinese (zhx) | Sino-Tibetan (sit) |
| ja | Japonic (jpx) | Japonic (jpx) |
| ar | Semitic (sem) | Afro-Asiatic (afa) |
| hi | Indo-Iranian (iir) | Indo-European (ine) |
| ko | Koreanic (kor) | Koreanic (kor) |

---

### Phase 3: Cultural Realms

**Duration**: ~2 hours
**Nodes**: CulturalRealm (6), CulturalSubRealm (~20)
**Arcs**: PART_OF_REALM, IN_CULTURAL_SUBREALM

**Tasks**:
1. Create YAML definitions with rich llm_context
2. Create seed data generator
3. Map Locale → CulturalSubRealm (inference from region + religion + language)
4. Regenerate and reseed

**Mapping Logic**:
```rust
fn infer_cultural_subrealm(locale: &Locale) -> &str {
    match (locale.region.as_str(), locale.language_family.as_str()) {
        ("europe", "romance") => "mediterranean",
        ("europe", "germanic") => "west-european",
        ("europe", "slavic") => "continental-european",
        ("americas", "romance") => "latin-american",
        ("americas", "germanic") => "anglo-american",
        ("asia", "sino_tibetan") => "east-asian-sinosphere",
        ("asia", "japonic") => "east-asian-japanese",
        ("middle_east", _) => "islamic-arab",
        ("asia", "indo_aryan") => "indic-hindu",
        _ => "other"
    }
}
```

---

### Phase 4: Economic Classification

**Duration**: ~1.5 hours
**Nodes**: IncomeGroup (4), LendingCategory (3), EconomicRegion (7)
**Arcs**: HAS_INCOME_LEVEL, HAS_LENDING_TYPE, IN_ECONOMIC_REGION

**Tasks**:
1. Create YAML definitions
2. Fetch World Bank data (API or static JSON)
3. Create seed data generator
4. Map Locale.country_code → IncomeGroup, LendingCategory, EconomicRegion
5. Regenerate and reseed

**World Bank API**:
```
GET https://api.worldbank.org/v2/country?format=json&per_page=300
```

Response includes `incomeLevel.id` (LIC/LMC/UMC/HIC), `region.id`, `lendingType.id`.

---

### Phase 5: Population Clusters

**Duration**: ~2 hours
**Nodes**: PopulationCluster (7), PopulationSubCluster (~25)
**Arcs**: HAS_PRIMARY_POPULATION, HAS_POPULATION, CLUSTER_OF

**Tasks**:
1. Create YAML definitions for PopulationCluster, PopulationSubCluster
2. Create YAML definitions for population arcs
3. Create seed data generator (`generators/population_clusters.rs`)
4. Populate visual_traits for each sub-cluster
5. Map Locale → PopulationSubCluster with percentages
6. Regenerate and reseed

**Cypher Example**:
```cypher
// Create population cluster
MERGE (pc:PopulationCluster {key: 'west-eurasian'})
SET pc.name = 'West Eurasian',
    pc.visual_traits = ['light to medium skin tone', 'varied hair colors', 'varied eye colors'],
    pc.llm_context = 'USE: for European, Middle Eastern, South Asian (light) imagery. TRIGGERS: europe, mena, caucasian.';

// Create population sub-cluster with visual traits
MERGE (ps:PopulationSubCluster {key: 'northern-european'})
SET ps.name = 'Northern European',
    ps.parent_cluster = 'west-eurasian',
    ps.visual_traits = ['light to very light skin tone', 'blonde to light brown hair', 'blue/green/grey eyes'],
    ps.skin_tone_range = 'very light to light',
    ps.hair_colors = ['blonde', 'light brown', 'red'],
    ps.eye_colors = ['blue', 'green', 'grey', 'light brown'],
    ps.llm_context = 'USE: for Nordic/Germanic markets. TRIGGERS: scandinavia, germany, netherlands. NOT: mediterranean.';

// Link sub-cluster to cluster
MATCH (ps:PopulationSubCluster {key: 'northern-european'})
MATCH (pc:PopulationCluster {key: 'west-eurasian'})
MERGE (ps)-[:CLUSTER_OF]->(pc);

// Link locale to populations with percentages
MATCH (l:Locale {key: 'fr-FR'})
MATCH (ps1:PopulationSubCluster {key: 'mediterranean'})
MATCH (ps2:PopulationSubCluster {key: 'northern-european'})
MERGE (l)-[:HAS_POPULATION {percentage: 0.6, is_majority: true}]->(ps1)
MERGE (l)-[:HAS_POPULATION {percentage: 0.3, is_majority: false}]->(ps2);

// Link primary population cluster
MATCH (l:Locale {key: 'fr-FR'})
MATCH (pc:PopulationCluster {key: 'west-eurasian'})
MERGE (l)-[:HAS_PRIMARY_POPULATION]->(pc);
```

**LLM Query Example (Image Generation)**:
```cypher
// Get visual traits for image generation in fr-FR
MATCH (l:Locale {key: 'fr-FR'})-[hp:HAS_POPULATION]->(ps:PopulationSubCluster)
RETURN ps.name, ps.visual_traits, ps.skin_tone_range, hp.percentage
ORDER BY hp.percentage DESC

// Result:
// Mediterranean, ["olive to light brown skin", "dark hair"], "olive to light brown", 0.6
// Northern European, ["light skin", "varied hair"], "very light to light", 0.3
```

---

### Phase 6: Similarity Pre-computation

**Duration**: ~2 hours
**Arcs**: CULTURALLY_SIMILAR (N:M with score)

**Tasks**:
1. Define similarity algorithm
2. Create batch job to compute pairwise similarity
3. Store as edges with similarity_score property
4. Index for fast retrieval

**Similarity Factors**:
```rust
fn compute_similarity(a: &Locale, b: &Locale) -> f32 {
    let mut score = 0.0;

    // Same language branch: +0.3
    if a.language_branch == b.language_branch { score += 0.3; }

    // Same cultural subrealm: +0.25
    if a.cultural_subrealm == b.cultural_subrealm { score += 0.25; }

    // Same geo region: +0.2
    if a.geo_region == b.geo_region { score += 0.2; }

    // Same income group: +0.15
    if a.income_group == b.income_group { score += 0.15; }

    // Same script: +0.1
    if a.script == b.script { score += 0.1; }

    score
}
```

---

### Phase 7: TUI & Studio Integration

**Duration**: ~3 hours

**Tasks**:
1. Add taxonomy tree views in TUI (Continent > Region > Locale)
2. Add filter panels in Studio for new node types
3. Add visual encoding for cultural realms in node colors
4. Create Mermaid diagrams for documentation

---

## Part 5: Migration Strategy

### Keep Properties as Fallback

The existing string properties on Locale (`region`, `language_family`, `script`, `text_direction`) will be **kept** as denormalized cache for:
- Fast property-based queries
- Backward compatibility
- Index-based filtering

The new node-based architecture adds **traversal capabilities** without breaking existing queries.

### Validation

Add validation that properties match graph relationships:
```cypher
// Check consistency
MATCH (l:Locale)-[:IN_SUBREGION]->(r:GeoRegion)
WHERE l.region <> r.key  // Should return 0 rows
RETURN l.key, l.region, r.key
```

---

## Part 6: Node/Arc Counts

### Before v10.8

| Type | Count |
|------|-------|
| NodeKinds | 46 |
| ArcKinds | 72 |

### After v10.8

| Type | Added | New Total |
|------|-------|-----------|
| NodeKinds | +12 | 58 |
| ArcKinds | +15 | 87 |

**New Nodes (12)**:
1. Continent
2. GeoRegion
3. GeoSubRegion
4. LanguageFamily
5. LanguageBranch
6. CulturalRealm
7. CulturalSubRealm
8. IncomeGroup
9. LendingCategory
10. EconomicRegion
11. PopulationCluster
12. PopulationSubCluster

**New Arcs (15)**:
1. IN_CONTINENT
2. IN_REGION
3. IN_SUBREGION
4. BRANCH_OF
5. SPEAKS_BRANCH
6. PART_OF_REALM
7. IN_CULTURAL_SUBREALM
8. HAS_INCOME_LEVEL
9. HAS_LENDING_TYPE
10. IN_ECONOMIC_REGION
11. CULTURALLY_SIMILAR
12. SHARES_LANGUAGE_WITH (optional)
13. HAS_PRIMARY_POPULATION
14. HAS_POPULATION
15. CLUSTER_OF

---

## Part 7: LLM Retrieval Queries

### Query 1: Find Similar Locales

```cypher
// Find locales culturally similar to fr-FR
MATCH (l:Locale {key: 'fr-FR'})
MATCH (l)-[:IN_CULTURAL_SUBREALM]->(cs:CulturalSubRealm)
MATCH (cs)<-[:IN_CULTURAL_SUBREALM]-(similar:Locale)
WHERE similar.key <> 'fr-FR'
RETURN similar.key, similar.display_name
ORDER BY similar.key

// Result: it-IT, es-ES, pt-PT, etc. (Mediterranean sub-realm)
```

### Query 2: Find All Romance Language Locales

```cypher
MATCH (b:LanguageBranch {key: 'roa'})<-[:SPEAKS_BRANCH]-(l:Locale)
RETURN l.key, l.display_name

// Result: fr-FR, es-ES, it-IT, pt-PT, pt-BR, ro-RO, ca-ES, etc.
```

### Query 3: Retrieve Context for Generation

```cypher
// Get full context for a locale
MATCH (l:Locale {key: 'fr-FR'})
OPTIONAL MATCH (l)-[:IN_SUBREGION]->(geo:GeoRegion)-[:IN_CONTINENT]->(c:Continent)
OPTIONAL MATCH (l)-[:SPEAKS_BRANCH]->(lang:LanguageBranch)-[:BRANCH_OF]->(fam:LanguageFamily)
OPTIONAL MATCH (l)-[:IN_CULTURAL_SUBREALM]->(cs:CulturalSubRealm)-[:PART_OF_REALM]->(cr:CulturalRealm)
OPTIONAL MATCH (l)-[:HAS_INCOME_LEVEL]->(inc:IncomeGroup)
RETURN l.key,
       c.name AS continent,
       geo.name AS region,
       fam.name AS language_family,
       lang.name AS language_branch,
       cr.name AS cultural_realm,
       cs.name AS cultural_subrealm,
       inc.name AS income_level
```

### Query 4: Pre-computed Similarity

```cypher
// Get top 5 most similar locales
MATCH (l:Locale {key: 'fr-FR'})-[sim:CULTURALLY_SIMILAR]->(similar:Locale)
RETURN similar.key, sim.similarity_score, sim.factors
ORDER BY sim.similarity_score DESC
LIMIT 5
```

### Query 5: Population Diversity for Image Generation

```cypher
// Get diverse population representation for image generation
MATCH (l:Locale {key: 'fr-FR'})-[hp:HAS_POPULATION]->(ps:PopulationSubCluster)-[:CLUSTER_OF]->(pc:PopulationCluster)
RETURN ps.name AS sub_cluster,
       pc.name AS macro_cluster,
       ps.visual_traits AS visual_traits,
       ps.skin_tone_range AS skin_tone,
       ps.hair_colors AS hair,
       ps.eye_colors AS eyes,
       hp.percentage AS population_percentage
ORDER BY hp.percentage DESC

// Example result for fr-FR:
// | sub_cluster       | macro_cluster  | visual_traits                   | skin_tone          | percentage |
// |-------------------|----------------|--------------------------------|--------------------|-----------:|
// | Mediterranean     | West Eurasian  | ["olive skin", "dark hair"]    | olive to light brown | 0.60    |
// | Northern European | West Eurasian  | ["light skin", "varied hair"]  | very light to light  | 0.30    |
// | Sub-Saharan       | Sub-Saharan    | ["dark skin", "black hair"]    | medium to dark brown | 0.10    |
```

### Query 6: Build Image Generation Prompt

```cypher
// Generate diverse image prompts weighted by population
MATCH (l:Locale {key: 'ja-JP'})-[hp:HAS_POPULATION]->(ps:PopulationSubCluster)
WHERE hp.percentage > 0.1  // Only significant populations
WITH ps, hp.percentage AS weight
RETURN ps.name,
       ps.visual_traits[0] AS primary_trait,
       ps.skin_tone_range,
       weight,
       // Image prompt template
       'professional person, ' + ps.visual_traits[0] + ', ' + ps.skin_tone_range + ' skin' AS image_prompt
ORDER BY weight DESC

// Result for ja-JP:
// | name     | primary_trait      | skin_tone_range  | weight | image_prompt                                      |
// |----------|--------------------|------------------|--------|---------------------------------------------------|
// | Japanese | light to medium    | light to medium  | 0.98   | professional person, light to medium, light to... |
```

---

## Appendix A: UN M49 Full Hierarchy

| Continent | M49 | Region | M49 |
|-----------|-----|--------|-----|
| Africa | 002 | Northern Africa | 015 |
| | | Eastern Africa | 014 |
| | | Middle Africa | 017 |
| | | Southern Africa | 018 |
| | | Western Africa | 011 |
| Americas | 019 | Northern America | 021 |
| | | Caribbean | 029 |
| | | Central America | 013 |
| | | South America | 005 |
| Asia | 142 | Central Asia | 143 |
| | | Eastern Asia | 030 |
| | | South-eastern Asia | 035 |
| | | Southern Asia | 034 |
| | | Western Asia | 145 |
| Europe | 150 | Eastern Europe | 151 |
| | | Northern Europe | 154 |
| | | Southern Europe | 039 |
| | | Western Europe | 155 |
| Oceania | 009 | Australia and New Zealand | 053 |
| | | Melanesia | 054 |
| | | Micronesia | 057 |
| | | Polynesia | 061 |

---

## Appendix B: ISO 639-5 Language Families

| Family | Code | Major Branches |
|--------|------|----------------|
| Indo-European | ine | Romance (roa), Germanic (gem), Slavic (sla), Celtic (cel), Indo-Iranian (iir), Greek (grk), Albanian (sqj), Armenian (hyx), Baltic (bat) |
| Sino-Tibetan | sit | Chinese (zhx), Tibeto-Burman (tbq) |
| Afro-Asiatic | afa | Semitic (sem), Berber (ber), Cushitic (cus), Egyptian (egx), Chadic (cdc), Omotic (omv) |
| Austronesian | map | Malayo-Polynesian (poz), Formosan (fox) |
| Dravidian | dra | - |
| Uralic | urj | Finno-Ugric (fiu), Samoyedic (syd) |
| Altaic | tut | Turkic (trk), Mongolic (xgn), Tungusic (tuw) |
| Japonic | jpx | - |
| Koreanic | kor | - |
| Austroasiatic | aav | Mon-Khmer (mkh), Munda (mun) |
| Tai-Kadai | tai | - |
| Niger-Congo | nic | Atlantic-Congo (alv), Mande (dmn) |
| Nilo-Saharan | ssa | - |

---

## Appendix C: World Bank Regions

| Region | Code | Countries (examples) |
|--------|------|---------------------|
| East Asia & Pacific | EAP | China, Japan, Korea, Indonesia |
| Europe & Central Asia | ECA | France, Germany, Russia, Turkey |
| Latin America & Caribbean | LAC | Brazil, Mexico, Argentina |
| Middle East & North Africa | MENA | Egypt, Morocco, Saudi Arabia |
| North America | NAC | USA, Canada |
| South Asia | SAS | India, Pakistan, Bangladesh |
| Sub-Saharan Africa | SSA | Nigeria, Kenya, South Africa |

---

## Appendix D: Population Clusters

| Macro Cluster | Key | Sub-Clusters | Primary Regions |
|---------------|-----|--------------|-----------------|
| West Eurasian | west-eurasian | Northern European, Mediterranean, Middle Eastern, South Asian (light), Central Asian | Europe, MENA, South Asia |
| East Asian | east-asian | Han Chinese, Japanese, Korean, Mongolian | East Asia |
| Southeast Asian | southeast-asian | Malay, Filipino, Vietnamese, Thai, Indonesian | Southeast Asia |
| South Asian | south-asian | North Indian, South Indian, Bengali, Sri Lankan | South Asia |
| Sub-Saharan African | sub-saharan | West African, East African, Central African, Southern African, Nilotic | Sub-Saharan Africa |
| Native American | native-american | North American Indigenous, Mesoamerican, Andean, Amazonian | Americas |
| Oceanian | oceanian | Australian Aboriginal, Melanesian, Polynesian, Micronesian | Oceania |

### Visual Traits Reference

| Sub-Cluster | Skin Tone | Hair Colors | Eye Colors |
|-------------|-----------|-------------|------------|
| Northern European | very light to light | blonde, light brown, red | blue, green, grey |
| Mediterranean | light to olive | dark brown, black | brown, hazel, green |
| Middle Eastern | olive to light brown | black, dark brown | brown, hazel |
| Han Chinese | light to medium | black | brown, dark brown |
| Japanese | light to medium | black | brown, dark brown |
| South Indian | medium to dark brown | black | brown, dark brown |
| West African | dark brown to black | black | brown, dark brown |
| Polynesian | medium to dark brown | black, dark brown | brown |

---

## Decision Log

| Decision | Rationale |
|----------|-----------|
| Keep existing properties | Backward compatibility, fast index queries |
| Add nodes for traversal | Enable LLM context retrieval via graph paths |
| Pre-compute similarity | Avoid runtime computation for common queries |
| Use M49 for geography | Official UN standard, widely adopted |
| Use ISO 639-5 for languages | Official ISO standard for language families |
| Use Broek & Webb for culture | Academic standard in cultural geography |
| Use World Bank for economics | Updated annually, API available |
| Use "Population" terminology | Academic, neutral term for genetic/phenotypic clusters |
| visual_traits for images | Enables diverse, representative image generation |
