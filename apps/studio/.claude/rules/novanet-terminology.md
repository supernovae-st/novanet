# NovaNet Terminology (v9.0.0)

## Core Concepts

| Term | Definition |
|------|------------|
| **Project** | Business entity with brand identity, audiences, and content structure |
| **Concept** | Abstract semantic meaning (business concept) that content references |
| **Page** | Content page template containing blocks |
| **Block** | Section/component within a page, typed by BlockType |
| **Locale** | Language/region code (BCP 47, e.g., "fr-FR", "en-US") |
| **Expression** | Specific phrase/word variant in a locale's lexicon |
| **Context Graph** | Knowledge graph enriched with operational metadata and self-describing schema for AI agents |

## Meta-Graph (v9 — Self-Describing Schema)

v9 introduces a **faceted classification** where each Kind sits at the intersection of 4 axes:

```
Axis 1 — WHERE?  :Realm     (global / project / shared)
Axis 2 — WHAT?   :Layer     (knowledge / structure / semantic / ...)
Axis 3 — HOW?    :Trait     (invariant / localized / knowledge / derived / job)
Axis 4 — LINKS?  :ArcKind  (SEMANTIC_LINK, HAS_OUTPUT, HAS_L10N, ...)
```

### Meta-Node Types (6 types)

| Meta-Type | Count | Purpose |
|-----------|-------|---------|
| **Realm** | 3 | Visibility boundary / data governance zone |
| **Layer** | 9 | Functional classification / architectural layer |
| **Kind** | 35 | A node type in the data graph (1:1 with Neo4j labels) |
| **Trait** | 5 | Locale behavior — how a node type changes across locales |
| **ArcFamily** | 5 | Classification of relationship types |
| **ArcKind** | 50 | Individual relationship type (1:1 with Neo4j rel types) |

All meta-nodes carry the `:Meta` double-label for easy filtering.

### :Realm (3 nodes)

| Key | Display Name | Emoji |
|-----|-------------|-------|
| `global` | Global | `🌍` |
| `project` | Project | `📦` |
| `shared` | Shared | `🎯` |

### :Layer (9 nodes)

| Key | Display Name | Emoji | Parent Realm |
|-----|-------------|-------|-------------|
| `config` | Configuration | `⚙️` | global |
| `knowledge` | Locale Knowledge | `📚` | global |
| `foundation` | Foundation | `🏛️` | project |
| `structure` | Structure | `🏗️` | project |
| `semantic` | Semantic Layer | `💡` | project |
| `instruction` | Instructions | `📝` | project |
| `output` | Generated Output | `✨` | project |
| `seo` | SEO Intelligence | `🔍` | shared |
| `geo` | GEO Intelligence | `📍` | shared |

### :Trait (5 nodes)

| Key | Display Name | Description |
|-----|-------------|-------------|
| `invariant` | Invariant | Does not change between locales |
| `localized` | Localized | Generated natively per locale |
| `knowledge` | Knowledge | Cultural/linguistic expertise per locale |
| `derived` | Derived | Computed/aggregated data |
| `job` | Job | Background processing tasks |

### :ArcFamily (5 nodes)

| Key | Display Name | Arrow Style |
|-----|-------------|------------|
| `ownership` | Ownership | `-->` |
| `localization` | Localization | `-.->` |
| `semantic` | Semantic | `-.->` |
| `generation` | Generation | `==>` |
| `mining` | Mining | `--o` |

## Kind Inventory (35 across 3 Realms)

| Kind (label) | Realm | Layer | Trait |
|-------------|-------|-------|-------|
| Locale | global | config | invariant |
| LocaleIdentity | global | knowledge | knowledge |
| LocaleVoice | global | knowledge | knowledge |
| LocaleCulture | global | knowledge | knowledge |
| LocaleCultureReferences | global | knowledge | knowledge |
| LocaleLexicon | global | knowledge | knowledge |
| LocaleMarket | global | knowledge | knowledge |
| LocaleRulesAdaptation | global | knowledge | knowledge |
| LocaleRulesFormatting | global | knowledge | knowledge |
| LocaleRulesSlug | global | knowledge | knowledge |
| Expression | global | knowledge | knowledge |
| Metaphor | global | knowledge | knowledge |
| Pattern | global | knowledge | knowledge |
| Reference | global | knowledge | knowledge |
| Constraint | global | knowledge | knowledge |
| Project | project | foundation | invariant |
| BrandIdentity | project | foundation | invariant |
| ProjectL10n | project | foundation | localized |
| Page | project | structure | invariant |
| Block | project | structure | invariant |
| Concept | project | semantic | invariant |
| ConceptL10n | project | semantic | localized |
| PageType | project | instruction | invariant |
| BlockType | project | instruction | invariant |
| PagePrompt | project | instruction | invariant |
| BlockPrompt | project | instruction | invariant |
| BlockRules | project | instruction | invariant |
| PageL10n | project | output | localized |
| BlockL10n | project | output | localized |
| SEOKeywordL10n | shared | seo | localized |
| SEOKeywordMetrics | shared | seo | derived |
| SEOMiningRun | shared | seo | job |
| GEOSeedL10n | shared | geo | localized |
| GEOSeedMetrics | shared | geo | derived |
| GEOMiningRun | shared | geo | job |

## Locale Knowledge Structure (14 nodes)

| Node | Purpose |
|------|---------|
| **LocaleIdentity** | Script, timezone, technical characteristics |
| **LocaleVoice** | Formality, tone, pronunciation rules |
| **LocaleCulture** | Cultural norms, taboos |
| **LocaleCultureReferences** | Cultural references container |
| **Reference** | Specific cultural references |
| **Metaphor** | Cultural metaphors |
| **Constraint** | Cultural constraints |
| **LocaleMarket** | Market data, demographics, platforms |
| **LocaleLexicon** | Vocabulary preferences per locale |
| **Expression** | Specific phrase variants |
| **LocaleRulesAdaptation** | Content adaptation rules |
| **LocaleRulesFormatting** | Format rules (dates, numbers) |
| **LocaleRulesSlug** | URL slug generation rules |
| **Pattern** | Reusable formatting patterns |

## Standard Properties (all data nodes)

| Property | Type | Description |
|----------|------|-------------|
| `key` | string | Unique identifier with semantic prefix |
| `display_name` | string | Human-readable name |
| `description` | string | Human-readable description |
| `llm_context` | string | AI hints: "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]." |
| `created_at` | date | Creation timestamp |
| `updated_at` | date | Last update timestamp |

## Meta-Graph Arcs

### Hierarchy (top-down navigation)
```
Realm -[:HAS_LAYER]-> Layer
Layer -[:HAS_KIND]-> Kind
ArcFamily -[:HAS_EDGE_KIND]-> ArcKind
```

### Facets (Kind-centric)
```
Kind -[:IN_REALM]-> Realm
Kind -[:IN_LAYER]-> Layer
Kind -[:HAS_TRAIT]-> Trait
```

### Arc Schema (OWL-inspired)
```
ArcKind -[:FROM_KIND]-> Kind   (source node type)
ArcKind -[:TO_KIND]-> Kind     (target node type)
ArcKind -[:IN_FAMILY]-> ArcFamily
```

### Instance Bridge
```
DataNode -[:OF_KIND]-> Kind     (every data node links to its Kind)
```

## Key Data Arcs

| Arc | From -> To | ArcFamily | Description |
|----------|-----------|------------|-------------|
| `HAS_CONCEPT` | Project -> Concept | ownership | Project owns concepts |
| `HAS_PAGE` | Project -> Page | ownership | Project owns pages |
| `SUPPORTS_LOCALE` | Project -> Locale | ownership | Available locales |
| `HAS_L10N` | Invariant -> L10n | localization | Curated localized content |
| `HAS_OUTPUT` | Page/Block -> L10n | generation | LLM-generated content |
| `HAS_BLOCK` | Page -> Block | ownership | Page structure (with `position`) |
| `OF_TYPE` | Block -> BlockType | ownership | Block template type |
| `USES_CONCEPT` | Page/Block -> Concept | semantic | Content references concept |
| `SEMANTIC_LINK` | Concept -> Concept | semantic | Spreading activation |
| `HAS_SEO_TARGET` | ConceptL10n -> SEOKeywordL10n | mining | Locale-aligned SEO |
| `HAS_GEO_TARGET` | ConceptL10n -> GEOSeedL10n | mining | Locale-aligned GEO |
| `FOR_LOCALE` | L10n -> Locale | localization | Locale assignment |

## v8 -> v9 Rename Mapping

| v8 Term | v9 Term |
|---------|---------|
| Scope | **Realm** |
| Subcategory | **Layer** |
| NodeTypeMeta | **Kind** |
| IN_SUBCATEGORY | **OF_KIND** |
| _(new)_ | **Trait** |
| _(new)_ | **ArcFamily** |
| _(new)_ | **ArcKind** |
| DataMode (data/schema) | **NavigationMode** (data/meta/overlay/query) |

## Abbreviations

- **L10n** - Localization (localized variant of invariant node)
- **SEO** - Search Engine Optimization
- **GEO** - Generative Engine Optimization (ChatGPT, Perplexity)
- **LLM** - Large Language Model (AI context)
- **TUI** - Terminal User Interface
- **CLI** - Command Line Interface
