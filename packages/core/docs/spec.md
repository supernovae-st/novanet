# NovaNet - Specification v7.2.1

Système de génération de contenu localisé avec Neo4j.

> **v7.2.1**: Filter & View system for graph visualization and context loading.
> **v7.2.0**: Prompts separation (PagePrompt, BlockPrompt, BlockRules) with versioning.
> **v7.1.0**: Context management (priority, freshness) and key naming conventions.
> **v7.0.0**: Standard node properties, graph-native Locale, Locale Knowledge nodes.

## Architecture

```
PROJECT              CONCEPTS                    PAGES                    LOCALE
═══════              ════════                    ═════                    ══════

┌───────────┐                                                          ┌────────┐
│  Project  │──[:SUPPORTS_LOCALE]─────────────────────────────────────▶│ Locale │
└─────┬─────┘                                                          └────┬───┘
      │                                                                     │
      ├─[:HAS_PAGE]──▶ Page ──[:HAS_OUTPUT]──▶ PageL10n ──:FOR_LOCALE────┘
      │                  │                                              ▲ ▲
      │                  ├─[:HAS_BLOCK]──▶ Block ──:HAS_OUTPUT──▶ BlockL10n ──:FOR_LOCALE──┘
      │                  │                    │                         │
      │                  │                    └─[:OF_TYPE]──▶ BlockType │
      │                  │                    └─[:USES_CONCEPT]──▶ Concept
      │                  └─[:USES_CONCEPT]──▶ Concept                   │
      │                                          │                      │
      ├─[:HAS_CONCEPT]──▶ Concept                │                      │
      │                      ├─[:HAS_L10N]──▶ ConceptL10n ──:FOR_LOCALE─┘
      │                      ├─[:TARGETS_SEO]──▶ SEOKeywordL10n ──:FOR_LOCALE──┐
      │                      ├─[:TARGETS_GEO]──▶ GEOSeedL10n ──:FOR_LOCALE─────┘
      │                      └─[:SEMANTIC_LINK {type, temp}]──▶ Concept
      │
      ├─[:HAS_BRAND_IDENTITY]──▶ BrandIdentity
      │
      └─[:HAS_L10N]──▶ ProjectL10n ──:FOR_LOCALE──────────────────────────────────────────┐
                          │                                                               │
                          └─[:BELONGS_TO_PROJECT_L10N]◀── PageL10n (v7.8.0)               │

Locale ──[:HAS_IDENTITY]──▶ LocaleIdentity
       ├─[:HAS_VOICE]──▶ LocaleVoice
       ├─[:HAS_CULTURE]──▶ LocaleCulture
       ├─[:HAS_MARKET]──▶ LocaleMarket
       └─[:HAS_LEXICON]──▶ LocaleLexicon ──[:HAS_EXPRESSION]──▶ Expression
```

## Relations

| Relation | De → Vers | Notes |
|----------|-----------|-------|
| `:HAS_PAGE` | Project → Page | Pages du projet |
| `:HAS_CONCEPT` | Project → Concept | Concepts du projet |
| `:SUPPORTS_LOCALE` | Project → Locale | Locales supportées {default} |
| `:HAS_BRAND_IDENTITY` | Project → BrandIdentity | Identité de marque |
| `:USES_CONCEPT` | Page/Block → Concept | {purpose, temperature} |
| `:SEMANTIC_LINK` | Concept ↔ Concept | Spreading activation (10 types) |
| `:HAS_L10N` | Concept/Project/Audience → *L10n | Contenu localisé |
| `:FOR_LOCALE` | *L10n/*Output → Locale | Cible une locale |
| `:FALLBACK_TO` | Locale → Locale | Chaîne de fallback |
| `:HAS_OUTPUT` | Page/Block → PageL10n/BlockL10n | Contenu généré |
| `:HAS_METRICS` | PageL10n → PageMetrics | Analytics data |
| `:HAS_BLOCK` | Page → Block | Structure page |
| `:OF_TYPE` | Block → BlockType | Type du block |
| `:TARGETS_SEO` | Concept → SEOKeywordL10n | SEO keywords |
| `:TARGETS_GEO` | Concept → GEOSeedL10n | GEO seeds |
| `:HAS_SNAPSHOT` | SEOKeywordL10n → SEOSnapshot | Historique SEO |
| `:HAS_VARIATION` | SEOKeywordL10n → SEOVariation | Variations SEO |
| `:HAS_REFORMULATION` | GEOSeedL10n → GEOReformulation | GEO mining |
| `:HAS_CITATION` | GEOSeedL10n → GEOCitation | GEO tracking |
| `:HAS_IDENTITY` | Locale → LocaleIdentity | Locale Knowledge |
| `:HAS_VOICE` | Locale → LocaleVoice | Locale Knowledge |
| `:HAS_CULTURE` | Locale → LocaleCulture | Locale Knowledge |
| `:HAS_MARKET` | Locale → LocaleMarket | Locale Knowledge |
| `:HAS_LEXICON` | Locale → LocaleLexicon | Locale Knowledge |
| `:HAS_PROMPT` | Page/Block → PagePrompt/BlockPrompt | AI instructions (v7.2.0) |
| `:HAS_RULES` | BlockType → BlockRules | Generation rules (v7.2.0) |
| `:GENERATED` | PagePrompt/BlockPrompt → Output | Provenance tracking (v7.2.0) |

## SEMANTIC_LINK Types

| Type | Inverse | Temperature | Description |
|------|---------|-------------|-------------|
| `is_action_on` | `has_action` | 0.95 | Verb-noun (create → qr-code) |
| `has_action` | `is_action_on` | 0.90 | Noun-verb inverse |
| `includes` | `included_in` | 0.85 | Container (tier-pro → analytics) |
| `included_in` | `includes` | 0.80 | Part-whole inverse |
| `type_of` | `has_type` | 0.90 | Taxonomy (vcard → qr-code-type) |
| `has_type` | `type_of` | 0.85 | Is-a inverse |
| `requires` | `required_by` | 0.80 | Dependency |
| `required_by` | `requires` | 0.75 | Dependency inverse |
| `related` | symmetric | 0.60 | Generic association |
| `opposite` | symmetric | 0.40 | Contrast |

## Types de Concept

| Type | Exemples |
|------|----------|
| `action` | create-qr-code, scan-qr-code |
| `object` | qr-code-generator, vcard |
| `feature` | analytics, bulk-create |
| `tier` | tier-free, tier-pro |
| `brand` | qrcode-ai |
| `content-type` | instagram, wifi, email |
| `abstract` | security, performance |

## Locale Knowledge (v7.0.0)

| Node | Description | Purpose |
|------|-------------|---------|
| `Locale` | Locale graph-native (fr-FR) | Central locale node with fallback chains |
| `LocaleIdentity` | Script, timezone, encoding | Technical characteristics |
| `LocaleVoice` | Formality, pronouns, tone | Voice generation rules |
| `LocaleCulture` | Values, taboos, sensitivities | Cultural content guidelines |
| `LocaleMarket` | Demographics, ecommerce | Market positioning |
| `LocaleLexicon` | Vocabulary, idioms | Lexical preferences |
| `Expression` | Idiomatic expressions | Semantic field expressions |

> **v7.0.0**: Locale Knowledge replaces L10NContent. Graph-native approach via :FOR_LOCALE relation.

## Locale Query Pattern (v7.0.0)

```cypher
// Get Locale with all knowledge for LLM context
MATCH (l:Locale {key: $locale})
OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(li:LocaleIdentity)
OPTIONAL MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
OPTIONAL MATCH (l)-[:HAS_CULTURE]->(lc:LocaleCulture)
OPTIONAL MATCH (l)-[:HAS_MARKET]->(lm:LocaleMarket)
OPTIONAL MATCH (l)-[:HAS_LEXICON]->(ll:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
RETURN l, li, lv, lc, lm, ll, collect(e) AS expressions
```

## Langue

| EN | LOCALE |
|----|--------|
| Concept.key, llm_context | ConceptL10n.* |
| Page.key, llm_context, instructions | PageL10n.* |
| Block.key, instructions | BlockL10n.*, SEOKeywordL10n.value, GEOSeedL10n.value |
| BlockType.key, rules | L10NContent.content |

## GEO Mining (GEOSeedL10n)

```
GEOSeedL10n (seed query)
    │
    ├─[:HAS_REFORMULATION]─▶ GEOReformulation (LLM sub-queries)
    │                        • type: search_query | sub_question | related_topic
    │                        • platform: chatgpt | claude | perplexity | gemini
    │                        • frequency: combien de fois observé
    │                        • content_gap: true si on ne couvre pas
    │
    └─[:HAS_CITATION]─▶ GEOCitation (brand tracking)
                        • cited: true/false
                        • position: rang dans la réponse
                        • sentiment: positive | neutral | negative
                        • competitors: [{name, position}]
```

## Filter System (v7.2.1)

The Filter/View system provides composable queries for graph visualization and context loading.

### Implemented Features (v7.2.1)

| Feature | NovaNetFilter Method | CypherGenerator | Status |
|---------|-------------------|-----------------|--------|
| Root node selection | `fromPage()`, `fromBlock()`, etc. | ✓ MATCH clause | ✅ |
| Include relations | `includeBlocks()`, `includeConcepts()`, etc. | ✓ OPTIONAL MATCH | ✅ |
| Locale filter | `forLocale()` | ✓ param only | ✅ |
| Priority filter | `withPriority()` | ✓ WHERE clause | ✅ |
| Freshness filter | `withFreshness()` | ✓ WHERE clause | ✅ |
| Node type filter | `byTypes()` | ✓ WHERE clause | ✅ |
| Exclude types | `excludeTypes()` | ✓ WHERE clause | ✅ |
| Category filter | `byCategory()` | ✓ WHERE (expanded) | ✅ |
| Fulltext search | `search()` | ✓ WHERE CONTAINS | ✅ |
| Active filter (includes) | `{ activeOnly: true }` | ✓ node props | ✅ |
| Spreading activation | `{ spreading: true }` | ✓ SEMANTIC_LINK*N | ✅ |

### Roadmap (v7.3.0+)

| Feature | Description | Status |
|---------|-------------|--------|
| `localeFamily` | Filter by locale family (e.g., 'fr' for fr-FR, fr-CA) | @planned |
| `active` (top-level) | Filter root nodes by active status | @planned |
| `maxDepth` | Global depth limit for all traversals | @planned |
| `targetTypes` | Filter include results by node type | @planned |
| Nested includes | Recursive include rules | @planned |
| `latestOnly` | Get only latest output version | @planned |

### Usage Example

```typescript
import { NovaNetFilter, CypherGenerator, ViewLoader } from 'novanet-core/filters';

// Programmatic filter
const filter = NovaNetFilter.create()
  .fromPage('page-pricing')
  .includeBlocks()
  .includeConcepts({ spreading: true })
  .includePrompts({ activeOnly: true })
  .forLocale('fr-FR')
  .withPriority('critical', 'high')
  .withFreshness('evergreen', 'stable');

const { query, params } = CypherGenerator.generate(filter);

// Or load from YAML view
const view = await ViewLoader.loadView('page-generation-context', viewsDir);
const filter2 = ViewLoader.toFilter(view, { key: 'page-pricing', locale: 'fr-FR' });
```
